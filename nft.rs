use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::token::spl_token;
use anchor_spl::token::TokenAccount;

use crate::state::DreamerState;
use crate::state::SwappedState;
// use crate::errors::DreamersError;

#[derive(Accounts)]
pub struct TransferTokenDreamer<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub pda_dreamer: Account<'info, DreamerState>,
    #[account(mut)]
    pub token_source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_destination: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub program_token: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct TransferTokenSwap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub pda_swapped: Account<'info, SwappedState>,
    #[account(mut)]
    pub token_source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_destination: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub program_token: UncheckedAccount<'info>,
}

pub fn transfer_nft_dreamer(ctx: Context<TransferTokenDreamer>) -> Result<()> {
    msg!(format!("Transferring NFT From Dreamer Account").as_str());

    let accounts_info = vec![
        ctx.accounts.program_token.to_account_info(),
        ctx.accounts.token_source.to_account_info(),
        ctx.accounts.token_destination.to_account_info(),
        ctx.accounts.pda_dreamer.to_account_info(),
    ];

    invoke_signed(
        &spl_token::instruction::transfer(
            &ctx.accounts.program_token.key(),
            &ctx.accounts.token_source.key(),
            &ctx.accounts.token_destination.key(),
            &ctx.accounts.pda_dreamer.key(),
            &[],
            1,
        )?,
        accounts_info.as_slice(),
        &[&[
            b"dreamer",
            &[ctx.accounts.pda_dreamer.id],
            ctx.accounts.pda_dreamer.authority.key().as_ref(),
            &[ctx.accounts.pda_dreamer.bump],
        ]],
    )?;

    Ok(())
}

pub fn transfer_nft_swap(ctx: Context<TransferTokenSwap>) -> Result<()> {
    msg!(format!("Transferring NFT From Swap Account").as_str());

    let accounts_info = vec![
        ctx.accounts.program_token.to_account_info(),
        ctx.accounts.token_source.to_account_info(),
        ctx.accounts.token_destination.to_account_info(),
        ctx.accounts.pda_swapped.to_account_info(),
    ];

    invoke_signed(
        &spl_token::instruction::transfer(
            &ctx.accounts.program_token.key(),
            &ctx.accounts.token_source.key(),
            &ctx.accounts.token_destination.key(),
            &ctx.accounts.pda_swapped.key(),
            &[],
            1,
        )?,
        accounts_info.as_slice(),
        &[&[
            b"swapped",
            ctx.accounts.pda_swapped.authority.key().as_ref(),
            &[ctx.accounts.pda_swapped.bump],
        ]],
    )?;

    Ok(())
}