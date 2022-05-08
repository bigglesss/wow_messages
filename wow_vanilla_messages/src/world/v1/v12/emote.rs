use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Emote {
    ONESHOT_NONE,
    ONESHOT_TALK,
    ONESHOT_BOW,
    ONESHOT_WAVE,
    ONESHOT_CHEER,
    ONESHOT_EXCLAMATION,
    ONESHOT_QUESTION,
    ONESHOT_EAT,
    STATE_DANCE,
    ONESHOT_LAUGH,
    STATE_SLEEP,
    STATE_SIT,
    ONESHOT_RUDE,
    ONESHOT_ROAR,
    ONESHOT_KNEEL,
    ONESHOT_KISS,
    ONESHOT_CRY,
    ONESHOT_CHICKEN,
    ONESHOT_BEG,
    ONESHOT_APPLAUD,
    ONESHOT_SHOUT,
    ONESHOT_FLEX,
    ONESHOT_SHY,
    ONESHOT_POINT,
    STATE_STAND,
    STATE_READYUNARMED,
    STATE_WORK_SHEATHED,
    STATE_POINT,
    STATE_NONE,
    ONESHOT_WOUND,
    ONESHOT_WOUNDCRITICAL,
    ONESHOT_ATTACKUNARMED,
    ONESHOT_ATTACK1H,
    ONESHOT_ATTACK2HTIGHT,
    ONESHOT_ATTACK2HLOOSE,
    ONESHOT_PARRYUNARMED,
    ONESHOT_PARRYSHIELD,
    ONESHOT_READYUNARMED,
    ONESHOT_READY1H,
    ONESHOT_READYBOW,
    ONESHOT_SPELLPRECAST,
    ONESHOT_SPELLCAST,
    ONESHOT_BATTLEROAR,
    ONESHOT_SPECIALATTACK1H,
    ONESHOT_KICK,
    ONESHOT_ATTACKTHROWN,
    STATE_STUN,
    STATE_DEAD,
    ONESHOT_SALUTE,
    STATE_KNEEL,
    STATE_USESTANDING,
    ONESHOT_WAVE_NOSHEATHE,
    ONESHOT_CHEER_NOSHEATHE,
    ONESHOT_EAT_NOSHEATHE,
    STATE_STUN_NOSHEATHE,
    ONESHOT_DANCE,
    ONESHOT_SALUTE_NOSHEATH,
    STATE_USESTANDING_NOSHEATHE,
    ONESHOT_LAUGH_NOSHEATHE,
    STATE_WORK,
    STATE_SPELLPRECAST,
    ONESHOT_READYRIFLE,
    STATE_READYRIFLE,
    STATE_WORK_MINING,
    STATE_WORK_CHOPWOOD,
    STATE_APPLAUD,
    ONESHOT_LIFTOFF,
    ONESHOT_YES,
    ONESHOT_NO,
    ONESHOT_TRAIN,
    ONESHOT_LAND,
    STATE_AT_EASE,
    STATE_READY1H,
    STATE_SPELLKNEELSTART,
    STATE_SUBMERGED,
    ONESHOT_SUBMERGE,
    STATE_READY2H,
    STATE_READYBOW,
    ONESHOT_MOUNTSPECIAL,
    STATE_TALK,
    STATE_FISHING,
    ONESHOT_FISHING,
    ONESHOT_LOOT,
    STATE_WHIRLWIND,
    STATE_DROWNED,
    STATE_HOLD_BOW,
    STATE_HOLD_RIFLE,
    STATE_HOLD_THROWN,
    ONESHOT_DROWN,
    ONESHOT_STOMP,
    ONESHOT_ATTACKOFF,
    ONESHOT_ATTACKOFFPIERCE,
    STATE_ROAR,
    STATE_LAUGH,
    ONESHOT_CREATURE_SPECIAL,
    ONESHOT_JUMPLANDRUN,
    ONESHOT_JUMPEND,
    ONESHOT_TALK_NOSHEATHE,
    ONESHOT_POINT_NOSHEATHE,
    STATE_CANNIBALIZE,
    ONESHOT_JUMPSTART,
    STATE_DANCESPECIAL,
    ONESHOT_DANCESPECIAL,
    ONESHOT_CUSTOMSPELL01,
    ONESHOT_CUSTOMSPELL02,
    ONESHOT_CUSTOMSPELL03,
    ONESHOT_CUSTOMSPELL04,
    ONESHOT_CUSTOMSPELL05,
    ONESHOT_CUSTOMSPELL06,
    ONESHOT_CUSTOMSPELL07,
    ONESHOT_CUSTOMSPELL08,
    ONESHOT_CUSTOMSPELL09,
    ONESHOT_CUSTOMSPELL10,
    STATE_EXCLAIM,
    STATE_SIT_CHAIR_MED,
    STATE_SPELLEFFECT_HOLD,
    STATE_EAT_NO_SHEATHE,
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for Emote {
    type Error = EmoteError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_int().to_le_bytes())?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::tokio_read_u32_le(r).await?;

        Ok(a.try_into()?)
    }

    fn tokio_write<'life0, 'life1, 'async_trait, W>(
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
            w.write_all(&self.as_int().to_le_bytes()).await?;
            Ok(())
        })
    }
    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::astd_read_u32_le(r).await?;

        Ok(a.try_into()?)
    }

    fn astd_write<'life0, 'life1, 'async_trait, W>(
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
            w.write_all(&self.as_int().to_le_bytes()).await?;
            Ok(())
        })
    }
}

impl Emote {
    #[cfg(feature = "sync")]
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, EmoteError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u32_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, EmoteError> {
        let a = crate::util::tokio_read_u32_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u32_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, EmoteError> {
        let a = crate::util::astd_read_u32_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_int() as u32)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u32_be(w, self.as_int() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u32_be(w, self.as_int() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, EmoteError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u32_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, EmoteError> {
        let a = crate::util::tokio_read_u32_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u32_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, EmoteError> {
        let a = crate::util::astd_read_u32_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_int() as u32)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u32_le(w, self.as_int() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u32_le(w, self.as_int() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, EmoteError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u64_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, EmoteError> {
        let a = crate::util::tokio_read_u64_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u64_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, EmoteError> {
        let a = crate::util::astd_read_u64_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_int() as u64)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u64_le(w, self.as_int() as u64).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u64_le(w, self.as_int() as u64).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, EmoteError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u64_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, EmoteError> {
        let a = crate::util::tokio_read_u64_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u64_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, EmoteError> {
        let a = crate::util::astd_read_u64_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_int() as u64)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u64_be(w, self.as_int() as u64).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u64_be(w, self.as_int() as u64).await?;
        Ok(())
    }

    pub const fn as_int(&self) -> u32 {
        match self {
            Self::ONESHOT_NONE => 0x0,
            Self::ONESHOT_TALK => 0x1,
            Self::ONESHOT_BOW => 0x2,
            Self::ONESHOT_WAVE => 0x3,
            Self::ONESHOT_CHEER => 0x4,
            Self::ONESHOT_EXCLAMATION => 0x5,
            Self::ONESHOT_QUESTION => 0x6,
            Self::ONESHOT_EAT => 0x7,
            Self::STATE_DANCE => 0xa,
            Self::ONESHOT_LAUGH => 0xb,
            Self::STATE_SLEEP => 0xc,
            Self::STATE_SIT => 0xd,
            Self::ONESHOT_RUDE => 0xe,
            Self::ONESHOT_ROAR => 0xf,
            Self::ONESHOT_KNEEL => 0x10,
            Self::ONESHOT_KISS => 0x11,
            Self::ONESHOT_CRY => 0x12,
            Self::ONESHOT_CHICKEN => 0x13,
            Self::ONESHOT_BEG => 0x14,
            Self::ONESHOT_APPLAUD => 0x15,
            Self::ONESHOT_SHOUT => 0x16,
            Self::ONESHOT_FLEX => 0x17,
            Self::ONESHOT_SHY => 0x18,
            Self::ONESHOT_POINT => 0x19,
            Self::STATE_STAND => 0x1a,
            Self::STATE_READYUNARMED => 0x1b,
            Self::STATE_WORK_SHEATHED => 0x1c,
            Self::STATE_POINT => 0x1d,
            Self::STATE_NONE => 0x1e,
            Self::ONESHOT_WOUND => 0x21,
            Self::ONESHOT_WOUNDCRITICAL => 0x22,
            Self::ONESHOT_ATTACKUNARMED => 0x23,
            Self::ONESHOT_ATTACK1H => 0x24,
            Self::ONESHOT_ATTACK2HTIGHT => 0x25,
            Self::ONESHOT_ATTACK2HLOOSE => 0x26,
            Self::ONESHOT_PARRYUNARMED => 0x27,
            Self::ONESHOT_PARRYSHIELD => 0x2b,
            Self::ONESHOT_READYUNARMED => 0x2c,
            Self::ONESHOT_READY1H => 0x2d,
            Self::ONESHOT_READYBOW => 0x30,
            Self::ONESHOT_SPELLPRECAST => 0x32,
            Self::ONESHOT_SPELLCAST => 0x33,
            Self::ONESHOT_BATTLEROAR => 0x35,
            Self::ONESHOT_SPECIALATTACK1H => 0x36,
            Self::ONESHOT_KICK => 0x3c,
            Self::ONESHOT_ATTACKTHROWN => 0x3d,
            Self::STATE_STUN => 0x40,
            Self::STATE_DEAD => 0x41,
            Self::ONESHOT_SALUTE => 0x42,
            Self::STATE_KNEEL => 0x44,
            Self::STATE_USESTANDING => 0x45,
            Self::ONESHOT_WAVE_NOSHEATHE => 0x46,
            Self::ONESHOT_CHEER_NOSHEATHE => 0x47,
            Self::ONESHOT_EAT_NOSHEATHE => 0x5c,
            Self::STATE_STUN_NOSHEATHE => 0x5d,
            Self::ONESHOT_DANCE => 0x5e,
            Self::ONESHOT_SALUTE_NOSHEATH => 0x71,
            Self::STATE_USESTANDING_NOSHEATHE => 0x85,
            Self::ONESHOT_LAUGH_NOSHEATHE => 0x99,
            Self::STATE_WORK => 0xad,
            Self::STATE_SPELLPRECAST => 0xc1,
            Self::ONESHOT_READYRIFLE => 0xd5,
            Self::STATE_READYRIFLE => 0xd6,
            Self::STATE_WORK_MINING => 0xe9,
            Self::STATE_WORK_CHOPWOOD => 0xea,
            Self::STATE_APPLAUD => 0xfd,
            Self::ONESHOT_LIFTOFF => 0xfe,
            Self::ONESHOT_YES => 0x111,
            Self::ONESHOT_NO => 0x112,
            Self::ONESHOT_TRAIN => 0x113,
            Self::ONESHOT_LAND => 0x125,
            Self::STATE_AT_EASE => 0x139,
            Self::STATE_READY1H => 0x14d,
            Self::STATE_SPELLKNEELSTART => 0x161,
            Self::STATE_SUBMERGED => 0x175,
            Self::ONESHOT_SUBMERGE => 0x176,
            Self::STATE_READY2H => 0x177,
            Self::STATE_READYBOW => 0x178,
            Self::ONESHOT_MOUNTSPECIAL => 0x179,
            Self::STATE_TALK => 0x17a,
            Self::STATE_FISHING => 0x17b,
            Self::ONESHOT_FISHING => 0x17c,
            Self::ONESHOT_LOOT => 0x17d,
            Self::STATE_WHIRLWIND => 0x17e,
            Self::STATE_DROWNED => 0x17f,
            Self::STATE_HOLD_BOW => 0x180,
            Self::STATE_HOLD_RIFLE => 0x181,
            Self::STATE_HOLD_THROWN => 0x182,
            Self::ONESHOT_DROWN => 0x183,
            Self::ONESHOT_STOMP => 0x184,
            Self::ONESHOT_ATTACKOFF => 0x185,
            Self::ONESHOT_ATTACKOFFPIERCE => 0x186,
            Self::STATE_ROAR => 0x187,
            Self::STATE_LAUGH => 0x188,
            Self::ONESHOT_CREATURE_SPECIAL => 0x189,
            Self::ONESHOT_JUMPLANDRUN => 0x18a,
            Self::ONESHOT_JUMPEND => 0x18b,
            Self::ONESHOT_TALK_NOSHEATHE => 0x18c,
            Self::ONESHOT_POINT_NOSHEATHE => 0x18d,
            Self::STATE_CANNIBALIZE => 0x18e,
            Self::ONESHOT_JUMPSTART => 0x18f,
            Self::STATE_DANCESPECIAL => 0x190,
            Self::ONESHOT_DANCESPECIAL => 0x191,
            Self::ONESHOT_CUSTOMSPELL01 => 0x192,
            Self::ONESHOT_CUSTOMSPELL02 => 0x193,
            Self::ONESHOT_CUSTOMSPELL03 => 0x194,
            Self::ONESHOT_CUSTOMSPELL04 => 0x195,
            Self::ONESHOT_CUSTOMSPELL05 => 0x196,
            Self::ONESHOT_CUSTOMSPELL06 => 0x197,
            Self::ONESHOT_CUSTOMSPELL07 => 0x198,
            Self::ONESHOT_CUSTOMSPELL08 => 0x199,
            Self::ONESHOT_CUSTOMSPELL09 => 0x19a,
            Self::ONESHOT_CUSTOMSPELL10 => 0x19b,
            Self::STATE_EXCLAIM => 0x19c,
            Self::STATE_SIT_CHAIR_MED => 0x19f,
            Self::STATE_SPELLEFFECT_HOLD => 0x1a6,
            Self::STATE_EAT_NO_SHEATHE => 0x1a7,
        }
    }

    pub const fn new() -> Self {
        Self::ONESHOT_NONE
    }

}

impl ConstantSized for Emote {}

impl MaximumPossibleSized for Emote {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for Emote {
    fn default() -> Self {
        Self::ONESHOT_NONE
    }
}

impl std::fmt::Display for Emote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ONESHOT_NONE => f.write_str("ONESHOT_NONE"),
            Self::ONESHOT_TALK => f.write_str("ONESHOT_TALK"),
            Self::ONESHOT_BOW => f.write_str("ONESHOT_BOW"),
            Self::ONESHOT_WAVE => f.write_str("ONESHOT_WAVE"),
            Self::ONESHOT_CHEER => f.write_str("ONESHOT_CHEER"),
            Self::ONESHOT_EXCLAMATION => f.write_str("ONESHOT_EXCLAMATION"),
            Self::ONESHOT_QUESTION => f.write_str("ONESHOT_QUESTION"),
            Self::ONESHOT_EAT => f.write_str("ONESHOT_EAT"),
            Self::STATE_DANCE => f.write_str("STATE_DANCE"),
            Self::ONESHOT_LAUGH => f.write_str("ONESHOT_LAUGH"),
            Self::STATE_SLEEP => f.write_str("STATE_SLEEP"),
            Self::STATE_SIT => f.write_str("STATE_SIT"),
            Self::ONESHOT_RUDE => f.write_str("ONESHOT_RUDE"),
            Self::ONESHOT_ROAR => f.write_str("ONESHOT_ROAR"),
            Self::ONESHOT_KNEEL => f.write_str("ONESHOT_KNEEL"),
            Self::ONESHOT_KISS => f.write_str("ONESHOT_KISS"),
            Self::ONESHOT_CRY => f.write_str("ONESHOT_CRY"),
            Self::ONESHOT_CHICKEN => f.write_str("ONESHOT_CHICKEN"),
            Self::ONESHOT_BEG => f.write_str("ONESHOT_BEG"),
            Self::ONESHOT_APPLAUD => f.write_str("ONESHOT_APPLAUD"),
            Self::ONESHOT_SHOUT => f.write_str("ONESHOT_SHOUT"),
            Self::ONESHOT_FLEX => f.write_str("ONESHOT_FLEX"),
            Self::ONESHOT_SHY => f.write_str("ONESHOT_SHY"),
            Self::ONESHOT_POINT => f.write_str("ONESHOT_POINT"),
            Self::STATE_STAND => f.write_str("STATE_STAND"),
            Self::STATE_READYUNARMED => f.write_str("STATE_READYUNARMED"),
            Self::STATE_WORK_SHEATHED => f.write_str("STATE_WORK_SHEATHED"),
            Self::STATE_POINT => f.write_str("STATE_POINT"),
            Self::STATE_NONE => f.write_str("STATE_NONE"),
            Self::ONESHOT_WOUND => f.write_str("ONESHOT_WOUND"),
            Self::ONESHOT_WOUNDCRITICAL => f.write_str("ONESHOT_WOUNDCRITICAL"),
            Self::ONESHOT_ATTACKUNARMED => f.write_str("ONESHOT_ATTACKUNARMED"),
            Self::ONESHOT_ATTACK1H => f.write_str("ONESHOT_ATTACK1H"),
            Self::ONESHOT_ATTACK2HTIGHT => f.write_str("ONESHOT_ATTACK2HTIGHT"),
            Self::ONESHOT_ATTACK2HLOOSE => f.write_str("ONESHOT_ATTACK2HLOOSE"),
            Self::ONESHOT_PARRYUNARMED => f.write_str("ONESHOT_PARRYUNARMED"),
            Self::ONESHOT_PARRYSHIELD => f.write_str("ONESHOT_PARRYSHIELD"),
            Self::ONESHOT_READYUNARMED => f.write_str("ONESHOT_READYUNARMED"),
            Self::ONESHOT_READY1H => f.write_str("ONESHOT_READY1H"),
            Self::ONESHOT_READYBOW => f.write_str("ONESHOT_READYBOW"),
            Self::ONESHOT_SPELLPRECAST => f.write_str("ONESHOT_SPELLPRECAST"),
            Self::ONESHOT_SPELLCAST => f.write_str("ONESHOT_SPELLCAST"),
            Self::ONESHOT_BATTLEROAR => f.write_str("ONESHOT_BATTLEROAR"),
            Self::ONESHOT_SPECIALATTACK1H => f.write_str("ONESHOT_SPECIALATTACK1H"),
            Self::ONESHOT_KICK => f.write_str("ONESHOT_KICK"),
            Self::ONESHOT_ATTACKTHROWN => f.write_str("ONESHOT_ATTACKTHROWN"),
            Self::STATE_STUN => f.write_str("STATE_STUN"),
            Self::STATE_DEAD => f.write_str("STATE_DEAD"),
            Self::ONESHOT_SALUTE => f.write_str("ONESHOT_SALUTE"),
            Self::STATE_KNEEL => f.write_str("STATE_KNEEL"),
            Self::STATE_USESTANDING => f.write_str("STATE_USESTANDING"),
            Self::ONESHOT_WAVE_NOSHEATHE => f.write_str("ONESHOT_WAVE_NOSHEATHE"),
            Self::ONESHOT_CHEER_NOSHEATHE => f.write_str("ONESHOT_CHEER_NOSHEATHE"),
            Self::ONESHOT_EAT_NOSHEATHE => f.write_str("ONESHOT_EAT_NOSHEATHE"),
            Self::STATE_STUN_NOSHEATHE => f.write_str("STATE_STUN_NOSHEATHE"),
            Self::ONESHOT_DANCE => f.write_str("ONESHOT_DANCE"),
            Self::ONESHOT_SALUTE_NOSHEATH => f.write_str("ONESHOT_SALUTE_NOSHEATH"),
            Self::STATE_USESTANDING_NOSHEATHE => f.write_str("STATE_USESTANDING_NOSHEATHE"),
            Self::ONESHOT_LAUGH_NOSHEATHE => f.write_str("ONESHOT_LAUGH_NOSHEATHE"),
            Self::STATE_WORK => f.write_str("STATE_WORK"),
            Self::STATE_SPELLPRECAST => f.write_str("STATE_SPELLPRECAST"),
            Self::ONESHOT_READYRIFLE => f.write_str("ONESHOT_READYRIFLE"),
            Self::STATE_READYRIFLE => f.write_str("STATE_READYRIFLE"),
            Self::STATE_WORK_MINING => f.write_str("STATE_WORK_MINING"),
            Self::STATE_WORK_CHOPWOOD => f.write_str("STATE_WORK_CHOPWOOD"),
            Self::STATE_APPLAUD => f.write_str("STATE_APPLAUD"),
            Self::ONESHOT_LIFTOFF => f.write_str("ONESHOT_LIFTOFF"),
            Self::ONESHOT_YES => f.write_str("ONESHOT_YES"),
            Self::ONESHOT_NO => f.write_str("ONESHOT_NO"),
            Self::ONESHOT_TRAIN => f.write_str("ONESHOT_TRAIN"),
            Self::ONESHOT_LAND => f.write_str("ONESHOT_LAND"),
            Self::STATE_AT_EASE => f.write_str("STATE_AT_EASE"),
            Self::STATE_READY1H => f.write_str("STATE_READY1H"),
            Self::STATE_SPELLKNEELSTART => f.write_str("STATE_SPELLKNEELSTART"),
            Self::STATE_SUBMERGED => f.write_str("STATE_SUBMERGED"),
            Self::ONESHOT_SUBMERGE => f.write_str("ONESHOT_SUBMERGE"),
            Self::STATE_READY2H => f.write_str("STATE_READY2H"),
            Self::STATE_READYBOW => f.write_str("STATE_READYBOW"),
            Self::ONESHOT_MOUNTSPECIAL => f.write_str("ONESHOT_MOUNTSPECIAL"),
            Self::STATE_TALK => f.write_str("STATE_TALK"),
            Self::STATE_FISHING => f.write_str("STATE_FISHING"),
            Self::ONESHOT_FISHING => f.write_str("ONESHOT_FISHING"),
            Self::ONESHOT_LOOT => f.write_str("ONESHOT_LOOT"),
            Self::STATE_WHIRLWIND => f.write_str("STATE_WHIRLWIND"),
            Self::STATE_DROWNED => f.write_str("STATE_DROWNED"),
            Self::STATE_HOLD_BOW => f.write_str("STATE_HOLD_BOW"),
            Self::STATE_HOLD_RIFLE => f.write_str("STATE_HOLD_RIFLE"),
            Self::STATE_HOLD_THROWN => f.write_str("STATE_HOLD_THROWN"),
            Self::ONESHOT_DROWN => f.write_str("ONESHOT_DROWN"),
            Self::ONESHOT_STOMP => f.write_str("ONESHOT_STOMP"),
            Self::ONESHOT_ATTACKOFF => f.write_str("ONESHOT_ATTACKOFF"),
            Self::ONESHOT_ATTACKOFFPIERCE => f.write_str("ONESHOT_ATTACKOFFPIERCE"),
            Self::STATE_ROAR => f.write_str("STATE_ROAR"),
            Self::STATE_LAUGH => f.write_str("STATE_LAUGH"),
            Self::ONESHOT_CREATURE_SPECIAL => f.write_str("ONESHOT_CREATURE_SPECIAL"),
            Self::ONESHOT_JUMPLANDRUN => f.write_str("ONESHOT_JUMPLANDRUN"),
            Self::ONESHOT_JUMPEND => f.write_str("ONESHOT_JUMPEND"),
            Self::ONESHOT_TALK_NOSHEATHE => f.write_str("ONESHOT_TALK_NOSHEATHE"),
            Self::ONESHOT_POINT_NOSHEATHE => f.write_str("ONESHOT_POINT_NOSHEATHE"),
            Self::STATE_CANNIBALIZE => f.write_str("STATE_CANNIBALIZE"),
            Self::ONESHOT_JUMPSTART => f.write_str("ONESHOT_JUMPSTART"),
            Self::STATE_DANCESPECIAL => f.write_str("STATE_DANCESPECIAL"),
            Self::ONESHOT_DANCESPECIAL => f.write_str("ONESHOT_DANCESPECIAL"),
            Self::ONESHOT_CUSTOMSPELL01 => f.write_str("ONESHOT_CUSTOMSPELL01"),
            Self::ONESHOT_CUSTOMSPELL02 => f.write_str("ONESHOT_CUSTOMSPELL02"),
            Self::ONESHOT_CUSTOMSPELL03 => f.write_str("ONESHOT_CUSTOMSPELL03"),
            Self::ONESHOT_CUSTOMSPELL04 => f.write_str("ONESHOT_CUSTOMSPELL04"),
            Self::ONESHOT_CUSTOMSPELL05 => f.write_str("ONESHOT_CUSTOMSPELL05"),
            Self::ONESHOT_CUSTOMSPELL06 => f.write_str("ONESHOT_CUSTOMSPELL06"),
            Self::ONESHOT_CUSTOMSPELL07 => f.write_str("ONESHOT_CUSTOMSPELL07"),
            Self::ONESHOT_CUSTOMSPELL08 => f.write_str("ONESHOT_CUSTOMSPELL08"),
            Self::ONESHOT_CUSTOMSPELL09 => f.write_str("ONESHOT_CUSTOMSPELL09"),
            Self::ONESHOT_CUSTOMSPELL10 => f.write_str("ONESHOT_CUSTOMSPELL10"),
            Self::STATE_EXCLAIM => f.write_str("STATE_EXCLAIM"),
            Self::STATE_SIT_CHAIR_MED => f.write_str("STATE_SIT_CHAIR_MED"),
            Self::STATE_SPELLEFFECT_HOLD => f.write_str("STATE_SPELLEFFECT_HOLD"),
            Self::STATE_EAT_NO_SHEATHE => f.write_str("STATE_EAT_NO_SHEATHE"),
        }
    }
}

impl TryFrom<u32> for Emote {
    type Error = TryFromEmoteError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ONESHOT_NONE),
            1 => Ok(Self::ONESHOT_TALK),
            2 => Ok(Self::ONESHOT_BOW),
            3 => Ok(Self::ONESHOT_WAVE),
            4 => Ok(Self::ONESHOT_CHEER),
            5 => Ok(Self::ONESHOT_EXCLAMATION),
            6 => Ok(Self::ONESHOT_QUESTION),
            7 => Ok(Self::ONESHOT_EAT),
            10 => Ok(Self::STATE_DANCE),
            11 => Ok(Self::ONESHOT_LAUGH),
            12 => Ok(Self::STATE_SLEEP),
            13 => Ok(Self::STATE_SIT),
            14 => Ok(Self::ONESHOT_RUDE),
            15 => Ok(Self::ONESHOT_ROAR),
            16 => Ok(Self::ONESHOT_KNEEL),
            17 => Ok(Self::ONESHOT_KISS),
            18 => Ok(Self::ONESHOT_CRY),
            19 => Ok(Self::ONESHOT_CHICKEN),
            20 => Ok(Self::ONESHOT_BEG),
            21 => Ok(Self::ONESHOT_APPLAUD),
            22 => Ok(Self::ONESHOT_SHOUT),
            23 => Ok(Self::ONESHOT_FLEX),
            24 => Ok(Self::ONESHOT_SHY),
            25 => Ok(Self::ONESHOT_POINT),
            26 => Ok(Self::STATE_STAND),
            27 => Ok(Self::STATE_READYUNARMED),
            28 => Ok(Self::STATE_WORK_SHEATHED),
            29 => Ok(Self::STATE_POINT),
            30 => Ok(Self::STATE_NONE),
            33 => Ok(Self::ONESHOT_WOUND),
            34 => Ok(Self::ONESHOT_WOUNDCRITICAL),
            35 => Ok(Self::ONESHOT_ATTACKUNARMED),
            36 => Ok(Self::ONESHOT_ATTACK1H),
            37 => Ok(Self::ONESHOT_ATTACK2HTIGHT),
            38 => Ok(Self::ONESHOT_ATTACK2HLOOSE),
            39 => Ok(Self::ONESHOT_PARRYUNARMED),
            43 => Ok(Self::ONESHOT_PARRYSHIELD),
            44 => Ok(Self::ONESHOT_READYUNARMED),
            45 => Ok(Self::ONESHOT_READY1H),
            48 => Ok(Self::ONESHOT_READYBOW),
            50 => Ok(Self::ONESHOT_SPELLPRECAST),
            51 => Ok(Self::ONESHOT_SPELLCAST),
            53 => Ok(Self::ONESHOT_BATTLEROAR),
            54 => Ok(Self::ONESHOT_SPECIALATTACK1H),
            60 => Ok(Self::ONESHOT_KICK),
            61 => Ok(Self::ONESHOT_ATTACKTHROWN),
            64 => Ok(Self::STATE_STUN),
            65 => Ok(Self::STATE_DEAD),
            66 => Ok(Self::ONESHOT_SALUTE),
            68 => Ok(Self::STATE_KNEEL),
            69 => Ok(Self::STATE_USESTANDING),
            70 => Ok(Self::ONESHOT_WAVE_NOSHEATHE),
            71 => Ok(Self::ONESHOT_CHEER_NOSHEATHE),
            92 => Ok(Self::ONESHOT_EAT_NOSHEATHE),
            93 => Ok(Self::STATE_STUN_NOSHEATHE),
            94 => Ok(Self::ONESHOT_DANCE),
            113 => Ok(Self::ONESHOT_SALUTE_NOSHEATH),
            133 => Ok(Self::STATE_USESTANDING_NOSHEATHE),
            153 => Ok(Self::ONESHOT_LAUGH_NOSHEATHE),
            173 => Ok(Self::STATE_WORK),
            193 => Ok(Self::STATE_SPELLPRECAST),
            213 => Ok(Self::ONESHOT_READYRIFLE),
            214 => Ok(Self::STATE_READYRIFLE),
            233 => Ok(Self::STATE_WORK_MINING),
            234 => Ok(Self::STATE_WORK_CHOPWOOD),
            253 => Ok(Self::STATE_APPLAUD),
            254 => Ok(Self::ONESHOT_LIFTOFF),
            273 => Ok(Self::ONESHOT_YES),
            274 => Ok(Self::ONESHOT_NO),
            275 => Ok(Self::ONESHOT_TRAIN),
            293 => Ok(Self::ONESHOT_LAND),
            313 => Ok(Self::STATE_AT_EASE),
            333 => Ok(Self::STATE_READY1H),
            353 => Ok(Self::STATE_SPELLKNEELSTART),
            373 => Ok(Self::STATE_SUBMERGED),
            374 => Ok(Self::ONESHOT_SUBMERGE),
            375 => Ok(Self::STATE_READY2H),
            376 => Ok(Self::STATE_READYBOW),
            377 => Ok(Self::ONESHOT_MOUNTSPECIAL),
            378 => Ok(Self::STATE_TALK),
            379 => Ok(Self::STATE_FISHING),
            380 => Ok(Self::ONESHOT_FISHING),
            381 => Ok(Self::ONESHOT_LOOT),
            382 => Ok(Self::STATE_WHIRLWIND),
            383 => Ok(Self::STATE_DROWNED),
            384 => Ok(Self::STATE_HOLD_BOW),
            385 => Ok(Self::STATE_HOLD_RIFLE),
            386 => Ok(Self::STATE_HOLD_THROWN),
            387 => Ok(Self::ONESHOT_DROWN),
            388 => Ok(Self::ONESHOT_STOMP),
            389 => Ok(Self::ONESHOT_ATTACKOFF),
            390 => Ok(Self::ONESHOT_ATTACKOFFPIERCE),
            391 => Ok(Self::STATE_ROAR),
            392 => Ok(Self::STATE_LAUGH),
            393 => Ok(Self::ONESHOT_CREATURE_SPECIAL),
            394 => Ok(Self::ONESHOT_JUMPLANDRUN),
            395 => Ok(Self::ONESHOT_JUMPEND),
            396 => Ok(Self::ONESHOT_TALK_NOSHEATHE),
            397 => Ok(Self::ONESHOT_POINT_NOSHEATHE),
            398 => Ok(Self::STATE_CANNIBALIZE),
            399 => Ok(Self::ONESHOT_JUMPSTART),
            400 => Ok(Self::STATE_DANCESPECIAL),
            401 => Ok(Self::ONESHOT_DANCESPECIAL),
            402 => Ok(Self::ONESHOT_CUSTOMSPELL01),
            403 => Ok(Self::ONESHOT_CUSTOMSPELL02),
            404 => Ok(Self::ONESHOT_CUSTOMSPELL03),
            405 => Ok(Self::ONESHOT_CUSTOMSPELL04),
            406 => Ok(Self::ONESHOT_CUSTOMSPELL05),
            407 => Ok(Self::ONESHOT_CUSTOMSPELL06),
            408 => Ok(Self::ONESHOT_CUSTOMSPELL07),
            409 => Ok(Self::ONESHOT_CUSTOMSPELL08),
            410 => Ok(Self::ONESHOT_CUSTOMSPELL09),
            411 => Ok(Self::ONESHOT_CUSTOMSPELL10),
            412 => Ok(Self::STATE_EXCLAIM),
            415 => Ok(Self::STATE_SIT_CHAIR_MED),
            422 => Ok(Self::STATE_SPELLEFFECT_HOLD),
            423 => Ok(Self::STATE_EAT_NO_SHEATHE),
            _ => Err(TryFromEmoteError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromEmoteError {
    value: u32,
}

impl TryFromEmoteError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum EmoteError {
    Read(std::io::Error),
    TryFrom(TryFromEmoteError),
}

impl std::error::Error for EmoteError {}
impl std::fmt::Display for TryFromEmoteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'Emote': '{}'", self.value))
    }
}

impl std::fmt::Display for EmoteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for EmoteError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromEmoteError> for EmoteError {
    fn from(value: TryFromEmoteError) -> Self {
        Self::TryFrom(value)
    }
}

