use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;
use std::mem::size_of;
use anchor_lang::solana_program::log::{
    sol_log_compute_units
};
// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("Rkv9eZwXuDZezacPK5SRSQyqbaYKa7upXuF4kr949ri");


const User_Name_Length: usize = 100;
const Viedo_Url_Length: usize = 225;
const User_Url_Length: usize = 225;

#[program]
mod hello_anchor {
    use super::*;
   pub fn create_user(
       ctx:Context<CreateUser>,
       name:String,
       profile_url:String) -> ProgramResult {
       let user = &mut ctx.accounts.user;
       user.user_wallet_address = ctx.accounts.authority.key();
       user.user_name = name;
       user.user_profile_image_url = profile_url;

       msg!("User Added!");
       sol_log_compute_units();
       
        Ok(())
   }
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    #[account(
        init,
        seeds = [b"user".as_ref(),authority.key().as_ref()],
        bump,
        payer = authority,
        space = size_of::<UserAccount>() + User_Name_Length + Viedo_Url_Length + 8 )]
    pub user:Account<'info,UserAccount>,

    #[account(mut)]
    pub authority:Signer<'info>,

    pub system_program:UncheckedAccount<'info>,

    pub clock:Sysvar<'info,Clock>
}

#[account]
pub struct UserAccount {
    pub user_name:String,
    pub user_wallet_address:Pubkey,
    pub user_profile_image_url:String
}