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
pub mod located;
pub mod stake_wrapper;
pub mod ticket_account;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod liquid_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
