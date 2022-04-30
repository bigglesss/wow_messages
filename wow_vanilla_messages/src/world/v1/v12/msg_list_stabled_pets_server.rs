use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{StabledPet, StabledPetError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MSG_LIST_STABLED_PETS_Server {
    pub npc: Guid,
    pub stable_slots: u8,
    pub pets: Vec<StabledPet>,
}

impl ServerMessageWrite for MSG_LIST_STABLED_PETS_Server {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for MSG_LIST_STABLED_PETS_Server {
    const OPCODE: u16 = 0x026f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

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

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc: Guid
        let npc = Guid::tokio_read(r).await?;

        // amount_of_pets: u8
        let amount_of_pets = crate::util::tokio_read_u8_le(r).await?;

        // stable_slots: u8
        let stable_slots = crate::util::tokio_read_u8_le(r).await?;

        // pets: StabledPet[amount_of_pets]
        let mut pets = Vec::with_capacity(amount_of_pets as usize);
        for i in 0..amount_of_pets {
            pets.push(StabledPet::tokio_read(r).await?);
        }

        Ok(Self {
            npc,
            stable_slots,
            pets,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc: Guid
        self.npc.tokio_write(w).await?;

        // amount_of_pets: u8
        w.write_all(&(self.pets.len() as u8).to_le_bytes()).await?;

        // stable_slots: u8
        w.write_all(&self.stable_slots.to_le_bytes()).await?;

        // pets: StabledPet[amount_of_pets]
        for i in self.pets.iter() {
            i.tokio_write(w).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc: Guid
        let npc = Guid::astd_read(r).await?;

        // amount_of_pets: u8
        let amount_of_pets = crate::util::astd_read_u8_le(r).await?;

        // stable_slots: u8
        let stable_slots = crate::util::astd_read_u8_le(r).await?;

        // pets: StabledPet[amount_of_pets]
        let mut pets = Vec::with_capacity(amount_of_pets as usize);
        for i in 0..amount_of_pets {
            pets.push(StabledPet::astd_read(r).await?);
        }

        Ok(Self {
            npc,
            stable_slots,
            pets,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc: Guid
        self.npc.astd_write(w).await?;

        // amount_of_pets: u8
        w.write_all(&(self.pets.len() as u8).to_le_bytes()).await?;

        // stable_slots: u8
        w.write_all(&self.stable_slots.to_le_bytes()).await?;

        // pets: StabledPet[amount_of_pets]
        for i in self.pets.iter() {
            i.astd_write(w).await?;
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

