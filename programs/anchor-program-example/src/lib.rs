#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("FhjBVpnwE8R6wBLS1X2YEfTXzjednen5MDFUDoeoiJG7");

#[program]
pub mod anchor_program_example {
    use super::*;

    pub fn create_user(ctx: Context<CreateUserContext>, name: String) -> Result<()> {
        *ctx.accounts.user_account = UserState {
            bump: ctx.bumps.user_account,
            user: ctx.accounts.user.key(),
            name,
        };
        Ok(())
    }
    
    pub fn close_user(_ctx: Context<CloseUserContext>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseUserContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"USER",
            user.key().as_ref(),
        ],
        bump = user_account.bump,
        close = user, // close account and return lamports to user
    )]
    pub user_account: Account<'info, UserState>,
}

#[derive(Accounts)]
pub struct CreateUserContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = UserState::INIT_SPACE,
        seeds = [
            b"USER",
            user.key().as_ref(),
        ],
        bump
    )]
    pub user_account: Account<'info, UserState>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct UserState {
    pub bump: u8,     // 1 byte
    pub user: Pubkey, // 32 bytes
    #[max_len(50)] // set a max length for the string
    pub name: String, // 4 bytes + 50 bytes
}
