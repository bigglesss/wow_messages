use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{ItemClass, ItemClassError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:2849`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L2849):
/// ```text
/// smsg SMSG_SET_PROFICIENCY = 0x127 {
///     ItemClass class;
///     u32 item_sub_class_mask;
/// }
/// ```
pub struct SMSG_SET_PROFICIENCY {
    pub class: ItemClass,
    pub item_sub_class_mask: u32,
}

impl WorldServerMessageWrite for SMSG_SET_PROFICIENCY {
    const OPCODE: u16 = 0x127;

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
impl WorldMessageBody for SMSG_SET_PROFICIENCY {
    type Error = SMSG_SET_PROFICIENCYError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // class: ItemClass
        let class = ItemClass::read(r)?;

        // item_sub_class_mask: u32
        let item_sub_class_mask = crate::util::read_u32_le(r)?;

        Ok(Self {
            class,
            item_sub_class_mask,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // class: ItemClass
        self.class.write(w)?;

        // item_sub_class_mask: u32
        w.write_all(&self.item_sub_class_mask.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_SET_PROFICIENCY {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_SET_PROFICIENCY {
    fn maximum_possible_size() -> usize {
        ItemClass::size() // class: ItemClass
        + 4 // item_sub_class_mask: u32
    }
}

#[derive(Debug)]
pub enum SMSG_SET_PROFICIENCYError {
    Io(std::io::Error),
    ItemClass(ItemClassError),
}

impl std::error::Error for SMSG_SET_PROFICIENCYError {}
impl std::fmt::Display for SMSG_SET_PROFICIENCYError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::ItemClass(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SET_PROFICIENCYError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ItemClassError> for SMSG_SET_PROFICIENCYError {
    fn from(e: ItemClassError) -> Self {
        Self::ItemClass(e)
    }
}

