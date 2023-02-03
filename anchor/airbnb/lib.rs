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
        let user_profile = ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.last_airbnb = 0;
        user_profile.airbnb_count = 0;
        Ok(())
    }

    pub fn add_airbnb(ctx:Context<AddAirbnb>,
                    location:String,
                    country:String,
                    price:String,
                    image:String)->Result<()>{
        let airbnb_account = &mut ctx.accounts.airbnb_account;
        airbnb_account.authority = ctx.accounts.authority.key();
        airbnb_account.location = location;
        airbnb_account.country = country;
        airbnb_account.price = price;
        airbnb_account.image = image;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seed=[USER_TAG,authority.key().as_ref()],
        bump,
        payer = authority,
        space = 32+1+1+8
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction]
pub struct AddAirbnb{
    #[account(mut)]
    pub authority:Signer<'info>,

    #[account(mut)]
    pub user_profile: Account<'info,UserProfile>,

    #[account(
        init,
        seeds=[b"Airbnb".as_ref(),authority.key().as_ref(), &mut[user_profile.last_airbnb].as_ref()],
        bump,
        payer = authority,
        space = 2865 + 8
    )]
    pub airbnb_account:Box<Account<'info,AirbnbAccount>>,
    
    pub system_program:Program<'info,System>
    
    
}