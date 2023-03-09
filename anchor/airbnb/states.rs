use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile {
    pub authority: Pubkey,
    pub last_airbnb: u8,
    pub airbnb_count: u8,
}


#[account]
#[derive(Default)]
pub struct AirbnbAccount{
    pub idx:u8,
    pub authority:Pubkey,
    pub location:String,
    pub country:String,
    pub price:String,
    pub image:String,
    pub isReserved:bool

}