use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use spl_token::instruction::transfer_checked;
// use anchor_spl::token::{Mint, Token, TokenAccount};

// use spl_associated_token_account::create_associated_token_account;
// use std::str::FromStr;

use error::CommonError;

pub mod error;

declare_id!("E2uTyMCm7XXnykZ4BSixXgQsTYVhjcNc1ahHVMhVJt1S");

#[program]
pub mod sighduck {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, mint_id: String, setreward: u64) -> ProgramResult {
        let initialize_account = &mut ctx.accounts.reward_account;
        initialize_account.mint = mint_id;
        initialize_account.user = ctx.accounts.user.key.to_string();
        initialize_account.rewards = setreward;
        initialize_account.reward_collected = 0;
        initialize_account.startdate = Clock::get().unwrap().unix_timestamp as u64;
        msg!("Account initialized");
        Ok(())
    }

    pub fn update(ctx: Context<Update>, should_update: bool) -> ProgramResult {
        require!(
            ctx.accounts.updator.key().to_string()
                == "BbKEiiH1amHxhEk6EckPZbMwSQPP8ET3G5HyuLqdjkU6",
            CommonError::InvalidUpdater
        );

        let reward_account = &mut ctx.accounts.reward_account;
        // let one_day = 86400;
        // let five_mins = 300;
        let two_mins = 120;

        let start_time = reward_account.startdate;
        let current_time = Clock::get().unwrap().unix_timestamp as u64;

        let time_passed = current_time - start_time;

        if should_update {
            let multiple = time_passed / two_mins as u64;
            reward_account.reward_collected = reward_account.rewards * multiple;
            msg!("Updated...");
        } else {
            msg!("Update stopped");
        }

        Ok(())
    }

    pub fn withdraw_treasury(ctx: Context<Withdraw>) -> ProgramResult {
        require!(
            ctx.accounts.reward_account.user == ctx.accounts.user.key().to_string(),
            CommonError::InvalidSigner
        );

        let reward_account = &mut ctx.accounts.reward_account;
        let convert_reward = reward_account.reward_collected * 1000000000;
        let source = &ctx.accounts.source;
        let destination = &ctx.accounts.destination;
        let token_program_id = &ctx.accounts.token_program_id;
        let authority = &ctx.accounts.authority;
        let mint_address = &ctx.accounts.mint_address;
        // let user = &ctx.accounts.user;
        // let system_account = &ctx.accounts.system_account;
        // let system_rent_account = &ctx.accounts.system_rent_account;
        // let associated_account = &ctx.accounts.associated_account;

        // let associated_ins =
        //     create_associated_token_account(&user.key(), &user.key(), &mint_address.key());

        // match invoke(
        //     &associated_ins,
        //     &[
        //         user.to_account_info(),
        //         mint_address.to_account_info(),
        //         system_account.to_account_info(),
        //         associated_account.to_account_info(),
        //         system_rent_account.to_account_info(),
        //         destination.to_account_info(),
        //         token_program_id.to_account_info(),
        //     ],
        // ) {
        //     Ok(_noerr) => {
        //         msg!("Successfully created associated account");
        //     }
        //     Err(_err) => {
        //         msg!("Account already exits");
        //     }
        // };

        let transfer_ins = transfer_checked(
            &token_program_id.key(),
            &source.key(),
            &mint_address.key(),
            &destination.key(),
            &authority.key(),
            &[&authority.key()],
            convert_reward,
            9,
        )?;

        invoke(
            &transfer_ins,
            &[
                token_program_id.to_account_info(),
                source.to_account_info(),
                mint_address.to_account_info(),
                destination.to_account_info(),
                authority.to_account_info(),
            ],
        )?;

        reward_account.reward_collected = 0;
        reward_account.startdate = Clock::get().unwrap().unix_timestamp as u64;

        msg!("successfully withdrawn ");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 54 + 54 + 8 + 8 + 8 )]
    pub reward_account: Account<'info, RewardAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub authority: Signer<'info>,
    pub token_program_id: UncheckedAccount<'info>,
    #[account(mut)]
    pub source: UncheckedAccount<'info>,
    #[account(mut)]
    pub destination: UncheckedAccount<'info>,
    pub mint_address: UncheckedAccount<'info>,
    #[account(mut)]
    pub reward_account: Account<'info, RewardAccount>,
    pub user: Signer<'info>,
    // pub system_account: UncheckedAccount<'info>,
    // pub system_rent_account: UncheckedAccount<'info>,
    // pub associated_account: UncheckedAccount<'info>,
}

// UncheckedAccount , Account, Program

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub reward_account: Account<'info, RewardAccount>,
    pub updator: Signer<'info>,
}

#[account]
pub struct RewardAccount {
    pub mint: String,
    pub user: String,
    pub rewards: u64,
    pub startdate: u64,
    pub reward_collected: u64,
}
