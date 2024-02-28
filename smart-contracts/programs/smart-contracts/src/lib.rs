use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("HYCWfDk8ZU8SF5oF9CE2CyChgBU6yndn82Umo4iE1Q9T");

#[program]
pub mod crowdFund {
    use super::*;

    //creates a campaign
    pub fn create(ctx: Context<Create>, name: String, description: String, target_amount: u64, project_url: String, progress_update_url:String, project_image_url:String, category:String) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        campaign.name = name;
        campaign.description = description;
        campaign.target_amount=target_amount;
        campaign.project_url=project_url;
        campaign.progress_update_url=progress_update_url;
        campaign.project_image_url=project_image_url;
        campaign.category=category;
        campaign.amount_donated = 0;
        campaign.amount_withdrawn=0;
        campaign.admin = *ctx.accounts.user.key;
        Ok(())
    }
        //Withdraw from a campaign
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        let user = &mut ctx.accounts.user;
        //restricts Withdrawal to campaign admin
        if campaign.admin != *user.key {
             return Err(ProgramError::IncorrectProgramId);
         }
        let rent_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());
        if **campaign.to_account_info().lamports.borrow() - rent_balance < amount {
             return Err(ProgramError::InsufficientFunds);
         }
        **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;
        (&mut ctx.accounts.campaign).amount_withdrawn += amount;
        Ok(())
        }
}
