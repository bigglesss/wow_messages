use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::AuraMask;
use crate::world::v1::v12::{Area, AreaError};
use crate::world::v1::v12::{GroupMemberOnlineStatus};
use crate::world::v1::v12::{GroupUpdateFlags};
use crate::world::v1::v12::{Power, PowerError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_party_member_stats.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_party_member_stats.wowm#L3):
/// ```text
/// smsg SMSG_PARTY_MEMBER_STATS = 0x7E {
///     PackedGuid guid;
///     GroupUpdateFlags mask;
///     if (mask & FLAG_STATUS) {
///         GroupMemberOnlineStatus status;
///     }
///     if (mask & FLAG_CUR_HP) {
///         u16 current_health;
///     }
///     if (mask & FLAG_MAX_HP) {
///         u16 max_health;
///     }
///     if (mask & FLAG_POWER_TYPE) {
///         Power power;
///     }
///     if (mask & FLAG_CUR_POWER) {
///         u16 current_power;
///     }
///     if (mask & FLAG_MAX_POWER) {
///         u16 max_power;
///     }
///     if (mask & FLAG_LEVEL) {
///         u16 level;
///     }
///     if (mask & FLAG_ZONE) {
///         Area area;
///     }
///     if (mask & FLAG_POSITION) {
///         u16 position_x;
///         u16 position_y;
///     }
///     if (mask & FLAG_AURAS) {
///         AuraMask auras;
///     }
///     if (mask & FLAG_PET_NAME) {
///         CString pet_name;
///     }
///     if (mask & FLAG_PET_MODEL_ID) {
///         u16 pet_display_id;
///     }
///     if (mask & FLAG_PET_CUR_HP) {
///         u16 pet_current_health;
///     }
///     if (mask & FLAG_PET_MAX_HP) {
///         u16 pet_max_health;
///     }
///     if (mask & FLAG_PET_POWER_TYPE) {
///         Power pet_power_type;
///     }
///     if (mask & FLAG_PET_CUR_POWER) {
///         u16 pet_current_power;
///     }
///     if (mask & FLAG_PET_MAX_POWER) {
///         u16 pet_max_power;
///     }
///     if (mask & FLAG_PET_AURAS) {
///         AuraMask pet_auras;
///     }
/// }
/// ```
pub struct SMSG_PARTY_MEMBER_STATS {
    pub guid: Guid,
    pub mask: SMSG_PARTY_MEMBER_STATSGroupUpdateFlags,
}

impl WorldServerMessageWrite for SMSG_PARTY_MEMBER_STATS {
    const OPCODE: u16 = 0x7e;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_PARTY_MEMBER_STATS {
    type Error = SMSG_PARTY_MEMBER_STATSError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // mask: GroupUpdateFlags
        let mask = GroupUpdateFlags::read(r)?;

        let mask_FLAG_STATUS = if mask.is_FLAG_STATUS() {
            // status: GroupMemberOnlineStatus
            let status = GroupMemberOnlineStatus::read(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_STATUS {
                status,
            })
        } else {
            None
        };

        let mask_FLAG_CUR_HP = if mask.is_FLAG_CUR_HP() {
            // current_health: u16
            let current_health = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_CUR_HP {
                current_health,
            })
        } else {
            None
        };

        let mask_FLAG_MAX_HP = if mask.is_FLAG_MAX_HP() {
            // max_health: u16
            let max_health = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_MAX_HP {
                max_health,
            })
        } else {
            None
        };

        let mask_FLAG_POWER_TYPE = if mask.is_FLAG_POWER_TYPE() {
            // power: Power
            let power = Power::read(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_POWER_TYPE {
                power,
            })
        } else {
            None
        };

        let mask_FLAG_CUR_POWER = if mask.is_FLAG_CUR_POWER() {
            // current_power: u16
            let current_power = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_CUR_POWER {
                current_power,
            })
        } else {
            None
        };

        let mask_FLAG_MAX_POWER = if mask.is_FLAG_MAX_POWER() {
            // max_power: u16
            let max_power = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_MAX_POWER {
                max_power,
            })
        } else {
            None
        };

        let mask_FLAG_LEVEL = if mask.is_FLAG_LEVEL() {
            // level: u16
            let level = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_LEVEL {
                level,
            })
        } else {
            None
        };

        let mask_FLAG_ZONE = if mask.is_FLAG_ZONE() {
            // area: Area
            let area = Area::read(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_ZONE {
                area,
            })
        } else {
            None
        };

        let mask_FLAG_POSITION = if mask.is_FLAG_POSITION() {
            // position_x: u16
            let position_x = crate::util::read_u16_le(r)?;

            // position_y: u16
            let position_y = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_POSITION {
                position_x,
                position_y,
            })
        } else {
            None
        };

        let mask_FLAG_AURAS = if mask.is_FLAG_AURAS() {
            // auras: AuraMask
            let auras = AuraMask::read(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_AURAS {
                auras,
            })
        } else {
            None
        };

        let mask_FLAG_PET_NAME = if mask.is_FLAG_PET_NAME() {
            // pet_name: CString
            let pet_name = crate::util::read_c_string_to_vec(r)?;
            let pet_name = String::from_utf8(pet_name)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_NAME {
                pet_name,
            })
        } else {
            None
        };

        let mask_FLAG_PET_MODEL_ID = if mask.is_FLAG_PET_MODEL_ID() {
            // pet_display_id: u16
            let pet_display_id = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MODEL_ID {
                pet_display_id,
            })
        } else {
            None
        };

        let mask_FLAG_PET_CUR_HP = if mask.is_FLAG_PET_CUR_HP() {
            // pet_current_health: u16
            let pet_current_health = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_CUR_HP {
                pet_current_health,
            })
        } else {
            None
        };

        let mask_FLAG_PET_MAX_HP = if mask.is_FLAG_PET_MAX_HP() {
            // pet_max_health: u16
            let pet_max_health = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MAX_HP {
                pet_max_health,
            })
        } else {
            None
        };

        let mask_FLAG_PET_POWER_TYPE = if mask.is_FLAG_PET_POWER_TYPE() {
            // pet_power_type: Power
            let pet_power_type = Power::read(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_POWER_TYPE {
                pet_power_type,
            })
        } else {
            None
        };

        let mask_FLAG_PET_CUR_POWER = if mask.is_FLAG_PET_CUR_POWER() {
            // pet_current_power: u16
            let pet_current_power = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_CUR_POWER {
                pet_current_power,
            })
        } else {
            None
        };

        let mask_FLAG_PET_MAX_POWER = if mask.is_FLAG_PET_MAX_POWER() {
            // pet_max_power: u16
            let pet_max_power = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MAX_POWER {
                pet_max_power,
            })
        } else {
            None
        };

        let mask_FLAG_PET_AURAS = if mask.is_FLAG_PET_AURAS() {
            // pet_auras: AuraMask
            let pet_auras = AuraMask::read(r)?;

            Some(SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_AURAS {
                pet_auras,
            })
        } else {
            None
        };

        let mask = SMSG_PARTY_MEMBER_STATSGroupUpdateFlags {
            inner: mask.as_u32(),
            flag_status: mask_FLAG_STATUS,
            flag_cur_hp: mask_FLAG_CUR_HP,
            flag_max_hp: mask_FLAG_MAX_HP,
            flag_power_type: mask_FLAG_POWER_TYPE,
            flag_cur_power: mask_FLAG_CUR_POWER,
            flag_max_power: mask_FLAG_MAX_POWER,
            flag_level: mask_FLAG_LEVEL,
            flag_zone: mask_FLAG_ZONE,
            flag_position: mask_FLAG_POSITION,
            flag_auras: mask_FLAG_AURAS,
            flag_pet_name: mask_FLAG_PET_NAME,
            flag_pet_model_id: mask_FLAG_PET_MODEL_ID,
            flag_pet_cur_hp: mask_FLAG_PET_CUR_HP,
            flag_pet_max_hp: mask_FLAG_PET_MAX_HP,
            flag_pet_power_type: mask_FLAG_PET_POWER_TYPE,
            flag_pet_cur_power: mask_FLAG_PET_CUR_POWER,
            flag_pet_max_power: mask_FLAG_PET_MAX_POWER,
            flag_pet_auras: mask_FLAG_PET_AURAS,
        };

        Ok(Self {
            guid,
            mask,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed(w)?;

        // mask: GroupUpdateFlags
        self.mask.write(w)?;

        if let Some(s) = &self.mask.flag_status {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_cur_hp {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_max_hp {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_power_type {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_cur_power {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_max_power {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_level {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_zone {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_position {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_auras {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_pet_name {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_pet_model_id {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_pet_cur_hp {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_pet_max_hp {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_pet_power_type {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_pet_cur_power {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_pet_max_power {
            s.write(w)?;
        }

        if let Some(s) = &self.mask.flag_pet_auras {
            s.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_PARTY_MEMBER_STATS {
    fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + self.mask.size() // mask: GroupUpdateFlags and subfields
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATS {
    fn maximum_possible_size() -> usize {
        9 // guid: PackedGuid
        + GroupUpdateFlags::maximum_possible_size() // mask: GroupUpdateFlags
    }
}

#[derive(Debug)]
pub enum SMSG_PARTY_MEMBER_STATSError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    Area(AreaError),
    Power(PowerError),
}

impl std::error::Error for SMSG_PARTY_MEMBER_STATSError {}
impl std::fmt::Display for SMSG_PARTY_MEMBER_STATSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Area(i) => i.fmt(f),
            Self::Power(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PARTY_MEMBER_STATSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_PARTY_MEMBER_STATSError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<AreaError> for SMSG_PARTY_MEMBER_STATSError {
    fn from(e: AreaError) -> Self {
        Self::Area(e)
    }
}

impl From<PowerError> for SMSG_PARTY_MEMBER_STATSError {
    fn from(e: PowerError) -> Self {
        Self::Power(e)
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlags {
    inner: u32,
    flag_status: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_STATUS>,
    flag_cur_hp: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_CUR_HP>,
    flag_max_hp: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_MAX_HP>,
    flag_power_type: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_POWER_TYPE>,
    flag_cur_power: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_CUR_POWER>,
    flag_max_power: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_MAX_POWER>,
    flag_level: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_LEVEL>,
    flag_zone: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_ZONE>,
    flag_position: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_POSITION>,
    flag_auras: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_AURAS>,
    flag_pet_name: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_NAME>,
    flag_pet_model_id: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MODEL_ID>,
    flag_pet_cur_hp: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_CUR_HP>,
    flag_pet_max_hp: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MAX_HP>,
    flag_pet_power_type: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_POWER_TYPE>,
    flag_pet_cur_power: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_CUR_POWER>,
    flag_pet_max_power: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MAX_POWER>,
    flag_pet_auras: Option<SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_AURAS>,
}

impl From<&SMSG_PARTY_MEMBER_STATSGroupUpdateFlags> for GroupUpdateFlags {
    fn from(e: &SMSG_PARTY_MEMBER_STATSGroupUpdateFlags) -> Self {
        Self::new(e.inner)
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlags {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GroupUpdateFlags = self.into();
        a.write(w)?;
        Ok(())
    }

    pub const fn new_FLAG_NONE() -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_NONE,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub const fn new_FLAG_AURAS_2() -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_AURAS_2,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub const fn new_FLAG_PET_GUID() -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_PET_GUID,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub const fn new_FLAG_PET_AURAS_2() -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_PET_AURAS_2,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub const fn new_MODE_OFFLINE() -> Self {
        Self {
            inner: GroupUpdateFlags::MODE_OFFLINE,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

}
impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlags {
    fn size(&self) -> usize {
        4 // inner: GroupUpdateFlags (u32)
        + {
            if let Some(s) = &self.flag_status {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_cur_hp {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_max_hp {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_power_type {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_cur_power {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_max_power {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_level {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_zone {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_position {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_auras {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_pet_name {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_pet_model_id {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_pet_cur_hp {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_pet_max_hp {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_pet_power_type {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_pet_cur_power {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_pet_max_power {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_pet_auras {
                s.size()
            } else {
                0
            }
        }
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlags {
    fn maximum_possible_size() -> usize {
        4 // inner: GroupUpdateFlags (u32)
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_STATUS::maximum_possible_size() // FLAG_STATUS enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_CUR_HP::maximum_possible_size() // FLAG_CUR_HP enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_MAX_HP::maximum_possible_size() // FLAG_MAX_HP enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_POWER_TYPE::maximum_possible_size() // FLAG_POWER_TYPE enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_CUR_POWER::maximum_possible_size() // FLAG_CUR_POWER enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_MAX_POWER::maximum_possible_size() // FLAG_MAX_POWER enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_LEVEL::maximum_possible_size() // FLAG_LEVEL enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_ZONE::maximum_possible_size() // FLAG_ZONE enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_POSITION::maximum_possible_size() // FLAG_POSITION enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_AURAS::maximum_possible_size() // FLAG_AURAS enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_NAME::maximum_possible_size() // FLAG_PET_NAME enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MODEL_ID::maximum_possible_size() // FLAG_PET_MODEL_ID enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_CUR_HP::maximum_possible_size() // FLAG_PET_CUR_HP enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MAX_HP::maximum_possible_size() // FLAG_PET_MAX_HP enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_POWER_TYPE::maximum_possible_size() // FLAG_PET_POWER_TYPE enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_CUR_POWER::maximum_possible_size() // FLAG_PET_CUR_POWER enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MAX_POWER::maximum_possible_size() // FLAG_PET_MAX_POWER enumerator
        + SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_AURAS::maximum_possible_size() // FLAG_PET_AURAS enumerator
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_STATUS {
    pub status: GroupMemberOnlineStatus,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_STATUS {
    fn size(&self) -> usize {
        GroupMemberOnlineStatus::size() // status: GroupMemberOnlineStatus
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_STATUS {
    fn maximum_possible_size() -> usize {
        GroupMemberOnlineStatus::maximum_possible_size() // status: GroupMemberOnlineStatus
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_STATUS {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        self.status.write(w)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_CUR_HP {
    pub current_health: u16,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_CUR_HP {
    fn size(&self) -> usize {
        2 // current_health: u16
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_CUR_HP {
    fn maximum_possible_size() -> usize {
        2 // current_health: u16
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_CUR_HP {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.current_health.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_MAX_HP {
    pub max_health: u16,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_MAX_HP {
    fn size(&self) -> usize {
        2 // max_health: u16
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_MAX_HP {
    fn maximum_possible_size() -> usize {
        2 // max_health: u16
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_MAX_HP {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.max_health.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_POWER_TYPE {
    pub power: Power,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_POWER_TYPE {
    fn size(&self) -> usize {
        Power::size() // power: Power
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_POWER_TYPE {
    fn maximum_possible_size() -> usize {
        Power::maximum_possible_size() // power: Power
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_POWER_TYPE {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        self.power.write(w)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_CUR_POWER {
    pub current_power: u16,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_CUR_POWER {
    fn size(&self) -> usize {
        2 // current_power: u16
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_CUR_POWER {
    fn maximum_possible_size() -> usize {
        2 // current_power: u16
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_CUR_POWER {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.current_power.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_MAX_POWER {
    pub max_power: u16,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_MAX_POWER {
    fn size(&self) -> usize {
        2 // max_power: u16
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_MAX_POWER {
    fn maximum_possible_size() -> usize {
        2 // max_power: u16
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_MAX_POWER {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.max_power.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_LEVEL {
    pub level: u16,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_LEVEL {
    fn size(&self) -> usize {
        2 // level: u16
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_LEVEL {
    fn maximum_possible_size() -> usize {
        2 // level: u16
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_LEVEL {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.level.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_ZONE {
    pub area: Area,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_ZONE {
    fn size(&self) -> usize {
        Area::size() // area: Area
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_ZONE {
    fn maximum_possible_size() -> usize {
        Area::maximum_possible_size() // area: Area
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_ZONE {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        self.area.write(w)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_POSITION {
    pub position_x: u16,
    pub position_y: u16,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_POSITION {
    fn size(&self) -> usize {
        2 // position_x: u16
        + 2 // position_y: u16
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_POSITION {
    fn maximum_possible_size() -> usize {
        2 // position_x: u16
        + 2 // position_y: u16
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_POSITION {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.position_x.to_le_bytes())?;

        w.write_all(&self.position_y.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_AURAS {
    pub auras: AuraMask,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_AURAS {
    fn size(&self) -> usize {
        self.auras.size() // auras: AuraMask
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_AURAS {
    fn maximum_possible_size() -> usize {
        65536 // auras: AuraMask
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_AURAS {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        self.auras.write(w)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_NAME {
    pub pet_name: String,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_NAME {
    fn size(&self) -> usize {
        self.pet_name.len() + 1 // pet_name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_NAME {
    fn maximum_possible_size() -> usize {
        256 // pet_name: CString
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_NAME {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(self.pet_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MODEL_ID {
    pub pet_display_id: u16,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MODEL_ID {
    fn size(&self) -> usize {
        2 // pet_display_id: u16
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MODEL_ID {
    fn maximum_possible_size() -> usize {
        2 // pet_display_id: u16
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MODEL_ID {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.pet_display_id.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_CUR_HP {
    pub pet_current_health: u16,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_CUR_HP {
    fn size(&self) -> usize {
        2 // pet_current_health: u16
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_CUR_HP {
    fn maximum_possible_size() -> usize {
        2 // pet_current_health: u16
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_CUR_HP {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.pet_current_health.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MAX_HP {
    pub pet_max_health: u16,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MAX_HP {
    fn size(&self) -> usize {
        2 // pet_max_health: u16
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MAX_HP {
    fn maximum_possible_size() -> usize {
        2 // pet_max_health: u16
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MAX_HP {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.pet_max_health.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_POWER_TYPE {
    pub pet_power_type: Power,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_POWER_TYPE {
    fn size(&self) -> usize {
        Power::size() // pet_power_type: Power
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_POWER_TYPE {
    fn maximum_possible_size() -> usize {
        Power::maximum_possible_size() // pet_power_type: Power
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_POWER_TYPE {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        self.pet_power_type.write(w)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_CUR_POWER {
    pub pet_current_power: u16,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_CUR_POWER {
    fn size(&self) -> usize {
        2 // pet_current_power: u16
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_CUR_POWER {
    fn maximum_possible_size() -> usize {
        2 // pet_current_power: u16
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_CUR_POWER {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.pet_current_power.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MAX_POWER {
    pub pet_max_power: u16,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MAX_POWER {
    fn size(&self) -> usize {
        2 // pet_max_power: u16
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MAX_POWER {
    fn maximum_possible_size() -> usize {
        2 // pet_max_power: u16
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_MAX_POWER {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.pet_max_power.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_AURAS {
    pub pet_auras: AuraMask,
}

impl VariableSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_AURAS {
    fn size(&self) -> usize {
        self.pet_auras.size() // pet_auras: AuraMask
    }
}

impl MaximumPossibleSized for SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_AURAS {
    fn maximum_possible_size() -> usize {
        65536 // pet_auras: AuraMask
    }
}

impl SMSG_PARTY_MEMBER_STATSGroupUpdateFlagsFLAG_PET_AURAS {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        self.pet_auras.write(w)?;

        Ok(())
    }
}
