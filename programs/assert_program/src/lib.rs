use anchor_lang::prelude::*;

declare_id!("CrGmBj9LobsbkVHXPguToqmyZoPgAAtf1gbf5795t91W");

#[program]
pub mod assert_program {
    use super::*;

    pub fn assert_balance_gte(ctx: Context<TokenAccount>, amount: u64) -> anchor_lang::Result<()> {
        require_gte!(ctx.accounts.account.amount, amount);
        Ok(())
    }

    pub fn assert_balance_lte(ctx: Context<TokenAccount>, amount: u64) -> anchor_lang::Result<()> {
        require_gt!(amount, ctx.accounts.account.amount);
        Ok(())
    }}

#[derive(Accounts)]
pub struct TokenAccount<'info> {
    #[account(mut)]
    pub account: Account<'info, anchor_spl::token::TokenAccount>,
}
