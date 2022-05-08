use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::QuestItemReward;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_QUESTGIVER_QUEST_COMPLETE {
    pub quest_id: u32,
    pub unknown: u32,
    pub experience_reward: u32,
    pub money_reward: u32,
    pub item_rewards: Vec<QuestItemReward>,
}

impl ServerMessageWrite for SMSG_QUESTGIVER_QUEST_COMPLETE {}

impl MessageBody for SMSG_QUESTGIVER_QUEST_COMPLETE {
    const OPCODE: u16 = 0x0191;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(r)?;

        // experience_reward: u32
        let experience_reward = crate::util::read_u32_le(r)?;

        // money_reward: u32
        let money_reward = crate::util::read_u32_le(r)?;

        // amount_of_item_rewards: u32
        let amount_of_item_rewards = crate::util::read_u32_le(r)?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
        for i in 0..amount_of_item_rewards {
            item_rewards.push(QuestItemReward::read(r)?);
        }

        Ok(Self {
            quest_id,
            unknown,
            experience_reward,
            money_reward,
            item_rewards,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        // experience_reward: u32
        w.write_all(&self.experience_reward.to_le_bytes())?;

        // money_reward: u32
        w.write_all(&self.money_reward.to_le_bytes())?;

        // amount_of_item_rewards: u32
        w.write_all(&(self.item_rewards.len() as u32).to_le_bytes())?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        for i in self.item_rewards.iter() {
            i.write(w)?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // quest_id: u32
            let quest_id = crate::util::tokio_read_u32_le(r).await?;

            // unknown: u32
            let unknown = crate::util::tokio_read_u32_le(r).await?;

            // experience_reward: u32
            let experience_reward = crate::util::tokio_read_u32_le(r).await?;

            // money_reward: u32
            let money_reward = crate::util::tokio_read_u32_le(r).await?;

            // amount_of_item_rewards: u32
            let amount_of_item_rewards = crate::util::tokio_read_u32_le(r).await?;

            // item_rewards: QuestItemReward[amount_of_item_rewards]
            let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
            for i in 0..amount_of_item_rewards {
                item_rewards.push(QuestItemReward::tokio_read(r).await?);
            }

            Ok(Self {
                quest_id,
                unknown,
                experience_reward,
                money_reward,
                item_rewards,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // quest_id: u32
            w.write_all(&self.quest_id.to_le_bytes()).await?;

            // unknown: u32
            w.write_all(&self.unknown.to_le_bytes()).await?;

            // experience_reward: u32
            w.write_all(&self.experience_reward.to_le_bytes()).await?;

            // money_reward: u32
            w.write_all(&self.money_reward.to_le_bytes()).await?;

            // amount_of_item_rewards: u32
            w.write_all(&(self.item_rewards.len() as u32).to_le_bytes()).await?;

            // item_rewards: QuestItemReward[amount_of_item_rewards]
            for i in self.item_rewards.iter() {
                i.tokio_write(w).await?;
            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // quest_id: u32
            let quest_id = crate::util::astd_read_u32_le(r).await?;

            // unknown: u32
            let unknown = crate::util::astd_read_u32_le(r).await?;

            // experience_reward: u32
            let experience_reward = crate::util::astd_read_u32_le(r).await?;

            // money_reward: u32
            let money_reward = crate::util::astd_read_u32_le(r).await?;

            // amount_of_item_rewards: u32
            let amount_of_item_rewards = crate::util::astd_read_u32_le(r).await?;

            // item_rewards: QuestItemReward[amount_of_item_rewards]
            let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
            for i in 0..amount_of_item_rewards {
                item_rewards.push(QuestItemReward::astd_read(r).await?);
            }

            Ok(Self {
                quest_id,
                unknown,
                experience_reward,
                money_reward,
                item_rewards,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // quest_id: u32
            w.write_all(&self.quest_id.to_le_bytes()).await?;

            // unknown: u32
            w.write_all(&self.unknown.to_le_bytes()).await?;

            // experience_reward: u32
            w.write_all(&self.experience_reward.to_le_bytes()).await?;

            // money_reward: u32
            w.write_all(&self.money_reward.to_le_bytes()).await?;

            // amount_of_item_rewards: u32
            w.write_all(&(self.item_rewards.len() as u32).to_le_bytes()).await?;

            // item_rewards: QuestItemReward[amount_of_item_rewards]
            for i in self.item_rewards.iter() {
                i.astd_write(w).await?;
            }

            Ok(())
        })
    }

}

impl VariableSized for SMSG_QUESTGIVER_QUEST_COMPLETE {
    fn size(&self) -> usize {
        0
        + 4 // quest_id: u32
        + 4 // unknown: u32
        + 4 // experience_reward: u32
        + 4 // money_reward: u32
        + 4 // amount_of_item_rewards: u32
        + self.item_rewards.iter().fold(0, |acc, x| acc + QuestItemReward::size()) // item_rewards: QuestItemReward[amount_of_item_rewards]
    }
}

impl MaximumPossibleSized for SMSG_QUESTGIVER_QUEST_COMPLETE {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

