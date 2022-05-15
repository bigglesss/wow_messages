use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable};

#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct SecurityFlag {
    inner: u8,
}

impl SecurityFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

}

impl SecurityFlag {
    pub(crate) const NONE: u8 = 0x00;
    pub(crate) const PIN: u8 = 0x01;
    pub(crate) const UNKNOWN0: u8 = 0x02;
    pub(crate) const AUTHENTICATOR: u8 = 0x04;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::PIN
                | Self::UNKNOWN0
                | Self::AUTHENTICATOR
        }
    }

    pub const fn is_NONE(&self) -> bool {
        // Underlying value is 0
        self.inner == Self::NONE
    }

    pub const fn new_NONE() -> Self {
        Self { inner: Self::NONE }
    }

    pub fn set_NONE(&mut self) -> Self {
        self.inner |= Self::NONE;
        *self
    }

    pub fn clear_NONE(&mut self) -> Self {
        self.inner &= Self::NONE.reverse_bits();
        *self
    }

    pub const fn is_PIN(&self) -> bool {
        (self.inner & Self::PIN) != 0
    }

    pub const fn new_PIN() -> Self {
        Self { inner: Self::PIN }
    }

    pub fn set_PIN(&mut self) -> Self {
        self.inner |= Self::PIN;
        *self
    }

    pub fn clear_PIN(&mut self) -> Self {
        self.inner &= Self::PIN.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN0(&self) -> bool {
        (self.inner & Self::UNKNOWN0) != 0
    }

    pub const fn new_UNKNOWN0() -> Self {
        Self { inner: Self::UNKNOWN0 }
    }

    pub fn set_UNKNOWN0(&mut self) -> Self {
        self.inner |= Self::UNKNOWN0;
        *self
    }

    pub fn clear_UNKNOWN0(&mut self) -> Self {
        self.inner &= Self::UNKNOWN0.reverse_bits();
        *self
    }

    pub const fn is_AUTHENTICATOR(&self) -> bool {
        (self.inner & Self::AUTHENTICATOR) != 0
    }

    pub const fn new_AUTHENTICATOR() -> Self {
        Self { inner: Self::AUTHENTICATOR }
    }

    pub fn set_AUTHENTICATOR(&mut self) -> Self {
        self.inner |= Self::AUTHENTICATOR;
        *self
    }

    pub fn clear_AUTHENTICATOR(&mut self) -> Self {
        self.inner &= Self::AUTHENTICATOR.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}

