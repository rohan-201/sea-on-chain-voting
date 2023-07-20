use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::token;
use std::convert::TryFrom;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(Debug)]
#[account]
pub struct VoteBank {
    is_open_to_vote: bool,
    gm: u64,
    gn: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, PartialEq)]
pub enum VoteType {
    GM,
    GN,
}

pub fn init_vote_bank_handler(mut ctx: Context<InitVoteBank>) -> Result<()> {
    let mut signer = &mut ctx.accounts.signer;
    let mut vote_account = &mut ctx.accounts.vote_account;
    let mut vote_account = vote_account;

    vote_account.is_open_to_vote = true;

    Ok(())
}

pub fn gib_vote_handler(mut ctx: Context<GibVote>, mut vote_type: VoteType) -> Result<()> {
    let mut vote_account = &mut ctx.accounts.vote_account;

    if vote_type == VoteType::GM {
        msg!("{}", "Voted for GM");

        vote_account.gm += 1;
    } else {
        if vote_type == VoteType::GN {
            msg!("{}", "Voted for GN");

            vote_account.gn += 1;
        }
    }

    Ok(())
}

#[derive(Accounts)]
pub struct InitVoteBank<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = ["vote_account".as_bytes().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<VoteBank>()
    )]
    pub vote_account: Box<Account<'info, VoteBank>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GibVote<'info> {
    #[account(mut)]
    pub vote_account: Box<Account<'info, VoteBank>>,
}

#[program]
pub mod sea_on_chain_voting {
    use super::*;

    pub fn init_vote_bank(ctx: Context<InitVoteBank>) -> Result<()> {
        init_vote_bank_handler(ctx)
    }

    pub fn gib_vote(ctx: Context<GibVote>, vote_type: VoteType) -> Result<()> {
        gib_vote_handler(ctx, vote_type)
    }
}
