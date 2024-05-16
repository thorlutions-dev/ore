use array_const_fn_init::array_const_fn_init;
use const_crypto::ed25519;
use solana_program::{pubkey, pubkey::Pubkey};

/// The reward rate to intialize the program with.
pub const INITIAL_BASE_REWARD_RATE: u64 = 10u64.pow(3u32);

/// The spam/liveness tolerance to initialize the program with.
pub const INITIAL_TOLERANCE: i64 = 5;

/// The minimum difficulty required of all submitted hashes.
pub const MIN_DIFFICULTY: u32 = 8; // 12;

/// The decimal precision of the Ore token.
/// There are 100 billion indivisible units per Ore (called "grains").
pub const TOKEN_DECIMALS: u8 = 11;

/// One Ore token, denominated in indivisible units.
pub const ONE_ORE: u64 = 10u64.pow(TOKEN_DECIMALS as u32);

/// The duration of one minute, in seconds.
pub const ONE_MINUTE: i64 = 60;

/// The duration of one day, in seconds.
pub const ONE_DAY: i64 = 86400;

/// The duration of one year, in minutes.
pub const ONE_YEAR: u64 = 525600;

/// The number of minutes in an Ore epoch.
pub const EPOCH_MINUTES: i64 = 1;

/// The duration of an Ore epoch, in seconds.
pub const EPOCH_DURATION: i64 = ONE_MINUTE.saturating_mul(EPOCH_MINUTES);

/// The maximum token supply (42 million).
pub const MAX_SUPPLY: u64 = ONE_ORE.saturating_mul(42_000_000);

/// The target quantity of ORE to be mined per epoch.
pub const TARGET_EPOCH_REWARDS: u64 = ONE_ORE.saturating_mul(EPOCH_MINUTES as u64);

/// The maximum quantity of ORE that can be mined per epoch.
/// Inflation rate ≈ 1 ORE / min (min 0, max 5)
pub const MAX_EPOCH_REWARDS: u64 = TARGET_EPOCH_REWARDS.saturating_mul(5);

/// The quantity of ORE each bus is allowed to issue per epoch.
pub const BUS_EPOCH_REWARDS: u64 = MAX_EPOCH_REWARDS.saturating_div(BUS_COUNT as u64);

/// The number of bus accounts, for parallelizing mine operations.
pub const BUS_COUNT: usize = 8;

/// The smoothing factor for reward rate changes. The reward rate cannot change by more or less
/// than a factor of this constant from one epoch to the next.
pub const SMOOTHING_FACTOR: u64 = 2;

// Assert MAX_EPOCH_REWARDS is evenly divisible by BUS_COUNT.
static_assertions::const_assert!(
    (MAX_EPOCH_REWARDS / BUS_COUNT as u64) * BUS_COUNT as u64 == MAX_EPOCH_REWARDS
);

/// The seed of the bus account PDA.
pub const BUS: &[u8] = b"bus";

/// The seed of the config account PDA.
pub const CONFIG: &[u8] = b"config";

/// The seed of the metadata account PDA.
pub const METADATA: &[u8] = b"metadata";

/// The seed of the mint account PDA.
pub const MINT: &[u8] = b"mint";

/// The seed of proof account PDAs.
pub const PROOF: &[u8] = b"proof";

/// The seed of the treasury account PDA.
pub const TREASURY: &[u8] = b"treasury";

/// Noise for deriving the mint pda
pub const MINT_NOISE: [u8; 16] = [
    166, 199, 85, 221, 225, 119, 21, 185, 160, 82, 242, 237, 194, 84, 250, 252,
];

/// The name for token metadata.
pub const METADATA_NAME: &str = "Ore";

/// The ticker symbol for token metadata.
pub const METADATA_SYMBOL: &str = "ORE";

/// The uri for token metdata.
pub const METADATA_URI: &str = "https://ore.supply/metadata.json";

/// Program id for const pda derivations
const PROGRAM_ID: [u8; 32] = unsafe { *(&crate::id() as *const Pubkey as *const [u8; 32]) };

/// The addresses of the bus accounts.
pub const BUS_ADDRESSES: [Pubkey; BUS_COUNT] = array_const_fn_init![const_bus_address; 8];

/// Function to derive const bus addresses.
const fn const_bus_address(i: usize) -> Pubkey {
    Pubkey::new_from_array(ed25519::derive_program_address(&[BUS, &[i as u8]], &PROGRAM_ID).0)
}

/// The address of the config account.
pub const CONFIG_ADDRESS: Pubkey =
    Pubkey::new_from_array(ed25519::derive_program_address(&[CONFIG], &PROGRAM_ID).0);

/// The address of the mint metadata account.
pub const METADATA_ADDRESS: Pubkey = Pubkey::new_from_array(
    ed25519::derive_program_address(
        &[
            METADATA,
            unsafe { &*(&mpl_token_metadata::ID as *const Pubkey as *const [u8; 32]) },
            unsafe { &*(&MINT_ADDRESS as *const Pubkey as *const [u8; 32]) },
        ],
        unsafe { &*(&mpl_token_metadata::ID as *const Pubkey as *const [u8; 32]) },
    )
    .0,
);

/// The address of the mint account.
pub const MINT_ADDRESS: Pubkey =
    Pubkey::new_from_array(ed25519::derive_program_address(&[MINT, &MINT_NOISE], &PROGRAM_ID).0);

/// The address of the v1 mint account.
pub const MINT_V1_ADDRESS: Pubkey = pubkey!("oreoN2tQbHXVaZsr3pf66A48miqcBXCDJozganhEJgz");

/// The address of the treasury account.
pub const TREASURY_ADDRESS: Pubkey =
    Pubkey::new_from_array(ed25519::derive_program_address(&[TREASURY], &PROGRAM_ID).0);

/// The bump of the treasury account, for cpis.
pub const TREASURY_BUMP: u8 = ed25519::derive_program_address(&[TREASURY], &PROGRAM_ID).1;
