use crate::error::CommonError;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

pub fn check_min_amount(amount: u64, min_amount: u64, action_name: u64) -> Result<()> {
    if amount >= min_amount {
        Ok(())
    } else {
        msg!(
            "{}: Number too low {} is (min is {})",
            action_name,
            amount,
            min_amount
        );
        Err(CommonError::NumberTooLow.into())
    }
}

pub fn check_address(
    actual_address: &Pubkey,
    reference_address: &Pubkey,
    field_name: &str,
) -> Result<()> {
    if actual_address == reference_address {
        Ok(())
    } else {
        msg!(
            "Invalid {} address: expected {} got {}",
            field_name,
            reference_address,
            actual_address
        );
        Err(ProgramError::InvalidArgument.into())
    }
}

pub fn check_owner_program<'info, A: ToAccountInfo<'info>>(
    account: &A,
    owner: &Pubkey,
    field_name: &str,
) -> Result<()> {
    let actual_owner = account.to_account_info().owner;
    if actual_owner == owner {
        Ok(())
    } else {
        msg!(
            "Invalid {} owner_program: expected {} got {}",
            field_name,
            owner,
            actual_owner
        );
        Err(ProgramError::InvalidArgument.into())
    }
}

pub fn check_mint_authority(mint: &Mint, mint_authority: Pubkey, field_name: &str) -> Result<()> {
    if mint.mint_authority.contains(&mint_authority) {
        Ok(())
    } else {
        msg!(
            "Invalid {} mint authority {}. Expected {}",
            field_name,
            mint.mint_authority.unwrap_or_default(),
            mint_authority
        );
        Err(ProgramError::InvalidArgument.into())
    }
}

pub fn check_freeze_authority(mint: &Mint, field_name: &str) -> Result<()> {
    if mint.freeze_authority.is_none() {
        Ok(())
    } else {
        msg!("Mint {} must have freeze authority not set", field_name);
        Err(ProgramError::InvalidArgument.into())
    }
}
