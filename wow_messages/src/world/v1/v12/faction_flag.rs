use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/add_messages.wowm:1444`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/add_messages.wowm#L1444):
/// ```text
/// flag FactionFlag : u8 {
///     VISIBLE = 0x01;
///     AT_WAR = 0x02;
///     HIDDEN = 0x04;
///     INVISIBLE_FORCED = 0x08;
///     PEACE_FORCED = 0x10;
///     INACTIVE = 0x20;
///     RIVAL = 0x40;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct FactionFlag {
    inner: u8,
}

impl FactionFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

}

impl ReadableAndWritable for FactionFlag {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let inner = crate::util::read_u8_le(r)?;
        Ok(Self { inner })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.inner.to_le_bytes())?;
        Ok(())
    }

}

impl FactionFlag {
    pub const VISIBLE: u8 = 0x01;
    pub const AT_WAR: u8 = 0x02;
    pub const HIDDEN: u8 = 0x04;
    pub const INVISIBLE_FORCED: u8 = 0x08;
    pub const PEACE_FORCED: u8 = 0x10;
    pub const INACTIVE: u8 = 0x20;
    pub const RIVAL: u8 = 0x40;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::VISIBLE
                | Self::AT_WAR
                | Self::HIDDEN
                | Self::INVISIBLE_FORCED
                | Self::PEACE_FORCED
                | Self::INACTIVE
                | Self::RIVAL
        }
    }

    pub const fn is_VISIBLE(&self) -> bool {
        (self.inner & Self::VISIBLE) != 0
    }

    pub const fn new_VISIBLE() -> Self {
        Self { inner: Self::VISIBLE }
    }

    pub fn set_VISIBLE(&mut self) -> Self {
        self.inner |= Self::VISIBLE;
        *self
    }

    pub fn clear_VISIBLE(&mut self) -> Self {
        self.inner &= Self::VISIBLE.reverse_bits();
        *self
    }

    pub const fn is_AT_WAR(&self) -> bool {
        (self.inner & Self::AT_WAR) != 0
    }

    pub const fn new_AT_WAR() -> Self {
        Self { inner: Self::AT_WAR }
    }

    pub fn set_AT_WAR(&mut self) -> Self {
        self.inner |= Self::AT_WAR;
        *self
    }

    pub fn clear_AT_WAR(&mut self) -> Self {
        self.inner &= Self::AT_WAR.reverse_bits();
        *self
    }

    pub const fn is_HIDDEN(&self) -> bool {
        (self.inner & Self::HIDDEN) != 0
    }

    pub const fn new_HIDDEN() -> Self {
        Self { inner: Self::HIDDEN }
    }

    pub fn set_HIDDEN(&mut self) -> Self {
        self.inner |= Self::HIDDEN;
        *self
    }

    pub fn clear_HIDDEN(&mut self) -> Self {
        self.inner &= Self::HIDDEN.reverse_bits();
        *self
    }

    pub const fn is_INVISIBLE_FORCED(&self) -> bool {
        (self.inner & Self::INVISIBLE_FORCED) != 0
    }

    pub const fn new_INVISIBLE_FORCED() -> Self {
        Self { inner: Self::INVISIBLE_FORCED }
    }

    pub fn set_INVISIBLE_FORCED(&mut self) -> Self {
        self.inner |= Self::INVISIBLE_FORCED;
        *self
    }

    pub fn clear_INVISIBLE_FORCED(&mut self) -> Self {
        self.inner &= Self::INVISIBLE_FORCED.reverse_bits();
        *self
    }

    pub const fn is_PEACE_FORCED(&self) -> bool {
        (self.inner & Self::PEACE_FORCED) != 0
    }

    pub const fn new_PEACE_FORCED() -> Self {
        Self { inner: Self::PEACE_FORCED }
    }

    pub fn set_PEACE_FORCED(&mut self) -> Self {
        self.inner |= Self::PEACE_FORCED;
        *self
    }

    pub fn clear_PEACE_FORCED(&mut self) -> Self {
        self.inner &= Self::PEACE_FORCED.reverse_bits();
        *self
    }

    pub const fn is_INACTIVE(&self) -> bool {
        (self.inner & Self::INACTIVE) != 0
    }

    pub const fn new_INACTIVE() -> Self {
        Self { inner: Self::INACTIVE }
    }

    pub fn set_INACTIVE(&mut self) -> Self {
        self.inner |= Self::INACTIVE;
        *self
    }

    pub fn clear_INACTIVE(&mut self) -> Self {
        self.inner &= Self::INACTIVE.reverse_bits();
        *self
    }

    pub const fn is_RIVAL(&self) -> bool {
        (self.inner & Self::RIVAL) != 0
    }

    pub const fn new_RIVAL() -> Self {
        Self { inner: Self::RIVAL }
    }

    pub fn set_RIVAL(&mut self) -> Self {
        self.inner |= Self::RIVAL;
        *self
    }

    pub fn clear_RIVAL(&mut self) -> Self {
        self.inner &= Self::RIVAL.reverse_bits();
        *self
    }

    pub const fn as_u8(&self) -> u8 {
        self.inner
    }

}

impl ConstantSized for FactionFlag {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for FactionFlag {
    fn maximum_possible_size() -> usize {
        1
    }
}

