use anchor_lang::prelude::*;
use std::mem::size_of;
// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("GCm7dxw1MRUi38tx8xnDR1RPv45a5k2Fqv8cPCwrwaRo");

pub const MASTER_SEED: &str = "master";
pub const LOTTERY_SEED: &str = "lottery";
pub const TICKET_SEED: &str = "ticket";

#[program]
mod lottery {
    use super::*;
    pub fn init_master(ctx: Context<InitMaster>) -> Result<()> {
        //let new_account =  &mut ctx.accounts.master_account;
        //new_account.last_id = 0;
        Ok(())
    }

    pub fn create_lottery(ctx: Context<CreateLottery>, ticket_price: u64) -> Result<()> {
        let master = &mut ctx.accounts.master;
        let lottery = &mut ctx.accounts.lottery;
        master.last_id += 1;
        lottery.id = master.last_id;
        lottery.authority = ctx.accounts.authority.key();
        lottery.ticket_price = ticket_price;
        lottery.last_ticket_id = master.last_id;
        //lottery.winner_id = master.last_id;
        lottery.claimed = false;

        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        let buyticket = &mut ctx.accounts.ticket;
        let payer = &ctx.accounts.authority;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMaster<'info> {
    #[account(init,
              payer = authority,
              seeds=[MASTER_SEED.as_bytes()],
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
             seeds=[LOTTERY_SEED.as_bytes(),&(master.last_id + 1).to_le_bytes()],
             bump,
             space= size_of::<LotteryAccount>() + 8)]
    pub lottery: Account<'info, LotteryAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(lottery_id:u32)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub lottery: Account<'info, LotteryAccount>,

    #[account(init,
             payer=authority,
             seeds=[TICKET_SEED.as_bytes(),lotter.key().as_ref(),&(lottery.last_ticket_id + 1).to_le_bytes()],
             bump,
             space=size_of::<TicketAccount>() + 8)]
    pub ticket: Account<'info, TicketAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MasterAccount {
    pub last_id: u32, //4
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

#[account]
pub struct TicketAccount {
    pub id: u32,
    pub authority: Pubkey,
    pub lottery_id: u32,
}
