use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{StabledPet, StabledPetError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/msg_list_stabled_pets_server.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/msg_list_stabled_pets_server.wowm#L14):
/// ```text
/// smsg MSG_LIST_STABLED_PETS_Server = 0x26F {
///     Guid npc;
///     u8 amount_of_pets;
///     u8 stable_slots;
///     StabledPet[amount_of_pets] pets;
/// }
/// ```
pub struct MSG_LIST_STABLED_PETS_Server {
    pub npc: Guid,
    pub amount_of_pets: u8,
    pub stable_slots: u8,
    pub pets: Vec<StabledPet>,
}

impl WorldServerMessageWrite for MSG_LIST_STABLED_PETS_Server {
    const OPCODE: u16 = 0x26f;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for MSG_LIST_STABLED_PETS_Server {
    type Error = MSG_LIST_STABLED_PETS_ServerError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc: Guid
        let npc = Guid::read(r)?;

        // amount_of_pets: u8
        let amount_of_pets = crate::util::read_u8_le(r)?;

        // stable_slots: u8
        let stable_slots = crate::util::read_u8_le(r)?;

        // pets: StabledPet[amount_of_pets]
        let mut pets = Vec::with_capacity(amount_of_pets as usize);
        for i in 0..amount_of_pets {
            pets.push(StabledPet::read(r)?);
        }

        Ok(Self {
            npc,
            amount_of_pets,
            stable_slots,
            pets,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc: Guid
        self.npc.write(w)?;

        // amount_of_pets: u8
        w.write_all(&(self.pets.len() as u8).to_le_bytes())?;

        // stable_slots: u8
        w.write_all(&self.stable_slots.to_le_bytes())?;

        // pets: StabledPet[amount_of_pets]
        for i in self.pets.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for MSG_LIST_STABLED_PETS_Server {
    fn size(&self) -> usize {
        8 // npc: Guid
        + 1 // amount_of_pets: u8
        + 1 // stable_slots: u8
        + self.pets.iter().fold(0, |acc, x| acc + x.size()) // pets: StabledPet[amount_of_pets]
    }
}

impl MaximumPossibleSized for MSG_LIST_STABLED_PETS_Server {
    fn maximum_possible_size() -> usize {
        8 // npc: Guid
        + 1 // amount_of_pets: u8
        + 1 // stable_slots: u8
        + 255 * StabledPet::maximum_possible_size() // pets: StabledPet[amount_of_pets]
    }
}

#[derive(Debug)]
pub enum MSG_LIST_STABLED_PETS_ServerError {
    Io(std::io::Error),
    StabledPet(StabledPetError),
}

impl std::error::Error for MSG_LIST_STABLED_PETS_ServerError {}
impl std::fmt::Display for MSG_LIST_STABLED_PETS_ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::StabledPet(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for MSG_LIST_STABLED_PETS_ServerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<StabledPetError> for MSG_LIST_STABLED_PETS_ServerError {
    fn from(e: StabledPetError) -> Self {
        Self::StabledPet(e)
    }
}
