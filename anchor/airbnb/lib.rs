use anchor_lang::prelude::*;
pub mod constant;
pub mod states;
use crate::{constant::*, states::*};
// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("2yahttj8sfh2Ge1EK2biPeMrppPhWgrRqycX27AvshJD");

#[program]
pub mod preline_airbnb {
    use super::*;
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        //let user_profile = ctx.accounts.user_profile;
        //user_profile.authority = ctx.accounts.authority.key();
        //msg!("Changed data to: {}!", data); // Message will show up in the tx logs
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init,seed=[USER_TAG,authority.key().as_ref()],bump, payer = authority, space = 32+1+1+8)]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}
