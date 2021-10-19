use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

static SEED: &[u8] = b"spacebums";

#[program]
pub mod spl_demo {
    use super::*;
    pub fn initialize_special_pda(
        ctx: Context<InitializePDA>,
        _authority_bump: u8,
    ) -> ProgramResult {
        ctx.accounts.specialpda.trusted_server = ctx.accounts.trustedserver.key();
        Ok(())
    }

    pub fn initialize_bum_base(
        ctx: Context<InitBumBase>,
        _authority_bump: u8,
        cool_name: String,
        cool_description: String,
    ) -> ProgramResult {
        ctx.accounts.hangerpda.cool_name = cool_name;
        ctx.accounts.hangerpda.cool_description = cool_description;
        Ok(())
    }

    pub fn send_base_bums(ctx: Context<SendBaseBums>) -> ProgramResult {
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.physicalbum.to_account_info(),
                    to: ctx.accounts.hangerpda.to_account_info(),
                    authority: ctx.accounts.bumholder.to_account_info(),
                },
            ),
            1,
        )?;
        Ok(())
    }

    pub fn return_bum(ctx: Context<ReturnBum>, authority_bump: u8) -> ProgramResult {
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.hangerpda.to_account_info(),
                    to: ctx.accounts.physicalbum.to_account_info(),
                    authority: ctx.accounts.bumholder.to_account_info(),
                },
                &[&[ctx.accounts.bumholder.key().as_ref(), &[authority_bump]]],
            ),
            1,
        )?;

        anchor_spl::token::close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::CloseAccount {
                account: ctx.accounts.hangerpda.to_account_info(),
                destination: ctx.accounts.bumholder.to_account_info(),
                authority: ctx.accounts.program_authority.to_account_info(),
            },
            &[&[ctx.accounts.bumholder.key().as_ref(), &[authority_bump]]],
        ))?;
        Ok(())
    }

    pub fn distribute_airdrops(ctx: Context<DistributeAirdrops>) -> ProgramResult {
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.airdropped_item.to_account_info(),
                    to: ctx.accounts.bumholder.to_account_info(),
                    authority: ctx.accounts.bumholder.to_account_info(),
                },
            ),
            1,
        )?;
        Ok(())
    }

    pub fn initiate_vote(
        ctx: Context<InitVote>,
        votingtitle: String,
        enddate: i64,
    ) -> ProgramResult {
        ctx.accounts.specialpda.current_vote = votingtitle;
        ctx.accounts.specialpda.end_date = enddate;
        ctx.accounts.specialpda.yes_votes = 0;
        ctx.accounts.specialpda.no_votes = 0;
        Ok(())
    }

    pub fn submit_vote(ctx: Context<SubmitVote>, yes: bool, _nonce: u8) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(authority_bump: u8)]
pub struct InitializePDA<'info> {
    trustedserver: Signer<'info>,
    #[account(init, seeds=[SEED], bump= authority_bump, payer= trustedserver, space= 500)]
    specialpda: Account<'info, SpecialPDA>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(authority_bump: u8)]
pub struct InitBumBase<'info> {
    trustedserver: Signer<'info>,
    #[account(mut)]
    bumholder: Signer<'info>,
    #[account(mut)]
    specialpda: Account<'info, SpecialPDA>,
    #[account(init, seeds = [bumholder.key().as_ref()], bump = authority_bump, payer = trustedserver, space=1000)]
    hangerpda: Account<'info, HangerPDA>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendBaseBums<'info> {
    trustedserver: Signer<'info>,
    #[account(mut)]
    bumholder: Signer<'info>,
    // #[account(mut)]
    // specialpda: Account<'info, SpecialPDA>,
    #[account(mut)]
    physicalbum: Account<'info, TokenAccount>,
    #[account(mut)]
    hangerpda: Account<'info, HangerPDA>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(authority_bump: u8)]
pub struct ReturnBum<'info> {
    trustedserver: Signer<'info>,
    #[account(mut)]
    bumholder: Signer<'info>,
    #[account(mut)]
    physicalbum: Account<'info, TokenAccount>,
    #[account(mut)]
    hangerpda: Account<'info, HangerPDA>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    #[account(seeds = [SEED], bump = authority_bump)]
    program_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(authority_bump: u8)]
pub struct DistributeAirdrops<'info> {
    trustedserver: Signer<'info>,
    #[account(mut)]
    bumholder: AccountInfo<'info>,
    #[account(mut)]
    airdropped_item: Account<'info, TokenAccount>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(authority_bump: u8)]
pub struct InitVote<'info> {
    trustedserver: Signer<'info>,
    #[account(mut)]
    specialpda: Account<'info, SpecialPDA>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(authority_bump: u8)]
pub struct SubmitVote<'info> {
    trustedserver: Signer<'info>,
    #[account(mut)]
    specialpda: Account<'info, SpecialPDA>,
    #[account(mut)]
    bumholder: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

#[account]
#[derive(Default)]
pub struct SpecialPDA {
    trusted_server: Pubkey,
    current_vote: String,
    yes_votes: u16,
    no_votes: u16,
    end_date: i64,
}

#[account]
#[derive(Default)]
pub struct HangerPDA {
    cool_name: String,
    cool_description: String,
}
