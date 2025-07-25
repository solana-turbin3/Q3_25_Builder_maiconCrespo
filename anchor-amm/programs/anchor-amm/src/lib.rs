#![allow(deprecated,unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use states::*;

declare_id!("CCpTvJaqt8P7wJAx9RZLvJvFpHrooSfW8mB2X6qvHpcK");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init(seed, fee, authority, ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)
    }

    pub fn swap(ctx: Context<Swap>, x_to_y: bool, amount_in: u64, slippage: u16) -> Result<()> {
        ctx.accounts.swap(x_to_y, amount_in, slippage)
    }
}