use std::convert::{TryFrom, TryInto};
use crate::world::v1::v2::{WorldResult, WorldResultError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm#L3):
/// ```text
/// smsg SMSG_AUTH_RESPONSE = 0x1EE {
///     WorldResult result;
///     if (result == AUTH_OK) {
///         u32 billing_time;
///         u8 billing_flags;
///         u32 billing_rested;
///         ELSE-IF-STATEMENT-DOCC: unimplemented
///     }
/// }
/// ```
pub struct SMSG_AUTH_RESPONSE {
    pub result: SMSG_AUTH_RESPONSEWorldResult,
}

impl WorldServerMessageWrite for SMSG_AUTH_RESPONSE {
    const OPCODE: u16 = 0x1ee;

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
impl WorldMessageBody for SMSG_AUTH_RESPONSE {
    type Error = SMSG_AUTH_RESPONSEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: WorldResult
        let result = WorldResult::read(r)?;

        let result_if = match result {
            WorldResult::RESPONSE_SUCCESS => SMSG_AUTH_RESPONSEWorldResult::RESPONSE_SUCCESS,
            WorldResult::RESPONSE_FAILURE => SMSG_AUTH_RESPONSEWorldResult::RESPONSE_FAILURE,
            WorldResult::RESPONSE_CANCELLED => SMSG_AUTH_RESPONSEWorldResult::RESPONSE_CANCELLED,
            WorldResult::RESPONSE_DISCONNECTED => SMSG_AUTH_RESPONSEWorldResult::RESPONSE_DISCONNECTED,
            WorldResult::RESPONSE_FAILED_TO_CONNECT => SMSG_AUTH_RESPONSEWorldResult::RESPONSE_FAILED_TO_CONNECT,
            WorldResult::RESPONSE_CONNECTED => SMSG_AUTH_RESPONSEWorldResult::RESPONSE_CONNECTED,
            WorldResult::RESPONSE_VERSION_MISMATCH => SMSG_AUTH_RESPONSEWorldResult::RESPONSE_VERSION_MISMATCH,
            WorldResult::CSTATUS_CONNECTING => SMSG_AUTH_RESPONSEWorldResult::CSTATUS_CONNECTING,
            WorldResult::CSTATUS_NEGOTIATING_SECURITY => SMSG_AUTH_RESPONSEWorldResult::CSTATUS_NEGOTIATING_SECURITY,
            WorldResult::CSTATUS_NEGOTIATION_COMPLETE => SMSG_AUTH_RESPONSEWorldResult::CSTATUS_NEGOTIATION_COMPLETE,
            WorldResult::CSTATUS_NEGOTIATION_FAILED => SMSG_AUTH_RESPONSEWorldResult::CSTATUS_NEGOTIATION_FAILED,
            WorldResult::CSTATUS_AUTHENTICATING => SMSG_AUTH_RESPONSEWorldResult::CSTATUS_AUTHENTICATING,
            WorldResult::AUTH_OK => {
                // billing_time: u32
                let billing_time = crate::util::read_u32_le(r)?;

                // billing_flags: u8
                let billing_flags = crate::util::read_u8_le(r)?;

                // billing_rested: u32
                let billing_rested = crate::util::read_u32_le(r)?;

                SMSG_AUTH_RESPONSEWorldResult::AUTH_OK {
                    billing_time,
                    billing_flags,
                    billing_rested,
                }
            }
            WorldResult::AUTH_FAILED => SMSG_AUTH_RESPONSEWorldResult::AUTH_FAILED,
            WorldResult::AUTH_REJECT => SMSG_AUTH_RESPONSEWorldResult::AUTH_REJECT,
            WorldResult::AUTH_BAD_SERVER_PROOF => SMSG_AUTH_RESPONSEWorldResult::AUTH_BAD_SERVER_PROOF,
            WorldResult::AUTH_UNAVAILABLE => SMSG_AUTH_RESPONSEWorldResult::AUTH_UNAVAILABLE,
            WorldResult::AUTH_SYSTEM_ERROR => SMSG_AUTH_RESPONSEWorldResult::AUTH_SYSTEM_ERROR,
            WorldResult::AUTH_BILLING_ERROR => SMSG_AUTH_RESPONSEWorldResult::AUTH_BILLING_ERROR,
            WorldResult::AUTH_BILLING_EXPIRED => SMSG_AUTH_RESPONSEWorldResult::AUTH_BILLING_EXPIRED,
            WorldResult::AUTH_VERSION_MISMATCH => SMSG_AUTH_RESPONSEWorldResult::AUTH_VERSION_MISMATCH,
            WorldResult::AUTH_UNKNOWN_ACCOUNT => SMSG_AUTH_RESPONSEWorldResult::AUTH_UNKNOWN_ACCOUNT,
            WorldResult::AUTH_INCORRECT_PASSWORD => SMSG_AUTH_RESPONSEWorldResult::AUTH_INCORRECT_PASSWORD,
            WorldResult::AUTH_SESSION_EXPIRED => SMSG_AUTH_RESPONSEWorldResult::AUTH_SESSION_EXPIRED,
            WorldResult::AUTH_SERVER_SHUTTING_DOWN => SMSG_AUTH_RESPONSEWorldResult::AUTH_SERVER_SHUTTING_DOWN,
            WorldResult::AUTH_ALREADY_LOGGING_IN => SMSG_AUTH_RESPONSEWorldResult::AUTH_ALREADY_LOGGING_IN,
            WorldResult::AUTH_LOGIN_SERVER_NOT_FOUND => SMSG_AUTH_RESPONSEWorldResult::AUTH_LOGIN_SERVER_NOT_FOUND,
            WorldResult::AUTH_WAIT_QUEUE => {
                // queue_position: u32
                let queue_position = crate::util::read_u32_le(r)?;

                SMSG_AUTH_RESPONSEWorldResult::AUTH_WAIT_QUEUE {
                    queue_position,
                }
            }
            WorldResult::AUTH_BANNED => SMSG_AUTH_RESPONSEWorldResult::AUTH_BANNED,
            WorldResult::AUTH_ALREADY_ONLINE => SMSG_AUTH_RESPONSEWorldResult::AUTH_ALREADY_ONLINE,
            WorldResult::AUTH_NO_TIME => SMSG_AUTH_RESPONSEWorldResult::AUTH_NO_TIME,
            WorldResult::AUTH_DB_BUSY => SMSG_AUTH_RESPONSEWorldResult::AUTH_DB_BUSY,
            WorldResult::AUTH_SUSPENDED => SMSG_AUTH_RESPONSEWorldResult::AUTH_SUSPENDED,
            WorldResult::AUTH_PARENTAL_CONTROL => SMSG_AUTH_RESPONSEWorldResult::AUTH_PARENTAL_CONTROL,
            WorldResult::REALM_LIST_IN_PROGRESS => SMSG_AUTH_RESPONSEWorldResult::REALM_LIST_IN_PROGRESS,
            WorldResult::REALM_LIST_SUCCESS => SMSG_AUTH_RESPONSEWorldResult::REALM_LIST_SUCCESS,
            WorldResult::REALM_LIST_FAILED => SMSG_AUTH_RESPONSEWorldResult::REALM_LIST_FAILED,
            WorldResult::REALM_LIST_INVALID => SMSG_AUTH_RESPONSEWorldResult::REALM_LIST_INVALID,
            WorldResult::REALM_LIST_REALM_NOT_FOUND => SMSG_AUTH_RESPONSEWorldResult::REALM_LIST_REALM_NOT_FOUND,
            WorldResult::ACCOUNT_CREATE_IN_PROGRESS => SMSG_AUTH_RESPONSEWorldResult::ACCOUNT_CREATE_IN_PROGRESS,
            WorldResult::ACCOUNT_CREATE_SUCCESS => SMSG_AUTH_RESPONSEWorldResult::ACCOUNT_CREATE_SUCCESS,
            WorldResult::ACCOUNT_CREATE_FAILED => SMSG_AUTH_RESPONSEWorldResult::ACCOUNT_CREATE_FAILED,
            WorldResult::CHAR_LIST_RETRIEVING => SMSG_AUTH_RESPONSEWorldResult::CHAR_LIST_RETRIEVING,
            WorldResult::CHAR_LIST_RETRIEVED => SMSG_AUTH_RESPONSEWorldResult::CHAR_LIST_RETRIEVED,
            WorldResult::CHAR_LIST_FAILED => SMSG_AUTH_RESPONSEWorldResult::CHAR_LIST_FAILED,
            WorldResult::CHAR_CREATE_IN_PROGRESS => SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_IN_PROGRESS,
            WorldResult::CHAR_CREATE_SUCCESS => SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_SUCCESS,
            WorldResult::CHAR_CREATE_ERROR => SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_ERROR,
            WorldResult::CHAR_CREATE_FAILED => SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_FAILED,
            WorldResult::CHAR_CREATE_NAME_IN_USE => SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_NAME_IN_USE,
            WorldResult::CHAR_CREATE_DISABLED => SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_DISABLED,
            WorldResult::CHAR_CREATE_PVP_TEAMS_VIOLATION => SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_PVP_TEAMS_VIOLATION,
            WorldResult::CHAR_CREATE_SERVER_LIMIT => SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_SERVER_LIMIT,
            WorldResult::CHAR_CREATE_ACCOUNT_LIMIT => SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_ACCOUNT_LIMIT,
            WorldResult::CHAR_CREATE_SERVER_QUEUE => SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_SERVER_QUEUE,
            WorldResult::CHAR_CREATE_ONLY_EXISTING => SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_ONLY_EXISTING,
            WorldResult::CHAR_DELETE_IN_PROGRESS => SMSG_AUTH_RESPONSEWorldResult::CHAR_DELETE_IN_PROGRESS,
            WorldResult::CHAR_DELETE_SUCCESS => SMSG_AUTH_RESPONSEWorldResult::CHAR_DELETE_SUCCESS,
            WorldResult::CHAR_DELETE_FAILED => SMSG_AUTH_RESPONSEWorldResult::CHAR_DELETE_FAILED,
            WorldResult::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER => SMSG_AUTH_RESPONSEWorldResult::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER,
            WorldResult::CHAR_LOGIN_IN_PROGRESS => SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_IN_PROGRESS,
            WorldResult::CHAR_LOGIN_SUCCESS => SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_SUCCESS,
            WorldResult::CHAR_LOGIN_NO_WORLD => SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_NO_WORLD,
            WorldResult::CHAR_LOGIN_DUPLICATE_CHARACTER => SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_DUPLICATE_CHARACTER,
            WorldResult::CHAR_LOGIN_NO_INSTANCES => SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_NO_INSTANCES,
            WorldResult::CHAR_LOGIN_FAILED => SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_FAILED,
            WorldResult::CHAR_LOGIN_DISABLED => SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_DISABLED,
            WorldResult::CHAR_LOGIN_NO_CHARACTER => SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_NO_CHARACTER,
            WorldResult::CHAR_LOGIN_LOCKED_FOR_TRANSFER => SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_LOCKED_FOR_TRANSFER,
            WorldResult::CHAR_NAME_NO_NAME => SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_NO_NAME,
            WorldResult::CHAR_NAME_TOO_SHORT => SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_TOO_SHORT,
            WorldResult::CHAR_NAME_TOO_LONG => SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_TOO_LONG,
            WorldResult::CHAR_NAME_ONLY_LETTERS => SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_ONLY_LETTERS,
            WorldResult::CHAR_NAME_MIXED_LANGUAGES => SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_MIXED_LANGUAGES,
            WorldResult::CHAR_NAME_PROFANE => SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_PROFANE,
            WorldResult::CHAR_NAME_RESERVED => SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_RESERVED,
            WorldResult::CHAR_NAME_INVALID_APOSTROPHE => SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_INVALID_APOSTROPHE,
            WorldResult::CHAR_NAME_MULTIPLE_APOSTROPHES => SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_MULTIPLE_APOSTROPHES,
            WorldResult::CHAR_NAME_THREE_CONSECUTIVE => SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_THREE_CONSECUTIVE,
            WorldResult::CHAR_NAME_INVALID_SPACE => SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_INVALID_SPACE,
            WorldResult::CHAR_NAME_SUCCESS => SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_SUCCESS,
            WorldResult::CHAR_NAME_FAILURE => SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_FAILURE,
        };

        Ok(Self {
            result: result_if,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: WorldResult
        self.result.write(w)?;

        match &self.result {
            SMSG_AUTH_RESPONSEWorldResult::RESPONSE_SUCCESS => {}
            SMSG_AUTH_RESPONSEWorldResult::RESPONSE_FAILURE => {}
            SMSG_AUTH_RESPONSEWorldResult::RESPONSE_CANCELLED => {}
            SMSG_AUTH_RESPONSEWorldResult::RESPONSE_DISCONNECTED => {}
            SMSG_AUTH_RESPONSEWorldResult::RESPONSE_FAILED_TO_CONNECT => {}
            SMSG_AUTH_RESPONSEWorldResult::RESPONSE_CONNECTED => {}
            SMSG_AUTH_RESPONSEWorldResult::RESPONSE_VERSION_MISMATCH => {}
            SMSG_AUTH_RESPONSEWorldResult::CSTATUS_CONNECTING => {}
            SMSG_AUTH_RESPONSEWorldResult::CSTATUS_NEGOTIATING_SECURITY => {}
            SMSG_AUTH_RESPONSEWorldResult::CSTATUS_NEGOTIATION_COMPLETE => {}
            SMSG_AUTH_RESPONSEWorldResult::CSTATUS_NEGOTIATION_FAILED => {}
            SMSG_AUTH_RESPONSEWorldResult::CSTATUS_AUTHENTICATING => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_OK {
                billing_time,
                billing_flags,
                billing_rested,
            } => {
                // billing_time: u32
                w.write_all(&billing_time.to_le_bytes())?;

                // billing_flags: u8
                w.write_all(&billing_flags.to_le_bytes())?;

                // billing_rested: u32
                w.write_all(&billing_rested.to_le_bytes())?;

            }
            SMSG_AUTH_RESPONSEWorldResult::AUTH_FAILED => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_REJECT => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_BAD_SERVER_PROOF => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_UNAVAILABLE => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_SYSTEM_ERROR => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_BILLING_ERROR => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_BILLING_EXPIRED => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_VERSION_MISMATCH => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_UNKNOWN_ACCOUNT => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_INCORRECT_PASSWORD => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_SESSION_EXPIRED => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_SERVER_SHUTTING_DOWN => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_ALREADY_LOGGING_IN => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_LOGIN_SERVER_NOT_FOUND => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_WAIT_QUEUE {
                queue_position,
            } => {
                // queue_position: u32
                w.write_all(&queue_position.to_le_bytes())?;

            }
            SMSG_AUTH_RESPONSEWorldResult::AUTH_BANNED => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_ALREADY_ONLINE => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_NO_TIME => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_DB_BUSY => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_SUSPENDED => {}
            SMSG_AUTH_RESPONSEWorldResult::AUTH_PARENTAL_CONTROL => {}
            SMSG_AUTH_RESPONSEWorldResult::REALM_LIST_IN_PROGRESS => {}
            SMSG_AUTH_RESPONSEWorldResult::REALM_LIST_SUCCESS => {}
            SMSG_AUTH_RESPONSEWorldResult::REALM_LIST_FAILED => {}
            SMSG_AUTH_RESPONSEWorldResult::REALM_LIST_INVALID => {}
            SMSG_AUTH_RESPONSEWorldResult::REALM_LIST_REALM_NOT_FOUND => {}
            SMSG_AUTH_RESPONSEWorldResult::ACCOUNT_CREATE_IN_PROGRESS => {}
            SMSG_AUTH_RESPONSEWorldResult::ACCOUNT_CREATE_SUCCESS => {}
            SMSG_AUTH_RESPONSEWorldResult::ACCOUNT_CREATE_FAILED => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LIST_RETRIEVING => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LIST_RETRIEVED => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LIST_FAILED => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_IN_PROGRESS => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_SUCCESS => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_ERROR => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_FAILED => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_NAME_IN_USE => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_DISABLED => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_PVP_TEAMS_VIOLATION => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_SERVER_LIMIT => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_ACCOUNT_LIMIT => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_SERVER_QUEUE => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_ONLY_EXISTING => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_DELETE_IN_PROGRESS => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_DELETE_SUCCESS => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_DELETE_FAILED => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_IN_PROGRESS => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_SUCCESS => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_NO_WORLD => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_DUPLICATE_CHARACTER => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_NO_INSTANCES => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_FAILED => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_DISABLED => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_NO_CHARACTER => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_LOCKED_FOR_TRANSFER => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_NO_NAME => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_TOO_SHORT => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_TOO_LONG => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_ONLY_LETTERS => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_MIXED_LANGUAGES => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_PROFANE => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_RESERVED => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_INVALID_APOSTROPHE => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_MULTIPLE_APOSTROPHES => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_THREE_CONSECUTIVE => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_INVALID_SPACE => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_SUCCESS => {}
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_FAILURE => {}
        }

        Ok(())
    }
}

impl VariableSized for SMSG_AUTH_RESPONSE {
    fn size(&self) -> usize {
        self.result.size() // result: WorldResult and subfields
    }
}

impl MaximumPossibleSized for SMSG_AUTH_RESPONSE {
    fn maximum_possible_size() -> usize {
        WorldResult::maximum_possible_size() // result: WorldResult
    }
}

#[derive(Debug)]
pub enum SMSG_AUTH_RESPONSEError {
    Io(std::io::Error),
    WorldResult(WorldResultError),
}

impl std::error::Error for SMSG_AUTH_RESPONSEError {}
impl std::fmt::Display for SMSG_AUTH_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::WorldResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_AUTH_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<WorldResultError> for SMSG_AUTH_RESPONSEError {
    fn from(e: WorldResultError) -> Self {
        Self::WorldResult(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_AUTH_RESPONSEWorldResult {
    RESPONSE_SUCCESS,
    RESPONSE_FAILURE,
    RESPONSE_CANCELLED,
    RESPONSE_DISCONNECTED,
    RESPONSE_FAILED_TO_CONNECT,
    RESPONSE_CONNECTED,
    RESPONSE_VERSION_MISMATCH,
    CSTATUS_CONNECTING,
    CSTATUS_NEGOTIATING_SECURITY,
    CSTATUS_NEGOTIATION_COMPLETE,
    CSTATUS_NEGOTIATION_FAILED,
    CSTATUS_AUTHENTICATING,
    AUTH_OK {
        billing_time: u32,
        billing_flags: u8,
        billing_rested: u32,
    },
    AUTH_FAILED,
    AUTH_REJECT,
    AUTH_BAD_SERVER_PROOF,
    AUTH_UNAVAILABLE,
    AUTH_SYSTEM_ERROR,
    AUTH_BILLING_ERROR,
    AUTH_BILLING_EXPIRED,
    AUTH_VERSION_MISMATCH,
    AUTH_UNKNOWN_ACCOUNT,
    AUTH_INCORRECT_PASSWORD,
    AUTH_SESSION_EXPIRED,
    AUTH_SERVER_SHUTTING_DOWN,
    AUTH_ALREADY_LOGGING_IN,
    AUTH_LOGIN_SERVER_NOT_FOUND,
    AUTH_WAIT_QUEUE {
        queue_position: u32,
    },
    AUTH_BANNED,
    AUTH_ALREADY_ONLINE,
    AUTH_NO_TIME,
    AUTH_DB_BUSY,
    AUTH_SUSPENDED,
    AUTH_PARENTAL_CONTROL,
    REALM_LIST_IN_PROGRESS,
    REALM_LIST_SUCCESS,
    REALM_LIST_FAILED,
    REALM_LIST_INVALID,
    REALM_LIST_REALM_NOT_FOUND,
    ACCOUNT_CREATE_IN_PROGRESS,
    ACCOUNT_CREATE_SUCCESS,
    ACCOUNT_CREATE_FAILED,
    CHAR_LIST_RETRIEVING,
    CHAR_LIST_RETRIEVED,
    CHAR_LIST_FAILED,
    CHAR_CREATE_IN_PROGRESS,
    CHAR_CREATE_SUCCESS,
    CHAR_CREATE_ERROR,
    CHAR_CREATE_FAILED,
    CHAR_CREATE_NAME_IN_USE,
    CHAR_CREATE_DISABLED,
    CHAR_CREATE_PVP_TEAMS_VIOLATION,
    CHAR_CREATE_SERVER_LIMIT,
    CHAR_CREATE_ACCOUNT_LIMIT,
    CHAR_CREATE_SERVER_QUEUE,
    CHAR_CREATE_ONLY_EXISTING,
    CHAR_DELETE_IN_PROGRESS,
    CHAR_DELETE_SUCCESS,
    CHAR_DELETE_FAILED,
    CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER,
    CHAR_LOGIN_IN_PROGRESS,
    CHAR_LOGIN_SUCCESS,
    CHAR_LOGIN_NO_WORLD,
    CHAR_LOGIN_DUPLICATE_CHARACTER,
    CHAR_LOGIN_NO_INSTANCES,
    CHAR_LOGIN_FAILED,
    CHAR_LOGIN_DISABLED,
    CHAR_LOGIN_NO_CHARACTER,
    CHAR_LOGIN_LOCKED_FOR_TRANSFER,
    CHAR_NAME_NO_NAME,
    CHAR_NAME_TOO_SHORT,
    CHAR_NAME_TOO_LONG,
    CHAR_NAME_ONLY_LETTERS,
    CHAR_NAME_MIXED_LANGUAGES,
    CHAR_NAME_PROFANE,
    CHAR_NAME_RESERVED,
    CHAR_NAME_INVALID_APOSTROPHE,
    CHAR_NAME_MULTIPLE_APOSTROPHES,
    CHAR_NAME_THREE_CONSECUTIVE,
    CHAR_NAME_INVALID_SPACE,
    CHAR_NAME_SUCCESS,
    CHAR_NAME_FAILURE,
}

impl From<&WorldResult> for SMSG_AUTH_RESPONSEWorldResult {
    fn from(e: &WorldResult) -> Self {
        match &e {
            WorldResult::RESPONSE_SUCCESS => Self::RESPONSE_SUCCESS,
            WorldResult::RESPONSE_FAILURE => Self::RESPONSE_FAILURE,
            WorldResult::RESPONSE_CANCELLED => Self::RESPONSE_CANCELLED,
            WorldResult::RESPONSE_DISCONNECTED => Self::RESPONSE_DISCONNECTED,
            WorldResult::RESPONSE_FAILED_TO_CONNECT => Self::RESPONSE_FAILED_TO_CONNECT,
            WorldResult::RESPONSE_CONNECTED => Self::RESPONSE_CONNECTED,
            WorldResult::RESPONSE_VERSION_MISMATCH => Self::RESPONSE_VERSION_MISMATCH,
            WorldResult::CSTATUS_CONNECTING => Self::CSTATUS_CONNECTING,
            WorldResult::CSTATUS_NEGOTIATING_SECURITY => Self::CSTATUS_NEGOTIATING_SECURITY,
            WorldResult::CSTATUS_NEGOTIATION_COMPLETE => Self::CSTATUS_NEGOTIATION_COMPLETE,
            WorldResult::CSTATUS_NEGOTIATION_FAILED => Self::CSTATUS_NEGOTIATION_FAILED,
            WorldResult::CSTATUS_AUTHENTICATING => Self::CSTATUS_AUTHENTICATING,
            WorldResult::AUTH_OK => Self::AUTH_OK {
                billing_time: Default::default(),
                billing_flags: Default::default(),
                billing_rested: Default::default(),
            },
            WorldResult::AUTH_FAILED => Self::AUTH_FAILED,
            WorldResult::AUTH_REJECT => Self::AUTH_REJECT,
            WorldResult::AUTH_BAD_SERVER_PROOF => Self::AUTH_BAD_SERVER_PROOF,
            WorldResult::AUTH_UNAVAILABLE => Self::AUTH_UNAVAILABLE,
            WorldResult::AUTH_SYSTEM_ERROR => Self::AUTH_SYSTEM_ERROR,
            WorldResult::AUTH_BILLING_ERROR => Self::AUTH_BILLING_ERROR,
            WorldResult::AUTH_BILLING_EXPIRED => Self::AUTH_BILLING_EXPIRED,
            WorldResult::AUTH_VERSION_MISMATCH => Self::AUTH_VERSION_MISMATCH,
            WorldResult::AUTH_UNKNOWN_ACCOUNT => Self::AUTH_UNKNOWN_ACCOUNT,
            WorldResult::AUTH_INCORRECT_PASSWORD => Self::AUTH_INCORRECT_PASSWORD,
            WorldResult::AUTH_SESSION_EXPIRED => Self::AUTH_SESSION_EXPIRED,
            WorldResult::AUTH_SERVER_SHUTTING_DOWN => Self::AUTH_SERVER_SHUTTING_DOWN,
            WorldResult::AUTH_ALREADY_LOGGING_IN => Self::AUTH_ALREADY_LOGGING_IN,
            WorldResult::AUTH_LOGIN_SERVER_NOT_FOUND => Self::AUTH_LOGIN_SERVER_NOT_FOUND,
            WorldResult::AUTH_WAIT_QUEUE => Self::AUTH_WAIT_QUEUE {
                queue_position: Default::default(),
            },
            WorldResult::AUTH_BANNED => Self::AUTH_BANNED,
            WorldResult::AUTH_ALREADY_ONLINE => Self::AUTH_ALREADY_ONLINE,
            WorldResult::AUTH_NO_TIME => Self::AUTH_NO_TIME,
            WorldResult::AUTH_DB_BUSY => Self::AUTH_DB_BUSY,
            WorldResult::AUTH_SUSPENDED => Self::AUTH_SUSPENDED,
            WorldResult::AUTH_PARENTAL_CONTROL => Self::AUTH_PARENTAL_CONTROL,
            WorldResult::REALM_LIST_IN_PROGRESS => Self::REALM_LIST_IN_PROGRESS,
            WorldResult::REALM_LIST_SUCCESS => Self::REALM_LIST_SUCCESS,
            WorldResult::REALM_LIST_FAILED => Self::REALM_LIST_FAILED,
            WorldResult::REALM_LIST_INVALID => Self::REALM_LIST_INVALID,
            WorldResult::REALM_LIST_REALM_NOT_FOUND => Self::REALM_LIST_REALM_NOT_FOUND,
            WorldResult::ACCOUNT_CREATE_IN_PROGRESS => Self::ACCOUNT_CREATE_IN_PROGRESS,
            WorldResult::ACCOUNT_CREATE_SUCCESS => Self::ACCOUNT_CREATE_SUCCESS,
            WorldResult::ACCOUNT_CREATE_FAILED => Self::ACCOUNT_CREATE_FAILED,
            WorldResult::CHAR_LIST_RETRIEVING => Self::CHAR_LIST_RETRIEVING,
            WorldResult::CHAR_LIST_RETRIEVED => Self::CHAR_LIST_RETRIEVED,
            WorldResult::CHAR_LIST_FAILED => Self::CHAR_LIST_FAILED,
            WorldResult::CHAR_CREATE_IN_PROGRESS => Self::CHAR_CREATE_IN_PROGRESS,
            WorldResult::CHAR_CREATE_SUCCESS => Self::CHAR_CREATE_SUCCESS,
            WorldResult::CHAR_CREATE_ERROR => Self::CHAR_CREATE_ERROR,
            WorldResult::CHAR_CREATE_FAILED => Self::CHAR_CREATE_FAILED,
            WorldResult::CHAR_CREATE_NAME_IN_USE => Self::CHAR_CREATE_NAME_IN_USE,
            WorldResult::CHAR_CREATE_DISABLED => Self::CHAR_CREATE_DISABLED,
            WorldResult::CHAR_CREATE_PVP_TEAMS_VIOLATION => Self::CHAR_CREATE_PVP_TEAMS_VIOLATION,
            WorldResult::CHAR_CREATE_SERVER_LIMIT => Self::CHAR_CREATE_SERVER_LIMIT,
            WorldResult::CHAR_CREATE_ACCOUNT_LIMIT => Self::CHAR_CREATE_ACCOUNT_LIMIT,
            WorldResult::CHAR_CREATE_SERVER_QUEUE => Self::CHAR_CREATE_SERVER_QUEUE,
            WorldResult::CHAR_CREATE_ONLY_EXISTING => Self::CHAR_CREATE_ONLY_EXISTING,
            WorldResult::CHAR_DELETE_IN_PROGRESS => Self::CHAR_DELETE_IN_PROGRESS,
            WorldResult::CHAR_DELETE_SUCCESS => Self::CHAR_DELETE_SUCCESS,
            WorldResult::CHAR_DELETE_FAILED => Self::CHAR_DELETE_FAILED,
            WorldResult::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER => Self::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER,
            WorldResult::CHAR_LOGIN_IN_PROGRESS => Self::CHAR_LOGIN_IN_PROGRESS,
            WorldResult::CHAR_LOGIN_SUCCESS => Self::CHAR_LOGIN_SUCCESS,
            WorldResult::CHAR_LOGIN_NO_WORLD => Self::CHAR_LOGIN_NO_WORLD,
            WorldResult::CHAR_LOGIN_DUPLICATE_CHARACTER => Self::CHAR_LOGIN_DUPLICATE_CHARACTER,
            WorldResult::CHAR_LOGIN_NO_INSTANCES => Self::CHAR_LOGIN_NO_INSTANCES,
            WorldResult::CHAR_LOGIN_FAILED => Self::CHAR_LOGIN_FAILED,
            WorldResult::CHAR_LOGIN_DISABLED => Self::CHAR_LOGIN_DISABLED,
            WorldResult::CHAR_LOGIN_NO_CHARACTER => Self::CHAR_LOGIN_NO_CHARACTER,
            WorldResult::CHAR_LOGIN_LOCKED_FOR_TRANSFER => Self::CHAR_LOGIN_LOCKED_FOR_TRANSFER,
            WorldResult::CHAR_NAME_NO_NAME => Self::CHAR_NAME_NO_NAME,
            WorldResult::CHAR_NAME_TOO_SHORT => Self::CHAR_NAME_TOO_SHORT,
            WorldResult::CHAR_NAME_TOO_LONG => Self::CHAR_NAME_TOO_LONG,
            WorldResult::CHAR_NAME_ONLY_LETTERS => Self::CHAR_NAME_ONLY_LETTERS,
            WorldResult::CHAR_NAME_MIXED_LANGUAGES => Self::CHAR_NAME_MIXED_LANGUAGES,
            WorldResult::CHAR_NAME_PROFANE => Self::CHAR_NAME_PROFANE,
            WorldResult::CHAR_NAME_RESERVED => Self::CHAR_NAME_RESERVED,
            WorldResult::CHAR_NAME_INVALID_APOSTROPHE => Self::CHAR_NAME_INVALID_APOSTROPHE,
            WorldResult::CHAR_NAME_MULTIPLE_APOSTROPHES => Self::CHAR_NAME_MULTIPLE_APOSTROPHES,
            WorldResult::CHAR_NAME_THREE_CONSECUTIVE => Self::CHAR_NAME_THREE_CONSECUTIVE,
            WorldResult::CHAR_NAME_INVALID_SPACE => Self::CHAR_NAME_INVALID_SPACE,
            WorldResult::CHAR_NAME_SUCCESS => Self::CHAR_NAME_SUCCESS,
            WorldResult::CHAR_NAME_FAILURE => Self::CHAR_NAME_FAILURE,
        }
    }
}

impl From<&SMSG_AUTH_RESPONSEWorldResult> for WorldResult {
    fn from(v: &SMSG_AUTH_RESPONSEWorldResult) -> Self {
        match &v {
            SMSG_AUTH_RESPONSEWorldResult::RESPONSE_SUCCESS => Self::RESPONSE_SUCCESS,
            SMSG_AUTH_RESPONSEWorldResult::RESPONSE_FAILURE => Self::RESPONSE_FAILURE,
            SMSG_AUTH_RESPONSEWorldResult::RESPONSE_CANCELLED => Self::RESPONSE_CANCELLED,
            SMSG_AUTH_RESPONSEWorldResult::RESPONSE_DISCONNECTED => Self::RESPONSE_DISCONNECTED,
            SMSG_AUTH_RESPONSEWorldResult::RESPONSE_FAILED_TO_CONNECT => Self::RESPONSE_FAILED_TO_CONNECT,
            SMSG_AUTH_RESPONSEWorldResult::RESPONSE_CONNECTED => Self::RESPONSE_CONNECTED,
            SMSG_AUTH_RESPONSEWorldResult::RESPONSE_VERSION_MISMATCH => Self::RESPONSE_VERSION_MISMATCH,
            SMSG_AUTH_RESPONSEWorldResult::CSTATUS_CONNECTING => Self::CSTATUS_CONNECTING,
            SMSG_AUTH_RESPONSEWorldResult::CSTATUS_NEGOTIATING_SECURITY => Self::CSTATUS_NEGOTIATING_SECURITY,
            SMSG_AUTH_RESPONSEWorldResult::CSTATUS_NEGOTIATION_COMPLETE => Self::CSTATUS_NEGOTIATION_COMPLETE,
            SMSG_AUTH_RESPONSEWorldResult::CSTATUS_NEGOTIATION_FAILED => Self::CSTATUS_NEGOTIATION_FAILED,
            SMSG_AUTH_RESPONSEWorldResult::CSTATUS_AUTHENTICATING => Self::CSTATUS_AUTHENTICATING,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_OK { .. } => Self::AUTH_OK,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_FAILED => Self::AUTH_FAILED,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_REJECT => Self::AUTH_REJECT,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_BAD_SERVER_PROOF => Self::AUTH_BAD_SERVER_PROOF,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_UNAVAILABLE => Self::AUTH_UNAVAILABLE,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_SYSTEM_ERROR => Self::AUTH_SYSTEM_ERROR,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_BILLING_ERROR => Self::AUTH_BILLING_ERROR,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_BILLING_EXPIRED => Self::AUTH_BILLING_EXPIRED,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_VERSION_MISMATCH => Self::AUTH_VERSION_MISMATCH,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_UNKNOWN_ACCOUNT => Self::AUTH_UNKNOWN_ACCOUNT,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_INCORRECT_PASSWORD => Self::AUTH_INCORRECT_PASSWORD,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_SESSION_EXPIRED => Self::AUTH_SESSION_EXPIRED,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_SERVER_SHUTTING_DOWN => Self::AUTH_SERVER_SHUTTING_DOWN,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_ALREADY_LOGGING_IN => Self::AUTH_ALREADY_LOGGING_IN,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_LOGIN_SERVER_NOT_FOUND => Self::AUTH_LOGIN_SERVER_NOT_FOUND,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_WAIT_QUEUE { .. } => Self::AUTH_WAIT_QUEUE,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_BANNED => Self::AUTH_BANNED,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_ALREADY_ONLINE => Self::AUTH_ALREADY_ONLINE,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_NO_TIME => Self::AUTH_NO_TIME,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_DB_BUSY => Self::AUTH_DB_BUSY,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_SUSPENDED => Self::AUTH_SUSPENDED,
            SMSG_AUTH_RESPONSEWorldResult::AUTH_PARENTAL_CONTROL => Self::AUTH_PARENTAL_CONTROL,
            SMSG_AUTH_RESPONSEWorldResult::REALM_LIST_IN_PROGRESS => Self::REALM_LIST_IN_PROGRESS,
            SMSG_AUTH_RESPONSEWorldResult::REALM_LIST_SUCCESS => Self::REALM_LIST_SUCCESS,
            SMSG_AUTH_RESPONSEWorldResult::REALM_LIST_FAILED => Self::REALM_LIST_FAILED,
            SMSG_AUTH_RESPONSEWorldResult::REALM_LIST_INVALID => Self::REALM_LIST_INVALID,
            SMSG_AUTH_RESPONSEWorldResult::REALM_LIST_REALM_NOT_FOUND => Self::REALM_LIST_REALM_NOT_FOUND,
            SMSG_AUTH_RESPONSEWorldResult::ACCOUNT_CREATE_IN_PROGRESS => Self::ACCOUNT_CREATE_IN_PROGRESS,
            SMSG_AUTH_RESPONSEWorldResult::ACCOUNT_CREATE_SUCCESS => Self::ACCOUNT_CREATE_SUCCESS,
            SMSG_AUTH_RESPONSEWorldResult::ACCOUNT_CREATE_FAILED => Self::ACCOUNT_CREATE_FAILED,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LIST_RETRIEVING => Self::CHAR_LIST_RETRIEVING,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LIST_RETRIEVED => Self::CHAR_LIST_RETRIEVED,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LIST_FAILED => Self::CHAR_LIST_FAILED,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_IN_PROGRESS => Self::CHAR_CREATE_IN_PROGRESS,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_SUCCESS => Self::CHAR_CREATE_SUCCESS,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_ERROR => Self::CHAR_CREATE_ERROR,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_FAILED => Self::CHAR_CREATE_FAILED,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_NAME_IN_USE => Self::CHAR_CREATE_NAME_IN_USE,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_DISABLED => Self::CHAR_CREATE_DISABLED,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_PVP_TEAMS_VIOLATION => Self::CHAR_CREATE_PVP_TEAMS_VIOLATION,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_SERVER_LIMIT => Self::CHAR_CREATE_SERVER_LIMIT,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_ACCOUNT_LIMIT => Self::CHAR_CREATE_ACCOUNT_LIMIT,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_SERVER_QUEUE => Self::CHAR_CREATE_SERVER_QUEUE,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_CREATE_ONLY_EXISTING => Self::CHAR_CREATE_ONLY_EXISTING,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_DELETE_IN_PROGRESS => Self::CHAR_DELETE_IN_PROGRESS,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_DELETE_SUCCESS => Self::CHAR_DELETE_SUCCESS,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_DELETE_FAILED => Self::CHAR_DELETE_FAILED,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER => Self::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_IN_PROGRESS => Self::CHAR_LOGIN_IN_PROGRESS,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_SUCCESS => Self::CHAR_LOGIN_SUCCESS,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_NO_WORLD => Self::CHAR_LOGIN_NO_WORLD,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_DUPLICATE_CHARACTER => Self::CHAR_LOGIN_DUPLICATE_CHARACTER,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_NO_INSTANCES => Self::CHAR_LOGIN_NO_INSTANCES,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_FAILED => Self::CHAR_LOGIN_FAILED,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_DISABLED => Self::CHAR_LOGIN_DISABLED,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_NO_CHARACTER => Self::CHAR_LOGIN_NO_CHARACTER,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_LOGIN_LOCKED_FOR_TRANSFER => Self::CHAR_LOGIN_LOCKED_FOR_TRANSFER,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_NO_NAME => Self::CHAR_NAME_NO_NAME,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_TOO_SHORT => Self::CHAR_NAME_TOO_SHORT,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_TOO_LONG => Self::CHAR_NAME_TOO_LONG,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_ONLY_LETTERS => Self::CHAR_NAME_ONLY_LETTERS,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_MIXED_LANGUAGES => Self::CHAR_NAME_MIXED_LANGUAGES,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_PROFANE => Self::CHAR_NAME_PROFANE,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_RESERVED => Self::CHAR_NAME_RESERVED,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_INVALID_APOSTROPHE => Self::CHAR_NAME_INVALID_APOSTROPHE,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_MULTIPLE_APOSTROPHES => Self::CHAR_NAME_MULTIPLE_APOSTROPHES,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_THREE_CONSECUTIVE => Self::CHAR_NAME_THREE_CONSECUTIVE,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_INVALID_SPACE => Self::CHAR_NAME_INVALID_SPACE,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_SUCCESS => Self::CHAR_NAME_SUCCESS,
            SMSG_AUTH_RESPONSEWorldResult::CHAR_NAME_FAILURE => Self::CHAR_NAME_FAILURE,
        }
    }
}

impl Default for SMSG_AUTH_RESPONSEWorldResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::RESPONSE_SUCCESS
    }
}

impl SMSG_AUTH_RESPONSEWorldResult {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: WorldResult = self.into();
        a.write(w)?;
        Ok(())
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: WorldResult = self.into();
        a.write_u32_be(w)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: WorldResult = self.into();
        a.write_u64_le(w)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: WorldResult = self.into();
        a.write_u64_be(w)
    }

}

impl VariableSized for SMSG_AUTH_RESPONSEWorldResult {
    fn size(&self) -> usize {
        match self {
            Self::RESPONSE_SUCCESS =>  {
                4
            }
            Self::RESPONSE_FAILURE =>  {
                4
            }
            Self::RESPONSE_CANCELLED =>  {
                4
            }
            Self::RESPONSE_DISCONNECTED =>  {
                4
            }
            Self::RESPONSE_FAILED_TO_CONNECT =>  {
                4
            }
            Self::RESPONSE_CONNECTED =>  {
                4
            }
            Self::RESPONSE_VERSION_MISMATCH =>  {
                4
            }
            Self::CSTATUS_CONNECTING =>  {
                4
            }
            Self::CSTATUS_NEGOTIATING_SECURITY =>  {
                4
            }
            Self::CSTATUS_NEGOTIATION_COMPLETE =>  {
                4
            }
            Self::CSTATUS_NEGOTIATION_FAILED =>  {
                4
            }
            Self::CSTATUS_AUTHENTICATING =>  {
                4
            }
            Self::AUTH_OK  {
                billing_time,
                billing_flags,
                billing_rested,
            } => {
                4
                + 4 // billing_time: u32
                + 1 // billing_flags: u8
                + 4 // billing_rested: u32
            }
            Self::AUTH_FAILED =>  {
                4
            }
            Self::AUTH_REJECT =>  {
                4
            }
            Self::AUTH_BAD_SERVER_PROOF =>  {
                4
            }
            Self::AUTH_UNAVAILABLE =>  {
                4
            }
            Self::AUTH_SYSTEM_ERROR =>  {
                4
            }
            Self::AUTH_BILLING_ERROR =>  {
                4
            }
            Self::AUTH_BILLING_EXPIRED =>  {
                4
            }
            Self::AUTH_VERSION_MISMATCH =>  {
                4
            }
            Self::AUTH_UNKNOWN_ACCOUNT =>  {
                4
            }
            Self::AUTH_INCORRECT_PASSWORD =>  {
                4
            }
            Self::AUTH_SESSION_EXPIRED =>  {
                4
            }
            Self::AUTH_SERVER_SHUTTING_DOWN =>  {
                4
            }
            Self::AUTH_ALREADY_LOGGING_IN =>  {
                4
            }
            Self::AUTH_LOGIN_SERVER_NOT_FOUND =>  {
                4
            }
            Self::AUTH_WAIT_QUEUE  {
                queue_position,
            } => {
                4
                + 4 // queue_position: u32
            }
            Self::AUTH_BANNED =>  {
                4
            }
            Self::AUTH_ALREADY_ONLINE =>  {
                4
            }
            Self::AUTH_NO_TIME =>  {
                4
            }
            Self::AUTH_DB_BUSY =>  {
                4
            }
            Self::AUTH_SUSPENDED =>  {
                4
            }
            Self::AUTH_PARENTAL_CONTROL =>  {
                4
            }
            Self::REALM_LIST_IN_PROGRESS =>  {
                4
            }
            Self::REALM_LIST_SUCCESS =>  {
                4
            }
            Self::REALM_LIST_FAILED =>  {
                4
            }
            Self::REALM_LIST_INVALID =>  {
                4
            }
            Self::REALM_LIST_REALM_NOT_FOUND =>  {
                4
            }
            Self::ACCOUNT_CREATE_IN_PROGRESS =>  {
                4
            }
            Self::ACCOUNT_CREATE_SUCCESS =>  {
                4
            }
            Self::ACCOUNT_CREATE_FAILED =>  {
                4
            }
            Self::CHAR_LIST_RETRIEVING =>  {
                4
            }
            Self::CHAR_LIST_RETRIEVED =>  {
                4
            }
            Self::CHAR_LIST_FAILED =>  {
                4
            }
            Self::CHAR_CREATE_IN_PROGRESS =>  {
                4
            }
            Self::CHAR_CREATE_SUCCESS =>  {
                4
            }
            Self::CHAR_CREATE_ERROR =>  {
                4
            }
            Self::CHAR_CREATE_FAILED =>  {
                4
            }
            Self::CHAR_CREATE_NAME_IN_USE =>  {
                4
            }
            Self::CHAR_CREATE_DISABLED =>  {
                4
            }
            Self::CHAR_CREATE_PVP_TEAMS_VIOLATION =>  {
                4
            }
            Self::CHAR_CREATE_SERVER_LIMIT =>  {
                4
            }
            Self::CHAR_CREATE_ACCOUNT_LIMIT =>  {
                4
            }
            Self::CHAR_CREATE_SERVER_QUEUE =>  {
                4
            }
            Self::CHAR_CREATE_ONLY_EXISTING =>  {
                4
            }
            Self::CHAR_DELETE_IN_PROGRESS =>  {
                4
            }
            Self::CHAR_DELETE_SUCCESS =>  {
                4
            }
            Self::CHAR_DELETE_FAILED =>  {
                4
            }
            Self::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER =>  {
                4
            }
            Self::CHAR_LOGIN_IN_PROGRESS =>  {
                4
            }
            Self::CHAR_LOGIN_SUCCESS =>  {
                4
            }
            Self::CHAR_LOGIN_NO_WORLD =>  {
                4
            }
            Self::CHAR_LOGIN_DUPLICATE_CHARACTER =>  {
                4
            }
            Self::CHAR_LOGIN_NO_INSTANCES =>  {
                4
            }
            Self::CHAR_LOGIN_FAILED =>  {
                4
            }
            Self::CHAR_LOGIN_DISABLED =>  {
                4
            }
            Self::CHAR_LOGIN_NO_CHARACTER =>  {
                4
            }
            Self::CHAR_LOGIN_LOCKED_FOR_TRANSFER =>  {
                4
            }
            Self::CHAR_NAME_NO_NAME =>  {
                4
            }
            Self::CHAR_NAME_TOO_SHORT =>  {
                4
            }
            Self::CHAR_NAME_TOO_LONG =>  {
                4
            }
            Self::CHAR_NAME_ONLY_LETTERS =>  {
                4
            }
            Self::CHAR_NAME_MIXED_LANGUAGES =>  {
                4
            }
            Self::CHAR_NAME_PROFANE =>  {
                4
            }
            Self::CHAR_NAME_RESERVED =>  {
                4
            }
            Self::CHAR_NAME_INVALID_APOSTROPHE =>  {
                4
            }
            Self::CHAR_NAME_MULTIPLE_APOSTROPHES =>  {
                4
            }
            Self::CHAR_NAME_THREE_CONSECUTIVE =>  {
                4
            }
            Self::CHAR_NAME_INVALID_SPACE =>  {
                4
            }
            Self::CHAR_NAME_SUCCESS =>  {
                4
            }
            Self::CHAR_NAME_FAILURE =>  {
                4
            }
        }
    }
}

impl MaximumPossibleSized for SMSG_AUTH_RESPONSEWorldResult {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}
