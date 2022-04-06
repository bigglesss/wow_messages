use std::convert::{TryFrom, TryInto};
use crate::helper::Guid;
use crate::world::v1::v12::{NewItemChatAlert, NewItemChatAlertError};
use crate::world::v1::v12::{NewItemCreationType, NewItemCreationTypeError};
use crate::world::v1::v12::{NewItemSource, NewItemSourceError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_push_result.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_push_result.wowm#L18):
/// ```text
/// smsg SMSG_ITEM_PUSH_RESULT = 0x166 {
///     Guid guid;
///     NewItemSource source;
///     NewItemCreationType creation_type;
///     NewItemChatAlert alert_chat;
///     u8 bag_slot;
///     u32 item_slot;
///     u32 item_id;
///     u32 item_suffix_factor;
///     u32 item_random_property_id;
///     u32 item_count;
/// }
/// ```
pub struct SMSG_ITEM_PUSH_RESULT {
    pub guid: Guid,
    pub source: NewItemSource,
    pub creation_type: NewItemCreationType,
    pub alert_chat: NewItemChatAlert,
    pub bag_slot: u8,
    pub item_slot: u32,
    pub item_id: u32,
    pub item_suffix_factor: u32,
    pub item_random_property_id: u32,
    pub item_count: u32,
}

impl WorldServerMessageWrite for SMSG_ITEM_PUSH_RESULT {
    const OPCODE: u16 = 0x166;

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
impl WorldMessageBody for SMSG_ITEM_PUSH_RESULT {
    type Error = SMSG_ITEM_PUSH_RESULTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // source: NewItemSource
        let source = NewItemSource::read(r)?;

        // creation_type: NewItemCreationType
        let creation_type = NewItemCreationType::read(r)?;

        // alert_chat: NewItemChatAlert
        let alert_chat = NewItemChatAlert::read(r)?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(r)?;

        // item_slot: u32
        let item_slot = crate::util::read_u32_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_suffix_factor: u32
        let item_suffix_factor = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // item_count: u32
        let item_count = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            source,
            creation_type,
            alert_chat,
            bag_slot,
            item_slot,
            item_id,
            item_suffix_factor,
            item_random_property_id,
            item_count,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // source: NewItemSource
        self.source.write(w)?;

        // creation_type: NewItemCreationType
        self.creation_type.write(w)?;

        // alert_chat: NewItemChatAlert
        self.alert_chat.write(w)?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // item_slot: u32
        w.write_all(&self.item_slot.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_suffix_factor: u32
        w.write_all(&self.item_suffix_factor.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_ITEM_PUSH_RESULT {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_ITEM_PUSH_RESULT {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + NewItemSource::size() // source: NewItemSource
        + NewItemCreationType::size() // creation_type: NewItemCreationType
        + NewItemChatAlert::size() // alert_chat: NewItemChatAlert
        + 1 // bag_slot: u8
        + 4 // item_slot: u32
        + 4 // item_id: u32
        + 4 // item_suffix_factor: u32
        + 4 // item_random_property_id: u32
        + 4 // item_count: u32
    }
}

#[derive(Debug)]
pub enum SMSG_ITEM_PUSH_RESULTError {
    Io(std::io::Error),
    NewItemChatAlert(NewItemChatAlertError),
    NewItemCreationType(NewItemCreationTypeError),
    NewItemSource(NewItemSourceError),
}

impl std::error::Error for SMSG_ITEM_PUSH_RESULTError {}
impl std::fmt::Display for SMSG_ITEM_PUSH_RESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::NewItemChatAlert(i) => i.fmt(f),
            Self::NewItemCreationType(i) => i.fmt(f),
            Self::NewItemSource(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_ITEM_PUSH_RESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<NewItemChatAlertError> for SMSG_ITEM_PUSH_RESULTError {
    fn from(e: NewItemChatAlertError) -> Self {
        Self::NewItemChatAlert(e)
    }
}

impl From<NewItemCreationTypeError> for SMSG_ITEM_PUSH_RESULTError {
    fn from(e: NewItemCreationTypeError) -> Self {
        Self::NewItemCreationType(e)
    }
}

impl From<NewItemSourceError> for SMSG_ITEM_PUSH_RESULTError {
    fn from(e: NewItemSourceError) -> Self {
        Self::NewItemSource(e)
    }
}

