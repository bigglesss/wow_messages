use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_PETITION_QUERY_RESPONSE {
    pub petition_guid: Guid,
    pub charter_owner: Guid,
    pub guild_name: String,
    pub body_text: String,
    pub unknown_flags: u32,
    pub minimum_signatures: u32,
    pub maximum_signatures: u32,
    pub deadline: u32,
    pub issue_date: u32,
    pub allowed_guild_id: u32,
    pub allowed_classes: u32,
    pub allowed_races: u32,
    pub allowed_genders: u16,
    pub allowed_minimum_level: u32,
    pub allowed_maximum_level: u32,
    pub todo_amount_of_signers: u32,
    pub number_of_choices: u32,
}

impl SMSG_PETITION_QUERY_RESPONSE {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        // charter_owner: Guid
        w.write_all(&self.charter_owner.guid().to_le_bytes())?;

        // guild_name: CString
        w.write_all(self.guild_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // body_text: CString
        w.write_all(self.body_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // unknown_flags: u32
        w.write_all(&self.unknown_flags.to_le_bytes())?;

        // minimum_signatures: u32
        w.write_all(&self.minimum_signatures.to_le_bytes())?;

        // maximum_signatures: u32
        w.write_all(&self.maximum_signatures.to_le_bytes())?;

        // deadline: u32
        w.write_all(&self.deadline.to_le_bytes())?;

        // issue_date: u32
        w.write_all(&self.issue_date.to_le_bytes())?;

        // allowed_guild_id: u32
        w.write_all(&self.allowed_guild_id.to_le_bytes())?;

        // allowed_classes: u32
        w.write_all(&self.allowed_classes.to_le_bytes())?;

        // allowed_races: u32
        w.write_all(&self.allowed_races.to_le_bytes())?;

        // allowed_genders: u16
        w.write_all(&self.allowed_genders.to_le_bytes())?;

        // allowed_minimum_level: u32
        w.write_all(&self.allowed_minimum_level.to_le_bytes())?;

        // allowed_maximum_level: u32
        w.write_all(&self.allowed_maximum_level.to_le_bytes())?;

        // todo_amount_of_signers: u32
        w.write_all(&self.todo_amount_of_signers.to_le_bytes())?;

        // number_of_choices: u32
        w.write_all(&self.number_of_choices.to_le_bytes())?;

        Ok(w)
    }
}

impl ServerMessage for SMSG_PETITION_QUERY_RESPONSE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        // charter_owner: Guid
        w.write_all(&self.charter_owner.guid().to_le_bytes())?;

        // guild_name: CString
        w.write_all(self.guild_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // body_text: CString
        w.write_all(self.body_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // unknown_flags: u32
        w.write_all(&self.unknown_flags.to_le_bytes())?;

        // minimum_signatures: u32
        w.write_all(&self.minimum_signatures.to_le_bytes())?;

        // maximum_signatures: u32
        w.write_all(&self.maximum_signatures.to_le_bytes())?;

        // deadline: u32
        w.write_all(&self.deadline.to_le_bytes())?;

        // issue_date: u32
        w.write_all(&self.issue_date.to_le_bytes())?;

        // allowed_guild_id: u32
        w.write_all(&self.allowed_guild_id.to_le_bytes())?;

        // allowed_classes: u32
        w.write_all(&self.allowed_classes.to_le_bytes())?;

        // allowed_races: u32
        w.write_all(&self.allowed_races.to_le_bytes())?;

        // allowed_genders: u16
        w.write_all(&self.allowed_genders.to_le_bytes())?;

        // allowed_minimum_level: u32
        w.write_all(&self.allowed_minimum_level.to_le_bytes())?;

        // allowed_maximum_level: u32
        w.write_all(&self.allowed_maximum_level.to_le_bytes())?;

        // todo_amount_of_signers: u32
        w.write_all(&self.todo_amount_of_signers.to_le_bytes())?;

        // number_of_choices: u32
        w.write_all(&self.number_of_choices.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01c7;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // charter_owner: Guid
        let charter_owner = Guid::read(r)?;

        // guild_name: CString
        let guild_name = crate::util::read_c_string_to_vec(r)?;
        let guild_name = String::from_utf8(guild_name)?;

        // body_text: CString
        let body_text = crate::util::read_c_string_to_vec(r)?;
        let body_text = String::from_utf8(body_text)?;

        // unknown_flags: u32
        let unknown_flags = crate::util::read_u32_le(r)?;

        // minimum_signatures: u32
        let minimum_signatures = crate::util::read_u32_le(r)?;

        // maximum_signatures: u32
        let maximum_signatures = crate::util::read_u32_le(r)?;

        // deadline: u32
        let deadline = crate::util::read_u32_le(r)?;

        // issue_date: u32
        let issue_date = crate::util::read_u32_le(r)?;

        // allowed_guild_id: u32
        let allowed_guild_id = crate::util::read_u32_le(r)?;

        // allowed_classes: u32
        let allowed_classes = crate::util::read_u32_le(r)?;

        // allowed_races: u32
        let allowed_races = crate::util::read_u32_le(r)?;

        // allowed_genders: u16
        let allowed_genders = crate::util::read_u16_le(r)?;

        // allowed_minimum_level: u32
        let allowed_minimum_level = crate::util::read_u32_le(r)?;

        // allowed_maximum_level: u32
        let allowed_maximum_level = crate::util::read_u32_le(r)?;

        // todo_amount_of_signers: u32
        let todo_amount_of_signers = crate::util::read_u32_le(r)?;

        // number_of_choices: u32
        let number_of_choices = crate::util::read_u32_le(r)?;

        Ok(Self {
            petition_guid,
            charter_owner,
            guild_name,
            body_text,
            unknown_flags,
            minimum_signatures,
            maximum_signatures,
            deadline,
            issue_date,
            allowed_guild_id,
            allowed_classes,
            allowed_races,
            allowed_genders,
            allowed_minimum_level,
            allowed_maximum_level,
            todo_amount_of_signers,
            number_of_choices,
        })
    }

}

impl SMSG_PETITION_QUERY_RESPONSE {
    pub fn size(&self) -> usize {
        0
        + 8 // petition_guid: Guid
        + 8 // charter_owner: Guid
        + self.guild_name.len() + 1 // guild_name: CString
        + self.body_text.len() + 1 // body_text: CString
        + 4 // unknown_flags: u32
        + 4 // minimum_signatures: u32
        + 4 // maximum_signatures: u32
        + 4 // deadline: u32
        + 4 // issue_date: u32
        + 4 // allowed_guild_id: u32
        + 4 // allowed_classes: u32
        + 4 // allowed_races: u32
        + 2 // allowed_genders: u16
        + 4 // allowed_minimum_level: u32
        + 4 // allowed_maximum_level: u32
        + 4 // todo_amount_of_signers: u32
        + 4 // number_of_choices: u32
    }
}

