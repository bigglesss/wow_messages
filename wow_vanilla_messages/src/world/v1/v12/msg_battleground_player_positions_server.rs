use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::BattlegroundPlayerPosition;
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
pub struct MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    pub flag_carriers: Vec<BattlegroundPlayerPosition>,
}

impl ServerMessageWrite for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {}

impl MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    pub const AMOUNT_OF_CARRIERS_VALUE: u32 = 0x00;

}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    const OPCODE: u16 = 0x02e9;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_carriers: u32
        let _amount_of_carriers = crate::util::read_u32_le(r)?;
        // amount_of_carriers is expected to always be 0 (0)

        // amount_of_flag_carriers: u32
        let amount_of_flag_carriers = crate::util::read_u32_le(r)?;

        // flag_carriers: BattlegroundPlayerPosition[amount_of_flag_carriers]
        let mut flag_carriers = Vec::with_capacity(amount_of_flag_carriers as usize);
        for i in 0..amount_of_flag_carriers {
            flag_carriers.push(BattlegroundPlayerPosition::read(r)?);
        }

        Ok(Self {
            flag_carriers,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_carriers: u32
        w.write_all(&Self::AMOUNT_OF_CARRIERS_VALUE.to_le_bytes())?;

        // amount_of_flag_carriers: u32
        w.write_all(&(self.flag_carriers.len() as u32).to_le_bytes())?;

        // flag_carriers: BattlegroundPlayerPosition[amount_of_flag_carriers]
        for i in self.flag_carriers.iter() {
            i.write(w)?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_carriers: u32
        let _amount_of_carriers = crate::util::tokio_read_u32_le(r).await?;
        // amount_of_carriers is expected to always be 0 (0)

        // amount_of_flag_carriers: u32
        let amount_of_flag_carriers = crate::util::tokio_read_u32_le(r).await?;

        // flag_carriers: BattlegroundPlayerPosition[amount_of_flag_carriers]
        let mut flag_carriers = Vec::with_capacity(amount_of_flag_carriers as usize);
        for i in 0..amount_of_flag_carriers {
            flag_carriers.push(BattlegroundPlayerPosition::tokio_read(r).await?);
        }

        Ok(Self {
            flag_carriers,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_carriers: u32
        w.write_all(&Self::AMOUNT_OF_CARRIERS_VALUE.to_le_bytes()).await?;

        // amount_of_flag_carriers: u32
        w.write_all(&(self.flag_carriers.len() as u32).to_le_bytes()).await?;

        // flag_carriers: BattlegroundPlayerPosition[amount_of_flag_carriers]
        for i in self.flag_carriers.iter() {
            i.tokio_write(w).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_carriers: u32
        let _amount_of_carriers = crate::util::astd_read_u32_le(r).await?;
        // amount_of_carriers is expected to always be 0 (0)

        // amount_of_flag_carriers: u32
        let amount_of_flag_carriers = crate::util::astd_read_u32_le(r).await?;

        // flag_carriers: BattlegroundPlayerPosition[amount_of_flag_carriers]
        let mut flag_carriers = Vec::with_capacity(amount_of_flag_carriers as usize);
        for i in 0..amount_of_flag_carriers {
            flag_carriers.push(BattlegroundPlayerPosition::astd_read(r).await?);
        }

        Ok(Self {
            flag_carriers,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_carriers: u32
        w.write_all(&Self::AMOUNT_OF_CARRIERS_VALUE.to_le_bytes()).await?;

        // amount_of_flag_carriers: u32
        w.write_all(&(self.flag_carriers.len() as u32).to_le_bytes()).await?;

        // flag_carriers: BattlegroundPlayerPosition[amount_of_flag_carriers]
        for i in self.flag_carriers.iter() {
            i.astd_write(w).await?;
        }

        Ok(())
    }

}

impl VariableSized for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    fn size(&self) -> usize {
        4 // amount_of_carriers: u32
        + 4 // amount_of_flag_carriers: u32
        + self.flag_carriers.iter().fold(0, |acc, x| acc + BattlegroundPlayerPosition::size()) // flag_carriers: BattlegroundPlayerPosition[amount_of_flag_carriers]
    }
}

impl MaximumPossibleSized for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    fn maximum_possible_size() -> usize {
        4 // amount_of_carriers: u32
        + 4 // amount_of_flag_carriers: u32
        + 4294967295 * BattlegroundPlayerPosition::maximum_possible_size() // flag_carriers: BattlegroundPlayerPosition[amount_of_flag_carriers]
    }
}

