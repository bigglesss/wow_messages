use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{BuyBankSlotResult, BuyBankSlotResultError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_buy_bank_slot_result.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_buy_bank_slot_result.wowm#L10):
/// ```text
/// smsg SMSG_BUY_BANK_SLOT_RESULT = 0x1BA {
///     BuyBankSlotResult result;
/// }
/// ```
pub struct SMSG_BUY_BANK_SLOT_RESULT {
    pub result: BuyBankSlotResult,
}

impl WorldServerMessageWrite for SMSG_BUY_BANK_SLOT_RESULT {
    const OPCODE: u16 = 0x1ba;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_BUY_BANK_SLOT_RESULT {
    type Error = SMSG_BUY_BANK_SLOT_RESULTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: BuyBankSlotResult
        let result = BuyBankSlotResult::read(r)?;

        Ok(Self {
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: BuyBankSlotResult
        self.result.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_BUY_BANK_SLOT_RESULT {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_BUY_BANK_SLOT_RESULT {
    fn maximum_possible_size() -> usize {
        BuyBankSlotResult::size() // result: BuyBankSlotResult
    }
}

#[derive(Debug)]
pub enum SMSG_BUY_BANK_SLOT_RESULTError {
    Io(std::io::Error),
    BuyBankSlotResult(BuyBankSlotResultError),
}

impl std::error::Error for SMSG_BUY_BANK_SLOT_RESULTError {}
impl std::fmt::Display for SMSG_BUY_BANK_SLOT_RESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::BuyBankSlotResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_BUY_BANK_SLOT_RESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<BuyBankSlotResultError> for SMSG_BUY_BANK_SLOT_RESULTError {
    fn from(e: BuyBankSlotResultError) -> Self {
        Self::BuyBankSlotResult(e)
    }
}
