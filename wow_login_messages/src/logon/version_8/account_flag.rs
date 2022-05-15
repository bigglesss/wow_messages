use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable};

#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct AccountFlag {
    inner: u32,
}

impl AccountFlag {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

}

impl AccountFlag {
    pub const GM: u32 = 0x01;
    pub const TRIAL: u32 = 0x08;
    pub const PROPASS: u32 = 0x800000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::GM
                | Self::TRIAL
                | Self::PROPASS
        }
    }

    pub const fn is_GM(&self) -> bool {
        (self.inner & Self::GM) != 0
    }

    pub const fn new_GM() -> Self {
        Self { inner: Self::GM }
    }

    pub fn set_GM(&mut self) -> Self {
        self.inner |= Self::GM;
        *self
    }

    pub fn clear_GM(&mut self) -> Self {
        self.inner &= Self::GM.reverse_bits();
        *self
    }

    pub const fn is_TRIAL(&self) -> bool {
        (self.inner & Self::TRIAL) != 0
    }

    pub const fn new_TRIAL() -> Self {
        Self { inner: Self::TRIAL }
    }

    pub fn set_TRIAL(&mut self) -> Self {
        self.inner |= Self::TRIAL;
        *self
    }

    pub fn clear_TRIAL(&mut self) -> Self {
        self.inner &= Self::TRIAL.reverse_bits();
        *self
    }

    pub const fn is_PROPASS(&self) -> bool {
        (self.inner & Self::PROPASS) != 0
    }

    pub const fn new_PROPASS() -> Self {
        Self { inner: Self::PROPASS }
    }

    pub fn set_PROPASS(&mut self) -> Self {
        self.inner |= Self::PROPASS;
        *self
    }

    pub fn clear_PROPASS(&mut self) -> Self {
        self.inner &= Self::PROPASS.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl ConstantSized for AccountFlag {}

impl MaximumPossibleSized for AccountFlag {
    fn maximum_possible_size() -> usize {
        4
    }
}

