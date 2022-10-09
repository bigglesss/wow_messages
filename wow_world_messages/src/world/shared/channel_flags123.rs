/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_channel_list.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_channel_list.wowm#L3):
/// ```text
/// flag ChannelFlags : u8 {
///     NONE = 0x00;
///     CUSTOM = 0x01;
///     TRADE = 0x04;
///     NOT_LFG = 0x08;
///     GENERAL = 0x10;
///     CITY = 0x20;
///     LFG = 0x40;
///     VOICE = 0x80;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
pub struct ChannelFlags {
    inner: u8,
}

impl ChannelFlags {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub(crate) const NONE: u8 = 0x00;
    pub(crate) const CUSTOM: u8 = 0x01;
    pub(crate) const TRADE: u8 = 0x04;
    pub(crate) const NOT_LFG: u8 = 0x08;
    pub(crate) const GENERAL: u8 = 0x10;
    pub(crate) const CITY: u8 = 0x20;
    pub(crate) const LFG: u8 = 0x40;
    pub(crate) const VOICE: u8 = 0x80;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::CUSTOM
                | Self::TRADE
                | Self::NOT_LFG
                | Self::GENERAL
                | Self::CITY
                | Self::LFG
                | Self::VOICE
        }
    }

    pub const fn is_CUSTOM(&self) -> bool {
        (self.inner & Self::CUSTOM) != 0
    }

    pub const fn new_CUSTOM() -> Self {
        Self { inner: Self::CUSTOM }
    }

    pub fn set_CUSTOM(&mut self) -> Self {
        self.inner |= Self::CUSTOM;
        *self
    }

    pub fn clear_CUSTOM(&mut self) -> Self {
        self.inner &= Self::CUSTOM.reverse_bits();
        *self
    }

    pub const fn is_TRADE(&self) -> bool {
        (self.inner & Self::TRADE) != 0
    }

    pub const fn new_TRADE() -> Self {
        Self { inner: Self::TRADE }
    }

    pub fn set_TRADE(&mut self) -> Self {
        self.inner |= Self::TRADE;
        *self
    }

    pub fn clear_TRADE(&mut self) -> Self {
        self.inner &= Self::TRADE.reverse_bits();
        *self
    }

    pub const fn is_NOT_LFG(&self) -> bool {
        (self.inner & Self::NOT_LFG) != 0
    }

    pub const fn new_NOT_LFG() -> Self {
        Self { inner: Self::NOT_LFG }
    }

    pub fn set_NOT_LFG(&mut self) -> Self {
        self.inner |= Self::NOT_LFG;
        *self
    }

    pub fn clear_NOT_LFG(&mut self) -> Self {
        self.inner &= Self::NOT_LFG.reverse_bits();
        *self
    }

    pub const fn is_GENERAL(&self) -> bool {
        (self.inner & Self::GENERAL) != 0
    }

    pub const fn new_GENERAL() -> Self {
        Self { inner: Self::GENERAL }
    }

    pub fn set_GENERAL(&mut self) -> Self {
        self.inner |= Self::GENERAL;
        *self
    }

    pub fn clear_GENERAL(&mut self) -> Self {
        self.inner &= Self::GENERAL.reverse_bits();
        *self
    }

    pub const fn is_CITY(&self) -> bool {
        (self.inner & Self::CITY) != 0
    }

    pub const fn new_CITY() -> Self {
        Self { inner: Self::CITY }
    }

    pub fn set_CITY(&mut self) -> Self {
        self.inner |= Self::CITY;
        *self
    }

    pub fn clear_CITY(&mut self) -> Self {
        self.inner &= Self::CITY.reverse_bits();
        *self
    }

    pub const fn is_LFG(&self) -> bool {
        (self.inner & Self::LFG) != 0
    }

    pub const fn new_LFG() -> Self {
        Self { inner: Self::LFG }
    }

    pub fn set_LFG(&mut self) -> Self {
        self.inner |= Self::LFG;
        *self
    }

    pub fn clear_LFG(&mut self) -> Self {
        self.inner &= Self::LFG.reverse_bits();
        *self
    }

    pub const fn is_VOICE(&self) -> bool {
        (self.inner & Self::VOICE) != 0
    }

    pub const fn new_VOICE() -> Self {
        Self { inner: Self::VOICE }
    }

    pub fn set_VOICE(&mut self) -> Self {
        self.inner |= Self::VOICE;
        *self
    }

    pub fn clear_VOICE(&mut self) -> Self {
        self.inner &= Self::VOICE.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}
