use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Friend, FriendError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_FRIEND_LIST {
    pub friends: Vec<Friend>,
}

impl ServerMessageWrite for SMSG_FRIEND_LIST {}

impl MessageBody for SMSG_FRIEND_LIST {
    const OPCODE: u16 = 0x0067;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_FRIEND_LISTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_friends: u8
        let amount_of_friends = crate::util::read_u8_le(r)?;

        // friends: Friend[amount_of_friends]
        let mut friends = Vec::with_capacity(amount_of_friends as usize);
        for i in 0..amount_of_friends {
            friends.push(Friend::read(r)?);
        }

        Ok(Self {
            friends,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_friends: u8
        w.write_all(&(self.friends.len() as u8).to_le_bytes())?;

        // friends: Friend[amount_of_friends]
        for i in self.friends.iter() {
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
            // amount_of_friends: u8
            let amount_of_friends = crate::util::tokio_read_u8_le(r).await?;

            // friends: Friend[amount_of_friends]
            let mut friends = Vec::with_capacity(amount_of_friends as usize);
            for i in 0..amount_of_friends {
                friends.push(Friend::tokio_read(r).await?);
            }

            Ok(Self {
                friends,
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
            // amount_of_friends: u8
            w.write_all(&(self.friends.len() as u8).to_le_bytes()).await?;

            // friends: Friend[amount_of_friends]
            for i in self.friends.iter() {
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
            // amount_of_friends: u8
            let amount_of_friends = crate::util::astd_read_u8_le(r).await?;

            // friends: Friend[amount_of_friends]
            let mut friends = Vec::with_capacity(amount_of_friends as usize);
            for i in 0..amount_of_friends {
                friends.push(Friend::astd_read(r).await?);
            }

            Ok(Self {
                friends,
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
            // amount_of_friends: u8
            w.write_all(&(self.friends.len() as u8).to_le_bytes()).await?;

            // friends: Friend[amount_of_friends]
            for i in self.friends.iter() {
                i.astd_write(w).await?;
            }

            Ok(())
        })
    }

}

impl SMSG_FRIEND_LIST {
    pub fn size(&self) -> usize {
        0
        + 1 // amount_of_friends: u8
        + self.friends.iter().fold(0, |acc, x| acc + x.size()) // friends: Friend[amount_of_friends]
    }
}

#[derive(Debug)]
pub enum SMSG_FRIEND_LISTError {
    Io(std::io::Error),
    Friend(FriendError),
}

impl std::error::Error for SMSG_FRIEND_LISTError {}
impl std::fmt::Display for SMSG_FRIEND_LISTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Friend(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_FRIEND_LISTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<FriendError> for SMSG_FRIEND_LISTError {
    fn from(e: FriendError) -> Self {
        Self::Friend(e)
    }
}

