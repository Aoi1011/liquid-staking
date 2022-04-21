use anchor_lang::prelude::*;
use error::CommonError;
use stake_wrapper::StakeWrapper;
use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};
use ticket_account::TicketAccountData;

pub mod calc;
pub mod checks;
pub mod error;
pub mod liq_pool;
pub mod located;
pub mod stake_wrapper;
pub mod ticket_account;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// pub static ID: Pubkey = Pubkey::new_from_array([
//     5, 69, 227, 101, 190, 242, 113, 173, 117, 53, 3, 103, 86, 93, 164, 13, 163, 54, 220, 28, 135,
//     155, 177, 84, 138, 122, 252, 197, 90, 169, 57, 30,
// ]);

#[program]
pub mod liquid_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(
    Clone, Copy, Debug, Default, AnchorSerialize, AnchorDeserialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Fee {
    pub basis_points: u32,
}

impl Fee {
    pub fn from_basis_points(basis_points: u32) -> Self {
        Self { basis_points }
    }

    pub fn check_max(&self, max_basis_points: u32) -> Result<()> {
        if self.basis_points > max_basis_points {
            Err(CommonError::FeeTooHigh.into())
        } else {
            Ok(())
        }
    }
}

#[derive(Accounts)]
pub struct Initialize {}
