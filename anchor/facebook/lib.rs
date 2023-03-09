use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("5kZbDPCS9wyyaJnoguKRCPUq2yiLBE8ET1vGhzxkxq8j");

const TEXT_LENGTH:usize = 1024;
const USER_NAME_LENGTH:usize = 100;
const USER_URL_LENGTH:usize = 255

#[program]
mod hello_anchor {
    use super::*;
    pub fn create_state(ctx: Context<CreateState>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = ctx.authority.key();
        state.post_count = 0;
        Ok(())
    }

    pub fn create_post(ctx:Context<CreatePost>,text:String,post_url:String,post_name:String)->Result<()>{
        let state = &mut ctx.accounts.state;
        let post = &mut ctx.accounts.post;
        post.authority = state.authority.key();
        post.text = text;
        post.post_url = post_url;
        post.post_name = post_name;
        post.comment_count = 0;
        post.index = state.post_count;
        post.post_time = "";
        Ok(())
    }

    pub fn create_comment(ctx:Context<CreateComment>,text:String)->Result<()>{
        
        Ok(())
    }
}


#[derive(Accounts)]
pub struct CreateState<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    #[account(init,
              seeds=[b'state'.as_ref()],
              bump,              
              payer = authority,
              space = size_of::<StateAccount>() + 8
             )]
    pub state: Account<'info, StateAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(contraint = token_program.key == &token::ID)]
    pub token_program:Program<'info,Token>
}

#[derive(Accounts)]
pub struct CreatePost<'info>{

    #[account(mut)]
    pub state :Account<'info,StateAccount>,
    #[account(init,
              seeds=[b'post'.as_ref(),state.post_count.to_be_bytes().as_ref()],
              bump,
              payer=authority,
              space=size_of::<PostAccount>() + TEXT_LENGTH + USER_NAME_LENGTH + USER_URL_LENGTH
             )]
    pub post :Account<'info,PostAccount>,

    pub authority:Singer<'info>,
    pub system_program:Program<'info,System>,
    #[account(contraint = token_program.key == &token::ID)]
    pub token_program:Program<'info,Token>
}

#[derive(Accounts)]
pub struct CreateComment<'info>{

    #[account(mut)]
    pub post
}

#[account]
pub struct StateAccount{
    pub authority : Pubkey;
    pub post_count : u64;
}

#[account]
pub struct PostAccount {
    pub authority : Pubkey;
    pub text : String;
    pub post_url : String;
    pub poster_name :String;
    pub comment_count : u64;
    pub index :u64;
    pub post_time : u64;
}


#[account]