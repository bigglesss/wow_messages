use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{BuyResult, BuyResultError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_buy_failed.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_buy_failed.wowm#L15):
/// ```text
/// smsg SMSG_BUY_FAILED = 0x1A5 {
///     u64 guid;
///     u32 item_id;
///     BuyResult result;
/// }
/// ```
pub struct SMSG_BUY_FAILED {
    pub guid: u64,
    pub item_id: u32,
    pub result: BuyResult,
}

impl WorldServerMessageWrite for SMSG_BUY_FAILED {
    const OPCODE: u16 = 0x1a5;

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
impl WorldMessageBody for SMSG_BUY_FAILED {
    type Error = SMSG_BUY_FAILEDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // result: BuyResult
        let result = BuyResult::read(r)?;

        Ok(Self {
            guid,
            item_id,
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // result: BuyResult
        self.result.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_BUY_FAILED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_BUY_FAILED {
    fn maximum_possible_size() -> usize {
        8 // guid: u64
        + 4 // item_id: u32
        + BuyResult::size() // result: BuyResult
    }
}

#[derive(Debug)]
pub enum SMSG_BUY_FAILEDError {
    Io(std::io::Error),
    BuyResult(BuyResultError),
}

impl std::error::Error for SMSG_BUY_FAILEDError {}
impl std::fmt::Display for SMSG_BUY_FAILEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::BuyResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_BUY_FAILEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<BuyResultError> for SMSG_BUY_FAILEDError {
    fn from(e: BuyResultError) -> Self {
        Self::BuyResult(e)
    }
}
