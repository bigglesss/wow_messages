use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::QuestDetailsEmote;
use crate::world::v1::v12::QuestItemReward;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_QUESTGIVER_QUEST_DETAILS {
    pub guid: Guid,
    pub quest_id: u32,
    pub title: String,
    pub details: String,
    pub objectives: String,
    pub auto_finish: u32,
    pub choice_item_rewards: Vec<QuestItemReward>,
    pub item_rewards: Vec<QuestItemReward>,
    pub money_reward: u32,
    pub reward_spell: u32,
    pub emotes: Vec<QuestDetailsEmote>,
}

impl ServerMessageWrite for SMSG_QUESTGIVER_QUEST_DETAILS {}

impl MessageBody for SMSG_QUESTGIVER_QUEST_DETAILS {
    const OPCODE: u16 = 0x0188;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_QUESTGIVER_QUEST_DETAILSError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // title: CString
        let title = crate::util::read_c_string_to_vec(r)?;
        let title = String::from_utf8(title)?;

        // details: CString
        let details = crate::util::read_c_string_to_vec(r)?;
        let details = String::from_utf8(details)?;

        // objectives: CString
        let objectives = crate::util::read_c_string_to_vec(r)?;
        let objectives = String::from_utf8(objectives)?;

        // auto_finish: u32
        let auto_finish = crate::util::read_u32_le(r)?;

        // amount_of_choice_item_rewards: u32
        let amount_of_choice_item_rewards = crate::util::read_u32_le(r)?;

        // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        let mut choice_item_rewards = Vec::with_capacity(amount_of_choice_item_rewards as usize);
        for i in 0..amount_of_choice_item_rewards {
            choice_item_rewards.push(QuestItemReward::read(r)?);
        }

        // amount_of_item_rewards: u32
        let amount_of_item_rewards = crate::util::read_u32_le(r)?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
        for i in 0..amount_of_item_rewards {
            item_rewards.push(QuestItemReward::read(r)?);
        }

        // money_reward: u32
        let money_reward = crate::util::read_u32_le(r)?;

        // reward_spell: u32
        let reward_spell = crate::util::read_u32_le(r)?;

        // amount_of_emotes: u32
        let amount_of_emotes = crate::util::read_u32_le(r)?;

        // emotes: QuestDetailsEmote[amount_of_emotes]
        let mut emotes = Vec::with_capacity(amount_of_emotes as usize);
        for i in 0..amount_of_emotes {
            emotes.push(QuestDetailsEmote::read(r)?);
        }

        Ok(Self {
            guid,
            quest_id,
            title,
            details,
            objectives,
            auto_finish,
            choice_item_rewards,
            item_rewards,
            money_reward,
            reward_spell,
            emotes,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // title: CString
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // details: CString
        w.write_all(self.details.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // objectives: CString
        w.write_all(self.objectives.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // auto_finish: u32
        w.write_all(&self.auto_finish.to_le_bytes())?;

        // amount_of_choice_item_rewards: u32
        w.write_all(&(self.choice_item_rewards.len() as u32).to_le_bytes())?;

        // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        for i in self.choice_item_rewards.iter() {
            i.write(w)?;
        }

        // amount_of_item_rewards: u32
        w.write_all(&(self.item_rewards.len() as u32).to_le_bytes())?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        for i in self.item_rewards.iter() {
            i.write(w)?;
        }

        // money_reward: u32
        w.write_all(&self.money_reward.to_le_bytes())?;

        // reward_spell: u32
        w.write_all(&self.reward_spell.to_le_bytes())?;

        // amount_of_emotes: u32
        w.write_all(&(self.emotes.len() as u32).to_le_bytes())?;

        // emotes: QuestDetailsEmote[amount_of_emotes]
        for i in self.emotes.iter() {
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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // quest_id: u32
            let quest_id = crate::util::tokio_read_u32_le(r).await?;

            // title: CString
            let title = crate::util::tokio_read_c_string_to_vec(r).await?;
            let title = String::from_utf8(title)?;

            // details: CString
            let details = crate::util::tokio_read_c_string_to_vec(r).await?;
            let details = String::from_utf8(details)?;

            // objectives: CString
            let objectives = crate::util::tokio_read_c_string_to_vec(r).await?;
            let objectives = String::from_utf8(objectives)?;

            // auto_finish: u32
            let auto_finish = crate::util::tokio_read_u32_le(r).await?;

            // amount_of_choice_item_rewards: u32
            let amount_of_choice_item_rewards = crate::util::tokio_read_u32_le(r).await?;

            // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
            let mut choice_item_rewards = Vec::with_capacity(amount_of_choice_item_rewards as usize);
            for i in 0..amount_of_choice_item_rewards {
                choice_item_rewards.push(QuestItemReward::tokio_read(r).await?);
            }

            // amount_of_item_rewards: u32
            let amount_of_item_rewards = crate::util::tokio_read_u32_le(r).await?;

            // item_rewards: QuestItemReward[amount_of_item_rewards]
            let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
            for i in 0..amount_of_item_rewards {
                item_rewards.push(QuestItemReward::tokio_read(r).await?);
            }

            // money_reward: u32
            let money_reward = crate::util::tokio_read_u32_le(r).await?;

            // reward_spell: u32
            let reward_spell = crate::util::tokio_read_u32_le(r).await?;

            // amount_of_emotes: u32
            let amount_of_emotes = crate::util::tokio_read_u32_le(r).await?;

            // emotes: QuestDetailsEmote[amount_of_emotes]
            let mut emotes = Vec::with_capacity(amount_of_emotes as usize);
            for i in 0..amount_of_emotes {
                emotes.push(QuestDetailsEmote::tokio_read(r).await?);
            }

            Ok(Self {
                guid,
                quest_id,
                title,
                details,
                objectives,
                auto_finish,
                choice_item_rewards,
                item_rewards,
                money_reward,
                reward_spell,
                emotes,
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
            // guid: Guid
            self.guid.tokio_write(w).await?;

            // quest_id: u32
            w.write_all(&self.quest_id.to_le_bytes()).await?;

            // title: CString
            w.write_all(self.title.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // details: CString
            w.write_all(self.details.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // objectives: CString
            w.write_all(self.objectives.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // auto_finish: u32
            w.write_all(&self.auto_finish.to_le_bytes()).await?;

            // amount_of_choice_item_rewards: u32
            w.write_all(&(self.choice_item_rewards.len() as u32).to_le_bytes()).await?;

            // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
            for i in self.choice_item_rewards.iter() {
                i.tokio_write(w).await?;
            }

            // amount_of_item_rewards: u32
            w.write_all(&(self.item_rewards.len() as u32).to_le_bytes()).await?;

            // item_rewards: QuestItemReward[amount_of_item_rewards]
            for i in self.item_rewards.iter() {
                i.tokio_write(w).await?;
            }

            // money_reward: u32
            w.write_all(&self.money_reward.to_le_bytes()).await?;

            // reward_spell: u32
            w.write_all(&self.reward_spell.to_le_bytes()).await?;

            // amount_of_emotes: u32
            w.write_all(&(self.emotes.len() as u32).to_le_bytes()).await?;

            // emotes: QuestDetailsEmote[amount_of_emotes]
            for i in self.emotes.iter() {
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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // quest_id: u32
            let quest_id = crate::util::astd_read_u32_le(r).await?;

            // title: CString
            let title = crate::util::astd_read_c_string_to_vec(r).await?;
            let title = String::from_utf8(title)?;

            // details: CString
            let details = crate::util::astd_read_c_string_to_vec(r).await?;
            let details = String::from_utf8(details)?;

            // objectives: CString
            let objectives = crate::util::astd_read_c_string_to_vec(r).await?;
            let objectives = String::from_utf8(objectives)?;

            // auto_finish: u32
            let auto_finish = crate::util::astd_read_u32_le(r).await?;

            // amount_of_choice_item_rewards: u32
            let amount_of_choice_item_rewards = crate::util::astd_read_u32_le(r).await?;

            // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
            let mut choice_item_rewards = Vec::with_capacity(amount_of_choice_item_rewards as usize);
            for i in 0..amount_of_choice_item_rewards {
                choice_item_rewards.push(QuestItemReward::astd_read(r).await?);
            }

            // amount_of_item_rewards: u32
            let amount_of_item_rewards = crate::util::astd_read_u32_le(r).await?;

            // item_rewards: QuestItemReward[amount_of_item_rewards]
            let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
            for i in 0..amount_of_item_rewards {
                item_rewards.push(QuestItemReward::astd_read(r).await?);
            }

            // money_reward: u32
            let money_reward = crate::util::astd_read_u32_le(r).await?;

            // reward_spell: u32
            let reward_spell = crate::util::astd_read_u32_le(r).await?;

            // amount_of_emotes: u32
            let amount_of_emotes = crate::util::astd_read_u32_le(r).await?;

            // emotes: QuestDetailsEmote[amount_of_emotes]
            let mut emotes = Vec::with_capacity(amount_of_emotes as usize);
            for i in 0..amount_of_emotes {
                emotes.push(QuestDetailsEmote::astd_read(r).await?);
            }

            Ok(Self {
                guid,
                quest_id,
                title,
                details,
                objectives,
                auto_finish,
                choice_item_rewards,
                item_rewards,
                money_reward,
                reward_spell,
                emotes,
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
            // guid: Guid
            self.guid.astd_write(w).await?;

            // quest_id: u32
            w.write_all(&self.quest_id.to_le_bytes()).await?;

            // title: CString
            w.write_all(self.title.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // details: CString
            w.write_all(self.details.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // objectives: CString
            w.write_all(self.objectives.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // auto_finish: u32
            w.write_all(&self.auto_finish.to_le_bytes()).await?;

            // amount_of_choice_item_rewards: u32
            w.write_all(&(self.choice_item_rewards.len() as u32).to_le_bytes()).await?;

            // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
            for i in self.choice_item_rewards.iter() {
                i.astd_write(w).await?;
            }

            // amount_of_item_rewards: u32
            w.write_all(&(self.item_rewards.len() as u32).to_le_bytes()).await?;

            // item_rewards: QuestItemReward[amount_of_item_rewards]
            for i in self.item_rewards.iter() {
                i.astd_write(w).await?;
            }

            // money_reward: u32
            w.write_all(&self.money_reward.to_le_bytes()).await?;

            // reward_spell: u32
            w.write_all(&self.reward_spell.to_le_bytes()).await?;

            // amount_of_emotes: u32
            w.write_all(&(self.emotes.len() as u32).to_le_bytes()).await?;

            // emotes: QuestDetailsEmote[amount_of_emotes]
            for i in self.emotes.iter() {
                i.astd_write(w).await?;
            }

            Ok(())
        })
    }

}

impl SMSG_QUESTGIVER_QUEST_DETAILS {
    pub fn size(&self) -> usize {
        0
        + 8 // guid: Guid
        + 4 // quest_id: u32
        + self.title.len() + 1 // title: CString
        + self.details.len() + 1 // details: CString
        + self.objectives.len() + 1 // objectives: CString
        + 4 // auto_finish: u32
        + 4 // amount_of_choice_item_rewards: u32
        + self.choice_item_rewards.iter().fold(0, |acc, x| acc + QuestItemReward::size()) // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        + 4 // amount_of_item_rewards: u32
        + self.item_rewards.iter().fold(0, |acc, x| acc + QuestItemReward::size()) // item_rewards: QuestItemReward[amount_of_item_rewards]
        + 4 // money_reward: u32
        + 4 // reward_spell: u32
        + 4 // amount_of_emotes: u32
        + self.emotes.iter().fold(0, |acc, x| acc + QuestDetailsEmote::size()) // emotes: QuestDetailsEmote[amount_of_emotes]
    }
}

#[derive(Debug)]
pub enum SMSG_QUESTGIVER_QUEST_DETAILSError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_QUESTGIVER_QUEST_DETAILSError {}
impl std::fmt::Display for SMSG_QUESTGIVER_QUEST_DETAILSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_QUESTGIVER_QUEST_DETAILSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_QUESTGIVER_QUEST_DETAILSError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

