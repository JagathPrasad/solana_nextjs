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
        let user_profile = &mut ctx.accounts.user_profile;
        airbnb_account.idx = ctx.accounts.user_profile.last_airbnb;
        airbnb_account.authority = ctx.accounts.authority.key();
        airbnb_account.location = location;
        airbnb_account.country = country;
        airbnb_account.price = price;
        airbnb_account.image = image;
        airbnb_account.isReserved = true;
        user_profile.last_airbnb = user_profile.last_airbnb.checked_add(1).unwrap();
        user_profile.airbnb_count = user_profile.airbnb_count.checked_add(1).unwrap(); 

        Ok(())
    }

    pub fun update_airbnb(
        ctx:Context<UpdateAirbnb>,
        airbnb_idx:u8,
        location:String,
        isreserved:bool,
        country:String,
        price:String,
        img:String)->Result<()>{
        let airbnb_account = &mut ctx.accounts.airbnb_account;
        //let user_profile = &mut ctx.accounts.user_profile;
        airbnb_account.location = location;
        airbnb_account.isReserved = isreserved;
        airbnb_account.country = country;
        airbnb_account.price = price;
        airbnb_account.image = img;
        Ok(())
    }

    pub fun remove_airbnb(ctx:Context<RemoveAirbnb>, isreserved:bool)->Result<()>{

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
pub struct AddAirbnb<'info>{
    #[account(mut)]
    pub authority:Signer<'info>,

    #[account(mut)]
    pub user_profile: Account<'info,UserProfile>,

    #[account(
        init,
        seeds=[AIRBNB_TAG.as_ref(),authority.key().as_ref(), &mut[user_profile.last_airbnb].as_ref()],
        bump,
        payer = authority,
        space = 2865 + 8
    )]
    pub airbnb_account:Box<Account<'info,AirbnbAccount>>,
    
    pub system_program:Program<'info,System>
    
    
}


#[derive(Accounts)]
#[instruction(airbnb_idx:u8)]
pub struct UpdateAirbnb<'info>{
    #[account(mut)]
    pub user_profile:Account<'info>


    // #[account(
    //     init,
    //     seeds=[AIRBNB_TAG.as_ref(),authority.key().as_ref(), &mut[user_profile.last_airbnb].as_ref()],
    //     bump,
    //     payer = authority,
    //     space = 2865 + 8
    // )]
    #[account(mut)]
    pub airbnb_account:Account<'info>
  
    

    #[account(mut)]
    pub authority:Signer<'info>

    pub system_program:Program<'info,System>

}

#[derive(Accounts)]
#[instruction]
pub struct RemoveAirbnb{
    pub 
}