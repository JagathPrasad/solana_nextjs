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

const Text_Length: usize = 1024;
const Number_Of_Allowed_Likes_Space: usize = 5;

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

    pub fn create_video(ctx:Context<CreateVideo>,
                        description:String,
                        video_url:String,
                        creator_name:String,
                        creator_url:String)->ProgramResult{
        let video : &mut Account<VideoAccount> = &mut ctx.accounts.video;
        video.authority = ctx.accounts.authority.key();
        video.description = description;
        video.video_url = video_url;
        video.creator_name = creator_name;
        video.creator_url = creator_url;
        video.comment_count = 0;
        video.creator_time = ctx.accounts.clock.unix_timestamp;
        video.likes =0;
        Ok(())
    }

    pub fn create_comment(ctx:Context<CreateComment>
                         ,text:String,commenter_name:String
                         ,commerter_url:String)->ProgramResult{
        let video:&mut Account<VideoAccount> = &mut ctx.accounts.video;
        let comment :&mut Account<CommentAccount> = &mut ctx.accounts.comment;
        comment.authority = ctx.accounts.authority.key();
        comment.commenter_name = commenter_name;
        comment.commerter_url = commenter_url;
        comment.text = text;
        comment.index = video.comment_count;
        comment.video_time = ctx.accounts.clock.unix_timestamp;
        video.comment_count +=1;
        Ok(())
        }

    pub fn like_video(ctx:Context<CreateLike>)->ProgramResult{
        let video: &mut Account<VideoAccount> = &mut ctx.accounts.video;
        let like:&mut Account<LikeAccount> = &mut ctx.account.like;
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


#[derive(Accounts)]
pub struct CreateVideo<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    #[account(
        init,
        seeds = [b"video".as_ref(),randomkey.key().as_ref()],
        bump,
        payer = authority,
        space = size_of::<VideoAccount>() + Text_Length + User_Name_Length +User_Url_Length 
        + Viedo_Url_Length + 8 +32 + Number_Of_Allowed_Likes_Space )]
    pub video:Account<'info,VideoAccount>,

    #[account(mut)]
    pub randomkey:AccountInfo<'info>,

     #[account(mut)]
    pub authority:Signer<'info>,
    
    pub system_program:UncheckedAccount<'info>,

    pub clock:Sysvar<'info,Clock>
}

#[derive(Account)]
pub struct CreateComment<'info>{

    #[account(mut)]
    pub video:Account<VideoAccount>,
    #[account(
        init,
        seeds=[b"comment".as_ref(),video.key().as_ref(),video.comment_count.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        space = size_of::<CommentAccount>() + Text_Length + User_Name_Length + User_Url_Length 
        + Video_Url_Length)]
   pub comment:Account<CommentAccount>,
    
    #[account(mut)]
    pub authority:Signer<'info>,
    
    pub system_program:UncheckedAccount<'info,System>,

    pub clock:Sysvar<'info,Clock>
}

#[derive(Account)]
pub struct CreateLike<'info>{
    #[account(mut)]
    pub video:Account<VideoAccount>,
    #[account(
        init,
        seed=[b"like".as_ref(),video.key().as_ref()],
        bump,
    payer = authority,
    space)]
    pub like:Account<LikeAccount>,
     #[account(mut)]
    pub authority:Signer<'info>,
    
    pub system_program:UncheckedAccount<'info,System>,

    pub clock:Sysvar<'info,Clock>
}

#[account]
pub struct UserAccount {
    pub user_name:String,
    pub user_wallet_address:Pubkey,
    pub user_profile_image_url:String
}

#[account]
pub struct VideoAccount {
    pub authority : Pubkey,
    pub description:String,
    pub video_url:String,
    pub creator_name:String,
    pub creator_url:String,
    pub comment_count : u64,
    pub index:u64,
    pub creator_time: i64,
    pub people_liked:Vec<Pubkey>,
    pub likes:u8,
    pub remove:i64
    
}

#[account]
pub struct CommentAccount{
    pub authority:Pubkey,
    pub text:String,
    pub commenter_name :String,
    pub commerter_url :String,
    pub index:u64,
    pub video_time:i64
}

#[account]
pub struct LikeAccount{
     pub authority:Pubkey,
    
}