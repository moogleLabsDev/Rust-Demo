use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use mpl_token_metadata::instruction::MetadataInstruction;
use mpl_token_metadata::instruction::TransferArgs;
use solana_program::instruction::Instruction;
use solana_program::program::{invoke, invoke_signed};

use crate::errors::ErrorCode;
use crate::state::DreamerState;
use crate::state::SwappedState;

#[derive(Accounts)]
pub struct RewardPNFTDreamer<'info> {
    #[account(mut)]
    owner: Box<Account<'info, DreamerState>>,
    mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    payer: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    destination: UncheckedAccount<'info>,
    #[account(mut, constraint =
        destination_token_account.owner == destination.key()
        @ ErrorCode::InvalidUserOriginalMintTokenAccount)]
    destination_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    token_record: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    destination_token_record: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    edition: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    authorization_rules: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    sysvar_instructions: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(address = mpl_token_metadata::id())]
    token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    authorization_rules_program: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RewardPNFTSwap<'info> {
    #[account(mut)]
    owner: Box<Account<'info, SwappedState>>,
    mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    payer: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    destination: UncheckedAccount<'info>,
    #[account(mut, constraint =
        destination_token_account.owner == destination.key()
        @ ErrorCode::InvalidUserOriginalMintTokenAccount)]
    destination_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    token_record: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    destination_token_record: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    edition: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    authorization_rules: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    sysvar_instructions: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(address = mpl_token_metadata::id())]
    token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    authorization_rules_program: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RewardPNFTWallet<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    owner: UncheckedAccount<'info>,
    mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    payer: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    destination: UncheckedAccount<'info>,
    #[account(mut, constraint =
        destination_token_account.owner == destination.key()
        @ ErrorCode::InvalidUserOriginalMintTokenAccount)]
    destination_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    token_record: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    destination_token_record: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    edition: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    authorization_rules: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    sysvar_instructions: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(address = mpl_token_metadata::id())]
    token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    authorization_rules_program: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}

pub fn transfer_pnft_dreamer(ctx: Context<RewardPNFTDreamer>) -> Result<()> {
    invoke_signed(
        &Instruction {
            program_id: mpl_token_metadata::id(),
            accounts: vec![
                // #[account(0, writable, name="token", desc="Token account")]
                AccountMeta::new(ctx.accounts.token_account.key(), false),
                // #[account(1, name="token_owner", desc="Token account owner")]
                AccountMeta::new_readonly(ctx.accounts.owner.key(), false),
                // #[account(2, writable, name="destination", desc="Destination token account")]
                AccountMeta::new(ctx.accounts.destination_token_account.key(), false),
                // #[account(3, name="destination_owner", desc="Destination token account owner")]
                AccountMeta::new_readonly(ctx.accounts.destination.key(), false),
                // #[account(4, name="mint", desc="Mint of token asset")]
                AccountMeta::new_readonly(ctx.accounts.mint.key(), false),
                // #[account(5, writable, name="metadata", desc="Metadata (pda of ['metadata', program id, mint id])")]
                AccountMeta::new(ctx.accounts.metadata.key(), false),
                // #[account(6, optional, name="edition", desc="Edition of token asset")]
                AccountMeta::new_readonly(ctx.accounts.edition.key(), false),
                // #[account(7, optional, writable, name="recipient_token_record", desc="Owner token record account")]
                AccountMeta::new(ctx.accounts.token_record.key(), false),
                // #[account(8, optional, writable, name="destination_token_record", desc="Destination token record account")]
                AccountMeta::new(ctx.accounts.destination_token_record.key(), false),
                // #[account(9, signer, name="authority", desc="Transfer authority (token owner or delegate)")]
                AccountMeta::new_readonly(ctx.accounts.owner.key(), true),
                // #[account(10, signer, writable, name="payer", desc="Payer")]
                AccountMeta::new(ctx.accounts.payer.key(), true),
                // #[account(11, name="system_program", desc="System Program")]
                AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
                // #[account(12, name="sysvar_instructions", desc="Instructions sysvar account")]
                AccountMeta::new_readonly(ctx.accounts.sysvar_instructions.key(), false),
                // #[account(13, name="spl_token_program", desc="SPL Token Program")]
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                // #[account(14, name="spl_ata_program", desc="SPL Associated Token Account program")]
                AccountMeta::new_readonly(ctx.accounts.associated_token_program.key(), false),
                // #[account(15, optional, name="authorization_rules_program", desc="Token Authorization Rules Program")]
                AccountMeta::new_readonly(ctx.accounts.authorization_rules_program.key(), false),
                // #[account(16, optional, name="authorization_rules", desc="Token Authorization Rules account")]
                AccountMeta::new_readonly(ctx.accounts.authorization_rules.key(), false),
            ],
            data: MetadataInstruction::Transfer(TransferArgs::V1 {
                amount: 1,
                authorization_data: None,
            })
            .try_to_vec()
            .unwrap(),
        },
        &[
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.destination_token_account.to_account_info(),
            ctx.accounts.destination.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.edition.to_account_info(),
            ctx.accounts.token_record.to_account_info(),
            ctx.accounts.destination_token_record.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.sysvar_instructions.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.associated_token_program.to_account_info(),
            ctx.accounts.authorization_rules_program.to_account_info(),
            ctx.accounts.authorization_rules.to_account_info(),
        ],
        &[&[
            b"dreamer",
            &[*&ctx.accounts.owner.id],
            *&ctx.accounts.owner.authority.key().as_ref(),
            &[*&ctx.accounts.owner.bump],
        ]],
    )?;
    Ok(())
}

pub fn transfer_pnft_swap(ctx: Context<RewardPNFTSwap>) -> Result<()> {
    invoke_signed(
        &Instruction {
            program_id: mpl_token_metadata::id(),
            accounts: vec![
                // #[account(0, writable, name="token", desc="Token account")]
                AccountMeta::new(ctx.accounts.token_account.key(), false),
                // #[account(1, name="token_owner", desc="Token account owner")]
                AccountMeta::new_readonly(ctx.accounts.owner.key(), false),
                // #[account(2, writable, name="destination", desc="Destination token account")]
                AccountMeta::new(ctx.accounts.destination_token_account.key(), false),
                // #[account(3, name="destination_owner", desc="Destination token account owner")]
                AccountMeta::new_readonly(ctx.accounts.destination.key(), false),
                // #[account(4, name="mint", desc="Mint of token asset")]
                AccountMeta::new_readonly(ctx.accounts.mint.key(), false),
                // #[account(5, writable, name="metadata", desc="Metadata (pda of ['metadata', program id, mint id])")]
                AccountMeta::new(ctx.accounts.metadata.key(), false),
                // #[account(6, optional, name="edition", desc="Edition of token asset")]
                AccountMeta::new_readonly(ctx.accounts.edition.key(), false),
                // #[account(7, optional, writable, name="recipient_token_record", desc="Owner token record account")]
                AccountMeta::new(ctx.accounts.token_record.key(), false),
                // #[account(8, optional, writable, name="destination_token_record", desc="Destination token record account")]
                AccountMeta::new(ctx.accounts.destination_token_record.key(), false),
                // #[account(9, signer, name="authority", desc="Transfer authority (token owner or delegate)")]
                AccountMeta::new_readonly(ctx.accounts.owner.key(), true),
                // #[account(10, signer, writable, name="payer", desc="Payer")]
                AccountMeta::new(ctx.accounts.payer.key(), true),
                // #[account(11, name="system_program", desc="System Program")]
                AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
                // #[account(12, name="sysvar_instructions", desc="Instructions sysvar account")]
                AccountMeta::new_readonly(ctx.accounts.sysvar_instructions.key(), false),
                // #[account(13, name="spl_token_program", desc="SPL Token Program")]
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                // #[account(14, name="spl_ata_program", desc="SPL Associated Token Account program")]
                AccountMeta::new_readonly(ctx.accounts.associated_token_program.key(), false),
                // #[account(15, optional, name="authorization_rules_program", desc="Token Authorization Rules Program")]
                AccountMeta::new_readonly(ctx.accounts.authorization_rules_program.key(), false),
                // #[account(16, optional, name="authorization_rules", desc="Token Authorization Rules account")]
                AccountMeta::new_readonly(ctx.accounts.authorization_rules.key(), false),
            ],
            data: MetadataInstruction::Transfer(TransferArgs::V1 {
                amount: 1,
                authorization_data: None,
            })
            .try_to_vec()
            .unwrap(),
        },
        &[
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.destination_token_account.to_account_info(),
            ctx.accounts.destination.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.edition.to_account_info(),
            ctx.accounts.token_record.to_account_info(),
            ctx.accounts.destination_token_record.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.sysvar_instructions.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.associated_token_program.to_account_info(),
            ctx.accounts.authorization_rules_program.to_account_info(),
            ctx.accounts.authorization_rules.to_account_info(),
        ],
        &[&[
            b"swapped",
            *&ctx.accounts.owner.authority.key().as_ref(),
            &[*&ctx.accounts.owner.bump],
        ]],
    )?;
    Ok(())
}

pub fn transfer_pnft(ctx: Context<RewardPNFTWallet>) -> Result<()> {
    invoke(
        &Instruction {
            program_id: mpl_token_metadata::id(),
            accounts: vec![
                // #[account(0, writable, name="token", desc="Token account")]
                AccountMeta::new(ctx.accounts.token_account.key(), false),
                // #[account(1, name="token_owner", desc="Token account owner")]
                AccountMeta::new_readonly(ctx.accounts.owner.key(), false),
                // #[account(2, writable, name="destination", desc="Destination token account")]
                AccountMeta::new(ctx.accounts.destination_token_account.key(), false),
                // #[account(3, name="destination_owner", desc="Destination token account owner")]
                AccountMeta::new_readonly(ctx.accounts.destination.key(), false),
                // #[account(4, name="mint", desc="Mint of token asset")]
                AccountMeta::new_readonly(ctx.accounts.mint.key(), false),
                // #[account(5, writable, name="metadata", desc="Metadata (pda of ['metadata', program id, mint id])")]
                AccountMeta::new(ctx.accounts.metadata.key(), false),
                // #[account(6, optional, name="edition", desc="Edition of token asset")]
                AccountMeta::new_readonly(ctx.accounts.edition.key(), false),
                // #[account(7, optional, writable, name="recipient_token_record", desc="Owner token record account")]
                AccountMeta::new(ctx.accounts.token_record.key(), false),
                // #[account(8, optional, writable, name="destination_token_record", desc="Destination token record account")]
                AccountMeta::new(ctx.accounts.destination_token_record.key(), false),
                // #[account(9, signer, name="authority", desc="Transfer authority (token owner or delegate)")]
                AccountMeta::new_readonly(ctx.accounts.owner.key(), true),
                // #[account(10, signer, writable, name="payer", desc="Payer")]
                AccountMeta::new(ctx.accounts.payer.key(), true),
                // #[account(11, name="system_program", desc="System Program")]
                AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
                // #[account(12, name="sysvar_instructions", desc="Instructions sysvar account")]
                AccountMeta::new_readonly(ctx.accounts.sysvar_instructions.key(), false),
                // #[account(13, name="spl_token_program", desc="SPL Token Program")]
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                // #[account(14, name="spl_ata_program", desc="SPL Associated Token Account program")]
                AccountMeta::new_readonly(ctx.accounts.associated_token_program.key(), false),
                // #[account(15, optional, name="authorization_rules_program", desc="Token Authorization Rules Program")]
                AccountMeta::new_readonly(ctx.accounts.authorization_rules_program.key(), false),
                // #[account(16, optional, name="authorization_rules", desc="Token Authorization Rules account")]
                AccountMeta::new_readonly(ctx.accounts.authorization_rules.key(), false),
            ],
            data: MetadataInstruction::Transfer(TransferArgs::V1 {
                amount: 1,
                authorization_data: None,
            })
            .try_to_vec()
            .unwrap(),
        },
        &[
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.destination_token_account.to_account_info(),
            ctx.accounts.destination.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.edition.to_account_info(),
            ctx.accounts.token_record.to_account_info(),
            ctx.accounts.destination_token_record.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.sysvar_instructions.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.associated_token_program.to_account_info(),
            ctx.accounts.authorization_rules_program.to_account_info(),
            ctx.accounts.authorization_rules.to_account_info(),
        ],
    )?;
    Ok(())
}