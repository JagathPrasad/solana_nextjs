use anchor_lang::prelude::*;
use std::mem::size_of;
// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("GCm7dxw1MRUi38tx8xnDR1RPv45a5k2Fqv8cPCwrwaRo");

#[program]
mod lottery {
    use super::*;
    pub fn init_master(ctx: Context<InitMaster>) -> Result<()> {
        //let new_account =  &mut ctx.accounts.master_account;
        //new_account.last_id = 0;
        Ok(())
    }

    pub fn create_lottery(ctx: Context<CreateLottery>) -> Result<()> {
        //let new_account =  &mut ctx.accounts.master_account;
        //new_account.last_id = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMaster<'info> {
    #[account(init,
              payer = authority,
              seeds=[b"master".as_ref(),authority.key().as_ref()],
              bump,
              space = size_of::<MasterAccount>() + 8
             )]
    pub master_account: Account<'info, MasterAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateLottery<'info> {
    #[account(mut)]
    pub master: Account<'info, MasterAccount>,

    #[account(init,
             payer=authority,
             seeds=[b"lottery".as_ref(),authority.key().as_ref()],
             bump,
             space= size_of::<LotteryAccount>() + 8)]
    pub lotter: Account<'info, LotteryAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MasterAccount {
    pub last_id: u32,
}

#[account]
pub struct LotteryAccount {
    pub id: u32,
    pub authority: Pubkey,
    pub ticket_price: u64,
    pub last_ticket_id: u32,
    pub winner_id: Option<u32>,
    pub claimed: bool,
}
