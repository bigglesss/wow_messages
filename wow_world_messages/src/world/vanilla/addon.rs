use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::AddonType;
use crate::world::vanilla::InfoBlock;
use crate::world::vanilla::KeyVersion;
use crate::world::vanilla::UrlInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm:42`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm#L42):
/// ```text
/// struct Addon {
///     AddonType addon_type;
///     InfoBlock info_block;
///     if (info_block == AVAILABLE) {
///         KeyVersion key_version;
///         if (key_version != ZERO) {
///             u8[256] public_key;
///         }
///         u32 update_available_flag;
///     }
///     UrlInfo url_info;
///     if (url_info == AVAILABLE) {
///         CString url;
///     }
/// }
/// ```
pub struct Addon {
    pub addon_type: AddonType,
    pub info_block: Addon_InfoBlock,
    pub url_info: Addon_UrlInfo,
}

impl Addon {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // addon_type: AddonType
        w.write_all(&(self.addon_type.as_int() as u8).to_le_bytes())?;

        // info_block: InfoBlock
        w.write_all(&(self.info_block.as_int() as u8).to_le_bytes())?;

        match &self.info_block {
            Addon_InfoBlock::Unavailable => {}
            Addon_InfoBlock::Available {
                key_version,
                update_available_flag,
            } => {
                // key_version: KeyVersion
                w.write_all(&(key_version.as_int() as u8).to_le_bytes())?;

                match &key_version {
                    Addon_KeyVersion::Zero => {}
                    Addon_KeyVersion::One {
                        public_key,
                    } => {
                        // public_key: u8[256]
                        for i in public_key.iter() {
                            w.write_all(&i.to_le_bytes())?;
                        }

                    }
                    Addon_KeyVersion::Two {
                        public_key,
                    } => {
                        // public_key: u8[256]
                        for i in public_key.iter() {
                            w.write_all(&i.to_le_bytes())?;
                        }

                    }
                    Addon_KeyVersion::Three {
                        public_key,
                    } => {
                        // public_key: u8[256]
                        for i in public_key.iter() {
                            w.write_all(&i.to_le_bytes())?;
                        }

                    }
                    Addon_KeyVersion::Four {
                        public_key,
                    } => {
                        // public_key: u8[256]
                        for i in public_key.iter() {
                            w.write_all(&i.to_le_bytes())?;
                        }

                    }
                    Addon_KeyVersion::Five {
                        public_key,
                    } => {
                        // public_key: u8[256]
                        for i in public_key.iter() {
                            w.write_all(&i.to_le_bytes())?;
                        }

                    }
                    Addon_KeyVersion::Six {
                        public_key,
                    } => {
                        // public_key: u8[256]
                        for i in public_key.iter() {
                            w.write_all(&i.to_le_bytes())?;
                        }

                    }
                    Addon_KeyVersion::Seven {
                        public_key,
                    } => {
                        // public_key: u8[256]
                        for i in public_key.iter() {
                            w.write_all(&i.to_le_bytes())?;
                        }

                    }
                    Addon_KeyVersion::Eight {
                        public_key,
                    } => {
                        // public_key: u8[256]
                        for i in public_key.iter() {
                            w.write_all(&i.to_le_bytes())?;
                        }

                    }
                    Addon_KeyVersion::Nine {
                        public_key,
                    } => {
                        // public_key: u8[256]
                        for i in public_key.iter() {
                            w.write_all(&i.to_le_bytes())?;
                        }

                    }
                }

                // update_available_flag: u32
                w.write_all(&update_available_flag.to_le_bytes())?;

            }
        }

        // url_info: UrlInfo
        w.write_all(&(self.url_info.as_int() as u8).to_le_bytes())?;

        match &self.url_info {
            Addon_UrlInfo::Unavailable => {}
            Addon_UrlInfo::Available {
                url,
            } => {
                // url: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(url.as_bytes().iter().rev().next(), Some(&0_u8), "String `url` must not be null-terminated.");
                w.write_all(url.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
        }

        assert_eq!(self.size() as usize, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
}

impl Addon {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // addon_type: AddonType
        let addon_type: AddonType = crate::util::read_u8_le(r)?.try_into()?;

        // info_block: InfoBlock
        let info_block: InfoBlock = crate::util::read_u8_le(r)?.try_into()?;

        let info_block_if = match info_block {
            InfoBlock::Unavailable => Addon_InfoBlock::Unavailable,
            InfoBlock::Available => {
                // key_version: KeyVersion
                let key_version: KeyVersion = crate::util::read_u8_le(r)?.try_into()?;

                let key_version_if = match key_version {
                    KeyVersion::Zero => Addon_KeyVersion::Zero,
                    KeyVersion::One => {
                        // public_key: u8[256]
                        let mut public_key = [0_u8; 256];
                        r.read_exact(&mut public_key)?;

                        Addon_KeyVersion::One {
                            public_key,
                        }
                    }
                    KeyVersion::Two => {
                        // public_key: u8[256]
                        let mut public_key = [0_u8; 256];
                        r.read_exact(&mut public_key)?;

                        Addon_KeyVersion::Two {
                            public_key,
                        }
                    }
                    KeyVersion::Three => {
                        // public_key: u8[256]
                        let mut public_key = [0_u8; 256];
                        r.read_exact(&mut public_key)?;

                        Addon_KeyVersion::Three {
                            public_key,
                        }
                    }
                    KeyVersion::Four => {
                        // public_key: u8[256]
                        let mut public_key = [0_u8; 256];
                        r.read_exact(&mut public_key)?;

                        Addon_KeyVersion::Four {
                            public_key,
                        }
                    }
                    KeyVersion::Five => {
                        // public_key: u8[256]
                        let mut public_key = [0_u8; 256];
                        r.read_exact(&mut public_key)?;

                        Addon_KeyVersion::Five {
                            public_key,
                        }
                    }
                    KeyVersion::Six => {
                        // public_key: u8[256]
                        let mut public_key = [0_u8; 256];
                        r.read_exact(&mut public_key)?;

                        Addon_KeyVersion::Six {
                            public_key,
                        }
                    }
                    KeyVersion::Seven => {
                        // public_key: u8[256]
                        let mut public_key = [0_u8; 256];
                        r.read_exact(&mut public_key)?;

                        Addon_KeyVersion::Seven {
                            public_key,
                        }
                    }
                    KeyVersion::Eight => {
                        // public_key: u8[256]
                        let mut public_key = [0_u8; 256];
                        r.read_exact(&mut public_key)?;

                        Addon_KeyVersion::Eight {
                            public_key,
                        }
                    }
                    KeyVersion::Nine => {
                        // public_key: u8[256]
                        let mut public_key = [0_u8; 256];
                        r.read_exact(&mut public_key)?;

                        Addon_KeyVersion::Nine {
                            public_key,
                        }
                    }
                };

                // update_available_flag: u32
                let update_available_flag = crate::util::read_u32_le(r)?;

                Addon_InfoBlock::Available {
                    key_version: key_version_if,
                    update_available_flag,
                }
            }
        };

        // url_info: UrlInfo
        let url_info: UrlInfo = crate::util::read_u8_le(r)?.try_into()?;

        let url_info_if = match url_info {
            UrlInfo::Unavailable => Addon_UrlInfo::Unavailable,
            UrlInfo::Available => {
                // url: CString
                let url = crate::util::read_c_string_to_vec(r)?;
                let url = String::from_utf8(url)?;

                Addon_UrlInfo::Available {
                    url,
                }
            }
        };

        Ok(Self {
            addon_type,
            info_block: info_block_if,
            url_info: url_info_if,
        })
    }

}

impl Addon {
    pub(crate) fn size(&self) -> usize {
        1 // addon_type: AddonType
        + self.info_block.size() // info_block: Addon_InfoBlock
        + self.url_info.size() // url_info: Addon_UrlInfo
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Addon_KeyVersion {
    Zero,
    One {
        public_key: [u8; 256],
    },
    Two {
        public_key: [u8; 256],
    },
    Three {
        public_key: [u8; 256],
    },
    Four {
        public_key: [u8; 256],
    },
    Five {
        public_key: [u8; 256],
    },
    Six {
        public_key: [u8; 256],
    },
    Seven {
        public_key: [u8; 256],
    },
    Eight {
        public_key: [u8; 256],
    },
    Nine {
        public_key: [u8; 256],
    },
}

impl Default for Addon_KeyVersion {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Zero
    }
}

impl Addon_KeyVersion {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One { .. } => 1,
            Self::Two { .. } => 2,
            Self::Three { .. } => 3,
            Self::Four { .. } => 4,
            Self::Five { .. } => 5,
            Self::Six { .. } => 6,
            Self::Seven { .. } => 7,
            Self::Eight { .. } => 8,
            Self::Nine { .. } => 9,
        }
    }

}

impl Addon_KeyVersion {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Zero => {
                1
            }
            Self::One {
                public_key,
            } => {
                1
                + 256 * core::mem::size_of::<u8>() // public_key: u8[256]
            }
            Self::Two {
                public_key,
            } => {
                1
                + 256 * core::mem::size_of::<u8>() // public_key: u8[256]
            }
            Self::Three {
                public_key,
            } => {
                1
                + 256 * core::mem::size_of::<u8>() // public_key: u8[256]
            }
            Self::Four {
                public_key,
            } => {
                1
                + 256 * core::mem::size_of::<u8>() // public_key: u8[256]
            }
            Self::Five {
                public_key,
            } => {
                1
                + 256 * core::mem::size_of::<u8>() // public_key: u8[256]
            }
            Self::Six {
                public_key,
            } => {
                1
                + 256 * core::mem::size_of::<u8>() // public_key: u8[256]
            }
            Self::Seven {
                public_key,
            } => {
                1
                + 256 * core::mem::size_of::<u8>() // public_key: u8[256]
            }
            Self::Eight {
                public_key,
            } => {
                1
                + 256 * core::mem::size_of::<u8>() // public_key: u8[256]
            }
            Self::Nine {
                public_key,
            } => {
                1
                + 256 * core::mem::size_of::<u8>() // public_key: u8[256]
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Addon_InfoBlock {
    Unavailable,
    Available {
        key_version: Addon_KeyVersion,
        update_available_flag: u32,
    },
}

impl Default for Addon_InfoBlock {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Unavailable
    }
}

impl Addon_InfoBlock {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Unavailable => 0,
            Self::Available { .. } => 1,
        }
    }

}

impl Addon_InfoBlock {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Unavailable => {
                1
            }
            Self::Available {
                key_version,
                update_available_flag,
            } => {
                1
                + key_version.size() // key_version: Addon_KeyVersion
                + 4 // update_available_flag: u32
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Addon_UrlInfo {
    Unavailable,
    Available {
        url: String,
    },
}

impl Default for Addon_UrlInfo {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Unavailable
    }
}

impl Addon_UrlInfo {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Unavailable => 0,
            Self::Available { .. } => 1,
        }
    }

}

impl Addon_UrlInfo {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Unavailable => {
                1
            }
            Self::Available {
                url,
            } => {
                1
                + url.len() + 1 // url: CString
            }
        }
    }
}

