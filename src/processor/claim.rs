use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult,
    program_error::ProgramError, pubkey::Pubkey, sysvar::Sysvar,
};

use crate::{
    error::OreError, instruction::ClaimArgs, loaders::*, state::Proof, utils::AccountDeserialize,
    MINT_ADDRESS, ONE_DAY, TREASURY, TREASURY_BUMP,
};

/// Claim distributes Ore from the treasury to a miner. Its responsibilies include:
/// 1. Decrement the miner's claimable balance.
/// 2. Transfer tokens from the treasury to the miner.
///
/// Safety requirements:
/// - Claim is a permissionless instruction and can be called by any user.
/// - Can only succeed if the claimed amount is less than or equal to the miner's claimable rewards.
/// - The provided beneficiary, token account, treasury, treasury token account, and token program must be valid.
pub fn process_claim<'a, 'info>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args
    let args = ClaimArgs::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts
    let [signer, beneficiary_info, mint_info, proof_info, treasury_info, treasury_tokens_info, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_token_account(beneficiary_info, None, &MINT_ADDRESS, true)?;
    load_mint(mint_info, MINT_ADDRESS, true)?;
    load_treasury(treasury_info, false)?;
    load_token_account(
        treasury_tokens_info,
        Some(treasury_info.key),
        &MINT_ADDRESS,
        true,
    )?;
    load_program(token_program, spl_token::id())?;

    // If last claim was less than 1 day ago, burn some of the claim amount
    let mut claim_amount = amount;
    let mut proof_data = proof_info.data.borrow_mut();
    let proof = Proof::try_from_bytes_mut(&mut proof_data)?;
    let clock = Clock::get().or(Err(ProgramError::InvalidAccountData))?;
    let t = proof.last_claim_at.saturating_add(ONE_DAY);
    if clock.unix_timestamp.lt(&t) {
        // Calculate burn amount
        let burn_amount = amount
            .saturating_mul(t.saturating_sub(clock.unix_timestamp) as u64)
            .saturating_div(ONE_DAY as u64);

        // Burn tokens from treasury
        solana_program::program::invoke_signed(
            &spl_token::instruction::burn(
                &spl_token::id(),
                treasury_tokens_info.key,
                mint_info.key,
                treasury_info.key,
                &[treasury_info.key],
                burn_amount,
            )?,
            &[
                token_program.clone(),
                treasury_tokens_info.clone(),
                mint_info.clone(),
                treasury_info.clone(),
            ],
            &[&[TREASURY, &[TREASURY_BUMP]]],
        )?;

        // Update claim amount
        claim_amount = amount.saturating_sub(burn_amount);
    }

    // Update miner balance
    proof.balance = proof
        .balance
        .checked_sub(amount)
        .ok_or(OreError::ClaimTooLarge)?;

    // Update timestamp
    proof.last_claim_at = clock.unix_timestamp;

    // Distribute tokens from treasury to beneficiary
    solana_program::program::invoke_signed(
        &spl_token::instruction::transfer(
            &spl_token::id(),
            treasury_tokens_info.key,
            beneficiary_info.key,
            treasury_info.key,
            &[treasury_info.key],
            claim_amount,
        )?,
        &[
            token_program.clone(),
            treasury_tokens_info.clone(),
            beneficiary_info.clone(),
            treasury_info.clone(),
        ],
        &[&[TREASURY, &[TREASURY_BUMP]]],
    )?;

    Ok(())
}
