use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable};

#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct GroupMemberOnlineStatus {
    inner: u8,
}

impl GroupMemberOnlineStatus {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

}

impl ReadableAndWritable for GroupMemberOnlineStatus {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let inner = crate::util::read_u8_le(r)?;
        Ok(Self { inner })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.inner.to_le_bytes())?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = crate::util::tokio_read_u8_le(r).await?;
            Ok(Self { inner })
        })
    }

    #[cfg(feature = "async_tokio")]
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
            w.write_all(&self.inner.to_le_bytes()).await?;
            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = crate::util::astd_read_u8_le(r).await?;
            Ok(Self { inner })
        })
    }

    #[cfg(feature = "async_std")]
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
            w.write_all(&self.inner.to_le_bytes()).await?;
            Ok(())
        })
    }

}

impl GroupMemberOnlineStatus {
    pub const OFFLINE: u8 = 0x00;
    pub const ONLINE: u8 = 0x01;
    pub const PVP: u8 = 0x02;
    pub const DEAD: u8 = 0x04;
    pub const GHOST: u8 = 0x08;
    pub const PVP_FFA: u8 = 0x10;
    pub const ZONE_OUT: u8 = 0x20;
    pub const AFK: u8 = 0x40;
    pub const DND: u8 = 0x80;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::OFFLINE
                | Self::ONLINE
                | Self::PVP
                | Self::DEAD
                | Self::GHOST
                | Self::PVP_FFA
                | Self::ZONE_OUT
                | Self::AFK
                | Self::DND
        }
    }

    pub const fn is_OFFLINE(&self) -> bool {
        // Underlying value is 0
        self.inner == Self::OFFLINE
    }

    pub const fn new_OFFLINE() -> Self {
        Self { inner: Self::OFFLINE }
    }

    pub fn set_OFFLINE(&mut self) -> Self {
        self.inner |= Self::OFFLINE;
        *self
    }

    pub fn clear_OFFLINE(&mut self) -> Self {
        self.inner &= Self::OFFLINE.reverse_bits();
        *self
    }

    pub const fn is_ONLINE(&self) -> bool {
        (self.inner & Self::ONLINE) != 0
    }

    pub const fn new_ONLINE() -> Self {
        Self { inner: Self::ONLINE }
    }

    pub fn set_ONLINE(&mut self) -> Self {
        self.inner |= Self::ONLINE;
        *self
    }

    pub fn clear_ONLINE(&mut self) -> Self {
        self.inner &= Self::ONLINE.reverse_bits();
        *self
    }

    pub const fn is_PVP(&self) -> bool {
        (self.inner & Self::PVP) != 0
    }

    pub const fn new_PVP() -> Self {
        Self { inner: Self::PVP }
    }

    pub fn set_PVP(&mut self) -> Self {
        self.inner |= Self::PVP;
        *self
    }

    pub fn clear_PVP(&mut self) -> Self {
        self.inner &= Self::PVP.reverse_bits();
        *self
    }

    pub const fn is_DEAD(&self) -> bool {
        (self.inner & Self::DEAD) != 0
    }

    pub const fn new_DEAD() -> Self {
        Self { inner: Self::DEAD }
    }

    pub fn set_DEAD(&mut self) -> Self {
        self.inner |= Self::DEAD;
        *self
    }

    pub fn clear_DEAD(&mut self) -> Self {
        self.inner &= Self::DEAD.reverse_bits();
        *self
    }

    pub const fn is_GHOST(&self) -> bool {
        (self.inner & Self::GHOST) != 0
    }

    pub const fn new_GHOST() -> Self {
        Self { inner: Self::GHOST }
    }

    pub fn set_GHOST(&mut self) -> Self {
        self.inner |= Self::GHOST;
        *self
    }

    pub fn clear_GHOST(&mut self) -> Self {
        self.inner &= Self::GHOST.reverse_bits();
        *self
    }

    pub const fn is_PVP_FFA(&self) -> bool {
        (self.inner & Self::PVP_FFA) != 0
    }

    pub const fn new_PVP_FFA() -> Self {
        Self { inner: Self::PVP_FFA }
    }

    pub fn set_PVP_FFA(&mut self) -> Self {
        self.inner |= Self::PVP_FFA;
        *self
    }

    pub fn clear_PVP_FFA(&mut self) -> Self {
        self.inner &= Self::PVP_FFA.reverse_bits();
        *self
    }

    pub const fn is_ZONE_OUT(&self) -> bool {
        (self.inner & Self::ZONE_OUT) != 0
    }

    pub const fn new_ZONE_OUT() -> Self {
        Self { inner: Self::ZONE_OUT }
    }

    pub fn set_ZONE_OUT(&mut self) -> Self {
        self.inner |= Self::ZONE_OUT;
        *self
    }

    pub fn clear_ZONE_OUT(&mut self) -> Self {
        self.inner &= Self::ZONE_OUT.reverse_bits();
        *self
    }

    pub const fn is_AFK(&self) -> bool {
        (self.inner & Self::AFK) != 0
    }

    pub const fn new_AFK() -> Self {
        Self { inner: Self::AFK }
    }

    pub fn set_AFK(&mut self) -> Self {
        self.inner |= Self::AFK;
        *self
    }

    pub fn clear_AFK(&mut self) -> Self {
        self.inner &= Self::AFK.reverse_bits();
        *self
    }

    pub const fn is_DND(&self) -> bool {
        (self.inner & Self::DND) != 0
    }

    pub const fn new_DND() -> Self {
        Self { inner: Self::DND }
    }

    pub fn set_DND(&mut self) -> Self {
        self.inner |= Self::DND;
        *self
    }

    pub fn clear_DND(&mut self) -> Self {
        self.inner &= Self::DND.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl ConstantSized for GroupMemberOnlineStatus {}

impl MaximumPossibleSized for GroupMemberOnlineStatus {
    fn maximum_possible_size() -> usize {
        1
    }
}

