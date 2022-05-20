use std::convert::{TryFrom, TryInto};
use crate::world::v1::v2::{WorldResult, WorldResultError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_AUTH_RESPONSE {
    pub result: SMSG_AUTH_RESPONSEWorldResult,
}

impl ServerMessageWrite for SMSG_AUTH_RESPONSE {}

impl SMSG_AUTH_RESPONSE {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // result: WorldResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

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
                billing_flags,
                billing_rested,
                billing_time,
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

        Ok(w)
    }
}

impl MessageBody for SMSG_AUTH_RESPONSE {
    const OPCODE: u16 = 0x01ee;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_AUTH_RESPONSEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: WorldResult
        let result: WorldResult = crate::util::read_u32_le(r)?.try_into()?;

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
                    billing_flags,
                    billing_rested,
                    billing_time,
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

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
    }

    #[cfg(feature = "tokio")]
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
            // result: WorldResult
            let result: WorldResult = crate::util::tokio_read_u32_le(r).await?.try_into()?;

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
                    let billing_time = crate::util::tokio_read_u32_le(r).await?;

                    // billing_flags: u8
                    let billing_flags = crate::util::tokio_read_u8_le(r).await?;

                    // billing_rested: u32
                    let billing_rested = crate::util::tokio_read_u32_le(r).await?;

                    SMSG_AUTH_RESPONSEWorldResult::AUTH_OK {
                        billing_flags,
                        billing_rested,
                        billing_time,
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
                    let queue_position = crate::util::tokio_read_u32_le(r).await?;

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
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

    #[cfg(feature = "async-std")]
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
            // result: WorldResult
            let result: WorldResult = crate::util::astd_read_u32_le(r).await?.try_into()?;

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
                    let billing_time = crate::util::astd_read_u32_le(r).await?;

                    // billing_flags: u8
                    let billing_flags = crate::util::astd_read_u8_le(r).await?;

                    // billing_rested: u32
                    let billing_rested = crate::util::astd_read_u32_le(r).await?;

                    SMSG_AUTH_RESPONSEWorldResult::AUTH_OK {
                        billing_flags,
                        billing_rested,
                        billing_time,
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
                    let queue_position = crate::util::astd_read_u32_le(r).await?;

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
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

impl SMSG_AUTH_RESPONSE {
    pub fn size(&self) -> usize {
        0
        + self.result.size() // result: SMSG_AUTH_RESPONSEWorldResult
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
        billing_flags: u8,
        billing_rested: u32,
        billing_time: u32,
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

impl Default for SMSG_AUTH_RESPONSEWorldResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::RESPONSE_SUCCESS
    }
}

impl SMSG_AUTH_RESPONSEWorldResult {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::RESPONSE_SUCCESS => 0,
            Self::RESPONSE_FAILURE => 1,
            Self::RESPONSE_CANCELLED => 2,
            Self::RESPONSE_DISCONNECTED => 3,
            Self::RESPONSE_FAILED_TO_CONNECT => 4,
            Self::RESPONSE_CONNECTED => 5,
            Self::RESPONSE_VERSION_MISMATCH => 6,
            Self::CSTATUS_CONNECTING => 7,
            Self::CSTATUS_NEGOTIATING_SECURITY => 8,
            Self::CSTATUS_NEGOTIATION_COMPLETE => 9,
            Self::CSTATUS_NEGOTIATION_FAILED => 10,
            Self::CSTATUS_AUTHENTICATING => 11,
            Self::AUTH_OK { .. } => 12,
            Self::AUTH_FAILED => 13,
            Self::AUTH_REJECT => 14,
            Self::AUTH_BAD_SERVER_PROOF => 15,
            Self::AUTH_UNAVAILABLE => 16,
            Self::AUTH_SYSTEM_ERROR => 17,
            Self::AUTH_BILLING_ERROR => 18,
            Self::AUTH_BILLING_EXPIRED => 19,
            Self::AUTH_VERSION_MISMATCH => 20,
            Self::AUTH_UNKNOWN_ACCOUNT => 21,
            Self::AUTH_INCORRECT_PASSWORD => 22,
            Self::AUTH_SESSION_EXPIRED => 23,
            Self::AUTH_SERVER_SHUTTING_DOWN => 24,
            Self::AUTH_ALREADY_LOGGING_IN => 25,
            Self::AUTH_LOGIN_SERVER_NOT_FOUND => 26,
            Self::AUTH_WAIT_QUEUE { .. } => 27,
            Self::AUTH_BANNED => 28,
            Self::AUTH_ALREADY_ONLINE => 29,
            Self::AUTH_NO_TIME => 30,
            Self::AUTH_DB_BUSY => 31,
            Self::AUTH_SUSPENDED => 32,
            Self::AUTH_PARENTAL_CONTROL => 33,
            Self::REALM_LIST_IN_PROGRESS => 34,
            Self::REALM_LIST_SUCCESS => 35,
            Self::REALM_LIST_FAILED => 36,
            Self::REALM_LIST_INVALID => 37,
            Self::REALM_LIST_REALM_NOT_FOUND => 38,
            Self::ACCOUNT_CREATE_IN_PROGRESS => 39,
            Self::ACCOUNT_CREATE_SUCCESS => 40,
            Self::ACCOUNT_CREATE_FAILED => 41,
            Self::CHAR_LIST_RETRIEVING => 42,
            Self::CHAR_LIST_RETRIEVED => 43,
            Self::CHAR_LIST_FAILED => 44,
            Self::CHAR_CREATE_IN_PROGRESS => 45,
            Self::CHAR_CREATE_SUCCESS => 46,
            Self::CHAR_CREATE_ERROR => 47,
            Self::CHAR_CREATE_FAILED => 48,
            Self::CHAR_CREATE_NAME_IN_USE => 49,
            Self::CHAR_CREATE_DISABLED => 50,
            Self::CHAR_CREATE_PVP_TEAMS_VIOLATION => 51,
            Self::CHAR_CREATE_SERVER_LIMIT => 52,
            Self::CHAR_CREATE_ACCOUNT_LIMIT => 53,
            Self::CHAR_CREATE_SERVER_QUEUE => 54,
            Self::CHAR_CREATE_ONLY_EXISTING => 55,
            Self::CHAR_DELETE_IN_PROGRESS => 56,
            Self::CHAR_DELETE_SUCCESS => 57,
            Self::CHAR_DELETE_FAILED => 58,
            Self::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER => 59,
            Self::CHAR_LOGIN_IN_PROGRESS => 60,
            Self::CHAR_LOGIN_SUCCESS => 61,
            Self::CHAR_LOGIN_NO_WORLD => 62,
            Self::CHAR_LOGIN_DUPLICATE_CHARACTER => 63,
            Self::CHAR_LOGIN_NO_INSTANCES => 64,
            Self::CHAR_LOGIN_FAILED => 65,
            Self::CHAR_LOGIN_DISABLED => 66,
            Self::CHAR_LOGIN_NO_CHARACTER => 67,
            Self::CHAR_LOGIN_LOCKED_FOR_TRANSFER => 68,
            Self::CHAR_NAME_NO_NAME => 69,
            Self::CHAR_NAME_TOO_SHORT => 70,
            Self::CHAR_NAME_TOO_LONG => 71,
            Self::CHAR_NAME_ONLY_LETTERS => 72,
            Self::CHAR_NAME_MIXED_LANGUAGES => 73,
            Self::CHAR_NAME_PROFANE => 74,
            Self::CHAR_NAME_RESERVED => 75,
            Self::CHAR_NAME_INVALID_APOSTROPHE => 76,
            Self::CHAR_NAME_MULTIPLE_APOSTROPHES => 77,
            Self::CHAR_NAME_THREE_CONSECUTIVE => 78,
            Self::CHAR_NAME_INVALID_SPACE => 79,
            Self::CHAR_NAME_SUCCESS => 80,
            Self::CHAR_NAME_FAILURE => 81,
        }
    }

}

impl SMSG_AUTH_RESPONSEWorldResult {
    pub fn size(&self) -> usize {
        match self {
            Self::RESPONSE_SUCCESS => {
                4
            }
            Self::RESPONSE_FAILURE => {
                4
            }
            Self::RESPONSE_CANCELLED => {
                4
            }
            Self::RESPONSE_DISCONNECTED => {
                4
            }
            Self::RESPONSE_FAILED_TO_CONNECT => {
                4
            }
            Self::RESPONSE_CONNECTED => {
                4
            }
            Self::RESPONSE_VERSION_MISMATCH => {
                4
            }
            Self::CSTATUS_CONNECTING => {
                4
            }
            Self::CSTATUS_NEGOTIATING_SECURITY => {
                4
            }
            Self::CSTATUS_NEGOTIATION_COMPLETE => {
                4
            }
            Self::CSTATUS_NEGOTIATION_FAILED => {
                4
            }
            Self::CSTATUS_AUTHENTICATING => {
                4
            }
            Self::AUTH_OK {
                billing_flags,
                billing_rested,
                billing_time,
            } => {
                4
                + 1 // billing_flags: u8
                + 4 // billing_rested: u32
                + 4 // billing_time: u32
            }
            Self::AUTH_FAILED => {
                4
            }
            Self::AUTH_REJECT => {
                4
            }
            Self::AUTH_BAD_SERVER_PROOF => {
                4
            }
            Self::AUTH_UNAVAILABLE => {
                4
            }
            Self::AUTH_SYSTEM_ERROR => {
                4
            }
            Self::AUTH_BILLING_ERROR => {
                4
            }
            Self::AUTH_BILLING_EXPIRED => {
                4
            }
            Self::AUTH_VERSION_MISMATCH => {
                4
            }
            Self::AUTH_UNKNOWN_ACCOUNT => {
                4
            }
            Self::AUTH_INCORRECT_PASSWORD => {
                4
            }
            Self::AUTH_SESSION_EXPIRED => {
                4
            }
            Self::AUTH_SERVER_SHUTTING_DOWN => {
                4
            }
            Self::AUTH_ALREADY_LOGGING_IN => {
                4
            }
            Self::AUTH_LOGIN_SERVER_NOT_FOUND => {
                4
            }
            Self::AUTH_WAIT_QUEUE {
                queue_position,
            } => {
                4
                + 4 // queue_position: u32
            }
            Self::AUTH_BANNED => {
                4
            }
            Self::AUTH_ALREADY_ONLINE => {
                4
            }
            Self::AUTH_NO_TIME => {
                4
            }
            Self::AUTH_DB_BUSY => {
                4
            }
            Self::AUTH_SUSPENDED => {
                4
            }
            Self::AUTH_PARENTAL_CONTROL => {
                4
            }
            Self::REALM_LIST_IN_PROGRESS => {
                4
            }
            Self::REALM_LIST_SUCCESS => {
                4
            }
            Self::REALM_LIST_FAILED => {
                4
            }
            Self::REALM_LIST_INVALID => {
                4
            }
            Self::REALM_LIST_REALM_NOT_FOUND => {
                4
            }
            Self::ACCOUNT_CREATE_IN_PROGRESS => {
                4
            }
            Self::ACCOUNT_CREATE_SUCCESS => {
                4
            }
            Self::ACCOUNT_CREATE_FAILED => {
                4
            }
            Self::CHAR_LIST_RETRIEVING => {
                4
            }
            Self::CHAR_LIST_RETRIEVED => {
                4
            }
            Self::CHAR_LIST_FAILED => {
                4
            }
            Self::CHAR_CREATE_IN_PROGRESS => {
                4
            }
            Self::CHAR_CREATE_SUCCESS => {
                4
            }
            Self::CHAR_CREATE_ERROR => {
                4
            }
            Self::CHAR_CREATE_FAILED => {
                4
            }
            Self::CHAR_CREATE_NAME_IN_USE => {
                4
            }
            Self::CHAR_CREATE_DISABLED => {
                4
            }
            Self::CHAR_CREATE_PVP_TEAMS_VIOLATION => {
                4
            }
            Self::CHAR_CREATE_SERVER_LIMIT => {
                4
            }
            Self::CHAR_CREATE_ACCOUNT_LIMIT => {
                4
            }
            Self::CHAR_CREATE_SERVER_QUEUE => {
                4
            }
            Self::CHAR_CREATE_ONLY_EXISTING => {
                4
            }
            Self::CHAR_DELETE_IN_PROGRESS => {
                4
            }
            Self::CHAR_DELETE_SUCCESS => {
                4
            }
            Self::CHAR_DELETE_FAILED => {
                4
            }
            Self::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER => {
                4
            }
            Self::CHAR_LOGIN_IN_PROGRESS => {
                4
            }
            Self::CHAR_LOGIN_SUCCESS => {
                4
            }
            Self::CHAR_LOGIN_NO_WORLD => {
                4
            }
            Self::CHAR_LOGIN_DUPLICATE_CHARACTER => {
                4
            }
            Self::CHAR_LOGIN_NO_INSTANCES => {
                4
            }
            Self::CHAR_LOGIN_FAILED => {
                4
            }
            Self::CHAR_LOGIN_DISABLED => {
                4
            }
            Self::CHAR_LOGIN_NO_CHARACTER => {
                4
            }
            Self::CHAR_LOGIN_LOCKED_FOR_TRANSFER => {
                4
            }
            Self::CHAR_NAME_NO_NAME => {
                4
            }
            Self::CHAR_NAME_TOO_SHORT => {
                4
            }
            Self::CHAR_NAME_TOO_LONG => {
                4
            }
            Self::CHAR_NAME_ONLY_LETTERS => {
                4
            }
            Self::CHAR_NAME_MIXED_LANGUAGES => {
                4
            }
            Self::CHAR_NAME_PROFANE => {
                4
            }
            Self::CHAR_NAME_RESERVED => {
                4
            }
            Self::CHAR_NAME_INVALID_APOSTROPHE => {
                4
            }
            Self::CHAR_NAME_MULTIPLE_APOSTROPHES => {
                4
            }
            Self::CHAR_NAME_THREE_CONSECUTIVE => {
                4
            }
            Self::CHAR_NAME_INVALID_SPACE => {
                4
            }
            Self::CHAR_NAME_SUCCESS => {
                4
            }
            Self::CHAR_NAME_FAILURE => {
                4
            }
        }
    }
}

