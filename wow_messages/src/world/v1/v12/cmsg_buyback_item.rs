use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{BuybackSlot, BuybackSlotError};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:3990`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L3990):
/// ```text
/// cmsg CMSG_BUYBACK_ITEM = 0x290 {
///     u64 guid;
///     BuybackSlot slot;
/// }
/// ```
pub struct CMSG_BUYBACK_ITEM {
    pub guid: u64,
    pub slot: BuybackSlot,
}

impl WorldClientMessageWrite for CMSG_BUYBACK_ITEM {
    const OPCODE: u32 = 0x290;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_BUYBACK_ITEM {
    type Error = CMSG_BUYBACK_ITEMError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        // slot: BuybackSlot
        let slot = BuybackSlot::read(r)?;

        Ok(Self {
            guid,
            slot,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        // slot: BuybackSlot
        self.slot.write(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_BUYBACK_ITEM {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_BUYBACK_ITEM {
    fn maximum_possible_size() -> usize {
        8 // guid: u64
        + BuybackSlot::size() // slot: BuybackSlot
    }
}

#[derive(Debug)]
pub enum CMSG_BUYBACK_ITEMError {
    Io(std::io::Error),
    BuybackSlot(BuybackSlotError),
}

impl std::error::Error for CMSG_BUYBACK_ITEMError {}
impl std::fmt::Display for CMSG_BUYBACK_ITEMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::BuybackSlot(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_BUYBACK_ITEMError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<BuybackSlotError> for CMSG_BUYBACK_ITEMError {
    fn from(e: BuybackSlotError) -> Self {
        Self::BuybackSlot(e)
    }
}

