use anchor_lang::prelude::*;
use solana_program::native_token::LAMPORTS_PER_SOL;

pub const FUNDING_AMOUNT: u64 = LAMPORTS_PER_SOL;
pub const WSOL_ID: Pubkey = solana_program::pubkey!("So11111111111111111111111111111111111111112");
pub const LOCK_CPMM_AUTHORITY: Pubkey = solana_program::pubkey!("3f7GcQFG397GAaEnv51zR6tsTVihYRydnydDD1cXekxH");
pub const DEFAULT_DECIMALS: u8 = 6;
pub const DEFAULT_SUPPLY: u64 = 1_000_000_000_000_000;
