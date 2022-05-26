use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct QuestDetailsEmote {
    pub emote: u32,
    pub emote_delay_in_msecs: u32,
}

impl QuestDetailsEmote {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        // emote_delay_in_msecs: u32
        w.write_all(&self.emote_delay_in_msecs.to_le_bytes())?;

        Ok(())
    }
}

impl QuestDetailsEmote {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // emote: u32
        let emote = crate::util::read_u32_le(r)?;

        // emote_delay_in_msecs: u32
        let emote_delay_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            emote,
            emote_delay_in_msecs,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // emote: u32
        let emote = crate::util::tokio_read_u32_le(r).await?;

        // emote_delay_in_msecs: u32
        let emote_delay_in_msecs = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            emote,
            emote_delay_in_msecs,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // emote: u32
        let emote = crate::util::astd_read_u32_le(r).await?;

        // emote_delay_in_msecs: u32
        let emote_delay_in_msecs = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            emote,
            emote_delay_in_msecs,
        })
    }

}

