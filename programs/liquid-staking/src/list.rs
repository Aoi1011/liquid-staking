use std::io::Cursor;

use anchor_lang::prelude::*;
use borsh::BorshSchema;
use std::convert::TryFrom;

use crate::error::CommonError;

#[derive(Default, Clone, AnchorSerialize, AnchorDeserialize, BorshSchema, Debug)]
pub struct List {
    pub account: Pubkey,
    pub item_size: u32,
    pub count: u32,
    // For chunked change account
    pub new_account: Pubkey,
    pub copied_count: u32,
}

impl List {
    pub fn new(
        discriminator: &[u8; 8],
        item_size: u32,
        account: Pubkey,
        data: &mut [u8],
        list_name: &str,
    ) -> Result<Self> {
        let result = Self {
            account,
            item_size,
            count: 0,
            new_account: Pubkey::default(),
            copied_count: 0,
        };
        result.init_account(discriminator, data, list_name)?;
        Ok(result)
    }

    pub fn bytes_for(item_size: u32, count: u32) -> u32 {
        8 + count * item_size
    }

    pub fn capacity_of(item_size: u32, account_len: usize) -> u32 {
        (account_len as u32 - 8) / item_size
    }
    fn init_account(
        &self,
        discriminator: &[u8; 8],
        data: &mut [u8],
        list_name: &str,
    ) -> Result<()> {
        assert_eq!(self.count, 0);
        if data.len() < 8 {
            msg!(
                "{} account must have at least 8 bytes of storage",
                list_name
            );
            return Err(ProgramError::AccountDataTooSmall.into());
        }
        if data[0..8] != [0; 8] {
            msg!("{} account is already initialized", list_name);
            return Err(ProgramError::AccountAlreadyInitialized.into());
        }

        data[0..8].copy_from_slice(discriminator);

        Ok(())
    }

    pub fn item_size(&self) -> u32 {
        self.item_size
    }

    pub fn len(&self) -> u32 {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn is_changing_account(&self) -> bool {
        self.new_account != Pubkey::default()
    }

    pub fn capacity(&self, account_len: usize) -> Result<u32> {
        Ok(u32::try_from(
            account_len
                .checked_sub(8)
                .ok_or(ProgramError::AccountDataTooSmall)?,
        )
        .map_err(|_| CommonError::CalculationFailure)?
        .checked_div(self.item_size())
        .unwrap_or(std::u32::MAX))
    }

    pub fn get<I: AnchorDeserialize>(
        &self,
        data: &[u8],
        index: u32,
        list_name: &str,
    ) -> std::result::Result<I, ProgramError> {
        if index >= self.len() {
            msg!(
                "list {} index out of bounds ({}/{})",
                list_name,
                index,
                self.len()
            );
            return Err(ProgramError::InvalidArgument.into());
        }
        let start = 8 + (index * self.item_size()) as usize;
        I::deserialize(&mut &data[start..(start + self.item_size() as usize)])
            .map_err(|err| ProgramError::BorshIoError(err.to_string()))
    }

    pub fn set<I: AnchorSerialize>(
        &self,
        data: &mut [u8],
        index: u32,
        item: I,
        list_name: &str,
    ) -> Result<()> {
        if self.new_account != Pubkey::default() {
            msg!(
                "Can not modify list {} while changing list's account",
                list_name
            );
            return Err(ProgramError::InvalidAccountData.into());
        }
        if index >= self.len() {
            msg!(
                "list {} index out of bounds ({}/{})",
                list_name,
                index,
                self.len()
            );
            return Err(ProgramError::InvalidArgument.into());
        }

        let start = 8 + (index * self.item_size()) as usize;
        let mut cursor = Cursor::new(&mut data[start..(start + self.item_size() as usize)]);
        item.serialize(&mut cursor)?;

        Ok(())
    }

    pub fn push<I: AnchorSerialize>(
        &mut self,
        data: &mut [u8],
        item: I,
        list_name: &str,
    ) -> Result<()> {
        if self.new_account != Pubkey::default() {
            msg!("Can not modify list {} while changing list's account");
            return Err(ProgramError::InvalidAccountData.into());
        }
        let capacity = self.capacity(data.len())?;
        if self.len() >= capacity {
            msg!("list {} with capacity {} is full", list_name, capacity);
            return Err(ProgramError::AccountDataTooSmall.into());
        }

        let start = 8 + (self.len() * self.item_size()) as usize;
        let mut cursor = Cursor::new(&mut data[start..(start + self.item_size() as usize)]);
        item.serialize(&mut cursor)?;

        self.count += 1;

        Ok(())
    }

    pub fn remove(&mut self, data: &mut [u8], index: u32, list_name: &str) -> Result<()> {
        if self.new_account != Pubkey::default() {
            msg!("Can not modify list {} while changing list's account");
            return Err(ProgramError::InvalidAccountData.into());
        }

        if index >= self.len() {
            msg!(
                "list {} remove out of bounds ({} / {})",
                list_name,
                index,
                self.len()
            );
            return Err(ProgramError::InvalidArgument.into());
        }

        self.count -= 1;
        if index == self.count {
            return Ok(());
        }
        let start = 8 + (index * self.item_size()) as usize;
        let last_item_start = 8 + (self.count * self.item_size()) as usize;
        data.copy_within(
            last_item_start..last_item_start + self.item_size() as usize,
            start,
        );

        Ok(())
    }
}
