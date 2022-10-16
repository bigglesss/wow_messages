use crate::tbc::{ServerMessage, ClientMessage};
use wow_srp::tbc_header::{DecrypterHalf, EncrypterHalf};

#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use crate::world::tbc::MovementInfo;
use crate::world::tbc::MSG_MOVE_WORLDPORT_ACK;
use crate::world::tbc::CMSG_CHAR_CREATE;
use crate::world::tbc::CMSG_CHAR_ENUM;
use crate::world::tbc::CMSG_CHAR_DELETE;
use crate::world::tbc::CMSG_PLAYER_LOGIN;
use crate::world::tbc::CMSG_PLAYER_LOGOUT;
use crate::world::tbc::CMSG_LOGOUT_REQUEST;
use crate::world::tbc::CMSG_LOGOUT_CANCEL;
use crate::world::tbc::CMSG_NAME_QUERY;
use crate::world::tbc::CMSG_MESSAGECHAT;
use crate::world::tbc::CMSG_JOIN_CHANNEL;
use crate::world::tbc::CMSG_LEAVE_CHANNEL;
use crate::world::tbc::CMSG_CHANNEL_LIST;
use crate::world::tbc::CMSG_CHANNEL_PASSWORD;
use crate::world::tbc::CMSG_CHANNEL_SET_OWNER;
use crate::world::tbc::CMSG_CHANNEL_OWNER;
use crate::world::tbc::CMSG_CHANNEL_MODERATOR;
use crate::world::tbc::CMSG_CHANNEL_UNMODERATOR;
use crate::world::tbc::CMSG_CHANNEL_MUTE;
use crate::world::tbc::CMSG_CHANNEL_UNMUTE;
use crate::world::tbc::CMSG_CHANNEL_INVITE;
use crate::world::tbc::CMSG_CHANNEL_KICK;
use crate::world::tbc::CMSG_CHANNEL_BAN;
use crate::world::tbc::CMSG_CHANNEL_UNBAN;
use crate::world::tbc::CMSG_CHANNEL_ANNOUNCEMENTS;
use crate::world::tbc::CMSG_CHANNEL_MODERATE;
use crate::world::tbc::CMSG_AREATRIGGER;
use crate::world::tbc::MSG_MOVE_START_FORWARD_Client;
use crate::world::tbc::MSG_MOVE_START_BACKWARD_Client;
use crate::world::tbc::MSG_MOVE_STOP_Client;
use crate::world::tbc::MSG_MOVE_START_STRAFE_LEFT_Client;
use crate::world::tbc::MSG_MOVE_START_STRAFE_RIGHT_Client;
use crate::world::tbc::MSG_MOVE_STOP_STRAFE_Client;
use crate::world::tbc::MSG_MOVE_JUMP_Client;
use crate::world::tbc::MSG_MOVE_START_TURN_LEFT_Client;
use crate::world::tbc::MSG_MOVE_START_TURN_RIGHT_Client;
use crate::world::tbc::MSG_MOVE_STOP_TURN_Client;
use crate::world::tbc::MSG_MOVE_START_PITCH_UP_Client;
use crate::world::tbc::MSG_MOVE_START_PITCH_DOWN_Client;
use crate::world::tbc::MSG_MOVE_STOP_PITCH_Client;
use crate::world::tbc::MSG_MOVE_SET_RUN_MODE_Client;
use crate::world::tbc::MSG_MOVE_SET_WALK_MODE_Client;
use crate::world::tbc::MSG_MOVE_TELEPORT_ACK_Client;
use crate::world::tbc::MSG_MOVE_FALL_LAND_Client;
use crate::world::tbc::MSG_MOVE_START_SWIM_Client;
use crate::world::tbc::MSG_MOVE_STOP_SWIM_Client;
use crate::world::tbc::MSG_MOVE_SET_FACING_Client;
use crate::world::tbc::MSG_MOVE_SET_PITCH_Client;
use crate::world::tbc::CMSG_FORCE_RUN_SPEED_CHANGE_ACK;
use crate::world::tbc::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK;
use crate::world::tbc::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK;
use crate::world::tbc::CMSG_FORCE_MOVE_ROOT_ACK;
use crate::world::tbc::CMSG_FORCE_MOVE_UNROOT_ACK;
use crate::world::tbc::MSG_MOVE_HEARTBEAT_Client;
use crate::world::tbc::CMSG_NEXT_CINEMATIC_CAMERA;
use crate::world::tbc::CMSG_COMPLETE_CINEMATIC;
use crate::world::tbc::CMSG_TUTORIAL_FLAG;
use crate::world::tbc::CMSG_TUTORIAL_CLEAR;
use crate::world::tbc::CMSG_TUTORIAL_RESET;
use crate::world::tbc::CMSG_EMOTE;
use crate::world::tbc::CMSG_TEXT_EMOTE;
use crate::world::tbc::CMSG_SET_FACTION_ATWAR;
use crate::world::tbc::CMSG_SET_ACTION_BUTTON;
use crate::world::tbc::CMSG_CANCEL_CHANNELLING;
use crate::world::tbc::CMSG_SET_SELECTION;
use crate::world::tbc::CMSG_SET_TARGET_OBSOLETE;
use crate::world::tbc::CMSG_ATTACKSWING;
use crate::world::tbc::CMSG_ATTACKSTOP;
use crate::world::tbc::CMSG_REPOP_REQUEST;
use crate::world::tbc::CMSG_DUEL_ACCEPTED;
use crate::world::tbc::CMSG_DUEL_CANCELLED;
use crate::world::tbc::CMSG_ACTIVATETAXI;
use crate::world::tbc::CMSG_BUG;
use crate::world::tbc::CMSG_QUERY_TIME;
use crate::world::tbc::CMSG_PING;
use crate::world::tbc::CMSG_SETSHEATHED;
use crate::world::tbc::CMSG_AUTH_SESSION;
use crate::world::tbc::CMSG_ZONEUPDATE;
use crate::world::tbc::CMSG_GMTICKET_UPDATETEXT;
use crate::world::tbc::CMSG_REQUEST_ACCOUNT_DATA;
use crate::world::tbc::CMSG_GMTICKET_GETTICKET;
use crate::world::tbc::CMSG_GMTICKET_DELETETICKET;
use crate::world::tbc::CMSG_GMTICKET_SYSTEMSTATUS;
use crate::world::tbc::CMSG_CHAT_IGNORED;
use crate::world::tbc::CMSG_BATTLEFIELD_LIST;
use crate::world::tbc::MSG_AUCTION_HELLO_Client;
use crate::world::tbc::CMSG_AUCTION_SELL_ITEM;
use crate::world::tbc::CMSG_AUCTION_REMOVE_ITEM;
use crate::world::tbc::CMSG_AUCTION_LIST_ITEMS;
use crate::world::tbc::CMSG_AUCTION_LIST_OWNER_ITEMS;
use crate::world::tbc::CMSG_AUCTION_PLACE_BID;
use crate::world::tbc::CMSG_AUCTION_LIST_BIDDER_ITEMS;
use crate::world::tbc::CMSG_SET_ACTIVE_MOVER;
use crate::world::tbc::CMSG_SET_ACTIONBAR_TOGGLES;
use crate::world::tbc::CMSG_CHAR_RENAME;
use crate::world::tbc::CMSG_MOVE_FALL_RESET;
use crate::world::tbc::CMSG_BATTLEFIELD_STATUS;
use crate::world::tbc::CMSG_BATTLEFIELD_PORT;
use crate::world::tbc::CMSG_BATTLEMASTER_HELLO;
use crate::world::tbc::CMSG_FORCE_WALK_SPEED_CHANGE_ACK;
use crate::world::tbc::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK;
use crate::world::tbc::CMSG_FORCE_TURN_RATE_CHANGE_ACK;
use crate::world::tbc::CMSG_LEAVE_BATTLEFIELD;
use crate::world::tbc::MSG_BATTLEGROUND_PLAYER_POSITIONS_Client;
use crate::world::tbc::CMSG_BATTLEMASTER_JOIN;
use crate::world::tbc::CMSG_ACTIVATETAXIEXPRESS;
use crate::world::tbc::CMSG_SET_FACTION_INACTIVE;
use crate::world::tbc::CMSG_SET_WATCHED_FACTION;
use crate::world::tbc::CMSG_GMSURVEY_SUBMIT;
use crate::world::tbc::CMSG_MOVE_SET_FLY;
use crate::world::tbc::MSG_MOVE_START_ASCEND_Client;
use crate::world::tbc::MSG_MOVE_STOP_ASCEND_Client;
use crate::world::tbc::CMSG_REALM_SPLIT;
use crate::world::tbc::CMSG_MOVE_CHNG_TRANSPORT;
use crate::world::tbc::CMSG_TIME_SYNC_RESP;
use crate::world::tbc::MSG_MOVE_START_DESCEND_Client;
use crate::world::tbc::CMSG_SET_ACTIVE_VOICE_CHANNEL;

#[derive(Debug, Clone, PartialEq)]
pub enum ClientOpcodeMessage {
    MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK),
    CMSG_CHAR_CREATE(CMSG_CHAR_CREATE),
    CMSG_CHAR_ENUM(CMSG_CHAR_ENUM),
    CMSG_CHAR_DELETE(CMSG_CHAR_DELETE),
    CMSG_PLAYER_LOGIN(CMSG_PLAYER_LOGIN),
    CMSG_PLAYER_LOGOUT(CMSG_PLAYER_LOGOUT),
    CMSG_LOGOUT_REQUEST(CMSG_LOGOUT_REQUEST),
    CMSG_LOGOUT_CANCEL(CMSG_LOGOUT_CANCEL),
    CMSG_NAME_QUERY(CMSG_NAME_QUERY),
    CMSG_MESSAGECHAT(CMSG_MESSAGECHAT),
    CMSG_JOIN_CHANNEL(CMSG_JOIN_CHANNEL),
    CMSG_LEAVE_CHANNEL(CMSG_LEAVE_CHANNEL),
    CMSG_CHANNEL_LIST(CMSG_CHANNEL_LIST),
    CMSG_CHANNEL_PASSWORD(CMSG_CHANNEL_PASSWORD),
    CMSG_CHANNEL_SET_OWNER(CMSG_CHANNEL_SET_OWNER),
    CMSG_CHANNEL_OWNER(CMSG_CHANNEL_OWNER),
    CMSG_CHANNEL_MODERATOR(CMSG_CHANNEL_MODERATOR),
    CMSG_CHANNEL_UNMODERATOR(CMSG_CHANNEL_UNMODERATOR),
    CMSG_CHANNEL_MUTE(CMSG_CHANNEL_MUTE),
    CMSG_CHANNEL_UNMUTE(CMSG_CHANNEL_UNMUTE),
    CMSG_CHANNEL_INVITE(CMSG_CHANNEL_INVITE),
    CMSG_CHANNEL_KICK(CMSG_CHANNEL_KICK),
    CMSG_CHANNEL_BAN(CMSG_CHANNEL_BAN),
    CMSG_CHANNEL_UNBAN(CMSG_CHANNEL_UNBAN),
    CMSG_CHANNEL_ANNOUNCEMENTS(CMSG_CHANNEL_ANNOUNCEMENTS),
    CMSG_CHANNEL_MODERATE(CMSG_CHANNEL_MODERATE),
    CMSG_AREATRIGGER(CMSG_AREATRIGGER),
    MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD_Client),
    MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD_Client),
    MSG_MOVE_STOP(MSG_MOVE_STOP_Client),
    MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT_Client),
    MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT_Client),
    MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE_Client),
    MSG_MOVE_JUMP(MSG_MOVE_JUMP_Client),
    MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT_Client),
    MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT_Client),
    MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN_Client),
    MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP_Client),
    MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN_Client),
    MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH_Client),
    MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE_Client),
    MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE_Client),
    MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK_Client),
    MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND_Client),
    MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM_Client),
    MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM_Client),
    MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING_Client),
    MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH_Client),
    CMSG_FORCE_RUN_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_SPEED_CHANGE_ACK),
    CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK),
    CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_SPEED_CHANGE_ACK),
    CMSG_FORCE_MOVE_ROOT_ACK(CMSG_FORCE_MOVE_ROOT_ACK),
    CMSG_FORCE_MOVE_UNROOT_ACK(CMSG_FORCE_MOVE_UNROOT_ACK),
    MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT_Client),
    CMSG_NEXT_CINEMATIC_CAMERA(CMSG_NEXT_CINEMATIC_CAMERA),
    CMSG_COMPLETE_CINEMATIC(CMSG_COMPLETE_CINEMATIC),
    CMSG_TUTORIAL_FLAG(CMSG_TUTORIAL_FLAG),
    CMSG_TUTORIAL_CLEAR(CMSG_TUTORIAL_CLEAR),
    CMSG_TUTORIAL_RESET(CMSG_TUTORIAL_RESET),
    CMSG_EMOTE(CMSG_EMOTE),
    CMSG_TEXT_EMOTE(CMSG_TEXT_EMOTE),
    CMSG_SET_FACTION_ATWAR(CMSG_SET_FACTION_ATWAR),
    CMSG_SET_ACTION_BUTTON(CMSG_SET_ACTION_BUTTON),
    CMSG_CANCEL_CHANNELLING(CMSG_CANCEL_CHANNELLING),
    CMSG_SET_SELECTION(CMSG_SET_SELECTION),
    CMSG_SET_TARGET_OBSOLETE(CMSG_SET_TARGET_OBSOLETE),
    CMSG_ATTACKSWING(CMSG_ATTACKSWING),
    CMSG_ATTACKSTOP(CMSG_ATTACKSTOP),
    CMSG_REPOP_REQUEST(CMSG_REPOP_REQUEST),
    CMSG_DUEL_ACCEPTED(CMSG_DUEL_ACCEPTED),
    CMSG_DUEL_CANCELLED(CMSG_DUEL_CANCELLED),
    CMSG_ACTIVATETAXI(CMSG_ACTIVATETAXI),
    CMSG_BUG(CMSG_BUG),
    CMSG_QUERY_TIME(CMSG_QUERY_TIME),
    CMSG_PING(CMSG_PING),
    CMSG_SETSHEATHED(CMSG_SETSHEATHED),
    CMSG_AUTH_SESSION(CMSG_AUTH_SESSION),
    CMSG_ZONEUPDATE(CMSG_ZONEUPDATE),
    CMSG_GMTICKET_UPDATETEXT(CMSG_GMTICKET_UPDATETEXT),
    CMSG_REQUEST_ACCOUNT_DATA(CMSG_REQUEST_ACCOUNT_DATA),
    CMSG_GMTICKET_GETTICKET(CMSG_GMTICKET_GETTICKET),
    CMSG_GMTICKET_DELETETICKET(CMSG_GMTICKET_DELETETICKET),
    CMSG_GMTICKET_SYSTEMSTATUS(CMSG_GMTICKET_SYSTEMSTATUS),
    CMSG_CHAT_IGNORED(CMSG_CHAT_IGNORED),
    CMSG_BATTLEFIELD_LIST(CMSG_BATTLEFIELD_LIST),
    MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Client),
    CMSG_AUCTION_SELL_ITEM(CMSG_AUCTION_SELL_ITEM),
    CMSG_AUCTION_REMOVE_ITEM(CMSG_AUCTION_REMOVE_ITEM),
    CMSG_AUCTION_LIST_ITEMS(CMSG_AUCTION_LIST_ITEMS),
    CMSG_AUCTION_LIST_OWNER_ITEMS(CMSG_AUCTION_LIST_OWNER_ITEMS),
    CMSG_AUCTION_PLACE_BID(CMSG_AUCTION_PLACE_BID),
    CMSG_AUCTION_LIST_BIDDER_ITEMS(CMSG_AUCTION_LIST_BIDDER_ITEMS),
    CMSG_SET_ACTIVE_MOVER(CMSG_SET_ACTIVE_MOVER),
    CMSG_SET_ACTIONBAR_TOGGLES(CMSG_SET_ACTIONBAR_TOGGLES),
    CMSG_CHAR_RENAME(CMSG_CHAR_RENAME),
    CMSG_MOVE_FALL_RESET(CMSG_MOVE_FALL_RESET),
    CMSG_BATTLEFIELD_STATUS(CMSG_BATTLEFIELD_STATUS),
    CMSG_BATTLEFIELD_PORT(CMSG_BATTLEFIELD_PORT),
    CMSG_BATTLEMASTER_HELLO(CMSG_BATTLEMASTER_HELLO),
    CMSG_FORCE_WALK_SPEED_CHANGE_ACK(CMSG_FORCE_WALK_SPEED_CHANGE_ACK),
    CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK),
    CMSG_FORCE_TURN_RATE_CHANGE_ACK(CMSG_FORCE_TURN_RATE_CHANGE_ACK),
    CMSG_LEAVE_BATTLEFIELD(CMSG_LEAVE_BATTLEFIELD),
    MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Client),
    CMSG_BATTLEMASTER_JOIN(CMSG_BATTLEMASTER_JOIN),
    CMSG_ACTIVATETAXIEXPRESS(CMSG_ACTIVATETAXIEXPRESS),
    CMSG_SET_FACTION_INACTIVE(CMSG_SET_FACTION_INACTIVE),
    CMSG_SET_WATCHED_FACTION(CMSG_SET_WATCHED_FACTION),
    CMSG_GMSURVEY_SUBMIT(CMSG_GMSURVEY_SUBMIT),
    CMSG_MOVE_SET_FLY(CMSG_MOVE_SET_FLY),
    MSG_MOVE_START_ASCEND(MSG_MOVE_START_ASCEND_Client),
    MSG_MOVE_STOP_ASCEND(MSG_MOVE_STOP_ASCEND_Client),
    CMSG_REALM_SPLIT(CMSG_REALM_SPLIT),
    CMSG_MOVE_CHNG_TRANSPORT(CMSG_MOVE_CHNG_TRANSPORT),
    CMSG_TIME_SYNC_RESP(CMSG_TIME_SYNC_RESP),
    MSG_MOVE_START_DESCEND(MSG_MOVE_START_DESCEND_Client),
    CMSG_SET_ACTIVE_VOICE_CHANNEL(CMSG_SET_ACTIVE_VOICE_CHANNEL),
}

impl ClientOpcodeMessage {
    fn read_opcodes(opcode: u32, body_size: u32, mut r: &[u8]) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        match opcode {
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(<MSG_MOVE_WORLDPORT_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x0036 => Ok(Self::CMSG_CHAR_CREATE(<CMSG_CHAR_CREATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(<CMSG_CHAR_ENUM as crate::Message>::read_body(&mut r, body_size)?)),
            0x0038 => Ok(Self::CMSG_CHAR_DELETE(<CMSG_CHAR_DELETE as crate::Message>::read_body(&mut r, body_size)?)),
            0x003D => Ok(Self::CMSG_PLAYER_LOGIN(<CMSG_PLAYER_LOGIN as crate::Message>::read_body(&mut r, body_size)?)),
            0x004A => Ok(Self::CMSG_PLAYER_LOGOUT(<CMSG_PLAYER_LOGOUT as crate::Message>::read_body(&mut r, body_size)?)),
            0x004B => Ok(Self::CMSG_LOGOUT_REQUEST(<CMSG_LOGOUT_REQUEST as crate::Message>::read_body(&mut r, body_size)?)),
            0x004E => Ok(Self::CMSG_LOGOUT_CANCEL(<CMSG_LOGOUT_CANCEL as crate::Message>::read_body(&mut r, body_size)?)),
            0x0050 => Ok(Self::CMSG_NAME_QUERY(<CMSG_NAME_QUERY as crate::Message>::read_body(&mut r, body_size)?)),
            0x0095 => Ok(Self::CMSG_MESSAGECHAT(<CMSG_MESSAGECHAT as crate::Message>::read_body(&mut r, body_size)?)),
            0x0097 => Ok(Self::CMSG_JOIN_CHANNEL(<CMSG_JOIN_CHANNEL as crate::Message>::read_body(&mut r, body_size)?)),
            0x0098 => Ok(Self::CMSG_LEAVE_CHANNEL(<CMSG_LEAVE_CHANNEL as crate::Message>::read_body(&mut r, body_size)?)),
            0x009A => Ok(Self::CMSG_CHANNEL_LIST(<CMSG_CHANNEL_LIST as crate::Message>::read_body(&mut r, body_size)?)),
            0x009C => Ok(Self::CMSG_CHANNEL_PASSWORD(<CMSG_CHANNEL_PASSWORD as crate::Message>::read_body(&mut r, body_size)?)),
            0x009D => Ok(Self::CMSG_CHANNEL_SET_OWNER(<CMSG_CHANNEL_SET_OWNER as crate::Message>::read_body(&mut r, body_size)?)),
            0x009E => Ok(Self::CMSG_CHANNEL_OWNER(<CMSG_CHANNEL_OWNER as crate::Message>::read_body(&mut r, body_size)?)),
            0x009F => Ok(Self::CMSG_CHANNEL_MODERATOR(<CMSG_CHANNEL_MODERATOR as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A0 => Ok(Self::CMSG_CHANNEL_UNMODERATOR(<CMSG_CHANNEL_UNMODERATOR as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A1 => Ok(Self::CMSG_CHANNEL_MUTE(<CMSG_CHANNEL_MUTE as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A2 => Ok(Self::CMSG_CHANNEL_UNMUTE(<CMSG_CHANNEL_UNMUTE as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A3 => Ok(Self::CMSG_CHANNEL_INVITE(<CMSG_CHANNEL_INVITE as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A4 => Ok(Self::CMSG_CHANNEL_KICK(<CMSG_CHANNEL_KICK as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A5 => Ok(Self::CMSG_CHANNEL_BAN(<CMSG_CHANNEL_BAN as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A6 => Ok(Self::CMSG_CHANNEL_UNBAN(<CMSG_CHANNEL_UNBAN as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A7 => Ok(Self::CMSG_CHANNEL_ANNOUNCEMENTS(<CMSG_CHANNEL_ANNOUNCEMENTS as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A8 => Ok(Self::CMSG_CHANNEL_MODERATE(<CMSG_CHANNEL_MODERATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B4 => Ok(Self::CMSG_AREATRIGGER(<CMSG_AREATRIGGER as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(<MSG_MOVE_START_FORWARD_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(<MSG_MOVE_START_BACKWARD_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(<MSG_MOVE_STOP_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(<MSG_MOVE_START_STRAFE_LEFT_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(<MSG_MOVE_START_STRAFE_RIGHT_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(<MSG_MOVE_STOP_STRAFE_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(<MSG_MOVE_JUMP_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(<MSG_MOVE_START_TURN_LEFT_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(<MSG_MOVE_START_TURN_RIGHT_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(<MSG_MOVE_STOP_TURN_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(<MSG_MOVE_START_PITCH_UP_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(<MSG_MOVE_START_PITCH_DOWN_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(<MSG_MOVE_STOP_PITCH_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(<MSG_MOVE_SET_RUN_MODE_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(<MSG_MOVE_SET_WALK_MODE_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(<MSG_MOVE_TELEPORT_ACK_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(<MSG_MOVE_FALL_LAND_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(<MSG_MOVE_START_SWIM_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(<MSG_MOVE_STOP_SWIM_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(<MSG_MOVE_SET_FACING_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(<MSG_MOVE_SET_PITCH_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00E3 => Ok(Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(<CMSG_FORCE_RUN_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x00E5 => Ok(Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(<CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x00E7 => Ok(Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(<CMSG_FORCE_SWIM_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x00E9 => Ok(Self::CMSG_FORCE_MOVE_ROOT_ACK(<CMSG_FORCE_MOVE_ROOT_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x00EB => Ok(Self::CMSG_FORCE_MOVE_UNROOT_ACK(<CMSG_FORCE_MOVE_UNROOT_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(<MSG_MOVE_HEARTBEAT_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00FB => Ok(Self::CMSG_NEXT_CINEMATIC_CAMERA(<CMSG_NEXT_CINEMATIC_CAMERA as crate::Message>::read_body(&mut r, body_size)?)),
            0x00FC => Ok(Self::CMSG_COMPLETE_CINEMATIC(<CMSG_COMPLETE_CINEMATIC as crate::Message>::read_body(&mut r, body_size)?)),
            0x00FE => Ok(Self::CMSG_TUTORIAL_FLAG(<CMSG_TUTORIAL_FLAG as crate::Message>::read_body(&mut r, body_size)?)),
            0x00FF => Ok(Self::CMSG_TUTORIAL_CLEAR(<CMSG_TUTORIAL_CLEAR as crate::Message>::read_body(&mut r, body_size)?)),
            0x0100 => Ok(Self::CMSG_TUTORIAL_RESET(<CMSG_TUTORIAL_RESET as crate::Message>::read_body(&mut r, body_size)?)),
            0x0102 => Ok(Self::CMSG_EMOTE(<CMSG_EMOTE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0104 => Ok(Self::CMSG_TEXT_EMOTE(<CMSG_TEXT_EMOTE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0125 => Ok(Self::CMSG_SET_FACTION_ATWAR(<CMSG_SET_FACTION_ATWAR as crate::Message>::read_body(&mut r, body_size)?)),
            0x0128 => Ok(Self::CMSG_SET_ACTION_BUTTON(<CMSG_SET_ACTION_BUTTON as crate::Message>::read_body(&mut r, body_size)?)),
            0x013B => Ok(Self::CMSG_CANCEL_CHANNELLING(<CMSG_CANCEL_CHANNELLING as crate::Message>::read_body(&mut r, body_size)?)),
            0x013D => Ok(Self::CMSG_SET_SELECTION(<CMSG_SET_SELECTION as crate::Message>::read_body(&mut r, body_size)?)),
            0x013E => Ok(Self::CMSG_SET_TARGET_OBSOLETE(<CMSG_SET_TARGET_OBSOLETE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0141 => Ok(Self::CMSG_ATTACKSWING(<CMSG_ATTACKSWING as crate::Message>::read_body(&mut r, body_size)?)),
            0x0142 => Ok(Self::CMSG_ATTACKSTOP(<CMSG_ATTACKSTOP as crate::Message>::read_body(&mut r, body_size)?)),
            0x015A => Ok(Self::CMSG_REPOP_REQUEST(<CMSG_REPOP_REQUEST as crate::Message>::read_body(&mut r, body_size)?)),
            0x016C => Ok(Self::CMSG_DUEL_ACCEPTED(<CMSG_DUEL_ACCEPTED as crate::Message>::read_body(&mut r, body_size)?)),
            0x016D => Ok(Self::CMSG_DUEL_CANCELLED(<CMSG_DUEL_CANCELLED as crate::Message>::read_body(&mut r, body_size)?)),
            0x01AD => Ok(Self::CMSG_ACTIVATETAXI(<CMSG_ACTIVATETAXI as crate::Message>::read_body(&mut r, body_size)?)),
            0x01CA => Ok(Self::CMSG_BUG(<CMSG_BUG as crate::Message>::read_body(&mut r, body_size)?)),
            0x01CE => Ok(Self::CMSG_QUERY_TIME(<CMSG_QUERY_TIME as crate::Message>::read_body(&mut r, body_size)?)),
            0x01DC => Ok(Self::CMSG_PING(<CMSG_PING as crate::Message>::read_body(&mut r, body_size)?)),
            0x01E0 => Ok(Self::CMSG_SETSHEATHED(<CMSG_SETSHEATHED as crate::Message>::read_body(&mut r, body_size)?)),
            0x01ED => Ok(Self::CMSG_AUTH_SESSION(<CMSG_AUTH_SESSION as crate::Message>::read_body(&mut r, body_size)?)),
            0x01F4 => Ok(Self::CMSG_ZONEUPDATE(<CMSG_ZONEUPDATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0207 => Ok(Self::CMSG_GMTICKET_UPDATETEXT(<CMSG_GMTICKET_UPDATETEXT as crate::Message>::read_body(&mut r, body_size)?)),
            0x020A => Ok(Self::CMSG_REQUEST_ACCOUNT_DATA(<CMSG_REQUEST_ACCOUNT_DATA as crate::Message>::read_body(&mut r, body_size)?)),
            0x0211 => Ok(Self::CMSG_GMTICKET_GETTICKET(<CMSG_GMTICKET_GETTICKET as crate::Message>::read_body(&mut r, body_size)?)),
            0x0217 => Ok(Self::CMSG_GMTICKET_DELETETICKET(<CMSG_GMTICKET_DELETETICKET as crate::Message>::read_body(&mut r, body_size)?)),
            0x021A => Ok(Self::CMSG_GMTICKET_SYSTEMSTATUS(<CMSG_GMTICKET_SYSTEMSTATUS as crate::Message>::read_body(&mut r, body_size)?)),
            0x0225 => Ok(Self::CMSG_CHAT_IGNORED(<CMSG_CHAT_IGNORED as crate::Message>::read_body(&mut r, body_size)?)),
            0x023C => Ok(Self::CMSG_BATTLEFIELD_LIST(<CMSG_BATTLEFIELD_LIST as crate::Message>::read_body(&mut r, body_size)?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(<MSG_AUCTION_HELLO_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x0256 => Ok(Self::CMSG_AUCTION_SELL_ITEM(<CMSG_AUCTION_SELL_ITEM as crate::Message>::read_body(&mut r, body_size)?)),
            0x0257 => Ok(Self::CMSG_AUCTION_REMOVE_ITEM(<CMSG_AUCTION_REMOVE_ITEM as crate::Message>::read_body(&mut r, body_size)?)),
            0x0258 => Ok(Self::CMSG_AUCTION_LIST_ITEMS(<CMSG_AUCTION_LIST_ITEMS as crate::Message>::read_body(&mut r, body_size)?)),
            0x0259 => Ok(Self::CMSG_AUCTION_LIST_OWNER_ITEMS(<CMSG_AUCTION_LIST_OWNER_ITEMS as crate::Message>::read_body(&mut r, body_size)?)),
            0x025A => Ok(Self::CMSG_AUCTION_PLACE_BID(<CMSG_AUCTION_PLACE_BID as crate::Message>::read_body(&mut r, body_size)?)),
            0x0264 => Ok(Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(<CMSG_AUCTION_LIST_BIDDER_ITEMS as crate::Message>::read_body(&mut r, body_size)?)),
            0x026A => Ok(Self::CMSG_SET_ACTIVE_MOVER(<CMSG_SET_ACTIVE_MOVER as crate::Message>::read_body(&mut r, body_size)?)),
            0x02BF => Ok(Self::CMSG_SET_ACTIONBAR_TOGGLES(<CMSG_SET_ACTIONBAR_TOGGLES as crate::Message>::read_body(&mut r, body_size)?)),
            0x02C7 => Ok(Self::CMSG_CHAR_RENAME(<CMSG_CHAR_RENAME as crate::Message>::read_body(&mut r, body_size)?)),
            0x02CA => Ok(Self::CMSG_MOVE_FALL_RESET(<CMSG_MOVE_FALL_RESET as crate::Message>::read_body(&mut r, body_size)?)),
            0x02D3 => Ok(Self::CMSG_BATTLEFIELD_STATUS(<CMSG_BATTLEFIELD_STATUS as crate::Message>::read_body(&mut r, body_size)?)),
            0x02D5 => Ok(Self::CMSG_BATTLEFIELD_PORT(<CMSG_BATTLEFIELD_PORT as crate::Message>::read_body(&mut r, body_size)?)),
            0x02D7 => Ok(Self::CMSG_BATTLEMASTER_HELLO(<CMSG_BATTLEMASTER_HELLO as crate::Message>::read_body(&mut r, body_size)?)),
            0x02DB => Ok(Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(<CMSG_FORCE_WALK_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x02DD => Ok(Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(<CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x02DF => Ok(Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(<CMSG_FORCE_TURN_RATE_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x02E1 => Ok(Self::CMSG_LEAVE_BATTLEFIELD(<CMSG_LEAVE_BATTLEFIELD as crate::Message>::read_body(&mut r, body_size)?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(<MSG_BATTLEGROUND_PLAYER_POSITIONS_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x02EE => Ok(Self::CMSG_BATTLEMASTER_JOIN(<CMSG_BATTLEMASTER_JOIN as crate::Message>::read_body(&mut r, body_size)?)),
            0x0312 => Ok(Self::CMSG_ACTIVATETAXIEXPRESS(<CMSG_ACTIVATETAXIEXPRESS as crate::Message>::read_body(&mut r, body_size)?)),
            0x0317 => Ok(Self::CMSG_SET_FACTION_INACTIVE(<CMSG_SET_FACTION_INACTIVE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0318 => Ok(Self::CMSG_SET_WATCHED_FACTION(<CMSG_SET_WATCHED_FACTION as crate::Message>::read_body(&mut r, body_size)?)),
            0x032A => Ok(Self::CMSG_GMSURVEY_SUBMIT(<CMSG_GMSURVEY_SUBMIT as crate::Message>::read_body(&mut r, body_size)?)),
            0x0346 => Ok(Self::CMSG_MOVE_SET_FLY(<CMSG_MOVE_SET_FLY as crate::Message>::read_body(&mut r, body_size)?)),
            0x0359 => Ok(Self::MSG_MOVE_START_ASCEND(<MSG_MOVE_START_ASCEND_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x035A => Ok(Self::MSG_MOVE_STOP_ASCEND(<MSG_MOVE_STOP_ASCEND_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x038C => Ok(Self::CMSG_REALM_SPLIT(<CMSG_REALM_SPLIT as crate::Message>::read_body(&mut r, body_size)?)),
            0x038D => Ok(Self::CMSG_MOVE_CHNG_TRANSPORT(<CMSG_MOVE_CHNG_TRANSPORT as crate::Message>::read_body(&mut r, body_size)?)),
            0x0391 => Ok(Self::CMSG_TIME_SYNC_RESP(<CMSG_TIME_SYNC_RESP as crate::Message>::read_body(&mut r, body_size)?)),
            0x03A7 => Ok(Self::MSG_MOVE_START_DESCEND(<MSG_MOVE_START_DESCEND_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x03D2 => Ok(Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(<CMSG_SET_ACTIVE_VOICE_CHANNEL as crate::Message>::read_body(&mut r, body_size)?)),
            _ => Err(crate::errors::ExpectedOpcodeError::Opcode{ opcode, size: body_size }),
        }
    }

    #[cfg(feature = "sync")]
    pub fn read_unencrypted<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::read_u16_be(r)?.saturating_sub(4)) as u32;
        let opcode = crate::util::read_u32_le(r)?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "sync")]
    pub fn read_encrypted<R: std::io::Read>(r: &mut R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 6];
        r.read_exact(&mut header)?;
        let header = d.decrypt_client_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(4)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read_unencrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::tokio_read_u16_be(r).await?.saturating_sub(4)) as u32;
        let opcode = crate::util::tokio_read_u32_le(r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "tokio")]
    pub async fn tokio_read_encrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 6];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_client_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(4)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read_unencrypted<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::astd_read_u16_be(r).await?.saturating_sub(4)) as u32;
        let opcode = crate::util::astd_read_u32_le(r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "async-std")]
    pub async fn astd_read_encrypted<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 6];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_client_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(4)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "sync")]
    pub fn write_encrypted_client<W: std::io::Write>(&self, w: &mut W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_CREATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_ENUM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_DELETE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PLAYER_LOGIN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PLAYER_LOGOUT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOGOUT_REQUEST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOGOUT_CANCEL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_NAME_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MESSAGECHAT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_JOIN_CHANNEL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LEAVE_CHANNEL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_LIST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_PASSWORD(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_SET_OWNER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_OWNER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_MODERATOR(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_UNMODERATOR(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_MUTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_UNMUTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_INVITE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_KICK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_BAN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_UNBAN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_ANNOUNCEMENTS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_MODERATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AREATRIGGER(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_FORWARD(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_BACKWARD(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_STOP(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_STOP_STRAFE(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_JUMP(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_STOP_TURN(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_PITCH_UP(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_STOP_PITCH(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_FALL_LAND(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_SWIM(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_STOP_SWIM(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_SET_FACING(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_SET_PITCH(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_HEARTBEAT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TUTORIAL_FLAG(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TUTORIAL_CLEAR(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TUTORIAL_RESET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_EMOTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TEXT_EMOTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_FACTION_ATWAR(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_ACTION_BUTTON(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CANCEL_CHANNELLING(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_SELECTION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ATTACKSWING(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ATTACKSTOP(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REPOP_REQUEST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_DUEL_ACCEPTED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_DUEL_CANCELLED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ACTIVATETAXI(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BUG(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUERY_TIME(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PING(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SETSHEATHED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUTH_SESSION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ZONEUPDATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_GETTICKET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAT_IGNORED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEFIELD_LIST(c) => c.write_encrypted_client(w, e),
            Self::MSG_AUCTION_HELLO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_PLACE_BID(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_RENAME(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_FALL_RESET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEFIELD_PORT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.write_encrypted_client(w, e),
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_WATCHED_FACTION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_SET_FLY(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_ASCEND(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_STOP_ASCEND(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REALM_SPLIT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TIME_SYNC_RESP(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_DESCEND(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.write_encrypted_client(w, e),
        }
    }

    #[cfg(feature = "sync")]
    pub fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_CREATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_ENUM(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_DELETE(c) => c.write_unencrypted_client(w),
            Self::CMSG_PLAYER_LOGIN(c) => c.write_unencrypted_client(w),
            Self::CMSG_PLAYER_LOGOUT(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOGOUT_REQUEST(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOGOUT_CANCEL(c) => c.write_unencrypted_client(w),
            Self::CMSG_NAME_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_MESSAGECHAT(c) => c.write_unencrypted_client(w),
            Self::CMSG_JOIN_CHANNEL(c) => c.write_unencrypted_client(w),
            Self::CMSG_LEAVE_CHANNEL(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_LIST(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_PASSWORD(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_SET_OWNER(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_OWNER(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_MODERATOR(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_UNMODERATOR(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_MUTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_UNMUTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_INVITE(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_KICK(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_BAN(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_UNBAN(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_ANNOUNCEMENTS(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_MODERATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_AREATRIGGER(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_FORWARD(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_BACKWARD(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_STOP(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_STOP_STRAFE(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_JUMP(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_STOP_TURN(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_PITCH_UP(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_STOP_PITCH(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_FALL_LAND(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_SWIM(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_STOP_SWIM(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_SET_FACING(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_SET_PITCH(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_HEARTBEAT(c) => c.write_unencrypted_client(w),
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.write_unencrypted_client(w),
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.write_unencrypted_client(w),
            Self::CMSG_TUTORIAL_FLAG(c) => c.write_unencrypted_client(w),
            Self::CMSG_TUTORIAL_CLEAR(c) => c.write_unencrypted_client(w),
            Self::CMSG_TUTORIAL_RESET(c) => c.write_unencrypted_client(w),
            Self::CMSG_EMOTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_TEXT_EMOTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_FACTION_ATWAR(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_ACTION_BUTTON(c) => c.write_unencrypted_client(w),
            Self::CMSG_CANCEL_CHANNELLING(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_SELECTION(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.write_unencrypted_client(w),
            Self::CMSG_ATTACKSWING(c) => c.write_unencrypted_client(w),
            Self::CMSG_ATTACKSTOP(c) => c.write_unencrypted_client(w),
            Self::CMSG_REPOP_REQUEST(c) => c.write_unencrypted_client(w),
            Self::CMSG_DUEL_ACCEPTED(c) => c.write_unencrypted_client(w),
            Self::CMSG_DUEL_CANCELLED(c) => c.write_unencrypted_client(w),
            Self::CMSG_ACTIVATETAXI(c) => c.write_unencrypted_client(w),
            Self::CMSG_BUG(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUERY_TIME(c) => c.write_unencrypted_client(w),
            Self::CMSG_PING(c) => c.write_unencrypted_client(w),
            Self::CMSG_SETSHEATHED(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUTH_SESSION(c) => c.write_unencrypted_client(w),
            Self::CMSG_ZONEUPDATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.write_unencrypted_client(w),
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_GETTICKET(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAT_IGNORED(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEFIELD_LIST(c) => c.write_unencrypted_client(w),
            Self::MSG_AUCTION_HELLO(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_PLACE_BID(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_RENAME(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_FALL_RESET(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEFIELD_PORT(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.write_unencrypted_client(w),
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.write_unencrypted_client(w),
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_WATCHED_FACTION(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_SET_FLY(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_ASCEND(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_STOP_ASCEND(c) => c.write_unencrypted_client(w),
            Self::CMSG_REALM_SPLIT(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.write_unencrypted_client(w),
            Self::CMSG_TIME_SYNC_RESP(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_DESCEND(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.write_unencrypted_client(w),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write_encrypted_client<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, w: &mut W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_CREATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_ENUM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_DELETE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGOUT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_REQUEST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_CANCEL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_NAME_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MESSAGECHAT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_JOIN_CHANNEL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LEAVE_CHANNEL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_LIST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_PASSWORD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_SET_OWNER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_OWNER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_MODERATOR(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_UNMODERATOR(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_MUTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_UNMUTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_INVITE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_KICK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_BAN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_UNBAN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_ANNOUNCEMENTS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_MODERATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AREATRIGGER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_FORWARD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_BACKWARD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_STRAFE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_JUMP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_TURN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_PITCH_UP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_PITCH(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_SWIM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_FACING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_FLAG(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_CLEAR(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_RESET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_EMOTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TEXT_EMOTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_FACTION_ATWAR(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTION_BUTTON(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_CHANNELLING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_SELECTION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSWING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSTOP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REPOP_REQUEST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_DUEL_ACCEPTED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_DUEL_CANCELLED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXI(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUG(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUERY_TIME(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SETSHEATHED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUTH_SESSION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ZONEUPDATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_GETTICKET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAT_IGNORED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_PLACE_BID(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_RENAME(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_PORT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_WATCHED_FACTION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_SET_FLY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REALM_SPLIT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TIME_SYNC_RESP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.tokio_write_encrypted_client(w, e).await,
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write_unencrypted_client<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_CREATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_ENUM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_DELETE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGOUT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_REQUEST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_CANCEL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_NAME_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MESSAGECHAT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_JOIN_CHANNEL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LEAVE_CHANNEL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_LIST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_PASSWORD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_SET_OWNER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_OWNER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_MODERATOR(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_UNMODERATOR(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_MUTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_UNMUTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_INVITE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_KICK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_BAN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_UNBAN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_ANNOUNCEMENTS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_MODERATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AREATRIGGER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_FORWARD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_BACKWARD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_STRAFE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_JUMP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_TURN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_PITCH_UP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_PITCH(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_SWIM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_FACING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_FLAG(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_CLEAR(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_RESET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_EMOTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TEXT_EMOTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_FACTION_ATWAR(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTION_BUTTON(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_CHANNELLING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_SELECTION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSWING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSTOP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REPOP_REQUEST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_DUEL_ACCEPTED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_DUEL_CANCELLED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXI(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUG(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUERY_TIME(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SETSHEATHED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUTH_SESSION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ZONEUPDATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_GETTICKET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAT_IGNORED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_PLACE_BID(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_RENAME(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_PORT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_WATCHED_FACTION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_SET_FLY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REALM_SPLIT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TIME_SYNC_RESP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.tokio_write_unencrypted_client(w).await,
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write_encrypted_client<W: async_std::io::WriteExt + Unpin + Send>(&self, w: &mut W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_CREATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_ENUM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_DELETE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGOUT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_REQUEST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_CANCEL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_NAME_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MESSAGECHAT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_JOIN_CHANNEL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LEAVE_CHANNEL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_LIST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_PASSWORD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_SET_OWNER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_OWNER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_MODERATOR(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_UNMODERATOR(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_MUTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_UNMUTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_INVITE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_KICK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_BAN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_UNBAN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_ANNOUNCEMENTS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_MODERATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AREATRIGGER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_FORWARD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_BACKWARD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_STRAFE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_JUMP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_TURN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_PITCH_UP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_PITCH(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_SWIM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_FACING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_FLAG(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_CLEAR(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_RESET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_EMOTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TEXT_EMOTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_FACTION_ATWAR(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTION_BUTTON(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_CHANNELLING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_SELECTION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSWING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSTOP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REPOP_REQUEST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_DUEL_ACCEPTED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_DUEL_CANCELLED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXI(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUG(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUERY_TIME(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SETSHEATHED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUTH_SESSION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ZONEUPDATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_GETTICKET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAT_IGNORED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_PLACE_BID(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_RENAME(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_PORT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_WATCHED_FACTION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_SET_FLY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REALM_SPLIT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TIME_SYNC_RESP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.astd_write_encrypted_client(w, e).await,
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write_unencrypted_client<W: async_std::io::WriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_CREATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_ENUM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_DELETE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGOUT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_REQUEST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_CANCEL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_NAME_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MESSAGECHAT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_JOIN_CHANNEL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LEAVE_CHANNEL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_LIST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_PASSWORD(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_SET_OWNER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_OWNER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_MODERATOR(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_UNMODERATOR(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_MUTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_UNMUTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_INVITE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_KICK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_BAN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_UNBAN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_ANNOUNCEMENTS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_MODERATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AREATRIGGER(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_FORWARD(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_BACKWARD(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_STRAFE(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_JUMP(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_TURN(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_PITCH_UP(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_PITCH(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_SWIM(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_FACING(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_FLAG(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_CLEAR(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_RESET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_EMOTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TEXT_EMOTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_FACTION_ATWAR(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTION_BUTTON(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_CHANNELLING(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_SELECTION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSWING(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSTOP(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REPOP_REQUEST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_DUEL_ACCEPTED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_DUEL_CANCELLED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXI(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUG(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUERY_TIME(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PING(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SETSHEATHED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUTH_SESSION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ZONEUPDATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_GETTICKET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAT_IGNORED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_PLACE_BID(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_RENAME(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_PORT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_WATCHED_FACTION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_SET_FLY(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REALM_SPLIT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TIME_SYNC_RESP(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.astd_write_unencrypted_client(w).await,
        }
    }

    pub fn movement_info(&self) -> Option<&MovementInfo> {
        match self {
            Self::MSG_MOVE_START_FORWARD(c) => Some(&c.info),
            Self::MSG_MOVE_START_BACKWARD(c) => Some(&c.info),
            Self::MSG_MOVE_STOP(c) => Some(&c.info),
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => Some(&c.info),
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => Some(&c.info),
            Self::MSG_MOVE_STOP_STRAFE(c) => Some(&c.info),
            Self::MSG_MOVE_JUMP(c) => Some(&c.info),
            Self::MSG_MOVE_START_TURN_LEFT(c) => Some(&c.info),
            Self::MSG_MOVE_START_TURN_RIGHT(c) => Some(&c.info),
            Self::MSG_MOVE_STOP_TURN(c) => Some(&c.info),
            Self::MSG_MOVE_START_PITCH_UP(c) => Some(&c.info),
            Self::MSG_MOVE_START_PITCH_DOWN(c) => Some(&c.info),
            Self::MSG_MOVE_STOP_PITCH(c) => Some(&c.info),
            Self::MSG_MOVE_SET_RUN_MODE(c) => Some(&c.info),
            Self::MSG_MOVE_SET_WALK_MODE(c) => Some(&c.info),
            Self::MSG_MOVE_FALL_LAND(c) => Some(&c.info),
            Self::MSG_MOVE_START_SWIM(c) => Some(&c.info),
            Self::MSG_MOVE_STOP_SWIM(c) => Some(&c.info),
            Self::MSG_MOVE_SET_FACING(c) => Some(&c.info),
            Self::MSG_MOVE_SET_PITCH(c) => Some(&c.info),
            Self::MSG_MOVE_HEARTBEAT(c) => Some(&c.info),
            Self::CMSG_MOVE_FALL_RESET(c) => Some(&c.info),
            Self::CMSG_MOVE_SET_FLY(c) => Some(&c.info),
            Self::MSG_MOVE_START_ASCEND(c) => Some(&c.info),
            Self::MSG_MOVE_STOP_ASCEND(c) => Some(&c.info),
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => Some(&c.info),
            Self::MSG_MOVE_START_DESCEND(c) => Some(&c.info),
            _ => None,
        }
    }

}

impl std::fmt::Display for ClientOpcodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ClientOpcodeMessage::MSG_MOVE_WORLDPORT_ACK(_) => "MSG_MOVE_WORLDPORT_ACK",
            ClientOpcodeMessage::CMSG_CHAR_CREATE(_) => "CMSG_CHAR_CREATE",
            ClientOpcodeMessage::CMSG_CHAR_ENUM(_) => "CMSG_CHAR_ENUM",
            ClientOpcodeMessage::CMSG_CHAR_DELETE(_) => "CMSG_CHAR_DELETE",
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(_) => "CMSG_PLAYER_LOGIN",
            ClientOpcodeMessage::CMSG_PLAYER_LOGOUT(_) => "CMSG_PLAYER_LOGOUT",
            ClientOpcodeMessage::CMSG_LOGOUT_REQUEST(_) => "CMSG_LOGOUT_REQUEST",
            ClientOpcodeMessage::CMSG_LOGOUT_CANCEL(_) => "CMSG_LOGOUT_CANCEL",
            ClientOpcodeMessage::CMSG_NAME_QUERY(_) => "CMSG_NAME_QUERY",
            ClientOpcodeMessage::CMSG_MESSAGECHAT(_) => "CMSG_MESSAGECHAT",
            ClientOpcodeMessage::CMSG_JOIN_CHANNEL(_) => "CMSG_JOIN_CHANNEL",
            ClientOpcodeMessage::CMSG_LEAVE_CHANNEL(_) => "CMSG_LEAVE_CHANNEL",
            ClientOpcodeMessage::CMSG_CHANNEL_LIST(_) => "CMSG_CHANNEL_LIST",
            ClientOpcodeMessage::CMSG_CHANNEL_PASSWORD(_) => "CMSG_CHANNEL_PASSWORD",
            ClientOpcodeMessage::CMSG_CHANNEL_SET_OWNER(_) => "CMSG_CHANNEL_SET_OWNER",
            ClientOpcodeMessage::CMSG_CHANNEL_OWNER(_) => "CMSG_CHANNEL_OWNER",
            ClientOpcodeMessage::CMSG_CHANNEL_MODERATOR(_) => "CMSG_CHANNEL_MODERATOR",
            ClientOpcodeMessage::CMSG_CHANNEL_UNMODERATOR(_) => "CMSG_CHANNEL_UNMODERATOR",
            ClientOpcodeMessage::CMSG_CHANNEL_MUTE(_) => "CMSG_CHANNEL_MUTE",
            ClientOpcodeMessage::CMSG_CHANNEL_UNMUTE(_) => "CMSG_CHANNEL_UNMUTE",
            ClientOpcodeMessage::CMSG_CHANNEL_INVITE(_) => "CMSG_CHANNEL_INVITE",
            ClientOpcodeMessage::CMSG_CHANNEL_KICK(_) => "CMSG_CHANNEL_KICK",
            ClientOpcodeMessage::CMSG_CHANNEL_BAN(_) => "CMSG_CHANNEL_BAN",
            ClientOpcodeMessage::CMSG_CHANNEL_UNBAN(_) => "CMSG_CHANNEL_UNBAN",
            ClientOpcodeMessage::CMSG_CHANNEL_ANNOUNCEMENTS(_) => "CMSG_CHANNEL_ANNOUNCEMENTS",
            ClientOpcodeMessage::CMSG_CHANNEL_MODERATE(_) => "CMSG_CHANNEL_MODERATE",
            ClientOpcodeMessage::CMSG_AREATRIGGER(_) => "CMSG_AREATRIGGER",
            ClientOpcodeMessage::MSG_MOVE_START_FORWARD(_) => "MSG_MOVE_START_FORWARD_Client",
            ClientOpcodeMessage::MSG_MOVE_START_BACKWARD(_) => "MSG_MOVE_START_BACKWARD_Client",
            ClientOpcodeMessage::MSG_MOVE_STOP(_) => "MSG_MOVE_STOP_Client",
            ClientOpcodeMessage::MSG_MOVE_START_STRAFE_LEFT(_) => "MSG_MOVE_START_STRAFE_LEFT_Client",
            ClientOpcodeMessage::MSG_MOVE_START_STRAFE_RIGHT(_) => "MSG_MOVE_START_STRAFE_RIGHT_Client",
            ClientOpcodeMessage::MSG_MOVE_STOP_STRAFE(_) => "MSG_MOVE_STOP_STRAFE_Client",
            ClientOpcodeMessage::MSG_MOVE_JUMP(_) => "MSG_MOVE_JUMP_Client",
            ClientOpcodeMessage::MSG_MOVE_START_TURN_LEFT(_) => "MSG_MOVE_START_TURN_LEFT_Client",
            ClientOpcodeMessage::MSG_MOVE_START_TURN_RIGHT(_) => "MSG_MOVE_START_TURN_RIGHT_Client",
            ClientOpcodeMessage::MSG_MOVE_STOP_TURN(_) => "MSG_MOVE_STOP_TURN_Client",
            ClientOpcodeMessage::MSG_MOVE_START_PITCH_UP(_) => "MSG_MOVE_START_PITCH_UP_Client",
            ClientOpcodeMessage::MSG_MOVE_START_PITCH_DOWN(_) => "MSG_MOVE_START_PITCH_DOWN_Client",
            ClientOpcodeMessage::MSG_MOVE_STOP_PITCH(_) => "MSG_MOVE_STOP_PITCH_Client",
            ClientOpcodeMessage::MSG_MOVE_SET_RUN_MODE(_) => "MSG_MOVE_SET_RUN_MODE_Client",
            ClientOpcodeMessage::MSG_MOVE_SET_WALK_MODE(_) => "MSG_MOVE_SET_WALK_MODE_Client",
            ClientOpcodeMessage::MSG_MOVE_TELEPORT_ACK(_) => "MSG_MOVE_TELEPORT_ACK_Client",
            ClientOpcodeMessage::MSG_MOVE_FALL_LAND(_) => "MSG_MOVE_FALL_LAND_Client",
            ClientOpcodeMessage::MSG_MOVE_START_SWIM(_) => "MSG_MOVE_START_SWIM_Client",
            ClientOpcodeMessage::MSG_MOVE_STOP_SWIM(_) => "MSG_MOVE_STOP_SWIM_Client",
            ClientOpcodeMessage::MSG_MOVE_SET_FACING(_) => "MSG_MOVE_SET_FACING_Client",
            ClientOpcodeMessage::MSG_MOVE_SET_PITCH(_) => "MSG_MOVE_SET_PITCH_Client",
            ClientOpcodeMessage::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_RUN_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_SWIM_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_MOVE_ROOT_ACK(_) => "CMSG_FORCE_MOVE_ROOT_ACK",
            ClientOpcodeMessage::CMSG_FORCE_MOVE_UNROOT_ACK(_) => "CMSG_FORCE_MOVE_UNROOT_ACK",
            ClientOpcodeMessage::MSG_MOVE_HEARTBEAT(_) => "MSG_MOVE_HEARTBEAT_Client",
            ClientOpcodeMessage::CMSG_NEXT_CINEMATIC_CAMERA(_) => "CMSG_NEXT_CINEMATIC_CAMERA",
            ClientOpcodeMessage::CMSG_COMPLETE_CINEMATIC(_) => "CMSG_COMPLETE_CINEMATIC",
            ClientOpcodeMessage::CMSG_TUTORIAL_FLAG(_) => "CMSG_TUTORIAL_FLAG",
            ClientOpcodeMessage::CMSG_TUTORIAL_CLEAR(_) => "CMSG_TUTORIAL_CLEAR",
            ClientOpcodeMessage::CMSG_TUTORIAL_RESET(_) => "CMSG_TUTORIAL_RESET",
            ClientOpcodeMessage::CMSG_EMOTE(_) => "CMSG_EMOTE",
            ClientOpcodeMessage::CMSG_TEXT_EMOTE(_) => "CMSG_TEXT_EMOTE",
            ClientOpcodeMessage::CMSG_SET_FACTION_ATWAR(_) => "CMSG_SET_FACTION_ATWAR",
            ClientOpcodeMessage::CMSG_SET_ACTION_BUTTON(_) => "CMSG_SET_ACTION_BUTTON",
            ClientOpcodeMessage::CMSG_CANCEL_CHANNELLING(_) => "CMSG_CANCEL_CHANNELLING",
            ClientOpcodeMessage::CMSG_SET_SELECTION(_) => "CMSG_SET_SELECTION",
            ClientOpcodeMessage::CMSG_SET_TARGET_OBSOLETE(_) => "CMSG_SET_TARGET_OBSOLETE",
            ClientOpcodeMessage::CMSG_ATTACKSWING(_) => "CMSG_ATTACKSWING",
            ClientOpcodeMessage::CMSG_ATTACKSTOP(_) => "CMSG_ATTACKSTOP",
            ClientOpcodeMessage::CMSG_REPOP_REQUEST(_) => "CMSG_REPOP_REQUEST",
            ClientOpcodeMessage::CMSG_DUEL_ACCEPTED(_) => "CMSG_DUEL_ACCEPTED",
            ClientOpcodeMessage::CMSG_DUEL_CANCELLED(_) => "CMSG_DUEL_CANCELLED",
            ClientOpcodeMessage::CMSG_ACTIVATETAXI(_) => "CMSG_ACTIVATETAXI",
            ClientOpcodeMessage::CMSG_BUG(_) => "CMSG_BUG",
            ClientOpcodeMessage::CMSG_QUERY_TIME(_) => "CMSG_QUERY_TIME",
            ClientOpcodeMessage::CMSG_PING(_) => "CMSG_PING",
            ClientOpcodeMessage::CMSG_SETSHEATHED(_) => "CMSG_SETSHEATHED",
            ClientOpcodeMessage::CMSG_AUTH_SESSION(_) => "CMSG_AUTH_SESSION",
            ClientOpcodeMessage::CMSG_ZONEUPDATE(_) => "CMSG_ZONEUPDATE",
            ClientOpcodeMessage::CMSG_GMTICKET_UPDATETEXT(_) => "CMSG_GMTICKET_UPDATETEXT",
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(_) => "CMSG_REQUEST_ACCOUNT_DATA",
            ClientOpcodeMessage::CMSG_GMTICKET_GETTICKET(_) => "CMSG_GMTICKET_GETTICKET",
            ClientOpcodeMessage::CMSG_GMTICKET_DELETETICKET(_) => "CMSG_GMTICKET_DELETETICKET",
            ClientOpcodeMessage::CMSG_GMTICKET_SYSTEMSTATUS(_) => "CMSG_GMTICKET_SYSTEMSTATUS",
            ClientOpcodeMessage::CMSG_CHAT_IGNORED(_) => "CMSG_CHAT_IGNORED",
            ClientOpcodeMessage::CMSG_BATTLEFIELD_LIST(_) => "CMSG_BATTLEFIELD_LIST",
            ClientOpcodeMessage::MSG_AUCTION_HELLO(_) => "MSG_AUCTION_HELLO_Client",
            ClientOpcodeMessage::CMSG_AUCTION_SELL_ITEM(_) => "CMSG_AUCTION_SELL_ITEM",
            ClientOpcodeMessage::CMSG_AUCTION_REMOVE_ITEM(_) => "CMSG_AUCTION_REMOVE_ITEM",
            ClientOpcodeMessage::CMSG_AUCTION_LIST_ITEMS(_) => "CMSG_AUCTION_LIST_ITEMS",
            ClientOpcodeMessage::CMSG_AUCTION_LIST_OWNER_ITEMS(_) => "CMSG_AUCTION_LIST_OWNER_ITEMS",
            ClientOpcodeMessage::CMSG_AUCTION_PLACE_BID(_) => "CMSG_AUCTION_PLACE_BID",
            ClientOpcodeMessage::CMSG_AUCTION_LIST_BIDDER_ITEMS(_) => "CMSG_AUCTION_LIST_BIDDER_ITEMS",
            ClientOpcodeMessage::CMSG_SET_ACTIVE_MOVER(_) => "CMSG_SET_ACTIVE_MOVER",
            ClientOpcodeMessage::CMSG_SET_ACTIONBAR_TOGGLES(_) => "CMSG_SET_ACTIONBAR_TOGGLES",
            ClientOpcodeMessage::CMSG_CHAR_RENAME(_) => "CMSG_CHAR_RENAME",
            ClientOpcodeMessage::CMSG_MOVE_FALL_RESET(_) => "CMSG_MOVE_FALL_RESET",
            ClientOpcodeMessage::CMSG_BATTLEFIELD_STATUS(_) => "CMSG_BATTLEFIELD_STATUS",
            ClientOpcodeMessage::CMSG_BATTLEFIELD_PORT(_) => "CMSG_BATTLEFIELD_PORT",
            ClientOpcodeMessage::CMSG_BATTLEMASTER_HELLO(_) => "CMSG_BATTLEMASTER_HELLO",
            ClientOpcodeMessage::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_WALK_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_TURN_RATE_CHANGE_ACK(_) => "CMSG_FORCE_TURN_RATE_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_LEAVE_BATTLEFIELD(_) => "CMSG_LEAVE_BATTLEFIELD",
            ClientOpcodeMessage::MSG_BATTLEGROUND_PLAYER_POSITIONS(_) => "MSG_BATTLEGROUND_PLAYER_POSITIONS_Client",
            ClientOpcodeMessage::CMSG_BATTLEMASTER_JOIN(_) => "CMSG_BATTLEMASTER_JOIN",
            ClientOpcodeMessage::CMSG_ACTIVATETAXIEXPRESS(_) => "CMSG_ACTIVATETAXIEXPRESS",
            ClientOpcodeMessage::CMSG_SET_FACTION_INACTIVE(_) => "CMSG_SET_FACTION_INACTIVE",
            ClientOpcodeMessage::CMSG_SET_WATCHED_FACTION(_) => "CMSG_SET_WATCHED_FACTION",
            ClientOpcodeMessage::CMSG_GMSURVEY_SUBMIT(_) => "CMSG_GMSURVEY_SUBMIT",
            ClientOpcodeMessage::CMSG_MOVE_SET_FLY(_) => "CMSG_MOVE_SET_FLY",
            ClientOpcodeMessage::MSG_MOVE_START_ASCEND(_) => "MSG_MOVE_START_ASCEND_Client",
            ClientOpcodeMessage::MSG_MOVE_STOP_ASCEND(_) => "MSG_MOVE_STOP_ASCEND_Client",
            ClientOpcodeMessage::CMSG_REALM_SPLIT(_) => "CMSG_REALM_SPLIT",
            ClientOpcodeMessage::CMSG_MOVE_CHNG_TRANSPORT(_) => "CMSG_MOVE_CHNG_TRANSPORT",
            ClientOpcodeMessage::CMSG_TIME_SYNC_RESP(_) => "CMSG_TIME_SYNC_RESP",
            ClientOpcodeMessage::MSG_MOVE_START_DESCEND(_) => "MSG_MOVE_START_DESCEND_Client",
            ClientOpcodeMessage::CMSG_SET_ACTIVE_VOICE_CHANNEL(_) => "CMSG_SET_ACTIVE_VOICE_CHANNEL",
        })
    }
}

impl From<MSG_MOVE_WORLDPORT_ACK> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_WORLDPORT_ACK) -> Self {
        Self::MSG_MOVE_WORLDPORT_ACK(c)
    }
}

impl From<CMSG_CHAR_CREATE> for ClientOpcodeMessage {
    fn from(c: CMSG_CHAR_CREATE) -> Self {
        Self::CMSG_CHAR_CREATE(c)
    }
}

impl From<CMSG_CHAR_ENUM> for ClientOpcodeMessage {
    fn from(c: CMSG_CHAR_ENUM) -> Self {
        Self::CMSG_CHAR_ENUM(c)
    }
}

impl From<CMSG_CHAR_DELETE> for ClientOpcodeMessage {
    fn from(c: CMSG_CHAR_DELETE) -> Self {
        Self::CMSG_CHAR_DELETE(c)
    }
}

impl From<CMSG_PLAYER_LOGIN> for ClientOpcodeMessage {
    fn from(c: CMSG_PLAYER_LOGIN) -> Self {
        Self::CMSG_PLAYER_LOGIN(c)
    }
}

impl From<CMSG_PLAYER_LOGOUT> for ClientOpcodeMessage {
    fn from(c: CMSG_PLAYER_LOGOUT) -> Self {
        Self::CMSG_PLAYER_LOGOUT(c)
    }
}

impl From<CMSG_LOGOUT_REQUEST> for ClientOpcodeMessage {
    fn from(c: CMSG_LOGOUT_REQUEST) -> Self {
        Self::CMSG_LOGOUT_REQUEST(c)
    }
}

impl From<CMSG_LOGOUT_CANCEL> for ClientOpcodeMessage {
    fn from(c: CMSG_LOGOUT_CANCEL) -> Self {
        Self::CMSG_LOGOUT_CANCEL(c)
    }
}

impl From<CMSG_NAME_QUERY> for ClientOpcodeMessage {
    fn from(c: CMSG_NAME_QUERY) -> Self {
        Self::CMSG_NAME_QUERY(c)
    }
}

impl From<CMSG_MESSAGECHAT> for ClientOpcodeMessage {
    fn from(c: CMSG_MESSAGECHAT) -> Self {
        Self::CMSG_MESSAGECHAT(c)
    }
}

impl From<CMSG_JOIN_CHANNEL> for ClientOpcodeMessage {
    fn from(c: CMSG_JOIN_CHANNEL) -> Self {
        Self::CMSG_JOIN_CHANNEL(c)
    }
}

impl From<CMSG_LEAVE_CHANNEL> for ClientOpcodeMessage {
    fn from(c: CMSG_LEAVE_CHANNEL) -> Self {
        Self::CMSG_LEAVE_CHANNEL(c)
    }
}

impl From<CMSG_CHANNEL_LIST> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_LIST) -> Self {
        Self::CMSG_CHANNEL_LIST(c)
    }
}

impl From<CMSG_CHANNEL_PASSWORD> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_PASSWORD) -> Self {
        Self::CMSG_CHANNEL_PASSWORD(c)
    }
}

impl From<CMSG_CHANNEL_SET_OWNER> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_SET_OWNER) -> Self {
        Self::CMSG_CHANNEL_SET_OWNER(c)
    }
}

impl From<CMSG_CHANNEL_OWNER> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_OWNER) -> Self {
        Self::CMSG_CHANNEL_OWNER(c)
    }
}

impl From<CMSG_CHANNEL_MODERATOR> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_MODERATOR) -> Self {
        Self::CMSG_CHANNEL_MODERATOR(c)
    }
}

impl From<CMSG_CHANNEL_UNMODERATOR> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_UNMODERATOR) -> Self {
        Self::CMSG_CHANNEL_UNMODERATOR(c)
    }
}

impl From<CMSG_CHANNEL_MUTE> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_MUTE) -> Self {
        Self::CMSG_CHANNEL_MUTE(c)
    }
}

impl From<CMSG_CHANNEL_UNMUTE> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_UNMUTE) -> Self {
        Self::CMSG_CHANNEL_UNMUTE(c)
    }
}

impl From<CMSG_CHANNEL_INVITE> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_INVITE) -> Self {
        Self::CMSG_CHANNEL_INVITE(c)
    }
}

impl From<CMSG_CHANNEL_KICK> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_KICK) -> Self {
        Self::CMSG_CHANNEL_KICK(c)
    }
}

impl From<CMSG_CHANNEL_BAN> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_BAN) -> Self {
        Self::CMSG_CHANNEL_BAN(c)
    }
}

impl From<CMSG_CHANNEL_UNBAN> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_UNBAN) -> Self {
        Self::CMSG_CHANNEL_UNBAN(c)
    }
}

impl From<CMSG_CHANNEL_ANNOUNCEMENTS> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_ANNOUNCEMENTS) -> Self {
        Self::CMSG_CHANNEL_ANNOUNCEMENTS(c)
    }
}

impl From<CMSG_CHANNEL_MODERATE> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_MODERATE) -> Self {
        Self::CMSG_CHANNEL_MODERATE(c)
    }
}

impl From<CMSG_AREATRIGGER> for ClientOpcodeMessage {
    fn from(c: CMSG_AREATRIGGER) -> Self {
        Self::CMSG_AREATRIGGER(c)
    }
}

impl From<MSG_MOVE_START_FORWARD_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_FORWARD_Client) -> Self {
        Self::MSG_MOVE_START_FORWARD(c)
    }
}

impl From<MSG_MOVE_START_BACKWARD_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_BACKWARD_Client) -> Self {
        Self::MSG_MOVE_START_BACKWARD(c)
    }
}

impl From<MSG_MOVE_STOP_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_Client) -> Self {
        Self::MSG_MOVE_STOP(c)
    }
}

impl From<MSG_MOVE_START_STRAFE_LEFT_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_STRAFE_LEFT_Client) -> Self {
        Self::MSG_MOVE_START_STRAFE_LEFT(c)
    }
}

impl From<MSG_MOVE_START_STRAFE_RIGHT_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_STRAFE_RIGHT_Client) -> Self {
        Self::MSG_MOVE_START_STRAFE_RIGHT(c)
    }
}

impl From<MSG_MOVE_STOP_STRAFE_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_STRAFE_Client) -> Self {
        Self::MSG_MOVE_STOP_STRAFE(c)
    }
}

impl From<MSG_MOVE_JUMP_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_JUMP_Client) -> Self {
        Self::MSG_MOVE_JUMP(c)
    }
}

impl From<MSG_MOVE_START_TURN_LEFT_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_TURN_LEFT_Client) -> Self {
        Self::MSG_MOVE_START_TURN_LEFT(c)
    }
}

impl From<MSG_MOVE_START_TURN_RIGHT_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_TURN_RIGHT_Client) -> Self {
        Self::MSG_MOVE_START_TURN_RIGHT(c)
    }
}

impl From<MSG_MOVE_STOP_TURN_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_TURN_Client) -> Self {
        Self::MSG_MOVE_STOP_TURN(c)
    }
}

impl From<MSG_MOVE_START_PITCH_UP_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_PITCH_UP_Client) -> Self {
        Self::MSG_MOVE_START_PITCH_UP(c)
    }
}

impl From<MSG_MOVE_START_PITCH_DOWN_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_PITCH_DOWN_Client) -> Self {
        Self::MSG_MOVE_START_PITCH_DOWN(c)
    }
}

impl From<MSG_MOVE_STOP_PITCH_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_PITCH_Client) -> Self {
        Self::MSG_MOVE_STOP_PITCH(c)
    }
}

impl From<MSG_MOVE_SET_RUN_MODE_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_SET_RUN_MODE_Client) -> Self {
        Self::MSG_MOVE_SET_RUN_MODE(c)
    }
}

impl From<MSG_MOVE_SET_WALK_MODE_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_SET_WALK_MODE_Client) -> Self {
        Self::MSG_MOVE_SET_WALK_MODE(c)
    }
}

impl From<MSG_MOVE_TELEPORT_ACK_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_TELEPORT_ACK_Client) -> Self {
        Self::MSG_MOVE_TELEPORT_ACK(c)
    }
}

impl From<MSG_MOVE_FALL_LAND_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_FALL_LAND_Client) -> Self {
        Self::MSG_MOVE_FALL_LAND(c)
    }
}

impl From<MSG_MOVE_START_SWIM_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_SWIM_Client) -> Self {
        Self::MSG_MOVE_START_SWIM(c)
    }
}

impl From<MSG_MOVE_STOP_SWIM_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_SWIM_Client) -> Self {
        Self::MSG_MOVE_STOP_SWIM(c)
    }
}

impl From<MSG_MOVE_SET_FACING_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_SET_FACING_Client) -> Self {
        Self::MSG_MOVE_SET_FACING(c)
    }
}

impl From<MSG_MOVE_SET_PITCH_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_SET_PITCH_Client) -> Self {
        Self::MSG_MOVE_SET_PITCH(c)
    }
}

impl From<CMSG_FORCE_RUN_SPEED_CHANGE_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_FORCE_RUN_SPEED_CHANGE_ACK) -> Self {
        Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c)
    }
}

impl From<CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK) -> Self {
        Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c)
    }
}

impl From<CMSG_FORCE_SWIM_SPEED_CHANGE_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_FORCE_SWIM_SPEED_CHANGE_ACK) -> Self {
        Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c)
    }
}

impl From<CMSG_FORCE_MOVE_ROOT_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_FORCE_MOVE_ROOT_ACK) -> Self {
        Self::CMSG_FORCE_MOVE_ROOT_ACK(c)
    }
}

impl From<CMSG_FORCE_MOVE_UNROOT_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_FORCE_MOVE_UNROOT_ACK) -> Self {
        Self::CMSG_FORCE_MOVE_UNROOT_ACK(c)
    }
}

impl From<MSG_MOVE_HEARTBEAT_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_HEARTBEAT_Client) -> Self {
        Self::MSG_MOVE_HEARTBEAT(c)
    }
}

impl From<CMSG_NEXT_CINEMATIC_CAMERA> for ClientOpcodeMessage {
    fn from(c: CMSG_NEXT_CINEMATIC_CAMERA) -> Self {
        Self::CMSG_NEXT_CINEMATIC_CAMERA(c)
    }
}

impl From<CMSG_COMPLETE_CINEMATIC> for ClientOpcodeMessage {
    fn from(c: CMSG_COMPLETE_CINEMATIC) -> Self {
        Self::CMSG_COMPLETE_CINEMATIC(c)
    }
}

impl From<CMSG_TUTORIAL_FLAG> for ClientOpcodeMessage {
    fn from(c: CMSG_TUTORIAL_FLAG) -> Self {
        Self::CMSG_TUTORIAL_FLAG(c)
    }
}

impl From<CMSG_TUTORIAL_CLEAR> for ClientOpcodeMessage {
    fn from(c: CMSG_TUTORIAL_CLEAR) -> Self {
        Self::CMSG_TUTORIAL_CLEAR(c)
    }
}

impl From<CMSG_TUTORIAL_RESET> for ClientOpcodeMessage {
    fn from(c: CMSG_TUTORIAL_RESET) -> Self {
        Self::CMSG_TUTORIAL_RESET(c)
    }
}

impl From<CMSG_EMOTE> for ClientOpcodeMessage {
    fn from(c: CMSG_EMOTE) -> Self {
        Self::CMSG_EMOTE(c)
    }
}

impl From<CMSG_TEXT_EMOTE> for ClientOpcodeMessage {
    fn from(c: CMSG_TEXT_EMOTE) -> Self {
        Self::CMSG_TEXT_EMOTE(c)
    }
}

impl From<CMSG_SET_FACTION_ATWAR> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_FACTION_ATWAR) -> Self {
        Self::CMSG_SET_FACTION_ATWAR(c)
    }
}

impl From<CMSG_SET_ACTION_BUTTON> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_ACTION_BUTTON) -> Self {
        Self::CMSG_SET_ACTION_BUTTON(c)
    }
}

impl From<CMSG_CANCEL_CHANNELLING> for ClientOpcodeMessage {
    fn from(c: CMSG_CANCEL_CHANNELLING) -> Self {
        Self::CMSG_CANCEL_CHANNELLING(c)
    }
}

impl From<CMSG_SET_SELECTION> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_SELECTION) -> Self {
        Self::CMSG_SET_SELECTION(c)
    }
}

impl From<CMSG_SET_TARGET_OBSOLETE> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_TARGET_OBSOLETE) -> Self {
        Self::CMSG_SET_TARGET_OBSOLETE(c)
    }
}

impl From<CMSG_ATTACKSWING> for ClientOpcodeMessage {
    fn from(c: CMSG_ATTACKSWING) -> Self {
        Self::CMSG_ATTACKSWING(c)
    }
}

impl From<CMSG_ATTACKSTOP> for ClientOpcodeMessage {
    fn from(c: CMSG_ATTACKSTOP) -> Self {
        Self::CMSG_ATTACKSTOP(c)
    }
}

impl From<CMSG_REPOP_REQUEST> for ClientOpcodeMessage {
    fn from(c: CMSG_REPOP_REQUEST) -> Self {
        Self::CMSG_REPOP_REQUEST(c)
    }
}

impl From<CMSG_DUEL_ACCEPTED> for ClientOpcodeMessage {
    fn from(c: CMSG_DUEL_ACCEPTED) -> Self {
        Self::CMSG_DUEL_ACCEPTED(c)
    }
}

impl From<CMSG_DUEL_CANCELLED> for ClientOpcodeMessage {
    fn from(c: CMSG_DUEL_CANCELLED) -> Self {
        Self::CMSG_DUEL_CANCELLED(c)
    }
}

impl From<CMSG_ACTIVATETAXI> for ClientOpcodeMessage {
    fn from(c: CMSG_ACTIVATETAXI) -> Self {
        Self::CMSG_ACTIVATETAXI(c)
    }
}

impl From<CMSG_BUG> for ClientOpcodeMessage {
    fn from(c: CMSG_BUG) -> Self {
        Self::CMSG_BUG(c)
    }
}

impl From<CMSG_QUERY_TIME> for ClientOpcodeMessage {
    fn from(c: CMSG_QUERY_TIME) -> Self {
        Self::CMSG_QUERY_TIME(c)
    }
}

impl From<CMSG_PING> for ClientOpcodeMessage {
    fn from(c: CMSG_PING) -> Self {
        Self::CMSG_PING(c)
    }
}

impl From<CMSG_SETSHEATHED> for ClientOpcodeMessage {
    fn from(c: CMSG_SETSHEATHED) -> Self {
        Self::CMSG_SETSHEATHED(c)
    }
}

impl From<CMSG_AUTH_SESSION> for ClientOpcodeMessage {
    fn from(c: CMSG_AUTH_SESSION) -> Self {
        Self::CMSG_AUTH_SESSION(c)
    }
}

impl From<CMSG_ZONEUPDATE> for ClientOpcodeMessage {
    fn from(c: CMSG_ZONEUPDATE) -> Self {
        Self::CMSG_ZONEUPDATE(c)
    }
}

impl From<CMSG_GMTICKET_UPDATETEXT> for ClientOpcodeMessage {
    fn from(c: CMSG_GMTICKET_UPDATETEXT) -> Self {
        Self::CMSG_GMTICKET_UPDATETEXT(c)
    }
}

impl From<CMSG_REQUEST_ACCOUNT_DATA> for ClientOpcodeMessage {
    fn from(c: CMSG_REQUEST_ACCOUNT_DATA) -> Self {
        Self::CMSG_REQUEST_ACCOUNT_DATA(c)
    }
}

impl From<CMSG_GMTICKET_GETTICKET> for ClientOpcodeMessage {
    fn from(c: CMSG_GMTICKET_GETTICKET) -> Self {
        Self::CMSG_GMTICKET_GETTICKET(c)
    }
}

impl From<CMSG_GMTICKET_DELETETICKET> for ClientOpcodeMessage {
    fn from(c: CMSG_GMTICKET_DELETETICKET) -> Self {
        Self::CMSG_GMTICKET_DELETETICKET(c)
    }
}

impl From<CMSG_GMTICKET_SYSTEMSTATUS> for ClientOpcodeMessage {
    fn from(c: CMSG_GMTICKET_SYSTEMSTATUS) -> Self {
        Self::CMSG_GMTICKET_SYSTEMSTATUS(c)
    }
}

impl From<CMSG_CHAT_IGNORED> for ClientOpcodeMessage {
    fn from(c: CMSG_CHAT_IGNORED) -> Self {
        Self::CMSG_CHAT_IGNORED(c)
    }
}

impl From<CMSG_BATTLEFIELD_LIST> for ClientOpcodeMessage {
    fn from(c: CMSG_BATTLEFIELD_LIST) -> Self {
        Self::CMSG_BATTLEFIELD_LIST(c)
    }
}

impl From<MSG_AUCTION_HELLO_Client> for ClientOpcodeMessage {
    fn from(c: MSG_AUCTION_HELLO_Client) -> Self {
        Self::MSG_AUCTION_HELLO(c)
    }
}

impl From<CMSG_AUCTION_SELL_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_AUCTION_SELL_ITEM) -> Self {
        Self::CMSG_AUCTION_SELL_ITEM(c)
    }
}

impl From<CMSG_AUCTION_REMOVE_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_AUCTION_REMOVE_ITEM) -> Self {
        Self::CMSG_AUCTION_REMOVE_ITEM(c)
    }
}

impl From<CMSG_AUCTION_LIST_ITEMS> for ClientOpcodeMessage {
    fn from(c: CMSG_AUCTION_LIST_ITEMS) -> Self {
        Self::CMSG_AUCTION_LIST_ITEMS(c)
    }
}

impl From<CMSG_AUCTION_LIST_OWNER_ITEMS> for ClientOpcodeMessage {
    fn from(c: CMSG_AUCTION_LIST_OWNER_ITEMS) -> Self {
        Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c)
    }
}

impl From<CMSG_AUCTION_PLACE_BID> for ClientOpcodeMessage {
    fn from(c: CMSG_AUCTION_PLACE_BID) -> Self {
        Self::CMSG_AUCTION_PLACE_BID(c)
    }
}

impl From<CMSG_AUCTION_LIST_BIDDER_ITEMS> for ClientOpcodeMessage {
    fn from(c: CMSG_AUCTION_LIST_BIDDER_ITEMS) -> Self {
        Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c)
    }
}

impl From<CMSG_SET_ACTIVE_MOVER> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_ACTIVE_MOVER) -> Self {
        Self::CMSG_SET_ACTIVE_MOVER(c)
    }
}

impl From<CMSG_SET_ACTIONBAR_TOGGLES> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_ACTIONBAR_TOGGLES) -> Self {
        Self::CMSG_SET_ACTIONBAR_TOGGLES(c)
    }
}

impl From<CMSG_CHAR_RENAME> for ClientOpcodeMessage {
    fn from(c: CMSG_CHAR_RENAME) -> Self {
        Self::CMSG_CHAR_RENAME(c)
    }
}

impl From<CMSG_MOVE_FALL_RESET> for ClientOpcodeMessage {
    fn from(c: CMSG_MOVE_FALL_RESET) -> Self {
        Self::CMSG_MOVE_FALL_RESET(c)
    }
}

impl From<CMSG_BATTLEFIELD_STATUS> for ClientOpcodeMessage {
    fn from(c: CMSG_BATTLEFIELD_STATUS) -> Self {
        Self::CMSG_BATTLEFIELD_STATUS(c)
    }
}

impl From<CMSG_BATTLEFIELD_PORT> for ClientOpcodeMessage {
    fn from(c: CMSG_BATTLEFIELD_PORT) -> Self {
        Self::CMSG_BATTLEFIELD_PORT(c)
    }
}

impl From<CMSG_BATTLEMASTER_HELLO> for ClientOpcodeMessage {
    fn from(c: CMSG_BATTLEMASTER_HELLO) -> Self {
        Self::CMSG_BATTLEMASTER_HELLO(c)
    }
}

impl From<CMSG_FORCE_WALK_SPEED_CHANGE_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_FORCE_WALK_SPEED_CHANGE_ACK) -> Self {
        Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c)
    }
}

impl From<CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK) -> Self {
        Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c)
    }
}

impl From<CMSG_FORCE_TURN_RATE_CHANGE_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_FORCE_TURN_RATE_CHANGE_ACK) -> Self {
        Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c)
    }
}

impl From<CMSG_LEAVE_BATTLEFIELD> for ClientOpcodeMessage {
    fn from(c: CMSG_LEAVE_BATTLEFIELD) -> Self {
        Self::CMSG_LEAVE_BATTLEFIELD(c)
    }
}

impl From<MSG_BATTLEGROUND_PLAYER_POSITIONS_Client> for ClientOpcodeMessage {
    fn from(c: MSG_BATTLEGROUND_PLAYER_POSITIONS_Client) -> Self {
        Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c)
    }
}

impl From<CMSG_BATTLEMASTER_JOIN> for ClientOpcodeMessage {
    fn from(c: CMSG_BATTLEMASTER_JOIN) -> Self {
        Self::CMSG_BATTLEMASTER_JOIN(c)
    }
}

impl From<CMSG_ACTIVATETAXIEXPRESS> for ClientOpcodeMessage {
    fn from(c: CMSG_ACTIVATETAXIEXPRESS) -> Self {
        Self::CMSG_ACTIVATETAXIEXPRESS(c)
    }
}

impl From<CMSG_SET_FACTION_INACTIVE> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_FACTION_INACTIVE) -> Self {
        Self::CMSG_SET_FACTION_INACTIVE(c)
    }
}

impl From<CMSG_SET_WATCHED_FACTION> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_WATCHED_FACTION) -> Self {
        Self::CMSG_SET_WATCHED_FACTION(c)
    }
}

impl From<CMSG_GMSURVEY_SUBMIT> for ClientOpcodeMessage {
    fn from(c: CMSG_GMSURVEY_SUBMIT) -> Self {
        Self::CMSG_GMSURVEY_SUBMIT(c)
    }
}

impl From<CMSG_MOVE_SET_FLY> for ClientOpcodeMessage {
    fn from(c: CMSG_MOVE_SET_FLY) -> Self {
        Self::CMSG_MOVE_SET_FLY(c)
    }
}

impl From<MSG_MOVE_START_ASCEND_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_ASCEND_Client) -> Self {
        Self::MSG_MOVE_START_ASCEND(c)
    }
}

impl From<MSG_MOVE_STOP_ASCEND_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_ASCEND_Client) -> Self {
        Self::MSG_MOVE_STOP_ASCEND(c)
    }
}

impl From<CMSG_REALM_SPLIT> for ClientOpcodeMessage {
    fn from(c: CMSG_REALM_SPLIT) -> Self {
        Self::CMSG_REALM_SPLIT(c)
    }
}

impl From<CMSG_MOVE_CHNG_TRANSPORT> for ClientOpcodeMessage {
    fn from(c: CMSG_MOVE_CHNG_TRANSPORT) -> Self {
        Self::CMSG_MOVE_CHNG_TRANSPORT(c)
    }
}

impl From<CMSG_TIME_SYNC_RESP> for ClientOpcodeMessage {
    fn from(c: CMSG_TIME_SYNC_RESP) -> Self {
        Self::CMSG_TIME_SYNC_RESP(c)
    }
}

impl From<MSG_MOVE_START_DESCEND_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_DESCEND_Client) -> Self {
        Self::MSG_MOVE_START_DESCEND(c)
    }
}

impl From<CMSG_SET_ACTIVE_VOICE_CHANNEL> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_ACTIVE_VOICE_CHANNEL) -> Self {
        Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c)
    }
}

use crate::world::tbc::SMSG_CHAR_CREATE;
use crate::world::tbc::SMSG_CHAR_ENUM;
use crate::world::tbc::SMSG_CHAR_DELETE;
use crate::world::tbc::SMSG_NEW_WORLD;
use crate::world::tbc::SMSG_TRANSFER_PENDING;
use crate::world::tbc::SMSG_CHARACTER_LOGIN_FAILED;
use crate::world::tbc::SMSG_LOGIN_SETTIMESPEED;
use crate::world::tbc::SMSG_LOGOUT_COMPLETE;
use crate::world::tbc::SMSG_LOGOUT_CANCEL_ACK;
use crate::world::tbc::SMSG_CHANNEL_NOTIFY;
use crate::world::tbc::SMSG_CHANNEL_LIST;
use crate::world::tbc::SMSG_UPDATE_OBJECT;
use crate::world::tbc::SMSG_DESTROY_OBJECT;
use crate::world::tbc::MSG_MOVE_START_FORWARD_Server;
use crate::world::tbc::MSG_MOVE_START_BACKWARD_Server;
use crate::world::tbc::MSG_MOVE_STOP_Server;
use crate::world::tbc::MSG_MOVE_START_STRAFE_LEFT_Server;
use crate::world::tbc::MSG_MOVE_START_STRAFE_RIGHT_Server;
use crate::world::tbc::MSG_MOVE_STOP_STRAFE_Server;
use crate::world::tbc::MSG_MOVE_JUMP_Server;
use crate::world::tbc::MSG_MOVE_START_TURN_LEFT_Server;
use crate::world::tbc::MSG_MOVE_START_TURN_RIGHT_Server;
use crate::world::tbc::MSG_MOVE_STOP_TURN_Server;
use crate::world::tbc::MSG_MOVE_START_PITCH_UP_Server;
use crate::world::tbc::MSG_MOVE_START_PITCH_DOWN_Server;
use crate::world::tbc::MSG_MOVE_STOP_PITCH_Server;
use crate::world::tbc::MSG_MOVE_SET_RUN_MODE_Server;
use crate::world::tbc::MSG_MOVE_SET_WALK_MODE_Server;
use crate::world::tbc::MSG_MOVE_TELEPORT_ACK_Server;
use crate::world::tbc::MSG_MOVE_FALL_LAND_Server;
use crate::world::tbc::MSG_MOVE_START_SWIM_Server;
use crate::world::tbc::MSG_MOVE_STOP_SWIM_Server;
use crate::world::tbc::MSG_MOVE_SET_FACING_Server;
use crate::world::tbc::MSG_MOVE_SET_PITCH_Server;
use crate::world::tbc::MSG_MOVE_HEARTBEAT_Server;
use crate::world::tbc::SMSG_TRIGGER_CINEMATIC;
use crate::world::tbc::SMSG_TUTORIAL_FLAGS;
use crate::world::tbc::SMSG_EMOTE;
use crate::world::tbc::SMSG_TEXT_EMOTE;
use crate::world::tbc::SMSG_INITIALIZE_FACTIONS;
use crate::world::tbc::SMSG_SET_FACTION_VISIBLE;
use crate::world::tbc::SMSG_SET_FACTION_STANDING;
use crate::world::tbc::SMSG_ACTION_BUTTONS;
use crate::world::tbc::SMSG_AI_REACTION;
use crate::world::tbc::SMSG_ATTACKSTART;
use crate::world::tbc::SMSG_ATTACKSTOP;
use crate::world::tbc::SMSG_ATTACKSWING_NOTINRANGE;
use crate::world::tbc::SMSG_ATTACKSWING_BADFACING;
use crate::world::tbc::SMSG_ATTACKSWING_NOTSTANDING;
use crate::world::tbc::SMSG_ATTACKSWING_DEADTARGET;
use crate::world::tbc::SMSG_ATTACKSWING_CANT_ATTACK;
use crate::world::tbc::SMSG_CANCEL_COMBAT;
use crate::world::tbc::SMSG_DUEL_REQUESTED;
use crate::world::tbc::SMSG_DUEL_OUTOFBOUNDS;
use crate::world::tbc::SMSG_DUEL_INBOUNDS;
use crate::world::tbc::SMSG_DUEL_COMPLETE;
use crate::world::tbc::SMSG_DUEL_WINNER;
use crate::world::tbc::SMSG_NOTIFICATION;
use crate::world::tbc::SMSG_LOG_XPGAIN;
use crate::world::tbc::SMSG_LEVELUP_INFO;
use crate::world::tbc::SMSG_PONG;
use crate::world::tbc::SMSG_AUTH_CHALLENGE;
use crate::world::tbc::SMSG_AUTH_RESPONSE;
use crate::world::tbc::SMSG_EXPLORATION_EXPERIENCE;
use crate::world::tbc::SMSG_ENVIRONMENTALDAMAGELOG;
use crate::world::tbc::SMSG_GMTICKET_CREATE;
use crate::world::tbc::SMSG_ACCOUNT_DATA_TIMES;
use crate::world::tbc::SMSG_CHAT_WRONG_FACTION;
use crate::world::tbc::SMSG_SET_REST_START;
use crate::world::tbc::SMSG_LOGIN_VERIFY_WORLD;
use crate::world::tbc::SMSG_BATTLEFIELD_LIST;
use crate::world::tbc::SMSG_ZONE_UNDER_ATTACK;
use crate::world::tbc::MSG_AUCTION_HELLO_Server;
use crate::world::tbc::SMSG_AUCTION_LIST_RESULT;
use crate::world::tbc::SMSG_AUCTION_OWNER_LIST_RESULT;
use crate::world::tbc::SMSG_AUCTION_BIDDER_NOTIFICATION;
use crate::world::tbc::SMSG_AUCTION_OWNER_NOTIFICATION;
use crate::world::tbc::SMSG_PROCRESIST;
use crate::world::tbc::SMSG_AUCTION_BIDDER_LIST_RESULT;
use crate::world::tbc::SMSG_AUCTION_REMOVED_NOTIFICATION;
use crate::world::tbc::SMSG_SERVER_MESSAGE;
use crate::world::tbc::SMSG_STANDSTATE_UPDATE;
use crate::world::tbc::SMSG_SET_FORCED_REACTIONS;
use crate::world::tbc::SMSG_CHAT_PLAYER_NOT_FOUND;
use crate::world::tbc::SMSG_DUEL_COUNTDOWN;
use crate::world::tbc::SMSG_DURABILITY_DAMAGE_DEATH;
use crate::world::tbc::SMSG_UPDATE_WORLD_STATE;
use crate::world::tbc::SMSG_CHAR_RENAME;
use crate::world::tbc::MSG_BATTLEGROUND_PLAYER_POSITIONS_Server;
use crate::world::tbc::SMSG_BATTLEGROUND_PLAYER_JOINED;
use crate::world::tbc::SMSG_BATTLEGROUND_PLAYER_LEFT;
use crate::world::tbc::SMSG_CHAT_RESTRICTED;
use crate::world::tbc::SMSG_GM_TICKET_STATUS_UPDATE;
use crate::world::tbc::SMSG_DEFENSE_MESSAGE;
use crate::world::tbc::MSG_MOVE_START_ASCEND_Server;
use crate::world::tbc::MSG_MOVE_STOP_ASCEND_Server;
use crate::world::tbc::SMSG_REALM_SPLIT;
use crate::world::tbc::SMSG_TIME_SYNC_REQ;
use crate::world::tbc::MSG_MOVE_START_DESCEND_Server;

#[derive(Debug, Clone, PartialEq)]
pub enum ServerOpcodeMessage {
    MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK),
    SMSG_CHAR_CREATE(SMSG_CHAR_CREATE),
    SMSG_CHAR_ENUM(SMSG_CHAR_ENUM),
    SMSG_CHAR_DELETE(SMSG_CHAR_DELETE),
    SMSG_NEW_WORLD(SMSG_NEW_WORLD),
    SMSG_TRANSFER_PENDING(SMSG_TRANSFER_PENDING),
    SMSG_CHARACTER_LOGIN_FAILED(SMSG_CHARACTER_LOGIN_FAILED),
    SMSG_LOGIN_SETTIMESPEED(SMSG_LOGIN_SETTIMESPEED),
    SMSG_LOGOUT_COMPLETE(SMSG_LOGOUT_COMPLETE),
    SMSG_LOGOUT_CANCEL_ACK(SMSG_LOGOUT_CANCEL_ACK),
    SMSG_CHANNEL_NOTIFY(SMSG_CHANNEL_NOTIFY),
    SMSG_CHANNEL_LIST(SMSG_CHANNEL_LIST),
    SMSG_UPDATE_OBJECT(SMSG_UPDATE_OBJECT),
    SMSG_DESTROY_OBJECT(SMSG_DESTROY_OBJECT),
    MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD_Server),
    MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD_Server),
    MSG_MOVE_STOP(MSG_MOVE_STOP_Server),
    MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT_Server),
    MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT_Server),
    MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE_Server),
    MSG_MOVE_JUMP(MSG_MOVE_JUMP_Server),
    MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT_Server),
    MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT_Server),
    MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN_Server),
    MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP_Server),
    MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN_Server),
    MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH_Server),
    MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE_Server),
    MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE_Server),
    MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK_Server),
    MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND_Server),
    MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM_Server),
    MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM_Server),
    MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING_Server),
    MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH_Server),
    MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT_Server),
    SMSG_TRIGGER_CINEMATIC(SMSG_TRIGGER_CINEMATIC),
    SMSG_TUTORIAL_FLAGS(SMSG_TUTORIAL_FLAGS),
    SMSG_EMOTE(SMSG_EMOTE),
    SMSG_TEXT_EMOTE(SMSG_TEXT_EMOTE),
    SMSG_INITIALIZE_FACTIONS(SMSG_INITIALIZE_FACTIONS),
    SMSG_SET_FACTION_VISIBLE(SMSG_SET_FACTION_VISIBLE),
    SMSG_SET_FACTION_STANDING(SMSG_SET_FACTION_STANDING),
    SMSG_ACTION_BUTTONS(SMSG_ACTION_BUTTONS),
    SMSG_AI_REACTION(SMSG_AI_REACTION),
    SMSG_ATTACKSTART(SMSG_ATTACKSTART),
    SMSG_ATTACKSTOP(SMSG_ATTACKSTOP),
    SMSG_ATTACKSWING_NOTINRANGE(SMSG_ATTACKSWING_NOTINRANGE),
    SMSG_ATTACKSWING_BADFACING(SMSG_ATTACKSWING_BADFACING),
    SMSG_ATTACKSWING_NOTSTANDING(SMSG_ATTACKSWING_NOTSTANDING),
    SMSG_ATTACKSWING_DEADTARGET(SMSG_ATTACKSWING_DEADTARGET),
    SMSG_ATTACKSWING_CANT_ATTACK(SMSG_ATTACKSWING_CANT_ATTACK),
    SMSG_CANCEL_COMBAT(SMSG_CANCEL_COMBAT),
    SMSG_DUEL_REQUESTED(SMSG_DUEL_REQUESTED),
    SMSG_DUEL_OUTOFBOUNDS(SMSG_DUEL_OUTOFBOUNDS),
    SMSG_DUEL_INBOUNDS(SMSG_DUEL_INBOUNDS),
    SMSG_DUEL_COMPLETE(SMSG_DUEL_COMPLETE),
    SMSG_DUEL_WINNER(SMSG_DUEL_WINNER),
    SMSG_NOTIFICATION(SMSG_NOTIFICATION),
    SMSG_LOG_XPGAIN(SMSG_LOG_XPGAIN),
    SMSG_LEVELUP_INFO(SMSG_LEVELUP_INFO),
    SMSG_PONG(SMSG_PONG),
    SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE),
    SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE),
    SMSG_EXPLORATION_EXPERIENCE(SMSG_EXPLORATION_EXPERIENCE),
    SMSG_ENVIRONMENTALDAMAGELOG(SMSG_ENVIRONMENTALDAMAGELOG),
    SMSG_GMTICKET_CREATE(SMSG_GMTICKET_CREATE),
    SMSG_ACCOUNT_DATA_TIMES(SMSG_ACCOUNT_DATA_TIMES),
    SMSG_CHAT_WRONG_FACTION(SMSG_CHAT_WRONG_FACTION),
    SMSG_SET_REST_START(SMSG_SET_REST_START),
    SMSG_LOGIN_VERIFY_WORLD(SMSG_LOGIN_VERIFY_WORLD),
    SMSG_BATTLEFIELD_LIST(SMSG_BATTLEFIELD_LIST),
    SMSG_ZONE_UNDER_ATTACK(SMSG_ZONE_UNDER_ATTACK),
    MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Server),
    SMSG_AUCTION_LIST_RESULT(SMSG_AUCTION_LIST_RESULT),
    SMSG_AUCTION_OWNER_LIST_RESULT(SMSG_AUCTION_OWNER_LIST_RESULT),
    SMSG_AUCTION_BIDDER_NOTIFICATION(SMSG_AUCTION_BIDDER_NOTIFICATION),
    SMSG_AUCTION_OWNER_NOTIFICATION(SMSG_AUCTION_OWNER_NOTIFICATION),
    SMSG_PROCRESIST(SMSG_PROCRESIST),
    SMSG_AUCTION_BIDDER_LIST_RESULT(SMSG_AUCTION_BIDDER_LIST_RESULT),
    SMSG_AUCTION_REMOVED_NOTIFICATION(SMSG_AUCTION_REMOVED_NOTIFICATION),
    SMSG_SERVER_MESSAGE(SMSG_SERVER_MESSAGE),
    SMSG_STANDSTATE_UPDATE(SMSG_STANDSTATE_UPDATE),
    SMSG_SET_FORCED_REACTIONS(SMSG_SET_FORCED_REACTIONS),
    SMSG_CHAT_PLAYER_NOT_FOUND(SMSG_CHAT_PLAYER_NOT_FOUND),
    SMSG_DUEL_COUNTDOWN(SMSG_DUEL_COUNTDOWN),
    SMSG_DURABILITY_DAMAGE_DEATH(SMSG_DURABILITY_DAMAGE_DEATH),
    SMSG_UPDATE_WORLD_STATE(SMSG_UPDATE_WORLD_STATE),
    SMSG_CHAR_RENAME(SMSG_CHAR_RENAME),
    MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Server),
    SMSG_BATTLEGROUND_PLAYER_JOINED(SMSG_BATTLEGROUND_PLAYER_JOINED),
    SMSG_BATTLEGROUND_PLAYER_LEFT(SMSG_BATTLEGROUND_PLAYER_LEFT),
    SMSG_CHAT_RESTRICTED(SMSG_CHAT_RESTRICTED),
    SMSG_GM_TICKET_STATUS_UPDATE(SMSG_GM_TICKET_STATUS_UPDATE),
    SMSG_DEFENSE_MESSAGE(SMSG_DEFENSE_MESSAGE),
    MSG_MOVE_START_ASCEND(MSG_MOVE_START_ASCEND_Server),
    MSG_MOVE_STOP_ASCEND(MSG_MOVE_STOP_ASCEND_Server),
    SMSG_REALM_SPLIT(SMSG_REALM_SPLIT),
    SMSG_TIME_SYNC_REQ(SMSG_TIME_SYNC_REQ),
    MSG_MOVE_START_DESCEND(MSG_MOVE_START_DESCEND_Server),
}

impl ServerOpcodeMessage {
    fn read_opcodes(opcode: u16, body_size: u32, mut r: &[u8]) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        match opcode {
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(<MSG_MOVE_WORLDPORT_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x003A => Ok(Self::SMSG_CHAR_CREATE(<SMSG_CHAR_CREATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x003B => Ok(Self::SMSG_CHAR_ENUM(<SMSG_CHAR_ENUM as crate::Message>::read_body(&mut r, body_size)?)),
            0x003C => Ok(Self::SMSG_CHAR_DELETE(<SMSG_CHAR_DELETE as crate::Message>::read_body(&mut r, body_size)?)),
            0x003E => Ok(Self::SMSG_NEW_WORLD(<SMSG_NEW_WORLD as crate::Message>::read_body(&mut r, body_size)?)),
            0x003F => Ok(Self::SMSG_TRANSFER_PENDING(<SMSG_TRANSFER_PENDING as crate::Message>::read_body(&mut r, body_size)?)),
            0x0041 => Ok(Self::SMSG_CHARACTER_LOGIN_FAILED(<SMSG_CHARACTER_LOGIN_FAILED as crate::Message>::read_body(&mut r, body_size)?)),
            0x0042 => Ok(Self::SMSG_LOGIN_SETTIMESPEED(<SMSG_LOGIN_SETTIMESPEED as crate::Message>::read_body(&mut r, body_size)?)),
            0x004D => Ok(Self::SMSG_LOGOUT_COMPLETE(<SMSG_LOGOUT_COMPLETE as crate::Message>::read_body(&mut r, body_size)?)),
            0x004F => Ok(Self::SMSG_LOGOUT_CANCEL_ACK(<SMSG_LOGOUT_CANCEL_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x0099 => Ok(Self::SMSG_CHANNEL_NOTIFY(<SMSG_CHANNEL_NOTIFY as crate::Message>::read_body(&mut r, body_size)?)),
            0x009B => Ok(Self::SMSG_CHANNEL_LIST(<SMSG_CHANNEL_LIST as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A9 => Ok(Self::SMSG_UPDATE_OBJECT(<SMSG_UPDATE_OBJECT as crate::Message>::read_body(&mut r, body_size)?)),
            0x00AA => Ok(Self::SMSG_DESTROY_OBJECT(<SMSG_DESTROY_OBJECT as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(<MSG_MOVE_START_FORWARD_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(<MSG_MOVE_START_BACKWARD_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(<MSG_MOVE_STOP_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(<MSG_MOVE_START_STRAFE_LEFT_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(<MSG_MOVE_START_STRAFE_RIGHT_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(<MSG_MOVE_STOP_STRAFE_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(<MSG_MOVE_JUMP_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(<MSG_MOVE_START_TURN_LEFT_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(<MSG_MOVE_START_TURN_RIGHT_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(<MSG_MOVE_STOP_TURN_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(<MSG_MOVE_START_PITCH_UP_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(<MSG_MOVE_START_PITCH_DOWN_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(<MSG_MOVE_STOP_PITCH_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(<MSG_MOVE_SET_RUN_MODE_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(<MSG_MOVE_SET_WALK_MODE_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(<MSG_MOVE_TELEPORT_ACK_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(<MSG_MOVE_FALL_LAND_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(<MSG_MOVE_START_SWIM_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(<MSG_MOVE_STOP_SWIM_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(<MSG_MOVE_SET_FACING_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(<MSG_MOVE_SET_PITCH_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(<MSG_MOVE_HEARTBEAT_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00FA => Ok(Self::SMSG_TRIGGER_CINEMATIC(<SMSG_TRIGGER_CINEMATIC as crate::Message>::read_body(&mut r, body_size)?)),
            0x00FD => Ok(Self::SMSG_TUTORIAL_FLAGS(<SMSG_TUTORIAL_FLAGS as crate::Message>::read_body(&mut r, body_size)?)),
            0x0103 => Ok(Self::SMSG_EMOTE(<SMSG_EMOTE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0105 => Ok(Self::SMSG_TEXT_EMOTE(<SMSG_TEXT_EMOTE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0122 => Ok(Self::SMSG_INITIALIZE_FACTIONS(<SMSG_INITIALIZE_FACTIONS as crate::Message>::read_body(&mut r, body_size)?)),
            0x0123 => Ok(Self::SMSG_SET_FACTION_VISIBLE(<SMSG_SET_FACTION_VISIBLE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0124 => Ok(Self::SMSG_SET_FACTION_STANDING(<SMSG_SET_FACTION_STANDING as crate::Message>::read_body(&mut r, body_size)?)),
            0x0129 => Ok(Self::SMSG_ACTION_BUTTONS(<SMSG_ACTION_BUTTONS as crate::Message>::read_body(&mut r, body_size)?)),
            0x013C => Ok(Self::SMSG_AI_REACTION(<SMSG_AI_REACTION as crate::Message>::read_body(&mut r, body_size)?)),
            0x0143 => Ok(Self::SMSG_ATTACKSTART(<SMSG_ATTACKSTART as crate::Message>::read_body(&mut r, body_size)?)),
            0x0144 => Ok(Self::SMSG_ATTACKSTOP(<SMSG_ATTACKSTOP as crate::Message>::read_body(&mut r, body_size)?)),
            0x0145 => Ok(Self::SMSG_ATTACKSWING_NOTINRANGE(<SMSG_ATTACKSWING_NOTINRANGE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0146 => Ok(Self::SMSG_ATTACKSWING_BADFACING(<SMSG_ATTACKSWING_BADFACING as crate::Message>::read_body(&mut r, body_size)?)),
            0x0147 => Ok(Self::SMSG_ATTACKSWING_NOTSTANDING(<SMSG_ATTACKSWING_NOTSTANDING as crate::Message>::read_body(&mut r, body_size)?)),
            0x0148 => Ok(Self::SMSG_ATTACKSWING_DEADTARGET(<SMSG_ATTACKSWING_DEADTARGET as crate::Message>::read_body(&mut r, body_size)?)),
            0x0149 => Ok(Self::SMSG_ATTACKSWING_CANT_ATTACK(<SMSG_ATTACKSWING_CANT_ATTACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x014E => Ok(Self::SMSG_CANCEL_COMBAT(<SMSG_CANCEL_COMBAT as crate::Message>::read_body(&mut r, body_size)?)),
            0x0167 => Ok(Self::SMSG_DUEL_REQUESTED(<SMSG_DUEL_REQUESTED as crate::Message>::read_body(&mut r, body_size)?)),
            0x0168 => Ok(Self::SMSG_DUEL_OUTOFBOUNDS(<SMSG_DUEL_OUTOFBOUNDS as crate::Message>::read_body(&mut r, body_size)?)),
            0x0169 => Ok(Self::SMSG_DUEL_INBOUNDS(<SMSG_DUEL_INBOUNDS as crate::Message>::read_body(&mut r, body_size)?)),
            0x016A => Ok(Self::SMSG_DUEL_COMPLETE(<SMSG_DUEL_COMPLETE as crate::Message>::read_body(&mut r, body_size)?)),
            0x016B => Ok(Self::SMSG_DUEL_WINNER(<SMSG_DUEL_WINNER as crate::Message>::read_body(&mut r, body_size)?)),
            0x01CB => Ok(Self::SMSG_NOTIFICATION(<SMSG_NOTIFICATION as crate::Message>::read_body(&mut r, body_size)?)),
            0x01D0 => Ok(Self::SMSG_LOG_XPGAIN(<SMSG_LOG_XPGAIN as crate::Message>::read_body(&mut r, body_size)?)),
            0x01D4 => Ok(Self::SMSG_LEVELUP_INFO(<SMSG_LEVELUP_INFO as crate::Message>::read_body(&mut r, body_size)?)),
            0x01DD => Ok(Self::SMSG_PONG(<SMSG_PONG as crate::Message>::read_body(&mut r, body_size)?)),
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(<SMSG_AUTH_CHALLENGE as crate::Message>::read_body(&mut r, body_size)?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(<SMSG_AUTH_RESPONSE as crate::Message>::read_body(&mut r, body_size)?)),
            0x01F8 => Ok(Self::SMSG_EXPLORATION_EXPERIENCE(<SMSG_EXPLORATION_EXPERIENCE as crate::Message>::read_body(&mut r, body_size)?)),
            0x01FC => Ok(Self::SMSG_ENVIRONMENTALDAMAGELOG(<SMSG_ENVIRONMENTALDAMAGELOG as crate::Message>::read_body(&mut r, body_size)?)),
            0x0206 => Ok(Self::SMSG_GMTICKET_CREATE(<SMSG_GMTICKET_CREATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0209 => Ok(Self::SMSG_ACCOUNT_DATA_TIMES(<SMSG_ACCOUNT_DATA_TIMES as crate::Message>::read_body(&mut r, body_size)?)),
            0x0219 => Ok(Self::SMSG_CHAT_WRONG_FACTION(<SMSG_CHAT_WRONG_FACTION as crate::Message>::read_body(&mut r, body_size)?)),
            0x021E => Ok(Self::SMSG_SET_REST_START(<SMSG_SET_REST_START as crate::Message>::read_body(&mut r, body_size)?)),
            0x0236 => Ok(Self::SMSG_LOGIN_VERIFY_WORLD(<SMSG_LOGIN_VERIFY_WORLD as crate::Message>::read_body(&mut r, body_size)?)),
            0x023D => Ok(Self::SMSG_BATTLEFIELD_LIST(<SMSG_BATTLEFIELD_LIST as crate::Message>::read_body(&mut r, body_size)?)),
            0x0254 => Ok(Self::SMSG_ZONE_UNDER_ATTACK(<SMSG_ZONE_UNDER_ATTACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(<MSG_AUCTION_HELLO_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x025C => Ok(Self::SMSG_AUCTION_LIST_RESULT(<SMSG_AUCTION_LIST_RESULT as crate::Message>::read_body(&mut r, body_size)?)),
            0x025D => Ok(Self::SMSG_AUCTION_OWNER_LIST_RESULT(<SMSG_AUCTION_OWNER_LIST_RESULT as crate::Message>::read_body(&mut r, body_size)?)),
            0x025E => Ok(Self::SMSG_AUCTION_BIDDER_NOTIFICATION(<SMSG_AUCTION_BIDDER_NOTIFICATION as crate::Message>::read_body(&mut r, body_size)?)),
            0x025F => Ok(Self::SMSG_AUCTION_OWNER_NOTIFICATION(<SMSG_AUCTION_OWNER_NOTIFICATION as crate::Message>::read_body(&mut r, body_size)?)),
            0x0260 => Ok(Self::SMSG_PROCRESIST(<SMSG_PROCRESIST as crate::Message>::read_body(&mut r, body_size)?)),
            0x0265 => Ok(Self::SMSG_AUCTION_BIDDER_LIST_RESULT(<SMSG_AUCTION_BIDDER_LIST_RESULT as crate::Message>::read_body(&mut r, body_size)?)),
            0x028D => Ok(Self::SMSG_AUCTION_REMOVED_NOTIFICATION(<SMSG_AUCTION_REMOVED_NOTIFICATION as crate::Message>::read_body(&mut r, body_size)?)),
            0x0291 => Ok(Self::SMSG_SERVER_MESSAGE(<SMSG_SERVER_MESSAGE as crate::Message>::read_body(&mut r, body_size)?)),
            0x029D => Ok(Self::SMSG_STANDSTATE_UPDATE(<SMSG_STANDSTATE_UPDATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x02A5 => Ok(Self::SMSG_SET_FORCED_REACTIONS(<SMSG_SET_FORCED_REACTIONS as crate::Message>::read_body(&mut r, body_size)?)),
            0x02A9 => Ok(Self::SMSG_CHAT_PLAYER_NOT_FOUND(<SMSG_CHAT_PLAYER_NOT_FOUND as crate::Message>::read_body(&mut r, body_size)?)),
            0x02B7 => Ok(Self::SMSG_DUEL_COUNTDOWN(<SMSG_DUEL_COUNTDOWN as crate::Message>::read_body(&mut r, body_size)?)),
            0x02BD => Ok(Self::SMSG_DURABILITY_DAMAGE_DEATH(<SMSG_DURABILITY_DAMAGE_DEATH as crate::Message>::read_body(&mut r, body_size)?)),
            0x02C3 => Ok(Self::SMSG_UPDATE_WORLD_STATE(<SMSG_UPDATE_WORLD_STATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x02C8 => Ok(Self::SMSG_CHAR_RENAME(<SMSG_CHAR_RENAME as crate::Message>::read_body(&mut r, body_size)?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(<MSG_BATTLEGROUND_PLAYER_POSITIONS_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x02EC => Ok(Self::SMSG_BATTLEGROUND_PLAYER_JOINED(<SMSG_BATTLEGROUND_PLAYER_JOINED as crate::Message>::read_body(&mut r, body_size)?)),
            0x02ED => Ok(Self::SMSG_BATTLEGROUND_PLAYER_LEFT(<SMSG_BATTLEGROUND_PLAYER_LEFT as crate::Message>::read_body(&mut r, body_size)?)),
            0x02FD => Ok(Self::SMSG_CHAT_RESTRICTED(<SMSG_CHAT_RESTRICTED as crate::Message>::read_body(&mut r, body_size)?)),
            0x0328 => Ok(Self::SMSG_GM_TICKET_STATUS_UPDATE(<SMSG_GM_TICKET_STATUS_UPDATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x033A => Ok(Self::SMSG_DEFENSE_MESSAGE(<SMSG_DEFENSE_MESSAGE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0359 => Ok(Self::MSG_MOVE_START_ASCEND(<MSG_MOVE_START_ASCEND_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x035A => Ok(Self::MSG_MOVE_STOP_ASCEND(<MSG_MOVE_STOP_ASCEND_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x038B => Ok(Self::SMSG_REALM_SPLIT(<SMSG_REALM_SPLIT as crate::Message>::read_body(&mut r, body_size)?)),
            0x0390 => Ok(Self::SMSG_TIME_SYNC_REQ(<SMSG_TIME_SYNC_REQ as crate::Message>::read_body(&mut r, body_size)?)),
            0x03A7 => Ok(Self::MSG_MOVE_START_DESCEND(<MSG_MOVE_START_DESCEND_Server as crate::Message>::read_body(&mut r, body_size)?)),
            _ => Err(crate::errors::ExpectedOpcodeError::Opcode{ opcode: opcode.into(), size: body_size }),
        }
    }

    #[cfg(feature = "sync")]
    pub fn read_unencrypted<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::read_u16_be(r)?.saturating_sub(2)) as u32;
        let opcode = crate::util::read_u16_le(r)?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "sync")]
    pub fn read_encrypted<R: std::io::Read>(r: &mut R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header)?;
        let header = d.decrypt_server_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(2)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read_unencrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::tokio_read_u16_be(r).await?.saturating_sub(2)) as u32;
        let opcode = crate::util::tokio_read_u16_le(r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "tokio")]
    pub async fn tokio_read_encrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_server_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(2)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read_unencrypted<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::astd_read_u16_be(r).await?.saturating_sub(2)) as u32;
        let opcode = crate::util::astd_read_u16_le(r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "async-std")]
    pub async fn astd_read_encrypted<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_server_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(2)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "sync")]
    pub fn write_encrypted_server<W: std::io::Write>(&self, w: &mut W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_CREATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_ENUM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_DELETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_NEW_WORLD(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TRANSFER_PENDING(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGOUT_COMPLETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHANNEL_NOTIFY(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHANNEL_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_OBJECT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DESTROY_OBJECT(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_FORWARD(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_BACKWARD(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_STOP(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_STOP_STRAFE(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_JUMP(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_STOP_TURN(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_PITCH_UP(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_STOP_PITCH(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_FALL_LAND(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_SWIM(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_STOP_SWIM(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_SET_FACING(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_SET_PITCH(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_HEARTBEAT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TUTORIAL_FLAGS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_EMOTE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TEXT_EMOTE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SET_FACTION_STANDING(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ACTION_BUTTONS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AI_REACTION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSTART(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSTOP(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_NOTSTANDING(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CANCEL_COMBAT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DUEL_REQUESTED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DUEL_OUTOFBOUNDS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DUEL_INBOUNDS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DUEL_COMPLETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DUEL_WINNER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOG_XPGAIN(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LEVELUP_INFO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PONG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUTH_CHALLENGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUTH_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GMTICKET_CREATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SET_REST_START(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BATTLEFIELD_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.write_encrypted_server(w, e),
            Self::MSG_AUCTION_HELLO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PROCRESIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SERVER_MESSAGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_STANDSTATE_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DUEL_COUNTDOWN(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_RENAME(c) => c.write_encrypted_server(w, e),
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAT_RESTRICTED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DEFENSE_MESSAGE(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_ASCEND(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_STOP_ASCEND(c) => c.write_encrypted_server(w, e),
            Self::SMSG_REALM_SPLIT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TIME_SYNC_REQ(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_DESCEND(c) => c.write_encrypted_server(w, e),
        }
    }

    #[cfg(feature = "sync")]
    pub fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_CREATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_ENUM(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_DELETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_NEW_WORLD(c) => c.write_unencrypted_server(w),
            Self::SMSG_TRANSFER_PENDING(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGOUT_COMPLETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHANNEL_NOTIFY(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHANNEL_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_OBJECT(c) => c.write_unencrypted_server(w),
            Self::SMSG_DESTROY_OBJECT(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_FORWARD(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_BACKWARD(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_STOP(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_STOP_STRAFE(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_JUMP(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_STOP_TURN(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_PITCH_UP(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_STOP_PITCH(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_FALL_LAND(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_SWIM(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_STOP_SWIM(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_SET_FACING(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_SET_PITCH(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_HEARTBEAT(c) => c.write_unencrypted_server(w),
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.write_unencrypted_server(w),
            Self::SMSG_TUTORIAL_FLAGS(c) => c.write_unencrypted_server(w),
            Self::SMSG_EMOTE(c) => c.write_unencrypted_server(w),
            Self::SMSG_TEXT_EMOTE(c) => c.write_unencrypted_server(w),
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.write_unencrypted_server(w),
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.write_unencrypted_server(w),
            Self::SMSG_SET_FACTION_STANDING(c) => c.write_unencrypted_server(w),
            Self::SMSG_ACTION_BUTTONS(c) => c.write_unencrypted_server(w),
            Self::SMSG_AI_REACTION(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSTART(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSTOP(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_NOTSTANDING(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.write_unencrypted_server(w),
            Self::SMSG_CANCEL_COMBAT(c) => c.write_unencrypted_server(w),
            Self::SMSG_DUEL_REQUESTED(c) => c.write_unencrypted_server(w),
            Self::SMSG_DUEL_OUTOFBOUNDS(c) => c.write_unencrypted_server(w),
            Self::SMSG_DUEL_INBOUNDS(c) => c.write_unencrypted_server(w),
            Self::SMSG_DUEL_COMPLETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_DUEL_WINNER(c) => c.write_unencrypted_server(w),
            Self::SMSG_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOG_XPGAIN(c) => c.write_unencrypted_server(w),
            Self::SMSG_LEVELUP_INFO(c) => c.write_unencrypted_server(w),
            Self::SMSG_PONG(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUTH_CHALLENGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUTH_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.write_unencrypted_server(w),
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.write_unencrypted_server(w),
            Self::SMSG_GMTICKET_CREATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.write_unencrypted_server(w),
            Self::SMSG_SET_REST_START(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.write_unencrypted_server(w),
            Self::SMSG_BATTLEFIELD_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.write_unencrypted_server(w),
            Self::MSG_AUCTION_HELLO(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_PROCRESIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_SERVER_MESSAGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_STANDSTATE_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.write_unencrypted_server(w),
            Self::SMSG_DUEL_COUNTDOWN(c) => c.write_unencrypted_server(w),
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_RENAME(c) => c.write_unencrypted_server(w),
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.write_unencrypted_server(w),
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.write_unencrypted_server(w),
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAT_RESTRICTED(c) => c.write_unencrypted_server(w),
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_DEFENSE_MESSAGE(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_ASCEND(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_STOP_ASCEND(c) => c.write_unencrypted_server(w),
            Self::SMSG_REALM_SPLIT(c) => c.write_unencrypted_server(w),
            Self::SMSG_TIME_SYNC_REQ(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_DESCEND(c) => c.write_unencrypted_server(w),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write_encrypted_server<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, w: &mut W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_CREATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_ENUM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_DELETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_NEW_WORLD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TRANSFER_PENDING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_COMPLETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHANNEL_NOTIFY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHANNEL_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_OBJECT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DESTROY_OBJECT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_FORWARD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_BACKWARD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_STRAFE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_JUMP(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_TURN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_PITCH_UP(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_PITCH(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_SWIM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_FACING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_EMOTE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TEXT_EMOTE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FACTION_STANDING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ACTION_BUTTONS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AI_REACTION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSTART(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSTOP(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_NOTSTANDING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CANCEL_COMBAT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_REQUESTED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_OUTOFBOUNDS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_INBOUNDS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_COMPLETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_WINNER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOG_XPGAIN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LEVELUP_INFO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PONG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_CREATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SET_REST_START(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEFIELD_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PROCRESIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SERVER_MESSAGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_STANDSTATE_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_COUNTDOWN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_RENAME(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_RESTRICTED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_REALM_SPLIT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TIME_SYNC_REQ(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.tokio_write_encrypted_server(w, e).await,
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write_unencrypted_server<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_CREATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_ENUM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_DELETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_NEW_WORLD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TRANSFER_PENDING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_COMPLETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHANNEL_NOTIFY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHANNEL_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_OBJECT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DESTROY_OBJECT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_FORWARD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_BACKWARD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_STRAFE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_JUMP(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_TURN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_PITCH_UP(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_PITCH(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_SWIM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_FACING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_EMOTE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TEXT_EMOTE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SET_FACTION_STANDING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ACTION_BUTTONS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AI_REACTION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSTART(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSTOP(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_NOTSTANDING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CANCEL_COMBAT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_REQUESTED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_OUTOFBOUNDS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_INBOUNDS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_COMPLETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_WINNER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOG_XPGAIN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LEVELUP_INFO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PONG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_CREATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SET_REST_START(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEFIELD_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PROCRESIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SERVER_MESSAGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_STANDSTATE_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_COUNTDOWN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_RENAME(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_RESTRICTED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_REALM_SPLIT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TIME_SYNC_REQ(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.tokio_write_unencrypted_server(w).await,
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write_encrypted_server<W: async_std::io::WriteExt + Unpin + Send>(&self, w: &mut W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_CREATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_ENUM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_DELETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_NEW_WORLD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TRANSFER_PENDING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_COMPLETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHANNEL_NOTIFY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHANNEL_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_OBJECT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DESTROY_OBJECT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_FORWARD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_BACKWARD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_STRAFE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_JUMP(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_TURN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_PITCH_UP(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_PITCH(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_SWIM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_FACING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_EMOTE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TEXT_EMOTE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FACTION_STANDING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ACTION_BUTTONS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AI_REACTION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSTART(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSTOP(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_NOTSTANDING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CANCEL_COMBAT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_REQUESTED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_OUTOFBOUNDS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_INBOUNDS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_COMPLETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_WINNER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOG_XPGAIN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LEVELUP_INFO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PONG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_CREATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SET_REST_START(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEFIELD_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PROCRESIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SERVER_MESSAGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_STANDSTATE_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_COUNTDOWN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_RENAME(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_RESTRICTED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_REALM_SPLIT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TIME_SYNC_REQ(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.astd_write_encrypted_server(w, e).await,
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write_unencrypted_server<W: async_std::io::WriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_CREATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_ENUM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_DELETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_NEW_WORLD(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TRANSFER_PENDING(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_COMPLETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHANNEL_NOTIFY(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHANNEL_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_OBJECT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DESTROY_OBJECT(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_FORWARD(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_BACKWARD(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_STRAFE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_JUMP(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_TURN(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_PITCH_UP(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_PITCH(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_SWIM(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_FACING(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_EMOTE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TEXT_EMOTE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SET_FACTION_STANDING(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ACTION_BUTTONS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AI_REACTION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSTART(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSTOP(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_NOTSTANDING(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CANCEL_COMBAT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_REQUESTED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_OUTOFBOUNDS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_INBOUNDS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_COMPLETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_WINNER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOG_XPGAIN(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LEVELUP_INFO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PONG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_CREATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SET_REST_START(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEFIELD_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PROCRESIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SERVER_MESSAGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_STANDSTATE_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_COUNTDOWN(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_RENAME(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_RESTRICTED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_REALM_SPLIT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TIME_SYNC_REQ(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.astd_write_unencrypted_server(w).await,
        }
    }

}

impl std::fmt::Display for ServerOpcodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ServerOpcodeMessage::MSG_MOVE_WORLDPORT_ACK(_) => "MSG_MOVE_WORLDPORT_ACK",
            ServerOpcodeMessage::SMSG_CHAR_CREATE(_) => "SMSG_CHAR_CREATE",
            ServerOpcodeMessage::SMSG_CHAR_ENUM(_) => "SMSG_CHAR_ENUM",
            ServerOpcodeMessage::SMSG_CHAR_DELETE(_) => "SMSG_CHAR_DELETE",
            ServerOpcodeMessage::SMSG_NEW_WORLD(_) => "SMSG_NEW_WORLD",
            ServerOpcodeMessage::SMSG_TRANSFER_PENDING(_) => "SMSG_TRANSFER_PENDING",
            ServerOpcodeMessage::SMSG_CHARACTER_LOGIN_FAILED(_) => "SMSG_CHARACTER_LOGIN_FAILED",
            ServerOpcodeMessage::SMSG_LOGIN_SETTIMESPEED(_) => "SMSG_LOGIN_SETTIMESPEED",
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE(_) => "SMSG_LOGOUT_COMPLETE",
            ServerOpcodeMessage::SMSG_LOGOUT_CANCEL_ACK(_) => "SMSG_LOGOUT_CANCEL_ACK",
            ServerOpcodeMessage::SMSG_CHANNEL_NOTIFY(_) => "SMSG_CHANNEL_NOTIFY",
            ServerOpcodeMessage::SMSG_CHANNEL_LIST(_) => "SMSG_CHANNEL_LIST",
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(_) => "SMSG_UPDATE_OBJECT",
            ServerOpcodeMessage::SMSG_DESTROY_OBJECT(_) => "SMSG_DESTROY_OBJECT",
            ServerOpcodeMessage::MSG_MOVE_START_FORWARD(_) => "MSG_MOVE_START_FORWARD_Server",
            ServerOpcodeMessage::MSG_MOVE_START_BACKWARD(_) => "MSG_MOVE_START_BACKWARD_Server",
            ServerOpcodeMessage::MSG_MOVE_STOP(_) => "MSG_MOVE_STOP_Server",
            ServerOpcodeMessage::MSG_MOVE_START_STRAFE_LEFT(_) => "MSG_MOVE_START_STRAFE_LEFT_Server",
            ServerOpcodeMessage::MSG_MOVE_START_STRAFE_RIGHT(_) => "MSG_MOVE_START_STRAFE_RIGHT_Server",
            ServerOpcodeMessage::MSG_MOVE_STOP_STRAFE(_) => "MSG_MOVE_STOP_STRAFE_Server",
            ServerOpcodeMessage::MSG_MOVE_JUMP(_) => "MSG_MOVE_JUMP_Server",
            ServerOpcodeMessage::MSG_MOVE_START_TURN_LEFT(_) => "MSG_MOVE_START_TURN_LEFT_Server",
            ServerOpcodeMessage::MSG_MOVE_START_TURN_RIGHT(_) => "MSG_MOVE_START_TURN_RIGHT_Server",
            ServerOpcodeMessage::MSG_MOVE_STOP_TURN(_) => "MSG_MOVE_STOP_TURN_Server",
            ServerOpcodeMessage::MSG_MOVE_START_PITCH_UP(_) => "MSG_MOVE_START_PITCH_UP_Server",
            ServerOpcodeMessage::MSG_MOVE_START_PITCH_DOWN(_) => "MSG_MOVE_START_PITCH_DOWN_Server",
            ServerOpcodeMessage::MSG_MOVE_STOP_PITCH(_) => "MSG_MOVE_STOP_PITCH_Server",
            ServerOpcodeMessage::MSG_MOVE_SET_RUN_MODE(_) => "MSG_MOVE_SET_RUN_MODE_Server",
            ServerOpcodeMessage::MSG_MOVE_SET_WALK_MODE(_) => "MSG_MOVE_SET_WALK_MODE_Server",
            ServerOpcodeMessage::MSG_MOVE_TELEPORT_ACK(_) => "MSG_MOVE_TELEPORT_ACK_Server",
            ServerOpcodeMessage::MSG_MOVE_FALL_LAND(_) => "MSG_MOVE_FALL_LAND_Server",
            ServerOpcodeMessage::MSG_MOVE_START_SWIM(_) => "MSG_MOVE_START_SWIM_Server",
            ServerOpcodeMessage::MSG_MOVE_STOP_SWIM(_) => "MSG_MOVE_STOP_SWIM_Server",
            ServerOpcodeMessage::MSG_MOVE_SET_FACING(_) => "MSG_MOVE_SET_FACING_Server",
            ServerOpcodeMessage::MSG_MOVE_SET_PITCH(_) => "MSG_MOVE_SET_PITCH_Server",
            ServerOpcodeMessage::MSG_MOVE_HEARTBEAT(_) => "MSG_MOVE_HEARTBEAT_Server",
            ServerOpcodeMessage::SMSG_TRIGGER_CINEMATIC(_) => "SMSG_TRIGGER_CINEMATIC",
            ServerOpcodeMessage::SMSG_TUTORIAL_FLAGS(_) => "SMSG_TUTORIAL_FLAGS",
            ServerOpcodeMessage::SMSG_EMOTE(_) => "SMSG_EMOTE",
            ServerOpcodeMessage::SMSG_TEXT_EMOTE(_) => "SMSG_TEXT_EMOTE",
            ServerOpcodeMessage::SMSG_INITIALIZE_FACTIONS(_) => "SMSG_INITIALIZE_FACTIONS",
            ServerOpcodeMessage::SMSG_SET_FACTION_VISIBLE(_) => "SMSG_SET_FACTION_VISIBLE",
            ServerOpcodeMessage::SMSG_SET_FACTION_STANDING(_) => "SMSG_SET_FACTION_STANDING",
            ServerOpcodeMessage::SMSG_ACTION_BUTTONS(_) => "SMSG_ACTION_BUTTONS",
            ServerOpcodeMessage::SMSG_AI_REACTION(_) => "SMSG_AI_REACTION",
            ServerOpcodeMessage::SMSG_ATTACKSTART(_) => "SMSG_ATTACKSTART",
            ServerOpcodeMessage::SMSG_ATTACKSTOP(_) => "SMSG_ATTACKSTOP",
            ServerOpcodeMessage::SMSG_ATTACKSWING_NOTINRANGE(_) => "SMSG_ATTACKSWING_NOTINRANGE",
            ServerOpcodeMessage::SMSG_ATTACKSWING_BADFACING(_) => "SMSG_ATTACKSWING_BADFACING",
            ServerOpcodeMessage::SMSG_ATTACKSWING_NOTSTANDING(_) => "SMSG_ATTACKSWING_NOTSTANDING",
            ServerOpcodeMessage::SMSG_ATTACKSWING_DEADTARGET(_) => "SMSG_ATTACKSWING_DEADTARGET",
            ServerOpcodeMessage::SMSG_ATTACKSWING_CANT_ATTACK(_) => "SMSG_ATTACKSWING_CANT_ATTACK",
            ServerOpcodeMessage::SMSG_CANCEL_COMBAT(_) => "SMSG_CANCEL_COMBAT",
            ServerOpcodeMessage::SMSG_DUEL_REQUESTED(_) => "SMSG_DUEL_REQUESTED",
            ServerOpcodeMessage::SMSG_DUEL_OUTOFBOUNDS(_) => "SMSG_DUEL_OUTOFBOUNDS",
            ServerOpcodeMessage::SMSG_DUEL_INBOUNDS(_) => "SMSG_DUEL_INBOUNDS",
            ServerOpcodeMessage::SMSG_DUEL_COMPLETE(_) => "SMSG_DUEL_COMPLETE",
            ServerOpcodeMessage::SMSG_DUEL_WINNER(_) => "SMSG_DUEL_WINNER",
            ServerOpcodeMessage::SMSG_NOTIFICATION(_) => "SMSG_NOTIFICATION",
            ServerOpcodeMessage::SMSG_LOG_XPGAIN(_) => "SMSG_LOG_XPGAIN",
            ServerOpcodeMessage::SMSG_LEVELUP_INFO(_) => "SMSG_LEVELUP_INFO",
            ServerOpcodeMessage::SMSG_PONG(_) => "SMSG_PONG",
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(_) => "SMSG_AUTH_CHALLENGE",
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(_) => "SMSG_AUTH_RESPONSE",
            ServerOpcodeMessage::SMSG_EXPLORATION_EXPERIENCE(_) => "SMSG_EXPLORATION_EXPERIENCE",
            ServerOpcodeMessage::SMSG_ENVIRONMENTALDAMAGELOG(_) => "SMSG_ENVIRONMENTALDAMAGELOG",
            ServerOpcodeMessage::SMSG_GMTICKET_CREATE(_) => "SMSG_GMTICKET_CREATE",
            ServerOpcodeMessage::SMSG_ACCOUNT_DATA_TIMES(_) => "SMSG_ACCOUNT_DATA_TIMES",
            ServerOpcodeMessage::SMSG_CHAT_WRONG_FACTION(_) => "SMSG_CHAT_WRONG_FACTION",
            ServerOpcodeMessage::SMSG_SET_REST_START(_) => "SMSG_SET_REST_START",
            ServerOpcodeMessage::SMSG_LOGIN_VERIFY_WORLD(_) => "SMSG_LOGIN_VERIFY_WORLD",
            ServerOpcodeMessage::SMSG_BATTLEFIELD_LIST(_) => "SMSG_BATTLEFIELD_LIST",
            ServerOpcodeMessage::SMSG_ZONE_UNDER_ATTACK(_) => "SMSG_ZONE_UNDER_ATTACK",
            ServerOpcodeMessage::MSG_AUCTION_HELLO(_) => "MSG_AUCTION_HELLO_Server",
            ServerOpcodeMessage::SMSG_AUCTION_LIST_RESULT(_) => "SMSG_AUCTION_LIST_RESULT",
            ServerOpcodeMessage::SMSG_AUCTION_OWNER_LIST_RESULT(_) => "SMSG_AUCTION_OWNER_LIST_RESULT",
            ServerOpcodeMessage::SMSG_AUCTION_BIDDER_NOTIFICATION(_) => "SMSG_AUCTION_BIDDER_NOTIFICATION",
            ServerOpcodeMessage::SMSG_AUCTION_OWNER_NOTIFICATION(_) => "SMSG_AUCTION_OWNER_NOTIFICATION",
            ServerOpcodeMessage::SMSG_PROCRESIST(_) => "SMSG_PROCRESIST",
            ServerOpcodeMessage::SMSG_AUCTION_BIDDER_LIST_RESULT(_) => "SMSG_AUCTION_BIDDER_LIST_RESULT",
            ServerOpcodeMessage::SMSG_AUCTION_REMOVED_NOTIFICATION(_) => "SMSG_AUCTION_REMOVED_NOTIFICATION",
            ServerOpcodeMessage::SMSG_SERVER_MESSAGE(_) => "SMSG_SERVER_MESSAGE",
            ServerOpcodeMessage::SMSG_STANDSTATE_UPDATE(_) => "SMSG_STANDSTATE_UPDATE",
            ServerOpcodeMessage::SMSG_SET_FORCED_REACTIONS(_) => "SMSG_SET_FORCED_REACTIONS",
            ServerOpcodeMessage::SMSG_CHAT_PLAYER_NOT_FOUND(_) => "SMSG_CHAT_PLAYER_NOT_FOUND",
            ServerOpcodeMessage::SMSG_DUEL_COUNTDOWN(_) => "SMSG_DUEL_COUNTDOWN",
            ServerOpcodeMessage::SMSG_DURABILITY_DAMAGE_DEATH(_) => "SMSG_DURABILITY_DAMAGE_DEATH",
            ServerOpcodeMessage::SMSG_UPDATE_WORLD_STATE(_) => "SMSG_UPDATE_WORLD_STATE",
            ServerOpcodeMessage::SMSG_CHAR_RENAME(_) => "SMSG_CHAR_RENAME",
            ServerOpcodeMessage::MSG_BATTLEGROUND_PLAYER_POSITIONS(_) => "MSG_BATTLEGROUND_PLAYER_POSITIONS_Server",
            ServerOpcodeMessage::SMSG_BATTLEGROUND_PLAYER_JOINED(_) => "SMSG_BATTLEGROUND_PLAYER_JOINED",
            ServerOpcodeMessage::SMSG_BATTLEGROUND_PLAYER_LEFT(_) => "SMSG_BATTLEGROUND_PLAYER_LEFT",
            ServerOpcodeMessage::SMSG_CHAT_RESTRICTED(_) => "SMSG_CHAT_RESTRICTED",
            ServerOpcodeMessage::SMSG_GM_TICKET_STATUS_UPDATE(_) => "SMSG_GM_TICKET_STATUS_UPDATE",
            ServerOpcodeMessage::SMSG_DEFENSE_MESSAGE(_) => "SMSG_DEFENSE_MESSAGE",
            ServerOpcodeMessage::MSG_MOVE_START_ASCEND(_) => "MSG_MOVE_START_ASCEND_Server",
            ServerOpcodeMessage::MSG_MOVE_STOP_ASCEND(_) => "MSG_MOVE_STOP_ASCEND_Server",
            ServerOpcodeMessage::SMSG_REALM_SPLIT(_) => "SMSG_REALM_SPLIT",
            ServerOpcodeMessage::SMSG_TIME_SYNC_REQ(_) => "SMSG_TIME_SYNC_REQ",
            ServerOpcodeMessage::MSG_MOVE_START_DESCEND(_) => "MSG_MOVE_START_DESCEND_Server",
        })
    }
}

impl From<MSG_MOVE_WORLDPORT_ACK> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_WORLDPORT_ACK) -> Self {
        Self::MSG_MOVE_WORLDPORT_ACK(c)
    }
}

impl From<SMSG_CHAR_CREATE> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAR_CREATE) -> Self {
        Self::SMSG_CHAR_CREATE(c)
    }
}

impl From<SMSG_CHAR_ENUM> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAR_ENUM) -> Self {
        Self::SMSG_CHAR_ENUM(c)
    }
}

impl From<SMSG_CHAR_DELETE> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAR_DELETE) -> Self {
        Self::SMSG_CHAR_DELETE(c)
    }
}

impl From<SMSG_NEW_WORLD> for ServerOpcodeMessage {
    fn from(c: SMSG_NEW_WORLD) -> Self {
        Self::SMSG_NEW_WORLD(c)
    }
}

impl From<SMSG_TRANSFER_PENDING> for ServerOpcodeMessage {
    fn from(c: SMSG_TRANSFER_PENDING) -> Self {
        Self::SMSG_TRANSFER_PENDING(c)
    }
}

impl From<SMSG_CHARACTER_LOGIN_FAILED> for ServerOpcodeMessage {
    fn from(c: SMSG_CHARACTER_LOGIN_FAILED) -> Self {
        Self::SMSG_CHARACTER_LOGIN_FAILED(c)
    }
}

impl From<SMSG_LOGIN_SETTIMESPEED> for ServerOpcodeMessage {
    fn from(c: SMSG_LOGIN_SETTIMESPEED) -> Self {
        Self::SMSG_LOGIN_SETTIMESPEED(c)
    }
}

impl From<SMSG_LOGOUT_COMPLETE> for ServerOpcodeMessage {
    fn from(c: SMSG_LOGOUT_COMPLETE) -> Self {
        Self::SMSG_LOGOUT_COMPLETE(c)
    }
}

impl From<SMSG_LOGOUT_CANCEL_ACK> for ServerOpcodeMessage {
    fn from(c: SMSG_LOGOUT_CANCEL_ACK) -> Self {
        Self::SMSG_LOGOUT_CANCEL_ACK(c)
    }
}

impl From<SMSG_CHANNEL_NOTIFY> for ServerOpcodeMessage {
    fn from(c: SMSG_CHANNEL_NOTIFY) -> Self {
        Self::SMSG_CHANNEL_NOTIFY(c)
    }
}

impl From<SMSG_CHANNEL_LIST> for ServerOpcodeMessage {
    fn from(c: SMSG_CHANNEL_LIST) -> Self {
        Self::SMSG_CHANNEL_LIST(c)
    }
}

impl From<SMSG_UPDATE_OBJECT> for ServerOpcodeMessage {
    fn from(c: SMSG_UPDATE_OBJECT) -> Self {
        Self::SMSG_UPDATE_OBJECT(c)
    }
}

impl From<SMSG_DESTROY_OBJECT> for ServerOpcodeMessage {
    fn from(c: SMSG_DESTROY_OBJECT) -> Self {
        Self::SMSG_DESTROY_OBJECT(c)
    }
}

impl From<MSG_MOVE_START_FORWARD_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_FORWARD_Server) -> Self {
        Self::MSG_MOVE_START_FORWARD(c)
    }
}

impl From<MSG_MOVE_START_BACKWARD_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_BACKWARD_Server) -> Self {
        Self::MSG_MOVE_START_BACKWARD(c)
    }
}

impl From<MSG_MOVE_STOP_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_Server) -> Self {
        Self::MSG_MOVE_STOP(c)
    }
}

impl From<MSG_MOVE_START_STRAFE_LEFT_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_STRAFE_LEFT_Server) -> Self {
        Self::MSG_MOVE_START_STRAFE_LEFT(c)
    }
}

impl From<MSG_MOVE_START_STRAFE_RIGHT_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_STRAFE_RIGHT_Server) -> Self {
        Self::MSG_MOVE_START_STRAFE_RIGHT(c)
    }
}

impl From<MSG_MOVE_STOP_STRAFE_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_STRAFE_Server) -> Self {
        Self::MSG_MOVE_STOP_STRAFE(c)
    }
}

impl From<MSG_MOVE_JUMP_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_JUMP_Server) -> Self {
        Self::MSG_MOVE_JUMP(c)
    }
}

impl From<MSG_MOVE_START_TURN_LEFT_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_TURN_LEFT_Server) -> Self {
        Self::MSG_MOVE_START_TURN_LEFT(c)
    }
}

impl From<MSG_MOVE_START_TURN_RIGHT_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_TURN_RIGHT_Server) -> Self {
        Self::MSG_MOVE_START_TURN_RIGHT(c)
    }
}

impl From<MSG_MOVE_STOP_TURN_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_TURN_Server) -> Self {
        Self::MSG_MOVE_STOP_TURN(c)
    }
}

impl From<MSG_MOVE_START_PITCH_UP_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_PITCH_UP_Server) -> Self {
        Self::MSG_MOVE_START_PITCH_UP(c)
    }
}

impl From<MSG_MOVE_START_PITCH_DOWN_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_PITCH_DOWN_Server) -> Self {
        Self::MSG_MOVE_START_PITCH_DOWN(c)
    }
}

impl From<MSG_MOVE_STOP_PITCH_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_PITCH_Server) -> Self {
        Self::MSG_MOVE_STOP_PITCH(c)
    }
}

impl From<MSG_MOVE_SET_RUN_MODE_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_SET_RUN_MODE_Server) -> Self {
        Self::MSG_MOVE_SET_RUN_MODE(c)
    }
}

impl From<MSG_MOVE_SET_WALK_MODE_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_SET_WALK_MODE_Server) -> Self {
        Self::MSG_MOVE_SET_WALK_MODE(c)
    }
}

impl From<MSG_MOVE_TELEPORT_ACK_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_TELEPORT_ACK_Server) -> Self {
        Self::MSG_MOVE_TELEPORT_ACK(c)
    }
}

impl From<MSG_MOVE_FALL_LAND_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_FALL_LAND_Server) -> Self {
        Self::MSG_MOVE_FALL_LAND(c)
    }
}

impl From<MSG_MOVE_START_SWIM_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_SWIM_Server) -> Self {
        Self::MSG_MOVE_START_SWIM(c)
    }
}

impl From<MSG_MOVE_STOP_SWIM_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_SWIM_Server) -> Self {
        Self::MSG_MOVE_STOP_SWIM(c)
    }
}

impl From<MSG_MOVE_SET_FACING_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_SET_FACING_Server) -> Self {
        Self::MSG_MOVE_SET_FACING(c)
    }
}

impl From<MSG_MOVE_SET_PITCH_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_SET_PITCH_Server) -> Self {
        Self::MSG_MOVE_SET_PITCH(c)
    }
}

impl From<MSG_MOVE_HEARTBEAT_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_HEARTBEAT_Server) -> Self {
        Self::MSG_MOVE_HEARTBEAT(c)
    }
}

impl From<SMSG_TRIGGER_CINEMATIC> for ServerOpcodeMessage {
    fn from(c: SMSG_TRIGGER_CINEMATIC) -> Self {
        Self::SMSG_TRIGGER_CINEMATIC(c)
    }
}

impl From<SMSG_TUTORIAL_FLAGS> for ServerOpcodeMessage {
    fn from(c: SMSG_TUTORIAL_FLAGS) -> Self {
        Self::SMSG_TUTORIAL_FLAGS(c)
    }
}

impl From<SMSG_EMOTE> for ServerOpcodeMessage {
    fn from(c: SMSG_EMOTE) -> Self {
        Self::SMSG_EMOTE(c)
    }
}

impl From<SMSG_TEXT_EMOTE> for ServerOpcodeMessage {
    fn from(c: SMSG_TEXT_EMOTE) -> Self {
        Self::SMSG_TEXT_EMOTE(c)
    }
}

impl From<SMSG_INITIALIZE_FACTIONS> for ServerOpcodeMessage {
    fn from(c: SMSG_INITIALIZE_FACTIONS) -> Self {
        Self::SMSG_INITIALIZE_FACTIONS(c)
    }
}

impl From<SMSG_SET_FACTION_VISIBLE> for ServerOpcodeMessage {
    fn from(c: SMSG_SET_FACTION_VISIBLE) -> Self {
        Self::SMSG_SET_FACTION_VISIBLE(c)
    }
}

impl From<SMSG_SET_FACTION_STANDING> for ServerOpcodeMessage {
    fn from(c: SMSG_SET_FACTION_STANDING) -> Self {
        Self::SMSG_SET_FACTION_STANDING(c)
    }
}

impl From<SMSG_ACTION_BUTTONS> for ServerOpcodeMessage {
    fn from(c: SMSG_ACTION_BUTTONS) -> Self {
        Self::SMSG_ACTION_BUTTONS(c)
    }
}

impl From<SMSG_AI_REACTION> for ServerOpcodeMessage {
    fn from(c: SMSG_AI_REACTION) -> Self {
        Self::SMSG_AI_REACTION(c)
    }
}

impl From<SMSG_ATTACKSTART> for ServerOpcodeMessage {
    fn from(c: SMSG_ATTACKSTART) -> Self {
        Self::SMSG_ATTACKSTART(c)
    }
}

impl From<SMSG_ATTACKSTOP> for ServerOpcodeMessage {
    fn from(c: SMSG_ATTACKSTOP) -> Self {
        Self::SMSG_ATTACKSTOP(c)
    }
}

impl From<SMSG_ATTACKSWING_NOTINRANGE> for ServerOpcodeMessage {
    fn from(c: SMSG_ATTACKSWING_NOTINRANGE) -> Self {
        Self::SMSG_ATTACKSWING_NOTINRANGE(c)
    }
}

impl From<SMSG_ATTACKSWING_BADFACING> for ServerOpcodeMessage {
    fn from(c: SMSG_ATTACKSWING_BADFACING) -> Self {
        Self::SMSG_ATTACKSWING_BADFACING(c)
    }
}

impl From<SMSG_ATTACKSWING_NOTSTANDING> for ServerOpcodeMessage {
    fn from(c: SMSG_ATTACKSWING_NOTSTANDING) -> Self {
        Self::SMSG_ATTACKSWING_NOTSTANDING(c)
    }
}

impl From<SMSG_ATTACKSWING_DEADTARGET> for ServerOpcodeMessage {
    fn from(c: SMSG_ATTACKSWING_DEADTARGET) -> Self {
        Self::SMSG_ATTACKSWING_DEADTARGET(c)
    }
}

impl From<SMSG_ATTACKSWING_CANT_ATTACK> for ServerOpcodeMessage {
    fn from(c: SMSG_ATTACKSWING_CANT_ATTACK) -> Self {
        Self::SMSG_ATTACKSWING_CANT_ATTACK(c)
    }
}

impl From<SMSG_CANCEL_COMBAT> for ServerOpcodeMessage {
    fn from(c: SMSG_CANCEL_COMBAT) -> Self {
        Self::SMSG_CANCEL_COMBAT(c)
    }
}

impl From<SMSG_DUEL_REQUESTED> for ServerOpcodeMessage {
    fn from(c: SMSG_DUEL_REQUESTED) -> Self {
        Self::SMSG_DUEL_REQUESTED(c)
    }
}

impl From<SMSG_DUEL_OUTOFBOUNDS> for ServerOpcodeMessage {
    fn from(c: SMSG_DUEL_OUTOFBOUNDS) -> Self {
        Self::SMSG_DUEL_OUTOFBOUNDS(c)
    }
}

impl From<SMSG_DUEL_INBOUNDS> for ServerOpcodeMessage {
    fn from(c: SMSG_DUEL_INBOUNDS) -> Self {
        Self::SMSG_DUEL_INBOUNDS(c)
    }
}

impl From<SMSG_DUEL_COMPLETE> for ServerOpcodeMessage {
    fn from(c: SMSG_DUEL_COMPLETE) -> Self {
        Self::SMSG_DUEL_COMPLETE(c)
    }
}

impl From<SMSG_DUEL_WINNER> for ServerOpcodeMessage {
    fn from(c: SMSG_DUEL_WINNER) -> Self {
        Self::SMSG_DUEL_WINNER(c)
    }
}

impl From<SMSG_NOTIFICATION> for ServerOpcodeMessage {
    fn from(c: SMSG_NOTIFICATION) -> Self {
        Self::SMSG_NOTIFICATION(c)
    }
}

impl From<SMSG_LOG_XPGAIN> for ServerOpcodeMessage {
    fn from(c: SMSG_LOG_XPGAIN) -> Self {
        Self::SMSG_LOG_XPGAIN(c)
    }
}

impl From<SMSG_LEVELUP_INFO> for ServerOpcodeMessage {
    fn from(c: SMSG_LEVELUP_INFO) -> Self {
        Self::SMSG_LEVELUP_INFO(c)
    }
}

impl From<SMSG_PONG> for ServerOpcodeMessage {
    fn from(c: SMSG_PONG) -> Self {
        Self::SMSG_PONG(c)
    }
}

impl From<SMSG_AUTH_CHALLENGE> for ServerOpcodeMessage {
    fn from(c: SMSG_AUTH_CHALLENGE) -> Self {
        Self::SMSG_AUTH_CHALLENGE(c)
    }
}

impl From<SMSG_AUTH_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_AUTH_RESPONSE) -> Self {
        Self::SMSG_AUTH_RESPONSE(c)
    }
}

impl From<SMSG_EXPLORATION_EXPERIENCE> for ServerOpcodeMessage {
    fn from(c: SMSG_EXPLORATION_EXPERIENCE) -> Self {
        Self::SMSG_EXPLORATION_EXPERIENCE(c)
    }
}

impl From<SMSG_ENVIRONMENTALDAMAGELOG> for ServerOpcodeMessage {
    fn from(c: SMSG_ENVIRONMENTALDAMAGELOG) -> Self {
        Self::SMSG_ENVIRONMENTALDAMAGELOG(c)
    }
}

impl From<SMSG_GMTICKET_CREATE> for ServerOpcodeMessage {
    fn from(c: SMSG_GMTICKET_CREATE) -> Self {
        Self::SMSG_GMTICKET_CREATE(c)
    }
}

impl From<SMSG_ACCOUNT_DATA_TIMES> for ServerOpcodeMessage {
    fn from(c: SMSG_ACCOUNT_DATA_TIMES) -> Self {
        Self::SMSG_ACCOUNT_DATA_TIMES(c)
    }
}

impl From<SMSG_CHAT_WRONG_FACTION> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAT_WRONG_FACTION) -> Self {
        Self::SMSG_CHAT_WRONG_FACTION(c)
    }
}

impl From<SMSG_SET_REST_START> for ServerOpcodeMessage {
    fn from(c: SMSG_SET_REST_START) -> Self {
        Self::SMSG_SET_REST_START(c)
    }
}

impl From<SMSG_LOGIN_VERIFY_WORLD> for ServerOpcodeMessage {
    fn from(c: SMSG_LOGIN_VERIFY_WORLD) -> Self {
        Self::SMSG_LOGIN_VERIFY_WORLD(c)
    }
}

impl From<SMSG_BATTLEFIELD_LIST> for ServerOpcodeMessage {
    fn from(c: SMSG_BATTLEFIELD_LIST) -> Self {
        Self::SMSG_BATTLEFIELD_LIST(c)
    }
}

impl From<SMSG_ZONE_UNDER_ATTACK> for ServerOpcodeMessage {
    fn from(c: SMSG_ZONE_UNDER_ATTACK) -> Self {
        Self::SMSG_ZONE_UNDER_ATTACK(c)
    }
}

impl From<MSG_AUCTION_HELLO_Server> for ServerOpcodeMessage {
    fn from(c: MSG_AUCTION_HELLO_Server) -> Self {
        Self::MSG_AUCTION_HELLO(c)
    }
}

impl From<SMSG_AUCTION_LIST_RESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_AUCTION_LIST_RESULT) -> Self {
        Self::SMSG_AUCTION_LIST_RESULT(c)
    }
}

impl From<SMSG_AUCTION_OWNER_LIST_RESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_AUCTION_OWNER_LIST_RESULT) -> Self {
        Self::SMSG_AUCTION_OWNER_LIST_RESULT(c)
    }
}

impl From<SMSG_AUCTION_BIDDER_NOTIFICATION> for ServerOpcodeMessage {
    fn from(c: SMSG_AUCTION_BIDDER_NOTIFICATION) -> Self {
        Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c)
    }
}

impl From<SMSG_AUCTION_OWNER_NOTIFICATION> for ServerOpcodeMessage {
    fn from(c: SMSG_AUCTION_OWNER_NOTIFICATION) -> Self {
        Self::SMSG_AUCTION_OWNER_NOTIFICATION(c)
    }
}

impl From<SMSG_PROCRESIST> for ServerOpcodeMessage {
    fn from(c: SMSG_PROCRESIST) -> Self {
        Self::SMSG_PROCRESIST(c)
    }
}

impl From<SMSG_AUCTION_BIDDER_LIST_RESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_AUCTION_BIDDER_LIST_RESULT) -> Self {
        Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c)
    }
}

impl From<SMSG_AUCTION_REMOVED_NOTIFICATION> for ServerOpcodeMessage {
    fn from(c: SMSG_AUCTION_REMOVED_NOTIFICATION) -> Self {
        Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c)
    }
}

impl From<SMSG_SERVER_MESSAGE> for ServerOpcodeMessage {
    fn from(c: SMSG_SERVER_MESSAGE) -> Self {
        Self::SMSG_SERVER_MESSAGE(c)
    }
}

impl From<SMSG_STANDSTATE_UPDATE> for ServerOpcodeMessage {
    fn from(c: SMSG_STANDSTATE_UPDATE) -> Self {
        Self::SMSG_STANDSTATE_UPDATE(c)
    }
}

impl From<SMSG_SET_FORCED_REACTIONS> for ServerOpcodeMessage {
    fn from(c: SMSG_SET_FORCED_REACTIONS) -> Self {
        Self::SMSG_SET_FORCED_REACTIONS(c)
    }
}

impl From<SMSG_CHAT_PLAYER_NOT_FOUND> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAT_PLAYER_NOT_FOUND) -> Self {
        Self::SMSG_CHAT_PLAYER_NOT_FOUND(c)
    }
}

impl From<SMSG_DUEL_COUNTDOWN> for ServerOpcodeMessage {
    fn from(c: SMSG_DUEL_COUNTDOWN) -> Self {
        Self::SMSG_DUEL_COUNTDOWN(c)
    }
}

impl From<SMSG_DURABILITY_DAMAGE_DEATH> for ServerOpcodeMessage {
    fn from(c: SMSG_DURABILITY_DAMAGE_DEATH) -> Self {
        Self::SMSG_DURABILITY_DAMAGE_DEATH(c)
    }
}

impl From<SMSG_UPDATE_WORLD_STATE> for ServerOpcodeMessage {
    fn from(c: SMSG_UPDATE_WORLD_STATE) -> Self {
        Self::SMSG_UPDATE_WORLD_STATE(c)
    }
}

impl From<SMSG_CHAR_RENAME> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAR_RENAME) -> Self {
        Self::SMSG_CHAR_RENAME(c)
    }
}

impl From<MSG_BATTLEGROUND_PLAYER_POSITIONS_Server> for ServerOpcodeMessage {
    fn from(c: MSG_BATTLEGROUND_PLAYER_POSITIONS_Server) -> Self {
        Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c)
    }
}

impl From<SMSG_BATTLEGROUND_PLAYER_JOINED> for ServerOpcodeMessage {
    fn from(c: SMSG_BATTLEGROUND_PLAYER_JOINED) -> Self {
        Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c)
    }
}

impl From<SMSG_BATTLEGROUND_PLAYER_LEFT> for ServerOpcodeMessage {
    fn from(c: SMSG_BATTLEGROUND_PLAYER_LEFT) -> Self {
        Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c)
    }
}

impl From<SMSG_CHAT_RESTRICTED> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAT_RESTRICTED) -> Self {
        Self::SMSG_CHAT_RESTRICTED(c)
    }
}

impl From<SMSG_GM_TICKET_STATUS_UPDATE> for ServerOpcodeMessage {
    fn from(c: SMSG_GM_TICKET_STATUS_UPDATE) -> Self {
        Self::SMSG_GM_TICKET_STATUS_UPDATE(c)
    }
}

impl From<SMSG_DEFENSE_MESSAGE> for ServerOpcodeMessage {
    fn from(c: SMSG_DEFENSE_MESSAGE) -> Self {
        Self::SMSG_DEFENSE_MESSAGE(c)
    }
}

impl From<MSG_MOVE_START_ASCEND_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_ASCEND_Server) -> Self {
        Self::MSG_MOVE_START_ASCEND(c)
    }
}

impl From<MSG_MOVE_STOP_ASCEND_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_ASCEND_Server) -> Self {
        Self::MSG_MOVE_STOP_ASCEND(c)
    }
}

impl From<SMSG_REALM_SPLIT> for ServerOpcodeMessage {
    fn from(c: SMSG_REALM_SPLIT) -> Self {
        Self::SMSG_REALM_SPLIT(c)
    }
}

impl From<SMSG_TIME_SYNC_REQ> for ServerOpcodeMessage {
    fn from(c: SMSG_TIME_SYNC_REQ) -> Self {
        Self::SMSG_TIME_SYNC_REQ(c)
    }
}

impl From<MSG_MOVE_START_DESCEND_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_DESCEND_Server) -> Self {
        Self::MSG_MOVE_START_DESCEND(c)
    }
}

