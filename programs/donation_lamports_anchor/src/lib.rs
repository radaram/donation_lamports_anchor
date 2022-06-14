use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


#[program]
pub mod donation_lamports_anchor {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let storage_account = &mut ctx.accounts.storage_account;
        storage_account.owner = "3mzC56NqGSrZZSTRkY2ya4zNcYkZjY6Pg2F47qrJ9ECd".parse().unwrap();
        Ok(())
    }

    pub fn donate(ctx: Context<Donation>, amount: u64) -> Result<()> {
        let storage_account = &mut ctx.accounts.storage_account;
        let user = &ctx.accounts.user;
        let system_account = &ctx.accounts.system_program;
        
        let donate_details = DonateDetails {
            user: user.key(),
            amount,
        };

        storage_account.transfers.push(donate_details);

        invoke(
            &transfer(
                &user.key(), 
                &storage_account.key(), 
                amount
            ),
            &[
                user.to_account_info(), 
                storage_account.to_account_info(),
                system_account.to_account_info(),
            ],
        ).unwrap();

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> ProgramResult {
        let storage_account = &ctx.accounts.storage_account.to_account_info();
        let owner = &ctx.accounts.owner;

        **owner.try_borrow_mut_lamports()? += storage_account.lamports();
        **storage_account.try_borrow_mut_lamports()? = 0;

        Ok(())
    }
}


#[account]
pub struct StorageAccount {
    pub transfers: Vec<DonateDetails>,
    pub owner: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DonateDetails {
    user: Pubkey,
    amount: u64,
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 1024)]
    pub storage_account: Account<'info, StorageAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct Donation<'info> {
    #[account(mut)]
    pub storage_account: Account<'info, StorageAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, signer)]
    pub storage_account: Account<'info, StorageAccount>,

    #[account(address = storage_account.owner)]
    pub owner: Signer<'info>,
}

