use crate::MessageBody;
use crate::{ServerMessageWrite, ClientMessageWrite};
use wow_srp::header_crypto::{Decrypter, Encrypter};

#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use crate::world::v1::v12::MSG_MOVE_START_FORWARD;
use crate::world::v1::v12::MSG_MOVE_START_BACKWARD;
use crate::world::v1::v12::MSG_MOVE_STOP;
use crate::world::v1::v12::MSG_MOVE_START_STRAFE_LEFT;
use crate::world::v1::v12::MSG_MOVE_START_STRAFE_RIGHT;
use crate::world::v1::v12::MSG_MOVE_STOP_STRAFE;
use crate::world::v1::v12::MSG_MOVE_JUMP;
use crate::world::v1::v12::MSG_MOVE_START_TURN_LEFT;
use crate::world::v1::v12::MSG_MOVE_START_TURN_RIGHT;
use crate::world::v1::v12::MSG_MOVE_STOP_TURN;
use crate::world::v1::v12::MSG_MOVE_START_PITCH_UP;
use crate::world::v1::v12::MSG_MOVE_START_PITCH_DOWN;
use crate::world::v1::v12::MSG_MOVE_STOP_PITCH;
use crate::world::v1::v12::MSG_MOVE_SET_RUN_MODE;
use crate::world::v1::v12::MSG_MOVE_SET_WALK_MODE;
use crate::world::v1::v12::MSG_MOVE_TELEPORT_ACK;
use crate::world::v1::v12::MSG_MOVE_FALL_LAND;
use crate::world::v1::v12::MSG_MOVE_START_SWIM;
use crate::world::v1::v12::MSG_MOVE_STOP_SWIM;
use crate::world::v1::v12::MSG_MOVE_SET_FACING;
use crate::world::v1::v12::MSG_MOVE_SET_PITCH;
use crate::world::v1::v12::MSG_MOVE_WORLDPORT_ACK;
use crate::world::v1::v12::MSG_MOVE_HEARTBEAT;
use crate::world::v1::v12::MSG_PETITION_DECLINE;
use crate::world::v1::v12::MSG_TABARDVENDOR_ACTIVATE;
use crate::world::v1::v12::{MSG_QUEST_PUSH_RESULT, MSG_QUEST_PUSH_RESULTError};
use crate::world::v1::v12::{MSG_PETITION_RENAME, MSG_PETITION_RENAMEError};
use crate::world::v1::v12::{CMSG_WORLD_TELEPORT, CMSG_WORLD_TELEPORTError};
use crate::world::v1::v12::{CMSG_CHAR_CREATE, CMSG_CHAR_CREATEError};
use crate::world::v1::v2::CMSG_CHAR_ENUM;
use crate::world::v1::v12::CMSG_CHAR_DELETE;
use crate::world::v1::v12::CMSG_PLAYER_LOGIN;
use crate::world::v1::v12::CMSG_PLAYER_LOGOUT;
use crate::world::v1::v12::CMSG_LOGOUT_REQUEST;
use crate::world::v1::v12::CMSG_LOGOUT_CANCEL;
use crate::world::v1::v12::CMSG_NAME_QUERY;
use crate::world::v1::v12::CMSG_PET_NAME_QUERY;
use crate::world::v1::v12::CMSG_GUILD_QUERY;
use crate::world::v1::v12::CMSG_ITEM_QUERY_SINGLE;
use crate::world::v1::v12::CMSG_PAGE_TEXT_QUERY;
use crate::world::v1::v12::CMSG_QUEST_QUERY;
use crate::world::v1::v12::CMSG_GAMEOBJECT_QUERY;
use crate::world::v1::v12::CMSG_CREATURE_QUERY;
use crate::world::v1::v12::{CMSG_WHO, CMSG_WHOError};
use crate::world::v1::v12::{CMSG_WHOIS, CMSG_WHOISError};
use crate::world::v1::v12::CMSG_FRIEND_LIST;
use crate::world::v1::v12::{CMSG_ADD_FRIEND, CMSG_ADD_FRIENDError};
use crate::world::v1::v12::CMSG_DEL_FRIEND;
use crate::world::v1::v12::{CMSG_ADD_IGNORE, CMSG_ADD_IGNOREError};
use crate::world::v1::v12::CMSG_DEL_IGNORE;
use crate::world::v1::v12::{CMSG_GROUP_INVITE, CMSG_GROUP_INVITEError};
use crate::world::v1::v12::CMSG_GROUP_ACCEPT;
use crate::world::v1::v12::CMSG_GROUP_DECLINE;
use crate::world::v1::v12::{CMSG_GROUP_UNINVITE, CMSG_GROUP_UNINVITEError};
use crate::world::v1::v12::CMSG_GROUP_UNINVITE_GUID;
use crate::world::v1::v12::CMSG_GROUP_SET_LEADER;
use crate::world::v1::v12::{CMSG_LOOT_METHOD, CMSG_LOOT_METHODError};
use crate::world::v1::v12::CMSG_GROUP_DISBAND;
use crate::world::v1::v12::{CMSG_GUILD_CREATE, CMSG_GUILD_CREATEError};
use crate::world::v1::v12::{CMSG_GUILD_INVITE, CMSG_GUILD_INVITEError};
use crate::world::v1::v12::CMSG_GUILD_ACCEPT;
use crate::world::v1::v12::CMSG_GUILD_DECLINE;
use crate::world::v1::v12::CMSG_GUILD_INFO;
use crate::world::v1::v12::CMSG_GUILD_ROSTER;
use crate::world::v1::v12::{CMSG_GUILD_PROMOTE, CMSG_GUILD_PROMOTEError};
use crate::world::v1::v12::{CMSG_GUILD_DEMOTE, CMSG_GUILD_DEMOTEError};
use crate::world::v1::v12::CMSG_GUILD_LEAVE;
use crate::world::v1::v12::{CMSG_GUILD_REMOVE, CMSG_GUILD_REMOVEError};
use crate::world::v1::v12::CMSG_GUILD_DISBAND;
use crate::world::v1::v12::{CMSG_GUILD_LEADER, CMSG_GUILD_LEADERError};
use crate::world::v1::v12::{CMSG_GUILD_MOTD, CMSG_GUILD_MOTDError};
use crate::world::v1::v12::{CMSG_MESSAGECHAT, CMSG_MESSAGECHATError};
use crate::world::v1::v12::{CMSG_JOIN_CHANNEL, CMSG_JOIN_CHANNELError};
use crate::world::v1::v12::{CMSG_LEAVE_CHANNEL, CMSG_LEAVE_CHANNELError};
use crate::world::v1::v12::{CMSG_CHANNEL_LIST, CMSG_CHANNEL_LISTError};
use crate::world::v1::v12::{CMSG_CHANNEL_PASSWORD, CMSG_CHANNEL_PASSWORDError};
use crate::world::v1::v12::{CMSG_CHANNEL_SET_OWNER, CMSG_CHANNEL_SET_OWNERError};
use crate::world::v1::v12::{CMSG_CHANNEL_OWNER, CMSG_CHANNEL_OWNERError};
use crate::world::v1::v12::{CMSG_CHANNEL_MODERATOR, CMSG_CHANNEL_MODERATORError};
use crate::world::v1::v12::{CMSG_CHANNEL_UNMODERATOR, CMSG_CHANNEL_UNMODERATORError};
use crate::world::v1::v12::{CMSG_CHANNEL_MUTE, CMSG_CHANNEL_MUTEError};
use crate::world::v1::v12::{CMSG_CHANNEL_UNMUTE, CMSG_CHANNEL_UNMUTEError};
use crate::world::v1::v12::{CMSG_CHANNEL_INVITE, CMSG_CHANNEL_INVITEError};
use crate::world::v1::v12::{CMSG_CHANNEL_KICK, CMSG_CHANNEL_KICKError};
use crate::world::v1::v12::{CMSG_CHANNEL_BAN, CMSG_CHANNEL_BANError};
use crate::world::v1::v12::{CMSG_CHANNEL_UNBAN, CMSG_CHANNEL_UNBANError};
use crate::world::v1::v12::{CMSG_CHANNEL_ANNOUNCEMENTS, CMSG_CHANNEL_ANNOUNCEMENTSError};
use crate::world::v1::v12::{CMSG_CHANNEL_MODERATE, CMSG_CHANNEL_MODERATEError};
use crate::world::v1::v12::{SMSG_UPDATE_OBJECT, SMSG_UPDATE_OBJECTError};
use crate::world::v1::v12::{CMSG_USE_ITEM, CMSG_USE_ITEMError};
use crate::world::v1::v12::CMSG_OPEN_ITEM;
use crate::world::v1::v12::CMSG_READ_ITEM;
use crate::world::v1::v12::CMSG_GAMEOBJ_USE;
use crate::world::v1::v12::CMSG_AREATRIGGER;
use crate::world::v1::v12::CMSG_MOVE_SET_RAW_POSITION;
use crate::world::v1::v12::CMSG_FORCE_RUN_SPEED_CHANGE_ACK;
use crate::world::v1::v12::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK;
use crate::world::v1::v12::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK;
use crate::world::v1::v12::CMSG_FORCE_MOVE_ROOT_ACK;
use crate::world::v1::v12::CMSG_FORCE_MOVE_UNROOT_ACK;
use crate::world::v1::v12::CMSG_MOVE_KNOCK_BACK_ACK;
use crate::world::v1::v12::CMSG_MOVE_HOVER_ACK;
use crate::world::v1::v12::CMSG_NEXT_CINEMATIC_CAMERA;
use crate::world::v1::v12::CMSG_COMPLETE_CINEMATIC;
use crate::world::v1::v12::CMSG_TUTORIAL_FLAG;
use crate::world::v1::v12::CMSG_TUTORIAL_CLEAR;
use crate::world::v1::v12::CMSG_TUTORIAL_RESET;
use crate::world::v1::v12::{CMSG_STANDSTATECHANGE, CMSG_STANDSTATECHANGEError};
use crate::world::v1::v12::{CMSG_EMOTE, CMSG_EMOTEError};
use crate::world::v1::v12::{CMSG_TEXT_EMOTE, CMSG_TEXT_EMOTEError};
use crate::world::v1::v12::CMSG_AUTOSTORE_LOOT_ITEM;
use crate::world::v1::v12::CMSG_AUTOEQUIP_ITEM;
use crate::world::v1::v12::CMSG_AUTOSTORE_BAG_ITEM;
use crate::world::v1::v12::CMSG_SWAP_ITEM;
use crate::world::v1::v12::CMSG_SWAP_INV_ITEM;
use crate::world::v1::v12::CMSG_SPLIT_ITEM;
use crate::world::v1::v12::CMSG_AUTOEQUIP_ITEM_SLOT;
use crate::world::v1::v12::CMSG_DESTROYITEM;
use crate::world::v1::v12::CMSG_INSPECT;
use crate::world::v1::v12::CMSG_INITIATE_TRADE;
use crate::world::v1::v12::CMSG_BEGIN_TRADE;
use crate::world::v1::v12::CMSG_BUSY_TRADE;
use crate::world::v1::v12::CMSG_IGNORE_TRADE;
use crate::world::v1::v12::CMSG_ACCEPT_TRADE;
use crate::world::v1::v12::CMSG_UNACCEPT_TRADE;
use crate::world::v1::v12::CMSG_CANCEL_TRADE;
use crate::world::v1::v12::CMSG_SET_TRADE_ITEM;
use crate::world::v1::v12::CMSG_CLEAR_TRADE_ITEM;
use crate::world::v1::v12::CMSG_SET_TRADE_GOLD;
use crate::world::v1::v12::CMSG_SET_FACTION_ATWAR;
use crate::world::v1::v12::CMSG_SET_ACTION_BUTTON;
use crate::world::v1::v12::{CMSG_CAST_SPELL, CMSG_CAST_SPELLError};
use crate::world::v1::v12::CMSG_CANCEL_CAST;
use crate::world::v1::v12::CMSG_CANCEL_AURA;
use crate::world::v1::v12::CMSG_CANCEL_CHANNELLING;
use crate::world::v1::v12::CMSG_SET_SELECTION;
use crate::world::v1::v12::CMSG_SET_TARGET_OBSOLETE;
use crate::world::v1::v12::CMSG_ATTACKSWING;
use crate::world::v1::v12::CMSG_ATTACKSTOP;
use crate::world::v1::v12::CMSG_REPOP_REQUEST;
use crate::world::v1::v12::CMSG_RESURRECT_RESPONSE;
use crate::world::v1::v12::CMSG_LOOT;
use crate::world::v1::v12::CMSG_LOOT_MONEY;
use crate::world::v1::v12::CMSG_LOOT_RELEASE;
use crate::world::v1::v12::CMSG_DUEL_ACCEPTED;
use crate::world::v1::v12::CMSG_DUEL_CANCELLED;
use crate::world::v1::v12::CMSG_MOUNTSPECIAL_ANIM;
use crate::world::v1::v12::CMSG_PET_SET_ACTION;
use crate::world::v1::v12::CMSG_PET_ACTION;
use crate::world::v1::v12::CMSG_PET_ABANDON;
use crate::world::v1::v12::{CMSG_PET_RENAME, CMSG_PET_RENAMEError};
use crate::world::v1::v12::CMSG_GOSSIP_HELLO;
use crate::world::v1::v12::{CMSG_GOSSIP_SELECT_OPTION, CMSG_GOSSIP_SELECT_OPTIONError};
use crate::world::v1::v12::CMSG_NPC_TEXT_QUERY;
use crate::world::v1::v12::CMSG_QUESTGIVER_STATUS_QUERY;
use crate::world::v1::v12::CMSG_QUESTGIVER_HELLO;
use crate::world::v1::v12::CMSG_QUESTGIVER_QUERY_QUEST;
use crate::world::v1::v12::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH;
use crate::world::v1::v12::CMSG_QUESTGIVER_ACCEPT_QUEST;
use crate::world::v1::v12::CMSG_QUESTGIVER_COMPLETE_QUEST;
use crate::world::v1::v12::CMSG_QUESTGIVER_REQUEST_REWARD;
use crate::world::v1::v12::CMSG_QUESTGIVER_CHOOSE_REWARD;
use crate::world::v1::v12::CMSG_QUESTGIVER_CANCEL;
use crate::world::v1::v12::CMSG_QUESTLOG_SWAP_QUEST;
use crate::world::v1::v12::CMSG_QUESTLOG_REMOVE_QUEST;
use crate::world::v1::v12::CMSG_QUEST_CONFIRM_ACCEPT;
use crate::world::v1::v12::CMSG_PUSHQUESTTOPARTY;
use crate::world::v1::v12::CMSG_LIST_INVENTORY;
use crate::world::v1::v12::CMSG_SELL_ITEM;
use crate::world::v1::v12::CMSG_BUY_ITEM;
use crate::world::v1::v12::CMSG_BUY_ITEM_IN_SLOT;
use crate::world::v1::v12::CMSG_TAXINODE_STATUS_QUERY;
use crate::world::v1::v12::CMSG_TAXIQUERYAVAILABLENODES;
use crate::world::v1::v12::CMSG_ACTIVATETAXI;
use crate::world::v1::v12::CMSG_TRAINER_LIST;
use crate::world::v1::v12::CMSG_TRAINER_BUY_SPELL;
use crate::world::v1::v12::CMSG_BINDER_ACTIVATE;
use crate::world::v1::v12::CMSG_BANKER_ACTIVATE;
use crate::world::v1::v12::CMSG_BUY_BANK_SLOT;
use crate::world::v1::v12::CMSG_PETITION_SHOWLIST;
use crate::world::v1::v12::{CMSG_PETITION_BUY, CMSG_PETITION_BUYError};
use crate::world::v1::v12::CMSG_PETITION_SHOW_SIGNATURES;
use crate::world::v1::v12::CMSG_PETITION_SIGN;
use crate::world::v1::v12::CMSG_OFFER_PETITION;
use crate::world::v1::v12::CMSG_TURN_IN_PETITION;
use crate::world::v1::v12::CMSG_PETITION_QUERY;
use crate::world::v1::v12::{CMSG_BUG, CMSG_BUGError};
use crate::world::v1::v12::CMSG_PLAYED_TIME;
use crate::world::v1::v12::CMSG_QUERY_TIME;
use crate::world::v1::v12::CMSG_RECLAIM_CORPSE;
use crate::world::v1::v12::CMSG_WRAP_ITEM;
use crate::world::v1::v12::MSG_MINIMAP_PING_Client;
use crate::world::v1::v12::CMSG_PING;
use crate::world::v1::v12::{CMSG_SETSHEATHED, CMSG_SETSHEATHEDError};
use crate::world::v1::v12::{CMSG_AUTH_SESSION, CMSG_AUTH_SESSIONError};
use crate::world::v1::v12::CMSG_PET_CAST_SPELL;
use crate::world::v1::v12::MSG_SAVE_GUILD_EMBLEM_Client;
use crate::world::v1::v12::CMSG_ZONEUPDATE;
use crate::world::v1::v12::MSG_RANDOM_ROLL_Client;
use crate::world::v1::v12::MSG_LOOKING_FOR_GROUP_Client;
use crate::world::v1::v12::CMSG_UNLEARN_SKILL;
use crate::world::v1::v12::{CMSG_GMTICKET_CREATE, CMSG_GMTICKET_CREATEError};
use crate::world::v1::v12::{CMSG_GMTICKET_UPDATETEXT, CMSG_GMTICKET_UPDATETEXTError};
use crate::world::v1::v12::CMSG_REQUEST_ACCOUNT_DATA;
use crate::world::v1::v12::CMSG_GMTICKET_GETTICKET;
use crate::world::v1::v12::MSG_CORPSE_QUERY_Client;
use crate::world::v1::v12::CMSG_GMTICKET_DELETETICKET;
use crate::world::v1::v12::CMSG_GMTICKET_SYSTEMSTATUS;
use crate::world::v1::v12::CMSG_SPIRIT_HEALER_ACTIVATE;
use crate::world::v1::v12::CMSG_CHAT_IGNORED;
use crate::world::v1::v12::{CMSG_GUILD_RANK, CMSG_GUILD_RANKError};
use crate::world::v1::v12::{CMSG_GUILD_ADD_RANK, CMSG_GUILD_ADD_RANKError};
use crate::world::v1::v12::CMSG_GUILD_DEL_RANK;
use crate::world::v1::v12::{CMSG_GUILD_SET_PUBLIC_NOTE, CMSG_GUILD_SET_PUBLIC_NOTEError};
use crate::world::v1::v12::{CMSG_GUILD_SET_OFFICER_NOTE, CMSG_GUILD_SET_OFFICER_NOTEError};
use crate::world::v1::v12::{CMSG_SEND_MAIL, CMSG_SEND_MAILError};
use crate::world::v1::v12::CMSG_GET_MAIL_LIST;
use crate::world::v1::v12::{CMSG_BATTLEFIELD_LIST, CMSG_BATTLEFIELD_LISTError};
use crate::world::v1::v12::{CMSG_BATTLEFIELD_JOIN, CMSG_BATTLEFIELD_JOINError};
use crate::world::v1::v12::CMSG_ITEM_TEXT_QUERY;
use crate::world::v1::v12::CMSG_MAIL_TAKE_MONEY;
use crate::world::v1::v12::CMSG_MAIL_TAKE_ITEM;
use crate::world::v1::v12::CMSG_MAIL_MARK_AS_READ;
use crate::world::v1::v12::CMSG_MAIL_RETURN_TO_SENDER;
use crate::world::v1::v12::CMSG_MAIL_DELETE;
use crate::world::v1::v12::CMSG_MAIL_CREATE_TEXT_ITEM;
use crate::world::v1::v12::CMSG_LEARN_TALENT;
use crate::world::v1::v12::CMSG_TOGGLE_PVP;
use crate::world::v1::v12::MSG_AUCTION_HELLO_Client;
use crate::world::v1::v12::CMSG_AUCTION_SELL_ITEM;
use crate::world::v1::v12::CMSG_AUCTION_REMOVE_ITEM;
use crate::world::v1::v12::{CMSG_AUCTION_LIST_ITEMS, CMSG_AUCTION_LIST_ITEMSError};
use crate::world::v1::v12::CMSG_AUCTION_LIST_OWNER_ITEMS;
use crate::world::v1::v12::CMSG_AUCTION_PLACE_BID;
use crate::world::v1::v12::CMSG_AUCTION_LIST_BIDDER_ITEMS;
use crate::world::v1::v12::CMSG_SET_AMMO;
use crate::world::v1::v12::CMSG_SET_ACTIVE_MOVER;
use crate::world::v1::v12::CMSG_PET_CANCEL_AURA;
use crate::world::v1::v12::CMSG_CANCEL_AUTO_REPEAT_SPELL;
use crate::world::v1::v12::MSG_LIST_STABLED_PETS_Client;
use crate::world::v1::v12::CMSG_STABLE_PET;
use crate::world::v1::v12::CMSG_UNSTABLE_PET;
use crate::world::v1::v12::CMSG_BUY_STABLE_SLOT;
use crate::world::v1::v12::CMSG_STABLE_SWAP_PET;
use crate::world::v1::v12::CMSG_REQUEST_PET_INFO;
use crate::world::v1::v12::{CMSG_FAR_SIGHT, CMSG_FAR_SIGHTError};
use crate::world::v1::v12::{CMSG_GROUP_CHANGE_SUB_GROUP, CMSG_GROUP_CHANGE_SUB_GROUPError};
use crate::world::v1::v12::CMSG_REQUEST_PARTY_MEMBER_STATS;
use crate::world::v1::v12::{CMSG_GROUP_SWAP_SUB_GROUP, CMSG_GROUP_SWAP_SUB_GROUPError};
use crate::world::v1::v12::CMSG_AUTOSTORE_BANK_ITEM;
use crate::world::v1::v12::CMSG_AUTOBANK_ITEM;
use crate::world::v1::v12::MSG_QUERY_NEXT_MAIL_TIME_Client;
use crate::world::v1::v12::CMSG_GROUP_RAID_CONVERT;
use crate::world::v1::v12::CMSG_GROUP_ASSISTANT_LEADER;
use crate::world::v1::v12::{CMSG_BUYBACK_ITEM, CMSG_BUYBACK_ITEMError};
use crate::world::v1::v12::CMSG_MEETINGSTONE_JOIN;
use crate::world::v1::v12::CMSG_MEETINGSTONE_LEAVE;
use crate::world::v1::v12::CMSG_MEETINGSTONE_INFO;
use crate::world::v1::v12::CMSG_CANCEL_GROWTH_AURA;
use crate::world::v1::v12::{CMSG_LOOT_ROLL, CMSG_LOOT_ROLLError};
use crate::world::v1::v12::CMSG_LOOT_MASTER_GIVE;
use crate::world::v1::v12::CMSG_REPAIR_ITEM;
use crate::world::v1::v12::MSG_TALENT_WIPE_CONFIRM_Client;
use crate::world::v1::v12::CMSG_SUMMON_RESPONSE;
use crate::world::v1::v12::CMSG_SELF_RES;
use crate::world::v1::v12::CMSG_TOGGLE_HELM;
use crate::world::v1::v12::CMSG_TOGGLE_CLOAK;
use crate::world::v1::v12::CMSG_SET_ACTIONBAR_TOGGLES;
use crate::world::v1::v12::CMSG_ITEM_NAME_QUERY;
use crate::world::v1::v12::{CMSG_CHAR_RENAME, CMSG_CHAR_RENAMEError};
use crate::world::v1::v12::CMSG_MOVE_SPLINE_DONE;
use crate::world::v1::v12::CMSG_MOVE_FALL_RESET;
use crate::world::v1::v12::CMSG_REQUEST_RAID_INFO;
use crate::world::v1::v12::CMSG_MOVE_TIME_SKIPPED;
use crate::world::v1::v12::CMSG_MOVE_FEATHER_FALL_ACK;
use crate::world::v1::v12::CMSG_MOVE_WATER_WALK_ACK;
use crate::world::v1::v12::CMSG_MOVE_NOT_ACTIVE_MOVER;
use crate::world::v1::v12::CMSG_BATTLEFIELD_STATUS;
use crate::world::v1::v12::{CMSG_BATTLEFIELD_PORT, CMSG_BATTLEFIELD_PORTError};
use crate::world::v1::v12::MSG_INSPECT_HONOR_STATS_Client;
use crate::world::v1::v12::CMSG_BATTLEMASTER_HELLO;
use crate::world::v1::v12::CMSG_FORCE_WALK_SPEED_CHANGE_ACK;
use crate::world::v1::v12::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK;
use crate::world::v1::v12::CMSG_FORCE_TURN_RATE_CHANGE_ACK;
use crate::world::v1::v12::MSG_PVP_LOG_DATA_Client;
use crate::world::v1::v12::CMSG_LEAVE_BATTLEFIELD;
use crate::world::v1::v12::CMSG_AREA_SPIRIT_HEALER_QUERY;
use crate::world::v1::v12::CMSG_AREA_SPIRIT_HEALER_QUEUE;
use crate::world::v1::v12::MSG_BATTLEGROUND_PLAYER_POSITIONS_Client;
use crate::world::v1::v12::CMSG_PET_STOP_ATTACK;
use crate::world::v1::v12::{CMSG_BATTLEMASTER_JOIN, CMSG_BATTLEMASTER_JOINError};
use crate::world::v1::v12::CMSG_PET_UNLEARN;
use crate::world::v1::v12::CMSG_PET_SPELL_AUTOCAST;
use crate::world::v1::v12::{CMSG_GUILD_INFO_TEXT, CMSG_GUILD_INFO_TEXTError};
use crate::world::v1::v12::CMSG_ACTIVATETAXIEXPRESS;
use crate::world::v1::v12::CMSG_SET_FACTION_INACTIVE;
use crate::world::v1::v12::CMSG_SET_WATCHED_FACTION;
use crate::world::v1::v12::CMSG_RESET_INSTANCES;
use crate::world::v1::v12::{MSG_RAID_TARGET_UPDATE_Client, MSG_RAID_TARGET_UPDATE_ClientError};
use crate::world::v1::v12::MSG_RAID_READY_CHECK_Client;
use crate::world::v1::v12::{CMSG_GMSURVEY_SUBMIT, CMSG_GMSURVEY_SUBMITError};

#[derive(Debug)]
pub enum ClientOpcode {
    MSG_MOVE_START_FORWARD,
    MSG_MOVE_START_BACKWARD,
    MSG_MOVE_STOP,
    MSG_MOVE_START_STRAFE_LEFT,
    MSG_MOVE_START_STRAFE_RIGHT,
    MSG_MOVE_STOP_STRAFE,
    MSG_MOVE_JUMP,
    MSG_MOVE_START_TURN_LEFT,
    MSG_MOVE_START_TURN_RIGHT,
    MSG_MOVE_STOP_TURN,
    MSG_MOVE_START_PITCH_UP,
    MSG_MOVE_START_PITCH_DOWN,
    MSG_MOVE_STOP_PITCH,
    MSG_MOVE_SET_RUN_MODE,
    MSG_MOVE_SET_WALK_MODE,
    MSG_MOVE_TELEPORT_ACK,
    MSG_MOVE_FALL_LAND,
    MSG_MOVE_START_SWIM,
    MSG_MOVE_STOP_SWIM,
    MSG_MOVE_SET_FACING,
    MSG_MOVE_SET_PITCH,
    MSG_MOVE_WORLDPORT_ACK,
    MSG_MOVE_HEARTBEAT,
    MSG_PETITION_DECLINE,
    MSG_TABARDVENDOR_ACTIVATE,
    MSG_QUEST_PUSH_RESULT,
    MSG_PETITION_RENAME,
    CMSG_WORLD_TELEPORT,
    CMSG_CHAR_CREATE,
    CMSG_CHAR_ENUM,
    CMSG_CHAR_DELETE,
    CMSG_PLAYER_LOGIN,
    CMSG_PLAYER_LOGOUT,
    CMSG_LOGOUT_REQUEST,
    CMSG_LOGOUT_CANCEL,
    CMSG_NAME_QUERY,
    CMSG_PET_NAME_QUERY,
    CMSG_GUILD_QUERY,
    CMSG_ITEM_QUERY_SINGLE,
    CMSG_PAGE_TEXT_QUERY,
    CMSG_QUEST_QUERY,
    CMSG_GAMEOBJECT_QUERY,
    CMSG_CREATURE_QUERY,
    CMSG_WHO,
    CMSG_WHOIS,
    CMSG_FRIEND_LIST,
    CMSG_ADD_FRIEND,
    CMSG_DEL_FRIEND,
    CMSG_ADD_IGNORE,
    CMSG_DEL_IGNORE,
    CMSG_GROUP_INVITE,
    CMSG_GROUP_ACCEPT,
    CMSG_GROUP_DECLINE,
    CMSG_GROUP_UNINVITE,
    CMSG_GROUP_UNINVITE_GUID,
    CMSG_GROUP_SET_LEADER,
    CMSG_LOOT_METHOD,
    CMSG_GROUP_DISBAND,
    CMSG_GUILD_CREATE,
    CMSG_GUILD_INVITE,
    CMSG_GUILD_ACCEPT,
    CMSG_GUILD_DECLINE,
    CMSG_GUILD_INFO,
    CMSG_GUILD_ROSTER,
    CMSG_GUILD_PROMOTE,
    CMSG_GUILD_DEMOTE,
    CMSG_GUILD_LEAVE,
    CMSG_GUILD_REMOVE,
    CMSG_GUILD_DISBAND,
    CMSG_GUILD_LEADER,
    CMSG_GUILD_MOTD,
    CMSG_MESSAGECHAT,
    CMSG_JOIN_CHANNEL,
    CMSG_LEAVE_CHANNEL,
    CMSG_CHANNEL_LIST,
    CMSG_CHANNEL_PASSWORD,
    CMSG_CHANNEL_SET_OWNER,
    CMSG_CHANNEL_OWNER,
    CMSG_CHANNEL_MODERATOR,
    CMSG_CHANNEL_UNMODERATOR,
    CMSG_CHANNEL_MUTE,
    CMSG_CHANNEL_UNMUTE,
    CMSG_CHANNEL_INVITE,
    CMSG_CHANNEL_KICK,
    CMSG_CHANNEL_BAN,
    CMSG_CHANNEL_UNBAN,
    CMSG_CHANNEL_ANNOUNCEMENTS,
    CMSG_CHANNEL_MODERATE,
    SMSG_UPDATE_OBJECT,
    CMSG_USE_ITEM,
    CMSG_OPEN_ITEM,
    CMSG_READ_ITEM,
    CMSG_GAMEOBJ_USE,
    CMSG_AREATRIGGER,
    CMSG_MOVE_SET_RAW_POSITION,
    CMSG_FORCE_RUN_SPEED_CHANGE_ACK,
    CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK,
    CMSG_FORCE_SWIM_SPEED_CHANGE_ACK,
    CMSG_FORCE_MOVE_ROOT_ACK,
    CMSG_FORCE_MOVE_UNROOT_ACK,
    CMSG_MOVE_KNOCK_BACK_ACK,
    CMSG_MOVE_HOVER_ACK,
    CMSG_NEXT_CINEMATIC_CAMERA,
    CMSG_COMPLETE_CINEMATIC,
    CMSG_TUTORIAL_FLAG,
    CMSG_TUTORIAL_CLEAR,
    CMSG_TUTORIAL_RESET,
    CMSG_STANDSTATECHANGE,
    CMSG_EMOTE,
    CMSG_TEXT_EMOTE,
    CMSG_AUTOSTORE_LOOT_ITEM,
    CMSG_AUTOEQUIP_ITEM,
    CMSG_AUTOSTORE_BAG_ITEM,
    CMSG_SWAP_ITEM,
    CMSG_SWAP_INV_ITEM,
    CMSG_SPLIT_ITEM,
    CMSG_AUTOEQUIP_ITEM_SLOT,
    CMSG_DESTROYITEM,
    CMSG_INSPECT,
    CMSG_INITIATE_TRADE,
    CMSG_BEGIN_TRADE,
    CMSG_BUSY_TRADE,
    CMSG_IGNORE_TRADE,
    CMSG_ACCEPT_TRADE,
    CMSG_UNACCEPT_TRADE,
    CMSG_CANCEL_TRADE,
    CMSG_SET_TRADE_ITEM,
    CMSG_CLEAR_TRADE_ITEM,
    CMSG_SET_TRADE_GOLD,
    CMSG_SET_FACTION_ATWAR,
    CMSG_SET_ACTION_BUTTON,
    CMSG_CAST_SPELL,
    CMSG_CANCEL_CAST,
    CMSG_CANCEL_AURA,
    CMSG_CANCEL_CHANNELLING,
    CMSG_SET_SELECTION,
    CMSG_SET_TARGET_OBSOLETE,
    CMSG_ATTACKSWING,
    CMSG_ATTACKSTOP,
    CMSG_REPOP_REQUEST,
    CMSG_RESURRECT_RESPONSE,
    CMSG_LOOT,
    CMSG_LOOT_MONEY,
    CMSG_LOOT_RELEASE,
    CMSG_DUEL_ACCEPTED,
    CMSG_DUEL_CANCELLED,
    CMSG_MOUNTSPECIAL_ANIM,
    CMSG_PET_SET_ACTION,
    CMSG_PET_ACTION,
    CMSG_PET_ABANDON,
    CMSG_PET_RENAME,
    CMSG_GOSSIP_HELLO,
    CMSG_GOSSIP_SELECT_OPTION,
    CMSG_NPC_TEXT_QUERY,
    CMSG_QUESTGIVER_STATUS_QUERY,
    CMSG_QUESTGIVER_HELLO,
    CMSG_QUESTGIVER_QUERY_QUEST,
    CMSG_QUESTGIVER_QUEST_AUTOLAUNCH,
    CMSG_QUESTGIVER_ACCEPT_QUEST,
    CMSG_QUESTGIVER_COMPLETE_QUEST,
    CMSG_QUESTGIVER_REQUEST_REWARD,
    CMSG_QUESTGIVER_CHOOSE_REWARD,
    CMSG_QUESTGIVER_CANCEL,
    CMSG_QUESTLOG_SWAP_QUEST,
    CMSG_QUESTLOG_REMOVE_QUEST,
    CMSG_QUEST_CONFIRM_ACCEPT,
    CMSG_PUSHQUESTTOPARTY,
    CMSG_LIST_INVENTORY,
    CMSG_SELL_ITEM,
    CMSG_BUY_ITEM,
    CMSG_BUY_ITEM_IN_SLOT,
    CMSG_TAXINODE_STATUS_QUERY,
    CMSG_TAXIQUERYAVAILABLENODES,
    CMSG_ACTIVATETAXI,
    CMSG_TRAINER_LIST,
    CMSG_TRAINER_BUY_SPELL,
    CMSG_BINDER_ACTIVATE,
    CMSG_BANKER_ACTIVATE,
    CMSG_BUY_BANK_SLOT,
    CMSG_PETITION_SHOWLIST,
    CMSG_PETITION_BUY,
    CMSG_PETITION_SHOW_SIGNATURES,
    CMSG_PETITION_SIGN,
    CMSG_OFFER_PETITION,
    CMSG_TURN_IN_PETITION,
    CMSG_PETITION_QUERY,
    CMSG_BUG,
    CMSG_PLAYED_TIME,
    CMSG_QUERY_TIME,
    CMSG_RECLAIM_CORPSE,
    CMSG_WRAP_ITEM,
    MSG_MINIMAP_PING,
    CMSG_PING,
    CMSG_SETSHEATHED,
    CMSG_AUTH_SESSION,
    CMSG_PET_CAST_SPELL,
    MSG_SAVE_GUILD_EMBLEM,
    CMSG_ZONEUPDATE,
    MSG_RANDOM_ROLL,
    MSG_LOOKING_FOR_GROUP,
    CMSG_UNLEARN_SKILL,
    CMSG_GMTICKET_CREATE,
    CMSG_GMTICKET_UPDATETEXT,
    CMSG_REQUEST_ACCOUNT_DATA,
    CMSG_GMTICKET_GETTICKET,
    MSG_CORPSE_QUERY,
    CMSG_GMTICKET_DELETETICKET,
    CMSG_GMTICKET_SYSTEMSTATUS,
    CMSG_SPIRIT_HEALER_ACTIVATE,
    CMSG_CHAT_IGNORED,
    CMSG_GUILD_RANK,
    CMSG_GUILD_ADD_RANK,
    CMSG_GUILD_DEL_RANK,
    CMSG_GUILD_SET_PUBLIC_NOTE,
    CMSG_GUILD_SET_OFFICER_NOTE,
    CMSG_SEND_MAIL,
    CMSG_GET_MAIL_LIST,
    CMSG_BATTLEFIELD_LIST,
    CMSG_BATTLEFIELD_JOIN,
    CMSG_ITEM_TEXT_QUERY,
    CMSG_MAIL_TAKE_MONEY,
    CMSG_MAIL_TAKE_ITEM,
    CMSG_MAIL_MARK_AS_READ,
    CMSG_MAIL_RETURN_TO_SENDER,
    CMSG_MAIL_DELETE,
    CMSG_MAIL_CREATE_TEXT_ITEM,
    CMSG_LEARN_TALENT,
    CMSG_TOGGLE_PVP,
    MSG_AUCTION_HELLO,
    CMSG_AUCTION_SELL_ITEM,
    CMSG_AUCTION_REMOVE_ITEM,
    CMSG_AUCTION_LIST_ITEMS,
    CMSG_AUCTION_LIST_OWNER_ITEMS,
    CMSG_AUCTION_PLACE_BID,
    CMSG_AUCTION_LIST_BIDDER_ITEMS,
    CMSG_SET_AMMO,
    CMSG_SET_ACTIVE_MOVER,
    CMSG_PET_CANCEL_AURA,
    CMSG_CANCEL_AUTO_REPEAT_SPELL,
    MSG_LIST_STABLED_PETS,
    CMSG_STABLE_PET,
    CMSG_UNSTABLE_PET,
    CMSG_BUY_STABLE_SLOT,
    CMSG_STABLE_SWAP_PET,
    CMSG_REQUEST_PET_INFO,
    CMSG_FAR_SIGHT,
    CMSG_GROUP_CHANGE_SUB_GROUP,
    CMSG_REQUEST_PARTY_MEMBER_STATS,
    CMSG_GROUP_SWAP_SUB_GROUP,
    CMSG_AUTOSTORE_BANK_ITEM,
    CMSG_AUTOBANK_ITEM,
    MSG_QUERY_NEXT_MAIL_TIME,
    CMSG_GROUP_RAID_CONVERT,
    CMSG_GROUP_ASSISTANT_LEADER,
    CMSG_BUYBACK_ITEM,
    CMSG_MEETINGSTONE_JOIN,
    CMSG_MEETINGSTONE_LEAVE,
    CMSG_MEETINGSTONE_INFO,
    CMSG_CANCEL_GROWTH_AURA,
    CMSG_LOOT_ROLL,
    CMSG_LOOT_MASTER_GIVE,
    CMSG_REPAIR_ITEM,
    MSG_TALENT_WIPE_CONFIRM,
    CMSG_SUMMON_RESPONSE,
    CMSG_SELF_RES,
    CMSG_TOGGLE_HELM,
    CMSG_TOGGLE_CLOAK,
    CMSG_SET_ACTIONBAR_TOGGLES,
    CMSG_ITEM_NAME_QUERY,
    CMSG_CHAR_RENAME,
    CMSG_MOVE_SPLINE_DONE,
    CMSG_MOVE_FALL_RESET,
    CMSG_REQUEST_RAID_INFO,
    CMSG_MOVE_TIME_SKIPPED,
    CMSG_MOVE_FEATHER_FALL_ACK,
    CMSG_MOVE_WATER_WALK_ACK,
    CMSG_MOVE_NOT_ACTIVE_MOVER,
    CMSG_BATTLEFIELD_STATUS,
    CMSG_BATTLEFIELD_PORT,
    MSG_INSPECT_HONOR_STATS,
    CMSG_BATTLEMASTER_HELLO,
    CMSG_FORCE_WALK_SPEED_CHANGE_ACK,
    CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK,
    CMSG_FORCE_TURN_RATE_CHANGE_ACK,
    MSG_PVP_LOG_DATA,
    CMSG_LEAVE_BATTLEFIELD,
    CMSG_AREA_SPIRIT_HEALER_QUERY,
    CMSG_AREA_SPIRIT_HEALER_QUEUE,
    MSG_BATTLEGROUND_PLAYER_POSITIONS,
    CMSG_PET_STOP_ATTACK,
    CMSG_BATTLEMASTER_JOIN,
    CMSG_PET_UNLEARN,
    CMSG_PET_SPELL_AUTOCAST,
    CMSG_GUILD_INFO_TEXT,
    CMSG_ACTIVATETAXIEXPRESS,
    CMSG_SET_FACTION_INACTIVE,
    CMSG_SET_WATCHED_FACTION,
    CMSG_RESET_INSTANCES,
    MSG_RAID_TARGET_UPDATE,
    MSG_RAID_READY_CHECK,
    CMSG_GMSURVEY_SUBMIT,
}

impl ClientOpcode {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::MSG_MOVE_START_FORWARD => 0xb5,
            Self::MSG_MOVE_START_BACKWARD => 0xb6,
            Self::MSG_MOVE_STOP => 0xb7,
            Self::MSG_MOVE_START_STRAFE_LEFT => 0xb8,
            Self::MSG_MOVE_START_STRAFE_RIGHT => 0xb9,
            Self::MSG_MOVE_STOP_STRAFE => 0xba,
            Self::MSG_MOVE_JUMP => 0xbb,
            Self::MSG_MOVE_START_TURN_LEFT => 0xbc,
            Self::MSG_MOVE_START_TURN_RIGHT => 0xbd,
            Self::MSG_MOVE_STOP_TURN => 0xbe,
            Self::MSG_MOVE_START_PITCH_UP => 0xbf,
            Self::MSG_MOVE_START_PITCH_DOWN => 0xc0,
            Self::MSG_MOVE_STOP_PITCH => 0xc1,
            Self::MSG_MOVE_SET_RUN_MODE => 0xc2,
            Self::MSG_MOVE_SET_WALK_MODE => 0xc3,
            Self::MSG_MOVE_TELEPORT_ACK => 0xc7,
            Self::MSG_MOVE_FALL_LAND => 0xc9,
            Self::MSG_MOVE_START_SWIM => 0xca,
            Self::MSG_MOVE_STOP_SWIM => 0xcb,
            Self::MSG_MOVE_SET_FACING => 0xda,
            Self::MSG_MOVE_SET_PITCH => 0xdb,
            Self::MSG_MOVE_WORLDPORT_ACK => 0xdc,
            Self::MSG_MOVE_HEARTBEAT => 0xee,
            Self::MSG_PETITION_DECLINE => 0x1c2,
            Self::MSG_TABARDVENDOR_ACTIVATE => 0x1f2,
            Self::MSG_QUEST_PUSH_RESULT => 0x276,
            Self::MSG_PETITION_RENAME => 0x2c1,
            Self::CMSG_WORLD_TELEPORT => 0x08,
            Self::CMSG_CHAR_CREATE => 0x36,
            Self::CMSG_CHAR_ENUM => 0x37,
            Self::CMSG_CHAR_DELETE => 0x38,
            Self::CMSG_PLAYER_LOGIN => 0x3d,
            Self::CMSG_PLAYER_LOGOUT => 0x4a,
            Self::CMSG_LOGOUT_REQUEST => 0x4b,
            Self::CMSG_LOGOUT_CANCEL => 0x4e,
            Self::CMSG_NAME_QUERY => 0x50,
            Self::CMSG_PET_NAME_QUERY => 0x52,
            Self::CMSG_GUILD_QUERY => 0x54,
            Self::CMSG_ITEM_QUERY_SINGLE => 0x56,
            Self::CMSG_PAGE_TEXT_QUERY => 0x5a,
            Self::CMSG_QUEST_QUERY => 0x5c,
            Self::CMSG_GAMEOBJECT_QUERY => 0x5e,
            Self::CMSG_CREATURE_QUERY => 0x60,
            Self::CMSG_WHO => 0x62,
            Self::CMSG_WHOIS => 0x64,
            Self::CMSG_FRIEND_LIST => 0x66,
            Self::CMSG_ADD_FRIEND => 0x69,
            Self::CMSG_DEL_FRIEND => 0x6a,
            Self::CMSG_ADD_IGNORE => 0x6c,
            Self::CMSG_DEL_IGNORE => 0x6d,
            Self::CMSG_GROUP_INVITE => 0x6e,
            Self::CMSG_GROUP_ACCEPT => 0x72,
            Self::CMSG_GROUP_DECLINE => 0x73,
            Self::CMSG_GROUP_UNINVITE => 0x75,
            Self::CMSG_GROUP_UNINVITE_GUID => 0x76,
            Self::CMSG_GROUP_SET_LEADER => 0x78,
            Self::CMSG_LOOT_METHOD => 0x7a,
            Self::CMSG_GROUP_DISBAND => 0x7b,
            Self::CMSG_GUILD_CREATE => 0x81,
            Self::CMSG_GUILD_INVITE => 0x82,
            Self::CMSG_GUILD_ACCEPT => 0x84,
            Self::CMSG_GUILD_DECLINE => 0x85,
            Self::CMSG_GUILD_INFO => 0x87,
            Self::CMSG_GUILD_ROSTER => 0x89,
            Self::CMSG_GUILD_PROMOTE => 0x8b,
            Self::CMSG_GUILD_DEMOTE => 0x8c,
            Self::CMSG_GUILD_LEAVE => 0x8d,
            Self::CMSG_GUILD_REMOVE => 0x8e,
            Self::CMSG_GUILD_DISBAND => 0x8f,
            Self::CMSG_GUILD_LEADER => 0x90,
            Self::CMSG_GUILD_MOTD => 0x91,
            Self::CMSG_MESSAGECHAT => 0x95,
            Self::CMSG_JOIN_CHANNEL => 0x97,
            Self::CMSG_LEAVE_CHANNEL => 0x98,
            Self::CMSG_CHANNEL_LIST => 0x9a,
            Self::CMSG_CHANNEL_PASSWORD => 0x9c,
            Self::CMSG_CHANNEL_SET_OWNER => 0x9d,
            Self::CMSG_CHANNEL_OWNER => 0x9e,
            Self::CMSG_CHANNEL_MODERATOR => 0x9f,
            Self::CMSG_CHANNEL_UNMODERATOR => 0xa0,
            Self::CMSG_CHANNEL_MUTE => 0xa1,
            Self::CMSG_CHANNEL_UNMUTE => 0xa2,
            Self::CMSG_CHANNEL_INVITE => 0xa3,
            Self::CMSG_CHANNEL_KICK => 0xa4,
            Self::CMSG_CHANNEL_BAN => 0xa5,
            Self::CMSG_CHANNEL_UNBAN => 0xa6,
            Self::CMSG_CHANNEL_ANNOUNCEMENTS => 0xa7,
            Self::CMSG_CHANNEL_MODERATE => 0xa8,
            Self::SMSG_UPDATE_OBJECT => 0xa9,
            Self::CMSG_USE_ITEM => 0xab,
            Self::CMSG_OPEN_ITEM => 0xac,
            Self::CMSG_READ_ITEM => 0xad,
            Self::CMSG_GAMEOBJ_USE => 0xb1,
            Self::CMSG_AREATRIGGER => 0xb4,
            Self::CMSG_MOVE_SET_RAW_POSITION => 0xe1,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK => 0xe3,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK => 0xe5,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK => 0xe7,
            Self::CMSG_FORCE_MOVE_ROOT_ACK => 0xe9,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK => 0xeb,
            Self::CMSG_MOVE_KNOCK_BACK_ACK => 0xf0,
            Self::CMSG_MOVE_HOVER_ACK => 0xf6,
            Self::CMSG_NEXT_CINEMATIC_CAMERA => 0xfb,
            Self::CMSG_COMPLETE_CINEMATIC => 0xfc,
            Self::CMSG_TUTORIAL_FLAG => 0xfe,
            Self::CMSG_TUTORIAL_CLEAR => 0xff,
            Self::CMSG_TUTORIAL_RESET => 0x100,
            Self::CMSG_STANDSTATECHANGE => 0x101,
            Self::CMSG_EMOTE => 0x102,
            Self::CMSG_TEXT_EMOTE => 0x104,
            Self::CMSG_AUTOSTORE_LOOT_ITEM => 0x108,
            Self::CMSG_AUTOEQUIP_ITEM => 0x10a,
            Self::CMSG_AUTOSTORE_BAG_ITEM => 0x10b,
            Self::CMSG_SWAP_ITEM => 0x10c,
            Self::CMSG_SWAP_INV_ITEM => 0x10d,
            Self::CMSG_SPLIT_ITEM => 0x10e,
            Self::CMSG_AUTOEQUIP_ITEM_SLOT => 0x10f,
            Self::CMSG_DESTROYITEM => 0x111,
            Self::CMSG_INSPECT => 0x114,
            Self::CMSG_INITIATE_TRADE => 0x116,
            Self::CMSG_BEGIN_TRADE => 0x117,
            Self::CMSG_BUSY_TRADE => 0x118,
            Self::CMSG_IGNORE_TRADE => 0x119,
            Self::CMSG_ACCEPT_TRADE => 0x11a,
            Self::CMSG_UNACCEPT_TRADE => 0x11b,
            Self::CMSG_CANCEL_TRADE => 0x11c,
            Self::CMSG_SET_TRADE_ITEM => 0x11d,
            Self::CMSG_CLEAR_TRADE_ITEM => 0x11e,
            Self::CMSG_SET_TRADE_GOLD => 0x11f,
            Self::CMSG_SET_FACTION_ATWAR => 0x125,
            Self::CMSG_SET_ACTION_BUTTON => 0x128,
            Self::CMSG_CAST_SPELL => 0x12e,
            Self::CMSG_CANCEL_CAST => 0x12f,
            Self::CMSG_CANCEL_AURA => 0x136,
            Self::CMSG_CANCEL_CHANNELLING => 0x13b,
            Self::CMSG_SET_SELECTION => 0x13d,
            Self::CMSG_SET_TARGET_OBSOLETE => 0x13e,
            Self::CMSG_ATTACKSWING => 0x141,
            Self::CMSG_ATTACKSTOP => 0x142,
            Self::CMSG_REPOP_REQUEST => 0x15a,
            Self::CMSG_RESURRECT_RESPONSE => 0x15c,
            Self::CMSG_LOOT => 0x15d,
            Self::CMSG_LOOT_MONEY => 0x15e,
            Self::CMSG_LOOT_RELEASE => 0x15f,
            Self::CMSG_DUEL_ACCEPTED => 0x16c,
            Self::CMSG_DUEL_CANCELLED => 0x16d,
            Self::CMSG_MOUNTSPECIAL_ANIM => 0x171,
            Self::CMSG_PET_SET_ACTION => 0x174,
            Self::CMSG_PET_ACTION => 0x175,
            Self::CMSG_PET_ABANDON => 0x176,
            Self::CMSG_PET_RENAME => 0x177,
            Self::CMSG_GOSSIP_HELLO => 0x17b,
            Self::CMSG_GOSSIP_SELECT_OPTION => 0x17c,
            Self::CMSG_NPC_TEXT_QUERY => 0x17f,
            Self::CMSG_QUESTGIVER_STATUS_QUERY => 0x182,
            Self::CMSG_QUESTGIVER_HELLO => 0x184,
            Self::CMSG_QUESTGIVER_QUERY_QUEST => 0x186,
            Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH => 0x187,
            Self::CMSG_QUESTGIVER_ACCEPT_QUEST => 0x189,
            Self::CMSG_QUESTGIVER_COMPLETE_QUEST => 0x18a,
            Self::CMSG_QUESTGIVER_REQUEST_REWARD => 0x18c,
            Self::CMSG_QUESTGIVER_CHOOSE_REWARD => 0x18e,
            Self::CMSG_QUESTGIVER_CANCEL => 0x190,
            Self::CMSG_QUESTLOG_SWAP_QUEST => 0x193,
            Self::CMSG_QUESTLOG_REMOVE_QUEST => 0x194,
            Self::CMSG_QUEST_CONFIRM_ACCEPT => 0x19b,
            Self::CMSG_PUSHQUESTTOPARTY => 0x19d,
            Self::CMSG_LIST_INVENTORY => 0x19e,
            Self::CMSG_SELL_ITEM => 0x1a0,
            Self::CMSG_BUY_ITEM => 0x1a2,
            Self::CMSG_BUY_ITEM_IN_SLOT => 0x1a3,
            Self::CMSG_TAXINODE_STATUS_QUERY => 0x1aa,
            Self::CMSG_TAXIQUERYAVAILABLENODES => 0x1ac,
            Self::CMSG_ACTIVATETAXI => 0x1ad,
            Self::CMSG_TRAINER_LIST => 0x1b0,
            Self::CMSG_TRAINER_BUY_SPELL => 0x1b2,
            Self::CMSG_BINDER_ACTIVATE => 0x1b5,
            Self::CMSG_BANKER_ACTIVATE => 0x1b7,
            Self::CMSG_BUY_BANK_SLOT => 0x1b9,
            Self::CMSG_PETITION_SHOWLIST => 0x1bb,
            Self::CMSG_PETITION_BUY => 0x1bd,
            Self::CMSG_PETITION_SHOW_SIGNATURES => 0x1be,
            Self::CMSG_PETITION_SIGN => 0x1c0,
            Self::CMSG_OFFER_PETITION => 0x1c3,
            Self::CMSG_TURN_IN_PETITION => 0x1c4,
            Self::CMSG_PETITION_QUERY => 0x1c6,
            Self::CMSG_BUG => 0x1ca,
            Self::CMSG_PLAYED_TIME => 0x1cc,
            Self::CMSG_QUERY_TIME => 0x1ce,
            Self::CMSG_RECLAIM_CORPSE => 0x1d2,
            Self::CMSG_WRAP_ITEM => 0x1d3,
            Self::MSG_MINIMAP_PING => 0x1d5,
            Self::CMSG_PING => 0x1dc,
            Self::CMSG_SETSHEATHED => 0x1e0,
            Self::CMSG_AUTH_SESSION => 0x1ed,
            Self::CMSG_PET_CAST_SPELL => 0x1f0,
            Self::MSG_SAVE_GUILD_EMBLEM => 0x1f1,
            Self::CMSG_ZONEUPDATE => 0x1f4,
            Self::MSG_RANDOM_ROLL => 0x1fb,
            Self::MSG_LOOKING_FOR_GROUP => 0x1ff,
            Self::CMSG_UNLEARN_SKILL => 0x202,
            Self::CMSG_GMTICKET_CREATE => 0x205,
            Self::CMSG_GMTICKET_UPDATETEXT => 0x207,
            Self::CMSG_REQUEST_ACCOUNT_DATA => 0x20a,
            Self::CMSG_GMTICKET_GETTICKET => 0x211,
            Self::MSG_CORPSE_QUERY => 0x216,
            Self::CMSG_GMTICKET_DELETETICKET => 0x217,
            Self::CMSG_GMTICKET_SYSTEMSTATUS => 0x21a,
            Self::CMSG_SPIRIT_HEALER_ACTIVATE => 0x21c,
            Self::CMSG_CHAT_IGNORED => 0x225,
            Self::CMSG_GUILD_RANK => 0x231,
            Self::CMSG_GUILD_ADD_RANK => 0x232,
            Self::CMSG_GUILD_DEL_RANK => 0x233,
            Self::CMSG_GUILD_SET_PUBLIC_NOTE => 0x234,
            Self::CMSG_GUILD_SET_OFFICER_NOTE => 0x235,
            Self::CMSG_SEND_MAIL => 0x238,
            Self::CMSG_GET_MAIL_LIST => 0x23a,
            Self::CMSG_BATTLEFIELD_LIST => 0x23c,
            Self::CMSG_BATTLEFIELD_JOIN => 0x23e,
            Self::CMSG_ITEM_TEXT_QUERY => 0x243,
            Self::CMSG_MAIL_TAKE_MONEY => 0x245,
            Self::CMSG_MAIL_TAKE_ITEM => 0x246,
            Self::CMSG_MAIL_MARK_AS_READ => 0x247,
            Self::CMSG_MAIL_RETURN_TO_SENDER => 0x248,
            Self::CMSG_MAIL_DELETE => 0x249,
            Self::CMSG_MAIL_CREATE_TEXT_ITEM => 0x24a,
            Self::CMSG_LEARN_TALENT => 0x251,
            Self::CMSG_TOGGLE_PVP => 0x253,
            Self::MSG_AUCTION_HELLO => 0x255,
            Self::CMSG_AUCTION_SELL_ITEM => 0x256,
            Self::CMSG_AUCTION_REMOVE_ITEM => 0x257,
            Self::CMSG_AUCTION_LIST_ITEMS => 0x258,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS => 0x259,
            Self::CMSG_AUCTION_PLACE_BID => 0x25a,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS => 0x264,
            Self::CMSG_SET_AMMO => 0x268,
            Self::CMSG_SET_ACTIVE_MOVER => 0x26a,
            Self::CMSG_PET_CANCEL_AURA => 0x26b,
            Self::CMSG_CANCEL_AUTO_REPEAT_SPELL => 0x26d,
            Self::MSG_LIST_STABLED_PETS => 0x26f,
            Self::CMSG_STABLE_PET => 0x270,
            Self::CMSG_UNSTABLE_PET => 0x271,
            Self::CMSG_BUY_STABLE_SLOT => 0x272,
            Self::CMSG_STABLE_SWAP_PET => 0x275,
            Self::CMSG_REQUEST_PET_INFO => 0x279,
            Self::CMSG_FAR_SIGHT => 0x27a,
            Self::CMSG_GROUP_CHANGE_SUB_GROUP => 0x27e,
            Self::CMSG_REQUEST_PARTY_MEMBER_STATS => 0x27f,
            Self::CMSG_GROUP_SWAP_SUB_GROUP => 0x280,
            Self::CMSG_AUTOSTORE_BANK_ITEM => 0x282,
            Self::CMSG_AUTOBANK_ITEM => 0x283,
            Self::MSG_QUERY_NEXT_MAIL_TIME => 0x284,
            Self::CMSG_GROUP_RAID_CONVERT => 0x28e,
            Self::CMSG_GROUP_ASSISTANT_LEADER => 0x28f,
            Self::CMSG_BUYBACK_ITEM => 0x290,
            Self::CMSG_MEETINGSTONE_JOIN => 0x292,
            Self::CMSG_MEETINGSTONE_LEAVE => 0x293,
            Self::CMSG_MEETINGSTONE_INFO => 0x296,
            Self::CMSG_CANCEL_GROWTH_AURA => 0x29b,
            Self::CMSG_LOOT_ROLL => 0x2a0,
            Self::CMSG_LOOT_MASTER_GIVE => 0x2a3,
            Self::CMSG_REPAIR_ITEM => 0x2a8,
            Self::MSG_TALENT_WIPE_CONFIRM => 0x2aa,
            Self::CMSG_SUMMON_RESPONSE => 0x2ac,
            Self::CMSG_SELF_RES => 0x2b3,
            Self::CMSG_TOGGLE_HELM => 0x2b9,
            Self::CMSG_TOGGLE_CLOAK => 0x2ba,
            Self::CMSG_SET_ACTIONBAR_TOGGLES => 0x2bf,
            Self::CMSG_ITEM_NAME_QUERY => 0x2c4,
            Self::CMSG_CHAR_RENAME => 0x2c7,
            Self::CMSG_MOVE_SPLINE_DONE => 0x2c9,
            Self::CMSG_MOVE_FALL_RESET => 0x2ca,
            Self::CMSG_REQUEST_RAID_INFO => 0x2cd,
            Self::CMSG_MOVE_TIME_SKIPPED => 0x2ce,
            Self::CMSG_MOVE_FEATHER_FALL_ACK => 0x2cf,
            Self::CMSG_MOVE_WATER_WALK_ACK => 0x2d0,
            Self::CMSG_MOVE_NOT_ACTIVE_MOVER => 0x2d1,
            Self::CMSG_BATTLEFIELD_STATUS => 0x2d3,
            Self::CMSG_BATTLEFIELD_PORT => 0x2d5,
            Self::MSG_INSPECT_HONOR_STATS => 0x2d6,
            Self::CMSG_BATTLEMASTER_HELLO => 0x2d7,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK => 0x2db,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK => 0x2dd,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK => 0x2df,
            Self::MSG_PVP_LOG_DATA => 0x2e0,
            Self::CMSG_LEAVE_BATTLEFIELD => 0x2e1,
            Self::CMSG_AREA_SPIRIT_HEALER_QUERY => 0x2e2,
            Self::CMSG_AREA_SPIRIT_HEALER_QUEUE => 0x2e3,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS => 0x2e9,
            Self::CMSG_PET_STOP_ATTACK => 0x2ea,
            Self::CMSG_BATTLEMASTER_JOIN => 0x2ee,
            Self::CMSG_PET_UNLEARN => 0x2f0,
            Self::CMSG_PET_SPELL_AUTOCAST => 0x2f3,
            Self::CMSG_GUILD_INFO_TEXT => 0x2fc,
            Self::CMSG_ACTIVATETAXIEXPRESS => 0x312,
            Self::CMSG_SET_FACTION_INACTIVE => 0x317,
            Self::CMSG_SET_WATCHED_FACTION => 0x318,
            Self::CMSG_RESET_INSTANCES => 0x31d,
            Self::MSG_RAID_TARGET_UPDATE => 0x321,
            Self::MSG_RAID_READY_CHECK => 0x322,
            Self::CMSG_GMSURVEY_SUBMIT => 0x32a,
        }
    }

}

impl ClientOpcode {
    pub fn new(opcode: u32) -> std::result::Result<Self, ClientOpcodeError> {
        match opcode {
            0xb5 => Ok(Self::MSG_MOVE_START_FORWARD),
            0xb6 => Ok(Self::MSG_MOVE_START_BACKWARD),
            0xb7 => Ok(Self::MSG_MOVE_STOP),
            0xb8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT),
            0xb9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT),
            0xba => Ok(Self::MSG_MOVE_STOP_STRAFE),
            0xbb => Ok(Self::MSG_MOVE_JUMP),
            0xbc => Ok(Self::MSG_MOVE_START_TURN_LEFT),
            0xbd => Ok(Self::MSG_MOVE_START_TURN_RIGHT),
            0xbe => Ok(Self::MSG_MOVE_STOP_TURN),
            0xbf => Ok(Self::MSG_MOVE_START_PITCH_UP),
            0xc0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN),
            0xc1 => Ok(Self::MSG_MOVE_STOP_PITCH),
            0xc2 => Ok(Self::MSG_MOVE_SET_RUN_MODE),
            0xc3 => Ok(Self::MSG_MOVE_SET_WALK_MODE),
            0xc7 => Ok(Self::MSG_MOVE_TELEPORT_ACK),
            0xc9 => Ok(Self::MSG_MOVE_FALL_LAND),
            0xca => Ok(Self::MSG_MOVE_START_SWIM),
            0xcb => Ok(Self::MSG_MOVE_STOP_SWIM),
            0xda => Ok(Self::MSG_MOVE_SET_FACING),
            0xdb => Ok(Self::MSG_MOVE_SET_PITCH),
            0xdc => Ok(Self::MSG_MOVE_WORLDPORT_ACK),
            0xee => Ok(Self::MSG_MOVE_HEARTBEAT),
            0x1c2 => Ok(Self::MSG_PETITION_DECLINE),
            0x1f2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE),
            0x276 => Ok(Self::MSG_QUEST_PUSH_RESULT),
            0x2c1 => Ok(Self::MSG_PETITION_RENAME),
            0x08 => Ok(Self::CMSG_WORLD_TELEPORT),
            0x36 => Ok(Self::CMSG_CHAR_CREATE),
            0x37 => Ok(Self::CMSG_CHAR_ENUM),
            0x38 => Ok(Self::CMSG_CHAR_DELETE),
            0x3d => Ok(Self::CMSG_PLAYER_LOGIN),
            0x4a => Ok(Self::CMSG_PLAYER_LOGOUT),
            0x4b => Ok(Self::CMSG_LOGOUT_REQUEST),
            0x4e => Ok(Self::CMSG_LOGOUT_CANCEL),
            0x50 => Ok(Self::CMSG_NAME_QUERY),
            0x52 => Ok(Self::CMSG_PET_NAME_QUERY),
            0x54 => Ok(Self::CMSG_GUILD_QUERY),
            0x56 => Ok(Self::CMSG_ITEM_QUERY_SINGLE),
            0x5a => Ok(Self::CMSG_PAGE_TEXT_QUERY),
            0x5c => Ok(Self::CMSG_QUEST_QUERY),
            0x5e => Ok(Self::CMSG_GAMEOBJECT_QUERY),
            0x60 => Ok(Self::CMSG_CREATURE_QUERY),
            0x62 => Ok(Self::CMSG_WHO),
            0x64 => Ok(Self::CMSG_WHOIS),
            0x66 => Ok(Self::CMSG_FRIEND_LIST),
            0x69 => Ok(Self::CMSG_ADD_FRIEND),
            0x6a => Ok(Self::CMSG_DEL_FRIEND),
            0x6c => Ok(Self::CMSG_ADD_IGNORE),
            0x6d => Ok(Self::CMSG_DEL_IGNORE),
            0x6e => Ok(Self::CMSG_GROUP_INVITE),
            0x72 => Ok(Self::CMSG_GROUP_ACCEPT),
            0x73 => Ok(Self::CMSG_GROUP_DECLINE),
            0x75 => Ok(Self::CMSG_GROUP_UNINVITE),
            0x76 => Ok(Self::CMSG_GROUP_UNINVITE_GUID),
            0x78 => Ok(Self::CMSG_GROUP_SET_LEADER),
            0x7a => Ok(Self::CMSG_LOOT_METHOD),
            0x7b => Ok(Self::CMSG_GROUP_DISBAND),
            0x81 => Ok(Self::CMSG_GUILD_CREATE),
            0x82 => Ok(Self::CMSG_GUILD_INVITE),
            0x84 => Ok(Self::CMSG_GUILD_ACCEPT),
            0x85 => Ok(Self::CMSG_GUILD_DECLINE),
            0x87 => Ok(Self::CMSG_GUILD_INFO),
            0x89 => Ok(Self::CMSG_GUILD_ROSTER),
            0x8b => Ok(Self::CMSG_GUILD_PROMOTE),
            0x8c => Ok(Self::CMSG_GUILD_DEMOTE),
            0x8d => Ok(Self::CMSG_GUILD_LEAVE),
            0x8e => Ok(Self::CMSG_GUILD_REMOVE),
            0x8f => Ok(Self::CMSG_GUILD_DISBAND),
            0x90 => Ok(Self::CMSG_GUILD_LEADER),
            0x91 => Ok(Self::CMSG_GUILD_MOTD),
            0x95 => Ok(Self::CMSG_MESSAGECHAT),
            0x97 => Ok(Self::CMSG_JOIN_CHANNEL),
            0x98 => Ok(Self::CMSG_LEAVE_CHANNEL),
            0x9a => Ok(Self::CMSG_CHANNEL_LIST),
            0x9c => Ok(Self::CMSG_CHANNEL_PASSWORD),
            0x9d => Ok(Self::CMSG_CHANNEL_SET_OWNER),
            0x9e => Ok(Self::CMSG_CHANNEL_OWNER),
            0x9f => Ok(Self::CMSG_CHANNEL_MODERATOR),
            0xa0 => Ok(Self::CMSG_CHANNEL_UNMODERATOR),
            0xa1 => Ok(Self::CMSG_CHANNEL_MUTE),
            0xa2 => Ok(Self::CMSG_CHANNEL_UNMUTE),
            0xa3 => Ok(Self::CMSG_CHANNEL_INVITE),
            0xa4 => Ok(Self::CMSG_CHANNEL_KICK),
            0xa5 => Ok(Self::CMSG_CHANNEL_BAN),
            0xa6 => Ok(Self::CMSG_CHANNEL_UNBAN),
            0xa7 => Ok(Self::CMSG_CHANNEL_ANNOUNCEMENTS),
            0xa8 => Ok(Self::CMSG_CHANNEL_MODERATE),
            0xa9 => Ok(Self::SMSG_UPDATE_OBJECT),
            0xab => Ok(Self::CMSG_USE_ITEM),
            0xac => Ok(Self::CMSG_OPEN_ITEM),
            0xad => Ok(Self::CMSG_READ_ITEM),
            0xb1 => Ok(Self::CMSG_GAMEOBJ_USE),
            0xb4 => Ok(Self::CMSG_AREATRIGGER),
            0xe1 => Ok(Self::CMSG_MOVE_SET_RAW_POSITION),
            0xe3 => Ok(Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK),
            0xe5 => Ok(Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK),
            0xe7 => Ok(Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK),
            0xe9 => Ok(Self::CMSG_FORCE_MOVE_ROOT_ACK),
            0xeb => Ok(Self::CMSG_FORCE_MOVE_UNROOT_ACK),
            0xf0 => Ok(Self::CMSG_MOVE_KNOCK_BACK_ACK),
            0xf6 => Ok(Self::CMSG_MOVE_HOVER_ACK),
            0xfb => Ok(Self::CMSG_NEXT_CINEMATIC_CAMERA),
            0xfc => Ok(Self::CMSG_COMPLETE_CINEMATIC),
            0xfe => Ok(Self::CMSG_TUTORIAL_FLAG),
            0xff => Ok(Self::CMSG_TUTORIAL_CLEAR),
            0x100 => Ok(Self::CMSG_TUTORIAL_RESET),
            0x101 => Ok(Self::CMSG_STANDSTATECHANGE),
            0x102 => Ok(Self::CMSG_EMOTE),
            0x104 => Ok(Self::CMSG_TEXT_EMOTE),
            0x108 => Ok(Self::CMSG_AUTOSTORE_LOOT_ITEM),
            0x10a => Ok(Self::CMSG_AUTOEQUIP_ITEM),
            0x10b => Ok(Self::CMSG_AUTOSTORE_BAG_ITEM),
            0x10c => Ok(Self::CMSG_SWAP_ITEM),
            0x10d => Ok(Self::CMSG_SWAP_INV_ITEM),
            0x10e => Ok(Self::CMSG_SPLIT_ITEM),
            0x10f => Ok(Self::CMSG_AUTOEQUIP_ITEM_SLOT),
            0x111 => Ok(Self::CMSG_DESTROYITEM),
            0x114 => Ok(Self::CMSG_INSPECT),
            0x116 => Ok(Self::CMSG_INITIATE_TRADE),
            0x117 => Ok(Self::CMSG_BEGIN_TRADE),
            0x118 => Ok(Self::CMSG_BUSY_TRADE),
            0x119 => Ok(Self::CMSG_IGNORE_TRADE),
            0x11a => Ok(Self::CMSG_ACCEPT_TRADE),
            0x11b => Ok(Self::CMSG_UNACCEPT_TRADE),
            0x11c => Ok(Self::CMSG_CANCEL_TRADE),
            0x11d => Ok(Self::CMSG_SET_TRADE_ITEM),
            0x11e => Ok(Self::CMSG_CLEAR_TRADE_ITEM),
            0x11f => Ok(Self::CMSG_SET_TRADE_GOLD),
            0x125 => Ok(Self::CMSG_SET_FACTION_ATWAR),
            0x128 => Ok(Self::CMSG_SET_ACTION_BUTTON),
            0x12e => Ok(Self::CMSG_CAST_SPELL),
            0x12f => Ok(Self::CMSG_CANCEL_CAST),
            0x136 => Ok(Self::CMSG_CANCEL_AURA),
            0x13b => Ok(Self::CMSG_CANCEL_CHANNELLING),
            0x13d => Ok(Self::CMSG_SET_SELECTION),
            0x13e => Ok(Self::CMSG_SET_TARGET_OBSOLETE),
            0x141 => Ok(Self::CMSG_ATTACKSWING),
            0x142 => Ok(Self::CMSG_ATTACKSTOP),
            0x15a => Ok(Self::CMSG_REPOP_REQUEST),
            0x15c => Ok(Self::CMSG_RESURRECT_RESPONSE),
            0x15d => Ok(Self::CMSG_LOOT),
            0x15e => Ok(Self::CMSG_LOOT_MONEY),
            0x15f => Ok(Self::CMSG_LOOT_RELEASE),
            0x16c => Ok(Self::CMSG_DUEL_ACCEPTED),
            0x16d => Ok(Self::CMSG_DUEL_CANCELLED),
            0x171 => Ok(Self::CMSG_MOUNTSPECIAL_ANIM),
            0x174 => Ok(Self::CMSG_PET_SET_ACTION),
            0x175 => Ok(Self::CMSG_PET_ACTION),
            0x176 => Ok(Self::CMSG_PET_ABANDON),
            0x177 => Ok(Self::CMSG_PET_RENAME),
            0x17b => Ok(Self::CMSG_GOSSIP_HELLO),
            0x17c => Ok(Self::CMSG_GOSSIP_SELECT_OPTION),
            0x17f => Ok(Self::CMSG_NPC_TEXT_QUERY),
            0x182 => Ok(Self::CMSG_QUESTGIVER_STATUS_QUERY),
            0x184 => Ok(Self::CMSG_QUESTGIVER_HELLO),
            0x186 => Ok(Self::CMSG_QUESTGIVER_QUERY_QUEST),
            0x187 => Ok(Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH),
            0x189 => Ok(Self::CMSG_QUESTGIVER_ACCEPT_QUEST),
            0x18a => Ok(Self::CMSG_QUESTGIVER_COMPLETE_QUEST),
            0x18c => Ok(Self::CMSG_QUESTGIVER_REQUEST_REWARD),
            0x18e => Ok(Self::CMSG_QUESTGIVER_CHOOSE_REWARD),
            0x190 => Ok(Self::CMSG_QUESTGIVER_CANCEL),
            0x193 => Ok(Self::CMSG_QUESTLOG_SWAP_QUEST),
            0x194 => Ok(Self::CMSG_QUESTLOG_REMOVE_QUEST),
            0x19b => Ok(Self::CMSG_QUEST_CONFIRM_ACCEPT),
            0x19d => Ok(Self::CMSG_PUSHQUESTTOPARTY),
            0x19e => Ok(Self::CMSG_LIST_INVENTORY),
            0x1a0 => Ok(Self::CMSG_SELL_ITEM),
            0x1a2 => Ok(Self::CMSG_BUY_ITEM),
            0x1a3 => Ok(Self::CMSG_BUY_ITEM_IN_SLOT),
            0x1aa => Ok(Self::CMSG_TAXINODE_STATUS_QUERY),
            0x1ac => Ok(Self::CMSG_TAXIQUERYAVAILABLENODES),
            0x1ad => Ok(Self::CMSG_ACTIVATETAXI),
            0x1b0 => Ok(Self::CMSG_TRAINER_LIST),
            0x1b2 => Ok(Self::CMSG_TRAINER_BUY_SPELL),
            0x1b5 => Ok(Self::CMSG_BINDER_ACTIVATE),
            0x1b7 => Ok(Self::CMSG_BANKER_ACTIVATE),
            0x1b9 => Ok(Self::CMSG_BUY_BANK_SLOT),
            0x1bb => Ok(Self::CMSG_PETITION_SHOWLIST),
            0x1bd => Ok(Self::CMSG_PETITION_BUY),
            0x1be => Ok(Self::CMSG_PETITION_SHOW_SIGNATURES),
            0x1c0 => Ok(Self::CMSG_PETITION_SIGN),
            0x1c3 => Ok(Self::CMSG_OFFER_PETITION),
            0x1c4 => Ok(Self::CMSG_TURN_IN_PETITION),
            0x1c6 => Ok(Self::CMSG_PETITION_QUERY),
            0x1ca => Ok(Self::CMSG_BUG),
            0x1cc => Ok(Self::CMSG_PLAYED_TIME),
            0x1ce => Ok(Self::CMSG_QUERY_TIME),
            0x1d2 => Ok(Self::CMSG_RECLAIM_CORPSE),
            0x1d3 => Ok(Self::CMSG_WRAP_ITEM),
            0x1d5 => Ok(Self::MSG_MINIMAP_PING),
            0x1dc => Ok(Self::CMSG_PING),
            0x1e0 => Ok(Self::CMSG_SETSHEATHED),
            0x1ed => Ok(Self::CMSG_AUTH_SESSION),
            0x1f0 => Ok(Self::CMSG_PET_CAST_SPELL),
            0x1f1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM),
            0x1f4 => Ok(Self::CMSG_ZONEUPDATE),
            0x1fb => Ok(Self::MSG_RANDOM_ROLL),
            0x1ff => Ok(Self::MSG_LOOKING_FOR_GROUP),
            0x202 => Ok(Self::CMSG_UNLEARN_SKILL),
            0x205 => Ok(Self::CMSG_GMTICKET_CREATE),
            0x207 => Ok(Self::CMSG_GMTICKET_UPDATETEXT),
            0x20a => Ok(Self::CMSG_REQUEST_ACCOUNT_DATA),
            0x211 => Ok(Self::CMSG_GMTICKET_GETTICKET),
            0x216 => Ok(Self::MSG_CORPSE_QUERY),
            0x217 => Ok(Self::CMSG_GMTICKET_DELETETICKET),
            0x21a => Ok(Self::CMSG_GMTICKET_SYSTEMSTATUS),
            0x21c => Ok(Self::CMSG_SPIRIT_HEALER_ACTIVATE),
            0x225 => Ok(Self::CMSG_CHAT_IGNORED),
            0x231 => Ok(Self::CMSG_GUILD_RANK),
            0x232 => Ok(Self::CMSG_GUILD_ADD_RANK),
            0x233 => Ok(Self::CMSG_GUILD_DEL_RANK),
            0x234 => Ok(Self::CMSG_GUILD_SET_PUBLIC_NOTE),
            0x235 => Ok(Self::CMSG_GUILD_SET_OFFICER_NOTE),
            0x238 => Ok(Self::CMSG_SEND_MAIL),
            0x23a => Ok(Self::CMSG_GET_MAIL_LIST),
            0x23c => Ok(Self::CMSG_BATTLEFIELD_LIST),
            0x23e => Ok(Self::CMSG_BATTLEFIELD_JOIN),
            0x243 => Ok(Self::CMSG_ITEM_TEXT_QUERY),
            0x245 => Ok(Self::CMSG_MAIL_TAKE_MONEY),
            0x246 => Ok(Self::CMSG_MAIL_TAKE_ITEM),
            0x247 => Ok(Self::CMSG_MAIL_MARK_AS_READ),
            0x248 => Ok(Self::CMSG_MAIL_RETURN_TO_SENDER),
            0x249 => Ok(Self::CMSG_MAIL_DELETE),
            0x24a => Ok(Self::CMSG_MAIL_CREATE_TEXT_ITEM),
            0x251 => Ok(Self::CMSG_LEARN_TALENT),
            0x253 => Ok(Self::CMSG_TOGGLE_PVP),
            0x255 => Ok(Self::MSG_AUCTION_HELLO),
            0x256 => Ok(Self::CMSG_AUCTION_SELL_ITEM),
            0x257 => Ok(Self::CMSG_AUCTION_REMOVE_ITEM),
            0x258 => Ok(Self::CMSG_AUCTION_LIST_ITEMS),
            0x259 => Ok(Self::CMSG_AUCTION_LIST_OWNER_ITEMS),
            0x25a => Ok(Self::CMSG_AUCTION_PLACE_BID),
            0x264 => Ok(Self::CMSG_AUCTION_LIST_BIDDER_ITEMS),
            0x268 => Ok(Self::CMSG_SET_AMMO),
            0x26a => Ok(Self::CMSG_SET_ACTIVE_MOVER),
            0x26b => Ok(Self::CMSG_PET_CANCEL_AURA),
            0x26d => Ok(Self::CMSG_CANCEL_AUTO_REPEAT_SPELL),
            0x26f => Ok(Self::MSG_LIST_STABLED_PETS),
            0x270 => Ok(Self::CMSG_STABLE_PET),
            0x271 => Ok(Self::CMSG_UNSTABLE_PET),
            0x272 => Ok(Self::CMSG_BUY_STABLE_SLOT),
            0x275 => Ok(Self::CMSG_STABLE_SWAP_PET),
            0x279 => Ok(Self::CMSG_REQUEST_PET_INFO),
            0x27a => Ok(Self::CMSG_FAR_SIGHT),
            0x27e => Ok(Self::CMSG_GROUP_CHANGE_SUB_GROUP),
            0x27f => Ok(Self::CMSG_REQUEST_PARTY_MEMBER_STATS),
            0x280 => Ok(Self::CMSG_GROUP_SWAP_SUB_GROUP),
            0x282 => Ok(Self::CMSG_AUTOSTORE_BANK_ITEM),
            0x283 => Ok(Self::CMSG_AUTOBANK_ITEM),
            0x284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME),
            0x28e => Ok(Self::CMSG_GROUP_RAID_CONVERT),
            0x28f => Ok(Self::CMSG_GROUP_ASSISTANT_LEADER),
            0x290 => Ok(Self::CMSG_BUYBACK_ITEM),
            0x292 => Ok(Self::CMSG_MEETINGSTONE_JOIN),
            0x293 => Ok(Self::CMSG_MEETINGSTONE_LEAVE),
            0x296 => Ok(Self::CMSG_MEETINGSTONE_INFO),
            0x29b => Ok(Self::CMSG_CANCEL_GROWTH_AURA),
            0x2a0 => Ok(Self::CMSG_LOOT_ROLL),
            0x2a3 => Ok(Self::CMSG_LOOT_MASTER_GIVE),
            0x2a8 => Ok(Self::CMSG_REPAIR_ITEM),
            0x2aa => Ok(Self::MSG_TALENT_WIPE_CONFIRM),
            0x2ac => Ok(Self::CMSG_SUMMON_RESPONSE),
            0x2b3 => Ok(Self::CMSG_SELF_RES),
            0x2b9 => Ok(Self::CMSG_TOGGLE_HELM),
            0x2ba => Ok(Self::CMSG_TOGGLE_CLOAK),
            0x2bf => Ok(Self::CMSG_SET_ACTIONBAR_TOGGLES),
            0x2c4 => Ok(Self::CMSG_ITEM_NAME_QUERY),
            0x2c7 => Ok(Self::CMSG_CHAR_RENAME),
            0x2c9 => Ok(Self::CMSG_MOVE_SPLINE_DONE),
            0x2ca => Ok(Self::CMSG_MOVE_FALL_RESET),
            0x2cd => Ok(Self::CMSG_REQUEST_RAID_INFO),
            0x2ce => Ok(Self::CMSG_MOVE_TIME_SKIPPED),
            0x2cf => Ok(Self::CMSG_MOVE_FEATHER_FALL_ACK),
            0x2d0 => Ok(Self::CMSG_MOVE_WATER_WALK_ACK),
            0x2d1 => Ok(Self::CMSG_MOVE_NOT_ACTIVE_MOVER),
            0x2d3 => Ok(Self::CMSG_BATTLEFIELD_STATUS),
            0x2d5 => Ok(Self::CMSG_BATTLEFIELD_PORT),
            0x2d6 => Ok(Self::MSG_INSPECT_HONOR_STATS),
            0x2d7 => Ok(Self::CMSG_BATTLEMASTER_HELLO),
            0x2db => Ok(Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK),
            0x2dd => Ok(Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK),
            0x2df => Ok(Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK),
            0x2e0 => Ok(Self::MSG_PVP_LOG_DATA),
            0x2e1 => Ok(Self::CMSG_LEAVE_BATTLEFIELD),
            0x2e2 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUERY),
            0x2e3 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUEUE),
            0x2e9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS),
            0x2ea => Ok(Self::CMSG_PET_STOP_ATTACK),
            0x2ee => Ok(Self::CMSG_BATTLEMASTER_JOIN),
            0x2f0 => Ok(Self::CMSG_PET_UNLEARN),
            0x2f3 => Ok(Self::CMSG_PET_SPELL_AUTOCAST),
            0x2fc => Ok(Self::CMSG_GUILD_INFO_TEXT),
            0x312 => Ok(Self::CMSG_ACTIVATETAXIEXPRESS),
            0x317 => Ok(Self::CMSG_SET_FACTION_INACTIVE),
            0x318 => Ok(Self::CMSG_SET_WATCHED_FACTION),
            0x31d => Ok(Self::CMSG_RESET_INSTANCES),
            0x321 => Ok(Self::MSG_RAID_TARGET_UPDATE),
            0x322 => Ok(Self::MSG_RAID_READY_CHECK),
            0x32a => Ok(Self::CMSG_GMSURVEY_SUBMIT),
            opcode => Err(ClientOpcodeError::InvalidOpcode(opcode)),
        }
    }

}

impl From<&ClientOpcodeMessage> for ClientOpcode {
    fn from(e: &ClientOpcodeMessage) -> Self {
        match *e {
            ClientOpcodeMessage::MSG_MOVE_START_FORWARD(_) => Self::MSG_MOVE_START_FORWARD,
            ClientOpcodeMessage::MSG_MOVE_START_BACKWARD(_) => Self::MSG_MOVE_START_BACKWARD,
            ClientOpcodeMessage::MSG_MOVE_STOP(_) => Self::MSG_MOVE_STOP,
            ClientOpcodeMessage::MSG_MOVE_START_STRAFE_LEFT(_) => Self::MSG_MOVE_START_STRAFE_LEFT,
            ClientOpcodeMessage::MSG_MOVE_START_STRAFE_RIGHT(_) => Self::MSG_MOVE_START_STRAFE_RIGHT,
            ClientOpcodeMessage::MSG_MOVE_STOP_STRAFE(_) => Self::MSG_MOVE_STOP_STRAFE,
            ClientOpcodeMessage::MSG_MOVE_JUMP(_) => Self::MSG_MOVE_JUMP,
            ClientOpcodeMessage::MSG_MOVE_START_TURN_LEFT(_) => Self::MSG_MOVE_START_TURN_LEFT,
            ClientOpcodeMessage::MSG_MOVE_START_TURN_RIGHT(_) => Self::MSG_MOVE_START_TURN_RIGHT,
            ClientOpcodeMessage::MSG_MOVE_STOP_TURN(_) => Self::MSG_MOVE_STOP_TURN,
            ClientOpcodeMessage::MSG_MOVE_START_PITCH_UP(_) => Self::MSG_MOVE_START_PITCH_UP,
            ClientOpcodeMessage::MSG_MOVE_START_PITCH_DOWN(_) => Self::MSG_MOVE_START_PITCH_DOWN,
            ClientOpcodeMessage::MSG_MOVE_STOP_PITCH(_) => Self::MSG_MOVE_STOP_PITCH,
            ClientOpcodeMessage::MSG_MOVE_SET_RUN_MODE(_) => Self::MSG_MOVE_SET_RUN_MODE,
            ClientOpcodeMessage::MSG_MOVE_SET_WALK_MODE(_) => Self::MSG_MOVE_SET_WALK_MODE,
            ClientOpcodeMessage::MSG_MOVE_TELEPORT_ACK(_) => Self::MSG_MOVE_TELEPORT_ACK,
            ClientOpcodeMessage::MSG_MOVE_FALL_LAND(_) => Self::MSG_MOVE_FALL_LAND,
            ClientOpcodeMessage::MSG_MOVE_START_SWIM(_) => Self::MSG_MOVE_START_SWIM,
            ClientOpcodeMessage::MSG_MOVE_STOP_SWIM(_) => Self::MSG_MOVE_STOP_SWIM,
            ClientOpcodeMessage::MSG_MOVE_SET_FACING(_) => Self::MSG_MOVE_SET_FACING,
            ClientOpcodeMessage::MSG_MOVE_SET_PITCH(_) => Self::MSG_MOVE_SET_PITCH,
            ClientOpcodeMessage::MSG_MOVE_WORLDPORT_ACK(_) => Self::MSG_MOVE_WORLDPORT_ACK,
            ClientOpcodeMessage::MSG_MOVE_HEARTBEAT(_) => Self::MSG_MOVE_HEARTBEAT,
            ClientOpcodeMessage::MSG_PETITION_DECLINE(_) => Self::MSG_PETITION_DECLINE,
            ClientOpcodeMessage::MSG_TABARDVENDOR_ACTIVATE(_) => Self::MSG_TABARDVENDOR_ACTIVATE,
            ClientOpcodeMessage::MSG_QUEST_PUSH_RESULT(_) => Self::MSG_QUEST_PUSH_RESULT,
            ClientOpcodeMessage::MSG_PETITION_RENAME(_) => Self::MSG_PETITION_RENAME,
            ClientOpcodeMessage::CMSG_WORLD_TELEPORT(_) => Self::CMSG_WORLD_TELEPORT,
            ClientOpcodeMessage::CMSG_CHAR_CREATE(_) => Self::CMSG_CHAR_CREATE,
            ClientOpcodeMessage::CMSG_CHAR_ENUM(_) => Self::CMSG_CHAR_ENUM,
            ClientOpcodeMessage::CMSG_CHAR_DELETE(_) => Self::CMSG_CHAR_DELETE,
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(_) => Self::CMSG_PLAYER_LOGIN,
            ClientOpcodeMessage::CMSG_PLAYER_LOGOUT(_) => Self::CMSG_PLAYER_LOGOUT,
            ClientOpcodeMessage::CMSG_LOGOUT_REQUEST(_) => Self::CMSG_LOGOUT_REQUEST,
            ClientOpcodeMessage::CMSG_LOGOUT_CANCEL(_) => Self::CMSG_LOGOUT_CANCEL,
            ClientOpcodeMessage::CMSG_NAME_QUERY(_) => Self::CMSG_NAME_QUERY,
            ClientOpcodeMessage::CMSG_PET_NAME_QUERY(_) => Self::CMSG_PET_NAME_QUERY,
            ClientOpcodeMessage::CMSG_GUILD_QUERY(_) => Self::CMSG_GUILD_QUERY,
            ClientOpcodeMessage::CMSG_ITEM_QUERY_SINGLE(_) => Self::CMSG_ITEM_QUERY_SINGLE,
            ClientOpcodeMessage::CMSG_PAGE_TEXT_QUERY(_) => Self::CMSG_PAGE_TEXT_QUERY,
            ClientOpcodeMessage::CMSG_QUEST_QUERY(_) => Self::CMSG_QUEST_QUERY,
            ClientOpcodeMessage::CMSG_GAMEOBJECT_QUERY(_) => Self::CMSG_GAMEOBJECT_QUERY,
            ClientOpcodeMessage::CMSG_CREATURE_QUERY(_) => Self::CMSG_CREATURE_QUERY,
            ClientOpcodeMessage::CMSG_WHO(_) => Self::CMSG_WHO,
            ClientOpcodeMessage::CMSG_WHOIS(_) => Self::CMSG_WHOIS,
            ClientOpcodeMessage::CMSG_FRIEND_LIST(_) => Self::CMSG_FRIEND_LIST,
            ClientOpcodeMessage::CMSG_ADD_FRIEND(_) => Self::CMSG_ADD_FRIEND,
            ClientOpcodeMessage::CMSG_DEL_FRIEND(_) => Self::CMSG_DEL_FRIEND,
            ClientOpcodeMessage::CMSG_ADD_IGNORE(_) => Self::CMSG_ADD_IGNORE,
            ClientOpcodeMessage::CMSG_DEL_IGNORE(_) => Self::CMSG_DEL_IGNORE,
            ClientOpcodeMessage::CMSG_GROUP_INVITE(_) => Self::CMSG_GROUP_INVITE,
            ClientOpcodeMessage::CMSG_GROUP_ACCEPT(_) => Self::CMSG_GROUP_ACCEPT,
            ClientOpcodeMessage::CMSG_GROUP_DECLINE(_) => Self::CMSG_GROUP_DECLINE,
            ClientOpcodeMessage::CMSG_GROUP_UNINVITE(_) => Self::CMSG_GROUP_UNINVITE,
            ClientOpcodeMessage::CMSG_GROUP_UNINVITE_GUID(_) => Self::CMSG_GROUP_UNINVITE_GUID,
            ClientOpcodeMessage::CMSG_GROUP_SET_LEADER(_) => Self::CMSG_GROUP_SET_LEADER,
            ClientOpcodeMessage::CMSG_LOOT_METHOD(_) => Self::CMSG_LOOT_METHOD,
            ClientOpcodeMessage::CMSG_GROUP_DISBAND(_) => Self::CMSG_GROUP_DISBAND,
            ClientOpcodeMessage::CMSG_GUILD_CREATE(_) => Self::CMSG_GUILD_CREATE,
            ClientOpcodeMessage::CMSG_GUILD_INVITE(_) => Self::CMSG_GUILD_INVITE,
            ClientOpcodeMessage::CMSG_GUILD_ACCEPT(_) => Self::CMSG_GUILD_ACCEPT,
            ClientOpcodeMessage::CMSG_GUILD_DECLINE(_) => Self::CMSG_GUILD_DECLINE,
            ClientOpcodeMessage::CMSG_GUILD_INFO(_) => Self::CMSG_GUILD_INFO,
            ClientOpcodeMessage::CMSG_GUILD_ROSTER(_) => Self::CMSG_GUILD_ROSTER,
            ClientOpcodeMessage::CMSG_GUILD_PROMOTE(_) => Self::CMSG_GUILD_PROMOTE,
            ClientOpcodeMessage::CMSG_GUILD_DEMOTE(_) => Self::CMSG_GUILD_DEMOTE,
            ClientOpcodeMessage::CMSG_GUILD_LEAVE(_) => Self::CMSG_GUILD_LEAVE,
            ClientOpcodeMessage::CMSG_GUILD_REMOVE(_) => Self::CMSG_GUILD_REMOVE,
            ClientOpcodeMessage::CMSG_GUILD_DISBAND(_) => Self::CMSG_GUILD_DISBAND,
            ClientOpcodeMessage::CMSG_GUILD_LEADER(_) => Self::CMSG_GUILD_LEADER,
            ClientOpcodeMessage::CMSG_GUILD_MOTD(_) => Self::CMSG_GUILD_MOTD,
            ClientOpcodeMessage::CMSG_MESSAGECHAT(_) => Self::CMSG_MESSAGECHAT,
            ClientOpcodeMessage::CMSG_JOIN_CHANNEL(_) => Self::CMSG_JOIN_CHANNEL,
            ClientOpcodeMessage::CMSG_LEAVE_CHANNEL(_) => Self::CMSG_LEAVE_CHANNEL,
            ClientOpcodeMessage::CMSG_CHANNEL_LIST(_) => Self::CMSG_CHANNEL_LIST,
            ClientOpcodeMessage::CMSG_CHANNEL_PASSWORD(_) => Self::CMSG_CHANNEL_PASSWORD,
            ClientOpcodeMessage::CMSG_CHANNEL_SET_OWNER(_) => Self::CMSG_CHANNEL_SET_OWNER,
            ClientOpcodeMessage::CMSG_CHANNEL_OWNER(_) => Self::CMSG_CHANNEL_OWNER,
            ClientOpcodeMessage::CMSG_CHANNEL_MODERATOR(_) => Self::CMSG_CHANNEL_MODERATOR,
            ClientOpcodeMessage::CMSG_CHANNEL_UNMODERATOR(_) => Self::CMSG_CHANNEL_UNMODERATOR,
            ClientOpcodeMessage::CMSG_CHANNEL_MUTE(_) => Self::CMSG_CHANNEL_MUTE,
            ClientOpcodeMessage::CMSG_CHANNEL_UNMUTE(_) => Self::CMSG_CHANNEL_UNMUTE,
            ClientOpcodeMessage::CMSG_CHANNEL_INVITE(_) => Self::CMSG_CHANNEL_INVITE,
            ClientOpcodeMessage::CMSG_CHANNEL_KICK(_) => Self::CMSG_CHANNEL_KICK,
            ClientOpcodeMessage::CMSG_CHANNEL_BAN(_) => Self::CMSG_CHANNEL_BAN,
            ClientOpcodeMessage::CMSG_CHANNEL_UNBAN(_) => Self::CMSG_CHANNEL_UNBAN,
            ClientOpcodeMessage::CMSG_CHANNEL_ANNOUNCEMENTS(_) => Self::CMSG_CHANNEL_ANNOUNCEMENTS,
            ClientOpcodeMessage::CMSG_CHANNEL_MODERATE(_) => Self::CMSG_CHANNEL_MODERATE,
            ClientOpcodeMessage::SMSG_UPDATE_OBJECT(_) => Self::SMSG_UPDATE_OBJECT,
            ClientOpcodeMessage::CMSG_USE_ITEM(_) => Self::CMSG_USE_ITEM,
            ClientOpcodeMessage::CMSG_OPEN_ITEM(_) => Self::CMSG_OPEN_ITEM,
            ClientOpcodeMessage::CMSG_READ_ITEM(_) => Self::CMSG_READ_ITEM,
            ClientOpcodeMessage::CMSG_GAMEOBJ_USE(_) => Self::CMSG_GAMEOBJ_USE,
            ClientOpcodeMessage::CMSG_AREATRIGGER(_) => Self::CMSG_AREATRIGGER,
            ClientOpcodeMessage::CMSG_MOVE_SET_RAW_POSITION(_) => Self::CMSG_MOVE_SET_RAW_POSITION,
            ClientOpcodeMessage::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(_) => Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK,
            ClientOpcodeMessage::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(_) => Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK,
            ClientOpcodeMessage::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(_) => Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK,
            ClientOpcodeMessage::CMSG_FORCE_MOVE_ROOT_ACK(_) => Self::CMSG_FORCE_MOVE_ROOT_ACK,
            ClientOpcodeMessage::CMSG_FORCE_MOVE_UNROOT_ACK(_) => Self::CMSG_FORCE_MOVE_UNROOT_ACK,
            ClientOpcodeMessage::CMSG_MOVE_KNOCK_BACK_ACK(_) => Self::CMSG_MOVE_KNOCK_BACK_ACK,
            ClientOpcodeMessage::CMSG_MOVE_HOVER_ACK(_) => Self::CMSG_MOVE_HOVER_ACK,
            ClientOpcodeMessage::CMSG_NEXT_CINEMATIC_CAMERA(_) => Self::CMSG_NEXT_CINEMATIC_CAMERA,
            ClientOpcodeMessage::CMSG_COMPLETE_CINEMATIC(_) => Self::CMSG_COMPLETE_CINEMATIC,
            ClientOpcodeMessage::CMSG_TUTORIAL_FLAG(_) => Self::CMSG_TUTORIAL_FLAG,
            ClientOpcodeMessage::CMSG_TUTORIAL_CLEAR(_) => Self::CMSG_TUTORIAL_CLEAR,
            ClientOpcodeMessage::CMSG_TUTORIAL_RESET(_) => Self::CMSG_TUTORIAL_RESET,
            ClientOpcodeMessage::CMSG_STANDSTATECHANGE(_) => Self::CMSG_STANDSTATECHANGE,
            ClientOpcodeMessage::CMSG_EMOTE(_) => Self::CMSG_EMOTE,
            ClientOpcodeMessage::CMSG_TEXT_EMOTE(_) => Self::CMSG_TEXT_EMOTE,
            ClientOpcodeMessage::CMSG_AUTOSTORE_LOOT_ITEM(_) => Self::CMSG_AUTOSTORE_LOOT_ITEM,
            ClientOpcodeMessage::CMSG_AUTOEQUIP_ITEM(_) => Self::CMSG_AUTOEQUIP_ITEM,
            ClientOpcodeMessage::CMSG_AUTOSTORE_BAG_ITEM(_) => Self::CMSG_AUTOSTORE_BAG_ITEM,
            ClientOpcodeMessage::CMSG_SWAP_ITEM(_) => Self::CMSG_SWAP_ITEM,
            ClientOpcodeMessage::CMSG_SWAP_INV_ITEM(_) => Self::CMSG_SWAP_INV_ITEM,
            ClientOpcodeMessage::CMSG_SPLIT_ITEM(_) => Self::CMSG_SPLIT_ITEM,
            ClientOpcodeMessage::CMSG_AUTOEQUIP_ITEM_SLOT(_) => Self::CMSG_AUTOEQUIP_ITEM_SLOT,
            ClientOpcodeMessage::CMSG_DESTROYITEM(_) => Self::CMSG_DESTROYITEM,
            ClientOpcodeMessage::CMSG_INSPECT(_) => Self::CMSG_INSPECT,
            ClientOpcodeMessage::CMSG_INITIATE_TRADE(_) => Self::CMSG_INITIATE_TRADE,
            ClientOpcodeMessage::CMSG_BEGIN_TRADE(_) => Self::CMSG_BEGIN_TRADE,
            ClientOpcodeMessage::CMSG_BUSY_TRADE(_) => Self::CMSG_BUSY_TRADE,
            ClientOpcodeMessage::CMSG_IGNORE_TRADE(_) => Self::CMSG_IGNORE_TRADE,
            ClientOpcodeMessage::CMSG_ACCEPT_TRADE(_) => Self::CMSG_ACCEPT_TRADE,
            ClientOpcodeMessage::CMSG_UNACCEPT_TRADE(_) => Self::CMSG_UNACCEPT_TRADE,
            ClientOpcodeMessage::CMSG_CANCEL_TRADE(_) => Self::CMSG_CANCEL_TRADE,
            ClientOpcodeMessage::CMSG_SET_TRADE_ITEM(_) => Self::CMSG_SET_TRADE_ITEM,
            ClientOpcodeMessage::CMSG_CLEAR_TRADE_ITEM(_) => Self::CMSG_CLEAR_TRADE_ITEM,
            ClientOpcodeMessage::CMSG_SET_TRADE_GOLD(_) => Self::CMSG_SET_TRADE_GOLD,
            ClientOpcodeMessage::CMSG_SET_FACTION_ATWAR(_) => Self::CMSG_SET_FACTION_ATWAR,
            ClientOpcodeMessage::CMSG_SET_ACTION_BUTTON(_) => Self::CMSG_SET_ACTION_BUTTON,
            ClientOpcodeMessage::CMSG_CAST_SPELL(_) => Self::CMSG_CAST_SPELL,
            ClientOpcodeMessage::CMSG_CANCEL_CAST(_) => Self::CMSG_CANCEL_CAST,
            ClientOpcodeMessage::CMSG_CANCEL_AURA(_) => Self::CMSG_CANCEL_AURA,
            ClientOpcodeMessage::CMSG_CANCEL_CHANNELLING(_) => Self::CMSG_CANCEL_CHANNELLING,
            ClientOpcodeMessage::CMSG_SET_SELECTION(_) => Self::CMSG_SET_SELECTION,
            ClientOpcodeMessage::CMSG_SET_TARGET_OBSOLETE(_) => Self::CMSG_SET_TARGET_OBSOLETE,
            ClientOpcodeMessage::CMSG_ATTACKSWING(_) => Self::CMSG_ATTACKSWING,
            ClientOpcodeMessage::CMSG_ATTACKSTOP(_) => Self::CMSG_ATTACKSTOP,
            ClientOpcodeMessage::CMSG_REPOP_REQUEST(_) => Self::CMSG_REPOP_REQUEST,
            ClientOpcodeMessage::CMSG_RESURRECT_RESPONSE(_) => Self::CMSG_RESURRECT_RESPONSE,
            ClientOpcodeMessage::CMSG_LOOT(_) => Self::CMSG_LOOT,
            ClientOpcodeMessage::CMSG_LOOT_MONEY(_) => Self::CMSG_LOOT_MONEY,
            ClientOpcodeMessage::CMSG_LOOT_RELEASE(_) => Self::CMSG_LOOT_RELEASE,
            ClientOpcodeMessage::CMSG_DUEL_ACCEPTED(_) => Self::CMSG_DUEL_ACCEPTED,
            ClientOpcodeMessage::CMSG_DUEL_CANCELLED(_) => Self::CMSG_DUEL_CANCELLED,
            ClientOpcodeMessage::CMSG_MOUNTSPECIAL_ANIM(_) => Self::CMSG_MOUNTSPECIAL_ANIM,
            ClientOpcodeMessage::CMSG_PET_SET_ACTION(_) => Self::CMSG_PET_SET_ACTION,
            ClientOpcodeMessage::CMSG_PET_ACTION(_) => Self::CMSG_PET_ACTION,
            ClientOpcodeMessage::CMSG_PET_ABANDON(_) => Self::CMSG_PET_ABANDON,
            ClientOpcodeMessage::CMSG_PET_RENAME(_) => Self::CMSG_PET_RENAME,
            ClientOpcodeMessage::CMSG_GOSSIP_HELLO(_) => Self::CMSG_GOSSIP_HELLO,
            ClientOpcodeMessage::CMSG_GOSSIP_SELECT_OPTION(_) => Self::CMSG_GOSSIP_SELECT_OPTION,
            ClientOpcodeMessage::CMSG_NPC_TEXT_QUERY(_) => Self::CMSG_NPC_TEXT_QUERY,
            ClientOpcodeMessage::CMSG_QUESTGIVER_STATUS_QUERY(_) => Self::CMSG_QUESTGIVER_STATUS_QUERY,
            ClientOpcodeMessage::CMSG_QUESTGIVER_HELLO(_) => Self::CMSG_QUESTGIVER_HELLO,
            ClientOpcodeMessage::CMSG_QUESTGIVER_QUERY_QUEST(_) => Self::CMSG_QUESTGIVER_QUERY_QUEST,
            ClientOpcodeMessage::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(_) => Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH,
            ClientOpcodeMessage::CMSG_QUESTGIVER_ACCEPT_QUEST(_) => Self::CMSG_QUESTGIVER_ACCEPT_QUEST,
            ClientOpcodeMessage::CMSG_QUESTGIVER_COMPLETE_QUEST(_) => Self::CMSG_QUESTGIVER_COMPLETE_QUEST,
            ClientOpcodeMessage::CMSG_QUESTGIVER_REQUEST_REWARD(_) => Self::CMSG_QUESTGIVER_REQUEST_REWARD,
            ClientOpcodeMessage::CMSG_QUESTGIVER_CHOOSE_REWARD(_) => Self::CMSG_QUESTGIVER_CHOOSE_REWARD,
            ClientOpcodeMessage::CMSG_QUESTGIVER_CANCEL(_) => Self::CMSG_QUESTGIVER_CANCEL,
            ClientOpcodeMessage::CMSG_QUESTLOG_SWAP_QUEST(_) => Self::CMSG_QUESTLOG_SWAP_QUEST,
            ClientOpcodeMessage::CMSG_QUESTLOG_REMOVE_QUEST(_) => Self::CMSG_QUESTLOG_REMOVE_QUEST,
            ClientOpcodeMessage::CMSG_QUEST_CONFIRM_ACCEPT(_) => Self::CMSG_QUEST_CONFIRM_ACCEPT,
            ClientOpcodeMessage::CMSG_PUSHQUESTTOPARTY(_) => Self::CMSG_PUSHQUESTTOPARTY,
            ClientOpcodeMessage::CMSG_LIST_INVENTORY(_) => Self::CMSG_LIST_INVENTORY,
            ClientOpcodeMessage::CMSG_SELL_ITEM(_) => Self::CMSG_SELL_ITEM,
            ClientOpcodeMessage::CMSG_BUY_ITEM(_) => Self::CMSG_BUY_ITEM,
            ClientOpcodeMessage::CMSG_BUY_ITEM_IN_SLOT(_) => Self::CMSG_BUY_ITEM_IN_SLOT,
            ClientOpcodeMessage::CMSG_TAXINODE_STATUS_QUERY(_) => Self::CMSG_TAXINODE_STATUS_QUERY,
            ClientOpcodeMessage::CMSG_TAXIQUERYAVAILABLENODES(_) => Self::CMSG_TAXIQUERYAVAILABLENODES,
            ClientOpcodeMessage::CMSG_ACTIVATETAXI(_) => Self::CMSG_ACTIVATETAXI,
            ClientOpcodeMessage::CMSG_TRAINER_LIST(_) => Self::CMSG_TRAINER_LIST,
            ClientOpcodeMessage::CMSG_TRAINER_BUY_SPELL(_) => Self::CMSG_TRAINER_BUY_SPELL,
            ClientOpcodeMessage::CMSG_BINDER_ACTIVATE(_) => Self::CMSG_BINDER_ACTIVATE,
            ClientOpcodeMessage::CMSG_BANKER_ACTIVATE(_) => Self::CMSG_BANKER_ACTIVATE,
            ClientOpcodeMessage::CMSG_BUY_BANK_SLOT(_) => Self::CMSG_BUY_BANK_SLOT,
            ClientOpcodeMessage::CMSG_PETITION_SHOWLIST(_) => Self::CMSG_PETITION_SHOWLIST,
            ClientOpcodeMessage::CMSG_PETITION_BUY(_) => Self::CMSG_PETITION_BUY,
            ClientOpcodeMessage::CMSG_PETITION_SHOW_SIGNATURES(_) => Self::CMSG_PETITION_SHOW_SIGNATURES,
            ClientOpcodeMessage::CMSG_PETITION_SIGN(_) => Self::CMSG_PETITION_SIGN,
            ClientOpcodeMessage::CMSG_OFFER_PETITION(_) => Self::CMSG_OFFER_PETITION,
            ClientOpcodeMessage::CMSG_TURN_IN_PETITION(_) => Self::CMSG_TURN_IN_PETITION,
            ClientOpcodeMessage::CMSG_PETITION_QUERY(_) => Self::CMSG_PETITION_QUERY,
            ClientOpcodeMessage::CMSG_BUG(_) => Self::CMSG_BUG,
            ClientOpcodeMessage::CMSG_PLAYED_TIME(_) => Self::CMSG_PLAYED_TIME,
            ClientOpcodeMessage::CMSG_QUERY_TIME(_) => Self::CMSG_QUERY_TIME,
            ClientOpcodeMessage::CMSG_RECLAIM_CORPSE(_) => Self::CMSG_RECLAIM_CORPSE,
            ClientOpcodeMessage::CMSG_WRAP_ITEM(_) => Self::CMSG_WRAP_ITEM,
            ClientOpcodeMessage::MSG_MINIMAP_PING(_) => Self::MSG_MINIMAP_PING,
            ClientOpcodeMessage::CMSG_PING(_) => Self::CMSG_PING,
            ClientOpcodeMessage::CMSG_SETSHEATHED(_) => Self::CMSG_SETSHEATHED,
            ClientOpcodeMessage::CMSG_AUTH_SESSION(_) => Self::CMSG_AUTH_SESSION,
            ClientOpcodeMessage::CMSG_PET_CAST_SPELL(_) => Self::CMSG_PET_CAST_SPELL,
            ClientOpcodeMessage::MSG_SAVE_GUILD_EMBLEM(_) => Self::MSG_SAVE_GUILD_EMBLEM,
            ClientOpcodeMessage::CMSG_ZONEUPDATE(_) => Self::CMSG_ZONEUPDATE,
            ClientOpcodeMessage::MSG_RANDOM_ROLL(_) => Self::MSG_RANDOM_ROLL,
            ClientOpcodeMessage::MSG_LOOKING_FOR_GROUP(_) => Self::MSG_LOOKING_FOR_GROUP,
            ClientOpcodeMessage::CMSG_UNLEARN_SKILL(_) => Self::CMSG_UNLEARN_SKILL,
            ClientOpcodeMessage::CMSG_GMTICKET_CREATE(_) => Self::CMSG_GMTICKET_CREATE,
            ClientOpcodeMessage::CMSG_GMTICKET_UPDATETEXT(_) => Self::CMSG_GMTICKET_UPDATETEXT,
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(_) => Self::CMSG_REQUEST_ACCOUNT_DATA,
            ClientOpcodeMessage::CMSG_GMTICKET_GETTICKET(_) => Self::CMSG_GMTICKET_GETTICKET,
            ClientOpcodeMessage::MSG_CORPSE_QUERY(_) => Self::MSG_CORPSE_QUERY,
            ClientOpcodeMessage::CMSG_GMTICKET_DELETETICKET(_) => Self::CMSG_GMTICKET_DELETETICKET,
            ClientOpcodeMessage::CMSG_GMTICKET_SYSTEMSTATUS(_) => Self::CMSG_GMTICKET_SYSTEMSTATUS,
            ClientOpcodeMessage::CMSG_SPIRIT_HEALER_ACTIVATE(_) => Self::CMSG_SPIRIT_HEALER_ACTIVATE,
            ClientOpcodeMessage::CMSG_CHAT_IGNORED(_) => Self::CMSG_CHAT_IGNORED,
            ClientOpcodeMessage::CMSG_GUILD_RANK(_) => Self::CMSG_GUILD_RANK,
            ClientOpcodeMessage::CMSG_GUILD_ADD_RANK(_) => Self::CMSG_GUILD_ADD_RANK,
            ClientOpcodeMessage::CMSG_GUILD_DEL_RANK(_) => Self::CMSG_GUILD_DEL_RANK,
            ClientOpcodeMessage::CMSG_GUILD_SET_PUBLIC_NOTE(_) => Self::CMSG_GUILD_SET_PUBLIC_NOTE,
            ClientOpcodeMessage::CMSG_GUILD_SET_OFFICER_NOTE(_) => Self::CMSG_GUILD_SET_OFFICER_NOTE,
            ClientOpcodeMessage::CMSG_SEND_MAIL(_) => Self::CMSG_SEND_MAIL,
            ClientOpcodeMessage::CMSG_GET_MAIL_LIST(_) => Self::CMSG_GET_MAIL_LIST,
            ClientOpcodeMessage::CMSG_BATTLEFIELD_LIST(_) => Self::CMSG_BATTLEFIELD_LIST,
            ClientOpcodeMessage::CMSG_BATTLEFIELD_JOIN(_) => Self::CMSG_BATTLEFIELD_JOIN,
            ClientOpcodeMessage::CMSG_ITEM_TEXT_QUERY(_) => Self::CMSG_ITEM_TEXT_QUERY,
            ClientOpcodeMessage::CMSG_MAIL_TAKE_MONEY(_) => Self::CMSG_MAIL_TAKE_MONEY,
            ClientOpcodeMessage::CMSG_MAIL_TAKE_ITEM(_) => Self::CMSG_MAIL_TAKE_ITEM,
            ClientOpcodeMessage::CMSG_MAIL_MARK_AS_READ(_) => Self::CMSG_MAIL_MARK_AS_READ,
            ClientOpcodeMessage::CMSG_MAIL_RETURN_TO_SENDER(_) => Self::CMSG_MAIL_RETURN_TO_SENDER,
            ClientOpcodeMessage::CMSG_MAIL_DELETE(_) => Self::CMSG_MAIL_DELETE,
            ClientOpcodeMessage::CMSG_MAIL_CREATE_TEXT_ITEM(_) => Self::CMSG_MAIL_CREATE_TEXT_ITEM,
            ClientOpcodeMessage::CMSG_LEARN_TALENT(_) => Self::CMSG_LEARN_TALENT,
            ClientOpcodeMessage::CMSG_TOGGLE_PVP(_) => Self::CMSG_TOGGLE_PVP,
            ClientOpcodeMessage::MSG_AUCTION_HELLO(_) => Self::MSG_AUCTION_HELLO,
            ClientOpcodeMessage::CMSG_AUCTION_SELL_ITEM(_) => Self::CMSG_AUCTION_SELL_ITEM,
            ClientOpcodeMessage::CMSG_AUCTION_REMOVE_ITEM(_) => Self::CMSG_AUCTION_REMOVE_ITEM,
            ClientOpcodeMessage::CMSG_AUCTION_LIST_ITEMS(_) => Self::CMSG_AUCTION_LIST_ITEMS,
            ClientOpcodeMessage::CMSG_AUCTION_LIST_OWNER_ITEMS(_) => Self::CMSG_AUCTION_LIST_OWNER_ITEMS,
            ClientOpcodeMessage::CMSG_AUCTION_PLACE_BID(_) => Self::CMSG_AUCTION_PLACE_BID,
            ClientOpcodeMessage::CMSG_AUCTION_LIST_BIDDER_ITEMS(_) => Self::CMSG_AUCTION_LIST_BIDDER_ITEMS,
            ClientOpcodeMessage::CMSG_SET_AMMO(_) => Self::CMSG_SET_AMMO,
            ClientOpcodeMessage::CMSG_SET_ACTIVE_MOVER(_) => Self::CMSG_SET_ACTIVE_MOVER,
            ClientOpcodeMessage::CMSG_PET_CANCEL_AURA(_) => Self::CMSG_PET_CANCEL_AURA,
            ClientOpcodeMessage::CMSG_CANCEL_AUTO_REPEAT_SPELL(_) => Self::CMSG_CANCEL_AUTO_REPEAT_SPELL,
            ClientOpcodeMessage::MSG_LIST_STABLED_PETS(_) => Self::MSG_LIST_STABLED_PETS,
            ClientOpcodeMessage::CMSG_STABLE_PET(_) => Self::CMSG_STABLE_PET,
            ClientOpcodeMessage::CMSG_UNSTABLE_PET(_) => Self::CMSG_UNSTABLE_PET,
            ClientOpcodeMessage::CMSG_BUY_STABLE_SLOT(_) => Self::CMSG_BUY_STABLE_SLOT,
            ClientOpcodeMessage::CMSG_STABLE_SWAP_PET(_) => Self::CMSG_STABLE_SWAP_PET,
            ClientOpcodeMessage::CMSG_REQUEST_PET_INFO(_) => Self::CMSG_REQUEST_PET_INFO,
            ClientOpcodeMessage::CMSG_FAR_SIGHT(_) => Self::CMSG_FAR_SIGHT,
            ClientOpcodeMessage::CMSG_GROUP_CHANGE_SUB_GROUP(_) => Self::CMSG_GROUP_CHANGE_SUB_GROUP,
            ClientOpcodeMessage::CMSG_REQUEST_PARTY_MEMBER_STATS(_) => Self::CMSG_REQUEST_PARTY_MEMBER_STATS,
            ClientOpcodeMessage::CMSG_GROUP_SWAP_SUB_GROUP(_) => Self::CMSG_GROUP_SWAP_SUB_GROUP,
            ClientOpcodeMessage::CMSG_AUTOSTORE_BANK_ITEM(_) => Self::CMSG_AUTOSTORE_BANK_ITEM,
            ClientOpcodeMessage::CMSG_AUTOBANK_ITEM(_) => Self::CMSG_AUTOBANK_ITEM,
            ClientOpcodeMessage::MSG_QUERY_NEXT_MAIL_TIME(_) => Self::MSG_QUERY_NEXT_MAIL_TIME,
            ClientOpcodeMessage::CMSG_GROUP_RAID_CONVERT(_) => Self::CMSG_GROUP_RAID_CONVERT,
            ClientOpcodeMessage::CMSG_GROUP_ASSISTANT_LEADER(_) => Self::CMSG_GROUP_ASSISTANT_LEADER,
            ClientOpcodeMessage::CMSG_BUYBACK_ITEM(_) => Self::CMSG_BUYBACK_ITEM,
            ClientOpcodeMessage::CMSG_MEETINGSTONE_JOIN(_) => Self::CMSG_MEETINGSTONE_JOIN,
            ClientOpcodeMessage::CMSG_MEETINGSTONE_LEAVE(_) => Self::CMSG_MEETINGSTONE_LEAVE,
            ClientOpcodeMessage::CMSG_MEETINGSTONE_INFO(_) => Self::CMSG_MEETINGSTONE_INFO,
            ClientOpcodeMessage::CMSG_CANCEL_GROWTH_AURA(_) => Self::CMSG_CANCEL_GROWTH_AURA,
            ClientOpcodeMessage::CMSG_LOOT_ROLL(_) => Self::CMSG_LOOT_ROLL,
            ClientOpcodeMessage::CMSG_LOOT_MASTER_GIVE(_) => Self::CMSG_LOOT_MASTER_GIVE,
            ClientOpcodeMessage::CMSG_REPAIR_ITEM(_) => Self::CMSG_REPAIR_ITEM,
            ClientOpcodeMessage::MSG_TALENT_WIPE_CONFIRM(_) => Self::MSG_TALENT_WIPE_CONFIRM,
            ClientOpcodeMessage::CMSG_SUMMON_RESPONSE(_) => Self::CMSG_SUMMON_RESPONSE,
            ClientOpcodeMessage::CMSG_SELF_RES(_) => Self::CMSG_SELF_RES,
            ClientOpcodeMessage::CMSG_TOGGLE_HELM(_) => Self::CMSG_TOGGLE_HELM,
            ClientOpcodeMessage::CMSG_TOGGLE_CLOAK(_) => Self::CMSG_TOGGLE_CLOAK,
            ClientOpcodeMessage::CMSG_SET_ACTIONBAR_TOGGLES(_) => Self::CMSG_SET_ACTIONBAR_TOGGLES,
            ClientOpcodeMessage::CMSG_ITEM_NAME_QUERY(_) => Self::CMSG_ITEM_NAME_QUERY,
            ClientOpcodeMessage::CMSG_CHAR_RENAME(_) => Self::CMSG_CHAR_RENAME,
            ClientOpcodeMessage::CMSG_MOVE_SPLINE_DONE(_) => Self::CMSG_MOVE_SPLINE_DONE,
            ClientOpcodeMessage::CMSG_MOVE_FALL_RESET(_) => Self::CMSG_MOVE_FALL_RESET,
            ClientOpcodeMessage::CMSG_REQUEST_RAID_INFO(_) => Self::CMSG_REQUEST_RAID_INFO,
            ClientOpcodeMessage::CMSG_MOVE_TIME_SKIPPED(_) => Self::CMSG_MOVE_TIME_SKIPPED,
            ClientOpcodeMessage::CMSG_MOVE_FEATHER_FALL_ACK(_) => Self::CMSG_MOVE_FEATHER_FALL_ACK,
            ClientOpcodeMessage::CMSG_MOVE_WATER_WALK_ACK(_) => Self::CMSG_MOVE_WATER_WALK_ACK,
            ClientOpcodeMessage::CMSG_MOVE_NOT_ACTIVE_MOVER(_) => Self::CMSG_MOVE_NOT_ACTIVE_MOVER,
            ClientOpcodeMessage::CMSG_BATTLEFIELD_STATUS(_) => Self::CMSG_BATTLEFIELD_STATUS,
            ClientOpcodeMessage::CMSG_BATTLEFIELD_PORT(_) => Self::CMSG_BATTLEFIELD_PORT,
            ClientOpcodeMessage::MSG_INSPECT_HONOR_STATS(_) => Self::MSG_INSPECT_HONOR_STATS,
            ClientOpcodeMessage::CMSG_BATTLEMASTER_HELLO(_) => Self::CMSG_BATTLEMASTER_HELLO,
            ClientOpcodeMessage::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(_) => Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK,
            ClientOpcodeMessage::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(_) => Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK,
            ClientOpcodeMessage::CMSG_FORCE_TURN_RATE_CHANGE_ACK(_) => Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK,
            ClientOpcodeMessage::MSG_PVP_LOG_DATA(_) => Self::MSG_PVP_LOG_DATA,
            ClientOpcodeMessage::CMSG_LEAVE_BATTLEFIELD(_) => Self::CMSG_LEAVE_BATTLEFIELD,
            ClientOpcodeMessage::CMSG_AREA_SPIRIT_HEALER_QUERY(_) => Self::CMSG_AREA_SPIRIT_HEALER_QUERY,
            ClientOpcodeMessage::CMSG_AREA_SPIRIT_HEALER_QUEUE(_) => Self::CMSG_AREA_SPIRIT_HEALER_QUEUE,
            ClientOpcodeMessage::MSG_BATTLEGROUND_PLAYER_POSITIONS(_) => Self::MSG_BATTLEGROUND_PLAYER_POSITIONS,
            ClientOpcodeMessage::CMSG_PET_STOP_ATTACK(_) => Self::CMSG_PET_STOP_ATTACK,
            ClientOpcodeMessage::CMSG_BATTLEMASTER_JOIN(_) => Self::CMSG_BATTLEMASTER_JOIN,
            ClientOpcodeMessage::CMSG_PET_UNLEARN(_) => Self::CMSG_PET_UNLEARN,
            ClientOpcodeMessage::CMSG_PET_SPELL_AUTOCAST(_) => Self::CMSG_PET_SPELL_AUTOCAST,
            ClientOpcodeMessage::CMSG_GUILD_INFO_TEXT(_) => Self::CMSG_GUILD_INFO_TEXT,
            ClientOpcodeMessage::CMSG_ACTIVATETAXIEXPRESS(_) => Self::CMSG_ACTIVATETAXIEXPRESS,
            ClientOpcodeMessage::CMSG_SET_FACTION_INACTIVE(_) => Self::CMSG_SET_FACTION_INACTIVE,
            ClientOpcodeMessage::CMSG_SET_WATCHED_FACTION(_) => Self::CMSG_SET_WATCHED_FACTION,
            ClientOpcodeMessage::CMSG_RESET_INSTANCES(_) => Self::CMSG_RESET_INSTANCES,
            ClientOpcodeMessage::MSG_RAID_TARGET_UPDATE(_) => Self::MSG_RAID_TARGET_UPDATE,
            ClientOpcodeMessage::MSG_RAID_READY_CHECK(_) => Self::MSG_RAID_READY_CHECK,
            ClientOpcodeMessage::CMSG_GMSURVEY_SUBMIT(_) => Self::CMSG_GMSURVEY_SUBMIT,
        }
    }
}

#[derive(Debug)]
pub enum ClientOpcodeError {
    Io(std::io::Error),
    InvalidOpcode(u32),
}

impl std::error::Error for ClientOpcodeError {
}

impl std::fmt::Display for ClientOpcodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode for Client: '{}'", i)),
        }
    }
}

impl From<std::io::Error> for ClientOpcodeError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug)]
pub enum ClientOpcodeMessage {
    MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD),
    MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD),
    MSG_MOVE_STOP(MSG_MOVE_STOP),
    MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT),
    MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT),
    MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE),
    MSG_MOVE_JUMP(MSG_MOVE_JUMP),
    MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT),
    MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT),
    MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN),
    MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP),
    MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN),
    MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH),
    MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE),
    MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE),
    MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK),
    MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND),
    MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM),
    MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM),
    MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING),
    MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH),
    MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK),
    MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT),
    MSG_PETITION_DECLINE(MSG_PETITION_DECLINE),
    MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE),
    MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT),
    MSG_PETITION_RENAME(MSG_PETITION_RENAME),
    CMSG_WORLD_TELEPORT(CMSG_WORLD_TELEPORT),
    CMSG_CHAR_CREATE(CMSG_CHAR_CREATE),
    CMSG_CHAR_ENUM(CMSG_CHAR_ENUM),
    CMSG_CHAR_DELETE(CMSG_CHAR_DELETE),
    CMSG_PLAYER_LOGIN(CMSG_PLAYER_LOGIN),
    CMSG_PLAYER_LOGOUT(CMSG_PLAYER_LOGOUT),
    CMSG_LOGOUT_REQUEST(CMSG_LOGOUT_REQUEST),
    CMSG_LOGOUT_CANCEL(CMSG_LOGOUT_CANCEL),
    CMSG_NAME_QUERY(CMSG_NAME_QUERY),
    CMSG_PET_NAME_QUERY(CMSG_PET_NAME_QUERY),
    CMSG_GUILD_QUERY(CMSG_GUILD_QUERY),
    CMSG_ITEM_QUERY_SINGLE(CMSG_ITEM_QUERY_SINGLE),
    CMSG_PAGE_TEXT_QUERY(CMSG_PAGE_TEXT_QUERY),
    CMSG_QUEST_QUERY(CMSG_QUEST_QUERY),
    CMSG_GAMEOBJECT_QUERY(CMSG_GAMEOBJECT_QUERY),
    CMSG_CREATURE_QUERY(CMSG_CREATURE_QUERY),
    CMSG_WHO(CMSG_WHO),
    CMSG_WHOIS(CMSG_WHOIS),
    CMSG_FRIEND_LIST(CMSG_FRIEND_LIST),
    CMSG_ADD_FRIEND(CMSG_ADD_FRIEND),
    CMSG_DEL_FRIEND(CMSG_DEL_FRIEND),
    CMSG_ADD_IGNORE(CMSG_ADD_IGNORE),
    CMSG_DEL_IGNORE(CMSG_DEL_IGNORE),
    CMSG_GROUP_INVITE(CMSG_GROUP_INVITE),
    CMSG_GROUP_ACCEPT(CMSG_GROUP_ACCEPT),
    CMSG_GROUP_DECLINE(CMSG_GROUP_DECLINE),
    CMSG_GROUP_UNINVITE(CMSG_GROUP_UNINVITE),
    CMSG_GROUP_UNINVITE_GUID(CMSG_GROUP_UNINVITE_GUID),
    CMSG_GROUP_SET_LEADER(CMSG_GROUP_SET_LEADER),
    CMSG_LOOT_METHOD(CMSG_LOOT_METHOD),
    CMSG_GROUP_DISBAND(CMSG_GROUP_DISBAND),
    CMSG_GUILD_CREATE(CMSG_GUILD_CREATE),
    CMSG_GUILD_INVITE(CMSG_GUILD_INVITE),
    CMSG_GUILD_ACCEPT(CMSG_GUILD_ACCEPT),
    CMSG_GUILD_DECLINE(CMSG_GUILD_DECLINE),
    CMSG_GUILD_INFO(CMSG_GUILD_INFO),
    CMSG_GUILD_ROSTER(CMSG_GUILD_ROSTER),
    CMSG_GUILD_PROMOTE(CMSG_GUILD_PROMOTE),
    CMSG_GUILD_DEMOTE(CMSG_GUILD_DEMOTE),
    CMSG_GUILD_LEAVE(CMSG_GUILD_LEAVE),
    CMSG_GUILD_REMOVE(CMSG_GUILD_REMOVE),
    CMSG_GUILD_DISBAND(CMSG_GUILD_DISBAND),
    CMSG_GUILD_LEADER(CMSG_GUILD_LEADER),
    CMSG_GUILD_MOTD(CMSG_GUILD_MOTD),
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
    SMSG_UPDATE_OBJECT(SMSG_UPDATE_OBJECT),
    CMSG_USE_ITEM(CMSG_USE_ITEM),
    CMSG_OPEN_ITEM(CMSG_OPEN_ITEM),
    CMSG_READ_ITEM(CMSG_READ_ITEM),
    CMSG_GAMEOBJ_USE(CMSG_GAMEOBJ_USE),
    CMSG_AREATRIGGER(CMSG_AREATRIGGER),
    CMSG_MOVE_SET_RAW_POSITION(CMSG_MOVE_SET_RAW_POSITION),
    CMSG_FORCE_RUN_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_SPEED_CHANGE_ACK),
    CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK),
    CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_SPEED_CHANGE_ACK),
    CMSG_FORCE_MOVE_ROOT_ACK(CMSG_FORCE_MOVE_ROOT_ACK),
    CMSG_FORCE_MOVE_UNROOT_ACK(CMSG_FORCE_MOVE_UNROOT_ACK),
    CMSG_MOVE_KNOCK_BACK_ACK(CMSG_MOVE_KNOCK_BACK_ACK),
    CMSG_MOVE_HOVER_ACK(CMSG_MOVE_HOVER_ACK),
    CMSG_NEXT_CINEMATIC_CAMERA(CMSG_NEXT_CINEMATIC_CAMERA),
    CMSG_COMPLETE_CINEMATIC(CMSG_COMPLETE_CINEMATIC),
    CMSG_TUTORIAL_FLAG(CMSG_TUTORIAL_FLAG),
    CMSG_TUTORIAL_CLEAR(CMSG_TUTORIAL_CLEAR),
    CMSG_TUTORIAL_RESET(CMSG_TUTORIAL_RESET),
    CMSG_STANDSTATECHANGE(CMSG_STANDSTATECHANGE),
    CMSG_EMOTE(CMSG_EMOTE),
    CMSG_TEXT_EMOTE(CMSG_TEXT_EMOTE),
    CMSG_AUTOSTORE_LOOT_ITEM(CMSG_AUTOSTORE_LOOT_ITEM),
    CMSG_AUTOEQUIP_ITEM(CMSG_AUTOEQUIP_ITEM),
    CMSG_AUTOSTORE_BAG_ITEM(CMSG_AUTOSTORE_BAG_ITEM),
    CMSG_SWAP_ITEM(CMSG_SWAP_ITEM),
    CMSG_SWAP_INV_ITEM(CMSG_SWAP_INV_ITEM),
    CMSG_SPLIT_ITEM(CMSG_SPLIT_ITEM),
    CMSG_AUTOEQUIP_ITEM_SLOT(CMSG_AUTOEQUIP_ITEM_SLOT),
    CMSG_DESTROYITEM(CMSG_DESTROYITEM),
    CMSG_INSPECT(CMSG_INSPECT),
    CMSG_INITIATE_TRADE(CMSG_INITIATE_TRADE),
    CMSG_BEGIN_TRADE(CMSG_BEGIN_TRADE),
    CMSG_BUSY_TRADE(CMSG_BUSY_TRADE),
    CMSG_IGNORE_TRADE(CMSG_IGNORE_TRADE),
    CMSG_ACCEPT_TRADE(CMSG_ACCEPT_TRADE),
    CMSG_UNACCEPT_TRADE(CMSG_UNACCEPT_TRADE),
    CMSG_CANCEL_TRADE(CMSG_CANCEL_TRADE),
    CMSG_SET_TRADE_ITEM(CMSG_SET_TRADE_ITEM),
    CMSG_CLEAR_TRADE_ITEM(CMSG_CLEAR_TRADE_ITEM),
    CMSG_SET_TRADE_GOLD(CMSG_SET_TRADE_GOLD),
    CMSG_SET_FACTION_ATWAR(CMSG_SET_FACTION_ATWAR),
    CMSG_SET_ACTION_BUTTON(CMSG_SET_ACTION_BUTTON),
    CMSG_CAST_SPELL(CMSG_CAST_SPELL),
    CMSG_CANCEL_CAST(CMSG_CANCEL_CAST),
    CMSG_CANCEL_AURA(CMSG_CANCEL_AURA),
    CMSG_CANCEL_CHANNELLING(CMSG_CANCEL_CHANNELLING),
    CMSG_SET_SELECTION(CMSG_SET_SELECTION),
    CMSG_SET_TARGET_OBSOLETE(CMSG_SET_TARGET_OBSOLETE),
    CMSG_ATTACKSWING(CMSG_ATTACKSWING),
    CMSG_ATTACKSTOP(CMSG_ATTACKSTOP),
    CMSG_REPOP_REQUEST(CMSG_REPOP_REQUEST),
    CMSG_RESURRECT_RESPONSE(CMSG_RESURRECT_RESPONSE),
    CMSG_LOOT(CMSG_LOOT),
    CMSG_LOOT_MONEY(CMSG_LOOT_MONEY),
    CMSG_LOOT_RELEASE(CMSG_LOOT_RELEASE),
    CMSG_DUEL_ACCEPTED(CMSG_DUEL_ACCEPTED),
    CMSG_DUEL_CANCELLED(CMSG_DUEL_CANCELLED),
    CMSG_MOUNTSPECIAL_ANIM(CMSG_MOUNTSPECIAL_ANIM),
    CMSG_PET_SET_ACTION(CMSG_PET_SET_ACTION),
    CMSG_PET_ACTION(CMSG_PET_ACTION),
    CMSG_PET_ABANDON(CMSG_PET_ABANDON),
    CMSG_PET_RENAME(CMSG_PET_RENAME),
    CMSG_GOSSIP_HELLO(CMSG_GOSSIP_HELLO),
    CMSG_GOSSIP_SELECT_OPTION(CMSG_GOSSIP_SELECT_OPTION),
    CMSG_NPC_TEXT_QUERY(CMSG_NPC_TEXT_QUERY),
    CMSG_QUESTGIVER_STATUS_QUERY(CMSG_QUESTGIVER_STATUS_QUERY),
    CMSG_QUESTGIVER_HELLO(CMSG_QUESTGIVER_HELLO),
    CMSG_QUESTGIVER_QUERY_QUEST(CMSG_QUESTGIVER_QUERY_QUEST),
    CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(CMSG_QUESTGIVER_QUEST_AUTOLAUNCH),
    CMSG_QUESTGIVER_ACCEPT_QUEST(CMSG_QUESTGIVER_ACCEPT_QUEST),
    CMSG_QUESTGIVER_COMPLETE_QUEST(CMSG_QUESTGIVER_COMPLETE_QUEST),
    CMSG_QUESTGIVER_REQUEST_REWARD(CMSG_QUESTGIVER_REQUEST_REWARD),
    CMSG_QUESTGIVER_CHOOSE_REWARD(CMSG_QUESTGIVER_CHOOSE_REWARD),
    CMSG_QUESTGIVER_CANCEL(CMSG_QUESTGIVER_CANCEL),
    CMSG_QUESTLOG_SWAP_QUEST(CMSG_QUESTLOG_SWAP_QUEST),
    CMSG_QUESTLOG_REMOVE_QUEST(CMSG_QUESTLOG_REMOVE_QUEST),
    CMSG_QUEST_CONFIRM_ACCEPT(CMSG_QUEST_CONFIRM_ACCEPT),
    CMSG_PUSHQUESTTOPARTY(CMSG_PUSHQUESTTOPARTY),
    CMSG_LIST_INVENTORY(CMSG_LIST_INVENTORY),
    CMSG_SELL_ITEM(CMSG_SELL_ITEM),
    CMSG_BUY_ITEM(CMSG_BUY_ITEM),
    CMSG_BUY_ITEM_IN_SLOT(CMSG_BUY_ITEM_IN_SLOT),
    CMSG_TAXINODE_STATUS_QUERY(CMSG_TAXINODE_STATUS_QUERY),
    CMSG_TAXIQUERYAVAILABLENODES(CMSG_TAXIQUERYAVAILABLENODES),
    CMSG_ACTIVATETAXI(CMSG_ACTIVATETAXI),
    CMSG_TRAINER_LIST(CMSG_TRAINER_LIST),
    CMSG_TRAINER_BUY_SPELL(CMSG_TRAINER_BUY_SPELL),
    CMSG_BINDER_ACTIVATE(CMSG_BINDER_ACTIVATE),
    CMSG_BANKER_ACTIVATE(CMSG_BANKER_ACTIVATE),
    CMSG_BUY_BANK_SLOT(CMSG_BUY_BANK_SLOT),
    CMSG_PETITION_SHOWLIST(CMSG_PETITION_SHOWLIST),
    CMSG_PETITION_BUY(CMSG_PETITION_BUY),
    CMSG_PETITION_SHOW_SIGNATURES(CMSG_PETITION_SHOW_SIGNATURES),
    CMSG_PETITION_SIGN(CMSG_PETITION_SIGN),
    CMSG_OFFER_PETITION(CMSG_OFFER_PETITION),
    CMSG_TURN_IN_PETITION(CMSG_TURN_IN_PETITION),
    CMSG_PETITION_QUERY(CMSG_PETITION_QUERY),
    CMSG_BUG(CMSG_BUG),
    CMSG_PLAYED_TIME(CMSG_PLAYED_TIME),
    CMSG_QUERY_TIME(CMSG_QUERY_TIME),
    CMSG_RECLAIM_CORPSE(CMSG_RECLAIM_CORPSE),
    CMSG_WRAP_ITEM(CMSG_WRAP_ITEM),
    MSG_MINIMAP_PING(MSG_MINIMAP_PING_Client),
    CMSG_PING(CMSG_PING),
    CMSG_SETSHEATHED(CMSG_SETSHEATHED),
    CMSG_AUTH_SESSION(CMSG_AUTH_SESSION),
    CMSG_PET_CAST_SPELL(CMSG_PET_CAST_SPELL),
    MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Client),
    CMSG_ZONEUPDATE(CMSG_ZONEUPDATE),
    MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Client),
    MSG_LOOKING_FOR_GROUP(MSG_LOOKING_FOR_GROUP_Client),
    CMSG_UNLEARN_SKILL(CMSG_UNLEARN_SKILL),
    CMSG_GMTICKET_CREATE(CMSG_GMTICKET_CREATE),
    CMSG_GMTICKET_UPDATETEXT(CMSG_GMTICKET_UPDATETEXT),
    CMSG_REQUEST_ACCOUNT_DATA(CMSG_REQUEST_ACCOUNT_DATA),
    CMSG_GMTICKET_GETTICKET(CMSG_GMTICKET_GETTICKET),
    MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Client),
    CMSG_GMTICKET_DELETETICKET(CMSG_GMTICKET_DELETETICKET),
    CMSG_GMTICKET_SYSTEMSTATUS(CMSG_GMTICKET_SYSTEMSTATUS),
    CMSG_SPIRIT_HEALER_ACTIVATE(CMSG_SPIRIT_HEALER_ACTIVATE),
    CMSG_CHAT_IGNORED(CMSG_CHAT_IGNORED),
    CMSG_GUILD_RANK(CMSG_GUILD_RANK),
    CMSG_GUILD_ADD_RANK(CMSG_GUILD_ADD_RANK),
    CMSG_GUILD_DEL_RANK(CMSG_GUILD_DEL_RANK),
    CMSG_GUILD_SET_PUBLIC_NOTE(CMSG_GUILD_SET_PUBLIC_NOTE),
    CMSG_GUILD_SET_OFFICER_NOTE(CMSG_GUILD_SET_OFFICER_NOTE),
    CMSG_SEND_MAIL(CMSG_SEND_MAIL),
    CMSG_GET_MAIL_LIST(CMSG_GET_MAIL_LIST),
    CMSG_BATTLEFIELD_LIST(CMSG_BATTLEFIELD_LIST),
    CMSG_BATTLEFIELD_JOIN(CMSG_BATTLEFIELD_JOIN),
    CMSG_ITEM_TEXT_QUERY(CMSG_ITEM_TEXT_QUERY),
    CMSG_MAIL_TAKE_MONEY(CMSG_MAIL_TAKE_MONEY),
    CMSG_MAIL_TAKE_ITEM(CMSG_MAIL_TAKE_ITEM),
    CMSG_MAIL_MARK_AS_READ(CMSG_MAIL_MARK_AS_READ),
    CMSG_MAIL_RETURN_TO_SENDER(CMSG_MAIL_RETURN_TO_SENDER),
    CMSG_MAIL_DELETE(CMSG_MAIL_DELETE),
    CMSG_MAIL_CREATE_TEXT_ITEM(CMSG_MAIL_CREATE_TEXT_ITEM),
    CMSG_LEARN_TALENT(CMSG_LEARN_TALENT),
    CMSG_TOGGLE_PVP(CMSG_TOGGLE_PVP),
    MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Client),
    CMSG_AUCTION_SELL_ITEM(CMSG_AUCTION_SELL_ITEM),
    CMSG_AUCTION_REMOVE_ITEM(CMSG_AUCTION_REMOVE_ITEM),
    CMSG_AUCTION_LIST_ITEMS(CMSG_AUCTION_LIST_ITEMS),
    CMSG_AUCTION_LIST_OWNER_ITEMS(CMSG_AUCTION_LIST_OWNER_ITEMS),
    CMSG_AUCTION_PLACE_BID(CMSG_AUCTION_PLACE_BID),
    CMSG_AUCTION_LIST_BIDDER_ITEMS(CMSG_AUCTION_LIST_BIDDER_ITEMS),
    CMSG_SET_AMMO(CMSG_SET_AMMO),
    CMSG_SET_ACTIVE_MOVER(CMSG_SET_ACTIVE_MOVER),
    CMSG_PET_CANCEL_AURA(CMSG_PET_CANCEL_AURA),
    CMSG_CANCEL_AUTO_REPEAT_SPELL(CMSG_CANCEL_AUTO_REPEAT_SPELL),
    MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Client),
    CMSG_STABLE_PET(CMSG_STABLE_PET),
    CMSG_UNSTABLE_PET(CMSG_UNSTABLE_PET),
    CMSG_BUY_STABLE_SLOT(CMSG_BUY_STABLE_SLOT),
    CMSG_STABLE_SWAP_PET(CMSG_STABLE_SWAP_PET),
    CMSG_REQUEST_PET_INFO(CMSG_REQUEST_PET_INFO),
    CMSG_FAR_SIGHT(CMSG_FAR_SIGHT),
    CMSG_GROUP_CHANGE_SUB_GROUP(CMSG_GROUP_CHANGE_SUB_GROUP),
    CMSG_REQUEST_PARTY_MEMBER_STATS(CMSG_REQUEST_PARTY_MEMBER_STATS),
    CMSG_GROUP_SWAP_SUB_GROUP(CMSG_GROUP_SWAP_SUB_GROUP),
    CMSG_AUTOSTORE_BANK_ITEM(CMSG_AUTOSTORE_BANK_ITEM),
    CMSG_AUTOBANK_ITEM(CMSG_AUTOBANK_ITEM),
    MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Client),
    CMSG_GROUP_RAID_CONVERT(CMSG_GROUP_RAID_CONVERT),
    CMSG_GROUP_ASSISTANT_LEADER(CMSG_GROUP_ASSISTANT_LEADER),
    CMSG_BUYBACK_ITEM(CMSG_BUYBACK_ITEM),
    CMSG_MEETINGSTONE_JOIN(CMSG_MEETINGSTONE_JOIN),
    CMSG_MEETINGSTONE_LEAVE(CMSG_MEETINGSTONE_LEAVE),
    CMSG_MEETINGSTONE_INFO(CMSG_MEETINGSTONE_INFO),
    CMSG_CANCEL_GROWTH_AURA(CMSG_CANCEL_GROWTH_AURA),
    CMSG_LOOT_ROLL(CMSG_LOOT_ROLL),
    CMSG_LOOT_MASTER_GIVE(CMSG_LOOT_MASTER_GIVE),
    CMSG_REPAIR_ITEM(CMSG_REPAIR_ITEM),
    MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Client),
    CMSG_SUMMON_RESPONSE(CMSG_SUMMON_RESPONSE),
    CMSG_SELF_RES(CMSG_SELF_RES),
    CMSG_TOGGLE_HELM(CMSG_TOGGLE_HELM),
    CMSG_TOGGLE_CLOAK(CMSG_TOGGLE_CLOAK),
    CMSG_SET_ACTIONBAR_TOGGLES(CMSG_SET_ACTIONBAR_TOGGLES),
    CMSG_ITEM_NAME_QUERY(CMSG_ITEM_NAME_QUERY),
    CMSG_CHAR_RENAME(CMSG_CHAR_RENAME),
    CMSG_MOVE_SPLINE_DONE(CMSG_MOVE_SPLINE_DONE),
    CMSG_MOVE_FALL_RESET(CMSG_MOVE_FALL_RESET),
    CMSG_REQUEST_RAID_INFO(CMSG_REQUEST_RAID_INFO),
    CMSG_MOVE_TIME_SKIPPED(CMSG_MOVE_TIME_SKIPPED),
    CMSG_MOVE_FEATHER_FALL_ACK(CMSG_MOVE_FEATHER_FALL_ACK),
    CMSG_MOVE_WATER_WALK_ACK(CMSG_MOVE_WATER_WALK_ACK),
    CMSG_MOVE_NOT_ACTIVE_MOVER(CMSG_MOVE_NOT_ACTIVE_MOVER),
    CMSG_BATTLEFIELD_STATUS(CMSG_BATTLEFIELD_STATUS),
    CMSG_BATTLEFIELD_PORT(CMSG_BATTLEFIELD_PORT),
    MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Client),
    CMSG_BATTLEMASTER_HELLO(CMSG_BATTLEMASTER_HELLO),
    CMSG_FORCE_WALK_SPEED_CHANGE_ACK(CMSG_FORCE_WALK_SPEED_CHANGE_ACK),
    CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK),
    CMSG_FORCE_TURN_RATE_CHANGE_ACK(CMSG_FORCE_TURN_RATE_CHANGE_ACK),
    MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_Client),
    CMSG_LEAVE_BATTLEFIELD(CMSG_LEAVE_BATTLEFIELD),
    CMSG_AREA_SPIRIT_HEALER_QUERY(CMSG_AREA_SPIRIT_HEALER_QUERY),
    CMSG_AREA_SPIRIT_HEALER_QUEUE(CMSG_AREA_SPIRIT_HEALER_QUEUE),
    MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Client),
    CMSG_PET_STOP_ATTACK(CMSG_PET_STOP_ATTACK),
    CMSG_BATTLEMASTER_JOIN(CMSG_BATTLEMASTER_JOIN),
    CMSG_PET_UNLEARN(CMSG_PET_UNLEARN),
    CMSG_PET_SPELL_AUTOCAST(CMSG_PET_SPELL_AUTOCAST),
    CMSG_GUILD_INFO_TEXT(CMSG_GUILD_INFO_TEXT),
    CMSG_ACTIVATETAXIEXPRESS(CMSG_ACTIVATETAXIEXPRESS),
    CMSG_SET_FACTION_INACTIVE(CMSG_SET_FACTION_INACTIVE),
    CMSG_SET_WATCHED_FACTION(CMSG_SET_WATCHED_FACTION),
    CMSG_RESET_INSTANCES(CMSG_RESET_INSTANCES),
    MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Client),
    MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Client),
    CMSG_GMSURVEY_SUBMIT(CMSG_GMSURVEY_SUBMIT),
}

impl ClientOpcodeMessage {

    #[cfg(feature = "sync")]
    pub fn read_unencrypted<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let size = (crate::util::read_u16_be(r)? - 4) as u32;
        let opcode = crate::util::read_u32_le(r)?;
        match opcode {
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD::read_body(r, size)?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD::read_body(r, size)?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(MSG_MOVE_STOP::read_body(r, size)?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT::read_body(r, size)?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT::read_body(r, size)?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE::read_body(r, size)?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(MSG_MOVE_JUMP::read_body(r, size)?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT::read_body(r, size)?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT::read_body(r, size)?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN::read_body(r, size)?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP::read_body(r, size)?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN::read_body(r, size)?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH::read_body(r, size)?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE::read_body(r, size)?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE::read_body(r, size)?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK::read_body(r, size)?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND::read_body(r, size)?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM::read_body(r, size)?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM::read_body(r, size)?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING::read_body(r, size)?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH::read_body(r, size)?)),
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK::read_body(r, size)?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT::read_body(r, size)?)),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(MSG_PETITION_DECLINE::read_body(r, size)?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE::read_body(r, size)?)),
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT::read_body(r, size)?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(MSG_PETITION_RENAME::read_body(r, size)?)),
            0x0008 => Ok(Self::CMSG_WORLD_TELEPORT(CMSG_WORLD_TELEPORT::read_body(r, size)?)),
            0x0036 => Ok(Self::CMSG_CHAR_CREATE(CMSG_CHAR_CREATE::read_body(r, size)?)),
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(CMSG_CHAR_ENUM::read_body(r, size)?)),
            0x0038 => Ok(Self::CMSG_CHAR_DELETE(CMSG_CHAR_DELETE::read_body(r, size)?)),
            0x003D => Ok(Self::CMSG_PLAYER_LOGIN(CMSG_PLAYER_LOGIN::read_body(r, size)?)),
            0x004A => Ok(Self::CMSG_PLAYER_LOGOUT(CMSG_PLAYER_LOGOUT::read_body(r, size)?)),
            0x004B => Ok(Self::CMSG_LOGOUT_REQUEST(CMSG_LOGOUT_REQUEST::read_body(r, size)?)),
            0x004E => Ok(Self::CMSG_LOGOUT_CANCEL(CMSG_LOGOUT_CANCEL::read_body(r, size)?)),
            0x0050 => Ok(Self::CMSG_NAME_QUERY(CMSG_NAME_QUERY::read_body(r, size)?)),
            0x0052 => Ok(Self::CMSG_PET_NAME_QUERY(CMSG_PET_NAME_QUERY::read_body(r, size)?)),
            0x0054 => Ok(Self::CMSG_GUILD_QUERY(CMSG_GUILD_QUERY::read_body(r, size)?)),
            0x0056 => Ok(Self::CMSG_ITEM_QUERY_SINGLE(CMSG_ITEM_QUERY_SINGLE::read_body(r, size)?)),
            0x005A => Ok(Self::CMSG_PAGE_TEXT_QUERY(CMSG_PAGE_TEXT_QUERY::read_body(r, size)?)),
            0x005C => Ok(Self::CMSG_QUEST_QUERY(CMSG_QUEST_QUERY::read_body(r, size)?)),
            0x005E => Ok(Self::CMSG_GAMEOBJECT_QUERY(CMSG_GAMEOBJECT_QUERY::read_body(r, size)?)),
            0x0060 => Ok(Self::CMSG_CREATURE_QUERY(CMSG_CREATURE_QUERY::read_body(r, size)?)),
            0x0062 => Ok(Self::CMSG_WHO(CMSG_WHO::read_body(r, size)?)),
            0x0064 => Ok(Self::CMSG_WHOIS(CMSG_WHOIS::read_body(r, size)?)),
            0x0066 => Ok(Self::CMSG_FRIEND_LIST(CMSG_FRIEND_LIST::read_body(r, size)?)),
            0x0069 => Ok(Self::CMSG_ADD_FRIEND(CMSG_ADD_FRIEND::read_body(r, size)?)),
            0x006A => Ok(Self::CMSG_DEL_FRIEND(CMSG_DEL_FRIEND::read_body(r, size)?)),
            0x006C => Ok(Self::CMSG_ADD_IGNORE(CMSG_ADD_IGNORE::read_body(r, size)?)),
            0x006D => Ok(Self::CMSG_DEL_IGNORE(CMSG_DEL_IGNORE::read_body(r, size)?)),
            0x006E => Ok(Self::CMSG_GROUP_INVITE(CMSG_GROUP_INVITE::read_body(r, size)?)),
            0x0072 => Ok(Self::CMSG_GROUP_ACCEPT(CMSG_GROUP_ACCEPT::read_body(r, size)?)),
            0x0073 => Ok(Self::CMSG_GROUP_DECLINE(CMSG_GROUP_DECLINE::read_body(r, size)?)),
            0x0075 => Ok(Self::CMSG_GROUP_UNINVITE(CMSG_GROUP_UNINVITE::read_body(r, size)?)),
            0x0076 => Ok(Self::CMSG_GROUP_UNINVITE_GUID(CMSG_GROUP_UNINVITE_GUID::read_body(r, size)?)),
            0x0078 => Ok(Self::CMSG_GROUP_SET_LEADER(CMSG_GROUP_SET_LEADER::read_body(r, size)?)),
            0x007A => Ok(Self::CMSG_LOOT_METHOD(CMSG_LOOT_METHOD::read_body(r, size)?)),
            0x007B => Ok(Self::CMSG_GROUP_DISBAND(CMSG_GROUP_DISBAND::read_body(r, size)?)),
            0x0081 => Ok(Self::CMSG_GUILD_CREATE(CMSG_GUILD_CREATE::read_body(r, size)?)),
            0x0082 => Ok(Self::CMSG_GUILD_INVITE(CMSG_GUILD_INVITE::read_body(r, size)?)),
            0x0084 => Ok(Self::CMSG_GUILD_ACCEPT(CMSG_GUILD_ACCEPT::read_body(r, size)?)),
            0x0085 => Ok(Self::CMSG_GUILD_DECLINE(CMSG_GUILD_DECLINE::read_body(r, size)?)),
            0x0087 => Ok(Self::CMSG_GUILD_INFO(CMSG_GUILD_INFO::read_body(r, size)?)),
            0x0089 => Ok(Self::CMSG_GUILD_ROSTER(CMSG_GUILD_ROSTER::read_body(r, size)?)),
            0x008B => Ok(Self::CMSG_GUILD_PROMOTE(CMSG_GUILD_PROMOTE::read_body(r, size)?)),
            0x008C => Ok(Self::CMSG_GUILD_DEMOTE(CMSG_GUILD_DEMOTE::read_body(r, size)?)),
            0x008D => Ok(Self::CMSG_GUILD_LEAVE(CMSG_GUILD_LEAVE::read_body(r, size)?)),
            0x008E => Ok(Self::CMSG_GUILD_REMOVE(CMSG_GUILD_REMOVE::read_body(r, size)?)),
            0x008F => Ok(Self::CMSG_GUILD_DISBAND(CMSG_GUILD_DISBAND::read_body(r, size)?)),
            0x0090 => Ok(Self::CMSG_GUILD_LEADER(CMSG_GUILD_LEADER::read_body(r, size)?)),
            0x0091 => Ok(Self::CMSG_GUILD_MOTD(CMSG_GUILD_MOTD::read_body(r, size)?)),
            0x0095 => Ok(Self::CMSG_MESSAGECHAT(CMSG_MESSAGECHAT::read_body(r, size)?)),
            0x0097 => Ok(Self::CMSG_JOIN_CHANNEL(CMSG_JOIN_CHANNEL::read_body(r, size)?)),
            0x0098 => Ok(Self::CMSG_LEAVE_CHANNEL(CMSG_LEAVE_CHANNEL::read_body(r, size)?)),
            0x009A => Ok(Self::CMSG_CHANNEL_LIST(CMSG_CHANNEL_LIST::read_body(r, size)?)),
            0x009C => Ok(Self::CMSG_CHANNEL_PASSWORD(CMSG_CHANNEL_PASSWORD::read_body(r, size)?)),
            0x009D => Ok(Self::CMSG_CHANNEL_SET_OWNER(CMSG_CHANNEL_SET_OWNER::read_body(r, size)?)),
            0x009E => Ok(Self::CMSG_CHANNEL_OWNER(CMSG_CHANNEL_OWNER::read_body(r, size)?)),
            0x009F => Ok(Self::CMSG_CHANNEL_MODERATOR(CMSG_CHANNEL_MODERATOR::read_body(r, size)?)),
            0x00A0 => Ok(Self::CMSG_CHANNEL_UNMODERATOR(CMSG_CHANNEL_UNMODERATOR::read_body(r, size)?)),
            0x00A1 => Ok(Self::CMSG_CHANNEL_MUTE(CMSG_CHANNEL_MUTE::read_body(r, size)?)),
            0x00A2 => Ok(Self::CMSG_CHANNEL_UNMUTE(CMSG_CHANNEL_UNMUTE::read_body(r, size)?)),
            0x00A3 => Ok(Self::CMSG_CHANNEL_INVITE(CMSG_CHANNEL_INVITE::read_body(r, size)?)),
            0x00A4 => Ok(Self::CMSG_CHANNEL_KICK(CMSG_CHANNEL_KICK::read_body(r, size)?)),
            0x00A5 => Ok(Self::CMSG_CHANNEL_BAN(CMSG_CHANNEL_BAN::read_body(r, size)?)),
            0x00A6 => Ok(Self::CMSG_CHANNEL_UNBAN(CMSG_CHANNEL_UNBAN::read_body(r, size)?)),
            0x00A7 => Ok(Self::CMSG_CHANNEL_ANNOUNCEMENTS(CMSG_CHANNEL_ANNOUNCEMENTS::read_body(r, size)?)),
            0x00A8 => Ok(Self::CMSG_CHANNEL_MODERATE(CMSG_CHANNEL_MODERATE::read_body(r, size)?)),
            0x00A9 => Ok(Self::SMSG_UPDATE_OBJECT(SMSG_UPDATE_OBJECT::read_body(r, size)?)),
            0x00AB => Ok(Self::CMSG_USE_ITEM(CMSG_USE_ITEM::read_body(r, size)?)),
            0x00AC => Ok(Self::CMSG_OPEN_ITEM(CMSG_OPEN_ITEM::read_body(r, size)?)),
            0x00AD => Ok(Self::CMSG_READ_ITEM(CMSG_READ_ITEM::read_body(r, size)?)),
            0x00B1 => Ok(Self::CMSG_GAMEOBJ_USE(CMSG_GAMEOBJ_USE::read_body(r, size)?)),
            0x00B4 => Ok(Self::CMSG_AREATRIGGER(CMSG_AREATRIGGER::read_body(r, size)?)),
            0x00E1 => Ok(Self::CMSG_MOVE_SET_RAW_POSITION(CMSG_MOVE_SET_RAW_POSITION::read_body(r, size)?)),
            0x00E3 => Ok(Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_SPEED_CHANGE_ACK::read_body(r, size)?)),
            0x00E5 => Ok(Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK::read_body(r, size)?)),
            0x00E7 => Ok(Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_SPEED_CHANGE_ACK::read_body(r, size)?)),
            0x00E9 => Ok(Self::CMSG_FORCE_MOVE_ROOT_ACK(CMSG_FORCE_MOVE_ROOT_ACK::read_body(r, size)?)),
            0x00EB => Ok(Self::CMSG_FORCE_MOVE_UNROOT_ACK(CMSG_FORCE_MOVE_UNROOT_ACK::read_body(r, size)?)),
            0x00F0 => Ok(Self::CMSG_MOVE_KNOCK_BACK_ACK(CMSG_MOVE_KNOCK_BACK_ACK::read_body(r, size)?)),
            0x00F6 => Ok(Self::CMSG_MOVE_HOVER_ACK(CMSG_MOVE_HOVER_ACK::read_body(r, size)?)),
            0x00FB => Ok(Self::CMSG_NEXT_CINEMATIC_CAMERA(CMSG_NEXT_CINEMATIC_CAMERA::read_body(r, size)?)),
            0x00FC => Ok(Self::CMSG_COMPLETE_CINEMATIC(CMSG_COMPLETE_CINEMATIC::read_body(r, size)?)),
            0x00FE => Ok(Self::CMSG_TUTORIAL_FLAG(CMSG_TUTORIAL_FLAG::read_body(r, size)?)),
            0x00FF => Ok(Self::CMSG_TUTORIAL_CLEAR(CMSG_TUTORIAL_CLEAR::read_body(r, size)?)),
            0x0100 => Ok(Self::CMSG_TUTORIAL_RESET(CMSG_TUTORIAL_RESET::read_body(r, size)?)),
            0x0101 => Ok(Self::CMSG_STANDSTATECHANGE(CMSG_STANDSTATECHANGE::read_body(r, size)?)),
            0x0102 => Ok(Self::CMSG_EMOTE(CMSG_EMOTE::read_body(r, size)?)),
            0x0104 => Ok(Self::CMSG_TEXT_EMOTE(CMSG_TEXT_EMOTE::read_body(r, size)?)),
            0x0108 => Ok(Self::CMSG_AUTOSTORE_LOOT_ITEM(CMSG_AUTOSTORE_LOOT_ITEM::read_body(r, size)?)),
            0x010A => Ok(Self::CMSG_AUTOEQUIP_ITEM(CMSG_AUTOEQUIP_ITEM::read_body(r, size)?)),
            0x010B => Ok(Self::CMSG_AUTOSTORE_BAG_ITEM(CMSG_AUTOSTORE_BAG_ITEM::read_body(r, size)?)),
            0x010C => Ok(Self::CMSG_SWAP_ITEM(CMSG_SWAP_ITEM::read_body(r, size)?)),
            0x010D => Ok(Self::CMSG_SWAP_INV_ITEM(CMSG_SWAP_INV_ITEM::read_body(r, size)?)),
            0x010E => Ok(Self::CMSG_SPLIT_ITEM(CMSG_SPLIT_ITEM::read_body(r, size)?)),
            0x010F => Ok(Self::CMSG_AUTOEQUIP_ITEM_SLOT(CMSG_AUTOEQUIP_ITEM_SLOT::read_body(r, size)?)),
            0x0111 => Ok(Self::CMSG_DESTROYITEM(CMSG_DESTROYITEM::read_body(r, size)?)),
            0x0114 => Ok(Self::CMSG_INSPECT(CMSG_INSPECT::read_body(r, size)?)),
            0x0116 => Ok(Self::CMSG_INITIATE_TRADE(CMSG_INITIATE_TRADE::read_body(r, size)?)),
            0x0117 => Ok(Self::CMSG_BEGIN_TRADE(CMSG_BEGIN_TRADE::read_body(r, size)?)),
            0x0118 => Ok(Self::CMSG_BUSY_TRADE(CMSG_BUSY_TRADE::read_body(r, size)?)),
            0x0119 => Ok(Self::CMSG_IGNORE_TRADE(CMSG_IGNORE_TRADE::read_body(r, size)?)),
            0x011A => Ok(Self::CMSG_ACCEPT_TRADE(CMSG_ACCEPT_TRADE::read_body(r, size)?)),
            0x011B => Ok(Self::CMSG_UNACCEPT_TRADE(CMSG_UNACCEPT_TRADE::read_body(r, size)?)),
            0x011C => Ok(Self::CMSG_CANCEL_TRADE(CMSG_CANCEL_TRADE::read_body(r, size)?)),
            0x011D => Ok(Self::CMSG_SET_TRADE_ITEM(CMSG_SET_TRADE_ITEM::read_body(r, size)?)),
            0x011E => Ok(Self::CMSG_CLEAR_TRADE_ITEM(CMSG_CLEAR_TRADE_ITEM::read_body(r, size)?)),
            0x011F => Ok(Self::CMSG_SET_TRADE_GOLD(CMSG_SET_TRADE_GOLD::read_body(r, size)?)),
            0x0125 => Ok(Self::CMSG_SET_FACTION_ATWAR(CMSG_SET_FACTION_ATWAR::read_body(r, size)?)),
            0x0128 => Ok(Self::CMSG_SET_ACTION_BUTTON(CMSG_SET_ACTION_BUTTON::read_body(r, size)?)),
            0x012E => Ok(Self::CMSG_CAST_SPELL(CMSG_CAST_SPELL::read_body(r, size)?)),
            0x012F => Ok(Self::CMSG_CANCEL_CAST(CMSG_CANCEL_CAST::read_body(r, size)?)),
            0x0136 => Ok(Self::CMSG_CANCEL_AURA(CMSG_CANCEL_AURA::read_body(r, size)?)),
            0x013B => Ok(Self::CMSG_CANCEL_CHANNELLING(CMSG_CANCEL_CHANNELLING::read_body(r, size)?)),
            0x013D => Ok(Self::CMSG_SET_SELECTION(CMSG_SET_SELECTION::read_body(r, size)?)),
            0x013E => Ok(Self::CMSG_SET_TARGET_OBSOLETE(CMSG_SET_TARGET_OBSOLETE::read_body(r, size)?)),
            0x0141 => Ok(Self::CMSG_ATTACKSWING(CMSG_ATTACKSWING::read_body(r, size)?)),
            0x0142 => Ok(Self::CMSG_ATTACKSTOP(CMSG_ATTACKSTOP::read_body(r, size)?)),
            0x015A => Ok(Self::CMSG_REPOP_REQUEST(CMSG_REPOP_REQUEST::read_body(r, size)?)),
            0x015C => Ok(Self::CMSG_RESURRECT_RESPONSE(CMSG_RESURRECT_RESPONSE::read_body(r, size)?)),
            0x015D => Ok(Self::CMSG_LOOT(CMSG_LOOT::read_body(r, size)?)),
            0x015E => Ok(Self::CMSG_LOOT_MONEY(CMSG_LOOT_MONEY::read_body(r, size)?)),
            0x015F => Ok(Self::CMSG_LOOT_RELEASE(CMSG_LOOT_RELEASE::read_body(r, size)?)),
            0x016C => Ok(Self::CMSG_DUEL_ACCEPTED(CMSG_DUEL_ACCEPTED::read_body(r, size)?)),
            0x016D => Ok(Self::CMSG_DUEL_CANCELLED(CMSG_DUEL_CANCELLED::read_body(r, size)?)),
            0x0171 => Ok(Self::CMSG_MOUNTSPECIAL_ANIM(CMSG_MOUNTSPECIAL_ANIM::read_body(r, size)?)),
            0x0174 => Ok(Self::CMSG_PET_SET_ACTION(CMSG_PET_SET_ACTION::read_body(r, size)?)),
            0x0175 => Ok(Self::CMSG_PET_ACTION(CMSG_PET_ACTION::read_body(r, size)?)),
            0x0176 => Ok(Self::CMSG_PET_ABANDON(CMSG_PET_ABANDON::read_body(r, size)?)),
            0x0177 => Ok(Self::CMSG_PET_RENAME(CMSG_PET_RENAME::read_body(r, size)?)),
            0x017B => Ok(Self::CMSG_GOSSIP_HELLO(CMSG_GOSSIP_HELLO::read_body(r, size)?)),
            0x017C => Ok(Self::CMSG_GOSSIP_SELECT_OPTION(CMSG_GOSSIP_SELECT_OPTION::read_body(r, size)?)),
            0x017F => Ok(Self::CMSG_NPC_TEXT_QUERY(CMSG_NPC_TEXT_QUERY::read_body(r, size)?)),
            0x0182 => Ok(Self::CMSG_QUESTGIVER_STATUS_QUERY(CMSG_QUESTGIVER_STATUS_QUERY::read_body(r, size)?)),
            0x0184 => Ok(Self::CMSG_QUESTGIVER_HELLO(CMSG_QUESTGIVER_HELLO::read_body(r, size)?)),
            0x0186 => Ok(Self::CMSG_QUESTGIVER_QUERY_QUEST(CMSG_QUESTGIVER_QUERY_QUEST::read_body(r, size)?)),
            0x0187 => Ok(Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(CMSG_QUESTGIVER_QUEST_AUTOLAUNCH::read_body(r, size)?)),
            0x0189 => Ok(Self::CMSG_QUESTGIVER_ACCEPT_QUEST(CMSG_QUESTGIVER_ACCEPT_QUEST::read_body(r, size)?)),
            0x018A => Ok(Self::CMSG_QUESTGIVER_COMPLETE_QUEST(CMSG_QUESTGIVER_COMPLETE_QUEST::read_body(r, size)?)),
            0x018C => Ok(Self::CMSG_QUESTGIVER_REQUEST_REWARD(CMSG_QUESTGIVER_REQUEST_REWARD::read_body(r, size)?)),
            0x018E => Ok(Self::CMSG_QUESTGIVER_CHOOSE_REWARD(CMSG_QUESTGIVER_CHOOSE_REWARD::read_body(r, size)?)),
            0x0190 => Ok(Self::CMSG_QUESTGIVER_CANCEL(CMSG_QUESTGIVER_CANCEL::read_body(r, size)?)),
            0x0193 => Ok(Self::CMSG_QUESTLOG_SWAP_QUEST(CMSG_QUESTLOG_SWAP_QUEST::read_body(r, size)?)),
            0x0194 => Ok(Self::CMSG_QUESTLOG_REMOVE_QUEST(CMSG_QUESTLOG_REMOVE_QUEST::read_body(r, size)?)),
            0x019B => Ok(Self::CMSG_QUEST_CONFIRM_ACCEPT(CMSG_QUEST_CONFIRM_ACCEPT::read_body(r, size)?)),
            0x019D => Ok(Self::CMSG_PUSHQUESTTOPARTY(CMSG_PUSHQUESTTOPARTY::read_body(r, size)?)),
            0x019E => Ok(Self::CMSG_LIST_INVENTORY(CMSG_LIST_INVENTORY::read_body(r, size)?)),
            0x01A0 => Ok(Self::CMSG_SELL_ITEM(CMSG_SELL_ITEM::read_body(r, size)?)),
            0x01A2 => Ok(Self::CMSG_BUY_ITEM(CMSG_BUY_ITEM::read_body(r, size)?)),
            0x01A3 => Ok(Self::CMSG_BUY_ITEM_IN_SLOT(CMSG_BUY_ITEM_IN_SLOT::read_body(r, size)?)),
            0x01AA => Ok(Self::CMSG_TAXINODE_STATUS_QUERY(CMSG_TAXINODE_STATUS_QUERY::read_body(r, size)?)),
            0x01AC => Ok(Self::CMSG_TAXIQUERYAVAILABLENODES(CMSG_TAXIQUERYAVAILABLENODES::read_body(r, size)?)),
            0x01AD => Ok(Self::CMSG_ACTIVATETAXI(CMSG_ACTIVATETAXI::read_body(r, size)?)),
            0x01B0 => Ok(Self::CMSG_TRAINER_LIST(CMSG_TRAINER_LIST::read_body(r, size)?)),
            0x01B2 => Ok(Self::CMSG_TRAINER_BUY_SPELL(CMSG_TRAINER_BUY_SPELL::read_body(r, size)?)),
            0x01B5 => Ok(Self::CMSG_BINDER_ACTIVATE(CMSG_BINDER_ACTIVATE::read_body(r, size)?)),
            0x01B7 => Ok(Self::CMSG_BANKER_ACTIVATE(CMSG_BANKER_ACTIVATE::read_body(r, size)?)),
            0x01B9 => Ok(Self::CMSG_BUY_BANK_SLOT(CMSG_BUY_BANK_SLOT::read_body(r, size)?)),
            0x01BB => Ok(Self::CMSG_PETITION_SHOWLIST(CMSG_PETITION_SHOWLIST::read_body(r, size)?)),
            0x01BD => Ok(Self::CMSG_PETITION_BUY(CMSG_PETITION_BUY::read_body(r, size)?)),
            0x01BE => Ok(Self::CMSG_PETITION_SHOW_SIGNATURES(CMSG_PETITION_SHOW_SIGNATURES::read_body(r, size)?)),
            0x01C0 => Ok(Self::CMSG_PETITION_SIGN(CMSG_PETITION_SIGN::read_body(r, size)?)),
            0x01C3 => Ok(Self::CMSG_OFFER_PETITION(CMSG_OFFER_PETITION::read_body(r, size)?)),
            0x01C4 => Ok(Self::CMSG_TURN_IN_PETITION(CMSG_TURN_IN_PETITION::read_body(r, size)?)),
            0x01C6 => Ok(Self::CMSG_PETITION_QUERY(CMSG_PETITION_QUERY::read_body(r, size)?)),
            0x01CA => Ok(Self::CMSG_BUG(CMSG_BUG::read_body(r, size)?)),
            0x01CC => Ok(Self::CMSG_PLAYED_TIME(CMSG_PLAYED_TIME::read_body(r, size)?)),
            0x01CE => Ok(Self::CMSG_QUERY_TIME(CMSG_QUERY_TIME::read_body(r, size)?)),
            0x01D2 => Ok(Self::CMSG_RECLAIM_CORPSE(CMSG_RECLAIM_CORPSE::read_body(r, size)?)),
            0x01D3 => Ok(Self::CMSG_WRAP_ITEM(CMSG_WRAP_ITEM::read_body(r, size)?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(MSG_MINIMAP_PING_Client::read_body(r, size)?)),
            0x01DC => Ok(Self::CMSG_PING(CMSG_PING::read_body(r, size)?)),
            0x01E0 => Ok(Self::CMSG_SETSHEATHED(CMSG_SETSHEATHED::read_body(r, size)?)),
            0x01ED => Ok(Self::CMSG_AUTH_SESSION(CMSG_AUTH_SESSION::read_body(r, size)?)),
            0x01F0 => Ok(Self::CMSG_PET_CAST_SPELL(CMSG_PET_CAST_SPELL::read_body(r, size)?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Client::read_body(r, size)?)),
            0x01F4 => Ok(Self::CMSG_ZONEUPDATE(CMSG_ZONEUPDATE::read_body(r, size)?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Client::read_body(r, size)?)),
            0x01FF => Ok(Self::MSG_LOOKING_FOR_GROUP(MSG_LOOKING_FOR_GROUP_Client::read_body(r, size)?)),
            0x0202 => Ok(Self::CMSG_UNLEARN_SKILL(CMSG_UNLEARN_SKILL::read_body(r, size)?)),
            0x0205 => Ok(Self::CMSG_GMTICKET_CREATE(CMSG_GMTICKET_CREATE::read_body(r, size)?)),
            0x0207 => Ok(Self::CMSG_GMTICKET_UPDATETEXT(CMSG_GMTICKET_UPDATETEXT::read_body(r, size)?)),
            0x020A => Ok(Self::CMSG_REQUEST_ACCOUNT_DATA(CMSG_REQUEST_ACCOUNT_DATA::read_body(r, size)?)),
            0x0211 => Ok(Self::CMSG_GMTICKET_GETTICKET(CMSG_GMTICKET_GETTICKET::read_body(r, size)?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Client::read_body(r, size)?)),
            0x0217 => Ok(Self::CMSG_GMTICKET_DELETETICKET(CMSG_GMTICKET_DELETETICKET::read_body(r, size)?)),
            0x021A => Ok(Self::CMSG_GMTICKET_SYSTEMSTATUS(CMSG_GMTICKET_SYSTEMSTATUS::read_body(r, size)?)),
            0x021C => Ok(Self::CMSG_SPIRIT_HEALER_ACTIVATE(CMSG_SPIRIT_HEALER_ACTIVATE::read_body(r, size)?)),
            0x0225 => Ok(Self::CMSG_CHAT_IGNORED(CMSG_CHAT_IGNORED::read_body(r, size)?)),
            0x0231 => Ok(Self::CMSG_GUILD_RANK(CMSG_GUILD_RANK::read_body(r, size)?)),
            0x0232 => Ok(Self::CMSG_GUILD_ADD_RANK(CMSG_GUILD_ADD_RANK::read_body(r, size)?)),
            0x0233 => Ok(Self::CMSG_GUILD_DEL_RANK(CMSG_GUILD_DEL_RANK::read_body(r, size)?)),
            0x0234 => Ok(Self::CMSG_GUILD_SET_PUBLIC_NOTE(CMSG_GUILD_SET_PUBLIC_NOTE::read_body(r, size)?)),
            0x0235 => Ok(Self::CMSG_GUILD_SET_OFFICER_NOTE(CMSG_GUILD_SET_OFFICER_NOTE::read_body(r, size)?)),
            0x0238 => Ok(Self::CMSG_SEND_MAIL(CMSG_SEND_MAIL::read_body(r, size)?)),
            0x023A => Ok(Self::CMSG_GET_MAIL_LIST(CMSG_GET_MAIL_LIST::read_body(r, size)?)),
            0x023C => Ok(Self::CMSG_BATTLEFIELD_LIST(CMSG_BATTLEFIELD_LIST::read_body(r, size)?)),
            0x023E => Ok(Self::CMSG_BATTLEFIELD_JOIN(CMSG_BATTLEFIELD_JOIN::read_body(r, size)?)),
            0x0243 => Ok(Self::CMSG_ITEM_TEXT_QUERY(CMSG_ITEM_TEXT_QUERY::read_body(r, size)?)),
            0x0245 => Ok(Self::CMSG_MAIL_TAKE_MONEY(CMSG_MAIL_TAKE_MONEY::read_body(r, size)?)),
            0x0246 => Ok(Self::CMSG_MAIL_TAKE_ITEM(CMSG_MAIL_TAKE_ITEM::read_body(r, size)?)),
            0x0247 => Ok(Self::CMSG_MAIL_MARK_AS_READ(CMSG_MAIL_MARK_AS_READ::read_body(r, size)?)),
            0x0248 => Ok(Self::CMSG_MAIL_RETURN_TO_SENDER(CMSG_MAIL_RETURN_TO_SENDER::read_body(r, size)?)),
            0x0249 => Ok(Self::CMSG_MAIL_DELETE(CMSG_MAIL_DELETE::read_body(r, size)?)),
            0x024A => Ok(Self::CMSG_MAIL_CREATE_TEXT_ITEM(CMSG_MAIL_CREATE_TEXT_ITEM::read_body(r, size)?)),
            0x0251 => Ok(Self::CMSG_LEARN_TALENT(CMSG_LEARN_TALENT::read_body(r, size)?)),
            0x0253 => Ok(Self::CMSG_TOGGLE_PVP(CMSG_TOGGLE_PVP::read_body(r, size)?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Client::read_body(r, size)?)),
            0x0256 => Ok(Self::CMSG_AUCTION_SELL_ITEM(CMSG_AUCTION_SELL_ITEM::read_body(r, size)?)),
            0x0257 => Ok(Self::CMSG_AUCTION_REMOVE_ITEM(CMSG_AUCTION_REMOVE_ITEM::read_body(r, size)?)),
            0x0258 => Ok(Self::CMSG_AUCTION_LIST_ITEMS(CMSG_AUCTION_LIST_ITEMS::read_body(r, size)?)),
            0x0259 => Ok(Self::CMSG_AUCTION_LIST_OWNER_ITEMS(CMSG_AUCTION_LIST_OWNER_ITEMS::read_body(r, size)?)),
            0x025A => Ok(Self::CMSG_AUCTION_PLACE_BID(CMSG_AUCTION_PLACE_BID::read_body(r, size)?)),
            0x0264 => Ok(Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(CMSG_AUCTION_LIST_BIDDER_ITEMS::read_body(r, size)?)),
            0x0268 => Ok(Self::CMSG_SET_AMMO(CMSG_SET_AMMO::read_body(r, size)?)),
            0x026A => Ok(Self::CMSG_SET_ACTIVE_MOVER(CMSG_SET_ACTIVE_MOVER::read_body(r, size)?)),
            0x026B => Ok(Self::CMSG_PET_CANCEL_AURA(CMSG_PET_CANCEL_AURA::read_body(r, size)?)),
            0x026D => Ok(Self::CMSG_CANCEL_AUTO_REPEAT_SPELL(CMSG_CANCEL_AUTO_REPEAT_SPELL::read_body(r, size)?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Client::read_body(r, size)?)),
            0x0270 => Ok(Self::CMSG_STABLE_PET(CMSG_STABLE_PET::read_body(r, size)?)),
            0x0271 => Ok(Self::CMSG_UNSTABLE_PET(CMSG_UNSTABLE_PET::read_body(r, size)?)),
            0x0272 => Ok(Self::CMSG_BUY_STABLE_SLOT(CMSG_BUY_STABLE_SLOT::read_body(r, size)?)),
            0x0275 => Ok(Self::CMSG_STABLE_SWAP_PET(CMSG_STABLE_SWAP_PET::read_body(r, size)?)),
            0x0279 => Ok(Self::CMSG_REQUEST_PET_INFO(CMSG_REQUEST_PET_INFO::read_body(r, size)?)),
            0x027A => Ok(Self::CMSG_FAR_SIGHT(CMSG_FAR_SIGHT::read_body(r, size)?)),
            0x027E => Ok(Self::CMSG_GROUP_CHANGE_SUB_GROUP(CMSG_GROUP_CHANGE_SUB_GROUP::read_body(r, size)?)),
            0x027F => Ok(Self::CMSG_REQUEST_PARTY_MEMBER_STATS(CMSG_REQUEST_PARTY_MEMBER_STATS::read_body(r, size)?)),
            0x0280 => Ok(Self::CMSG_GROUP_SWAP_SUB_GROUP(CMSG_GROUP_SWAP_SUB_GROUP::read_body(r, size)?)),
            0x0282 => Ok(Self::CMSG_AUTOSTORE_BANK_ITEM(CMSG_AUTOSTORE_BANK_ITEM::read_body(r, size)?)),
            0x0283 => Ok(Self::CMSG_AUTOBANK_ITEM(CMSG_AUTOBANK_ITEM::read_body(r, size)?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Client::read_body(r, size)?)),
            0x028E => Ok(Self::CMSG_GROUP_RAID_CONVERT(CMSG_GROUP_RAID_CONVERT::read_body(r, size)?)),
            0x028F => Ok(Self::CMSG_GROUP_ASSISTANT_LEADER(CMSG_GROUP_ASSISTANT_LEADER::read_body(r, size)?)),
            0x0290 => Ok(Self::CMSG_BUYBACK_ITEM(CMSG_BUYBACK_ITEM::read_body(r, size)?)),
            0x0292 => Ok(Self::CMSG_MEETINGSTONE_JOIN(CMSG_MEETINGSTONE_JOIN::read_body(r, size)?)),
            0x0293 => Ok(Self::CMSG_MEETINGSTONE_LEAVE(CMSG_MEETINGSTONE_LEAVE::read_body(r, size)?)),
            0x0296 => Ok(Self::CMSG_MEETINGSTONE_INFO(CMSG_MEETINGSTONE_INFO::read_body(r, size)?)),
            0x029B => Ok(Self::CMSG_CANCEL_GROWTH_AURA(CMSG_CANCEL_GROWTH_AURA::read_body(r, size)?)),
            0x02A0 => Ok(Self::CMSG_LOOT_ROLL(CMSG_LOOT_ROLL::read_body(r, size)?)),
            0x02A3 => Ok(Self::CMSG_LOOT_MASTER_GIVE(CMSG_LOOT_MASTER_GIVE::read_body(r, size)?)),
            0x02A8 => Ok(Self::CMSG_REPAIR_ITEM(CMSG_REPAIR_ITEM::read_body(r, size)?)),
            0x02AA => Ok(Self::MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Client::read_body(r, size)?)),
            0x02AC => Ok(Self::CMSG_SUMMON_RESPONSE(CMSG_SUMMON_RESPONSE::read_body(r, size)?)),
            0x02B3 => Ok(Self::CMSG_SELF_RES(CMSG_SELF_RES::read_body(r, size)?)),
            0x02B9 => Ok(Self::CMSG_TOGGLE_HELM(CMSG_TOGGLE_HELM::read_body(r, size)?)),
            0x02BA => Ok(Self::CMSG_TOGGLE_CLOAK(CMSG_TOGGLE_CLOAK::read_body(r, size)?)),
            0x02BF => Ok(Self::CMSG_SET_ACTIONBAR_TOGGLES(CMSG_SET_ACTIONBAR_TOGGLES::read_body(r, size)?)),
            0x02C4 => Ok(Self::CMSG_ITEM_NAME_QUERY(CMSG_ITEM_NAME_QUERY::read_body(r, size)?)),
            0x02C7 => Ok(Self::CMSG_CHAR_RENAME(CMSG_CHAR_RENAME::read_body(r, size)?)),
            0x02C9 => Ok(Self::CMSG_MOVE_SPLINE_DONE(CMSG_MOVE_SPLINE_DONE::read_body(r, size)?)),
            0x02CA => Ok(Self::CMSG_MOVE_FALL_RESET(CMSG_MOVE_FALL_RESET::read_body(r, size)?)),
            0x02CD => Ok(Self::CMSG_REQUEST_RAID_INFO(CMSG_REQUEST_RAID_INFO::read_body(r, size)?)),
            0x02CE => Ok(Self::CMSG_MOVE_TIME_SKIPPED(CMSG_MOVE_TIME_SKIPPED::read_body(r, size)?)),
            0x02CF => Ok(Self::CMSG_MOVE_FEATHER_FALL_ACK(CMSG_MOVE_FEATHER_FALL_ACK::read_body(r, size)?)),
            0x02D0 => Ok(Self::CMSG_MOVE_WATER_WALK_ACK(CMSG_MOVE_WATER_WALK_ACK::read_body(r, size)?)),
            0x02D1 => Ok(Self::CMSG_MOVE_NOT_ACTIVE_MOVER(CMSG_MOVE_NOT_ACTIVE_MOVER::read_body(r, size)?)),
            0x02D3 => Ok(Self::CMSG_BATTLEFIELD_STATUS(CMSG_BATTLEFIELD_STATUS::read_body(r, size)?)),
            0x02D5 => Ok(Self::CMSG_BATTLEFIELD_PORT(CMSG_BATTLEFIELD_PORT::read_body(r, size)?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Client::read_body(r, size)?)),
            0x02D7 => Ok(Self::CMSG_BATTLEMASTER_HELLO(CMSG_BATTLEMASTER_HELLO::read_body(r, size)?)),
            0x02DB => Ok(Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(CMSG_FORCE_WALK_SPEED_CHANGE_ACK::read_body(r, size)?)),
            0x02DD => Ok(Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK::read_body(r, size)?)),
            0x02DF => Ok(Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(CMSG_FORCE_TURN_RATE_CHANGE_ACK::read_body(r, size)?)),
            0x02E0 => Ok(Self::MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_Client::read_body(r, size)?)),
            0x02E1 => Ok(Self::CMSG_LEAVE_BATTLEFIELD(CMSG_LEAVE_BATTLEFIELD::read_body(r, size)?)),
            0x02E2 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUERY(CMSG_AREA_SPIRIT_HEALER_QUERY::read_body(r, size)?)),
            0x02E3 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(CMSG_AREA_SPIRIT_HEALER_QUEUE::read_body(r, size)?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Client::read_body(r, size)?)),
            0x02EA => Ok(Self::CMSG_PET_STOP_ATTACK(CMSG_PET_STOP_ATTACK::read_body(r, size)?)),
            0x02EE => Ok(Self::CMSG_BATTLEMASTER_JOIN(CMSG_BATTLEMASTER_JOIN::read_body(r, size)?)),
            0x02F0 => Ok(Self::CMSG_PET_UNLEARN(CMSG_PET_UNLEARN::read_body(r, size)?)),
            0x02F3 => Ok(Self::CMSG_PET_SPELL_AUTOCAST(CMSG_PET_SPELL_AUTOCAST::read_body(r, size)?)),
            0x02FC => Ok(Self::CMSG_GUILD_INFO_TEXT(CMSG_GUILD_INFO_TEXT::read_body(r, size)?)),
            0x0312 => Ok(Self::CMSG_ACTIVATETAXIEXPRESS(CMSG_ACTIVATETAXIEXPRESS::read_body(r, size)?)),
            0x0317 => Ok(Self::CMSG_SET_FACTION_INACTIVE(CMSG_SET_FACTION_INACTIVE::read_body(r, size)?)),
            0x0318 => Ok(Self::CMSG_SET_WATCHED_FACTION(CMSG_SET_WATCHED_FACTION::read_body(r, size)?)),
            0x031D => Ok(Self::CMSG_RESET_INSTANCES(CMSG_RESET_INSTANCES::read_body(r, size)?)),
            0x0321 => Ok(Self::MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Client::read_body(r, size)?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Client::read_body(r, size)?)),
            0x032A => Ok(Self::CMSG_GMSURVEY_SUBMIT(CMSG_GMSURVEY_SUBMIT::read_body(r, size)?)),
            _ => Err(ClientOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }
    #[cfg(feature = "sync")]
    pub fn read_encrypted<R: std::io::Read, D: Decrypter>(r: &mut R, d: &mut D) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let mut header = [0u8; 6];
        r.read_exact(&mut header)?;
        let header = d.decrypt_client_header(header);
        let header_size = (header.size - 4) as u32;
        match header.opcode {
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD::read_body(r, header_size)?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD::read_body(r, header_size)?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(MSG_MOVE_STOP::read_body(r, header_size)?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT::read_body(r, header_size)?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT::read_body(r, header_size)?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE::read_body(r, header_size)?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(MSG_MOVE_JUMP::read_body(r, header_size)?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT::read_body(r, header_size)?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT::read_body(r, header_size)?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN::read_body(r, header_size)?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP::read_body(r, header_size)?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN::read_body(r, header_size)?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH::read_body(r, header_size)?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE::read_body(r, header_size)?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE::read_body(r, header_size)?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK::read_body(r, header_size)?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND::read_body(r, header_size)?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM::read_body(r, header_size)?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM::read_body(r, header_size)?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING::read_body(r, header_size)?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH::read_body(r, header_size)?)),
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK::read_body(r, header_size)?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT::read_body(r, header_size)?)),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(MSG_PETITION_DECLINE::read_body(r, header_size)?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE::read_body(r, header_size)?)),
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT::read_body(r, header_size)?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(MSG_PETITION_RENAME::read_body(r, header_size)?)),
            0x0008 => Ok(Self::CMSG_WORLD_TELEPORT(CMSG_WORLD_TELEPORT::read_body(r, header_size)?)),
            0x0036 => Ok(Self::CMSG_CHAR_CREATE(CMSG_CHAR_CREATE::read_body(r, header_size)?)),
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(CMSG_CHAR_ENUM::read_body(r, header_size)?)),
            0x0038 => Ok(Self::CMSG_CHAR_DELETE(CMSG_CHAR_DELETE::read_body(r, header_size)?)),
            0x003D => Ok(Self::CMSG_PLAYER_LOGIN(CMSG_PLAYER_LOGIN::read_body(r, header_size)?)),
            0x004A => Ok(Self::CMSG_PLAYER_LOGOUT(CMSG_PLAYER_LOGOUT::read_body(r, header_size)?)),
            0x004B => Ok(Self::CMSG_LOGOUT_REQUEST(CMSG_LOGOUT_REQUEST::read_body(r, header_size)?)),
            0x004E => Ok(Self::CMSG_LOGOUT_CANCEL(CMSG_LOGOUT_CANCEL::read_body(r, header_size)?)),
            0x0050 => Ok(Self::CMSG_NAME_QUERY(CMSG_NAME_QUERY::read_body(r, header_size)?)),
            0x0052 => Ok(Self::CMSG_PET_NAME_QUERY(CMSG_PET_NAME_QUERY::read_body(r, header_size)?)),
            0x0054 => Ok(Self::CMSG_GUILD_QUERY(CMSG_GUILD_QUERY::read_body(r, header_size)?)),
            0x0056 => Ok(Self::CMSG_ITEM_QUERY_SINGLE(CMSG_ITEM_QUERY_SINGLE::read_body(r, header_size)?)),
            0x005A => Ok(Self::CMSG_PAGE_TEXT_QUERY(CMSG_PAGE_TEXT_QUERY::read_body(r, header_size)?)),
            0x005C => Ok(Self::CMSG_QUEST_QUERY(CMSG_QUEST_QUERY::read_body(r, header_size)?)),
            0x005E => Ok(Self::CMSG_GAMEOBJECT_QUERY(CMSG_GAMEOBJECT_QUERY::read_body(r, header_size)?)),
            0x0060 => Ok(Self::CMSG_CREATURE_QUERY(CMSG_CREATURE_QUERY::read_body(r, header_size)?)),
            0x0062 => Ok(Self::CMSG_WHO(CMSG_WHO::read_body(r, header_size)?)),
            0x0064 => Ok(Self::CMSG_WHOIS(CMSG_WHOIS::read_body(r, header_size)?)),
            0x0066 => Ok(Self::CMSG_FRIEND_LIST(CMSG_FRIEND_LIST::read_body(r, header_size)?)),
            0x0069 => Ok(Self::CMSG_ADD_FRIEND(CMSG_ADD_FRIEND::read_body(r, header_size)?)),
            0x006A => Ok(Self::CMSG_DEL_FRIEND(CMSG_DEL_FRIEND::read_body(r, header_size)?)),
            0x006C => Ok(Self::CMSG_ADD_IGNORE(CMSG_ADD_IGNORE::read_body(r, header_size)?)),
            0x006D => Ok(Self::CMSG_DEL_IGNORE(CMSG_DEL_IGNORE::read_body(r, header_size)?)),
            0x006E => Ok(Self::CMSG_GROUP_INVITE(CMSG_GROUP_INVITE::read_body(r, header_size)?)),
            0x0072 => Ok(Self::CMSG_GROUP_ACCEPT(CMSG_GROUP_ACCEPT::read_body(r, header_size)?)),
            0x0073 => Ok(Self::CMSG_GROUP_DECLINE(CMSG_GROUP_DECLINE::read_body(r, header_size)?)),
            0x0075 => Ok(Self::CMSG_GROUP_UNINVITE(CMSG_GROUP_UNINVITE::read_body(r, header_size)?)),
            0x0076 => Ok(Self::CMSG_GROUP_UNINVITE_GUID(CMSG_GROUP_UNINVITE_GUID::read_body(r, header_size)?)),
            0x0078 => Ok(Self::CMSG_GROUP_SET_LEADER(CMSG_GROUP_SET_LEADER::read_body(r, header_size)?)),
            0x007A => Ok(Self::CMSG_LOOT_METHOD(CMSG_LOOT_METHOD::read_body(r, header_size)?)),
            0x007B => Ok(Self::CMSG_GROUP_DISBAND(CMSG_GROUP_DISBAND::read_body(r, header_size)?)),
            0x0081 => Ok(Self::CMSG_GUILD_CREATE(CMSG_GUILD_CREATE::read_body(r, header_size)?)),
            0x0082 => Ok(Self::CMSG_GUILD_INVITE(CMSG_GUILD_INVITE::read_body(r, header_size)?)),
            0x0084 => Ok(Self::CMSG_GUILD_ACCEPT(CMSG_GUILD_ACCEPT::read_body(r, header_size)?)),
            0x0085 => Ok(Self::CMSG_GUILD_DECLINE(CMSG_GUILD_DECLINE::read_body(r, header_size)?)),
            0x0087 => Ok(Self::CMSG_GUILD_INFO(CMSG_GUILD_INFO::read_body(r, header_size)?)),
            0x0089 => Ok(Self::CMSG_GUILD_ROSTER(CMSG_GUILD_ROSTER::read_body(r, header_size)?)),
            0x008B => Ok(Self::CMSG_GUILD_PROMOTE(CMSG_GUILD_PROMOTE::read_body(r, header_size)?)),
            0x008C => Ok(Self::CMSG_GUILD_DEMOTE(CMSG_GUILD_DEMOTE::read_body(r, header_size)?)),
            0x008D => Ok(Self::CMSG_GUILD_LEAVE(CMSG_GUILD_LEAVE::read_body(r, header_size)?)),
            0x008E => Ok(Self::CMSG_GUILD_REMOVE(CMSG_GUILD_REMOVE::read_body(r, header_size)?)),
            0x008F => Ok(Self::CMSG_GUILD_DISBAND(CMSG_GUILD_DISBAND::read_body(r, header_size)?)),
            0x0090 => Ok(Self::CMSG_GUILD_LEADER(CMSG_GUILD_LEADER::read_body(r, header_size)?)),
            0x0091 => Ok(Self::CMSG_GUILD_MOTD(CMSG_GUILD_MOTD::read_body(r, header_size)?)),
            0x0095 => Ok(Self::CMSG_MESSAGECHAT(CMSG_MESSAGECHAT::read_body(r, header_size)?)),
            0x0097 => Ok(Self::CMSG_JOIN_CHANNEL(CMSG_JOIN_CHANNEL::read_body(r, header_size)?)),
            0x0098 => Ok(Self::CMSG_LEAVE_CHANNEL(CMSG_LEAVE_CHANNEL::read_body(r, header_size)?)),
            0x009A => Ok(Self::CMSG_CHANNEL_LIST(CMSG_CHANNEL_LIST::read_body(r, header_size)?)),
            0x009C => Ok(Self::CMSG_CHANNEL_PASSWORD(CMSG_CHANNEL_PASSWORD::read_body(r, header_size)?)),
            0x009D => Ok(Self::CMSG_CHANNEL_SET_OWNER(CMSG_CHANNEL_SET_OWNER::read_body(r, header_size)?)),
            0x009E => Ok(Self::CMSG_CHANNEL_OWNER(CMSG_CHANNEL_OWNER::read_body(r, header_size)?)),
            0x009F => Ok(Self::CMSG_CHANNEL_MODERATOR(CMSG_CHANNEL_MODERATOR::read_body(r, header_size)?)),
            0x00A0 => Ok(Self::CMSG_CHANNEL_UNMODERATOR(CMSG_CHANNEL_UNMODERATOR::read_body(r, header_size)?)),
            0x00A1 => Ok(Self::CMSG_CHANNEL_MUTE(CMSG_CHANNEL_MUTE::read_body(r, header_size)?)),
            0x00A2 => Ok(Self::CMSG_CHANNEL_UNMUTE(CMSG_CHANNEL_UNMUTE::read_body(r, header_size)?)),
            0x00A3 => Ok(Self::CMSG_CHANNEL_INVITE(CMSG_CHANNEL_INVITE::read_body(r, header_size)?)),
            0x00A4 => Ok(Self::CMSG_CHANNEL_KICK(CMSG_CHANNEL_KICK::read_body(r, header_size)?)),
            0x00A5 => Ok(Self::CMSG_CHANNEL_BAN(CMSG_CHANNEL_BAN::read_body(r, header_size)?)),
            0x00A6 => Ok(Self::CMSG_CHANNEL_UNBAN(CMSG_CHANNEL_UNBAN::read_body(r, header_size)?)),
            0x00A7 => Ok(Self::CMSG_CHANNEL_ANNOUNCEMENTS(CMSG_CHANNEL_ANNOUNCEMENTS::read_body(r, header_size)?)),
            0x00A8 => Ok(Self::CMSG_CHANNEL_MODERATE(CMSG_CHANNEL_MODERATE::read_body(r, header_size)?)),
            0x00A9 => Ok(Self::SMSG_UPDATE_OBJECT(SMSG_UPDATE_OBJECT::read_body(r, header_size)?)),
            0x00AB => Ok(Self::CMSG_USE_ITEM(CMSG_USE_ITEM::read_body(r, header_size)?)),
            0x00AC => Ok(Self::CMSG_OPEN_ITEM(CMSG_OPEN_ITEM::read_body(r, header_size)?)),
            0x00AD => Ok(Self::CMSG_READ_ITEM(CMSG_READ_ITEM::read_body(r, header_size)?)),
            0x00B1 => Ok(Self::CMSG_GAMEOBJ_USE(CMSG_GAMEOBJ_USE::read_body(r, header_size)?)),
            0x00B4 => Ok(Self::CMSG_AREATRIGGER(CMSG_AREATRIGGER::read_body(r, header_size)?)),
            0x00E1 => Ok(Self::CMSG_MOVE_SET_RAW_POSITION(CMSG_MOVE_SET_RAW_POSITION::read_body(r, header_size)?)),
            0x00E3 => Ok(Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_SPEED_CHANGE_ACK::read_body(r, header_size)?)),
            0x00E5 => Ok(Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK::read_body(r, header_size)?)),
            0x00E7 => Ok(Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_SPEED_CHANGE_ACK::read_body(r, header_size)?)),
            0x00E9 => Ok(Self::CMSG_FORCE_MOVE_ROOT_ACK(CMSG_FORCE_MOVE_ROOT_ACK::read_body(r, header_size)?)),
            0x00EB => Ok(Self::CMSG_FORCE_MOVE_UNROOT_ACK(CMSG_FORCE_MOVE_UNROOT_ACK::read_body(r, header_size)?)),
            0x00F0 => Ok(Self::CMSG_MOVE_KNOCK_BACK_ACK(CMSG_MOVE_KNOCK_BACK_ACK::read_body(r, header_size)?)),
            0x00F6 => Ok(Self::CMSG_MOVE_HOVER_ACK(CMSG_MOVE_HOVER_ACK::read_body(r, header_size)?)),
            0x00FB => Ok(Self::CMSG_NEXT_CINEMATIC_CAMERA(CMSG_NEXT_CINEMATIC_CAMERA::read_body(r, header_size)?)),
            0x00FC => Ok(Self::CMSG_COMPLETE_CINEMATIC(CMSG_COMPLETE_CINEMATIC::read_body(r, header_size)?)),
            0x00FE => Ok(Self::CMSG_TUTORIAL_FLAG(CMSG_TUTORIAL_FLAG::read_body(r, header_size)?)),
            0x00FF => Ok(Self::CMSG_TUTORIAL_CLEAR(CMSG_TUTORIAL_CLEAR::read_body(r, header_size)?)),
            0x0100 => Ok(Self::CMSG_TUTORIAL_RESET(CMSG_TUTORIAL_RESET::read_body(r, header_size)?)),
            0x0101 => Ok(Self::CMSG_STANDSTATECHANGE(CMSG_STANDSTATECHANGE::read_body(r, header_size)?)),
            0x0102 => Ok(Self::CMSG_EMOTE(CMSG_EMOTE::read_body(r, header_size)?)),
            0x0104 => Ok(Self::CMSG_TEXT_EMOTE(CMSG_TEXT_EMOTE::read_body(r, header_size)?)),
            0x0108 => Ok(Self::CMSG_AUTOSTORE_LOOT_ITEM(CMSG_AUTOSTORE_LOOT_ITEM::read_body(r, header_size)?)),
            0x010A => Ok(Self::CMSG_AUTOEQUIP_ITEM(CMSG_AUTOEQUIP_ITEM::read_body(r, header_size)?)),
            0x010B => Ok(Self::CMSG_AUTOSTORE_BAG_ITEM(CMSG_AUTOSTORE_BAG_ITEM::read_body(r, header_size)?)),
            0x010C => Ok(Self::CMSG_SWAP_ITEM(CMSG_SWAP_ITEM::read_body(r, header_size)?)),
            0x010D => Ok(Self::CMSG_SWAP_INV_ITEM(CMSG_SWAP_INV_ITEM::read_body(r, header_size)?)),
            0x010E => Ok(Self::CMSG_SPLIT_ITEM(CMSG_SPLIT_ITEM::read_body(r, header_size)?)),
            0x010F => Ok(Self::CMSG_AUTOEQUIP_ITEM_SLOT(CMSG_AUTOEQUIP_ITEM_SLOT::read_body(r, header_size)?)),
            0x0111 => Ok(Self::CMSG_DESTROYITEM(CMSG_DESTROYITEM::read_body(r, header_size)?)),
            0x0114 => Ok(Self::CMSG_INSPECT(CMSG_INSPECT::read_body(r, header_size)?)),
            0x0116 => Ok(Self::CMSG_INITIATE_TRADE(CMSG_INITIATE_TRADE::read_body(r, header_size)?)),
            0x0117 => Ok(Self::CMSG_BEGIN_TRADE(CMSG_BEGIN_TRADE::read_body(r, header_size)?)),
            0x0118 => Ok(Self::CMSG_BUSY_TRADE(CMSG_BUSY_TRADE::read_body(r, header_size)?)),
            0x0119 => Ok(Self::CMSG_IGNORE_TRADE(CMSG_IGNORE_TRADE::read_body(r, header_size)?)),
            0x011A => Ok(Self::CMSG_ACCEPT_TRADE(CMSG_ACCEPT_TRADE::read_body(r, header_size)?)),
            0x011B => Ok(Self::CMSG_UNACCEPT_TRADE(CMSG_UNACCEPT_TRADE::read_body(r, header_size)?)),
            0x011C => Ok(Self::CMSG_CANCEL_TRADE(CMSG_CANCEL_TRADE::read_body(r, header_size)?)),
            0x011D => Ok(Self::CMSG_SET_TRADE_ITEM(CMSG_SET_TRADE_ITEM::read_body(r, header_size)?)),
            0x011E => Ok(Self::CMSG_CLEAR_TRADE_ITEM(CMSG_CLEAR_TRADE_ITEM::read_body(r, header_size)?)),
            0x011F => Ok(Self::CMSG_SET_TRADE_GOLD(CMSG_SET_TRADE_GOLD::read_body(r, header_size)?)),
            0x0125 => Ok(Self::CMSG_SET_FACTION_ATWAR(CMSG_SET_FACTION_ATWAR::read_body(r, header_size)?)),
            0x0128 => Ok(Self::CMSG_SET_ACTION_BUTTON(CMSG_SET_ACTION_BUTTON::read_body(r, header_size)?)),
            0x012E => Ok(Self::CMSG_CAST_SPELL(CMSG_CAST_SPELL::read_body(r, header_size)?)),
            0x012F => Ok(Self::CMSG_CANCEL_CAST(CMSG_CANCEL_CAST::read_body(r, header_size)?)),
            0x0136 => Ok(Self::CMSG_CANCEL_AURA(CMSG_CANCEL_AURA::read_body(r, header_size)?)),
            0x013B => Ok(Self::CMSG_CANCEL_CHANNELLING(CMSG_CANCEL_CHANNELLING::read_body(r, header_size)?)),
            0x013D => Ok(Self::CMSG_SET_SELECTION(CMSG_SET_SELECTION::read_body(r, header_size)?)),
            0x013E => Ok(Self::CMSG_SET_TARGET_OBSOLETE(CMSG_SET_TARGET_OBSOLETE::read_body(r, header_size)?)),
            0x0141 => Ok(Self::CMSG_ATTACKSWING(CMSG_ATTACKSWING::read_body(r, header_size)?)),
            0x0142 => Ok(Self::CMSG_ATTACKSTOP(CMSG_ATTACKSTOP::read_body(r, header_size)?)),
            0x015A => Ok(Self::CMSG_REPOP_REQUEST(CMSG_REPOP_REQUEST::read_body(r, header_size)?)),
            0x015C => Ok(Self::CMSG_RESURRECT_RESPONSE(CMSG_RESURRECT_RESPONSE::read_body(r, header_size)?)),
            0x015D => Ok(Self::CMSG_LOOT(CMSG_LOOT::read_body(r, header_size)?)),
            0x015E => Ok(Self::CMSG_LOOT_MONEY(CMSG_LOOT_MONEY::read_body(r, header_size)?)),
            0x015F => Ok(Self::CMSG_LOOT_RELEASE(CMSG_LOOT_RELEASE::read_body(r, header_size)?)),
            0x016C => Ok(Self::CMSG_DUEL_ACCEPTED(CMSG_DUEL_ACCEPTED::read_body(r, header_size)?)),
            0x016D => Ok(Self::CMSG_DUEL_CANCELLED(CMSG_DUEL_CANCELLED::read_body(r, header_size)?)),
            0x0171 => Ok(Self::CMSG_MOUNTSPECIAL_ANIM(CMSG_MOUNTSPECIAL_ANIM::read_body(r, header_size)?)),
            0x0174 => Ok(Self::CMSG_PET_SET_ACTION(CMSG_PET_SET_ACTION::read_body(r, header_size)?)),
            0x0175 => Ok(Self::CMSG_PET_ACTION(CMSG_PET_ACTION::read_body(r, header_size)?)),
            0x0176 => Ok(Self::CMSG_PET_ABANDON(CMSG_PET_ABANDON::read_body(r, header_size)?)),
            0x0177 => Ok(Self::CMSG_PET_RENAME(CMSG_PET_RENAME::read_body(r, header_size)?)),
            0x017B => Ok(Self::CMSG_GOSSIP_HELLO(CMSG_GOSSIP_HELLO::read_body(r, header_size)?)),
            0x017C => Ok(Self::CMSG_GOSSIP_SELECT_OPTION(CMSG_GOSSIP_SELECT_OPTION::read_body(r, header_size)?)),
            0x017F => Ok(Self::CMSG_NPC_TEXT_QUERY(CMSG_NPC_TEXT_QUERY::read_body(r, header_size)?)),
            0x0182 => Ok(Self::CMSG_QUESTGIVER_STATUS_QUERY(CMSG_QUESTGIVER_STATUS_QUERY::read_body(r, header_size)?)),
            0x0184 => Ok(Self::CMSG_QUESTGIVER_HELLO(CMSG_QUESTGIVER_HELLO::read_body(r, header_size)?)),
            0x0186 => Ok(Self::CMSG_QUESTGIVER_QUERY_QUEST(CMSG_QUESTGIVER_QUERY_QUEST::read_body(r, header_size)?)),
            0x0187 => Ok(Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(CMSG_QUESTGIVER_QUEST_AUTOLAUNCH::read_body(r, header_size)?)),
            0x0189 => Ok(Self::CMSG_QUESTGIVER_ACCEPT_QUEST(CMSG_QUESTGIVER_ACCEPT_QUEST::read_body(r, header_size)?)),
            0x018A => Ok(Self::CMSG_QUESTGIVER_COMPLETE_QUEST(CMSG_QUESTGIVER_COMPLETE_QUEST::read_body(r, header_size)?)),
            0x018C => Ok(Self::CMSG_QUESTGIVER_REQUEST_REWARD(CMSG_QUESTGIVER_REQUEST_REWARD::read_body(r, header_size)?)),
            0x018E => Ok(Self::CMSG_QUESTGIVER_CHOOSE_REWARD(CMSG_QUESTGIVER_CHOOSE_REWARD::read_body(r, header_size)?)),
            0x0190 => Ok(Self::CMSG_QUESTGIVER_CANCEL(CMSG_QUESTGIVER_CANCEL::read_body(r, header_size)?)),
            0x0193 => Ok(Self::CMSG_QUESTLOG_SWAP_QUEST(CMSG_QUESTLOG_SWAP_QUEST::read_body(r, header_size)?)),
            0x0194 => Ok(Self::CMSG_QUESTLOG_REMOVE_QUEST(CMSG_QUESTLOG_REMOVE_QUEST::read_body(r, header_size)?)),
            0x019B => Ok(Self::CMSG_QUEST_CONFIRM_ACCEPT(CMSG_QUEST_CONFIRM_ACCEPT::read_body(r, header_size)?)),
            0x019D => Ok(Self::CMSG_PUSHQUESTTOPARTY(CMSG_PUSHQUESTTOPARTY::read_body(r, header_size)?)),
            0x019E => Ok(Self::CMSG_LIST_INVENTORY(CMSG_LIST_INVENTORY::read_body(r, header_size)?)),
            0x01A0 => Ok(Self::CMSG_SELL_ITEM(CMSG_SELL_ITEM::read_body(r, header_size)?)),
            0x01A2 => Ok(Self::CMSG_BUY_ITEM(CMSG_BUY_ITEM::read_body(r, header_size)?)),
            0x01A3 => Ok(Self::CMSG_BUY_ITEM_IN_SLOT(CMSG_BUY_ITEM_IN_SLOT::read_body(r, header_size)?)),
            0x01AA => Ok(Self::CMSG_TAXINODE_STATUS_QUERY(CMSG_TAXINODE_STATUS_QUERY::read_body(r, header_size)?)),
            0x01AC => Ok(Self::CMSG_TAXIQUERYAVAILABLENODES(CMSG_TAXIQUERYAVAILABLENODES::read_body(r, header_size)?)),
            0x01AD => Ok(Self::CMSG_ACTIVATETAXI(CMSG_ACTIVATETAXI::read_body(r, header_size)?)),
            0x01B0 => Ok(Self::CMSG_TRAINER_LIST(CMSG_TRAINER_LIST::read_body(r, header_size)?)),
            0x01B2 => Ok(Self::CMSG_TRAINER_BUY_SPELL(CMSG_TRAINER_BUY_SPELL::read_body(r, header_size)?)),
            0x01B5 => Ok(Self::CMSG_BINDER_ACTIVATE(CMSG_BINDER_ACTIVATE::read_body(r, header_size)?)),
            0x01B7 => Ok(Self::CMSG_BANKER_ACTIVATE(CMSG_BANKER_ACTIVATE::read_body(r, header_size)?)),
            0x01B9 => Ok(Self::CMSG_BUY_BANK_SLOT(CMSG_BUY_BANK_SLOT::read_body(r, header_size)?)),
            0x01BB => Ok(Self::CMSG_PETITION_SHOWLIST(CMSG_PETITION_SHOWLIST::read_body(r, header_size)?)),
            0x01BD => Ok(Self::CMSG_PETITION_BUY(CMSG_PETITION_BUY::read_body(r, header_size)?)),
            0x01BE => Ok(Self::CMSG_PETITION_SHOW_SIGNATURES(CMSG_PETITION_SHOW_SIGNATURES::read_body(r, header_size)?)),
            0x01C0 => Ok(Self::CMSG_PETITION_SIGN(CMSG_PETITION_SIGN::read_body(r, header_size)?)),
            0x01C3 => Ok(Self::CMSG_OFFER_PETITION(CMSG_OFFER_PETITION::read_body(r, header_size)?)),
            0x01C4 => Ok(Self::CMSG_TURN_IN_PETITION(CMSG_TURN_IN_PETITION::read_body(r, header_size)?)),
            0x01C6 => Ok(Self::CMSG_PETITION_QUERY(CMSG_PETITION_QUERY::read_body(r, header_size)?)),
            0x01CA => Ok(Self::CMSG_BUG(CMSG_BUG::read_body(r, header_size)?)),
            0x01CC => Ok(Self::CMSG_PLAYED_TIME(CMSG_PLAYED_TIME::read_body(r, header_size)?)),
            0x01CE => Ok(Self::CMSG_QUERY_TIME(CMSG_QUERY_TIME::read_body(r, header_size)?)),
            0x01D2 => Ok(Self::CMSG_RECLAIM_CORPSE(CMSG_RECLAIM_CORPSE::read_body(r, header_size)?)),
            0x01D3 => Ok(Self::CMSG_WRAP_ITEM(CMSG_WRAP_ITEM::read_body(r, header_size)?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(MSG_MINIMAP_PING_Client::read_body(r, header_size)?)),
            0x01DC => Ok(Self::CMSG_PING(CMSG_PING::read_body(r, header_size)?)),
            0x01E0 => Ok(Self::CMSG_SETSHEATHED(CMSG_SETSHEATHED::read_body(r, header_size)?)),
            0x01ED => Ok(Self::CMSG_AUTH_SESSION(CMSG_AUTH_SESSION::read_body(r, header_size)?)),
            0x01F0 => Ok(Self::CMSG_PET_CAST_SPELL(CMSG_PET_CAST_SPELL::read_body(r, header_size)?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Client::read_body(r, header_size)?)),
            0x01F4 => Ok(Self::CMSG_ZONEUPDATE(CMSG_ZONEUPDATE::read_body(r, header_size)?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Client::read_body(r, header_size)?)),
            0x01FF => Ok(Self::MSG_LOOKING_FOR_GROUP(MSG_LOOKING_FOR_GROUP_Client::read_body(r, header_size)?)),
            0x0202 => Ok(Self::CMSG_UNLEARN_SKILL(CMSG_UNLEARN_SKILL::read_body(r, header_size)?)),
            0x0205 => Ok(Self::CMSG_GMTICKET_CREATE(CMSG_GMTICKET_CREATE::read_body(r, header_size)?)),
            0x0207 => Ok(Self::CMSG_GMTICKET_UPDATETEXT(CMSG_GMTICKET_UPDATETEXT::read_body(r, header_size)?)),
            0x020A => Ok(Self::CMSG_REQUEST_ACCOUNT_DATA(CMSG_REQUEST_ACCOUNT_DATA::read_body(r, header_size)?)),
            0x0211 => Ok(Self::CMSG_GMTICKET_GETTICKET(CMSG_GMTICKET_GETTICKET::read_body(r, header_size)?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Client::read_body(r, header_size)?)),
            0x0217 => Ok(Self::CMSG_GMTICKET_DELETETICKET(CMSG_GMTICKET_DELETETICKET::read_body(r, header_size)?)),
            0x021A => Ok(Self::CMSG_GMTICKET_SYSTEMSTATUS(CMSG_GMTICKET_SYSTEMSTATUS::read_body(r, header_size)?)),
            0x021C => Ok(Self::CMSG_SPIRIT_HEALER_ACTIVATE(CMSG_SPIRIT_HEALER_ACTIVATE::read_body(r, header_size)?)),
            0x0225 => Ok(Self::CMSG_CHAT_IGNORED(CMSG_CHAT_IGNORED::read_body(r, header_size)?)),
            0x0231 => Ok(Self::CMSG_GUILD_RANK(CMSG_GUILD_RANK::read_body(r, header_size)?)),
            0x0232 => Ok(Self::CMSG_GUILD_ADD_RANK(CMSG_GUILD_ADD_RANK::read_body(r, header_size)?)),
            0x0233 => Ok(Self::CMSG_GUILD_DEL_RANK(CMSG_GUILD_DEL_RANK::read_body(r, header_size)?)),
            0x0234 => Ok(Self::CMSG_GUILD_SET_PUBLIC_NOTE(CMSG_GUILD_SET_PUBLIC_NOTE::read_body(r, header_size)?)),
            0x0235 => Ok(Self::CMSG_GUILD_SET_OFFICER_NOTE(CMSG_GUILD_SET_OFFICER_NOTE::read_body(r, header_size)?)),
            0x0238 => Ok(Self::CMSG_SEND_MAIL(CMSG_SEND_MAIL::read_body(r, header_size)?)),
            0x023A => Ok(Self::CMSG_GET_MAIL_LIST(CMSG_GET_MAIL_LIST::read_body(r, header_size)?)),
            0x023C => Ok(Self::CMSG_BATTLEFIELD_LIST(CMSG_BATTLEFIELD_LIST::read_body(r, header_size)?)),
            0x023E => Ok(Self::CMSG_BATTLEFIELD_JOIN(CMSG_BATTLEFIELD_JOIN::read_body(r, header_size)?)),
            0x0243 => Ok(Self::CMSG_ITEM_TEXT_QUERY(CMSG_ITEM_TEXT_QUERY::read_body(r, header_size)?)),
            0x0245 => Ok(Self::CMSG_MAIL_TAKE_MONEY(CMSG_MAIL_TAKE_MONEY::read_body(r, header_size)?)),
            0x0246 => Ok(Self::CMSG_MAIL_TAKE_ITEM(CMSG_MAIL_TAKE_ITEM::read_body(r, header_size)?)),
            0x0247 => Ok(Self::CMSG_MAIL_MARK_AS_READ(CMSG_MAIL_MARK_AS_READ::read_body(r, header_size)?)),
            0x0248 => Ok(Self::CMSG_MAIL_RETURN_TO_SENDER(CMSG_MAIL_RETURN_TO_SENDER::read_body(r, header_size)?)),
            0x0249 => Ok(Self::CMSG_MAIL_DELETE(CMSG_MAIL_DELETE::read_body(r, header_size)?)),
            0x024A => Ok(Self::CMSG_MAIL_CREATE_TEXT_ITEM(CMSG_MAIL_CREATE_TEXT_ITEM::read_body(r, header_size)?)),
            0x0251 => Ok(Self::CMSG_LEARN_TALENT(CMSG_LEARN_TALENT::read_body(r, header_size)?)),
            0x0253 => Ok(Self::CMSG_TOGGLE_PVP(CMSG_TOGGLE_PVP::read_body(r, header_size)?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Client::read_body(r, header_size)?)),
            0x0256 => Ok(Self::CMSG_AUCTION_SELL_ITEM(CMSG_AUCTION_SELL_ITEM::read_body(r, header_size)?)),
            0x0257 => Ok(Self::CMSG_AUCTION_REMOVE_ITEM(CMSG_AUCTION_REMOVE_ITEM::read_body(r, header_size)?)),
            0x0258 => Ok(Self::CMSG_AUCTION_LIST_ITEMS(CMSG_AUCTION_LIST_ITEMS::read_body(r, header_size)?)),
            0x0259 => Ok(Self::CMSG_AUCTION_LIST_OWNER_ITEMS(CMSG_AUCTION_LIST_OWNER_ITEMS::read_body(r, header_size)?)),
            0x025A => Ok(Self::CMSG_AUCTION_PLACE_BID(CMSG_AUCTION_PLACE_BID::read_body(r, header_size)?)),
            0x0264 => Ok(Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(CMSG_AUCTION_LIST_BIDDER_ITEMS::read_body(r, header_size)?)),
            0x0268 => Ok(Self::CMSG_SET_AMMO(CMSG_SET_AMMO::read_body(r, header_size)?)),
            0x026A => Ok(Self::CMSG_SET_ACTIVE_MOVER(CMSG_SET_ACTIVE_MOVER::read_body(r, header_size)?)),
            0x026B => Ok(Self::CMSG_PET_CANCEL_AURA(CMSG_PET_CANCEL_AURA::read_body(r, header_size)?)),
            0x026D => Ok(Self::CMSG_CANCEL_AUTO_REPEAT_SPELL(CMSG_CANCEL_AUTO_REPEAT_SPELL::read_body(r, header_size)?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Client::read_body(r, header_size)?)),
            0x0270 => Ok(Self::CMSG_STABLE_PET(CMSG_STABLE_PET::read_body(r, header_size)?)),
            0x0271 => Ok(Self::CMSG_UNSTABLE_PET(CMSG_UNSTABLE_PET::read_body(r, header_size)?)),
            0x0272 => Ok(Self::CMSG_BUY_STABLE_SLOT(CMSG_BUY_STABLE_SLOT::read_body(r, header_size)?)),
            0x0275 => Ok(Self::CMSG_STABLE_SWAP_PET(CMSG_STABLE_SWAP_PET::read_body(r, header_size)?)),
            0x0279 => Ok(Self::CMSG_REQUEST_PET_INFO(CMSG_REQUEST_PET_INFO::read_body(r, header_size)?)),
            0x027A => Ok(Self::CMSG_FAR_SIGHT(CMSG_FAR_SIGHT::read_body(r, header_size)?)),
            0x027E => Ok(Self::CMSG_GROUP_CHANGE_SUB_GROUP(CMSG_GROUP_CHANGE_SUB_GROUP::read_body(r, header_size)?)),
            0x027F => Ok(Self::CMSG_REQUEST_PARTY_MEMBER_STATS(CMSG_REQUEST_PARTY_MEMBER_STATS::read_body(r, header_size)?)),
            0x0280 => Ok(Self::CMSG_GROUP_SWAP_SUB_GROUP(CMSG_GROUP_SWAP_SUB_GROUP::read_body(r, header_size)?)),
            0x0282 => Ok(Self::CMSG_AUTOSTORE_BANK_ITEM(CMSG_AUTOSTORE_BANK_ITEM::read_body(r, header_size)?)),
            0x0283 => Ok(Self::CMSG_AUTOBANK_ITEM(CMSG_AUTOBANK_ITEM::read_body(r, header_size)?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Client::read_body(r, header_size)?)),
            0x028E => Ok(Self::CMSG_GROUP_RAID_CONVERT(CMSG_GROUP_RAID_CONVERT::read_body(r, header_size)?)),
            0x028F => Ok(Self::CMSG_GROUP_ASSISTANT_LEADER(CMSG_GROUP_ASSISTANT_LEADER::read_body(r, header_size)?)),
            0x0290 => Ok(Self::CMSG_BUYBACK_ITEM(CMSG_BUYBACK_ITEM::read_body(r, header_size)?)),
            0x0292 => Ok(Self::CMSG_MEETINGSTONE_JOIN(CMSG_MEETINGSTONE_JOIN::read_body(r, header_size)?)),
            0x0293 => Ok(Self::CMSG_MEETINGSTONE_LEAVE(CMSG_MEETINGSTONE_LEAVE::read_body(r, header_size)?)),
            0x0296 => Ok(Self::CMSG_MEETINGSTONE_INFO(CMSG_MEETINGSTONE_INFO::read_body(r, header_size)?)),
            0x029B => Ok(Self::CMSG_CANCEL_GROWTH_AURA(CMSG_CANCEL_GROWTH_AURA::read_body(r, header_size)?)),
            0x02A0 => Ok(Self::CMSG_LOOT_ROLL(CMSG_LOOT_ROLL::read_body(r, header_size)?)),
            0x02A3 => Ok(Self::CMSG_LOOT_MASTER_GIVE(CMSG_LOOT_MASTER_GIVE::read_body(r, header_size)?)),
            0x02A8 => Ok(Self::CMSG_REPAIR_ITEM(CMSG_REPAIR_ITEM::read_body(r, header_size)?)),
            0x02AA => Ok(Self::MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Client::read_body(r, header_size)?)),
            0x02AC => Ok(Self::CMSG_SUMMON_RESPONSE(CMSG_SUMMON_RESPONSE::read_body(r, header_size)?)),
            0x02B3 => Ok(Self::CMSG_SELF_RES(CMSG_SELF_RES::read_body(r, header_size)?)),
            0x02B9 => Ok(Self::CMSG_TOGGLE_HELM(CMSG_TOGGLE_HELM::read_body(r, header_size)?)),
            0x02BA => Ok(Self::CMSG_TOGGLE_CLOAK(CMSG_TOGGLE_CLOAK::read_body(r, header_size)?)),
            0x02BF => Ok(Self::CMSG_SET_ACTIONBAR_TOGGLES(CMSG_SET_ACTIONBAR_TOGGLES::read_body(r, header_size)?)),
            0x02C4 => Ok(Self::CMSG_ITEM_NAME_QUERY(CMSG_ITEM_NAME_QUERY::read_body(r, header_size)?)),
            0x02C7 => Ok(Self::CMSG_CHAR_RENAME(CMSG_CHAR_RENAME::read_body(r, header_size)?)),
            0x02C9 => Ok(Self::CMSG_MOVE_SPLINE_DONE(CMSG_MOVE_SPLINE_DONE::read_body(r, header_size)?)),
            0x02CA => Ok(Self::CMSG_MOVE_FALL_RESET(CMSG_MOVE_FALL_RESET::read_body(r, header_size)?)),
            0x02CD => Ok(Self::CMSG_REQUEST_RAID_INFO(CMSG_REQUEST_RAID_INFO::read_body(r, header_size)?)),
            0x02CE => Ok(Self::CMSG_MOVE_TIME_SKIPPED(CMSG_MOVE_TIME_SKIPPED::read_body(r, header_size)?)),
            0x02CF => Ok(Self::CMSG_MOVE_FEATHER_FALL_ACK(CMSG_MOVE_FEATHER_FALL_ACK::read_body(r, header_size)?)),
            0x02D0 => Ok(Self::CMSG_MOVE_WATER_WALK_ACK(CMSG_MOVE_WATER_WALK_ACK::read_body(r, header_size)?)),
            0x02D1 => Ok(Self::CMSG_MOVE_NOT_ACTIVE_MOVER(CMSG_MOVE_NOT_ACTIVE_MOVER::read_body(r, header_size)?)),
            0x02D3 => Ok(Self::CMSG_BATTLEFIELD_STATUS(CMSG_BATTLEFIELD_STATUS::read_body(r, header_size)?)),
            0x02D5 => Ok(Self::CMSG_BATTLEFIELD_PORT(CMSG_BATTLEFIELD_PORT::read_body(r, header_size)?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Client::read_body(r, header_size)?)),
            0x02D7 => Ok(Self::CMSG_BATTLEMASTER_HELLO(CMSG_BATTLEMASTER_HELLO::read_body(r, header_size)?)),
            0x02DB => Ok(Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(CMSG_FORCE_WALK_SPEED_CHANGE_ACK::read_body(r, header_size)?)),
            0x02DD => Ok(Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK::read_body(r, header_size)?)),
            0x02DF => Ok(Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(CMSG_FORCE_TURN_RATE_CHANGE_ACK::read_body(r, header_size)?)),
            0x02E0 => Ok(Self::MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_Client::read_body(r, header_size)?)),
            0x02E1 => Ok(Self::CMSG_LEAVE_BATTLEFIELD(CMSG_LEAVE_BATTLEFIELD::read_body(r, header_size)?)),
            0x02E2 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUERY(CMSG_AREA_SPIRIT_HEALER_QUERY::read_body(r, header_size)?)),
            0x02E3 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(CMSG_AREA_SPIRIT_HEALER_QUEUE::read_body(r, header_size)?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Client::read_body(r, header_size)?)),
            0x02EA => Ok(Self::CMSG_PET_STOP_ATTACK(CMSG_PET_STOP_ATTACK::read_body(r, header_size)?)),
            0x02EE => Ok(Self::CMSG_BATTLEMASTER_JOIN(CMSG_BATTLEMASTER_JOIN::read_body(r, header_size)?)),
            0x02F0 => Ok(Self::CMSG_PET_UNLEARN(CMSG_PET_UNLEARN::read_body(r, header_size)?)),
            0x02F3 => Ok(Self::CMSG_PET_SPELL_AUTOCAST(CMSG_PET_SPELL_AUTOCAST::read_body(r, header_size)?)),
            0x02FC => Ok(Self::CMSG_GUILD_INFO_TEXT(CMSG_GUILD_INFO_TEXT::read_body(r, header_size)?)),
            0x0312 => Ok(Self::CMSG_ACTIVATETAXIEXPRESS(CMSG_ACTIVATETAXIEXPRESS::read_body(r, header_size)?)),
            0x0317 => Ok(Self::CMSG_SET_FACTION_INACTIVE(CMSG_SET_FACTION_INACTIVE::read_body(r, header_size)?)),
            0x0318 => Ok(Self::CMSG_SET_WATCHED_FACTION(CMSG_SET_WATCHED_FACTION::read_body(r, header_size)?)),
            0x031D => Ok(Self::CMSG_RESET_INSTANCES(CMSG_RESET_INSTANCES::read_body(r, header_size)?)),
            0x0321 => Ok(Self::MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Client::read_body(r, header_size)?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Client::read_body(r, header_size)?)),
            0x032A => Ok(Self::CMSG_GMSURVEY_SUBMIT(CMSG_GMSURVEY_SUBMIT::read_body(r, header_size)?)),
            _ => Err(ClientOpcodeMessageError::InvalidOpcode(header.opcode)),
        }
    }


    #[cfg(feature = "tokio")]
    pub async fn tokio_read_unencrypted<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let size = (crate::util::tokio_read_u16_be(r).await? - 4) as u32;
        let opcode = crate::util::tokio_read_u32_le(r).await?;
        match opcode {
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD::tokio_read_body(r, size).await?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD::tokio_read_body(r, size).await?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(MSG_MOVE_STOP::tokio_read_body(r, size).await?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT::tokio_read_body(r, size).await?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT::tokio_read_body(r, size).await?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE::tokio_read_body(r, size).await?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(MSG_MOVE_JUMP::tokio_read_body(r, size).await?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT::tokio_read_body(r, size).await?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT::tokio_read_body(r, size).await?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN::tokio_read_body(r, size).await?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP::tokio_read_body(r, size).await?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN::tokio_read_body(r, size).await?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH::tokio_read_body(r, size).await?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE::tokio_read_body(r, size).await?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE::tokio_read_body(r, size).await?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK::tokio_read_body(r, size).await?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND::tokio_read_body(r, size).await?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM::tokio_read_body(r, size).await?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM::tokio_read_body(r, size).await?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING::tokio_read_body(r, size).await?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH::tokio_read_body(r, size).await?)),
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK::tokio_read_body(r, size).await?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT::tokio_read_body(r, size).await?)),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(MSG_PETITION_DECLINE::tokio_read_body(r, size).await?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE::tokio_read_body(r, size).await?)),
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT::tokio_read_body(r, size).await?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(MSG_PETITION_RENAME::tokio_read_body(r, size).await?)),
            0x0008 => Ok(Self::CMSG_WORLD_TELEPORT(CMSG_WORLD_TELEPORT::tokio_read_body(r, size).await?)),
            0x0036 => Ok(Self::CMSG_CHAR_CREATE(CMSG_CHAR_CREATE::tokio_read_body(r, size).await?)),
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(CMSG_CHAR_ENUM::tokio_read_body(r, size).await?)),
            0x0038 => Ok(Self::CMSG_CHAR_DELETE(CMSG_CHAR_DELETE::tokio_read_body(r, size).await?)),
            0x003D => Ok(Self::CMSG_PLAYER_LOGIN(CMSG_PLAYER_LOGIN::tokio_read_body(r, size).await?)),
            0x004A => Ok(Self::CMSG_PLAYER_LOGOUT(CMSG_PLAYER_LOGOUT::tokio_read_body(r, size).await?)),
            0x004B => Ok(Self::CMSG_LOGOUT_REQUEST(CMSG_LOGOUT_REQUEST::tokio_read_body(r, size).await?)),
            0x004E => Ok(Self::CMSG_LOGOUT_CANCEL(CMSG_LOGOUT_CANCEL::tokio_read_body(r, size).await?)),
            0x0050 => Ok(Self::CMSG_NAME_QUERY(CMSG_NAME_QUERY::tokio_read_body(r, size).await?)),
            0x0052 => Ok(Self::CMSG_PET_NAME_QUERY(CMSG_PET_NAME_QUERY::tokio_read_body(r, size).await?)),
            0x0054 => Ok(Self::CMSG_GUILD_QUERY(CMSG_GUILD_QUERY::tokio_read_body(r, size).await?)),
            0x0056 => Ok(Self::CMSG_ITEM_QUERY_SINGLE(CMSG_ITEM_QUERY_SINGLE::tokio_read_body(r, size).await?)),
            0x005A => Ok(Self::CMSG_PAGE_TEXT_QUERY(CMSG_PAGE_TEXT_QUERY::tokio_read_body(r, size).await?)),
            0x005C => Ok(Self::CMSG_QUEST_QUERY(CMSG_QUEST_QUERY::tokio_read_body(r, size).await?)),
            0x005E => Ok(Self::CMSG_GAMEOBJECT_QUERY(CMSG_GAMEOBJECT_QUERY::tokio_read_body(r, size).await?)),
            0x0060 => Ok(Self::CMSG_CREATURE_QUERY(CMSG_CREATURE_QUERY::tokio_read_body(r, size).await?)),
            0x0062 => Ok(Self::CMSG_WHO(CMSG_WHO::tokio_read_body(r, size).await?)),
            0x0064 => Ok(Self::CMSG_WHOIS(CMSG_WHOIS::tokio_read_body(r, size).await?)),
            0x0066 => Ok(Self::CMSG_FRIEND_LIST(CMSG_FRIEND_LIST::tokio_read_body(r, size).await?)),
            0x0069 => Ok(Self::CMSG_ADD_FRIEND(CMSG_ADD_FRIEND::tokio_read_body(r, size).await?)),
            0x006A => Ok(Self::CMSG_DEL_FRIEND(CMSG_DEL_FRIEND::tokio_read_body(r, size).await?)),
            0x006C => Ok(Self::CMSG_ADD_IGNORE(CMSG_ADD_IGNORE::tokio_read_body(r, size).await?)),
            0x006D => Ok(Self::CMSG_DEL_IGNORE(CMSG_DEL_IGNORE::tokio_read_body(r, size).await?)),
            0x006E => Ok(Self::CMSG_GROUP_INVITE(CMSG_GROUP_INVITE::tokio_read_body(r, size).await?)),
            0x0072 => Ok(Self::CMSG_GROUP_ACCEPT(CMSG_GROUP_ACCEPT::tokio_read_body(r, size).await?)),
            0x0073 => Ok(Self::CMSG_GROUP_DECLINE(CMSG_GROUP_DECLINE::tokio_read_body(r, size).await?)),
            0x0075 => Ok(Self::CMSG_GROUP_UNINVITE(CMSG_GROUP_UNINVITE::tokio_read_body(r, size).await?)),
            0x0076 => Ok(Self::CMSG_GROUP_UNINVITE_GUID(CMSG_GROUP_UNINVITE_GUID::tokio_read_body(r, size).await?)),
            0x0078 => Ok(Self::CMSG_GROUP_SET_LEADER(CMSG_GROUP_SET_LEADER::tokio_read_body(r, size).await?)),
            0x007A => Ok(Self::CMSG_LOOT_METHOD(CMSG_LOOT_METHOD::tokio_read_body(r, size).await?)),
            0x007B => Ok(Self::CMSG_GROUP_DISBAND(CMSG_GROUP_DISBAND::tokio_read_body(r, size).await?)),
            0x0081 => Ok(Self::CMSG_GUILD_CREATE(CMSG_GUILD_CREATE::tokio_read_body(r, size).await?)),
            0x0082 => Ok(Self::CMSG_GUILD_INVITE(CMSG_GUILD_INVITE::tokio_read_body(r, size).await?)),
            0x0084 => Ok(Self::CMSG_GUILD_ACCEPT(CMSG_GUILD_ACCEPT::tokio_read_body(r, size).await?)),
            0x0085 => Ok(Self::CMSG_GUILD_DECLINE(CMSG_GUILD_DECLINE::tokio_read_body(r, size).await?)),
            0x0087 => Ok(Self::CMSG_GUILD_INFO(CMSG_GUILD_INFO::tokio_read_body(r, size).await?)),
            0x0089 => Ok(Self::CMSG_GUILD_ROSTER(CMSG_GUILD_ROSTER::tokio_read_body(r, size).await?)),
            0x008B => Ok(Self::CMSG_GUILD_PROMOTE(CMSG_GUILD_PROMOTE::tokio_read_body(r, size).await?)),
            0x008C => Ok(Self::CMSG_GUILD_DEMOTE(CMSG_GUILD_DEMOTE::tokio_read_body(r, size).await?)),
            0x008D => Ok(Self::CMSG_GUILD_LEAVE(CMSG_GUILD_LEAVE::tokio_read_body(r, size).await?)),
            0x008E => Ok(Self::CMSG_GUILD_REMOVE(CMSG_GUILD_REMOVE::tokio_read_body(r, size).await?)),
            0x008F => Ok(Self::CMSG_GUILD_DISBAND(CMSG_GUILD_DISBAND::tokio_read_body(r, size).await?)),
            0x0090 => Ok(Self::CMSG_GUILD_LEADER(CMSG_GUILD_LEADER::tokio_read_body(r, size).await?)),
            0x0091 => Ok(Self::CMSG_GUILD_MOTD(CMSG_GUILD_MOTD::tokio_read_body(r, size).await?)),
            0x0095 => Ok(Self::CMSG_MESSAGECHAT(CMSG_MESSAGECHAT::tokio_read_body(r, size).await?)),
            0x0097 => Ok(Self::CMSG_JOIN_CHANNEL(CMSG_JOIN_CHANNEL::tokio_read_body(r, size).await?)),
            0x0098 => Ok(Self::CMSG_LEAVE_CHANNEL(CMSG_LEAVE_CHANNEL::tokio_read_body(r, size).await?)),
            0x009A => Ok(Self::CMSG_CHANNEL_LIST(CMSG_CHANNEL_LIST::tokio_read_body(r, size).await?)),
            0x009C => Ok(Self::CMSG_CHANNEL_PASSWORD(CMSG_CHANNEL_PASSWORD::tokio_read_body(r, size).await?)),
            0x009D => Ok(Self::CMSG_CHANNEL_SET_OWNER(CMSG_CHANNEL_SET_OWNER::tokio_read_body(r, size).await?)),
            0x009E => Ok(Self::CMSG_CHANNEL_OWNER(CMSG_CHANNEL_OWNER::tokio_read_body(r, size).await?)),
            0x009F => Ok(Self::CMSG_CHANNEL_MODERATOR(CMSG_CHANNEL_MODERATOR::tokio_read_body(r, size).await?)),
            0x00A0 => Ok(Self::CMSG_CHANNEL_UNMODERATOR(CMSG_CHANNEL_UNMODERATOR::tokio_read_body(r, size).await?)),
            0x00A1 => Ok(Self::CMSG_CHANNEL_MUTE(CMSG_CHANNEL_MUTE::tokio_read_body(r, size).await?)),
            0x00A2 => Ok(Self::CMSG_CHANNEL_UNMUTE(CMSG_CHANNEL_UNMUTE::tokio_read_body(r, size).await?)),
            0x00A3 => Ok(Self::CMSG_CHANNEL_INVITE(CMSG_CHANNEL_INVITE::tokio_read_body(r, size).await?)),
            0x00A4 => Ok(Self::CMSG_CHANNEL_KICK(CMSG_CHANNEL_KICK::tokio_read_body(r, size).await?)),
            0x00A5 => Ok(Self::CMSG_CHANNEL_BAN(CMSG_CHANNEL_BAN::tokio_read_body(r, size).await?)),
            0x00A6 => Ok(Self::CMSG_CHANNEL_UNBAN(CMSG_CHANNEL_UNBAN::tokio_read_body(r, size).await?)),
            0x00A7 => Ok(Self::CMSG_CHANNEL_ANNOUNCEMENTS(CMSG_CHANNEL_ANNOUNCEMENTS::tokio_read_body(r, size).await?)),
            0x00A8 => Ok(Self::CMSG_CHANNEL_MODERATE(CMSG_CHANNEL_MODERATE::tokio_read_body(r, size).await?)),
            0x00A9 => Ok(Self::SMSG_UPDATE_OBJECT(SMSG_UPDATE_OBJECT::tokio_read_body(r, size).await?)),
            0x00AB => Ok(Self::CMSG_USE_ITEM(CMSG_USE_ITEM::tokio_read_body(r, size).await?)),
            0x00AC => Ok(Self::CMSG_OPEN_ITEM(CMSG_OPEN_ITEM::tokio_read_body(r, size).await?)),
            0x00AD => Ok(Self::CMSG_READ_ITEM(CMSG_READ_ITEM::tokio_read_body(r, size).await?)),
            0x00B1 => Ok(Self::CMSG_GAMEOBJ_USE(CMSG_GAMEOBJ_USE::tokio_read_body(r, size).await?)),
            0x00B4 => Ok(Self::CMSG_AREATRIGGER(CMSG_AREATRIGGER::tokio_read_body(r, size).await?)),
            0x00E1 => Ok(Self::CMSG_MOVE_SET_RAW_POSITION(CMSG_MOVE_SET_RAW_POSITION::tokio_read_body(r, size).await?)),
            0x00E3 => Ok(Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_SPEED_CHANGE_ACK::tokio_read_body(r, size).await?)),
            0x00E5 => Ok(Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK::tokio_read_body(r, size).await?)),
            0x00E7 => Ok(Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_SPEED_CHANGE_ACK::tokio_read_body(r, size).await?)),
            0x00E9 => Ok(Self::CMSG_FORCE_MOVE_ROOT_ACK(CMSG_FORCE_MOVE_ROOT_ACK::tokio_read_body(r, size).await?)),
            0x00EB => Ok(Self::CMSG_FORCE_MOVE_UNROOT_ACK(CMSG_FORCE_MOVE_UNROOT_ACK::tokio_read_body(r, size).await?)),
            0x00F0 => Ok(Self::CMSG_MOVE_KNOCK_BACK_ACK(CMSG_MOVE_KNOCK_BACK_ACK::tokio_read_body(r, size).await?)),
            0x00F6 => Ok(Self::CMSG_MOVE_HOVER_ACK(CMSG_MOVE_HOVER_ACK::tokio_read_body(r, size).await?)),
            0x00FB => Ok(Self::CMSG_NEXT_CINEMATIC_CAMERA(CMSG_NEXT_CINEMATIC_CAMERA::tokio_read_body(r, size).await?)),
            0x00FC => Ok(Self::CMSG_COMPLETE_CINEMATIC(CMSG_COMPLETE_CINEMATIC::tokio_read_body(r, size).await?)),
            0x00FE => Ok(Self::CMSG_TUTORIAL_FLAG(CMSG_TUTORIAL_FLAG::tokio_read_body(r, size).await?)),
            0x00FF => Ok(Self::CMSG_TUTORIAL_CLEAR(CMSG_TUTORIAL_CLEAR::tokio_read_body(r, size).await?)),
            0x0100 => Ok(Self::CMSG_TUTORIAL_RESET(CMSG_TUTORIAL_RESET::tokio_read_body(r, size).await?)),
            0x0101 => Ok(Self::CMSG_STANDSTATECHANGE(CMSG_STANDSTATECHANGE::tokio_read_body(r, size).await?)),
            0x0102 => Ok(Self::CMSG_EMOTE(CMSG_EMOTE::tokio_read_body(r, size).await?)),
            0x0104 => Ok(Self::CMSG_TEXT_EMOTE(CMSG_TEXT_EMOTE::tokio_read_body(r, size).await?)),
            0x0108 => Ok(Self::CMSG_AUTOSTORE_LOOT_ITEM(CMSG_AUTOSTORE_LOOT_ITEM::tokio_read_body(r, size).await?)),
            0x010A => Ok(Self::CMSG_AUTOEQUIP_ITEM(CMSG_AUTOEQUIP_ITEM::tokio_read_body(r, size).await?)),
            0x010B => Ok(Self::CMSG_AUTOSTORE_BAG_ITEM(CMSG_AUTOSTORE_BAG_ITEM::tokio_read_body(r, size).await?)),
            0x010C => Ok(Self::CMSG_SWAP_ITEM(CMSG_SWAP_ITEM::tokio_read_body(r, size).await?)),
            0x010D => Ok(Self::CMSG_SWAP_INV_ITEM(CMSG_SWAP_INV_ITEM::tokio_read_body(r, size).await?)),
            0x010E => Ok(Self::CMSG_SPLIT_ITEM(CMSG_SPLIT_ITEM::tokio_read_body(r, size).await?)),
            0x010F => Ok(Self::CMSG_AUTOEQUIP_ITEM_SLOT(CMSG_AUTOEQUIP_ITEM_SLOT::tokio_read_body(r, size).await?)),
            0x0111 => Ok(Self::CMSG_DESTROYITEM(CMSG_DESTROYITEM::tokio_read_body(r, size).await?)),
            0x0114 => Ok(Self::CMSG_INSPECT(CMSG_INSPECT::tokio_read_body(r, size).await?)),
            0x0116 => Ok(Self::CMSG_INITIATE_TRADE(CMSG_INITIATE_TRADE::tokio_read_body(r, size).await?)),
            0x0117 => Ok(Self::CMSG_BEGIN_TRADE(CMSG_BEGIN_TRADE::tokio_read_body(r, size).await?)),
            0x0118 => Ok(Self::CMSG_BUSY_TRADE(CMSG_BUSY_TRADE::tokio_read_body(r, size).await?)),
            0x0119 => Ok(Self::CMSG_IGNORE_TRADE(CMSG_IGNORE_TRADE::tokio_read_body(r, size).await?)),
            0x011A => Ok(Self::CMSG_ACCEPT_TRADE(CMSG_ACCEPT_TRADE::tokio_read_body(r, size).await?)),
            0x011B => Ok(Self::CMSG_UNACCEPT_TRADE(CMSG_UNACCEPT_TRADE::tokio_read_body(r, size).await?)),
            0x011C => Ok(Self::CMSG_CANCEL_TRADE(CMSG_CANCEL_TRADE::tokio_read_body(r, size).await?)),
            0x011D => Ok(Self::CMSG_SET_TRADE_ITEM(CMSG_SET_TRADE_ITEM::tokio_read_body(r, size).await?)),
            0x011E => Ok(Self::CMSG_CLEAR_TRADE_ITEM(CMSG_CLEAR_TRADE_ITEM::tokio_read_body(r, size).await?)),
            0x011F => Ok(Self::CMSG_SET_TRADE_GOLD(CMSG_SET_TRADE_GOLD::tokio_read_body(r, size).await?)),
            0x0125 => Ok(Self::CMSG_SET_FACTION_ATWAR(CMSG_SET_FACTION_ATWAR::tokio_read_body(r, size).await?)),
            0x0128 => Ok(Self::CMSG_SET_ACTION_BUTTON(CMSG_SET_ACTION_BUTTON::tokio_read_body(r, size).await?)),
            0x012E => Ok(Self::CMSG_CAST_SPELL(CMSG_CAST_SPELL::tokio_read_body(r, size).await?)),
            0x012F => Ok(Self::CMSG_CANCEL_CAST(CMSG_CANCEL_CAST::tokio_read_body(r, size).await?)),
            0x0136 => Ok(Self::CMSG_CANCEL_AURA(CMSG_CANCEL_AURA::tokio_read_body(r, size).await?)),
            0x013B => Ok(Self::CMSG_CANCEL_CHANNELLING(CMSG_CANCEL_CHANNELLING::tokio_read_body(r, size).await?)),
            0x013D => Ok(Self::CMSG_SET_SELECTION(CMSG_SET_SELECTION::tokio_read_body(r, size).await?)),
            0x013E => Ok(Self::CMSG_SET_TARGET_OBSOLETE(CMSG_SET_TARGET_OBSOLETE::tokio_read_body(r, size).await?)),
            0x0141 => Ok(Self::CMSG_ATTACKSWING(CMSG_ATTACKSWING::tokio_read_body(r, size).await?)),
            0x0142 => Ok(Self::CMSG_ATTACKSTOP(CMSG_ATTACKSTOP::tokio_read_body(r, size).await?)),
            0x015A => Ok(Self::CMSG_REPOP_REQUEST(CMSG_REPOP_REQUEST::tokio_read_body(r, size).await?)),
            0x015C => Ok(Self::CMSG_RESURRECT_RESPONSE(CMSG_RESURRECT_RESPONSE::tokio_read_body(r, size).await?)),
            0x015D => Ok(Self::CMSG_LOOT(CMSG_LOOT::tokio_read_body(r, size).await?)),
            0x015E => Ok(Self::CMSG_LOOT_MONEY(CMSG_LOOT_MONEY::tokio_read_body(r, size).await?)),
            0x015F => Ok(Self::CMSG_LOOT_RELEASE(CMSG_LOOT_RELEASE::tokio_read_body(r, size).await?)),
            0x016C => Ok(Self::CMSG_DUEL_ACCEPTED(CMSG_DUEL_ACCEPTED::tokio_read_body(r, size).await?)),
            0x016D => Ok(Self::CMSG_DUEL_CANCELLED(CMSG_DUEL_CANCELLED::tokio_read_body(r, size).await?)),
            0x0171 => Ok(Self::CMSG_MOUNTSPECIAL_ANIM(CMSG_MOUNTSPECIAL_ANIM::tokio_read_body(r, size).await?)),
            0x0174 => Ok(Self::CMSG_PET_SET_ACTION(CMSG_PET_SET_ACTION::tokio_read_body(r, size).await?)),
            0x0175 => Ok(Self::CMSG_PET_ACTION(CMSG_PET_ACTION::tokio_read_body(r, size).await?)),
            0x0176 => Ok(Self::CMSG_PET_ABANDON(CMSG_PET_ABANDON::tokio_read_body(r, size).await?)),
            0x0177 => Ok(Self::CMSG_PET_RENAME(CMSG_PET_RENAME::tokio_read_body(r, size).await?)),
            0x017B => Ok(Self::CMSG_GOSSIP_HELLO(CMSG_GOSSIP_HELLO::tokio_read_body(r, size).await?)),
            0x017C => Ok(Self::CMSG_GOSSIP_SELECT_OPTION(CMSG_GOSSIP_SELECT_OPTION::tokio_read_body(r, size).await?)),
            0x017F => Ok(Self::CMSG_NPC_TEXT_QUERY(CMSG_NPC_TEXT_QUERY::tokio_read_body(r, size).await?)),
            0x0182 => Ok(Self::CMSG_QUESTGIVER_STATUS_QUERY(CMSG_QUESTGIVER_STATUS_QUERY::tokio_read_body(r, size).await?)),
            0x0184 => Ok(Self::CMSG_QUESTGIVER_HELLO(CMSG_QUESTGIVER_HELLO::tokio_read_body(r, size).await?)),
            0x0186 => Ok(Self::CMSG_QUESTGIVER_QUERY_QUEST(CMSG_QUESTGIVER_QUERY_QUEST::tokio_read_body(r, size).await?)),
            0x0187 => Ok(Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(CMSG_QUESTGIVER_QUEST_AUTOLAUNCH::tokio_read_body(r, size).await?)),
            0x0189 => Ok(Self::CMSG_QUESTGIVER_ACCEPT_QUEST(CMSG_QUESTGIVER_ACCEPT_QUEST::tokio_read_body(r, size).await?)),
            0x018A => Ok(Self::CMSG_QUESTGIVER_COMPLETE_QUEST(CMSG_QUESTGIVER_COMPLETE_QUEST::tokio_read_body(r, size).await?)),
            0x018C => Ok(Self::CMSG_QUESTGIVER_REQUEST_REWARD(CMSG_QUESTGIVER_REQUEST_REWARD::tokio_read_body(r, size).await?)),
            0x018E => Ok(Self::CMSG_QUESTGIVER_CHOOSE_REWARD(CMSG_QUESTGIVER_CHOOSE_REWARD::tokio_read_body(r, size).await?)),
            0x0190 => Ok(Self::CMSG_QUESTGIVER_CANCEL(CMSG_QUESTGIVER_CANCEL::tokio_read_body(r, size).await?)),
            0x0193 => Ok(Self::CMSG_QUESTLOG_SWAP_QUEST(CMSG_QUESTLOG_SWAP_QUEST::tokio_read_body(r, size).await?)),
            0x0194 => Ok(Self::CMSG_QUESTLOG_REMOVE_QUEST(CMSG_QUESTLOG_REMOVE_QUEST::tokio_read_body(r, size).await?)),
            0x019B => Ok(Self::CMSG_QUEST_CONFIRM_ACCEPT(CMSG_QUEST_CONFIRM_ACCEPT::tokio_read_body(r, size).await?)),
            0x019D => Ok(Self::CMSG_PUSHQUESTTOPARTY(CMSG_PUSHQUESTTOPARTY::tokio_read_body(r, size).await?)),
            0x019E => Ok(Self::CMSG_LIST_INVENTORY(CMSG_LIST_INVENTORY::tokio_read_body(r, size).await?)),
            0x01A0 => Ok(Self::CMSG_SELL_ITEM(CMSG_SELL_ITEM::tokio_read_body(r, size).await?)),
            0x01A2 => Ok(Self::CMSG_BUY_ITEM(CMSG_BUY_ITEM::tokio_read_body(r, size).await?)),
            0x01A3 => Ok(Self::CMSG_BUY_ITEM_IN_SLOT(CMSG_BUY_ITEM_IN_SLOT::tokio_read_body(r, size).await?)),
            0x01AA => Ok(Self::CMSG_TAXINODE_STATUS_QUERY(CMSG_TAXINODE_STATUS_QUERY::tokio_read_body(r, size).await?)),
            0x01AC => Ok(Self::CMSG_TAXIQUERYAVAILABLENODES(CMSG_TAXIQUERYAVAILABLENODES::tokio_read_body(r, size).await?)),
            0x01AD => Ok(Self::CMSG_ACTIVATETAXI(CMSG_ACTIVATETAXI::tokio_read_body(r, size).await?)),
            0x01B0 => Ok(Self::CMSG_TRAINER_LIST(CMSG_TRAINER_LIST::tokio_read_body(r, size).await?)),
            0x01B2 => Ok(Self::CMSG_TRAINER_BUY_SPELL(CMSG_TRAINER_BUY_SPELL::tokio_read_body(r, size).await?)),
            0x01B5 => Ok(Self::CMSG_BINDER_ACTIVATE(CMSG_BINDER_ACTIVATE::tokio_read_body(r, size).await?)),
            0x01B7 => Ok(Self::CMSG_BANKER_ACTIVATE(CMSG_BANKER_ACTIVATE::tokio_read_body(r, size).await?)),
            0x01B9 => Ok(Self::CMSG_BUY_BANK_SLOT(CMSG_BUY_BANK_SLOT::tokio_read_body(r, size).await?)),
            0x01BB => Ok(Self::CMSG_PETITION_SHOWLIST(CMSG_PETITION_SHOWLIST::tokio_read_body(r, size).await?)),
            0x01BD => Ok(Self::CMSG_PETITION_BUY(CMSG_PETITION_BUY::tokio_read_body(r, size).await?)),
            0x01BE => Ok(Self::CMSG_PETITION_SHOW_SIGNATURES(CMSG_PETITION_SHOW_SIGNATURES::tokio_read_body(r, size).await?)),
            0x01C0 => Ok(Self::CMSG_PETITION_SIGN(CMSG_PETITION_SIGN::tokio_read_body(r, size).await?)),
            0x01C3 => Ok(Self::CMSG_OFFER_PETITION(CMSG_OFFER_PETITION::tokio_read_body(r, size).await?)),
            0x01C4 => Ok(Self::CMSG_TURN_IN_PETITION(CMSG_TURN_IN_PETITION::tokio_read_body(r, size).await?)),
            0x01C6 => Ok(Self::CMSG_PETITION_QUERY(CMSG_PETITION_QUERY::tokio_read_body(r, size).await?)),
            0x01CA => Ok(Self::CMSG_BUG(CMSG_BUG::tokio_read_body(r, size).await?)),
            0x01CC => Ok(Self::CMSG_PLAYED_TIME(CMSG_PLAYED_TIME::tokio_read_body(r, size).await?)),
            0x01CE => Ok(Self::CMSG_QUERY_TIME(CMSG_QUERY_TIME::tokio_read_body(r, size).await?)),
            0x01D2 => Ok(Self::CMSG_RECLAIM_CORPSE(CMSG_RECLAIM_CORPSE::tokio_read_body(r, size).await?)),
            0x01D3 => Ok(Self::CMSG_WRAP_ITEM(CMSG_WRAP_ITEM::tokio_read_body(r, size).await?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(MSG_MINIMAP_PING_Client::tokio_read_body(r, size).await?)),
            0x01DC => Ok(Self::CMSG_PING(CMSG_PING::tokio_read_body(r, size).await?)),
            0x01E0 => Ok(Self::CMSG_SETSHEATHED(CMSG_SETSHEATHED::tokio_read_body(r, size).await?)),
            0x01ED => Ok(Self::CMSG_AUTH_SESSION(CMSG_AUTH_SESSION::tokio_read_body(r, size).await?)),
            0x01F0 => Ok(Self::CMSG_PET_CAST_SPELL(CMSG_PET_CAST_SPELL::tokio_read_body(r, size).await?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Client::tokio_read_body(r, size).await?)),
            0x01F4 => Ok(Self::CMSG_ZONEUPDATE(CMSG_ZONEUPDATE::tokio_read_body(r, size).await?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Client::tokio_read_body(r, size).await?)),
            0x01FF => Ok(Self::MSG_LOOKING_FOR_GROUP(MSG_LOOKING_FOR_GROUP_Client::tokio_read_body(r, size).await?)),
            0x0202 => Ok(Self::CMSG_UNLEARN_SKILL(CMSG_UNLEARN_SKILL::tokio_read_body(r, size).await?)),
            0x0205 => Ok(Self::CMSG_GMTICKET_CREATE(CMSG_GMTICKET_CREATE::tokio_read_body(r, size).await?)),
            0x0207 => Ok(Self::CMSG_GMTICKET_UPDATETEXT(CMSG_GMTICKET_UPDATETEXT::tokio_read_body(r, size).await?)),
            0x020A => Ok(Self::CMSG_REQUEST_ACCOUNT_DATA(CMSG_REQUEST_ACCOUNT_DATA::tokio_read_body(r, size).await?)),
            0x0211 => Ok(Self::CMSG_GMTICKET_GETTICKET(CMSG_GMTICKET_GETTICKET::tokio_read_body(r, size).await?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Client::tokio_read_body(r, size).await?)),
            0x0217 => Ok(Self::CMSG_GMTICKET_DELETETICKET(CMSG_GMTICKET_DELETETICKET::tokio_read_body(r, size).await?)),
            0x021A => Ok(Self::CMSG_GMTICKET_SYSTEMSTATUS(CMSG_GMTICKET_SYSTEMSTATUS::tokio_read_body(r, size).await?)),
            0x021C => Ok(Self::CMSG_SPIRIT_HEALER_ACTIVATE(CMSG_SPIRIT_HEALER_ACTIVATE::tokio_read_body(r, size).await?)),
            0x0225 => Ok(Self::CMSG_CHAT_IGNORED(CMSG_CHAT_IGNORED::tokio_read_body(r, size).await?)),
            0x0231 => Ok(Self::CMSG_GUILD_RANK(CMSG_GUILD_RANK::tokio_read_body(r, size).await?)),
            0x0232 => Ok(Self::CMSG_GUILD_ADD_RANK(CMSG_GUILD_ADD_RANK::tokio_read_body(r, size).await?)),
            0x0233 => Ok(Self::CMSG_GUILD_DEL_RANK(CMSG_GUILD_DEL_RANK::tokio_read_body(r, size).await?)),
            0x0234 => Ok(Self::CMSG_GUILD_SET_PUBLIC_NOTE(CMSG_GUILD_SET_PUBLIC_NOTE::tokio_read_body(r, size).await?)),
            0x0235 => Ok(Self::CMSG_GUILD_SET_OFFICER_NOTE(CMSG_GUILD_SET_OFFICER_NOTE::tokio_read_body(r, size).await?)),
            0x0238 => Ok(Self::CMSG_SEND_MAIL(CMSG_SEND_MAIL::tokio_read_body(r, size).await?)),
            0x023A => Ok(Self::CMSG_GET_MAIL_LIST(CMSG_GET_MAIL_LIST::tokio_read_body(r, size).await?)),
            0x023C => Ok(Self::CMSG_BATTLEFIELD_LIST(CMSG_BATTLEFIELD_LIST::tokio_read_body(r, size).await?)),
            0x023E => Ok(Self::CMSG_BATTLEFIELD_JOIN(CMSG_BATTLEFIELD_JOIN::tokio_read_body(r, size).await?)),
            0x0243 => Ok(Self::CMSG_ITEM_TEXT_QUERY(CMSG_ITEM_TEXT_QUERY::tokio_read_body(r, size).await?)),
            0x0245 => Ok(Self::CMSG_MAIL_TAKE_MONEY(CMSG_MAIL_TAKE_MONEY::tokio_read_body(r, size).await?)),
            0x0246 => Ok(Self::CMSG_MAIL_TAKE_ITEM(CMSG_MAIL_TAKE_ITEM::tokio_read_body(r, size).await?)),
            0x0247 => Ok(Self::CMSG_MAIL_MARK_AS_READ(CMSG_MAIL_MARK_AS_READ::tokio_read_body(r, size).await?)),
            0x0248 => Ok(Self::CMSG_MAIL_RETURN_TO_SENDER(CMSG_MAIL_RETURN_TO_SENDER::tokio_read_body(r, size).await?)),
            0x0249 => Ok(Self::CMSG_MAIL_DELETE(CMSG_MAIL_DELETE::tokio_read_body(r, size).await?)),
            0x024A => Ok(Self::CMSG_MAIL_CREATE_TEXT_ITEM(CMSG_MAIL_CREATE_TEXT_ITEM::tokio_read_body(r, size).await?)),
            0x0251 => Ok(Self::CMSG_LEARN_TALENT(CMSG_LEARN_TALENT::tokio_read_body(r, size).await?)),
            0x0253 => Ok(Self::CMSG_TOGGLE_PVP(CMSG_TOGGLE_PVP::tokio_read_body(r, size).await?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Client::tokio_read_body(r, size).await?)),
            0x0256 => Ok(Self::CMSG_AUCTION_SELL_ITEM(CMSG_AUCTION_SELL_ITEM::tokio_read_body(r, size).await?)),
            0x0257 => Ok(Self::CMSG_AUCTION_REMOVE_ITEM(CMSG_AUCTION_REMOVE_ITEM::tokio_read_body(r, size).await?)),
            0x0258 => Ok(Self::CMSG_AUCTION_LIST_ITEMS(CMSG_AUCTION_LIST_ITEMS::tokio_read_body(r, size).await?)),
            0x0259 => Ok(Self::CMSG_AUCTION_LIST_OWNER_ITEMS(CMSG_AUCTION_LIST_OWNER_ITEMS::tokio_read_body(r, size).await?)),
            0x025A => Ok(Self::CMSG_AUCTION_PLACE_BID(CMSG_AUCTION_PLACE_BID::tokio_read_body(r, size).await?)),
            0x0264 => Ok(Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(CMSG_AUCTION_LIST_BIDDER_ITEMS::tokio_read_body(r, size).await?)),
            0x0268 => Ok(Self::CMSG_SET_AMMO(CMSG_SET_AMMO::tokio_read_body(r, size).await?)),
            0x026A => Ok(Self::CMSG_SET_ACTIVE_MOVER(CMSG_SET_ACTIVE_MOVER::tokio_read_body(r, size).await?)),
            0x026B => Ok(Self::CMSG_PET_CANCEL_AURA(CMSG_PET_CANCEL_AURA::tokio_read_body(r, size).await?)),
            0x026D => Ok(Self::CMSG_CANCEL_AUTO_REPEAT_SPELL(CMSG_CANCEL_AUTO_REPEAT_SPELL::tokio_read_body(r, size).await?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Client::tokio_read_body(r, size).await?)),
            0x0270 => Ok(Self::CMSG_STABLE_PET(CMSG_STABLE_PET::tokio_read_body(r, size).await?)),
            0x0271 => Ok(Self::CMSG_UNSTABLE_PET(CMSG_UNSTABLE_PET::tokio_read_body(r, size).await?)),
            0x0272 => Ok(Self::CMSG_BUY_STABLE_SLOT(CMSG_BUY_STABLE_SLOT::tokio_read_body(r, size).await?)),
            0x0275 => Ok(Self::CMSG_STABLE_SWAP_PET(CMSG_STABLE_SWAP_PET::tokio_read_body(r, size).await?)),
            0x0279 => Ok(Self::CMSG_REQUEST_PET_INFO(CMSG_REQUEST_PET_INFO::tokio_read_body(r, size).await?)),
            0x027A => Ok(Self::CMSG_FAR_SIGHT(CMSG_FAR_SIGHT::tokio_read_body(r, size).await?)),
            0x027E => Ok(Self::CMSG_GROUP_CHANGE_SUB_GROUP(CMSG_GROUP_CHANGE_SUB_GROUP::tokio_read_body(r, size).await?)),
            0x027F => Ok(Self::CMSG_REQUEST_PARTY_MEMBER_STATS(CMSG_REQUEST_PARTY_MEMBER_STATS::tokio_read_body(r, size).await?)),
            0x0280 => Ok(Self::CMSG_GROUP_SWAP_SUB_GROUP(CMSG_GROUP_SWAP_SUB_GROUP::tokio_read_body(r, size).await?)),
            0x0282 => Ok(Self::CMSG_AUTOSTORE_BANK_ITEM(CMSG_AUTOSTORE_BANK_ITEM::tokio_read_body(r, size).await?)),
            0x0283 => Ok(Self::CMSG_AUTOBANK_ITEM(CMSG_AUTOBANK_ITEM::tokio_read_body(r, size).await?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Client::tokio_read_body(r, size).await?)),
            0x028E => Ok(Self::CMSG_GROUP_RAID_CONVERT(CMSG_GROUP_RAID_CONVERT::tokio_read_body(r, size).await?)),
            0x028F => Ok(Self::CMSG_GROUP_ASSISTANT_LEADER(CMSG_GROUP_ASSISTANT_LEADER::tokio_read_body(r, size).await?)),
            0x0290 => Ok(Self::CMSG_BUYBACK_ITEM(CMSG_BUYBACK_ITEM::tokio_read_body(r, size).await?)),
            0x0292 => Ok(Self::CMSG_MEETINGSTONE_JOIN(CMSG_MEETINGSTONE_JOIN::tokio_read_body(r, size).await?)),
            0x0293 => Ok(Self::CMSG_MEETINGSTONE_LEAVE(CMSG_MEETINGSTONE_LEAVE::tokio_read_body(r, size).await?)),
            0x0296 => Ok(Self::CMSG_MEETINGSTONE_INFO(CMSG_MEETINGSTONE_INFO::tokio_read_body(r, size).await?)),
            0x029B => Ok(Self::CMSG_CANCEL_GROWTH_AURA(CMSG_CANCEL_GROWTH_AURA::tokio_read_body(r, size).await?)),
            0x02A0 => Ok(Self::CMSG_LOOT_ROLL(CMSG_LOOT_ROLL::tokio_read_body(r, size).await?)),
            0x02A3 => Ok(Self::CMSG_LOOT_MASTER_GIVE(CMSG_LOOT_MASTER_GIVE::tokio_read_body(r, size).await?)),
            0x02A8 => Ok(Self::CMSG_REPAIR_ITEM(CMSG_REPAIR_ITEM::tokio_read_body(r, size).await?)),
            0x02AA => Ok(Self::MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Client::tokio_read_body(r, size).await?)),
            0x02AC => Ok(Self::CMSG_SUMMON_RESPONSE(CMSG_SUMMON_RESPONSE::tokio_read_body(r, size).await?)),
            0x02B3 => Ok(Self::CMSG_SELF_RES(CMSG_SELF_RES::tokio_read_body(r, size).await?)),
            0x02B9 => Ok(Self::CMSG_TOGGLE_HELM(CMSG_TOGGLE_HELM::tokio_read_body(r, size).await?)),
            0x02BA => Ok(Self::CMSG_TOGGLE_CLOAK(CMSG_TOGGLE_CLOAK::tokio_read_body(r, size).await?)),
            0x02BF => Ok(Self::CMSG_SET_ACTIONBAR_TOGGLES(CMSG_SET_ACTIONBAR_TOGGLES::tokio_read_body(r, size).await?)),
            0x02C4 => Ok(Self::CMSG_ITEM_NAME_QUERY(CMSG_ITEM_NAME_QUERY::tokio_read_body(r, size).await?)),
            0x02C7 => Ok(Self::CMSG_CHAR_RENAME(CMSG_CHAR_RENAME::tokio_read_body(r, size).await?)),
            0x02C9 => Ok(Self::CMSG_MOVE_SPLINE_DONE(CMSG_MOVE_SPLINE_DONE::tokio_read_body(r, size).await?)),
            0x02CA => Ok(Self::CMSG_MOVE_FALL_RESET(CMSG_MOVE_FALL_RESET::tokio_read_body(r, size).await?)),
            0x02CD => Ok(Self::CMSG_REQUEST_RAID_INFO(CMSG_REQUEST_RAID_INFO::tokio_read_body(r, size).await?)),
            0x02CE => Ok(Self::CMSG_MOVE_TIME_SKIPPED(CMSG_MOVE_TIME_SKIPPED::tokio_read_body(r, size).await?)),
            0x02CF => Ok(Self::CMSG_MOVE_FEATHER_FALL_ACK(CMSG_MOVE_FEATHER_FALL_ACK::tokio_read_body(r, size).await?)),
            0x02D0 => Ok(Self::CMSG_MOVE_WATER_WALK_ACK(CMSG_MOVE_WATER_WALK_ACK::tokio_read_body(r, size).await?)),
            0x02D1 => Ok(Self::CMSG_MOVE_NOT_ACTIVE_MOVER(CMSG_MOVE_NOT_ACTIVE_MOVER::tokio_read_body(r, size).await?)),
            0x02D3 => Ok(Self::CMSG_BATTLEFIELD_STATUS(CMSG_BATTLEFIELD_STATUS::tokio_read_body(r, size).await?)),
            0x02D5 => Ok(Self::CMSG_BATTLEFIELD_PORT(CMSG_BATTLEFIELD_PORT::tokio_read_body(r, size).await?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Client::tokio_read_body(r, size).await?)),
            0x02D7 => Ok(Self::CMSG_BATTLEMASTER_HELLO(CMSG_BATTLEMASTER_HELLO::tokio_read_body(r, size).await?)),
            0x02DB => Ok(Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(CMSG_FORCE_WALK_SPEED_CHANGE_ACK::tokio_read_body(r, size).await?)),
            0x02DD => Ok(Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK::tokio_read_body(r, size).await?)),
            0x02DF => Ok(Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(CMSG_FORCE_TURN_RATE_CHANGE_ACK::tokio_read_body(r, size).await?)),
            0x02E0 => Ok(Self::MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_Client::tokio_read_body(r, size).await?)),
            0x02E1 => Ok(Self::CMSG_LEAVE_BATTLEFIELD(CMSG_LEAVE_BATTLEFIELD::tokio_read_body(r, size).await?)),
            0x02E2 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUERY(CMSG_AREA_SPIRIT_HEALER_QUERY::tokio_read_body(r, size).await?)),
            0x02E3 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(CMSG_AREA_SPIRIT_HEALER_QUEUE::tokio_read_body(r, size).await?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Client::tokio_read_body(r, size).await?)),
            0x02EA => Ok(Self::CMSG_PET_STOP_ATTACK(CMSG_PET_STOP_ATTACK::tokio_read_body(r, size).await?)),
            0x02EE => Ok(Self::CMSG_BATTLEMASTER_JOIN(CMSG_BATTLEMASTER_JOIN::tokio_read_body(r, size).await?)),
            0x02F0 => Ok(Self::CMSG_PET_UNLEARN(CMSG_PET_UNLEARN::tokio_read_body(r, size).await?)),
            0x02F3 => Ok(Self::CMSG_PET_SPELL_AUTOCAST(CMSG_PET_SPELL_AUTOCAST::tokio_read_body(r, size).await?)),
            0x02FC => Ok(Self::CMSG_GUILD_INFO_TEXT(CMSG_GUILD_INFO_TEXT::tokio_read_body(r, size).await?)),
            0x0312 => Ok(Self::CMSG_ACTIVATETAXIEXPRESS(CMSG_ACTIVATETAXIEXPRESS::tokio_read_body(r, size).await?)),
            0x0317 => Ok(Self::CMSG_SET_FACTION_INACTIVE(CMSG_SET_FACTION_INACTIVE::tokio_read_body(r, size).await?)),
            0x0318 => Ok(Self::CMSG_SET_WATCHED_FACTION(CMSG_SET_WATCHED_FACTION::tokio_read_body(r, size).await?)),
            0x031D => Ok(Self::CMSG_RESET_INSTANCES(CMSG_RESET_INSTANCES::tokio_read_body(r, size).await?)),
            0x0321 => Ok(Self::MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Client::tokio_read_body(r, size).await?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Client::tokio_read_body(r, size).await?)),
            0x032A => Ok(Self::CMSG_GMSURVEY_SUBMIT(CMSG_GMSURVEY_SUBMIT::tokio_read_body(r, size).await?)),
            _ => Err(ClientOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }
    #[cfg(feature = "tokio")]
    pub async fn tokio_read_encrypted<R: AsyncReadExt + Unpin + Send, D: Decrypter + Send>(r: &mut R, d: &mut D) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let mut header = [0u8; 6];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_client_header(header);
        let header_size = (header.size - 4) as u32;
        match header.opcode {
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD::tokio_read_body(r, header_size).await?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD::tokio_read_body(r, header_size).await?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(MSG_MOVE_STOP::tokio_read_body(r, header_size).await?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT::tokio_read_body(r, header_size).await?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT::tokio_read_body(r, header_size).await?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE::tokio_read_body(r, header_size).await?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(MSG_MOVE_JUMP::tokio_read_body(r, header_size).await?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT::tokio_read_body(r, header_size).await?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT::tokio_read_body(r, header_size).await?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN::tokio_read_body(r, header_size).await?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP::tokio_read_body(r, header_size).await?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN::tokio_read_body(r, header_size).await?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH::tokio_read_body(r, header_size).await?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE::tokio_read_body(r, header_size).await?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE::tokio_read_body(r, header_size).await?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK::tokio_read_body(r, header_size).await?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND::tokio_read_body(r, header_size).await?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM::tokio_read_body(r, header_size).await?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM::tokio_read_body(r, header_size).await?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING::tokio_read_body(r, header_size).await?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH::tokio_read_body(r, header_size).await?)),
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK::tokio_read_body(r, header_size).await?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT::tokio_read_body(r, header_size).await?)),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(MSG_PETITION_DECLINE::tokio_read_body(r, header_size).await?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE::tokio_read_body(r, header_size).await?)),
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT::tokio_read_body(r, header_size).await?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(MSG_PETITION_RENAME::tokio_read_body(r, header_size).await?)),
            0x0008 => Ok(Self::CMSG_WORLD_TELEPORT(CMSG_WORLD_TELEPORT::tokio_read_body(r, header_size).await?)),
            0x0036 => Ok(Self::CMSG_CHAR_CREATE(CMSG_CHAR_CREATE::tokio_read_body(r, header_size).await?)),
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(CMSG_CHAR_ENUM::tokio_read_body(r, header_size).await?)),
            0x0038 => Ok(Self::CMSG_CHAR_DELETE(CMSG_CHAR_DELETE::tokio_read_body(r, header_size).await?)),
            0x003D => Ok(Self::CMSG_PLAYER_LOGIN(CMSG_PLAYER_LOGIN::tokio_read_body(r, header_size).await?)),
            0x004A => Ok(Self::CMSG_PLAYER_LOGOUT(CMSG_PLAYER_LOGOUT::tokio_read_body(r, header_size).await?)),
            0x004B => Ok(Self::CMSG_LOGOUT_REQUEST(CMSG_LOGOUT_REQUEST::tokio_read_body(r, header_size).await?)),
            0x004E => Ok(Self::CMSG_LOGOUT_CANCEL(CMSG_LOGOUT_CANCEL::tokio_read_body(r, header_size).await?)),
            0x0050 => Ok(Self::CMSG_NAME_QUERY(CMSG_NAME_QUERY::tokio_read_body(r, header_size).await?)),
            0x0052 => Ok(Self::CMSG_PET_NAME_QUERY(CMSG_PET_NAME_QUERY::tokio_read_body(r, header_size).await?)),
            0x0054 => Ok(Self::CMSG_GUILD_QUERY(CMSG_GUILD_QUERY::tokio_read_body(r, header_size).await?)),
            0x0056 => Ok(Self::CMSG_ITEM_QUERY_SINGLE(CMSG_ITEM_QUERY_SINGLE::tokio_read_body(r, header_size).await?)),
            0x005A => Ok(Self::CMSG_PAGE_TEXT_QUERY(CMSG_PAGE_TEXT_QUERY::tokio_read_body(r, header_size).await?)),
            0x005C => Ok(Self::CMSG_QUEST_QUERY(CMSG_QUEST_QUERY::tokio_read_body(r, header_size).await?)),
            0x005E => Ok(Self::CMSG_GAMEOBJECT_QUERY(CMSG_GAMEOBJECT_QUERY::tokio_read_body(r, header_size).await?)),
            0x0060 => Ok(Self::CMSG_CREATURE_QUERY(CMSG_CREATURE_QUERY::tokio_read_body(r, header_size).await?)),
            0x0062 => Ok(Self::CMSG_WHO(CMSG_WHO::tokio_read_body(r, header_size).await?)),
            0x0064 => Ok(Self::CMSG_WHOIS(CMSG_WHOIS::tokio_read_body(r, header_size).await?)),
            0x0066 => Ok(Self::CMSG_FRIEND_LIST(CMSG_FRIEND_LIST::tokio_read_body(r, header_size).await?)),
            0x0069 => Ok(Self::CMSG_ADD_FRIEND(CMSG_ADD_FRIEND::tokio_read_body(r, header_size).await?)),
            0x006A => Ok(Self::CMSG_DEL_FRIEND(CMSG_DEL_FRIEND::tokio_read_body(r, header_size).await?)),
            0x006C => Ok(Self::CMSG_ADD_IGNORE(CMSG_ADD_IGNORE::tokio_read_body(r, header_size).await?)),
            0x006D => Ok(Self::CMSG_DEL_IGNORE(CMSG_DEL_IGNORE::tokio_read_body(r, header_size).await?)),
            0x006E => Ok(Self::CMSG_GROUP_INVITE(CMSG_GROUP_INVITE::tokio_read_body(r, header_size).await?)),
            0x0072 => Ok(Self::CMSG_GROUP_ACCEPT(CMSG_GROUP_ACCEPT::tokio_read_body(r, header_size).await?)),
            0x0073 => Ok(Self::CMSG_GROUP_DECLINE(CMSG_GROUP_DECLINE::tokio_read_body(r, header_size).await?)),
            0x0075 => Ok(Self::CMSG_GROUP_UNINVITE(CMSG_GROUP_UNINVITE::tokio_read_body(r, header_size).await?)),
            0x0076 => Ok(Self::CMSG_GROUP_UNINVITE_GUID(CMSG_GROUP_UNINVITE_GUID::tokio_read_body(r, header_size).await?)),
            0x0078 => Ok(Self::CMSG_GROUP_SET_LEADER(CMSG_GROUP_SET_LEADER::tokio_read_body(r, header_size).await?)),
            0x007A => Ok(Self::CMSG_LOOT_METHOD(CMSG_LOOT_METHOD::tokio_read_body(r, header_size).await?)),
            0x007B => Ok(Self::CMSG_GROUP_DISBAND(CMSG_GROUP_DISBAND::tokio_read_body(r, header_size).await?)),
            0x0081 => Ok(Self::CMSG_GUILD_CREATE(CMSG_GUILD_CREATE::tokio_read_body(r, header_size).await?)),
            0x0082 => Ok(Self::CMSG_GUILD_INVITE(CMSG_GUILD_INVITE::tokio_read_body(r, header_size).await?)),
            0x0084 => Ok(Self::CMSG_GUILD_ACCEPT(CMSG_GUILD_ACCEPT::tokio_read_body(r, header_size).await?)),
            0x0085 => Ok(Self::CMSG_GUILD_DECLINE(CMSG_GUILD_DECLINE::tokio_read_body(r, header_size).await?)),
            0x0087 => Ok(Self::CMSG_GUILD_INFO(CMSG_GUILD_INFO::tokio_read_body(r, header_size).await?)),
            0x0089 => Ok(Self::CMSG_GUILD_ROSTER(CMSG_GUILD_ROSTER::tokio_read_body(r, header_size).await?)),
            0x008B => Ok(Self::CMSG_GUILD_PROMOTE(CMSG_GUILD_PROMOTE::tokio_read_body(r, header_size).await?)),
            0x008C => Ok(Self::CMSG_GUILD_DEMOTE(CMSG_GUILD_DEMOTE::tokio_read_body(r, header_size).await?)),
            0x008D => Ok(Self::CMSG_GUILD_LEAVE(CMSG_GUILD_LEAVE::tokio_read_body(r, header_size).await?)),
            0x008E => Ok(Self::CMSG_GUILD_REMOVE(CMSG_GUILD_REMOVE::tokio_read_body(r, header_size).await?)),
            0x008F => Ok(Self::CMSG_GUILD_DISBAND(CMSG_GUILD_DISBAND::tokio_read_body(r, header_size).await?)),
            0x0090 => Ok(Self::CMSG_GUILD_LEADER(CMSG_GUILD_LEADER::tokio_read_body(r, header_size).await?)),
            0x0091 => Ok(Self::CMSG_GUILD_MOTD(CMSG_GUILD_MOTD::tokio_read_body(r, header_size).await?)),
            0x0095 => Ok(Self::CMSG_MESSAGECHAT(CMSG_MESSAGECHAT::tokio_read_body(r, header_size).await?)),
            0x0097 => Ok(Self::CMSG_JOIN_CHANNEL(CMSG_JOIN_CHANNEL::tokio_read_body(r, header_size).await?)),
            0x0098 => Ok(Self::CMSG_LEAVE_CHANNEL(CMSG_LEAVE_CHANNEL::tokio_read_body(r, header_size).await?)),
            0x009A => Ok(Self::CMSG_CHANNEL_LIST(CMSG_CHANNEL_LIST::tokio_read_body(r, header_size).await?)),
            0x009C => Ok(Self::CMSG_CHANNEL_PASSWORD(CMSG_CHANNEL_PASSWORD::tokio_read_body(r, header_size).await?)),
            0x009D => Ok(Self::CMSG_CHANNEL_SET_OWNER(CMSG_CHANNEL_SET_OWNER::tokio_read_body(r, header_size).await?)),
            0x009E => Ok(Self::CMSG_CHANNEL_OWNER(CMSG_CHANNEL_OWNER::tokio_read_body(r, header_size).await?)),
            0x009F => Ok(Self::CMSG_CHANNEL_MODERATOR(CMSG_CHANNEL_MODERATOR::tokio_read_body(r, header_size).await?)),
            0x00A0 => Ok(Self::CMSG_CHANNEL_UNMODERATOR(CMSG_CHANNEL_UNMODERATOR::tokio_read_body(r, header_size).await?)),
            0x00A1 => Ok(Self::CMSG_CHANNEL_MUTE(CMSG_CHANNEL_MUTE::tokio_read_body(r, header_size).await?)),
            0x00A2 => Ok(Self::CMSG_CHANNEL_UNMUTE(CMSG_CHANNEL_UNMUTE::tokio_read_body(r, header_size).await?)),
            0x00A3 => Ok(Self::CMSG_CHANNEL_INVITE(CMSG_CHANNEL_INVITE::tokio_read_body(r, header_size).await?)),
            0x00A4 => Ok(Self::CMSG_CHANNEL_KICK(CMSG_CHANNEL_KICK::tokio_read_body(r, header_size).await?)),
            0x00A5 => Ok(Self::CMSG_CHANNEL_BAN(CMSG_CHANNEL_BAN::tokio_read_body(r, header_size).await?)),
            0x00A6 => Ok(Self::CMSG_CHANNEL_UNBAN(CMSG_CHANNEL_UNBAN::tokio_read_body(r, header_size).await?)),
            0x00A7 => Ok(Self::CMSG_CHANNEL_ANNOUNCEMENTS(CMSG_CHANNEL_ANNOUNCEMENTS::tokio_read_body(r, header_size).await?)),
            0x00A8 => Ok(Self::CMSG_CHANNEL_MODERATE(CMSG_CHANNEL_MODERATE::tokio_read_body(r, header_size).await?)),
            0x00A9 => Ok(Self::SMSG_UPDATE_OBJECT(SMSG_UPDATE_OBJECT::tokio_read_body(r, header_size).await?)),
            0x00AB => Ok(Self::CMSG_USE_ITEM(CMSG_USE_ITEM::tokio_read_body(r, header_size).await?)),
            0x00AC => Ok(Self::CMSG_OPEN_ITEM(CMSG_OPEN_ITEM::tokio_read_body(r, header_size).await?)),
            0x00AD => Ok(Self::CMSG_READ_ITEM(CMSG_READ_ITEM::tokio_read_body(r, header_size).await?)),
            0x00B1 => Ok(Self::CMSG_GAMEOBJ_USE(CMSG_GAMEOBJ_USE::tokio_read_body(r, header_size).await?)),
            0x00B4 => Ok(Self::CMSG_AREATRIGGER(CMSG_AREATRIGGER::tokio_read_body(r, header_size).await?)),
            0x00E1 => Ok(Self::CMSG_MOVE_SET_RAW_POSITION(CMSG_MOVE_SET_RAW_POSITION::tokio_read_body(r, header_size).await?)),
            0x00E3 => Ok(Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_SPEED_CHANGE_ACK::tokio_read_body(r, header_size).await?)),
            0x00E5 => Ok(Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK::tokio_read_body(r, header_size).await?)),
            0x00E7 => Ok(Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_SPEED_CHANGE_ACK::tokio_read_body(r, header_size).await?)),
            0x00E9 => Ok(Self::CMSG_FORCE_MOVE_ROOT_ACK(CMSG_FORCE_MOVE_ROOT_ACK::tokio_read_body(r, header_size).await?)),
            0x00EB => Ok(Self::CMSG_FORCE_MOVE_UNROOT_ACK(CMSG_FORCE_MOVE_UNROOT_ACK::tokio_read_body(r, header_size).await?)),
            0x00F0 => Ok(Self::CMSG_MOVE_KNOCK_BACK_ACK(CMSG_MOVE_KNOCK_BACK_ACK::tokio_read_body(r, header_size).await?)),
            0x00F6 => Ok(Self::CMSG_MOVE_HOVER_ACK(CMSG_MOVE_HOVER_ACK::tokio_read_body(r, header_size).await?)),
            0x00FB => Ok(Self::CMSG_NEXT_CINEMATIC_CAMERA(CMSG_NEXT_CINEMATIC_CAMERA::tokio_read_body(r, header_size).await?)),
            0x00FC => Ok(Self::CMSG_COMPLETE_CINEMATIC(CMSG_COMPLETE_CINEMATIC::tokio_read_body(r, header_size).await?)),
            0x00FE => Ok(Self::CMSG_TUTORIAL_FLAG(CMSG_TUTORIAL_FLAG::tokio_read_body(r, header_size).await?)),
            0x00FF => Ok(Self::CMSG_TUTORIAL_CLEAR(CMSG_TUTORIAL_CLEAR::tokio_read_body(r, header_size).await?)),
            0x0100 => Ok(Self::CMSG_TUTORIAL_RESET(CMSG_TUTORIAL_RESET::tokio_read_body(r, header_size).await?)),
            0x0101 => Ok(Self::CMSG_STANDSTATECHANGE(CMSG_STANDSTATECHANGE::tokio_read_body(r, header_size).await?)),
            0x0102 => Ok(Self::CMSG_EMOTE(CMSG_EMOTE::tokio_read_body(r, header_size).await?)),
            0x0104 => Ok(Self::CMSG_TEXT_EMOTE(CMSG_TEXT_EMOTE::tokio_read_body(r, header_size).await?)),
            0x0108 => Ok(Self::CMSG_AUTOSTORE_LOOT_ITEM(CMSG_AUTOSTORE_LOOT_ITEM::tokio_read_body(r, header_size).await?)),
            0x010A => Ok(Self::CMSG_AUTOEQUIP_ITEM(CMSG_AUTOEQUIP_ITEM::tokio_read_body(r, header_size).await?)),
            0x010B => Ok(Self::CMSG_AUTOSTORE_BAG_ITEM(CMSG_AUTOSTORE_BAG_ITEM::tokio_read_body(r, header_size).await?)),
            0x010C => Ok(Self::CMSG_SWAP_ITEM(CMSG_SWAP_ITEM::tokio_read_body(r, header_size).await?)),
            0x010D => Ok(Self::CMSG_SWAP_INV_ITEM(CMSG_SWAP_INV_ITEM::tokio_read_body(r, header_size).await?)),
            0x010E => Ok(Self::CMSG_SPLIT_ITEM(CMSG_SPLIT_ITEM::tokio_read_body(r, header_size).await?)),
            0x010F => Ok(Self::CMSG_AUTOEQUIP_ITEM_SLOT(CMSG_AUTOEQUIP_ITEM_SLOT::tokio_read_body(r, header_size).await?)),
            0x0111 => Ok(Self::CMSG_DESTROYITEM(CMSG_DESTROYITEM::tokio_read_body(r, header_size).await?)),
            0x0114 => Ok(Self::CMSG_INSPECT(CMSG_INSPECT::tokio_read_body(r, header_size).await?)),
            0x0116 => Ok(Self::CMSG_INITIATE_TRADE(CMSG_INITIATE_TRADE::tokio_read_body(r, header_size).await?)),
            0x0117 => Ok(Self::CMSG_BEGIN_TRADE(CMSG_BEGIN_TRADE::tokio_read_body(r, header_size).await?)),
            0x0118 => Ok(Self::CMSG_BUSY_TRADE(CMSG_BUSY_TRADE::tokio_read_body(r, header_size).await?)),
            0x0119 => Ok(Self::CMSG_IGNORE_TRADE(CMSG_IGNORE_TRADE::tokio_read_body(r, header_size).await?)),
            0x011A => Ok(Self::CMSG_ACCEPT_TRADE(CMSG_ACCEPT_TRADE::tokio_read_body(r, header_size).await?)),
            0x011B => Ok(Self::CMSG_UNACCEPT_TRADE(CMSG_UNACCEPT_TRADE::tokio_read_body(r, header_size).await?)),
            0x011C => Ok(Self::CMSG_CANCEL_TRADE(CMSG_CANCEL_TRADE::tokio_read_body(r, header_size).await?)),
            0x011D => Ok(Self::CMSG_SET_TRADE_ITEM(CMSG_SET_TRADE_ITEM::tokio_read_body(r, header_size).await?)),
            0x011E => Ok(Self::CMSG_CLEAR_TRADE_ITEM(CMSG_CLEAR_TRADE_ITEM::tokio_read_body(r, header_size).await?)),
            0x011F => Ok(Self::CMSG_SET_TRADE_GOLD(CMSG_SET_TRADE_GOLD::tokio_read_body(r, header_size).await?)),
            0x0125 => Ok(Self::CMSG_SET_FACTION_ATWAR(CMSG_SET_FACTION_ATWAR::tokio_read_body(r, header_size).await?)),
            0x0128 => Ok(Self::CMSG_SET_ACTION_BUTTON(CMSG_SET_ACTION_BUTTON::tokio_read_body(r, header_size).await?)),
            0x012E => Ok(Self::CMSG_CAST_SPELL(CMSG_CAST_SPELL::tokio_read_body(r, header_size).await?)),
            0x012F => Ok(Self::CMSG_CANCEL_CAST(CMSG_CANCEL_CAST::tokio_read_body(r, header_size).await?)),
            0x0136 => Ok(Self::CMSG_CANCEL_AURA(CMSG_CANCEL_AURA::tokio_read_body(r, header_size).await?)),
            0x013B => Ok(Self::CMSG_CANCEL_CHANNELLING(CMSG_CANCEL_CHANNELLING::tokio_read_body(r, header_size).await?)),
            0x013D => Ok(Self::CMSG_SET_SELECTION(CMSG_SET_SELECTION::tokio_read_body(r, header_size).await?)),
            0x013E => Ok(Self::CMSG_SET_TARGET_OBSOLETE(CMSG_SET_TARGET_OBSOLETE::tokio_read_body(r, header_size).await?)),
            0x0141 => Ok(Self::CMSG_ATTACKSWING(CMSG_ATTACKSWING::tokio_read_body(r, header_size).await?)),
            0x0142 => Ok(Self::CMSG_ATTACKSTOP(CMSG_ATTACKSTOP::tokio_read_body(r, header_size).await?)),
            0x015A => Ok(Self::CMSG_REPOP_REQUEST(CMSG_REPOP_REQUEST::tokio_read_body(r, header_size).await?)),
            0x015C => Ok(Self::CMSG_RESURRECT_RESPONSE(CMSG_RESURRECT_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x015D => Ok(Self::CMSG_LOOT(CMSG_LOOT::tokio_read_body(r, header_size).await?)),
            0x015E => Ok(Self::CMSG_LOOT_MONEY(CMSG_LOOT_MONEY::tokio_read_body(r, header_size).await?)),
            0x015F => Ok(Self::CMSG_LOOT_RELEASE(CMSG_LOOT_RELEASE::tokio_read_body(r, header_size).await?)),
            0x016C => Ok(Self::CMSG_DUEL_ACCEPTED(CMSG_DUEL_ACCEPTED::tokio_read_body(r, header_size).await?)),
            0x016D => Ok(Self::CMSG_DUEL_CANCELLED(CMSG_DUEL_CANCELLED::tokio_read_body(r, header_size).await?)),
            0x0171 => Ok(Self::CMSG_MOUNTSPECIAL_ANIM(CMSG_MOUNTSPECIAL_ANIM::tokio_read_body(r, header_size).await?)),
            0x0174 => Ok(Self::CMSG_PET_SET_ACTION(CMSG_PET_SET_ACTION::tokio_read_body(r, header_size).await?)),
            0x0175 => Ok(Self::CMSG_PET_ACTION(CMSG_PET_ACTION::tokio_read_body(r, header_size).await?)),
            0x0176 => Ok(Self::CMSG_PET_ABANDON(CMSG_PET_ABANDON::tokio_read_body(r, header_size).await?)),
            0x0177 => Ok(Self::CMSG_PET_RENAME(CMSG_PET_RENAME::tokio_read_body(r, header_size).await?)),
            0x017B => Ok(Self::CMSG_GOSSIP_HELLO(CMSG_GOSSIP_HELLO::tokio_read_body(r, header_size).await?)),
            0x017C => Ok(Self::CMSG_GOSSIP_SELECT_OPTION(CMSG_GOSSIP_SELECT_OPTION::tokio_read_body(r, header_size).await?)),
            0x017F => Ok(Self::CMSG_NPC_TEXT_QUERY(CMSG_NPC_TEXT_QUERY::tokio_read_body(r, header_size).await?)),
            0x0182 => Ok(Self::CMSG_QUESTGIVER_STATUS_QUERY(CMSG_QUESTGIVER_STATUS_QUERY::tokio_read_body(r, header_size).await?)),
            0x0184 => Ok(Self::CMSG_QUESTGIVER_HELLO(CMSG_QUESTGIVER_HELLO::tokio_read_body(r, header_size).await?)),
            0x0186 => Ok(Self::CMSG_QUESTGIVER_QUERY_QUEST(CMSG_QUESTGIVER_QUERY_QUEST::tokio_read_body(r, header_size).await?)),
            0x0187 => Ok(Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(CMSG_QUESTGIVER_QUEST_AUTOLAUNCH::tokio_read_body(r, header_size).await?)),
            0x0189 => Ok(Self::CMSG_QUESTGIVER_ACCEPT_QUEST(CMSG_QUESTGIVER_ACCEPT_QUEST::tokio_read_body(r, header_size).await?)),
            0x018A => Ok(Self::CMSG_QUESTGIVER_COMPLETE_QUEST(CMSG_QUESTGIVER_COMPLETE_QUEST::tokio_read_body(r, header_size).await?)),
            0x018C => Ok(Self::CMSG_QUESTGIVER_REQUEST_REWARD(CMSG_QUESTGIVER_REQUEST_REWARD::tokio_read_body(r, header_size).await?)),
            0x018E => Ok(Self::CMSG_QUESTGIVER_CHOOSE_REWARD(CMSG_QUESTGIVER_CHOOSE_REWARD::tokio_read_body(r, header_size).await?)),
            0x0190 => Ok(Self::CMSG_QUESTGIVER_CANCEL(CMSG_QUESTGIVER_CANCEL::tokio_read_body(r, header_size).await?)),
            0x0193 => Ok(Self::CMSG_QUESTLOG_SWAP_QUEST(CMSG_QUESTLOG_SWAP_QUEST::tokio_read_body(r, header_size).await?)),
            0x0194 => Ok(Self::CMSG_QUESTLOG_REMOVE_QUEST(CMSG_QUESTLOG_REMOVE_QUEST::tokio_read_body(r, header_size).await?)),
            0x019B => Ok(Self::CMSG_QUEST_CONFIRM_ACCEPT(CMSG_QUEST_CONFIRM_ACCEPT::tokio_read_body(r, header_size).await?)),
            0x019D => Ok(Self::CMSG_PUSHQUESTTOPARTY(CMSG_PUSHQUESTTOPARTY::tokio_read_body(r, header_size).await?)),
            0x019E => Ok(Self::CMSG_LIST_INVENTORY(CMSG_LIST_INVENTORY::tokio_read_body(r, header_size).await?)),
            0x01A0 => Ok(Self::CMSG_SELL_ITEM(CMSG_SELL_ITEM::tokio_read_body(r, header_size).await?)),
            0x01A2 => Ok(Self::CMSG_BUY_ITEM(CMSG_BUY_ITEM::tokio_read_body(r, header_size).await?)),
            0x01A3 => Ok(Self::CMSG_BUY_ITEM_IN_SLOT(CMSG_BUY_ITEM_IN_SLOT::tokio_read_body(r, header_size).await?)),
            0x01AA => Ok(Self::CMSG_TAXINODE_STATUS_QUERY(CMSG_TAXINODE_STATUS_QUERY::tokio_read_body(r, header_size).await?)),
            0x01AC => Ok(Self::CMSG_TAXIQUERYAVAILABLENODES(CMSG_TAXIQUERYAVAILABLENODES::tokio_read_body(r, header_size).await?)),
            0x01AD => Ok(Self::CMSG_ACTIVATETAXI(CMSG_ACTIVATETAXI::tokio_read_body(r, header_size).await?)),
            0x01B0 => Ok(Self::CMSG_TRAINER_LIST(CMSG_TRAINER_LIST::tokio_read_body(r, header_size).await?)),
            0x01B2 => Ok(Self::CMSG_TRAINER_BUY_SPELL(CMSG_TRAINER_BUY_SPELL::tokio_read_body(r, header_size).await?)),
            0x01B5 => Ok(Self::CMSG_BINDER_ACTIVATE(CMSG_BINDER_ACTIVATE::tokio_read_body(r, header_size).await?)),
            0x01B7 => Ok(Self::CMSG_BANKER_ACTIVATE(CMSG_BANKER_ACTIVATE::tokio_read_body(r, header_size).await?)),
            0x01B9 => Ok(Self::CMSG_BUY_BANK_SLOT(CMSG_BUY_BANK_SLOT::tokio_read_body(r, header_size).await?)),
            0x01BB => Ok(Self::CMSG_PETITION_SHOWLIST(CMSG_PETITION_SHOWLIST::tokio_read_body(r, header_size).await?)),
            0x01BD => Ok(Self::CMSG_PETITION_BUY(CMSG_PETITION_BUY::tokio_read_body(r, header_size).await?)),
            0x01BE => Ok(Self::CMSG_PETITION_SHOW_SIGNATURES(CMSG_PETITION_SHOW_SIGNATURES::tokio_read_body(r, header_size).await?)),
            0x01C0 => Ok(Self::CMSG_PETITION_SIGN(CMSG_PETITION_SIGN::tokio_read_body(r, header_size).await?)),
            0x01C3 => Ok(Self::CMSG_OFFER_PETITION(CMSG_OFFER_PETITION::tokio_read_body(r, header_size).await?)),
            0x01C4 => Ok(Self::CMSG_TURN_IN_PETITION(CMSG_TURN_IN_PETITION::tokio_read_body(r, header_size).await?)),
            0x01C6 => Ok(Self::CMSG_PETITION_QUERY(CMSG_PETITION_QUERY::tokio_read_body(r, header_size).await?)),
            0x01CA => Ok(Self::CMSG_BUG(CMSG_BUG::tokio_read_body(r, header_size).await?)),
            0x01CC => Ok(Self::CMSG_PLAYED_TIME(CMSG_PLAYED_TIME::tokio_read_body(r, header_size).await?)),
            0x01CE => Ok(Self::CMSG_QUERY_TIME(CMSG_QUERY_TIME::tokio_read_body(r, header_size).await?)),
            0x01D2 => Ok(Self::CMSG_RECLAIM_CORPSE(CMSG_RECLAIM_CORPSE::tokio_read_body(r, header_size).await?)),
            0x01D3 => Ok(Self::CMSG_WRAP_ITEM(CMSG_WRAP_ITEM::tokio_read_body(r, header_size).await?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(MSG_MINIMAP_PING_Client::tokio_read_body(r, header_size).await?)),
            0x01DC => Ok(Self::CMSG_PING(CMSG_PING::tokio_read_body(r, header_size).await?)),
            0x01E0 => Ok(Self::CMSG_SETSHEATHED(CMSG_SETSHEATHED::tokio_read_body(r, header_size).await?)),
            0x01ED => Ok(Self::CMSG_AUTH_SESSION(CMSG_AUTH_SESSION::tokio_read_body(r, header_size).await?)),
            0x01F0 => Ok(Self::CMSG_PET_CAST_SPELL(CMSG_PET_CAST_SPELL::tokio_read_body(r, header_size).await?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Client::tokio_read_body(r, header_size).await?)),
            0x01F4 => Ok(Self::CMSG_ZONEUPDATE(CMSG_ZONEUPDATE::tokio_read_body(r, header_size).await?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Client::tokio_read_body(r, header_size).await?)),
            0x01FF => Ok(Self::MSG_LOOKING_FOR_GROUP(MSG_LOOKING_FOR_GROUP_Client::tokio_read_body(r, header_size).await?)),
            0x0202 => Ok(Self::CMSG_UNLEARN_SKILL(CMSG_UNLEARN_SKILL::tokio_read_body(r, header_size).await?)),
            0x0205 => Ok(Self::CMSG_GMTICKET_CREATE(CMSG_GMTICKET_CREATE::tokio_read_body(r, header_size).await?)),
            0x0207 => Ok(Self::CMSG_GMTICKET_UPDATETEXT(CMSG_GMTICKET_UPDATETEXT::tokio_read_body(r, header_size).await?)),
            0x020A => Ok(Self::CMSG_REQUEST_ACCOUNT_DATA(CMSG_REQUEST_ACCOUNT_DATA::tokio_read_body(r, header_size).await?)),
            0x0211 => Ok(Self::CMSG_GMTICKET_GETTICKET(CMSG_GMTICKET_GETTICKET::tokio_read_body(r, header_size).await?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Client::tokio_read_body(r, header_size).await?)),
            0x0217 => Ok(Self::CMSG_GMTICKET_DELETETICKET(CMSG_GMTICKET_DELETETICKET::tokio_read_body(r, header_size).await?)),
            0x021A => Ok(Self::CMSG_GMTICKET_SYSTEMSTATUS(CMSG_GMTICKET_SYSTEMSTATUS::tokio_read_body(r, header_size).await?)),
            0x021C => Ok(Self::CMSG_SPIRIT_HEALER_ACTIVATE(CMSG_SPIRIT_HEALER_ACTIVATE::tokio_read_body(r, header_size).await?)),
            0x0225 => Ok(Self::CMSG_CHAT_IGNORED(CMSG_CHAT_IGNORED::tokio_read_body(r, header_size).await?)),
            0x0231 => Ok(Self::CMSG_GUILD_RANK(CMSG_GUILD_RANK::tokio_read_body(r, header_size).await?)),
            0x0232 => Ok(Self::CMSG_GUILD_ADD_RANK(CMSG_GUILD_ADD_RANK::tokio_read_body(r, header_size).await?)),
            0x0233 => Ok(Self::CMSG_GUILD_DEL_RANK(CMSG_GUILD_DEL_RANK::tokio_read_body(r, header_size).await?)),
            0x0234 => Ok(Self::CMSG_GUILD_SET_PUBLIC_NOTE(CMSG_GUILD_SET_PUBLIC_NOTE::tokio_read_body(r, header_size).await?)),
            0x0235 => Ok(Self::CMSG_GUILD_SET_OFFICER_NOTE(CMSG_GUILD_SET_OFFICER_NOTE::tokio_read_body(r, header_size).await?)),
            0x0238 => Ok(Self::CMSG_SEND_MAIL(CMSG_SEND_MAIL::tokio_read_body(r, header_size).await?)),
            0x023A => Ok(Self::CMSG_GET_MAIL_LIST(CMSG_GET_MAIL_LIST::tokio_read_body(r, header_size).await?)),
            0x023C => Ok(Self::CMSG_BATTLEFIELD_LIST(CMSG_BATTLEFIELD_LIST::tokio_read_body(r, header_size).await?)),
            0x023E => Ok(Self::CMSG_BATTLEFIELD_JOIN(CMSG_BATTLEFIELD_JOIN::tokio_read_body(r, header_size).await?)),
            0x0243 => Ok(Self::CMSG_ITEM_TEXT_QUERY(CMSG_ITEM_TEXT_QUERY::tokio_read_body(r, header_size).await?)),
            0x0245 => Ok(Self::CMSG_MAIL_TAKE_MONEY(CMSG_MAIL_TAKE_MONEY::tokio_read_body(r, header_size).await?)),
            0x0246 => Ok(Self::CMSG_MAIL_TAKE_ITEM(CMSG_MAIL_TAKE_ITEM::tokio_read_body(r, header_size).await?)),
            0x0247 => Ok(Self::CMSG_MAIL_MARK_AS_READ(CMSG_MAIL_MARK_AS_READ::tokio_read_body(r, header_size).await?)),
            0x0248 => Ok(Self::CMSG_MAIL_RETURN_TO_SENDER(CMSG_MAIL_RETURN_TO_SENDER::tokio_read_body(r, header_size).await?)),
            0x0249 => Ok(Self::CMSG_MAIL_DELETE(CMSG_MAIL_DELETE::tokio_read_body(r, header_size).await?)),
            0x024A => Ok(Self::CMSG_MAIL_CREATE_TEXT_ITEM(CMSG_MAIL_CREATE_TEXT_ITEM::tokio_read_body(r, header_size).await?)),
            0x0251 => Ok(Self::CMSG_LEARN_TALENT(CMSG_LEARN_TALENT::tokio_read_body(r, header_size).await?)),
            0x0253 => Ok(Self::CMSG_TOGGLE_PVP(CMSG_TOGGLE_PVP::tokio_read_body(r, header_size).await?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Client::tokio_read_body(r, header_size).await?)),
            0x0256 => Ok(Self::CMSG_AUCTION_SELL_ITEM(CMSG_AUCTION_SELL_ITEM::tokio_read_body(r, header_size).await?)),
            0x0257 => Ok(Self::CMSG_AUCTION_REMOVE_ITEM(CMSG_AUCTION_REMOVE_ITEM::tokio_read_body(r, header_size).await?)),
            0x0258 => Ok(Self::CMSG_AUCTION_LIST_ITEMS(CMSG_AUCTION_LIST_ITEMS::tokio_read_body(r, header_size).await?)),
            0x0259 => Ok(Self::CMSG_AUCTION_LIST_OWNER_ITEMS(CMSG_AUCTION_LIST_OWNER_ITEMS::tokio_read_body(r, header_size).await?)),
            0x025A => Ok(Self::CMSG_AUCTION_PLACE_BID(CMSG_AUCTION_PLACE_BID::tokio_read_body(r, header_size).await?)),
            0x0264 => Ok(Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(CMSG_AUCTION_LIST_BIDDER_ITEMS::tokio_read_body(r, header_size).await?)),
            0x0268 => Ok(Self::CMSG_SET_AMMO(CMSG_SET_AMMO::tokio_read_body(r, header_size).await?)),
            0x026A => Ok(Self::CMSG_SET_ACTIVE_MOVER(CMSG_SET_ACTIVE_MOVER::tokio_read_body(r, header_size).await?)),
            0x026B => Ok(Self::CMSG_PET_CANCEL_AURA(CMSG_PET_CANCEL_AURA::tokio_read_body(r, header_size).await?)),
            0x026D => Ok(Self::CMSG_CANCEL_AUTO_REPEAT_SPELL(CMSG_CANCEL_AUTO_REPEAT_SPELL::tokio_read_body(r, header_size).await?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Client::tokio_read_body(r, header_size).await?)),
            0x0270 => Ok(Self::CMSG_STABLE_PET(CMSG_STABLE_PET::tokio_read_body(r, header_size).await?)),
            0x0271 => Ok(Self::CMSG_UNSTABLE_PET(CMSG_UNSTABLE_PET::tokio_read_body(r, header_size).await?)),
            0x0272 => Ok(Self::CMSG_BUY_STABLE_SLOT(CMSG_BUY_STABLE_SLOT::tokio_read_body(r, header_size).await?)),
            0x0275 => Ok(Self::CMSG_STABLE_SWAP_PET(CMSG_STABLE_SWAP_PET::tokio_read_body(r, header_size).await?)),
            0x0279 => Ok(Self::CMSG_REQUEST_PET_INFO(CMSG_REQUEST_PET_INFO::tokio_read_body(r, header_size).await?)),
            0x027A => Ok(Self::CMSG_FAR_SIGHT(CMSG_FAR_SIGHT::tokio_read_body(r, header_size).await?)),
            0x027E => Ok(Self::CMSG_GROUP_CHANGE_SUB_GROUP(CMSG_GROUP_CHANGE_SUB_GROUP::tokio_read_body(r, header_size).await?)),
            0x027F => Ok(Self::CMSG_REQUEST_PARTY_MEMBER_STATS(CMSG_REQUEST_PARTY_MEMBER_STATS::tokio_read_body(r, header_size).await?)),
            0x0280 => Ok(Self::CMSG_GROUP_SWAP_SUB_GROUP(CMSG_GROUP_SWAP_SUB_GROUP::tokio_read_body(r, header_size).await?)),
            0x0282 => Ok(Self::CMSG_AUTOSTORE_BANK_ITEM(CMSG_AUTOSTORE_BANK_ITEM::tokio_read_body(r, header_size).await?)),
            0x0283 => Ok(Self::CMSG_AUTOBANK_ITEM(CMSG_AUTOBANK_ITEM::tokio_read_body(r, header_size).await?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Client::tokio_read_body(r, header_size).await?)),
            0x028E => Ok(Self::CMSG_GROUP_RAID_CONVERT(CMSG_GROUP_RAID_CONVERT::tokio_read_body(r, header_size).await?)),
            0x028F => Ok(Self::CMSG_GROUP_ASSISTANT_LEADER(CMSG_GROUP_ASSISTANT_LEADER::tokio_read_body(r, header_size).await?)),
            0x0290 => Ok(Self::CMSG_BUYBACK_ITEM(CMSG_BUYBACK_ITEM::tokio_read_body(r, header_size).await?)),
            0x0292 => Ok(Self::CMSG_MEETINGSTONE_JOIN(CMSG_MEETINGSTONE_JOIN::tokio_read_body(r, header_size).await?)),
            0x0293 => Ok(Self::CMSG_MEETINGSTONE_LEAVE(CMSG_MEETINGSTONE_LEAVE::tokio_read_body(r, header_size).await?)),
            0x0296 => Ok(Self::CMSG_MEETINGSTONE_INFO(CMSG_MEETINGSTONE_INFO::tokio_read_body(r, header_size).await?)),
            0x029B => Ok(Self::CMSG_CANCEL_GROWTH_AURA(CMSG_CANCEL_GROWTH_AURA::tokio_read_body(r, header_size).await?)),
            0x02A0 => Ok(Self::CMSG_LOOT_ROLL(CMSG_LOOT_ROLL::tokio_read_body(r, header_size).await?)),
            0x02A3 => Ok(Self::CMSG_LOOT_MASTER_GIVE(CMSG_LOOT_MASTER_GIVE::tokio_read_body(r, header_size).await?)),
            0x02A8 => Ok(Self::CMSG_REPAIR_ITEM(CMSG_REPAIR_ITEM::tokio_read_body(r, header_size).await?)),
            0x02AA => Ok(Self::MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Client::tokio_read_body(r, header_size).await?)),
            0x02AC => Ok(Self::CMSG_SUMMON_RESPONSE(CMSG_SUMMON_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x02B3 => Ok(Self::CMSG_SELF_RES(CMSG_SELF_RES::tokio_read_body(r, header_size).await?)),
            0x02B9 => Ok(Self::CMSG_TOGGLE_HELM(CMSG_TOGGLE_HELM::tokio_read_body(r, header_size).await?)),
            0x02BA => Ok(Self::CMSG_TOGGLE_CLOAK(CMSG_TOGGLE_CLOAK::tokio_read_body(r, header_size).await?)),
            0x02BF => Ok(Self::CMSG_SET_ACTIONBAR_TOGGLES(CMSG_SET_ACTIONBAR_TOGGLES::tokio_read_body(r, header_size).await?)),
            0x02C4 => Ok(Self::CMSG_ITEM_NAME_QUERY(CMSG_ITEM_NAME_QUERY::tokio_read_body(r, header_size).await?)),
            0x02C7 => Ok(Self::CMSG_CHAR_RENAME(CMSG_CHAR_RENAME::tokio_read_body(r, header_size).await?)),
            0x02C9 => Ok(Self::CMSG_MOVE_SPLINE_DONE(CMSG_MOVE_SPLINE_DONE::tokio_read_body(r, header_size).await?)),
            0x02CA => Ok(Self::CMSG_MOVE_FALL_RESET(CMSG_MOVE_FALL_RESET::tokio_read_body(r, header_size).await?)),
            0x02CD => Ok(Self::CMSG_REQUEST_RAID_INFO(CMSG_REQUEST_RAID_INFO::tokio_read_body(r, header_size).await?)),
            0x02CE => Ok(Self::CMSG_MOVE_TIME_SKIPPED(CMSG_MOVE_TIME_SKIPPED::tokio_read_body(r, header_size).await?)),
            0x02CF => Ok(Self::CMSG_MOVE_FEATHER_FALL_ACK(CMSG_MOVE_FEATHER_FALL_ACK::tokio_read_body(r, header_size).await?)),
            0x02D0 => Ok(Self::CMSG_MOVE_WATER_WALK_ACK(CMSG_MOVE_WATER_WALK_ACK::tokio_read_body(r, header_size).await?)),
            0x02D1 => Ok(Self::CMSG_MOVE_NOT_ACTIVE_MOVER(CMSG_MOVE_NOT_ACTIVE_MOVER::tokio_read_body(r, header_size).await?)),
            0x02D3 => Ok(Self::CMSG_BATTLEFIELD_STATUS(CMSG_BATTLEFIELD_STATUS::tokio_read_body(r, header_size).await?)),
            0x02D5 => Ok(Self::CMSG_BATTLEFIELD_PORT(CMSG_BATTLEFIELD_PORT::tokio_read_body(r, header_size).await?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Client::tokio_read_body(r, header_size).await?)),
            0x02D7 => Ok(Self::CMSG_BATTLEMASTER_HELLO(CMSG_BATTLEMASTER_HELLO::tokio_read_body(r, header_size).await?)),
            0x02DB => Ok(Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(CMSG_FORCE_WALK_SPEED_CHANGE_ACK::tokio_read_body(r, header_size).await?)),
            0x02DD => Ok(Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK::tokio_read_body(r, header_size).await?)),
            0x02DF => Ok(Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(CMSG_FORCE_TURN_RATE_CHANGE_ACK::tokio_read_body(r, header_size).await?)),
            0x02E0 => Ok(Self::MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_Client::tokio_read_body(r, header_size).await?)),
            0x02E1 => Ok(Self::CMSG_LEAVE_BATTLEFIELD(CMSG_LEAVE_BATTLEFIELD::tokio_read_body(r, header_size).await?)),
            0x02E2 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUERY(CMSG_AREA_SPIRIT_HEALER_QUERY::tokio_read_body(r, header_size).await?)),
            0x02E3 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(CMSG_AREA_SPIRIT_HEALER_QUEUE::tokio_read_body(r, header_size).await?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Client::tokio_read_body(r, header_size).await?)),
            0x02EA => Ok(Self::CMSG_PET_STOP_ATTACK(CMSG_PET_STOP_ATTACK::tokio_read_body(r, header_size).await?)),
            0x02EE => Ok(Self::CMSG_BATTLEMASTER_JOIN(CMSG_BATTLEMASTER_JOIN::tokio_read_body(r, header_size).await?)),
            0x02F0 => Ok(Self::CMSG_PET_UNLEARN(CMSG_PET_UNLEARN::tokio_read_body(r, header_size).await?)),
            0x02F3 => Ok(Self::CMSG_PET_SPELL_AUTOCAST(CMSG_PET_SPELL_AUTOCAST::tokio_read_body(r, header_size).await?)),
            0x02FC => Ok(Self::CMSG_GUILD_INFO_TEXT(CMSG_GUILD_INFO_TEXT::tokio_read_body(r, header_size).await?)),
            0x0312 => Ok(Self::CMSG_ACTIVATETAXIEXPRESS(CMSG_ACTIVATETAXIEXPRESS::tokio_read_body(r, header_size).await?)),
            0x0317 => Ok(Self::CMSG_SET_FACTION_INACTIVE(CMSG_SET_FACTION_INACTIVE::tokio_read_body(r, header_size).await?)),
            0x0318 => Ok(Self::CMSG_SET_WATCHED_FACTION(CMSG_SET_WATCHED_FACTION::tokio_read_body(r, header_size).await?)),
            0x031D => Ok(Self::CMSG_RESET_INSTANCES(CMSG_RESET_INSTANCES::tokio_read_body(r, header_size).await?)),
            0x0321 => Ok(Self::MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Client::tokio_read_body(r, header_size).await?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Client::tokio_read_body(r, header_size).await?)),
            0x032A => Ok(Self::CMSG_GMSURVEY_SUBMIT(CMSG_GMSURVEY_SUBMIT::tokio_read_body(r, header_size).await?)),
            _ => Err(ClientOpcodeMessageError::InvalidOpcode(header.opcode)),
        }
    }


    #[cfg(feature = "async-std")]
    pub async fn astd_read_unencrypted<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let size = (crate::util::astd_read_u16_be(r).await? - 4) as u32;
        let opcode = crate::util::astd_read_u32_le(r).await?;
        match opcode {
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD::astd_read_body(r, size).await?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD::astd_read_body(r, size).await?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(MSG_MOVE_STOP::astd_read_body(r, size).await?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT::astd_read_body(r, size).await?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT::astd_read_body(r, size).await?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE::astd_read_body(r, size).await?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(MSG_MOVE_JUMP::astd_read_body(r, size).await?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT::astd_read_body(r, size).await?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT::astd_read_body(r, size).await?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN::astd_read_body(r, size).await?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP::astd_read_body(r, size).await?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN::astd_read_body(r, size).await?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH::astd_read_body(r, size).await?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE::astd_read_body(r, size).await?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE::astd_read_body(r, size).await?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK::astd_read_body(r, size).await?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND::astd_read_body(r, size).await?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM::astd_read_body(r, size).await?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM::astd_read_body(r, size).await?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING::astd_read_body(r, size).await?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH::astd_read_body(r, size).await?)),
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK::astd_read_body(r, size).await?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT::astd_read_body(r, size).await?)),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(MSG_PETITION_DECLINE::astd_read_body(r, size).await?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE::astd_read_body(r, size).await?)),
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT::astd_read_body(r, size).await?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(MSG_PETITION_RENAME::astd_read_body(r, size).await?)),
            0x0008 => Ok(Self::CMSG_WORLD_TELEPORT(CMSG_WORLD_TELEPORT::astd_read_body(r, size).await?)),
            0x0036 => Ok(Self::CMSG_CHAR_CREATE(CMSG_CHAR_CREATE::astd_read_body(r, size).await?)),
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(CMSG_CHAR_ENUM::astd_read_body(r, size).await?)),
            0x0038 => Ok(Self::CMSG_CHAR_DELETE(CMSG_CHAR_DELETE::astd_read_body(r, size).await?)),
            0x003D => Ok(Self::CMSG_PLAYER_LOGIN(CMSG_PLAYER_LOGIN::astd_read_body(r, size).await?)),
            0x004A => Ok(Self::CMSG_PLAYER_LOGOUT(CMSG_PLAYER_LOGOUT::astd_read_body(r, size).await?)),
            0x004B => Ok(Self::CMSG_LOGOUT_REQUEST(CMSG_LOGOUT_REQUEST::astd_read_body(r, size).await?)),
            0x004E => Ok(Self::CMSG_LOGOUT_CANCEL(CMSG_LOGOUT_CANCEL::astd_read_body(r, size).await?)),
            0x0050 => Ok(Self::CMSG_NAME_QUERY(CMSG_NAME_QUERY::astd_read_body(r, size).await?)),
            0x0052 => Ok(Self::CMSG_PET_NAME_QUERY(CMSG_PET_NAME_QUERY::astd_read_body(r, size).await?)),
            0x0054 => Ok(Self::CMSG_GUILD_QUERY(CMSG_GUILD_QUERY::astd_read_body(r, size).await?)),
            0x0056 => Ok(Self::CMSG_ITEM_QUERY_SINGLE(CMSG_ITEM_QUERY_SINGLE::astd_read_body(r, size).await?)),
            0x005A => Ok(Self::CMSG_PAGE_TEXT_QUERY(CMSG_PAGE_TEXT_QUERY::astd_read_body(r, size).await?)),
            0x005C => Ok(Self::CMSG_QUEST_QUERY(CMSG_QUEST_QUERY::astd_read_body(r, size).await?)),
            0x005E => Ok(Self::CMSG_GAMEOBJECT_QUERY(CMSG_GAMEOBJECT_QUERY::astd_read_body(r, size).await?)),
            0x0060 => Ok(Self::CMSG_CREATURE_QUERY(CMSG_CREATURE_QUERY::astd_read_body(r, size).await?)),
            0x0062 => Ok(Self::CMSG_WHO(CMSG_WHO::astd_read_body(r, size).await?)),
            0x0064 => Ok(Self::CMSG_WHOIS(CMSG_WHOIS::astd_read_body(r, size).await?)),
            0x0066 => Ok(Self::CMSG_FRIEND_LIST(CMSG_FRIEND_LIST::astd_read_body(r, size).await?)),
            0x0069 => Ok(Self::CMSG_ADD_FRIEND(CMSG_ADD_FRIEND::astd_read_body(r, size).await?)),
            0x006A => Ok(Self::CMSG_DEL_FRIEND(CMSG_DEL_FRIEND::astd_read_body(r, size).await?)),
            0x006C => Ok(Self::CMSG_ADD_IGNORE(CMSG_ADD_IGNORE::astd_read_body(r, size).await?)),
            0x006D => Ok(Self::CMSG_DEL_IGNORE(CMSG_DEL_IGNORE::astd_read_body(r, size).await?)),
            0x006E => Ok(Self::CMSG_GROUP_INVITE(CMSG_GROUP_INVITE::astd_read_body(r, size).await?)),
            0x0072 => Ok(Self::CMSG_GROUP_ACCEPT(CMSG_GROUP_ACCEPT::astd_read_body(r, size).await?)),
            0x0073 => Ok(Self::CMSG_GROUP_DECLINE(CMSG_GROUP_DECLINE::astd_read_body(r, size).await?)),
            0x0075 => Ok(Self::CMSG_GROUP_UNINVITE(CMSG_GROUP_UNINVITE::astd_read_body(r, size).await?)),
            0x0076 => Ok(Self::CMSG_GROUP_UNINVITE_GUID(CMSG_GROUP_UNINVITE_GUID::astd_read_body(r, size).await?)),
            0x0078 => Ok(Self::CMSG_GROUP_SET_LEADER(CMSG_GROUP_SET_LEADER::astd_read_body(r, size).await?)),
            0x007A => Ok(Self::CMSG_LOOT_METHOD(CMSG_LOOT_METHOD::astd_read_body(r, size).await?)),
            0x007B => Ok(Self::CMSG_GROUP_DISBAND(CMSG_GROUP_DISBAND::astd_read_body(r, size).await?)),
            0x0081 => Ok(Self::CMSG_GUILD_CREATE(CMSG_GUILD_CREATE::astd_read_body(r, size).await?)),
            0x0082 => Ok(Self::CMSG_GUILD_INVITE(CMSG_GUILD_INVITE::astd_read_body(r, size).await?)),
            0x0084 => Ok(Self::CMSG_GUILD_ACCEPT(CMSG_GUILD_ACCEPT::astd_read_body(r, size).await?)),
            0x0085 => Ok(Self::CMSG_GUILD_DECLINE(CMSG_GUILD_DECLINE::astd_read_body(r, size).await?)),
            0x0087 => Ok(Self::CMSG_GUILD_INFO(CMSG_GUILD_INFO::astd_read_body(r, size).await?)),
            0x0089 => Ok(Self::CMSG_GUILD_ROSTER(CMSG_GUILD_ROSTER::astd_read_body(r, size).await?)),
            0x008B => Ok(Self::CMSG_GUILD_PROMOTE(CMSG_GUILD_PROMOTE::astd_read_body(r, size).await?)),
            0x008C => Ok(Self::CMSG_GUILD_DEMOTE(CMSG_GUILD_DEMOTE::astd_read_body(r, size).await?)),
            0x008D => Ok(Self::CMSG_GUILD_LEAVE(CMSG_GUILD_LEAVE::astd_read_body(r, size).await?)),
            0x008E => Ok(Self::CMSG_GUILD_REMOVE(CMSG_GUILD_REMOVE::astd_read_body(r, size).await?)),
            0x008F => Ok(Self::CMSG_GUILD_DISBAND(CMSG_GUILD_DISBAND::astd_read_body(r, size).await?)),
            0x0090 => Ok(Self::CMSG_GUILD_LEADER(CMSG_GUILD_LEADER::astd_read_body(r, size).await?)),
            0x0091 => Ok(Self::CMSG_GUILD_MOTD(CMSG_GUILD_MOTD::astd_read_body(r, size).await?)),
            0x0095 => Ok(Self::CMSG_MESSAGECHAT(CMSG_MESSAGECHAT::astd_read_body(r, size).await?)),
            0x0097 => Ok(Self::CMSG_JOIN_CHANNEL(CMSG_JOIN_CHANNEL::astd_read_body(r, size).await?)),
            0x0098 => Ok(Self::CMSG_LEAVE_CHANNEL(CMSG_LEAVE_CHANNEL::astd_read_body(r, size).await?)),
            0x009A => Ok(Self::CMSG_CHANNEL_LIST(CMSG_CHANNEL_LIST::astd_read_body(r, size).await?)),
            0x009C => Ok(Self::CMSG_CHANNEL_PASSWORD(CMSG_CHANNEL_PASSWORD::astd_read_body(r, size).await?)),
            0x009D => Ok(Self::CMSG_CHANNEL_SET_OWNER(CMSG_CHANNEL_SET_OWNER::astd_read_body(r, size).await?)),
            0x009E => Ok(Self::CMSG_CHANNEL_OWNER(CMSG_CHANNEL_OWNER::astd_read_body(r, size).await?)),
            0x009F => Ok(Self::CMSG_CHANNEL_MODERATOR(CMSG_CHANNEL_MODERATOR::astd_read_body(r, size).await?)),
            0x00A0 => Ok(Self::CMSG_CHANNEL_UNMODERATOR(CMSG_CHANNEL_UNMODERATOR::astd_read_body(r, size).await?)),
            0x00A1 => Ok(Self::CMSG_CHANNEL_MUTE(CMSG_CHANNEL_MUTE::astd_read_body(r, size).await?)),
            0x00A2 => Ok(Self::CMSG_CHANNEL_UNMUTE(CMSG_CHANNEL_UNMUTE::astd_read_body(r, size).await?)),
            0x00A3 => Ok(Self::CMSG_CHANNEL_INVITE(CMSG_CHANNEL_INVITE::astd_read_body(r, size).await?)),
            0x00A4 => Ok(Self::CMSG_CHANNEL_KICK(CMSG_CHANNEL_KICK::astd_read_body(r, size).await?)),
            0x00A5 => Ok(Self::CMSG_CHANNEL_BAN(CMSG_CHANNEL_BAN::astd_read_body(r, size).await?)),
            0x00A6 => Ok(Self::CMSG_CHANNEL_UNBAN(CMSG_CHANNEL_UNBAN::astd_read_body(r, size).await?)),
            0x00A7 => Ok(Self::CMSG_CHANNEL_ANNOUNCEMENTS(CMSG_CHANNEL_ANNOUNCEMENTS::astd_read_body(r, size).await?)),
            0x00A8 => Ok(Self::CMSG_CHANNEL_MODERATE(CMSG_CHANNEL_MODERATE::astd_read_body(r, size).await?)),
            0x00A9 => Ok(Self::SMSG_UPDATE_OBJECT(SMSG_UPDATE_OBJECT::astd_read_body(r, size).await?)),
            0x00AB => Ok(Self::CMSG_USE_ITEM(CMSG_USE_ITEM::astd_read_body(r, size).await?)),
            0x00AC => Ok(Self::CMSG_OPEN_ITEM(CMSG_OPEN_ITEM::astd_read_body(r, size).await?)),
            0x00AD => Ok(Self::CMSG_READ_ITEM(CMSG_READ_ITEM::astd_read_body(r, size).await?)),
            0x00B1 => Ok(Self::CMSG_GAMEOBJ_USE(CMSG_GAMEOBJ_USE::astd_read_body(r, size).await?)),
            0x00B4 => Ok(Self::CMSG_AREATRIGGER(CMSG_AREATRIGGER::astd_read_body(r, size).await?)),
            0x00E1 => Ok(Self::CMSG_MOVE_SET_RAW_POSITION(CMSG_MOVE_SET_RAW_POSITION::astd_read_body(r, size).await?)),
            0x00E3 => Ok(Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_SPEED_CHANGE_ACK::astd_read_body(r, size).await?)),
            0x00E5 => Ok(Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK::astd_read_body(r, size).await?)),
            0x00E7 => Ok(Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_SPEED_CHANGE_ACK::astd_read_body(r, size).await?)),
            0x00E9 => Ok(Self::CMSG_FORCE_MOVE_ROOT_ACK(CMSG_FORCE_MOVE_ROOT_ACK::astd_read_body(r, size).await?)),
            0x00EB => Ok(Self::CMSG_FORCE_MOVE_UNROOT_ACK(CMSG_FORCE_MOVE_UNROOT_ACK::astd_read_body(r, size).await?)),
            0x00F0 => Ok(Self::CMSG_MOVE_KNOCK_BACK_ACK(CMSG_MOVE_KNOCK_BACK_ACK::astd_read_body(r, size).await?)),
            0x00F6 => Ok(Self::CMSG_MOVE_HOVER_ACK(CMSG_MOVE_HOVER_ACK::astd_read_body(r, size).await?)),
            0x00FB => Ok(Self::CMSG_NEXT_CINEMATIC_CAMERA(CMSG_NEXT_CINEMATIC_CAMERA::astd_read_body(r, size).await?)),
            0x00FC => Ok(Self::CMSG_COMPLETE_CINEMATIC(CMSG_COMPLETE_CINEMATIC::astd_read_body(r, size).await?)),
            0x00FE => Ok(Self::CMSG_TUTORIAL_FLAG(CMSG_TUTORIAL_FLAG::astd_read_body(r, size).await?)),
            0x00FF => Ok(Self::CMSG_TUTORIAL_CLEAR(CMSG_TUTORIAL_CLEAR::astd_read_body(r, size).await?)),
            0x0100 => Ok(Self::CMSG_TUTORIAL_RESET(CMSG_TUTORIAL_RESET::astd_read_body(r, size).await?)),
            0x0101 => Ok(Self::CMSG_STANDSTATECHANGE(CMSG_STANDSTATECHANGE::astd_read_body(r, size).await?)),
            0x0102 => Ok(Self::CMSG_EMOTE(CMSG_EMOTE::astd_read_body(r, size).await?)),
            0x0104 => Ok(Self::CMSG_TEXT_EMOTE(CMSG_TEXT_EMOTE::astd_read_body(r, size).await?)),
            0x0108 => Ok(Self::CMSG_AUTOSTORE_LOOT_ITEM(CMSG_AUTOSTORE_LOOT_ITEM::astd_read_body(r, size).await?)),
            0x010A => Ok(Self::CMSG_AUTOEQUIP_ITEM(CMSG_AUTOEQUIP_ITEM::astd_read_body(r, size).await?)),
            0x010B => Ok(Self::CMSG_AUTOSTORE_BAG_ITEM(CMSG_AUTOSTORE_BAG_ITEM::astd_read_body(r, size).await?)),
            0x010C => Ok(Self::CMSG_SWAP_ITEM(CMSG_SWAP_ITEM::astd_read_body(r, size).await?)),
            0x010D => Ok(Self::CMSG_SWAP_INV_ITEM(CMSG_SWAP_INV_ITEM::astd_read_body(r, size).await?)),
            0x010E => Ok(Self::CMSG_SPLIT_ITEM(CMSG_SPLIT_ITEM::astd_read_body(r, size).await?)),
            0x010F => Ok(Self::CMSG_AUTOEQUIP_ITEM_SLOT(CMSG_AUTOEQUIP_ITEM_SLOT::astd_read_body(r, size).await?)),
            0x0111 => Ok(Self::CMSG_DESTROYITEM(CMSG_DESTROYITEM::astd_read_body(r, size).await?)),
            0x0114 => Ok(Self::CMSG_INSPECT(CMSG_INSPECT::astd_read_body(r, size).await?)),
            0x0116 => Ok(Self::CMSG_INITIATE_TRADE(CMSG_INITIATE_TRADE::astd_read_body(r, size).await?)),
            0x0117 => Ok(Self::CMSG_BEGIN_TRADE(CMSG_BEGIN_TRADE::astd_read_body(r, size).await?)),
            0x0118 => Ok(Self::CMSG_BUSY_TRADE(CMSG_BUSY_TRADE::astd_read_body(r, size).await?)),
            0x0119 => Ok(Self::CMSG_IGNORE_TRADE(CMSG_IGNORE_TRADE::astd_read_body(r, size).await?)),
            0x011A => Ok(Self::CMSG_ACCEPT_TRADE(CMSG_ACCEPT_TRADE::astd_read_body(r, size).await?)),
            0x011B => Ok(Self::CMSG_UNACCEPT_TRADE(CMSG_UNACCEPT_TRADE::astd_read_body(r, size).await?)),
            0x011C => Ok(Self::CMSG_CANCEL_TRADE(CMSG_CANCEL_TRADE::astd_read_body(r, size).await?)),
            0x011D => Ok(Self::CMSG_SET_TRADE_ITEM(CMSG_SET_TRADE_ITEM::astd_read_body(r, size).await?)),
            0x011E => Ok(Self::CMSG_CLEAR_TRADE_ITEM(CMSG_CLEAR_TRADE_ITEM::astd_read_body(r, size).await?)),
            0x011F => Ok(Self::CMSG_SET_TRADE_GOLD(CMSG_SET_TRADE_GOLD::astd_read_body(r, size).await?)),
            0x0125 => Ok(Self::CMSG_SET_FACTION_ATWAR(CMSG_SET_FACTION_ATWAR::astd_read_body(r, size).await?)),
            0x0128 => Ok(Self::CMSG_SET_ACTION_BUTTON(CMSG_SET_ACTION_BUTTON::astd_read_body(r, size).await?)),
            0x012E => Ok(Self::CMSG_CAST_SPELL(CMSG_CAST_SPELL::astd_read_body(r, size).await?)),
            0x012F => Ok(Self::CMSG_CANCEL_CAST(CMSG_CANCEL_CAST::astd_read_body(r, size).await?)),
            0x0136 => Ok(Self::CMSG_CANCEL_AURA(CMSG_CANCEL_AURA::astd_read_body(r, size).await?)),
            0x013B => Ok(Self::CMSG_CANCEL_CHANNELLING(CMSG_CANCEL_CHANNELLING::astd_read_body(r, size).await?)),
            0x013D => Ok(Self::CMSG_SET_SELECTION(CMSG_SET_SELECTION::astd_read_body(r, size).await?)),
            0x013E => Ok(Self::CMSG_SET_TARGET_OBSOLETE(CMSG_SET_TARGET_OBSOLETE::astd_read_body(r, size).await?)),
            0x0141 => Ok(Self::CMSG_ATTACKSWING(CMSG_ATTACKSWING::astd_read_body(r, size).await?)),
            0x0142 => Ok(Self::CMSG_ATTACKSTOP(CMSG_ATTACKSTOP::astd_read_body(r, size).await?)),
            0x015A => Ok(Self::CMSG_REPOP_REQUEST(CMSG_REPOP_REQUEST::astd_read_body(r, size).await?)),
            0x015C => Ok(Self::CMSG_RESURRECT_RESPONSE(CMSG_RESURRECT_RESPONSE::astd_read_body(r, size).await?)),
            0x015D => Ok(Self::CMSG_LOOT(CMSG_LOOT::astd_read_body(r, size).await?)),
            0x015E => Ok(Self::CMSG_LOOT_MONEY(CMSG_LOOT_MONEY::astd_read_body(r, size).await?)),
            0x015F => Ok(Self::CMSG_LOOT_RELEASE(CMSG_LOOT_RELEASE::astd_read_body(r, size).await?)),
            0x016C => Ok(Self::CMSG_DUEL_ACCEPTED(CMSG_DUEL_ACCEPTED::astd_read_body(r, size).await?)),
            0x016D => Ok(Self::CMSG_DUEL_CANCELLED(CMSG_DUEL_CANCELLED::astd_read_body(r, size).await?)),
            0x0171 => Ok(Self::CMSG_MOUNTSPECIAL_ANIM(CMSG_MOUNTSPECIAL_ANIM::astd_read_body(r, size).await?)),
            0x0174 => Ok(Self::CMSG_PET_SET_ACTION(CMSG_PET_SET_ACTION::astd_read_body(r, size).await?)),
            0x0175 => Ok(Self::CMSG_PET_ACTION(CMSG_PET_ACTION::astd_read_body(r, size).await?)),
            0x0176 => Ok(Self::CMSG_PET_ABANDON(CMSG_PET_ABANDON::astd_read_body(r, size).await?)),
            0x0177 => Ok(Self::CMSG_PET_RENAME(CMSG_PET_RENAME::astd_read_body(r, size).await?)),
            0x017B => Ok(Self::CMSG_GOSSIP_HELLO(CMSG_GOSSIP_HELLO::astd_read_body(r, size).await?)),
            0x017C => Ok(Self::CMSG_GOSSIP_SELECT_OPTION(CMSG_GOSSIP_SELECT_OPTION::astd_read_body(r, size).await?)),
            0x017F => Ok(Self::CMSG_NPC_TEXT_QUERY(CMSG_NPC_TEXT_QUERY::astd_read_body(r, size).await?)),
            0x0182 => Ok(Self::CMSG_QUESTGIVER_STATUS_QUERY(CMSG_QUESTGIVER_STATUS_QUERY::astd_read_body(r, size).await?)),
            0x0184 => Ok(Self::CMSG_QUESTGIVER_HELLO(CMSG_QUESTGIVER_HELLO::astd_read_body(r, size).await?)),
            0x0186 => Ok(Self::CMSG_QUESTGIVER_QUERY_QUEST(CMSG_QUESTGIVER_QUERY_QUEST::astd_read_body(r, size).await?)),
            0x0187 => Ok(Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(CMSG_QUESTGIVER_QUEST_AUTOLAUNCH::astd_read_body(r, size).await?)),
            0x0189 => Ok(Self::CMSG_QUESTGIVER_ACCEPT_QUEST(CMSG_QUESTGIVER_ACCEPT_QUEST::astd_read_body(r, size).await?)),
            0x018A => Ok(Self::CMSG_QUESTGIVER_COMPLETE_QUEST(CMSG_QUESTGIVER_COMPLETE_QUEST::astd_read_body(r, size).await?)),
            0x018C => Ok(Self::CMSG_QUESTGIVER_REQUEST_REWARD(CMSG_QUESTGIVER_REQUEST_REWARD::astd_read_body(r, size).await?)),
            0x018E => Ok(Self::CMSG_QUESTGIVER_CHOOSE_REWARD(CMSG_QUESTGIVER_CHOOSE_REWARD::astd_read_body(r, size).await?)),
            0x0190 => Ok(Self::CMSG_QUESTGIVER_CANCEL(CMSG_QUESTGIVER_CANCEL::astd_read_body(r, size).await?)),
            0x0193 => Ok(Self::CMSG_QUESTLOG_SWAP_QUEST(CMSG_QUESTLOG_SWAP_QUEST::astd_read_body(r, size).await?)),
            0x0194 => Ok(Self::CMSG_QUESTLOG_REMOVE_QUEST(CMSG_QUESTLOG_REMOVE_QUEST::astd_read_body(r, size).await?)),
            0x019B => Ok(Self::CMSG_QUEST_CONFIRM_ACCEPT(CMSG_QUEST_CONFIRM_ACCEPT::astd_read_body(r, size).await?)),
            0x019D => Ok(Self::CMSG_PUSHQUESTTOPARTY(CMSG_PUSHQUESTTOPARTY::astd_read_body(r, size).await?)),
            0x019E => Ok(Self::CMSG_LIST_INVENTORY(CMSG_LIST_INVENTORY::astd_read_body(r, size).await?)),
            0x01A0 => Ok(Self::CMSG_SELL_ITEM(CMSG_SELL_ITEM::astd_read_body(r, size).await?)),
            0x01A2 => Ok(Self::CMSG_BUY_ITEM(CMSG_BUY_ITEM::astd_read_body(r, size).await?)),
            0x01A3 => Ok(Self::CMSG_BUY_ITEM_IN_SLOT(CMSG_BUY_ITEM_IN_SLOT::astd_read_body(r, size).await?)),
            0x01AA => Ok(Self::CMSG_TAXINODE_STATUS_QUERY(CMSG_TAXINODE_STATUS_QUERY::astd_read_body(r, size).await?)),
            0x01AC => Ok(Self::CMSG_TAXIQUERYAVAILABLENODES(CMSG_TAXIQUERYAVAILABLENODES::astd_read_body(r, size).await?)),
            0x01AD => Ok(Self::CMSG_ACTIVATETAXI(CMSG_ACTIVATETAXI::astd_read_body(r, size).await?)),
            0x01B0 => Ok(Self::CMSG_TRAINER_LIST(CMSG_TRAINER_LIST::astd_read_body(r, size).await?)),
            0x01B2 => Ok(Self::CMSG_TRAINER_BUY_SPELL(CMSG_TRAINER_BUY_SPELL::astd_read_body(r, size).await?)),
            0x01B5 => Ok(Self::CMSG_BINDER_ACTIVATE(CMSG_BINDER_ACTIVATE::astd_read_body(r, size).await?)),
            0x01B7 => Ok(Self::CMSG_BANKER_ACTIVATE(CMSG_BANKER_ACTIVATE::astd_read_body(r, size).await?)),
            0x01B9 => Ok(Self::CMSG_BUY_BANK_SLOT(CMSG_BUY_BANK_SLOT::astd_read_body(r, size).await?)),
            0x01BB => Ok(Self::CMSG_PETITION_SHOWLIST(CMSG_PETITION_SHOWLIST::astd_read_body(r, size).await?)),
            0x01BD => Ok(Self::CMSG_PETITION_BUY(CMSG_PETITION_BUY::astd_read_body(r, size).await?)),
            0x01BE => Ok(Self::CMSG_PETITION_SHOW_SIGNATURES(CMSG_PETITION_SHOW_SIGNATURES::astd_read_body(r, size).await?)),
            0x01C0 => Ok(Self::CMSG_PETITION_SIGN(CMSG_PETITION_SIGN::astd_read_body(r, size).await?)),
            0x01C3 => Ok(Self::CMSG_OFFER_PETITION(CMSG_OFFER_PETITION::astd_read_body(r, size).await?)),
            0x01C4 => Ok(Self::CMSG_TURN_IN_PETITION(CMSG_TURN_IN_PETITION::astd_read_body(r, size).await?)),
            0x01C6 => Ok(Self::CMSG_PETITION_QUERY(CMSG_PETITION_QUERY::astd_read_body(r, size).await?)),
            0x01CA => Ok(Self::CMSG_BUG(CMSG_BUG::astd_read_body(r, size).await?)),
            0x01CC => Ok(Self::CMSG_PLAYED_TIME(CMSG_PLAYED_TIME::astd_read_body(r, size).await?)),
            0x01CE => Ok(Self::CMSG_QUERY_TIME(CMSG_QUERY_TIME::astd_read_body(r, size).await?)),
            0x01D2 => Ok(Self::CMSG_RECLAIM_CORPSE(CMSG_RECLAIM_CORPSE::astd_read_body(r, size).await?)),
            0x01D3 => Ok(Self::CMSG_WRAP_ITEM(CMSG_WRAP_ITEM::astd_read_body(r, size).await?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(MSG_MINIMAP_PING_Client::astd_read_body(r, size).await?)),
            0x01DC => Ok(Self::CMSG_PING(CMSG_PING::astd_read_body(r, size).await?)),
            0x01E0 => Ok(Self::CMSG_SETSHEATHED(CMSG_SETSHEATHED::astd_read_body(r, size).await?)),
            0x01ED => Ok(Self::CMSG_AUTH_SESSION(CMSG_AUTH_SESSION::astd_read_body(r, size).await?)),
            0x01F0 => Ok(Self::CMSG_PET_CAST_SPELL(CMSG_PET_CAST_SPELL::astd_read_body(r, size).await?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Client::astd_read_body(r, size).await?)),
            0x01F4 => Ok(Self::CMSG_ZONEUPDATE(CMSG_ZONEUPDATE::astd_read_body(r, size).await?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Client::astd_read_body(r, size).await?)),
            0x01FF => Ok(Self::MSG_LOOKING_FOR_GROUP(MSG_LOOKING_FOR_GROUP_Client::astd_read_body(r, size).await?)),
            0x0202 => Ok(Self::CMSG_UNLEARN_SKILL(CMSG_UNLEARN_SKILL::astd_read_body(r, size).await?)),
            0x0205 => Ok(Self::CMSG_GMTICKET_CREATE(CMSG_GMTICKET_CREATE::astd_read_body(r, size).await?)),
            0x0207 => Ok(Self::CMSG_GMTICKET_UPDATETEXT(CMSG_GMTICKET_UPDATETEXT::astd_read_body(r, size).await?)),
            0x020A => Ok(Self::CMSG_REQUEST_ACCOUNT_DATA(CMSG_REQUEST_ACCOUNT_DATA::astd_read_body(r, size).await?)),
            0x0211 => Ok(Self::CMSG_GMTICKET_GETTICKET(CMSG_GMTICKET_GETTICKET::astd_read_body(r, size).await?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Client::astd_read_body(r, size).await?)),
            0x0217 => Ok(Self::CMSG_GMTICKET_DELETETICKET(CMSG_GMTICKET_DELETETICKET::astd_read_body(r, size).await?)),
            0x021A => Ok(Self::CMSG_GMTICKET_SYSTEMSTATUS(CMSG_GMTICKET_SYSTEMSTATUS::astd_read_body(r, size).await?)),
            0x021C => Ok(Self::CMSG_SPIRIT_HEALER_ACTIVATE(CMSG_SPIRIT_HEALER_ACTIVATE::astd_read_body(r, size).await?)),
            0x0225 => Ok(Self::CMSG_CHAT_IGNORED(CMSG_CHAT_IGNORED::astd_read_body(r, size).await?)),
            0x0231 => Ok(Self::CMSG_GUILD_RANK(CMSG_GUILD_RANK::astd_read_body(r, size).await?)),
            0x0232 => Ok(Self::CMSG_GUILD_ADD_RANK(CMSG_GUILD_ADD_RANK::astd_read_body(r, size).await?)),
            0x0233 => Ok(Self::CMSG_GUILD_DEL_RANK(CMSG_GUILD_DEL_RANK::astd_read_body(r, size).await?)),
            0x0234 => Ok(Self::CMSG_GUILD_SET_PUBLIC_NOTE(CMSG_GUILD_SET_PUBLIC_NOTE::astd_read_body(r, size).await?)),
            0x0235 => Ok(Self::CMSG_GUILD_SET_OFFICER_NOTE(CMSG_GUILD_SET_OFFICER_NOTE::astd_read_body(r, size).await?)),
            0x0238 => Ok(Self::CMSG_SEND_MAIL(CMSG_SEND_MAIL::astd_read_body(r, size).await?)),
            0x023A => Ok(Self::CMSG_GET_MAIL_LIST(CMSG_GET_MAIL_LIST::astd_read_body(r, size).await?)),
            0x023C => Ok(Self::CMSG_BATTLEFIELD_LIST(CMSG_BATTLEFIELD_LIST::astd_read_body(r, size).await?)),
            0x023E => Ok(Self::CMSG_BATTLEFIELD_JOIN(CMSG_BATTLEFIELD_JOIN::astd_read_body(r, size).await?)),
            0x0243 => Ok(Self::CMSG_ITEM_TEXT_QUERY(CMSG_ITEM_TEXT_QUERY::astd_read_body(r, size).await?)),
            0x0245 => Ok(Self::CMSG_MAIL_TAKE_MONEY(CMSG_MAIL_TAKE_MONEY::astd_read_body(r, size).await?)),
            0x0246 => Ok(Self::CMSG_MAIL_TAKE_ITEM(CMSG_MAIL_TAKE_ITEM::astd_read_body(r, size).await?)),
            0x0247 => Ok(Self::CMSG_MAIL_MARK_AS_READ(CMSG_MAIL_MARK_AS_READ::astd_read_body(r, size).await?)),
            0x0248 => Ok(Self::CMSG_MAIL_RETURN_TO_SENDER(CMSG_MAIL_RETURN_TO_SENDER::astd_read_body(r, size).await?)),
            0x0249 => Ok(Self::CMSG_MAIL_DELETE(CMSG_MAIL_DELETE::astd_read_body(r, size).await?)),
            0x024A => Ok(Self::CMSG_MAIL_CREATE_TEXT_ITEM(CMSG_MAIL_CREATE_TEXT_ITEM::astd_read_body(r, size).await?)),
            0x0251 => Ok(Self::CMSG_LEARN_TALENT(CMSG_LEARN_TALENT::astd_read_body(r, size).await?)),
            0x0253 => Ok(Self::CMSG_TOGGLE_PVP(CMSG_TOGGLE_PVP::astd_read_body(r, size).await?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Client::astd_read_body(r, size).await?)),
            0x0256 => Ok(Self::CMSG_AUCTION_SELL_ITEM(CMSG_AUCTION_SELL_ITEM::astd_read_body(r, size).await?)),
            0x0257 => Ok(Self::CMSG_AUCTION_REMOVE_ITEM(CMSG_AUCTION_REMOVE_ITEM::astd_read_body(r, size).await?)),
            0x0258 => Ok(Self::CMSG_AUCTION_LIST_ITEMS(CMSG_AUCTION_LIST_ITEMS::astd_read_body(r, size).await?)),
            0x0259 => Ok(Self::CMSG_AUCTION_LIST_OWNER_ITEMS(CMSG_AUCTION_LIST_OWNER_ITEMS::astd_read_body(r, size).await?)),
            0x025A => Ok(Self::CMSG_AUCTION_PLACE_BID(CMSG_AUCTION_PLACE_BID::astd_read_body(r, size).await?)),
            0x0264 => Ok(Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(CMSG_AUCTION_LIST_BIDDER_ITEMS::astd_read_body(r, size).await?)),
            0x0268 => Ok(Self::CMSG_SET_AMMO(CMSG_SET_AMMO::astd_read_body(r, size).await?)),
            0x026A => Ok(Self::CMSG_SET_ACTIVE_MOVER(CMSG_SET_ACTIVE_MOVER::astd_read_body(r, size).await?)),
            0x026B => Ok(Self::CMSG_PET_CANCEL_AURA(CMSG_PET_CANCEL_AURA::astd_read_body(r, size).await?)),
            0x026D => Ok(Self::CMSG_CANCEL_AUTO_REPEAT_SPELL(CMSG_CANCEL_AUTO_REPEAT_SPELL::astd_read_body(r, size).await?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Client::astd_read_body(r, size).await?)),
            0x0270 => Ok(Self::CMSG_STABLE_PET(CMSG_STABLE_PET::astd_read_body(r, size).await?)),
            0x0271 => Ok(Self::CMSG_UNSTABLE_PET(CMSG_UNSTABLE_PET::astd_read_body(r, size).await?)),
            0x0272 => Ok(Self::CMSG_BUY_STABLE_SLOT(CMSG_BUY_STABLE_SLOT::astd_read_body(r, size).await?)),
            0x0275 => Ok(Self::CMSG_STABLE_SWAP_PET(CMSG_STABLE_SWAP_PET::astd_read_body(r, size).await?)),
            0x0279 => Ok(Self::CMSG_REQUEST_PET_INFO(CMSG_REQUEST_PET_INFO::astd_read_body(r, size).await?)),
            0x027A => Ok(Self::CMSG_FAR_SIGHT(CMSG_FAR_SIGHT::astd_read_body(r, size).await?)),
            0x027E => Ok(Self::CMSG_GROUP_CHANGE_SUB_GROUP(CMSG_GROUP_CHANGE_SUB_GROUP::astd_read_body(r, size).await?)),
            0x027F => Ok(Self::CMSG_REQUEST_PARTY_MEMBER_STATS(CMSG_REQUEST_PARTY_MEMBER_STATS::astd_read_body(r, size).await?)),
            0x0280 => Ok(Self::CMSG_GROUP_SWAP_SUB_GROUP(CMSG_GROUP_SWAP_SUB_GROUP::astd_read_body(r, size).await?)),
            0x0282 => Ok(Self::CMSG_AUTOSTORE_BANK_ITEM(CMSG_AUTOSTORE_BANK_ITEM::astd_read_body(r, size).await?)),
            0x0283 => Ok(Self::CMSG_AUTOBANK_ITEM(CMSG_AUTOBANK_ITEM::astd_read_body(r, size).await?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Client::astd_read_body(r, size).await?)),
            0x028E => Ok(Self::CMSG_GROUP_RAID_CONVERT(CMSG_GROUP_RAID_CONVERT::astd_read_body(r, size).await?)),
            0x028F => Ok(Self::CMSG_GROUP_ASSISTANT_LEADER(CMSG_GROUP_ASSISTANT_LEADER::astd_read_body(r, size).await?)),
            0x0290 => Ok(Self::CMSG_BUYBACK_ITEM(CMSG_BUYBACK_ITEM::astd_read_body(r, size).await?)),
            0x0292 => Ok(Self::CMSG_MEETINGSTONE_JOIN(CMSG_MEETINGSTONE_JOIN::astd_read_body(r, size).await?)),
            0x0293 => Ok(Self::CMSG_MEETINGSTONE_LEAVE(CMSG_MEETINGSTONE_LEAVE::astd_read_body(r, size).await?)),
            0x0296 => Ok(Self::CMSG_MEETINGSTONE_INFO(CMSG_MEETINGSTONE_INFO::astd_read_body(r, size).await?)),
            0x029B => Ok(Self::CMSG_CANCEL_GROWTH_AURA(CMSG_CANCEL_GROWTH_AURA::astd_read_body(r, size).await?)),
            0x02A0 => Ok(Self::CMSG_LOOT_ROLL(CMSG_LOOT_ROLL::astd_read_body(r, size).await?)),
            0x02A3 => Ok(Self::CMSG_LOOT_MASTER_GIVE(CMSG_LOOT_MASTER_GIVE::astd_read_body(r, size).await?)),
            0x02A8 => Ok(Self::CMSG_REPAIR_ITEM(CMSG_REPAIR_ITEM::astd_read_body(r, size).await?)),
            0x02AA => Ok(Self::MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Client::astd_read_body(r, size).await?)),
            0x02AC => Ok(Self::CMSG_SUMMON_RESPONSE(CMSG_SUMMON_RESPONSE::astd_read_body(r, size).await?)),
            0x02B3 => Ok(Self::CMSG_SELF_RES(CMSG_SELF_RES::astd_read_body(r, size).await?)),
            0x02B9 => Ok(Self::CMSG_TOGGLE_HELM(CMSG_TOGGLE_HELM::astd_read_body(r, size).await?)),
            0x02BA => Ok(Self::CMSG_TOGGLE_CLOAK(CMSG_TOGGLE_CLOAK::astd_read_body(r, size).await?)),
            0x02BF => Ok(Self::CMSG_SET_ACTIONBAR_TOGGLES(CMSG_SET_ACTIONBAR_TOGGLES::astd_read_body(r, size).await?)),
            0x02C4 => Ok(Self::CMSG_ITEM_NAME_QUERY(CMSG_ITEM_NAME_QUERY::astd_read_body(r, size).await?)),
            0x02C7 => Ok(Self::CMSG_CHAR_RENAME(CMSG_CHAR_RENAME::astd_read_body(r, size).await?)),
            0x02C9 => Ok(Self::CMSG_MOVE_SPLINE_DONE(CMSG_MOVE_SPLINE_DONE::astd_read_body(r, size).await?)),
            0x02CA => Ok(Self::CMSG_MOVE_FALL_RESET(CMSG_MOVE_FALL_RESET::astd_read_body(r, size).await?)),
            0x02CD => Ok(Self::CMSG_REQUEST_RAID_INFO(CMSG_REQUEST_RAID_INFO::astd_read_body(r, size).await?)),
            0x02CE => Ok(Self::CMSG_MOVE_TIME_SKIPPED(CMSG_MOVE_TIME_SKIPPED::astd_read_body(r, size).await?)),
            0x02CF => Ok(Self::CMSG_MOVE_FEATHER_FALL_ACK(CMSG_MOVE_FEATHER_FALL_ACK::astd_read_body(r, size).await?)),
            0x02D0 => Ok(Self::CMSG_MOVE_WATER_WALK_ACK(CMSG_MOVE_WATER_WALK_ACK::astd_read_body(r, size).await?)),
            0x02D1 => Ok(Self::CMSG_MOVE_NOT_ACTIVE_MOVER(CMSG_MOVE_NOT_ACTIVE_MOVER::astd_read_body(r, size).await?)),
            0x02D3 => Ok(Self::CMSG_BATTLEFIELD_STATUS(CMSG_BATTLEFIELD_STATUS::astd_read_body(r, size).await?)),
            0x02D5 => Ok(Self::CMSG_BATTLEFIELD_PORT(CMSG_BATTLEFIELD_PORT::astd_read_body(r, size).await?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Client::astd_read_body(r, size).await?)),
            0x02D7 => Ok(Self::CMSG_BATTLEMASTER_HELLO(CMSG_BATTLEMASTER_HELLO::astd_read_body(r, size).await?)),
            0x02DB => Ok(Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(CMSG_FORCE_WALK_SPEED_CHANGE_ACK::astd_read_body(r, size).await?)),
            0x02DD => Ok(Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK::astd_read_body(r, size).await?)),
            0x02DF => Ok(Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(CMSG_FORCE_TURN_RATE_CHANGE_ACK::astd_read_body(r, size).await?)),
            0x02E0 => Ok(Self::MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_Client::astd_read_body(r, size).await?)),
            0x02E1 => Ok(Self::CMSG_LEAVE_BATTLEFIELD(CMSG_LEAVE_BATTLEFIELD::astd_read_body(r, size).await?)),
            0x02E2 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUERY(CMSG_AREA_SPIRIT_HEALER_QUERY::astd_read_body(r, size).await?)),
            0x02E3 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(CMSG_AREA_SPIRIT_HEALER_QUEUE::astd_read_body(r, size).await?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Client::astd_read_body(r, size).await?)),
            0x02EA => Ok(Self::CMSG_PET_STOP_ATTACK(CMSG_PET_STOP_ATTACK::astd_read_body(r, size).await?)),
            0x02EE => Ok(Self::CMSG_BATTLEMASTER_JOIN(CMSG_BATTLEMASTER_JOIN::astd_read_body(r, size).await?)),
            0x02F0 => Ok(Self::CMSG_PET_UNLEARN(CMSG_PET_UNLEARN::astd_read_body(r, size).await?)),
            0x02F3 => Ok(Self::CMSG_PET_SPELL_AUTOCAST(CMSG_PET_SPELL_AUTOCAST::astd_read_body(r, size).await?)),
            0x02FC => Ok(Self::CMSG_GUILD_INFO_TEXT(CMSG_GUILD_INFO_TEXT::astd_read_body(r, size).await?)),
            0x0312 => Ok(Self::CMSG_ACTIVATETAXIEXPRESS(CMSG_ACTIVATETAXIEXPRESS::astd_read_body(r, size).await?)),
            0x0317 => Ok(Self::CMSG_SET_FACTION_INACTIVE(CMSG_SET_FACTION_INACTIVE::astd_read_body(r, size).await?)),
            0x0318 => Ok(Self::CMSG_SET_WATCHED_FACTION(CMSG_SET_WATCHED_FACTION::astd_read_body(r, size).await?)),
            0x031D => Ok(Self::CMSG_RESET_INSTANCES(CMSG_RESET_INSTANCES::astd_read_body(r, size).await?)),
            0x0321 => Ok(Self::MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Client::astd_read_body(r, size).await?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Client::astd_read_body(r, size).await?)),
            0x032A => Ok(Self::CMSG_GMSURVEY_SUBMIT(CMSG_GMSURVEY_SUBMIT::astd_read_body(r, size).await?)),
            _ => Err(ClientOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }
    #[cfg(feature = "async-std")]
    pub async fn astd_read_encrypted<R: ReadExt + Unpin + Send, D: Decrypter + Send>(r: &mut R, d: &mut D) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let mut header = [0u8; 6];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_client_header(header);
        let header_size = (header.size - 4) as u32;
        match header.opcode {
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD::astd_read_body(r, header_size).await?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD::astd_read_body(r, header_size).await?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(MSG_MOVE_STOP::astd_read_body(r, header_size).await?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT::astd_read_body(r, header_size).await?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT::astd_read_body(r, header_size).await?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE::astd_read_body(r, header_size).await?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(MSG_MOVE_JUMP::astd_read_body(r, header_size).await?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT::astd_read_body(r, header_size).await?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT::astd_read_body(r, header_size).await?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN::astd_read_body(r, header_size).await?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP::astd_read_body(r, header_size).await?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN::astd_read_body(r, header_size).await?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH::astd_read_body(r, header_size).await?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE::astd_read_body(r, header_size).await?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE::astd_read_body(r, header_size).await?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK::astd_read_body(r, header_size).await?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND::astd_read_body(r, header_size).await?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM::astd_read_body(r, header_size).await?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM::astd_read_body(r, header_size).await?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING::astd_read_body(r, header_size).await?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH::astd_read_body(r, header_size).await?)),
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK::astd_read_body(r, header_size).await?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT::astd_read_body(r, header_size).await?)),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(MSG_PETITION_DECLINE::astd_read_body(r, header_size).await?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE::astd_read_body(r, header_size).await?)),
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT::astd_read_body(r, header_size).await?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(MSG_PETITION_RENAME::astd_read_body(r, header_size).await?)),
            0x0008 => Ok(Self::CMSG_WORLD_TELEPORT(CMSG_WORLD_TELEPORT::astd_read_body(r, header_size).await?)),
            0x0036 => Ok(Self::CMSG_CHAR_CREATE(CMSG_CHAR_CREATE::astd_read_body(r, header_size).await?)),
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(CMSG_CHAR_ENUM::astd_read_body(r, header_size).await?)),
            0x0038 => Ok(Self::CMSG_CHAR_DELETE(CMSG_CHAR_DELETE::astd_read_body(r, header_size).await?)),
            0x003D => Ok(Self::CMSG_PLAYER_LOGIN(CMSG_PLAYER_LOGIN::astd_read_body(r, header_size).await?)),
            0x004A => Ok(Self::CMSG_PLAYER_LOGOUT(CMSG_PLAYER_LOGOUT::astd_read_body(r, header_size).await?)),
            0x004B => Ok(Self::CMSG_LOGOUT_REQUEST(CMSG_LOGOUT_REQUEST::astd_read_body(r, header_size).await?)),
            0x004E => Ok(Self::CMSG_LOGOUT_CANCEL(CMSG_LOGOUT_CANCEL::astd_read_body(r, header_size).await?)),
            0x0050 => Ok(Self::CMSG_NAME_QUERY(CMSG_NAME_QUERY::astd_read_body(r, header_size).await?)),
            0x0052 => Ok(Self::CMSG_PET_NAME_QUERY(CMSG_PET_NAME_QUERY::astd_read_body(r, header_size).await?)),
            0x0054 => Ok(Self::CMSG_GUILD_QUERY(CMSG_GUILD_QUERY::astd_read_body(r, header_size).await?)),
            0x0056 => Ok(Self::CMSG_ITEM_QUERY_SINGLE(CMSG_ITEM_QUERY_SINGLE::astd_read_body(r, header_size).await?)),
            0x005A => Ok(Self::CMSG_PAGE_TEXT_QUERY(CMSG_PAGE_TEXT_QUERY::astd_read_body(r, header_size).await?)),
            0x005C => Ok(Self::CMSG_QUEST_QUERY(CMSG_QUEST_QUERY::astd_read_body(r, header_size).await?)),
            0x005E => Ok(Self::CMSG_GAMEOBJECT_QUERY(CMSG_GAMEOBJECT_QUERY::astd_read_body(r, header_size).await?)),
            0x0060 => Ok(Self::CMSG_CREATURE_QUERY(CMSG_CREATURE_QUERY::astd_read_body(r, header_size).await?)),
            0x0062 => Ok(Self::CMSG_WHO(CMSG_WHO::astd_read_body(r, header_size).await?)),
            0x0064 => Ok(Self::CMSG_WHOIS(CMSG_WHOIS::astd_read_body(r, header_size).await?)),
            0x0066 => Ok(Self::CMSG_FRIEND_LIST(CMSG_FRIEND_LIST::astd_read_body(r, header_size).await?)),
            0x0069 => Ok(Self::CMSG_ADD_FRIEND(CMSG_ADD_FRIEND::astd_read_body(r, header_size).await?)),
            0x006A => Ok(Self::CMSG_DEL_FRIEND(CMSG_DEL_FRIEND::astd_read_body(r, header_size).await?)),
            0x006C => Ok(Self::CMSG_ADD_IGNORE(CMSG_ADD_IGNORE::astd_read_body(r, header_size).await?)),
            0x006D => Ok(Self::CMSG_DEL_IGNORE(CMSG_DEL_IGNORE::astd_read_body(r, header_size).await?)),
            0x006E => Ok(Self::CMSG_GROUP_INVITE(CMSG_GROUP_INVITE::astd_read_body(r, header_size).await?)),
            0x0072 => Ok(Self::CMSG_GROUP_ACCEPT(CMSG_GROUP_ACCEPT::astd_read_body(r, header_size).await?)),
            0x0073 => Ok(Self::CMSG_GROUP_DECLINE(CMSG_GROUP_DECLINE::astd_read_body(r, header_size).await?)),
            0x0075 => Ok(Self::CMSG_GROUP_UNINVITE(CMSG_GROUP_UNINVITE::astd_read_body(r, header_size).await?)),
            0x0076 => Ok(Self::CMSG_GROUP_UNINVITE_GUID(CMSG_GROUP_UNINVITE_GUID::astd_read_body(r, header_size).await?)),
            0x0078 => Ok(Self::CMSG_GROUP_SET_LEADER(CMSG_GROUP_SET_LEADER::astd_read_body(r, header_size).await?)),
            0x007A => Ok(Self::CMSG_LOOT_METHOD(CMSG_LOOT_METHOD::astd_read_body(r, header_size).await?)),
            0x007B => Ok(Self::CMSG_GROUP_DISBAND(CMSG_GROUP_DISBAND::astd_read_body(r, header_size).await?)),
            0x0081 => Ok(Self::CMSG_GUILD_CREATE(CMSG_GUILD_CREATE::astd_read_body(r, header_size).await?)),
            0x0082 => Ok(Self::CMSG_GUILD_INVITE(CMSG_GUILD_INVITE::astd_read_body(r, header_size).await?)),
            0x0084 => Ok(Self::CMSG_GUILD_ACCEPT(CMSG_GUILD_ACCEPT::astd_read_body(r, header_size).await?)),
            0x0085 => Ok(Self::CMSG_GUILD_DECLINE(CMSG_GUILD_DECLINE::astd_read_body(r, header_size).await?)),
            0x0087 => Ok(Self::CMSG_GUILD_INFO(CMSG_GUILD_INFO::astd_read_body(r, header_size).await?)),
            0x0089 => Ok(Self::CMSG_GUILD_ROSTER(CMSG_GUILD_ROSTER::astd_read_body(r, header_size).await?)),
            0x008B => Ok(Self::CMSG_GUILD_PROMOTE(CMSG_GUILD_PROMOTE::astd_read_body(r, header_size).await?)),
            0x008C => Ok(Self::CMSG_GUILD_DEMOTE(CMSG_GUILD_DEMOTE::astd_read_body(r, header_size).await?)),
            0x008D => Ok(Self::CMSG_GUILD_LEAVE(CMSG_GUILD_LEAVE::astd_read_body(r, header_size).await?)),
            0x008E => Ok(Self::CMSG_GUILD_REMOVE(CMSG_GUILD_REMOVE::astd_read_body(r, header_size).await?)),
            0x008F => Ok(Self::CMSG_GUILD_DISBAND(CMSG_GUILD_DISBAND::astd_read_body(r, header_size).await?)),
            0x0090 => Ok(Self::CMSG_GUILD_LEADER(CMSG_GUILD_LEADER::astd_read_body(r, header_size).await?)),
            0x0091 => Ok(Self::CMSG_GUILD_MOTD(CMSG_GUILD_MOTD::astd_read_body(r, header_size).await?)),
            0x0095 => Ok(Self::CMSG_MESSAGECHAT(CMSG_MESSAGECHAT::astd_read_body(r, header_size).await?)),
            0x0097 => Ok(Self::CMSG_JOIN_CHANNEL(CMSG_JOIN_CHANNEL::astd_read_body(r, header_size).await?)),
            0x0098 => Ok(Self::CMSG_LEAVE_CHANNEL(CMSG_LEAVE_CHANNEL::astd_read_body(r, header_size).await?)),
            0x009A => Ok(Self::CMSG_CHANNEL_LIST(CMSG_CHANNEL_LIST::astd_read_body(r, header_size).await?)),
            0x009C => Ok(Self::CMSG_CHANNEL_PASSWORD(CMSG_CHANNEL_PASSWORD::astd_read_body(r, header_size).await?)),
            0x009D => Ok(Self::CMSG_CHANNEL_SET_OWNER(CMSG_CHANNEL_SET_OWNER::astd_read_body(r, header_size).await?)),
            0x009E => Ok(Self::CMSG_CHANNEL_OWNER(CMSG_CHANNEL_OWNER::astd_read_body(r, header_size).await?)),
            0x009F => Ok(Self::CMSG_CHANNEL_MODERATOR(CMSG_CHANNEL_MODERATOR::astd_read_body(r, header_size).await?)),
            0x00A0 => Ok(Self::CMSG_CHANNEL_UNMODERATOR(CMSG_CHANNEL_UNMODERATOR::astd_read_body(r, header_size).await?)),
            0x00A1 => Ok(Self::CMSG_CHANNEL_MUTE(CMSG_CHANNEL_MUTE::astd_read_body(r, header_size).await?)),
            0x00A2 => Ok(Self::CMSG_CHANNEL_UNMUTE(CMSG_CHANNEL_UNMUTE::astd_read_body(r, header_size).await?)),
            0x00A3 => Ok(Self::CMSG_CHANNEL_INVITE(CMSG_CHANNEL_INVITE::astd_read_body(r, header_size).await?)),
            0x00A4 => Ok(Self::CMSG_CHANNEL_KICK(CMSG_CHANNEL_KICK::astd_read_body(r, header_size).await?)),
            0x00A5 => Ok(Self::CMSG_CHANNEL_BAN(CMSG_CHANNEL_BAN::astd_read_body(r, header_size).await?)),
            0x00A6 => Ok(Self::CMSG_CHANNEL_UNBAN(CMSG_CHANNEL_UNBAN::astd_read_body(r, header_size).await?)),
            0x00A7 => Ok(Self::CMSG_CHANNEL_ANNOUNCEMENTS(CMSG_CHANNEL_ANNOUNCEMENTS::astd_read_body(r, header_size).await?)),
            0x00A8 => Ok(Self::CMSG_CHANNEL_MODERATE(CMSG_CHANNEL_MODERATE::astd_read_body(r, header_size).await?)),
            0x00A9 => Ok(Self::SMSG_UPDATE_OBJECT(SMSG_UPDATE_OBJECT::astd_read_body(r, header_size).await?)),
            0x00AB => Ok(Self::CMSG_USE_ITEM(CMSG_USE_ITEM::astd_read_body(r, header_size).await?)),
            0x00AC => Ok(Self::CMSG_OPEN_ITEM(CMSG_OPEN_ITEM::astd_read_body(r, header_size).await?)),
            0x00AD => Ok(Self::CMSG_READ_ITEM(CMSG_READ_ITEM::astd_read_body(r, header_size).await?)),
            0x00B1 => Ok(Self::CMSG_GAMEOBJ_USE(CMSG_GAMEOBJ_USE::astd_read_body(r, header_size).await?)),
            0x00B4 => Ok(Self::CMSG_AREATRIGGER(CMSG_AREATRIGGER::astd_read_body(r, header_size).await?)),
            0x00E1 => Ok(Self::CMSG_MOVE_SET_RAW_POSITION(CMSG_MOVE_SET_RAW_POSITION::astd_read_body(r, header_size).await?)),
            0x00E3 => Ok(Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_SPEED_CHANGE_ACK::astd_read_body(r, header_size).await?)),
            0x00E5 => Ok(Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK::astd_read_body(r, header_size).await?)),
            0x00E7 => Ok(Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_SPEED_CHANGE_ACK::astd_read_body(r, header_size).await?)),
            0x00E9 => Ok(Self::CMSG_FORCE_MOVE_ROOT_ACK(CMSG_FORCE_MOVE_ROOT_ACK::astd_read_body(r, header_size).await?)),
            0x00EB => Ok(Self::CMSG_FORCE_MOVE_UNROOT_ACK(CMSG_FORCE_MOVE_UNROOT_ACK::astd_read_body(r, header_size).await?)),
            0x00F0 => Ok(Self::CMSG_MOVE_KNOCK_BACK_ACK(CMSG_MOVE_KNOCK_BACK_ACK::astd_read_body(r, header_size).await?)),
            0x00F6 => Ok(Self::CMSG_MOVE_HOVER_ACK(CMSG_MOVE_HOVER_ACK::astd_read_body(r, header_size).await?)),
            0x00FB => Ok(Self::CMSG_NEXT_CINEMATIC_CAMERA(CMSG_NEXT_CINEMATIC_CAMERA::astd_read_body(r, header_size).await?)),
            0x00FC => Ok(Self::CMSG_COMPLETE_CINEMATIC(CMSG_COMPLETE_CINEMATIC::astd_read_body(r, header_size).await?)),
            0x00FE => Ok(Self::CMSG_TUTORIAL_FLAG(CMSG_TUTORIAL_FLAG::astd_read_body(r, header_size).await?)),
            0x00FF => Ok(Self::CMSG_TUTORIAL_CLEAR(CMSG_TUTORIAL_CLEAR::astd_read_body(r, header_size).await?)),
            0x0100 => Ok(Self::CMSG_TUTORIAL_RESET(CMSG_TUTORIAL_RESET::astd_read_body(r, header_size).await?)),
            0x0101 => Ok(Self::CMSG_STANDSTATECHANGE(CMSG_STANDSTATECHANGE::astd_read_body(r, header_size).await?)),
            0x0102 => Ok(Self::CMSG_EMOTE(CMSG_EMOTE::astd_read_body(r, header_size).await?)),
            0x0104 => Ok(Self::CMSG_TEXT_EMOTE(CMSG_TEXT_EMOTE::astd_read_body(r, header_size).await?)),
            0x0108 => Ok(Self::CMSG_AUTOSTORE_LOOT_ITEM(CMSG_AUTOSTORE_LOOT_ITEM::astd_read_body(r, header_size).await?)),
            0x010A => Ok(Self::CMSG_AUTOEQUIP_ITEM(CMSG_AUTOEQUIP_ITEM::astd_read_body(r, header_size).await?)),
            0x010B => Ok(Self::CMSG_AUTOSTORE_BAG_ITEM(CMSG_AUTOSTORE_BAG_ITEM::astd_read_body(r, header_size).await?)),
            0x010C => Ok(Self::CMSG_SWAP_ITEM(CMSG_SWAP_ITEM::astd_read_body(r, header_size).await?)),
            0x010D => Ok(Self::CMSG_SWAP_INV_ITEM(CMSG_SWAP_INV_ITEM::astd_read_body(r, header_size).await?)),
            0x010E => Ok(Self::CMSG_SPLIT_ITEM(CMSG_SPLIT_ITEM::astd_read_body(r, header_size).await?)),
            0x010F => Ok(Self::CMSG_AUTOEQUIP_ITEM_SLOT(CMSG_AUTOEQUIP_ITEM_SLOT::astd_read_body(r, header_size).await?)),
            0x0111 => Ok(Self::CMSG_DESTROYITEM(CMSG_DESTROYITEM::astd_read_body(r, header_size).await?)),
            0x0114 => Ok(Self::CMSG_INSPECT(CMSG_INSPECT::astd_read_body(r, header_size).await?)),
            0x0116 => Ok(Self::CMSG_INITIATE_TRADE(CMSG_INITIATE_TRADE::astd_read_body(r, header_size).await?)),
            0x0117 => Ok(Self::CMSG_BEGIN_TRADE(CMSG_BEGIN_TRADE::astd_read_body(r, header_size).await?)),
            0x0118 => Ok(Self::CMSG_BUSY_TRADE(CMSG_BUSY_TRADE::astd_read_body(r, header_size).await?)),
            0x0119 => Ok(Self::CMSG_IGNORE_TRADE(CMSG_IGNORE_TRADE::astd_read_body(r, header_size).await?)),
            0x011A => Ok(Self::CMSG_ACCEPT_TRADE(CMSG_ACCEPT_TRADE::astd_read_body(r, header_size).await?)),
            0x011B => Ok(Self::CMSG_UNACCEPT_TRADE(CMSG_UNACCEPT_TRADE::astd_read_body(r, header_size).await?)),
            0x011C => Ok(Self::CMSG_CANCEL_TRADE(CMSG_CANCEL_TRADE::astd_read_body(r, header_size).await?)),
            0x011D => Ok(Self::CMSG_SET_TRADE_ITEM(CMSG_SET_TRADE_ITEM::astd_read_body(r, header_size).await?)),
            0x011E => Ok(Self::CMSG_CLEAR_TRADE_ITEM(CMSG_CLEAR_TRADE_ITEM::astd_read_body(r, header_size).await?)),
            0x011F => Ok(Self::CMSG_SET_TRADE_GOLD(CMSG_SET_TRADE_GOLD::astd_read_body(r, header_size).await?)),
            0x0125 => Ok(Self::CMSG_SET_FACTION_ATWAR(CMSG_SET_FACTION_ATWAR::astd_read_body(r, header_size).await?)),
            0x0128 => Ok(Self::CMSG_SET_ACTION_BUTTON(CMSG_SET_ACTION_BUTTON::astd_read_body(r, header_size).await?)),
            0x012E => Ok(Self::CMSG_CAST_SPELL(CMSG_CAST_SPELL::astd_read_body(r, header_size).await?)),
            0x012F => Ok(Self::CMSG_CANCEL_CAST(CMSG_CANCEL_CAST::astd_read_body(r, header_size).await?)),
            0x0136 => Ok(Self::CMSG_CANCEL_AURA(CMSG_CANCEL_AURA::astd_read_body(r, header_size).await?)),
            0x013B => Ok(Self::CMSG_CANCEL_CHANNELLING(CMSG_CANCEL_CHANNELLING::astd_read_body(r, header_size).await?)),
            0x013D => Ok(Self::CMSG_SET_SELECTION(CMSG_SET_SELECTION::astd_read_body(r, header_size).await?)),
            0x013E => Ok(Self::CMSG_SET_TARGET_OBSOLETE(CMSG_SET_TARGET_OBSOLETE::astd_read_body(r, header_size).await?)),
            0x0141 => Ok(Self::CMSG_ATTACKSWING(CMSG_ATTACKSWING::astd_read_body(r, header_size).await?)),
            0x0142 => Ok(Self::CMSG_ATTACKSTOP(CMSG_ATTACKSTOP::astd_read_body(r, header_size).await?)),
            0x015A => Ok(Self::CMSG_REPOP_REQUEST(CMSG_REPOP_REQUEST::astd_read_body(r, header_size).await?)),
            0x015C => Ok(Self::CMSG_RESURRECT_RESPONSE(CMSG_RESURRECT_RESPONSE::astd_read_body(r, header_size).await?)),
            0x015D => Ok(Self::CMSG_LOOT(CMSG_LOOT::astd_read_body(r, header_size).await?)),
            0x015E => Ok(Self::CMSG_LOOT_MONEY(CMSG_LOOT_MONEY::astd_read_body(r, header_size).await?)),
            0x015F => Ok(Self::CMSG_LOOT_RELEASE(CMSG_LOOT_RELEASE::astd_read_body(r, header_size).await?)),
            0x016C => Ok(Self::CMSG_DUEL_ACCEPTED(CMSG_DUEL_ACCEPTED::astd_read_body(r, header_size).await?)),
            0x016D => Ok(Self::CMSG_DUEL_CANCELLED(CMSG_DUEL_CANCELLED::astd_read_body(r, header_size).await?)),
            0x0171 => Ok(Self::CMSG_MOUNTSPECIAL_ANIM(CMSG_MOUNTSPECIAL_ANIM::astd_read_body(r, header_size).await?)),
            0x0174 => Ok(Self::CMSG_PET_SET_ACTION(CMSG_PET_SET_ACTION::astd_read_body(r, header_size).await?)),
            0x0175 => Ok(Self::CMSG_PET_ACTION(CMSG_PET_ACTION::astd_read_body(r, header_size).await?)),
            0x0176 => Ok(Self::CMSG_PET_ABANDON(CMSG_PET_ABANDON::astd_read_body(r, header_size).await?)),
            0x0177 => Ok(Self::CMSG_PET_RENAME(CMSG_PET_RENAME::astd_read_body(r, header_size).await?)),
            0x017B => Ok(Self::CMSG_GOSSIP_HELLO(CMSG_GOSSIP_HELLO::astd_read_body(r, header_size).await?)),
            0x017C => Ok(Self::CMSG_GOSSIP_SELECT_OPTION(CMSG_GOSSIP_SELECT_OPTION::astd_read_body(r, header_size).await?)),
            0x017F => Ok(Self::CMSG_NPC_TEXT_QUERY(CMSG_NPC_TEXT_QUERY::astd_read_body(r, header_size).await?)),
            0x0182 => Ok(Self::CMSG_QUESTGIVER_STATUS_QUERY(CMSG_QUESTGIVER_STATUS_QUERY::astd_read_body(r, header_size).await?)),
            0x0184 => Ok(Self::CMSG_QUESTGIVER_HELLO(CMSG_QUESTGIVER_HELLO::astd_read_body(r, header_size).await?)),
            0x0186 => Ok(Self::CMSG_QUESTGIVER_QUERY_QUEST(CMSG_QUESTGIVER_QUERY_QUEST::astd_read_body(r, header_size).await?)),
            0x0187 => Ok(Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(CMSG_QUESTGIVER_QUEST_AUTOLAUNCH::astd_read_body(r, header_size).await?)),
            0x0189 => Ok(Self::CMSG_QUESTGIVER_ACCEPT_QUEST(CMSG_QUESTGIVER_ACCEPT_QUEST::astd_read_body(r, header_size).await?)),
            0x018A => Ok(Self::CMSG_QUESTGIVER_COMPLETE_QUEST(CMSG_QUESTGIVER_COMPLETE_QUEST::astd_read_body(r, header_size).await?)),
            0x018C => Ok(Self::CMSG_QUESTGIVER_REQUEST_REWARD(CMSG_QUESTGIVER_REQUEST_REWARD::astd_read_body(r, header_size).await?)),
            0x018E => Ok(Self::CMSG_QUESTGIVER_CHOOSE_REWARD(CMSG_QUESTGIVER_CHOOSE_REWARD::astd_read_body(r, header_size).await?)),
            0x0190 => Ok(Self::CMSG_QUESTGIVER_CANCEL(CMSG_QUESTGIVER_CANCEL::astd_read_body(r, header_size).await?)),
            0x0193 => Ok(Self::CMSG_QUESTLOG_SWAP_QUEST(CMSG_QUESTLOG_SWAP_QUEST::astd_read_body(r, header_size).await?)),
            0x0194 => Ok(Self::CMSG_QUESTLOG_REMOVE_QUEST(CMSG_QUESTLOG_REMOVE_QUEST::astd_read_body(r, header_size).await?)),
            0x019B => Ok(Self::CMSG_QUEST_CONFIRM_ACCEPT(CMSG_QUEST_CONFIRM_ACCEPT::astd_read_body(r, header_size).await?)),
            0x019D => Ok(Self::CMSG_PUSHQUESTTOPARTY(CMSG_PUSHQUESTTOPARTY::astd_read_body(r, header_size).await?)),
            0x019E => Ok(Self::CMSG_LIST_INVENTORY(CMSG_LIST_INVENTORY::astd_read_body(r, header_size).await?)),
            0x01A0 => Ok(Self::CMSG_SELL_ITEM(CMSG_SELL_ITEM::astd_read_body(r, header_size).await?)),
            0x01A2 => Ok(Self::CMSG_BUY_ITEM(CMSG_BUY_ITEM::astd_read_body(r, header_size).await?)),
            0x01A3 => Ok(Self::CMSG_BUY_ITEM_IN_SLOT(CMSG_BUY_ITEM_IN_SLOT::astd_read_body(r, header_size).await?)),
            0x01AA => Ok(Self::CMSG_TAXINODE_STATUS_QUERY(CMSG_TAXINODE_STATUS_QUERY::astd_read_body(r, header_size).await?)),
            0x01AC => Ok(Self::CMSG_TAXIQUERYAVAILABLENODES(CMSG_TAXIQUERYAVAILABLENODES::astd_read_body(r, header_size).await?)),
            0x01AD => Ok(Self::CMSG_ACTIVATETAXI(CMSG_ACTIVATETAXI::astd_read_body(r, header_size).await?)),
            0x01B0 => Ok(Self::CMSG_TRAINER_LIST(CMSG_TRAINER_LIST::astd_read_body(r, header_size).await?)),
            0x01B2 => Ok(Self::CMSG_TRAINER_BUY_SPELL(CMSG_TRAINER_BUY_SPELL::astd_read_body(r, header_size).await?)),
            0x01B5 => Ok(Self::CMSG_BINDER_ACTIVATE(CMSG_BINDER_ACTIVATE::astd_read_body(r, header_size).await?)),
            0x01B7 => Ok(Self::CMSG_BANKER_ACTIVATE(CMSG_BANKER_ACTIVATE::astd_read_body(r, header_size).await?)),
            0x01B9 => Ok(Self::CMSG_BUY_BANK_SLOT(CMSG_BUY_BANK_SLOT::astd_read_body(r, header_size).await?)),
            0x01BB => Ok(Self::CMSG_PETITION_SHOWLIST(CMSG_PETITION_SHOWLIST::astd_read_body(r, header_size).await?)),
            0x01BD => Ok(Self::CMSG_PETITION_BUY(CMSG_PETITION_BUY::astd_read_body(r, header_size).await?)),
            0x01BE => Ok(Self::CMSG_PETITION_SHOW_SIGNATURES(CMSG_PETITION_SHOW_SIGNATURES::astd_read_body(r, header_size).await?)),
            0x01C0 => Ok(Self::CMSG_PETITION_SIGN(CMSG_PETITION_SIGN::astd_read_body(r, header_size).await?)),
            0x01C3 => Ok(Self::CMSG_OFFER_PETITION(CMSG_OFFER_PETITION::astd_read_body(r, header_size).await?)),
            0x01C4 => Ok(Self::CMSG_TURN_IN_PETITION(CMSG_TURN_IN_PETITION::astd_read_body(r, header_size).await?)),
            0x01C6 => Ok(Self::CMSG_PETITION_QUERY(CMSG_PETITION_QUERY::astd_read_body(r, header_size).await?)),
            0x01CA => Ok(Self::CMSG_BUG(CMSG_BUG::astd_read_body(r, header_size).await?)),
            0x01CC => Ok(Self::CMSG_PLAYED_TIME(CMSG_PLAYED_TIME::astd_read_body(r, header_size).await?)),
            0x01CE => Ok(Self::CMSG_QUERY_TIME(CMSG_QUERY_TIME::astd_read_body(r, header_size).await?)),
            0x01D2 => Ok(Self::CMSG_RECLAIM_CORPSE(CMSG_RECLAIM_CORPSE::astd_read_body(r, header_size).await?)),
            0x01D3 => Ok(Self::CMSG_WRAP_ITEM(CMSG_WRAP_ITEM::astd_read_body(r, header_size).await?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(MSG_MINIMAP_PING_Client::astd_read_body(r, header_size).await?)),
            0x01DC => Ok(Self::CMSG_PING(CMSG_PING::astd_read_body(r, header_size).await?)),
            0x01E0 => Ok(Self::CMSG_SETSHEATHED(CMSG_SETSHEATHED::astd_read_body(r, header_size).await?)),
            0x01ED => Ok(Self::CMSG_AUTH_SESSION(CMSG_AUTH_SESSION::astd_read_body(r, header_size).await?)),
            0x01F0 => Ok(Self::CMSG_PET_CAST_SPELL(CMSG_PET_CAST_SPELL::astd_read_body(r, header_size).await?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Client::astd_read_body(r, header_size).await?)),
            0x01F4 => Ok(Self::CMSG_ZONEUPDATE(CMSG_ZONEUPDATE::astd_read_body(r, header_size).await?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Client::astd_read_body(r, header_size).await?)),
            0x01FF => Ok(Self::MSG_LOOKING_FOR_GROUP(MSG_LOOKING_FOR_GROUP_Client::astd_read_body(r, header_size).await?)),
            0x0202 => Ok(Self::CMSG_UNLEARN_SKILL(CMSG_UNLEARN_SKILL::astd_read_body(r, header_size).await?)),
            0x0205 => Ok(Self::CMSG_GMTICKET_CREATE(CMSG_GMTICKET_CREATE::astd_read_body(r, header_size).await?)),
            0x0207 => Ok(Self::CMSG_GMTICKET_UPDATETEXT(CMSG_GMTICKET_UPDATETEXT::astd_read_body(r, header_size).await?)),
            0x020A => Ok(Self::CMSG_REQUEST_ACCOUNT_DATA(CMSG_REQUEST_ACCOUNT_DATA::astd_read_body(r, header_size).await?)),
            0x0211 => Ok(Self::CMSG_GMTICKET_GETTICKET(CMSG_GMTICKET_GETTICKET::astd_read_body(r, header_size).await?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Client::astd_read_body(r, header_size).await?)),
            0x0217 => Ok(Self::CMSG_GMTICKET_DELETETICKET(CMSG_GMTICKET_DELETETICKET::astd_read_body(r, header_size).await?)),
            0x021A => Ok(Self::CMSG_GMTICKET_SYSTEMSTATUS(CMSG_GMTICKET_SYSTEMSTATUS::astd_read_body(r, header_size).await?)),
            0x021C => Ok(Self::CMSG_SPIRIT_HEALER_ACTIVATE(CMSG_SPIRIT_HEALER_ACTIVATE::astd_read_body(r, header_size).await?)),
            0x0225 => Ok(Self::CMSG_CHAT_IGNORED(CMSG_CHAT_IGNORED::astd_read_body(r, header_size).await?)),
            0x0231 => Ok(Self::CMSG_GUILD_RANK(CMSG_GUILD_RANK::astd_read_body(r, header_size).await?)),
            0x0232 => Ok(Self::CMSG_GUILD_ADD_RANK(CMSG_GUILD_ADD_RANK::astd_read_body(r, header_size).await?)),
            0x0233 => Ok(Self::CMSG_GUILD_DEL_RANK(CMSG_GUILD_DEL_RANK::astd_read_body(r, header_size).await?)),
            0x0234 => Ok(Self::CMSG_GUILD_SET_PUBLIC_NOTE(CMSG_GUILD_SET_PUBLIC_NOTE::astd_read_body(r, header_size).await?)),
            0x0235 => Ok(Self::CMSG_GUILD_SET_OFFICER_NOTE(CMSG_GUILD_SET_OFFICER_NOTE::astd_read_body(r, header_size).await?)),
            0x0238 => Ok(Self::CMSG_SEND_MAIL(CMSG_SEND_MAIL::astd_read_body(r, header_size).await?)),
            0x023A => Ok(Self::CMSG_GET_MAIL_LIST(CMSG_GET_MAIL_LIST::astd_read_body(r, header_size).await?)),
            0x023C => Ok(Self::CMSG_BATTLEFIELD_LIST(CMSG_BATTLEFIELD_LIST::astd_read_body(r, header_size).await?)),
            0x023E => Ok(Self::CMSG_BATTLEFIELD_JOIN(CMSG_BATTLEFIELD_JOIN::astd_read_body(r, header_size).await?)),
            0x0243 => Ok(Self::CMSG_ITEM_TEXT_QUERY(CMSG_ITEM_TEXT_QUERY::astd_read_body(r, header_size).await?)),
            0x0245 => Ok(Self::CMSG_MAIL_TAKE_MONEY(CMSG_MAIL_TAKE_MONEY::astd_read_body(r, header_size).await?)),
            0x0246 => Ok(Self::CMSG_MAIL_TAKE_ITEM(CMSG_MAIL_TAKE_ITEM::astd_read_body(r, header_size).await?)),
            0x0247 => Ok(Self::CMSG_MAIL_MARK_AS_READ(CMSG_MAIL_MARK_AS_READ::astd_read_body(r, header_size).await?)),
            0x0248 => Ok(Self::CMSG_MAIL_RETURN_TO_SENDER(CMSG_MAIL_RETURN_TO_SENDER::astd_read_body(r, header_size).await?)),
            0x0249 => Ok(Self::CMSG_MAIL_DELETE(CMSG_MAIL_DELETE::astd_read_body(r, header_size).await?)),
            0x024A => Ok(Self::CMSG_MAIL_CREATE_TEXT_ITEM(CMSG_MAIL_CREATE_TEXT_ITEM::astd_read_body(r, header_size).await?)),
            0x0251 => Ok(Self::CMSG_LEARN_TALENT(CMSG_LEARN_TALENT::astd_read_body(r, header_size).await?)),
            0x0253 => Ok(Self::CMSG_TOGGLE_PVP(CMSG_TOGGLE_PVP::astd_read_body(r, header_size).await?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Client::astd_read_body(r, header_size).await?)),
            0x0256 => Ok(Self::CMSG_AUCTION_SELL_ITEM(CMSG_AUCTION_SELL_ITEM::astd_read_body(r, header_size).await?)),
            0x0257 => Ok(Self::CMSG_AUCTION_REMOVE_ITEM(CMSG_AUCTION_REMOVE_ITEM::astd_read_body(r, header_size).await?)),
            0x0258 => Ok(Self::CMSG_AUCTION_LIST_ITEMS(CMSG_AUCTION_LIST_ITEMS::astd_read_body(r, header_size).await?)),
            0x0259 => Ok(Self::CMSG_AUCTION_LIST_OWNER_ITEMS(CMSG_AUCTION_LIST_OWNER_ITEMS::astd_read_body(r, header_size).await?)),
            0x025A => Ok(Self::CMSG_AUCTION_PLACE_BID(CMSG_AUCTION_PLACE_BID::astd_read_body(r, header_size).await?)),
            0x0264 => Ok(Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(CMSG_AUCTION_LIST_BIDDER_ITEMS::astd_read_body(r, header_size).await?)),
            0x0268 => Ok(Self::CMSG_SET_AMMO(CMSG_SET_AMMO::astd_read_body(r, header_size).await?)),
            0x026A => Ok(Self::CMSG_SET_ACTIVE_MOVER(CMSG_SET_ACTIVE_MOVER::astd_read_body(r, header_size).await?)),
            0x026B => Ok(Self::CMSG_PET_CANCEL_AURA(CMSG_PET_CANCEL_AURA::astd_read_body(r, header_size).await?)),
            0x026D => Ok(Self::CMSG_CANCEL_AUTO_REPEAT_SPELL(CMSG_CANCEL_AUTO_REPEAT_SPELL::astd_read_body(r, header_size).await?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Client::astd_read_body(r, header_size).await?)),
            0x0270 => Ok(Self::CMSG_STABLE_PET(CMSG_STABLE_PET::astd_read_body(r, header_size).await?)),
            0x0271 => Ok(Self::CMSG_UNSTABLE_PET(CMSG_UNSTABLE_PET::astd_read_body(r, header_size).await?)),
            0x0272 => Ok(Self::CMSG_BUY_STABLE_SLOT(CMSG_BUY_STABLE_SLOT::astd_read_body(r, header_size).await?)),
            0x0275 => Ok(Self::CMSG_STABLE_SWAP_PET(CMSG_STABLE_SWAP_PET::astd_read_body(r, header_size).await?)),
            0x0279 => Ok(Self::CMSG_REQUEST_PET_INFO(CMSG_REQUEST_PET_INFO::astd_read_body(r, header_size).await?)),
            0x027A => Ok(Self::CMSG_FAR_SIGHT(CMSG_FAR_SIGHT::astd_read_body(r, header_size).await?)),
            0x027E => Ok(Self::CMSG_GROUP_CHANGE_SUB_GROUP(CMSG_GROUP_CHANGE_SUB_GROUP::astd_read_body(r, header_size).await?)),
            0x027F => Ok(Self::CMSG_REQUEST_PARTY_MEMBER_STATS(CMSG_REQUEST_PARTY_MEMBER_STATS::astd_read_body(r, header_size).await?)),
            0x0280 => Ok(Self::CMSG_GROUP_SWAP_SUB_GROUP(CMSG_GROUP_SWAP_SUB_GROUP::astd_read_body(r, header_size).await?)),
            0x0282 => Ok(Self::CMSG_AUTOSTORE_BANK_ITEM(CMSG_AUTOSTORE_BANK_ITEM::astd_read_body(r, header_size).await?)),
            0x0283 => Ok(Self::CMSG_AUTOBANK_ITEM(CMSG_AUTOBANK_ITEM::astd_read_body(r, header_size).await?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Client::astd_read_body(r, header_size).await?)),
            0x028E => Ok(Self::CMSG_GROUP_RAID_CONVERT(CMSG_GROUP_RAID_CONVERT::astd_read_body(r, header_size).await?)),
            0x028F => Ok(Self::CMSG_GROUP_ASSISTANT_LEADER(CMSG_GROUP_ASSISTANT_LEADER::astd_read_body(r, header_size).await?)),
            0x0290 => Ok(Self::CMSG_BUYBACK_ITEM(CMSG_BUYBACK_ITEM::astd_read_body(r, header_size).await?)),
            0x0292 => Ok(Self::CMSG_MEETINGSTONE_JOIN(CMSG_MEETINGSTONE_JOIN::astd_read_body(r, header_size).await?)),
            0x0293 => Ok(Self::CMSG_MEETINGSTONE_LEAVE(CMSG_MEETINGSTONE_LEAVE::astd_read_body(r, header_size).await?)),
            0x0296 => Ok(Self::CMSG_MEETINGSTONE_INFO(CMSG_MEETINGSTONE_INFO::astd_read_body(r, header_size).await?)),
            0x029B => Ok(Self::CMSG_CANCEL_GROWTH_AURA(CMSG_CANCEL_GROWTH_AURA::astd_read_body(r, header_size).await?)),
            0x02A0 => Ok(Self::CMSG_LOOT_ROLL(CMSG_LOOT_ROLL::astd_read_body(r, header_size).await?)),
            0x02A3 => Ok(Self::CMSG_LOOT_MASTER_GIVE(CMSG_LOOT_MASTER_GIVE::astd_read_body(r, header_size).await?)),
            0x02A8 => Ok(Self::CMSG_REPAIR_ITEM(CMSG_REPAIR_ITEM::astd_read_body(r, header_size).await?)),
            0x02AA => Ok(Self::MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Client::astd_read_body(r, header_size).await?)),
            0x02AC => Ok(Self::CMSG_SUMMON_RESPONSE(CMSG_SUMMON_RESPONSE::astd_read_body(r, header_size).await?)),
            0x02B3 => Ok(Self::CMSG_SELF_RES(CMSG_SELF_RES::astd_read_body(r, header_size).await?)),
            0x02B9 => Ok(Self::CMSG_TOGGLE_HELM(CMSG_TOGGLE_HELM::astd_read_body(r, header_size).await?)),
            0x02BA => Ok(Self::CMSG_TOGGLE_CLOAK(CMSG_TOGGLE_CLOAK::astd_read_body(r, header_size).await?)),
            0x02BF => Ok(Self::CMSG_SET_ACTIONBAR_TOGGLES(CMSG_SET_ACTIONBAR_TOGGLES::astd_read_body(r, header_size).await?)),
            0x02C4 => Ok(Self::CMSG_ITEM_NAME_QUERY(CMSG_ITEM_NAME_QUERY::astd_read_body(r, header_size).await?)),
            0x02C7 => Ok(Self::CMSG_CHAR_RENAME(CMSG_CHAR_RENAME::astd_read_body(r, header_size).await?)),
            0x02C9 => Ok(Self::CMSG_MOVE_SPLINE_DONE(CMSG_MOVE_SPLINE_DONE::astd_read_body(r, header_size).await?)),
            0x02CA => Ok(Self::CMSG_MOVE_FALL_RESET(CMSG_MOVE_FALL_RESET::astd_read_body(r, header_size).await?)),
            0x02CD => Ok(Self::CMSG_REQUEST_RAID_INFO(CMSG_REQUEST_RAID_INFO::astd_read_body(r, header_size).await?)),
            0x02CE => Ok(Self::CMSG_MOVE_TIME_SKIPPED(CMSG_MOVE_TIME_SKIPPED::astd_read_body(r, header_size).await?)),
            0x02CF => Ok(Self::CMSG_MOVE_FEATHER_FALL_ACK(CMSG_MOVE_FEATHER_FALL_ACK::astd_read_body(r, header_size).await?)),
            0x02D0 => Ok(Self::CMSG_MOVE_WATER_WALK_ACK(CMSG_MOVE_WATER_WALK_ACK::astd_read_body(r, header_size).await?)),
            0x02D1 => Ok(Self::CMSG_MOVE_NOT_ACTIVE_MOVER(CMSG_MOVE_NOT_ACTIVE_MOVER::astd_read_body(r, header_size).await?)),
            0x02D3 => Ok(Self::CMSG_BATTLEFIELD_STATUS(CMSG_BATTLEFIELD_STATUS::astd_read_body(r, header_size).await?)),
            0x02D5 => Ok(Self::CMSG_BATTLEFIELD_PORT(CMSG_BATTLEFIELD_PORT::astd_read_body(r, header_size).await?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Client::astd_read_body(r, header_size).await?)),
            0x02D7 => Ok(Self::CMSG_BATTLEMASTER_HELLO(CMSG_BATTLEMASTER_HELLO::astd_read_body(r, header_size).await?)),
            0x02DB => Ok(Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(CMSG_FORCE_WALK_SPEED_CHANGE_ACK::astd_read_body(r, header_size).await?)),
            0x02DD => Ok(Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK::astd_read_body(r, header_size).await?)),
            0x02DF => Ok(Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(CMSG_FORCE_TURN_RATE_CHANGE_ACK::astd_read_body(r, header_size).await?)),
            0x02E0 => Ok(Self::MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_Client::astd_read_body(r, header_size).await?)),
            0x02E1 => Ok(Self::CMSG_LEAVE_BATTLEFIELD(CMSG_LEAVE_BATTLEFIELD::astd_read_body(r, header_size).await?)),
            0x02E2 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUERY(CMSG_AREA_SPIRIT_HEALER_QUERY::astd_read_body(r, header_size).await?)),
            0x02E3 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(CMSG_AREA_SPIRIT_HEALER_QUEUE::astd_read_body(r, header_size).await?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Client::astd_read_body(r, header_size).await?)),
            0x02EA => Ok(Self::CMSG_PET_STOP_ATTACK(CMSG_PET_STOP_ATTACK::astd_read_body(r, header_size).await?)),
            0x02EE => Ok(Self::CMSG_BATTLEMASTER_JOIN(CMSG_BATTLEMASTER_JOIN::astd_read_body(r, header_size).await?)),
            0x02F0 => Ok(Self::CMSG_PET_UNLEARN(CMSG_PET_UNLEARN::astd_read_body(r, header_size).await?)),
            0x02F3 => Ok(Self::CMSG_PET_SPELL_AUTOCAST(CMSG_PET_SPELL_AUTOCAST::astd_read_body(r, header_size).await?)),
            0x02FC => Ok(Self::CMSG_GUILD_INFO_TEXT(CMSG_GUILD_INFO_TEXT::astd_read_body(r, header_size).await?)),
            0x0312 => Ok(Self::CMSG_ACTIVATETAXIEXPRESS(CMSG_ACTIVATETAXIEXPRESS::astd_read_body(r, header_size).await?)),
            0x0317 => Ok(Self::CMSG_SET_FACTION_INACTIVE(CMSG_SET_FACTION_INACTIVE::astd_read_body(r, header_size).await?)),
            0x0318 => Ok(Self::CMSG_SET_WATCHED_FACTION(CMSG_SET_WATCHED_FACTION::astd_read_body(r, header_size).await?)),
            0x031D => Ok(Self::CMSG_RESET_INSTANCES(CMSG_RESET_INSTANCES::astd_read_body(r, header_size).await?)),
            0x0321 => Ok(Self::MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Client::astd_read_body(r, header_size).await?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Client::astd_read_body(r, header_size).await?)),
            0x032A => Ok(Self::CMSG_GMSURVEY_SUBMIT(CMSG_GMSURVEY_SUBMIT::astd_read_body(r, header_size).await?)),
            _ => Err(ClientOpcodeMessageError::InvalidOpcode(header.opcode)),
        }
    }

}

#[derive(Debug)]
pub enum ClientOpcodeMessageError {
    Io(std::io::Error),
    InvalidOpcode(u32),
    MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULTError),
    MSG_PETITION_RENAME(MSG_PETITION_RENAMEError),
    CMSG_WORLD_TELEPORT(CMSG_WORLD_TELEPORTError),
    CMSG_CHAR_CREATE(CMSG_CHAR_CREATEError),
    CMSG_WHO(CMSG_WHOError),
    CMSG_WHOIS(CMSG_WHOISError),
    CMSG_ADD_FRIEND(CMSG_ADD_FRIENDError),
    CMSG_ADD_IGNORE(CMSG_ADD_IGNOREError),
    CMSG_GROUP_INVITE(CMSG_GROUP_INVITEError),
    CMSG_GROUP_UNINVITE(CMSG_GROUP_UNINVITEError),
    CMSG_LOOT_METHOD(CMSG_LOOT_METHODError),
    CMSG_GUILD_CREATE(CMSG_GUILD_CREATEError),
    CMSG_GUILD_INVITE(CMSG_GUILD_INVITEError),
    CMSG_GUILD_PROMOTE(CMSG_GUILD_PROMOTEError),
    CMSG_GUILD_DEMOTE(CMSG_GUILD_DEMOTEError),
    CMSG_GUILD_REMOVE(CMSG_GUILD_REMOVEError),
    CMSG_GUILD_LEADER(CMSG_GUILD_LEADERError),
    CMSG_GUILD_MOTD(CMSG_GUILD_MOTDError),
    CMSG_MESSAGECHAT(CMSG_MESSAGECHATError),
    CMSG_JOIN_CHANNEL(CMSG_JOIN_CHANNELError),
    CMSG_LEAVE_CHANNEL(CMSG_LEAVE_CHANNELError),
    CMSG_CHANNEL_LIST(CMSG_CHANNEL_LISTError),
    CMSG_CHANNEL_PASSWORD(CMSG_CHANNEL_PASSWORDError),
    CMSG_CHANNEL_SET_OWNER(CMSG_CHANNEL_SET_OWNERError),
    CMSG_CHANNEL_OWNER(CMSG_CHANNEL_OWNERError),
    CMSG_CHANNEL_MODERATOR(CMSG_CHANNEL_MODERATORError),
    CMSG_CHANNEL_UNMODERATOR(CMSG_CHANNEL_UNMODERATORError),
    CMSG_CHANNEL_MUTE(CMSG_CHANNEL_MUTEError),
    CMSG_CHANNEL_UNMUTE(CMSG_CHANNEL_UNMUTEError),
    CMSG_CHANNEL_INVITE(CMSG_CHANNEL_INVITEError),
    CMSG_CHANNEL_KICK(CMSG_CHANNEL_KICKError),
    CMSG_CHANNEL_BAN(CMSG_CHANNEL_BANError),
    CMSG_CHANNEL_UNBAN(CMSG_CHANNEL_UNBANError),
    CMSG_CHANNEL_ANNOUNCEMENTS(CMSG_CHANNEL_ANNOUNCEMENTSError),
    CMSG_CHANNEL_MODERATE(CMSG_CHANNEL_MODERATEError),
    SMSG_UPDATE_OBJECT(SMSG_UPDATE_OBJECTError),
    CMSG_USE_ITEM(CMSG_USE_ITEMError),
    CMSG_STANDSTATECHANGE(CMSG_STANDSTATECHANGEError),
    CMSG_EMOTE(CMSG_EMOTEError),
    CMSG_TEXT_EMOTE(CMSG_TEXT_EMOTEError),
    CMSG_CAST_SPELL(CMSG_CAST_SPELLError),
    CMSG_PET_RENAME(CMSG_PET_RENAMEError),
    CMSG_GOSSIP_SELECT_OPTION(CMSG_GOSSIP_SELECT_OPTIONError),
    CMSG_PETITION_BUY(CMSG_PETITION_BUYError),
    CMSG_BUG(CMSG_BUGError),
    CMSG_SETSHEATHED(CMSG_SETSHEATHEDError),
    CMSG_AUTH_SESSION(CMSG_AUTH_SESSIONError),
    CMSG_GMTICKET_CREATE(CMSG_GMTICKET_CREATEError),
    CMSG_GMTICKET_UPDATETEXT(CMSG_GMTICKET_UPDATETEXTError),
    CMSG_GUILD_RANK(CMSG_GUILD_RANKError),
    CMSG_GUILD_ADD_RANK(CMSG_GUILD_ADD_RANKError),
    CMSG_GUILD_SET_PUBLIC_NOTE(CMSG_GUILD_SET_PUBLIC_NOTEError),
    CMSG_GUILD_SET_OFFICER_NOTE(CMSG_GUILD_SET_OFFICER_NOTEError),
    CMSG_SEND_MAIL(CMSG_SEND_MAILError),
    CMSG_BATTLEFIELD_LIST(CMSG_BATTLEFIELD_LISTError),
    CMSG_BATTLEFIELD_JOIN(CMSG_BATTLEFIELD_JOINError),
    CMSG_AUCTION_LIST_ITEMS(CMSG_AUCTION_LIST_ITEMSError),
    CMSG_FAR_SIGHT(CMSG_FAR_SIGHTError),
    CMSG_GROUP_CHANGE_SUB_GROUP(CMSG_GROUP_CHANGE_SUB_GROUPError),
    CMSG_GROUP_SWAP_SUB_GROUP(CMSG_GROUP_SWAP_SUB_GROUPError),
    CMSG_BUYBACK_ITEM(CMSG_BUYBACK_ITEMError),
    CMSG_LOOT_ROLL(CMSG_LOOT_ROLLError),
    CMSG_CHAR_RENAME(CMSG_CHAR_RENAMEError),
    CMSG_BATTLEFIELD_PORT(CMSG_BATTLEFIELD_PORTError),
    CMSG_BATTLEMASTER_JOIN(CMSG_BATTLEMASTER_JOINError),
    CMSG_GUILD_INFO_TEXT(CMSG_GUILD_INFO_TEXTError),
    MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_ClientError),
    CMSG_GMSURVEY_SUBMIT(CMSG_GMSURVEY_SUBMITError),
}

impl std::error::Error for ClientOpcodeMessageError {}
impl std::fmt::Display for ClientOpcodeMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode received for ClientMessage: '{}'", i)),
            Self::MSG_QUEST_PUSH_RESULT(i) => i.fmt(f),
            Self::MSG_PETITION_RENAME(i) => i.fmt(f),
            Self::CMSG_WORLD_TELEPORT(i) => i.fmt(f),
            Self::CMSG_CHAR_CREATE(i) => i.fmt(f),
            Self::CMSG_WHO(i) => i.fmt(f),
            Self::CMSG_WHOIS(i) => i.fmt(f),
            Self::CMSG_ADD_FRIEND(i) => i.fmt(f),
            Self::CMSG_ADD_IGNORE(i) => i.fmt(f),
            Self::CMSG_GROUP_INVITE(i) => i.fmt(f),
            Self::CMSG_GROUP_UNINVITE(i) => i.fmt(f),
            Self::CMSG_LOOT_METHOD(i) => i.fmt(f),
            Self::CMSG_GUILD_CREATE(i) => i.fmt(f),
            Self::CMSG_GUILD_INVITE(i) => i.fmt(f),
            Self::CMSG_GUILD_PROMOTE(i) => i.fmt(f),
            Self::CMSG_GUILD_DEMOTE(i) => i.fmt(f),
            Self::CMSG_GUILD_REMOVE(i) => i.fmt(f),
            Self::CMSG_GUILD_LEADER(i) => i.fmt(f),
            Self::CMSG_GUILD_MOTD(i) => i.fmt(f),
            Self::CMSG_MESSAGECHAT(i) => i.fmt(f),
            Self::CMSG_JOIN_CHANNEL(i) => i.fmt(f),
            Self::CMSG_LEAVE_CHANNEL(i) => i.fmt(f),
            Self::CMSG_CHANNEL_LIST(i) => i.fmt(f),
            Self::CMSG_CHANNEL_PASSWORD(i) => i.fmt(f),
            Self::CMSG_CHANNEL_SET_OWNER(i) => i.fmt(f),
            Self::CMSG_CHANNEL_OWNER(i) => i.fmt(f),
            Self::CMSG_CHANNEL_MODERATOR(i) => i.fmt(f),
            Self::CMSG_CHANNEL_UNMODERATOR(i) => i.fmt(f),
            Self::CMSG_CHANNEL_MUTE(i) => i.fmt(f),
            Self::CMSG_CHANNEL_UNMUTE(i) => i.fmt(f),
            Self::CMSG_CHANNEL_INVITE(i) => i.fmt(f),
            Self::CMSG_CHANNEL_KICK(i) => i.fmt(f),
            Self::CMSG_CHANNEL_BAN(i) => i.fmt(f),
            Self::CMSG_CHANNEL_UNBAN(i) => i.fmt(f),
            Self::CMSG_CHANNEL_ANNOUNCEMENTS(i) => i.fmt(f),
            Self::CMSG_CHANNEL_MODERATE(i) => i.fmt(f),
            Self::SMSG_UPDATE_OBJECT(i) => i.fmt(f),
            Self::CMSG_USE_ITEM(i) => i.fmt(f),
            Self::CMSG_STANDSTATECHANGE(i) => i.fmt(f),
            Self::CMSG_EMOTE(i) => i.fmt(f),
            Self::CMSG_TEXT_EMOTE(i) => i.fmt(f),
            Self::CMSG_CAST_SPELL(i) => i.fmt(f),
            Self::CMSG_PET_RENAME(i) => i.fmt(f),
            Self::CMSG_GOSSIP_SELECT_OPTION(i) => i.fmt(f),
            Self::CMSG_PETITION_BUY(i) => i.fmt(f),
            Self::CMSG_BUG(i) => i.fmt(f),
            Self::CMSG_SETSHEATHED(i) => i.fmt(f),
            Self::CMSG_AUTH_SESSION(i) => i.fmt(f),
            Self::CMSG_GMTICKET_CREATE(i) => i.fmt(f),
            Self::CMSG_GMTICKET_UPDATETEXT(i) => i.fmt(f),
            Self::CMSG_GUILD_RANK(i) => i.fmt(f),
            Self::CMSG_GUILD_ADD_RANK(i) => i.fmt(f),
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(i) => i.fmt(f),
            Self::CMSG_GUILD_SET_OFFICER_NOTE(i) => i.fmt(f),
            Self::CMSG_SEND_MAIL(i) => i.fmt(f),
            Self::CMSG_BATTLEFIELD_LIST(i) => i.fmt(f),
            Self::CMSG_BATTLEFIELD_JOIN(i) => i.fmt(f),
            Self::CMSG_AUCTION_LIST_ITEMS(i) => i.fmt(f),
            Self::CMSG_FAR_SIGHT(i) => i.fmt(f),
            Self::CMSG_GROUP_CHANGE_SUB_GROUP(i) => i.fmt(f),
            Self::CMSG_GROUP_SWAP_SUB_GROUP(i) => i.fmt(f),
            Self::CMSG_BUYBACK_ITEM(i) => i.fmt(f),
            Self::CMSG_LOOT_ROLL(i) => i.fmt(f),
            Self::CMSG_CHAR_RENAME(i) => i.fmt(f),
            Self::CMSG_BATTLEFIELD_PORT(i) => i.fmt(f),
            Self::CMSG_BATTLEMASTER_JOIN(i) => i.fmt(f),
            Self::CMSG_GUILD_INFO_TEXT(i) => i.fmt(f),
            Self::MSG_RAID_TARGET_UPDATE(i) => i.fmt(f),
            Self::CMSG_GMSURVEY_SUBMIT(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for ClientOpcodeMessageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MSG_QUEST_PUSH_RESULTError> for ClientOpcodeMessageError {
    fn from(e: MSG_QUEST_PUSH_RESULTError) -> Self {
        match e {
            MSG_QUEST_PUSH_RESULTError::Io(i) => Self::Io(i),
            _ => Self::MSG_QUEST_PUSH_RESULT(e),
        }
    }
}

impl From<MSG_PETITION_RENAMEError> for ClientOpcodeMessageError {
    fn from(e: MSG_PETITION_RENAMEError) -> Self {
        match e {
            MSG_PETITION_RENAMEError::Io(i) => Self::Io(i),
            _ => Self::MSG_PETITION_RENAME(e),
        }
    }
}

impl From<CMSG_WORLD_TELEPORTError> for ClientOpcodeMessageError {
    fn from(e: CMSG_WORLD_TELEPORTError) -> Self {
        match e {
            CMSG_WORLD_TELEPORTError::Io(i) => Self::Io(i),
            _ => Self::CMSG_WORLD_TELEPORT(e),
        }
    }
}

impl From<CMSG_CHAR_CREATEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CHAR_CREATEError) -> Self {
        match e {
            CMSG_CHAR_CREATEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CHAR_CREATE(e),
        }
    }
}

impl From<CMSG_WHOError> for ClientOpcodeMessageError {
    fn from(e: CMSG_WHOError) -> Self {
        match e {
            CMSG_WHOError::Io(i) => Self::Io(i),
            _ => Self::CMSG_WHO(e),
        }
    }
}

impl From<CMSG_WHOISError> for ClientOpcodeMessageError {
    fn from(e: CMSG_WHOISError) -> Self {
        match e {
            CMSG_WHOISError::Io(i) => Self::Io(i),
            _ => Self::CMSG_WHOIS(e),
        }
    }
}

impl From<CMSG_ADD_FRIENDError> for ClientOpcodeMessageError {
    fn from(e: CMSG_ADD_FRIENDError) -> Self {
        match e {
            CMSG_ADD_FRIENDError::Io(i) => Self::Io(i),
            _ => Self::CMSG_ADD_FRIEND(e),
        }
    }
}

impl From<CMSG_ADD_IGNOREError> for ClientOpcodeMessageError {
    fn from(e: CMSG_ADD_IGNOREError) -> Self {
        match e {
            CMSG_ADD_IGNOREError::Io(i) => Self::Io(i),
            _ => Self::CMSG_ADD_IGNORE(e),
        }
    }
}

impl From<CMSG_GROUP_INVITEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GROUP_INVITEError) -> Self {
        match e {
            CMSG_GROUP_INVITEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GROUP_INVITE(e),
        }
    }
}

impl From<CMSG_GROUP_UNINVITEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GROUP_UNINVITEError) -> Self {
        match e {
            CMSG_GROUP_UNINVITEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GROUP_UNINVITE(e),
        }
    }
}

impl From<CMSG_LOOT_METHODError> for ClientOpcodeMessageError {
    fn from(e: CMSG_LOOT_METHODError) -> Self {
        match e {
            CMSG_LOOT_METHODError::Io(i) => Self::Io(i),
            _ => Self::CMSG_LOOT_METHOD(e),
        }
    }
}

impl From<CMSG_GUILD_CREATEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GUILD_CREATEError) -> Self {
        match e {
            CMSG_GUILD_CREATEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GUILD_CREATE(e),
        }
    }
}

impl From<CMSG_GUILD_INVITEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GUILD_INVITEError) -> Self {
        match e {
            CMSG_GUILD_INVITEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GUILD_INVITE(e),
        }
    }
}

impl From<CMSG_GUILD_PROMOTEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GUILD_PROMOTEError) -> Self {
        match e {
            CMSG_GUILD_PROMOTEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GUILD_PROMOTE(e),
        }
    }
}

impl From<CMSG_GUILD_DEMOTEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GUILD_DEMOTEError) -> Self {
        match e {
            CMSG_GUILD_DEMOTEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GUILD_DEMOTE(e),
        }
    }
}

impl From<CMSG_GUILD_REMOVEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GUILD_REMOVEError) -> Self {
        match e {
            CMSG_GUILD_REMOVEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GUILD_REMOVE(e),
        }
    }
}

impl From<CMSG_GUILD_LEADERError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GUILD_LEADERError) -> Self {
        match e {
            CMSG_GUILD_LEADERError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GUILD_LEADER(e),
        }
    }
}

impl From<CMSG_GUILD_MOTDError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GUILD_MOTDError) -> Self {
        match e {
            CMSG_GUILD_MOTDError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GUILD_MOTD(e),
        }
    }
}

impl From<CMSG_MESSAGECHATError> for ClientOpcodeMessageError {
    fn from(e: CMSG_MESSAGECHATError) -> Self {
        match e {
            CMSG_MESSAGECHATError::Io(i) => Self::Io(i),
            _ => Self::CMSG_MESSAGECHAT(e),
        }
    }
}

impl From<CMSG_JOIN_CHANNELError> for ClientOpcodeMessageError {
    fn from(e: CMSG_JOIN_CHANNELError) -> Self {
        match e {
            CMSG_JOIN_CHANNELError::Io(i) => Self::Io(i),
            _ => Self::CMSG_JOIN_CHANNEL(e),
        }
    }
}

impl From<CMSG_LEAVE_CHANNELError> for ClientOpcodeMessageError {
    fn from(e: CMSG_LEAVE_CHANNELError) -> Self {
        match e {
            CMSG_LEAVE_CHANNELError::Io(i) => Self::Io(i),
            _ => Self::CMSG_LEAVE_CHANNEL(e),
        }
    }
}

impl From<CMSG_CHANNEL_LISTError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CHANNEL_LISTError) -> Self {
        match e {
            CMSG_CHANNEL_LISTError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CHANNEL_LIST(e),
        }
    }
}

impl From<CMSG_CHANNEL_PASSWORDError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CHANNEL_PASSWORDError) -> Self {
        match e {
            CMSG_CHANNEL_PASSWORDError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CHANNEL_PASSWORD(e),
        }
    }
}

impl From<CMSG_CHANNEL_SET_OWNERError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CHANNEL_SET_OWNERError) -> Self {
        match e {
            CMSG_CHANNEL_SET_OWNERError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CHANNEL_SET_OWNER(e),
        }
    }
}

impl From<CMSG_CHANNEL_OWNERError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CHANNEL_OWNERError) -> Self {
        match e {
            CMSG_CHANNEL_OWNERError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CHANNEL_OWNER(e),
        }
    }
}

impl From<CMSG_CHANNEL_MODERATORError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CHANNEL_MODERATORError) -> Self {
        match e {
            CMSG_CHANNEL_MODERATORError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CHANNEL_MODERATOR(e),
        }
    }
}

impl From<CMSG_CHANNEL_UNMODERATORError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CHANNEL_UNMODERATORError) -> Self {
        match e {
            CMSG_CHANNEL_UNMODERATORError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CHANNEL_UNMODERATOR(e),
        }
    }
}

impl From<CMSG_CHANNEL_MUTEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CHANNEL_MUTEError) -> Self {
        match e {
            CMSG_CHANNEL_MUTEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CHANNEL_MUTE(e),
        }
    }
}

impl From<CMSG_CHANNEL_UNMUTEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CHANNEL_UNMUTEError) -> Self {
        match e {
            CMSG_CHANNEL_UNMUTEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CHANNEL_UNMUTE(e),
        }
    }
}

impl From<CMSG_CHANNEL_INVITEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CHANNEL_INVITEError) -> Self {
        match e {
            CMSG_CHANNEL_INVITEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CHANNEL_INVITE(e),
        }
    }
}

impl From<CMSG_CHANNEL_KICKError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CHANNEL_KICKError) -> Self {
        match e {
            CMSG_CHANNEL_KICKError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CHANNEL_KICK(e),
        }
    }
}

impl From<CMSG_CHANNEL_BANError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CHANNEL_BANError) -> Self {
        match e {
            CMSG_CHANNEL_BANError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CHANNEL_BAN(e),
        }
    }
}

impl From<CMSG_CHANNEL_UNBANError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CHANNEL_UNBANError) -> Self {
        match e {
            CMSG_CHANNEL_UNBANError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CHANNEL_UNBAN(e),
        }
    }
}

impl From<CMSG_CHANNEL_ANNOUNCEMENTSError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CHANNEL_ANNOUNCEMENTSError) -> Self {
        match e {
            CMSG_CHANNEL_ANNOUNCEMENTSError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CHANNEL_ANNOUNCEMENTS(e),
        }
    }
}

impl From<CMSG_CHANNEL_MODERATEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CHANNEL_MODERATEError) -> Self {
        match e {
            CMSG_CHANNEL_MODERATEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CHANNEL_MODERATE(e),
        }
    }
}

impl From<SMSG_UPDATE_OBJECTError> for ClientOpcodeMessageError {
    fn from(e: SMSG_UPDATE_OBJECTError) -> Self {
        match e {
            SMSG_UPDATE_OBJECTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_UPDATE_OBJECT(e),
        }
    }
}

impl From<CMSG_USE_ITEMError> for ClientOpcodeMessageError {
    fn from(e: CMSG_USE_ITEMError) -> Self {
        match e {
            CMSG_USE_ITEMError::Io(i) => Self::Io(i),
            _ => Self::CMSG_USE_ITEM(e),
        }
    }
}

impl From<CMSG_STANDSTATECHANGEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_STANDSTATECHANGEError) -> Self {
        match e {
            CMSG_STANDSTATECHANGEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_STANDSTATECHANGE(e),
        }
    }
}

impl From<CMSG_EMOTEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_EMOTEError) -> Self {
        match e {
            CMSG_EMOTEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_EMOTE(e),
        }
    }
}

impl From<CMSG_TEXT_EMOTEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_TEXT_EMOTEError) -> Self {
        match e {
            CMSG_TEXT_EMOTEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_TEXT_EMOTE(e),
        }
    }
}

impl From<CMSG_CAST_SPELLError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CAST_SPELLError) -> Self {
        match e {
            CMSG_CAST_SPELLError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CAST_SPELL(e),
        }
    }
}

impl From<CMSG_PET_RENAMEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_PET_RENAMEError) -> Self {
        match e {
            CMSG_PET_RENAMEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_PET_RENAME(e),
        }
    }
}

impl From<CMSG_GOSSIP_SELECT_OPTIONError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GOSSIP_SELECT_OPTIONError) -> Self {
        match e {
            CMSG_GOSSIP_SELECT_OPTIONError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GOSSIP_SELECT_OPTION(e),
        }
    }
}

impl From<CMSG_PETITION_BUYError> for ClientOpcodeMessageError {
    fn from(e: CMSG_PETITION_BUYError) -> Self {
        match e {
            CMSG_PETITION_BUYError::Io(i) => Self::Io(i),
            _ => Self::CMSG_PETITION_BUY(e),
        }
    }
}

impl From<CMSG_BUGError> for ClientOpcodeMessageError {
    fn from(e: CMSG_BUGError) -> Self {
        match e {
            CMSG_BUGError::Io(i) => Self::Io(i),
            _ => Self::CMSG_BUG(e),
        }
    }
}

impl From<CMSG_SETSHEATHEDError> for ClientOpcodeMessageError {
    fn from(e: CMSG_SETSHEATHEDError) -> Self {
        match e {
            CMSG_SETSHEATHEDError::Io(i) => Self::Io(i),
            _ => Self::CMSG_SETSHEATHED(e),
        }
    }
}

impl From<CMSG_AUTH_SESSIONError> for ClientOpcodeMessageError {
    fn from(e: CMSG_AUTH_SESSIONError) -> Self {
        match e {
            CMSG_AUTH_SESSIONError::Io(i) => Self::Io(i),
            _ => Self::CMSG_AUTH_SESSION(e),
        }
    }
}

impl From<CMSG_GMTICKET_CREATEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GMTICKET_CREATEError) -> Self {
        match e {
            CMSG_GMTICKET_CREATEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GMTICKET_CREATE(e),
        }
    }
}

impl From<CMSG_GMTICKET_UPDATETEXTError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GMTICKET_UPDATETEXTError) -> Self {
        match e {
            CMSG_GMTICKET_UPDATETEXTError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GMTICKET_UPDATETEXT(e),
        }
    }
}

impl From<CMSG_GUILD_RANKError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GUILD_RANKError) -> Self {
        match e {
            CMSG_GUILD_RANKError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GUILD_RANK(e),
        }
    }
}

impl From<CMSG_GUILD_ADD_RANKError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GUILD_ADD_RANKError) -> Self {
        match e {
            CMSG_GUILD_ADD_RANKError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GUILD_ADD_RANK(e),
        }
    }
}

impl From<CMSG_GUILD_SET_PUBLIC_NOTEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GUILD_SET_PUBLIC_NOTEError) -> Self {
        match e {
            CMSG_GUILD_SET_PUBLIC_NOTEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GUILD_SET_PUBLIC_NOTE(e),
        }
    }
}

impl From<CMSG_GUILD_SET_OFFICER_NOTEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GUILD_SET_OFFICER_NOTEError) -> Self {
        match e {
            CMSG_GUILD_SET_OFFICER_NOTEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GUILD_SET_OFFICER_NOTE(e),
        }
    }
}

impl From<CMSG_SEND_MAILError> for ClientOpcodeMessageError {
    fn from(e: CMSG_SEND_MAILError) -> Self {
        match e {
            CMSG_SEND_MAILError::Io(i) => Self::Io(i),
            _ => Self::CMSG_SEND_MAIL(e),
        }
    }
}

impl From<CMSG_BATTLEFIELD_LISTError> for ClientOpcodeMessageError {
    fn from(e: CMSG_BATTLEFIELD_LISTError) -> Self {
        match e {
            CMSG_BATTLEFIELD_LISTError::Io(i) => Self::Io(i),
            _ => Self::CMSG_BATTLEFIELD_LIST(e),
        }
    }
}

impl From<CMSG_BATTLEFIELD_JOINError> for ClientOpcodeMessageError {
    fn from(e: CMSG_BATTLEFIELD_JOINError) -> Self {
        match e {
            CMSG_BATTLEFIELD_JOINError::Io(i) => Self::Io(i),
            _ => Self::CMSG_BATTLEFIELD_JOIN(e),
        }
    }
}

impl From<CMSG_AUCTION_LIST_ITEMSError> for ClientOpcodeMessageError {
    fn from(e: CMSG_AUCTION_LIST_ITEMSError) -> Self {
        match e {
            CMSG_AUCTION_LIST_ITEMSError::Io(i) => Self::Io(i),
            _ => Self::CMSG_AUCTION_LIST_ITEMS(e),
        }
    }
}

impl From<CMSG_FAR_SIGHTError> for ClientOpcodeMessageError {
    fn from(e: CMSG_FAR_SIGHTError) -> Self {
        match e {
            CMSG_FAR_SIGHTError::Io(i) => Self::Io(i),
            _ => Self::CMSG_FAR_SIGHT(e),
        }
    }
}

impl From<CMSG_GROUP_CHANGE_SUB_GROUPError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GROUP_CHANGE_SUB_GROUPError) -> Self {
        match e {
            CMSG_GROUP_CHANGE_SUB_GROUPError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GROUP_CHANGE_SUB_GROUP(e),
        }
    }
}

impl From<CMSG_GROUP_SWAP_SUB_GROUPError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GROUP_SWAP_SUB_GROUPError) -> Self {
        match e {
            CMSG_GROUP_SWAP_SUB_GROUPError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GROUP_SWAP_SUB_GROUP(e),
        }
    }
}

impl From<CMSG_BUYBACK_ITEMError> for ClientOpcodeMessageError {
    fn from(e: CMSG_BUYBACK_ITEMError) -> Self {
        match e {
            CMSG_BUYBACK_ITEMError::Io(i) => Self::Io(i),
            _ => Self::CMSG_BUYBACK_ITEM(e),
        }
    }
}

impl From<CMSG_LOOT_ROLLError> for ClientOpcodeMessageError {
    fn from(e: CMSG_LOOT_ROLLError) -> Self {
        match e {
            CMSG_LOOT_ROLLError::Io(i) => Self::Io(i),
            _ => Self::CMSG_LOOT_ROLL(e),
        }
    }
}

impl From<CMSG_CHAR_RENAMEError> for ClientOpcodeMessageError {
    fn from(e: CMSG_CHAR_RENAMEError) -> Self {
        match e {
            CMSG_CHAR_RENAMEError::Io(i) => Self::Io(i),
            _ => Self::CMSG_CHAR_RENAME(e),
        }
    }
}

impl From<CMSG_BATTLEFIELD_PORTError> for ClientOpcodeMessageError {
    fn from(e: CMSG_BATTLEFIELD_PORTError) -> Self {
        match e {
            CMSG_BATTLEFIELD_PORTError::Io(i) => Self::Io(i),
            _ => Self::CMSG_BATTLEFIELD_PORT(e),
        }
    }
}

impl From<CMSG_BATTLEMASTER_JOINError> for ClientOpcodeMessageError {
    fn from(e: CMSG_BATTLEMASTER_JOINError) -> Self {
        match e {
            CMSG_BATTLEMASTER_JOINError::Io(i) => Self::Io(i),
            _ => Self::CMSG_BATTLEMASTER_JOIN(e),
        }
    }
}

impl From<CMSG_GUILD_INFO_TEXTError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GUILD_INFO_TEXTError) -> Self {
        match e {
            CMSG_GUILD_INFO_TEXTError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GUILD_INFO_TEXT(e),
        }
    }
}

impl From<MSG_RAID_TARGET_UPDATE_ClientError> for ClientOpcodeMessageError {
    fn from(e: MSG_RAID_TARGET_UPDATE_ClientError) -> Self {
        match e {
            MSG_RAID_TARGET_UPDATE_ClientError::Io(i) => Self::Io(i),
            _ => Self::MSG_RAID_TARGET_UPDATE(e),
        }
    }
}

impl From<CMSG_GMSURVEY_SUBMITError> for ClientOpcodeMessageError {
    fn from(e: CMSG_GMSURVEY_SUBMITError) -> Self {
        match e {
            CMSG_GMSURVEY_SUBMITError::Io(i) => Self::Io(i),
            _ => Self::CMSG_GMSURVEY_SUBMIT(e),
        }
    }
}

use crate::world::v1::v12::{SMSG_CHAR_CREATE, SMSG_CHAR_CREATEError};
use crate::world::v1::v12::{SMSG_CHAR_ENUM, SMSG_CHAR_ENUMError};
use crate::world::v1::v12::{SMSG_CHAR_DELETE, SMSG_CHAR_DELETEError};
use crate::world::v1::v12::SMSG_NEW_WORLD;
use crate::world::v1::v12::{SMSG_TRANSFER_PENDING, SMSG_TRANSFER_PENDINGError};
use crate::world::v1::v12::{SMSG_TRANSFER_ABORTED, SMSG_TRANSFER_ABORTEDError};
use crate::world::v1::v12::{SMSG_CHARACTER_LOGIN_FAILED, SMSG_CHARACTER_LOGIN_FAILEDError};
use crate::world::v1::v12::SMSG_LOGIN_SETTIMESPEED;
use crate::world::v1::v12::{SMSG_LOGOUT_RESPONSE, SMSG_LOGOUT_RESPONSEError};
use crate::world::v1::v12::SMSG_LOGOUT_COMPLETE;
use crate::world::v1::v12::SMSG_LOGOUT_CANCEL_ACK;
use crate::world::v1::v12::{SMSG_NAME_QUERY_RESPONSE, SMSG_NAME_QUERY_RESPONSEError};
use crate::world::v1::v12::{SMSG_PET_NAME_QUERY_RESPONSE, SMSG_PET_NAME_QUERY_RESPONSEError};
use crate::world::v1::v12::{SMSG_GUILD_QUERY_RESPONSE, SMSG_GUILD_QUERY_RESPONSEError};
use crate::world::v1::v12::{SMSG_ITEM_QUERY_SINGLE_RESPONSE, SMSG_ITEM_QUERY_SINGLE_RESPONSEError};
use crate::world::v1::v12::{SMSG_PAGE_TEXT_QUERY_RESPONSE, SMSG_PAGE_TEXT_QUERY_RESPONSEError};
use crate::world::v1::v12::{SMSG_QUEST_QUERY_RESPONSE, SMSG_QUEST_QUERY_RESPONSEError};
use crate::world::v1::v12::{SMSG_GAMEOBJECT_QUERY_RESPONSE, SMSG_GAMEOBJECT_QUERY_RESPONSEError};
use crate::world::v1::v12::{SMSG_CREATURE_QUERY_RESPONSE, SMSG_CREATURE_QUERY_RESPONSEError};
use crate::world::v1::v12::{SMSG_WHO, SMSG_WHOError};
use crate::world::v1::v12::{SMSG_WHOIS, SMSG_WHOISError};
use crate::world::v1::v12::{SMSG_FRIEND_LIST, SMSG_FRIEND_LISTError};
use crate::world::v1::v12::{SMSG_FRIEND_STATUS, SMSG_FRIEND_STATUSError};
use crate::world::v1::v12::SMSG_IGNORE_LIST;
use crate::world::v1::v12::{SMSG_GROUP_INVITE, SMSG_GROUP_INVITEError};
use crate::world::v1::v12::{SMSG_GROUP_DECLINE, SMSG_GROUP_DECLINEError};
use crate::world::v1::v12::SMSG_GROUP_UNINVITE;
use crate::world::v1::v12::{SMSG_GROUP_SET_LEADER, SMSG_GROUP_SET_LEADERError};
use crate::world::v1::v12::SMSG_GROUP_DESTROYED;
use crate::world::v1::v12::{SMSG_GROUP_LIST, SMSG_GROUP_LISTError};
use crate::world::v1::v12::{SMSG_PARTY_MEMBER_STATS, SMSG_PARTY_MEMBER_STATSError};
use crate::world::v1::v12::{SMSG_PARTY_COMMAND_RESULT, SMSG_PARTY_COMMAND_RESULTError};
use crate::world::v1::v12::{SMSG_GUILD_INVITE, SMSG_GUILD_INVITEError};
use crate::world::v1::v12::{SMSG_GUILD_INFO, SMSG_GUILD_INFOError};
use crate::world::v1::v12::SMSG_GUILD_ROSTER;
use crate::world::v1::v12::{SMSG_GUILD_EVENT, SMSG_GUILD_EVENTError};
use crate::world::v1::v12::{SMSG_GUILD_COMMAND_RESULT, SMSG_GUILD_COMMAND_RESULTError};
use crate::world::v1::v12::{SMSG_MESSAGECHAT, SMSG_MESSAGECHATError};
use crate::world::v1::v12::{SMSG_CHANNEL_NOTIFY, SMSG_CHANNEL_NOTIFYError};
use crate::world::v1::v12::{SMSG_CHANNEL_LIST, SMSG_CHANNEL_LISTError};
use crate::world::v1::v12::SMSG_DESTROY_OBJECT;
use crate::world::v1::v12::SMSG_READ_ITEM_OK;
use crate::world::v1::v12::SMSG_READ_ITEM_FAILED;
use crate::world::v1::v12::SMSG_ITEM_COOLDOWN;
use crate::world::v1::v12::SMSG_GAMEOBJECT_CUSTOM_ANIM;
use crate::world::v1::v12::{SMSG_MONSTER_MOVE, SMSG_MONSTER_MOVEError};
use crate::world::v1::v12::SMSG_MOVE_WATER_WALK;
use crate::world::v1::v12::SMSG_MOVE_LAND_WALK;
use crate::world::v1::v12::SMSG_FORCE_RUN_SPEED_CHANGE;
use crate::world::v1::v12::SMSG_FORCE_RUN_BACK_SPEED_CHANGE;
use crate::world::v1::v12::SMSG_FORCE_SWIM_SPEED_CHANGE;
use crate::world::v1::v12::SMSG_FORCE_MOVE_ROOT;
use crate::world::v1::v12::SMSG_FORCE_MOVE_UNROOT;
use crate::world::v1::v12::SMSG_MOVE_KNOCK_BACK;
use crate::world::v1::v12::SMSG_MOVE_FEATHER_FALL;
use crate::world::v1::v12::SMSG_MOVE_NORMAL_FALL;
use crate::world::v1::v12::SMSG_MOVE_SET_HOVER;
use crate::world::v1::v12::SMSG_MOVE_UNSET_HOVER;
use crate::world::v1::v12::{SMSG_TRIGGER_CINEMATIC, SMSG_TRIGGER_CINEMATICError};
use crate::world::v1::v12::SMSG_TUTORIAL_FLAGS;
use crate::world::v1::v12::{SMSG_EMOTE, SMSG_EMOTEError};
use crate::world::v1::v12::{SMSG_TEXT_EMOTE, SMSG_TEXT_EMOTEError};
use crate::world::v1::v12::{SMSG_INVENTORY_CHANGE_FAILURE, SMSG_INVENTORY_CHANGE_FAILUREError};
use crate::world::v1::v12::SMSG_OPEN_CONTAINER;
use crate::world::v1::v12::SMSG_INSPECT;
use crate::world::v1::v12::{SMSG_TRADE_STATUS, SMSG_TRADE_STATUSError};
use crate::world::v1::v12::SMSG_TRADE_STATUS_EXTENDED;
use crate::world::v1::v12::SMSG_INITIALIZE_FACTIONS;
use crate::world::v1::v12::SMSG_SET_FACTION_VISIBLE;
use crate::world::v1::v12::SMSG_SET_FACTION_STANDING;
use crate::world::v1::v12::{SMSG_SET_PROFICIENCY, SMSG_SET_PROFICIENCYError};
use crate::world::v1::v12::SMSG_ACTION_BUTTONS;
use crate::world::v1::v12::SMSG_INITIAL_SPELLS;
use crate::world::v1::v12::SMSG_LEARNED_SPELL;
use crate::world::v1::v12::SMSG_SUPERCEDED_SPELL;
use crate::world::v1::v12::{SMSG_CAST_RESULT, SMSG_CAST_RESULTError};
use crate::world::v1::v12::{SMSG_SPELL_START, SMSG_SPELL_STARTError};
use crate::world::v1::v12::{SMSG_SPELL_GO, SMSG_SPELL_GOError};
use crate::world::v1::v12::{SMSG_SPELL_FAILURE, SMSG_SPELL_FAILUREError};
use crate::world::v1::v12::SMSG_SPELL_COOLDOWN;
use crate::world::v1::v12::SMSG_COOLDOWN_EVENT;
use crate::world::v1::v12::SMSG_UPDATE_AURA_DURATION;
use crate::world::v1::v12::{SMSG_PET_CAST_FAILED, SMSG_PET_CAST_FAILEDError};
use crate::world::v1::v12::{SMSG_AI_REACTION, SMSG_AI_REACTIONError};
use crate::world::v1::v12::SMSG_ATTACKSTART;
use crate::world::v1::v12::SMSG_ATTACKSTOP;
use crate::world::v1::v12::SMSG_ATTACKSWING_NOTINRANGE;
use crate::world::v1::v12::SMSG_ATTACKSWING_BADFACING;
use crate::world::v1::v12::SMSG_ATTACKSWING_NOTSTANDING;
use crate::world::v1::v12::SMSG_ATTACKSWING_DEADTARGET;
use crate::world::v1::v12::SMSG_ATTACKSWING_CANT_ATTACK;
use crate::world::v1::v12::SMSG_ATTACKERSTATEUPDATE;
use crate::world::v1::v12::SMSG_CANCEL_COMBAT;
use crate::world::v1::v12::SMSG_SPELLHEALLOG;
use crate::world::v1::v12::{SMSG_SPELLENERGIZELOG, SMSG_SPELLENERGIZELOGError};
use crate::world::v1::v12::{SMSG_BINDPOINTUPDATE, SMSG_BINDPOINTUPDATEError};
use crate::world::v1::v12::{SMSG_PLAYERBOUND, SMSG_PLAYERBOUNDError};
use crate::world::v1::v12::SMSG_CLIENT_CONTROL_UPDATE;
use crate::world::v1::v12::{SMSG_RESURRECT_REQUEST, SMSG_RESURRECT_REQUESTError};
use crate::world::v1::v12::{SMSG_LOOT_RESPONSE, SMSG_LOOT_RESPONSEError};
use crate::world::v1::v12::SMSG_LOOT_RELEASE_RESPONSE;
use crate::world::v1::v12::SMSG_LOOT_REMOVED;
use crate::world::v1::v12::SMSG_LOOT_MONEY_NOTIFY;
use crate::world::v1::v12::SMSG_LOOT_CLEAR_MONEY;
use crate::world::v1::v12::{SMSG_ITEM_PUSH_RESULT, SMSG_ITEM_PUSH_RESULTError};
use crate::world::v1::v12::SMSG_DUEL_REQUESTED;
use crate::world::v1::v12::SMSG_DUEL_OUTOFBOUNDS;
use crate::world::v1::v12::SMSG_DUEL_INBOUNDS;
use crate::world::v1::v12::SMSG_DUEL_COMPLETE;
use crate::world::v1::v12::{SMSG_DUEL_WINNER, SMSG_DUEL_WINNERError};
use crate::world::v1::v12::{SMSG_MOUNTRESULT, SMSG_MOUNTRESULTError};
use crate::world::v1::v12::{SMSG_DISMOUNTRESULT, SMSG_DISMOUNTRESULTError};
use crate::world::v1::v12::SMSG_MOUNTSPECIAL_ANIM;
use crate::world::v1::v12::{SMSG_PET_TAME_FAILURE, SMSG_PET_TAME_FAILUREError};
use crate::world::v1::v12::SMSG_PET_NAME_INVALID;
use crate::world::v1::v12::{SMSG_PET_SPELLS, SMSG_PET_SPELLSError};
use crate::world::v1::v12::{SMSG_PET_MODE, SMSG_PET_MODEError};
use crate::world::v1::v12::{SMSG_GOSSIP_MESSAGE, SMSG_GOSSIP_MESSAGEError};
use crate::world::v1::v12::SMSG_GOSSIP_COMPLETE;
use crate::world::v1::v12::{SMSG_NPC_TEXT_UPDATE, SMSG_NPC_TEXT_UPDATEError};
use crate::world::v1::v12::{SMSG_QUESTGIVER_STATUS, SMSG_QUESTGIVER_STATUSError};
use crate::world::v1::v12::{SMSG_QUESTGIVER_QUEST_LIST, SMSG_QUESTGIVER_QUEST_LISTError};
use crate::world::v1::v12::{SMSG_QUESTGIVER_QUEST_DETAILS, SMSG_QUESTGIVER_QUEST_DETAILSError};
use crate::world::v1::v12::{SMSG_QUESTGIVER_REQUEST_ITEMS, SMSG_QUESTGIVER_REQUEST_ITEMSError};
use crate::world::v1::v12::{SMSG_QUESTGIVER_OFFER_REWARD, SMSG_QUESTGIVER_OFFER_REWARDError};
use crate::world::v1::v12::{SMSG_QUESTGIVER_QUEST_INVALID, SMSG_QUESTGIVER_QUEST_INVALIDError};
use crate::world::v1::v12::SMSG_QUESTGIVER_QUEST_COMPLETE;
use crate::world::v1::v12::{SMSG_QUESTGIVER_QUEST_FAILED, SMSG_QUESTGIVER_QUEST_FAILEDError};
use crate::world::v1::v12::SMSG_QUESTLOG_FULL;
use crate::world::v1::v12::SMSG_QUESTUPDATE_FAILED;
use crate::world::v1::v12::SMSG_QUESTUPDATE_FAILEDTIMER;
use crate::world::v1::v12::SMSG_QUESTUPDATE_COMPLETE;
use crate::world::v1::v12::SMSG_QUESTUPDATE_ADD_KILL;
use crate::world::v1::v12::SMSG_QUESTUPDATE_ADD_ITEM;
use crate::world::v1::v12::{SMSG_QUEST_CONFIRM_ACCEPT, SMSG_QUEST_CONFIRM_ACCEPTError};
use crate::world::v1::v12::SMSG_LIST_INVENTORY;
use crate::world::v1::v12::{SMSG_SELL_ITEM, SMSG_SELL_ITEMError};
use crate::world::v1::v12::SMSG_BUY_ITEM;
use crate::world::v1::v12::{SMSG_BUY_FAILED, SMSG_BUY_FAILEDError};
use crate::world::v1::v12::SMSG_SHOWTAXINODES;
use crate::world::v1::v12::SMSG_TAXINODE_STATUS;
use crate::world::v1::v12::{SMSG_ACTIVATETAXIREPLY, SMSG_ACTIVATETAXIREPLYError};
use crate::world::v1::v12::SMSG_NEW_TAXI_PATH;
use crate::world::v1::v12::{SMSG_TRAINER_LIST, SMSG_TRAINER_LISTError};
use crate::world::v1::v12::SMSG_TRAINER_BUY_SUCCEEDED;
use crate::world::v1::v12::{SMSG_TRAINER_BUY_FAILED, SMSG_TRAINER_BUY_FAILEDError};
use crate::world::v1::v12::SMSG_SHOW_BANK;
use crate::world::v1::v12::{SMSG_BUY_BANK_SLOT_RESULT, SMSG_BUY_BANK_SLOT_RESULTError};
use crate::world::v1::v12::SMSG_PETITION_SHOWLIST;
use crate::world::v1::v12::SMSG_PETITION_SHOW_SIGNATURES;
use crate::world::v1::v12::{SMSG_PETITION_SIGN_RESULTS, SMSG_PETITION_SIGN_RESULTSError};
use crate::world::v1::v12::{SMSG_TURN_IN_PETITION_RESULTS, SMSG_TURN_IN_PETITION_RESULTSError};
use crate::world::v1::v12::{SMSG_PETITION_QUERY_RESPONSE, SMSG_PETITION_QUERY_RESPONSEError};
use crate::world::v1::v12::SMSG_FISH_NOT_HOOKED;
use crate::world::v1::v12::SMSG_FISH_ESCAPED;
use crate::world::v1::v12::{SMSG_NOTIFICATION, SMSG_NOTIFICATIONError};
use crate::world::v1::v12::SMSG_PLAYED_TIME;
use crate::world::v1::v12::SMSG_QUERY_TIME_RESPONSE;
use crate::world::v1::v12::{SMSG_LOG_XPGAIN, SMSG_LOG_XPGAINError};
use crate::world::v1::v12::SMSG_LEVELUP_INFO;
use crate::world::v1::v12::MSG_MINIMAP_PING_Server;
use crate::world::v1::v12::SMSG_RESISTLOG;
use crate::world::v1::v12::SMSG_ENCHANTMENTLOG;
use crate::world::v1::v12::{SMSG_START_MIRROR_TIMER, SMSG_START_MIRROR_TIMERError};
use crate::world::v1::v12::{SMSG_PAUSE_MIRROR_TIMER, SMSG_PAUSE_MIRROR_TIMERError};
use crate::world::v1::v12::{SMSG_STOP_MIRROR_TIMER, SMSG_STOP_MIRROR_TIMERError};
use crate::world::v1::v12::SMSG_PONG;
use crate::world::v1::v12::SMSG_CLEAR_COOLDOWN;
use crate::world::v1::v12::SMSG_GAMEOBJECT_PAGETEXT;
use crate::world::v1::v12::SMSG_SPELL_DELAYED;
use crate::world::v1::v12::SMSG_ITEM_TIME_UPDATE;
use crate::world::v1::v12::SMSG_ITEM_ENCHANT_TIME_UPDATE;
use crate::world::v1::v2::SMSG_AUTH_CHALLENGE;
use crate::world::v1::v2::{SMSG_AUTH_RESPONSE, SMSG_AUTH_RESPONSEError};
use crate::world::v1::v12::{MSG_SAVE_GUILD_EMBLEM_Server, MSG_SAVE_GUILD_EMBLEM_ServerError};
use crate::world::v1::v12::SMSG_PLAY_SPELL_VISUAL;
use crate::world::v1::v12::SMSG_PARTYKILLLOG;
use crate::world::v1::v12::SMSG_PLAY_SPELL_IMPACT;
use crate::world::v1::v12::{SMSG_EXPLORATION_EXPERIENCE, SMSG_EXPLORATION_EXPERIENCEError};
use crate::world::v1::v12::MSG_RANDOM_ROLL_Server;
use crate::world::v1::v12::{SMSG_ENVIRONMENTALDAMAGELOG, SMSG_ENVIRONMENTALDAMAGELOGError};
use crate::world::v1::v12::MSG_LOOKING_FOR_GROUP_Server;
use crate::world::v1::v12::SMSG_REMOVED_SPELL;
use crate::world::v1::v12::{SMSG_GMTICKET_CREATE, SMSG_GMTICKET_CREATEError};
use crate::world::v1::v12::{SMSG_GMTICKET_UPDATETEXT, SMSG_GMTICKET_UPDATETEXTError};
use crate::world::v1::v12::SMSG_ACCOUNT_DATA_TIMES;
use crate::world::v1::v12::{SMSG_GMTICKET_GETTICKET, SMSG_GMTICKET_GETTICKETError};
use crate::world::v1::v12::SMSG_GAMEOBJECT_SPAWN_ANIM;
use crate::world::v1::v12::SMSG_GAMEOBJECT_DESPAWN_ANIM;
use crate::world::v1::v12::{MSG_CORPSE_QUERY_Server, MSG_CORPSE_QUERY_ServerError};
use crate::world::v1::v12::{SMSG_GMTICKET_DELETETICKET, SMSG_GMTICKET_DELETETICKETError};
use crate::world::v1::v12::SMSG_CHAT_WRONG_FACTION;
use crate::world::v1::v12::SMSG_GMTICKET_SYSTEMSTATUS;
use crate::world::v1::v12::SMSG_SET_REST_START;
use crate::world::v1::v12::SMSG_SPIRIT_HEALER_CONFIRM;
use crate::world::v1::v12::{SMSG_GOSSIP_POI, SMSG_GOSSIP_POIError};
use crate::world::v1::v12::{SMSG_LOGIN_VERIFY_WORLD, SMSG_LOGIN_VERIFY_WORLDError};
use crate::world::v1::v12::{SMSG_MAIL_LIST_RESULT, SMSG_MAIL_LIST_RESULTError};
use crate::world::v1::v12::{SMSG_BATTLEFIELD_LIST, SMSG_BATTLEFIELD_LISTError};
use crate::world::v1::v12::{SMSG_ITEM_TEXT_QUERY_RESPONSE, SMSG_ITEM_TEXT_QUERY_RESPONSEError};
use crate::world::v1::v12::{SMSG_SPELLLOGMISS, SMSG_SPELLLOGMISSError};
use crate::world::v1::v12::{SMSG_SPELLLOGEXECUTE, SMSG_SPELLLOGEXECUTEError};
use crate::world::v1::v12::{SMSG_PERIODICAURALOG, SMSG_PERIODICAURALOGError};
use crate::world::v1::v12::{SMSG_SPELLDAMAGESHIELD, SMSG_SPELLDAMAGESHIELDError};
use crate::world::v1::v12::{SMSG_SPELLNONMELEEDAMAGELOG, SMSG_SPELLNONMELEEDAMAGELOGError};
use crate::world::v1::v12::SMSG_ZONE_UNDER_ATTACK;
use crate::world::v1::v12::MSG_AUCTION_HELLO_Server;
use crate::world::v1::v12::SMSG_AUCTION_LIST_RESULT;
use crate::world::v1::v12::SMSG_AUCTION_OWNER_LIST_RESULT;
use crate::world::v1::v12::SMSG_AUCTION_BIDDER_NOTIFICATION;
use crate::world::v1::v12::SMSG_AUCTION_OWNER_NOTIFICATION;
use crate::world::v1::v12::{SMSG_PROCRESIST, SMSG_PROCRESISTError};
use crate::world::v1::v12::SMSG_DISPEL_FAILED;
use crate::world::v1::v12::SMSG_SPELLORDAMAGE_IMMUNE;
use crate::world::v1::v12::SMSG_AUCTION_BIDDER_LIST_RESULT;
use crate::world::v1::v12::SMSG_SET_FLAT_SPELL_MODIFIER;
use crate::world::v1::v12::SMSG_SET_PCT_SPELL_MODIFIER;
use crate::world::v1::v12::SMSG_CORPSE_RECLAIM_DELAY;
use crate::world::v1::v12::{MSG_LIST_STABLED_PETS_Server, MSG_LIST_STABLED_PETS_ServerError};
use crate::world::v1::v12::{SMSG_STABLE_RESULT, SMSG_STABLE_RESULTError};
use crate::world::v1::v12::SMSG_PLAY_MUSIC;
use crate::world::v1::v12::SMSG_PLAY_OBJECT_SOUND;
use crate::world::v1::v12::SMSG_SPELLDISPELLOG;
use crate::world::v1::v12::MSG_QUERY_NEXT_MAIL_TIME_Server;
use crate::world::v1::v12::SMSG_RECEIVED_MAIL;
use crate::world::v1::v12::{SMSG_RAID_GROUP_ONLY, SMSG_RAID_GROUP_ONLYError};
use crate::world::v1::v12::{SMSG_PVP_CREDIT, SMSG_PVP_CREDITError};
use crate::world::v1::v12::SMSG_AUCTION_REMOVED_NOTIFICATION;
use crate::world::v1::v12::{SMSG_SERVER_MESSAGE, SMSG_SERVER_MESSAGEError};
use crate::world::v1::v12::{SMSG_MEETINGSTONE_SETQUEUE, SMSG_MEETINGSTONE_SETQUEUEError};
use crate::world::v1::v12::SMSG_MEETINGSTONE_COMPLETE;
use crate::world::v1::v12::SMSG_MEETINGSTONE_IN_PROGRESS;
use crate::world::v1::v12::SMSG_MEETINGSTONE_MEMBER_ADDED;
use crate::world::v1::v12::SMSG_CANCEL_AUTO_REPEAT;
use crate::world::v1::v12::{SMSG_STANDSTATE_UPDATE, SMSG_STANDSTATE_UPDATEError};
use crate::world::v1::v12::SMSG_LOOT_ALL_PASSED;
use crate::world::v1::v12::{SMSG_LOOT_ROLL_WON, SMSG_LOOT_ROLL_WONError};
use crate::world::v1::v12::SMSG_LOOT_START_ROLL;
use crate::world::v1::v12::{SMSG_LOOT_ROLL, SMSG_LOOT_ROLLError};
use crate::world::v1::v12::SMSG_LOOT_MASTER_LIST;
use crate::world::v1::v12::SMSG_SET_FORCED_REACTIONS;
use crate::world::v1::v12::SMSG_SPELL_FAILED_OTHER;
use crate::world::v1::v12::SMSG_GAMEOBJECT_RESET_STATE;
use crate::world::v1::v12::{SMSG_CHAT_PLAYER_NOT_FOUND, SMSG_CHAT_PLAYER_NOT_FOUNDError};
use crate::world::v1::v12::MSG_TALENT_WIPE_CONFIRM_Server;
use crate::world::v1::v12::SMSG_SUMMON_REQUEST;
use crate::world::v1::v12::{SMSG_MONSTER_MOVE_TRANSPORT, SMSG_MONSTER_MOVE_TRANSPORTError};
use crate::world::v1::v12::SMSG_PET_BROKEN;
use crate::world::v1::v12::SMSG_FEIGN_DEATH_RESISTED;
use crate::world::v1::v12::SMSG_DUEL_COUNTDOWN;
use crate::world::v1::v12::{SMSG_AREA_TRIGGER_MESSAGE, SMSG_AREA_TRIGGER_MESSAGEError};
use crate::world::v1::v12::{SMSG_MEETINGSTONE_JOINFAILED, SMSG_MEETINGSTONE_JOINFAILEDError};
use crate::world::v1::v12::SMSG_PLAYER_SKINNED;
use crate::world::v1::v12::SMSG_DURABILITY_DAMAGE_DEATH;
use crate::world::v1::v12::{SMSG_INIT_WORLD_STATES, SMSG_INIT_WORLD_STATESError};
use crate::world::v1::v12::SMSG_UPDATE_WORLD_STATE;
use crate::world::v1::v12::{SMSG_ITEM_NAME_QUERY_RESPONSE, SMSG_ITEM_NAME_QUERY_RESPONSEError};
use crate::world::v1::v12::{SMSG_PET_ACTION_FEEDBACK, SMSG_PET_ACTION_FEEDBACKError};
use crate::world::v1::v12::{SMSG_CHAR_RENAME, SMSG_CHAR_RENAMEError};
use crate::world::v1::v12::SMSG_INSTANCE_SAVE_CREATED;
use crate::world::v1::v12::{SMSG_RAID_INSTANCE_INFO, SMSG_RAID_INSTANCE_INFOError};
use crate::world::v1::v12::SMSG_PLAY_SOUND;
use crate::world::v1::v12::{SMSG_BATTLEFIELD_STATUS, SMSG_BATTLEFIELD_STATUSError};
use crate::world::v1::v12::{MSG_INSPECT_HONOR_STATS_Server, MSG_INSPECT_HONOR_STATS_ServerError};
use crate::world::v1::v12::SMSG_FORCE_WALK_SPEED_CHANGE;
use crate::world::v1::v12::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE;
use crate::world::v1::v12::SMSG_FORCE_TURN_RATE_CHANGE;
use crate::world::v1::v12::{MSG_PVP_LOG_DATA_Server, MSG_PVP_LOG_DATA_ServerError};
use crate::world::v1::v12::SMSG_AREA_SPIRIT_HEALER_TIME;
use crate::world::v1::v12::{SMSG_GROUP_JOINED_BATTLEGROUND, SMSG_GROUP_JOINED_BATTLEGROUNDError};
use crate::world::v1::v12::MSG_BATTLEGROUND_PLAYER_POSITIONS_Server;
use crate::world::v1::v12::SMSG_BINDER_CONFIRM;
use crate::world::v1::v12::SMSG_BATTLEGROUND_PLAYER_JOINED;
use crate::world::v1::v12::SMSG_BATTLEGROUND_PLAYER_LEFT;
use crate::world::v1::v12::SMSG_PET_UNLEARN_CONFIRM;
use crate::world::v1::v12::{SMSG_PARTY_MEMBER_STATS_FULL, SMSG_PARTY_MEMBER_STATS_FULLError};
use crate::world::v1::v12::{SMSG_WEATHER, SMSG_WEATHERError};
use crate::world::v1::v12::{SMSG_RAID_INSTANCE_MESSAGE, SMSG_RAID_INSTANCE_MESSAGEError};
use crate::world::v1::v12::SMSG_CHAT_RESTRICTED;
use crate::world::v1::v12::SMSG_SPLINE_SET_RUN_SPEED;
use crate::world::v1::v12::SMSG_SPLINE_SET_RUN_BACK_SPEED;
use crate::world::v1::v12::SMSG_SPLINE_SET_SWIM_SPEED;
use crate::world::v1::v12::SMSG_SPLINE_SET_WALK_SPEED;
use crate::world::v1::v12::SMSG_SPLINE_SET_SWIM_BACK_SPEED;
use crate::world::v1::v12::SMSG_SPLINE_SET_TURN_RATE;
use crate::world::v1::v12::SMSG_SPLINE_MOVE_UNROOT;
use crate::world::v1::v12::SMSG_SPLINE_MOVE_FEATHER_FALL;
use crate::world::v1::v12::SMSG_SPLINE_MOVE_NORMAL_FALL;
use crate::world::v1::v12::SMSG_SPLINE_MOVE_SET_HOVER;
use crate::world::v1::v12::SMSG_SPLINE_MOVE_UNSET_HOVER;
use crate::world::v1::v12::SMSG_SPLINE_MOVE_WATER_WALK;
use crate::world::v1::v12::SMSG_SPLINE_MOVE_LAND_WALK;
use crate::world::v1::v12::SMSG_SPLINE_MOVE_START_SWIM;
use crate::world::v1::v12::SMSG_SPLINE_MOVE_STOP_SWIM;
use crate::world::v1::v12::SMSG_SPLINE_MOVE_SET_RUN_MODE;
use crate::world::v1::v12::SMSG_SPLINE_MOVE_SET_WALK_MODE;
use crate::world::v1::v12::SMSG_SPLINE_MOVE_ROOT;
use crate::world::v1::v12::SMSG_INVALIDATE_PLAYER;
use crate::world::v1::v12::{SMSG_INSTANCE_RESET, SMSG_INSTANCE_RESETError};
use crate::world::v1::v12::{SMSG_INSTANCE_RESET_FAILED, SMSG_INSTANCE_RESET_FAILEDError};
use crate::world::v1::v12::{SMSG_UPDATE_LAST_INSTANCE, SMSG_UPDATE_LAST_INSTANCEError};
use crate::world::v1::v12::{MSG_RAID_TARGET_UPDATE_Server, MSG_RAID_TARGET_UPDATE_ServerError};
use crate::world::v1::v12::MSG_RAID_READY_CHECK_Server;
use crate::world::v1::v12::{SMSG_PET_ACTION_SOUND, SMSG_PET_ACTION_SOUNDError};
use crate::world::v1::v12::SMSG_PET_DISMISS_SOUND;
use crate::world::v1::v12::{SMSG_GM_TICKET_STATUS_UPDATE, SMSG_GM_TICKET_STATUS_UPDATEError};
use crate::world::v1::v12::SMSG_UPDATE_INSTANCE_OWNERSHIP;
use crate::world::v1::v12::SMSG_SPELLINSTAKILLLOG;
use crate::world::v1::v12::SMSG_SPELL_UPDATE_CHAIN_TARGETS;
use crate::world::v1::v12::{SMSG_EXPECTED_SPAM_RECORDS, SMSG_EXPECTED_SPAM_RECORDSError};
use crate::world::v1::v12::{SMSG_DEFENSE_MESSAGE, SMSG_DEFENSE_MESSAGEError};

#[derive(Debug)]
pub enum ServerOpcode {
    MSG_MOVE_START_FORWARD,
    MSG_MOVE_START_BACKWARD,
    MSG_MOVE_STOP,
    MSG_MOVE_START_STRAFE_LEFT,
    MSG_MOVE_START_STRAFE_RIGHT,
    MSG_MOVE_STOP_STRAFE,
    MSG_MOVE_JUMP,
    MSG_MOVE_START_TURN_LEFT,
    MSG_MOVE_START_TURN_RIGHT,
    MSG_MOVE_STOP_TURN,
    MSG_MOVE_START_PITCH_UP,
    MSG_MOVE_START_PITCH_DOWN,
    MSG_MOVE_STOP_PITCH,
    MSG_MOVE_SET_RUN_MODE,
    MSG_MOVE_SET_WALK_MODE,
    MSG_MOVE_TELEPORT_ACK,
    MSG_MOVE_FALL_LAND,
    MSG_MOVE_START_SWIM,
    MSG_MOVE_STOP_SWIM,
    MSG_MOVE_SET_FACING,
    MSG_MOVE_SET_PITCH,
    MSG_MOVE_WORLDPORT_ACK,
    MSG_MOVE_HEARTBEAT,
    MSG_PETITION_DECLINE,
    MSG_TABARDVENDOR_ACTIVATE,
    MSG_QUEST_PUSH_RESULT,
    MSG_PETITION_RENAME,
    SMSG_CHAR_CREATE,
    SMSG_CHAR_ENUM,
    SMSG_CHAR_DELETE,
    SMSG_NEW_WORLD,
    SMSG_TRANSFER_PENDING,
    SMSG_TRANSFER_ABORTED,
    SMSG_CHARACTER_LOGIN_FAILED,
    SMSG_LOGIN_SETTIMESPEED,
    SMSG_LOGOUT_RESPONSE,
    SMSG_LOGOUT_COMPLETE,
    SMSG_LOGOUT_CANCEL_ACK,
    SMSG_NAME_QUERY_RESPONSE,
    SMSG_PET_NAME_QUERY_RESPONSE,
    SMSG_GUILD_QUERY_RESPONSE,
    SMSG_ITEM_QUERY_SINGLE_RESPONSE,
    SMSG_PAGE_TEXT_QUERY_RESPONSE,
    SMSG_QUEST_QUERY_RESPONSE,
    SMSG_GAMEOBJECT_QUERY_RESPONSE,
    SMSG_CREATURE_QUERY_RESPONSE,
    SMSG_WHO,
    SMSG_WHOIS,
    SMSG_FRIEND_LIST,
    SMSG_FRIEND_STATUS,
    SMSG_IGNORE_LIST,
    SMSG_GROUP_INVITE,
    SMSG_GROUP_DECLINE,
    SMSG_GROUP_UNINVITE,
    SMSG_GROUP_SET_LEADER,
    SMSG_GROUP_DESTROYED,
    SMSG_GROUP_LIST,
    SMSG_PARTY_MEMBER_STATS,
    SMSG_PARTY_COMMAND_RESULT,
    SMSG_GUILD_INVITE,
    SMSG_GUILD_INFO,
    SMSG_GUILD_ROSTER,
    SMSG_GUILD_EVENT,
    SMSG_GUILD_COMMAND_RESULT,
    SMSG_MESSAGECHAT,
    SMSG_CHANNEL_NOTIFY,
    SMSG_CHANNEL_LIST,
    SMSG_DESTROY_OBJECT,
    SMSG_READ_ITEM_OK,
    SMSG_READ_ITEM_FAILED,
    SMSG_ITEM_COOLDOWN,
    SMSG_GAMEOBJECT_CUSTOM_ANIM,
    SMSG_MONSTER_MOVE,
    SMSG_MOVE_WATER_WALK,
    SMSG_MOVE_LAND_WALK,
    SMSG_FORCE_RUN_SPEED_CHANGE,
    SMSG_FORCE_RUN_BACK_SPEED_CHANGE,
    SMSG_FORCE_SWIM_SPEED_CHANGE,
    SMSG_FORCE_MOVE_ROOT,
    SMSG_FORCE_MOVE_UNROOT,
    SMSG_MOVE_KNOCK_BACK,
    SMSG_MOVE_FEATHER_FALL,
    SMSG_MOVE_NORMAL_FALL,
    SMSG_MOVE_SET_HOVER,
    SMSG_MOVE_UNSET_HOVER,
    SMSG_TRIGGER_CINEMATIC,
    SMSG_TUTORIAL_FLAGS,
    SMSG_EMOTE,
    SMSG_TEXT_EMOTE,
    SMSG_INVENTORY_CHANGE_FAILURE,
    SMSG_OPEN_CONTAINER,
    SMSG_INSPECT,
    SMSG_TRADE_STATUS,
    SMSG_TRADE_STATUS_EXTENDED,
    SMSG_INITIALIZE_FACTIONS,
    SMSG_SET_FACTION_VISIBLE,
    SMSG_SET_FACTION_STANDING,
    SMSG_SET_PROFICIENCY,
    SMSG_ACTION_BUTTONS,
    SMSG_INITIAL_SPELLS,
    SMSG_LEARNED_SPELL,
    SMSG_SUPERCEDED_SPELL,
    SMSG_CAST_RESULT,
    SMSG_SPELL_START,
    SMSG_SPELL_GO,
    SMSG_SPELL_FAILURE,
    SMSG_SPELL_COOLDOWN,
    SMSG_COOLDOWN_EVENT,
    SMSG_UPDATE_AURA_DURATION,
    SMSG_PET_CAST_FAILED,
    SMSG_AI_REACTION,
    SMSG_ATTACKSTART,
    SMSG_ATTACKSTOP,
    SMSG_ATTACKSWING_NOTINRANGE,
    SMSG_ATTACKSWING_BADFACING,
    SMSG_ATTACKSWING_NOTSTANDING,
    SMSG_ATTACKSWING_DEADTARGET,
    SMSG_ATTACKSWING_CANT_ATTACK,
    SMSG_ATTACKERSTATEUPDATE,
    SMSG_CANCEL_COMBAT,
    SMSG_SPELLHEALLOG,
    SMSG_SPELLENERGIZELOG,
    SMSG_BINDPOINTUPDATE,
    SMSG_PLAYERBOUND,
    SMSG_CLIENT_CONTROL_UPDATE,
    SMSG_RESURRECT_REQUEST,
    SMSG_LOOT_RESPONSE,
    SMSG_LOOT_RELEASE_RESPONSE,
    SMSG_LOOT_REMOVED,
    SMSG_LOOT_MONEY_NOTIFY,
    SMSG_LOOT_CLEAR_MONEY,
    SMSG_ITEM_PUSH_RESULT,
    SMSG_DUEL_REQUESTED,
    SMSG_DUEL_OUTOFBOUNDS,
    SMSG_DUEL_INBOUNDS,
    SMSG_DUEL_COMPLETE,
    SMSG_DUEL_WINNER,
    SMSG_MOUNTRESULT,
    SMSG_DISMOUNTRESULT,
    SMSG_MOUNTSPECIAL_ANIM,
    SMSG_PET_TAME_FAILURE,
    SMSG_PET_NAME_INVALID,
    SMSG_PET_SPELLS,
    SMSG_PET_MODE,
    SMSG_GOSSIP_MESSAGE,
    SMSG_GOSSIP_COMPLETE,
    SMSG_NPC_TEXT_UPDATE,
    SMSG_QUESTGIVER_STATUS,
    SMSG_QUESTGIVER_QUEST_LIST,
    SMSG_QUESTGIVER_QUEST_DETAILS,
    SMSG_QUESTGIVER_REQUEST_ITEMS,
    SMSG_QUESTGIVER_OFFER_REWARD,
    SMSG_QUESTGIVER_QUEST_INVALID,
    SMSG_QUESTGIVER_QUEST_COMPLETE,
    SMSG_QUESTGIVER_QUEST_FAILED,
    SMSG_QUESTLOG_FULL,
    SMSG_QUESTUPDATE_FAILED,
    SMSG_QUESTUPDATE_FAILEDTIMER,
    SMSG_QUESTUPDATE_COMPLETE,
    SMSG_QUESTUPDATE_ADD_KILL,
    SMSG_QUESTUPDATE_ADD_ITEM,
    SMSG_QUEST_CONFIRM_ACCEPT,
    SMSG_LIST_INVENTORY,
    SMSG_SELL_ITEM,
    SMSG_BUY_ITEM,
    SMSG_BUY_FAILED,
    SMSG_SHOWTAXINODES,
    SMSG_TAXINODE_STATUS,
    SMSG_ACTIVATETAXIREPLY,
    SMSG_NEW_TAXI_PATH,
    SMSG_TRAINER_LIST,
    SMSG_TRAINER_BUY_SUCCEEDED,
    SMSG_TRAINER_BUY_FAILED,
    SMSG_SHOW_BANK,
    SMSG_BUY_BANK_SLOT_RESULT,
    SMSG_PETITION_SHOWLIST,
    SMSG_PETITION_SHOW_SIGNATURES,
    SMSG_PETITION_SIGN_RESULTS,
    SMSG_TURN_IN_PETITION_RESULTS,
    SMSG_PETITION_QUERY_RESPONSE,
    SMSG_FISH_NOT_HOOKED,
    SMSG_FISH_ESCAPED,
    SMSG_NOTIFICATION,
    SMSG_PLAYED_TIME,
    SMSG_QUERY_TIME_RESPONSE,
    SMSG_LOG_XPGAIN,
    SMSG_LEVELUP_INFO,
    MSG_MINIMAP_PING,
    SMSG_RESISTLOG,
    SMSG_ENCHANTMENTLOG,
    SMSG_START_MIRROR_TIMER,
    SMSG_PAUSE_MIRROR_TIMER,
    SMSG_STOP_MIRROR_TIMER,
    SMSG_PONG,
    SMSG_CLEAR_COOLDOWN,
    SMSG_GAMEOBJECT_PAGETEXT,
    SMSG_SPELL_DELAYED,
    SMSG_ITEM_TIME_UPDATE,
    SMSG_ITEM_ENCHANT_TIME_UPDATE,
    SMSG_AUTH_CHALLENGE,
    SMSG_AUTH_RESPONSE,
    MSG_SAVE_GUILD_EMBLEM,
    SMSG_PLAY_SPELL_VISUAL,
    SMSG_PARTYKILLLOG,
    SMSG_PLAY_SPELL_IMPACT,
    SMSG_EXPLORATION_EXPERIENCE,
    MSG_RANDOM_ROLL,
    SMSG_ENVIRONMENTALDAMAGELOG,
    MSG_LOOKING_FOR_GROUP,
    SMSG_REMOVED_SPELL,
    SMSG_GMTICKET_CREATE,
    SMSG_GMTICKET_UPDATETEXT,
    SMSG_ACCOUNT_DATA_TIMES,
    SMSG_GMTICKET_GETTICKET,
    SMSG_GAMEOBJECT_SPAWN_ANIM,
    SMSG_GAMEOBJECT_DESPAWN_ANIM,
    MSG_CORPSE_QUERY,
    SMSG_GMTICKET_DELETETICKET,
    SMSG_CHAT_WRONG_FACTION,
    SMSG_GMTICKET_SYSTEMSTATUS,
    SMSG_SET_REST_START,
    SMSG_SPIRIT_HEALER_CONFIRM,
    SMSG_GOSSIP_POI,
    SMSG_LOGIN_VERIFY_WORLD,
    SMSG_MAIL_LIST_RESULT,
    SMSG_BATTLEFIELD_LIST,
    SMSG_ITEM_TEXT_QUERY_RESPONSE,
    SMSG_SPELLLOGMISS,
    SMSG_SPELLLOGEXECUTE,
    SMSG_PERIODICAURALOG,
    SMSG_SPELLDAMAGESHIELD,
    SMSG_SPELLNONMELEEDAMAGELOG,
    SMSG_ZONE_UNDER_ATTACK,
    MSG_AUCTION_HELLO,
    SMSG_AUCTION_LIST_RESULT,
    SMSG_AUCTION_OWNER_LIST_RESULT,
    SMSG_AUCTION_BIDDER_NOTIFICATION,
    SMSG_AUCTION_OWNER_NOTIFICATION,
    SMSG_PROCRESIST,
    SMSG_DISPEL_FAILED,
    SMSG_SPELLORDAMAGE_IMMUNE,
    SMSG_AUCTION_BIDDER_LIST_RESULT,
    SMSG_SET_FLAT_SPELL_MODIFIER,
    SMSG_SET_PCT_SPELL_MODIFIER,
    SMSG_CORPSE_RECLAIM_DELAY,
    MSG_LIST_STABLED_PETS,
    SMSG_STABLE_RESULT,
    SMSG_PLAY_MUSIC,
    SMSG_PLAY_OBJECT_SOUND,
    SMSG_SPELLDISPELLOG,
    MSG_QUERY_NEXT_MAIL_TIME,
    SMSG_RECEIVED_MAIL,
    SMSG_RAID_GROUP_ONLY,
    SMSG_PVP_CREDIT,
    SMSG_AUCTION_REMOVED_NOTIFICATION,
    SMSG_SERVER_MESSAGE,
    SMSG_MEETINGSTONE_SETQUEUE,
    SMSG_MEETINGSTONE_COMPLETE,
    SMSG_MEETINGSTONE_IN_PROGRESS,
    SMSG_MEETINGSTONE_MEMBER_ADDED,
    SMSG_CANCEL_AUTO_REPEAT,
    SMSG_STANDSTATE_UPDATE,
    SMSG_LOOT_ALL_PASSED,
    SMSG_LOOT_ROLL_WON,
    SMSG_LOOT_START_ROLL,
    SMSG_LOOT_ROLL,
    SMSG_LOOT_MASTER_LIST,
    SMSG_SET_FORCED_REACTIONS,
    SMSG_SPELL_FAILED_OTHER,
    SMSG_GAMEOBJECT_RESET_STATE,
    SMSG_CHAT_PLAYER_NOT_FOUND,
    MSG_TALENT_WIPE_CONFIRM,
    SMSG_SUMMON_REQUEST,
    SMSG_MONSTER_MOVE_TRANSPORT,
    SMSG_PET_BROKEN,
    SMSG_FEIGN_DEATH_RESISTED,
    SMSG_DUEL_COUNTDOWN,
    SMSG_AREA_TRIGGER_MESSAGE,
    SMSG_MEETINGSTONE_JOINFAILED,
    SMSG_PLAYER_SKINNED,
    SMSG_DURABILITY_DAMAGE_DEATH,
    SMSG_INIT_WORLD_STATES,
    SMSG_UPDATE_WORLD_STATE,
    SMSG_ITEM_NAME_QUERY_RESPONSE,
    SMSG_PET_ACTION_FEEDBACK,
    SMSG_CHAR_RENAME,
    SMSG_INSTANCE_SAVE_CREATED,
    SMSG_RAID_INSTANCE_INFO,
    SMSG_PLAY_SOUND,
    SMSG_BATTLEFIELD_STATUS,
    MSG_INSPECT_HONOR_STATS,
    SMSG_FORCE_WALK_SPEED_CHANGE,
    SMSG_FORCE_SWIM_BACK_SPEED_CHANGE,
    SMSG_FORCE_TURN_RATE_CHANGE,
    MSG_PVP_LOG_DATA,
    SMSG_AREA_SPIRIT_HEALER_TIME,
    SMSG_GROUP_JOINED_BATTLEGROUND,
    MSG_BATTLEGROUND_PLAYER_POSITIONS,
    SMSG_BINDER_CONFIRM,
    SMSG_BATTLEGROUND_PLAYER_JOINED,
    SMSG_BATTLEGROUND_PLAYER_LEFT,
    SMSG_PET_UNLEARN_CONFIRM,
    SMSG_PARTY_MEMBER_STATS_FULL,
    SMSG_WEATHER,
    SMSG_RAID_INSTANCE_MESSAGE,
    SMSG_CHAT_RESTRICTED,
    SMSG_SPLINE_SET_RUN_SPEED,
    SMSG_SPLINE_SET_RUN_BACK_SPEED,
    SMSG_SPLINE_SET_SWIM_SPEED,
    SMSG_SPLINE_SET_WALK_SPEED,
    SMSG_SPLINE_SET_SWIM_BACK_SPEED,
    SMSG_SPLINE_SET_TURN_RATE,
    SMSG_SPLINE_MOVE_UNROOT,
    SMSG_SPLINE_MOVE_FEATHER_FALL,
    SMSG_SPLINE_MOVE_NORMAL_FALL,
    SMSG_SPLINE_MOVE_SET_HOVER,
    SMSG_SPLINE_MOVE_UNSET_HOVER,
    SMSG_SPLINE_MOVE_WATER_WALK,
    SMSG_SPLINE_MOVE_LAND_WALK,
    SMSG_SPLINE_MOVE_START_SWIM,
    SMSG_SPLINE_MOVE_STOP_SWIM,
    SMSG_SPLINE_MOVE_SET_RUN_MODE,
    SMSG_SPLINE_MOVE_SET_WALK_MODE,
    SMSG_SPLINE_MOVE_ROOT,
    SMSG_INVALIDATE_PLAYER,
    SMSG_INSTANCE_RESET,
    SMSG_INSTANCE_RESET_FAILED,
    SMSG_UPDATE_LAST_INSTANCE,
    MSG_RAID_TARGET_UPDATE,
    MSG_RAID_READY_CHECK,
    SMSG_PET_ACTION_SOUND,
    SMSG_PET_DISMISS_SOUND,
    SMSG_GM_TICKET_STATUS_UPDATE,
    SMSG_UPDATE_INSTANCE_OWNERSHIP,
    SMSG_SPELLINSTAKILLLOG,
    SMSG_SPELL_UPDATE_CHAIN_TARGETS,
    SMSG_EXPECTED_SPAM_RECORDS,
    SMSG_DEFENSE_MESSAGE,
}

impl ServerOpcode {
    pub(crate) const fn as_int(&self) -> u16 {
        match self {
            Self::MSG_MOVE_START_FORWARD => 0xb5,
            Self::MSG_MOVE_START_BACKWARD => 0xb6,
            Self::MSG_MOVE_STOP => 0xb7,
            Self::MSG_MOVE_START_STRAFE_LEFT => 0xb8,
            Self::MSG_MOVE_START_STRAFE_RIGHT => 0xb9,
            Self::MSG_MOVE_STOP_STRAFE => 0xba,
            Self::MSG_MOVE_JUMP => 0xbb,
            Self::MSG_MOVE_START_TURN_LEFT => 0xbc,
            Self::MSG_MOVE_START_TURN_RIGHT => 0xbd,
            Self::MSG_MOVE_STOP_TURN => 0xbe,
            Self::MSG_MOVE_START_PITCH_UP => 0xbf,
            Self::MSG_MOVE_START_PITCH_DOWN => 0xc0,
            Self::MSG_MOVE_STOP_PITCH => 0xc1,
            Self::MSG_MOVE_SET_RUN_MODE => 0xc2,
            Self::MSG_MOVE_SET_WALK_MODE => 0xc3,
            Self::MSG_MOVE_TELEPORT_ACK => 0xc7,
            Self::MSG_MOVE_FALL_LAND => 0xc9,
            Self::MSG_MOVE_START_SWIM => 0xca,
            Self::MSG_MOVE_STOP_SWIM => 0xcb,
            Self::MSG_MOVE_SET_FACING => 0xda,
            Self::MSG_MOVE_SET_PITCH => 0xdb,
            Self::MSG_MOVE_WORLDPORT_ACK => 0xdc,
            Self::MSG_MOVE_HEARTBEAT => 0xee,
            Self::MSG_PETITION_DECLINE => 0x1c2,
            Self::MSG_TABARDVENDOR_ACTIVATE => 0x1f2,
            Self::MSG_QUEST_PUSH_RESULT => 0x276,
            Self::MSG_PETITION_RENAME => 0x2c1,
            Self::SMSG_CHAR_CREATE => 0x3a,
            Self::SMSG_CHAR_ENUM => 0x3b,
            Self::SMSG_CHAR_DELETE => 0x3c,
            Self::SMSG_NEW_WORLD => 0x3e,
            Self::SMSG_TRANSFER_PENDING => 0x3f,
            Self::SMSG_TRANSFER_ABORTED => 0x40,
            Self::SMSG_CHARACTER_LOGIN_FAILED => 0x41,
            Self::SMSG_LOGIN_SETTIMESPEED => 0x42,
            Self::SMSG_LOGOUT_RESPONSE => 0x4c,
            Self::SMSG_LOGOUT_COMPLETE => 0x4d,
            Self::SMSG_LOGOUT_CANCEL_ACK => 0x4f,
            Self::SMSG_NAME_QUERY_RESPONSE => 0x51,
            Self::SMSG_PET_NAME_QUERY_RESPONSE => 0x53,
            Self::SMSG_GUILD_QUERY_RESPONSE => 0x55,
            Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE => 0x58,
            Self::SMSG_PAGE_TEXT_QUERY_RESPONSE => 0x5b,
            Self::SMSG_QUEST_QUERY_RESPONSE => 0x5d,
            Self::SMSG_GAMEOBJECT_QUERY_RESPONSE => 0x5f,
            Self::SMSG_CREATURE_QUERY_RESPONSE => 0x61,
            Self::SMSG_WHO => 0x63,
            Self::SMSG_WHOIS => 0x65,
            Self::SMSG_FRIEND_LIST => 0x67,
            Self::SMSG_FRIEND_STATUS => 0x68,
            Self::SMSG_IGNORE_LIST => 0x6b,
            Self::SMSG_GROUP_INVITE => 0x6f,
            Self::SMSG_GROUP_DECLINE => 0x74,
            Self::SMSG_GROUP_UNINVITE => 0x77,
            Self::SMSG_GROUP_SET_LEADER => 0x79,
            Self::SMSG_GROUP_DESTROYED => 0x7c,
            Self::SMSG_GROUP_LIST => 0x7d,
            Self::SMSG_PARTY_MEMBER_STATS => 0x7e,
            Self::SMSG_PARTY_COMMAND_RESULT => 0x7f,
            Self::SMSG_GUILD_INVITE => 0x83,
            Self::SMSG_GUILD_INFO => 0x88,
            Self::SMSG_GUILD_ROSTER => 0x8a,
            Self::SMSG_GUILD_EVENT => 0x92,
            Self::SMSG_GUILD_COMMAND_RESULT => 0x93,
            Self::SMSG_MESSAGECHAT => 0x96,
            Self::SMSG_CHANNEL_NOTIFY => 0x99,
            Self::SMSG_CHANNEL_LIST => 0x9b,
            Self::SMSG_DESTROY_OBJECT => 0xaa,
            Self::SMSG_READ_ITEM_OK => 0xae,
            Self::SMSG_READ_ITEM_FAILED => 0xaf,
            Self::SMSG_ITEM_COOLDOWN => 0xb0,
            Self::SMSG_GAMEOBJECT_CUSTOM_ANIM => 0xb3,
            Self::SMSG_MONSTER_MOVE => 0xdd,
            Self::SMSG_MOVE_WATER_WALK => 0xde,
            Self::SMSG_MOVE_LAND_WALK => 0xdf,
            Self::SMSG_FORCE_RUN_SPEED_CHANGE => 0xe2,
            Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE => 0xe4,
            Self::SMSG_FORCE_SWIM_SPEED_CHANGE => 0xe6,
            Self::SMSG_FORCE_MOVE_ROOT => 0xe8,
            Self::SMSG_FORCE_MOVE_UNROOT => 0xea,
            Self::SMSG_MOVE_KNOCK_BACK => 0xef,
            Self::SMSG_MOVE_FEATHER_FALL => 0xf2,
            Self::SMSG_MOVE_NORMAL_FALL => 0xf3,
            Self::SMSG_MOVE_SET_HOVER => 0xf4,
            Self::SMSG_MOVE_UNSET_HOVER => 0xf5,
            Self::SMSG_TRIGGER_CINEMATIC => 0xfa,
            Self::SMSG_TUTORIAL_FLAGS => 0xfd,
            Self::SMSG_EMOTE => 0x103,
            Self::SMSG_TEXT_EMOTE => 0x105,
            Self::SMSG_INVENTORY_CHANGE_FAILURE => 0x112,
            Self::SMSG_OPEN_CONTAINER => 0x113,
            Self::SMSG_INSPECT => 0x115,
            Self::SMSG_TRADE_STATUS => 0x120,
            Self::SMSG_TRADE_STATUS_EXTENDED => 0x121,
            Self::SMSG_INITIALIZE_FACTIONS => 0x122,
            Self::SMSG_SET_FACTION_VISIBLE => 0x123,
            Self::SMSG_SET_FACTION_STANDING => 0x124,
            Self::SMSG_SET_PROFICIENCY => 0x127,
            Self::SMSG_ACTION_BUTTONS => 0x129,
            Self::SMSG_INITIAL_SPELLS => 0x12a,
            Self::SMSG_LEARNED_SPELL => 0x12b,
            Self::SMSG_SUPERCEDED_SPELL => 0x12c,
            Self::SMSG_CAST_RESULT => 0x130,
            Self::SMSG_SPELL_START => 0x131,
            Self::SMSG_SPELL_GO => 0x132,
            Self::SMSG_SPELL_FAILURE => 0x133,
            Self::SMSG_SPELL_COOLDOWN => 0x134,
            Self::SMSG_COOLDOWN_EVENT => 0x135,
            Self::SMSG_UPDATE_AURA_DURATION => 0x137,
            Self::SMSG_PET_CAST_FAILED => 0x138,
            Self::SMSG_AI_REACTION => 0x13c,
            Self::SMSG_ATTACKSTART => 0x143,
            Self::SMSG_ATTACKSTOP => 0x144,
            Self::SMSG_ATTACKSWING_NOTINRANGE => 0x145,
            Self::SMSG_ATTACKSWING_BADFACING => 0x146,
            Self::SMSG_ATTACKSWING_NOTSTANDING => 0x147,
            Self::SMSG_ATTACKSWING_DEADTARGET => 0x148,
            Self::SMSG_ATTACKSWING_CANT_ATTACK => 0x149,
            Self::SMSG_ATTACKERSTATEUPDATE => 0x14a,
            Self::SMSG_CANCEL_COMBAT => 0x14e,
            Self::SMSG_SPELLHEALLOG => 0x150,
            Self::SMSG_SPELLENERGIZELOG => 0x151,
            Self::SMSG_BINDPOINTUPDATE => 0x155,
            Self::SMSG_PLAYERBOUND => 0x158,
            Self::SMSG_CLIENT_CONTROL_UPDATE => 0x159,
            Self::SMSG_RESURRECT_REQUEST => 0x15b,
            Self::SMSG_LOOT_RESPONSE => 0x160,
            Self::SMSG_LOOT_RELEASE_RESPONSE => 0x161,
            Self::SMSG_LOOT_REMOVED => 0x162,
            Self::SMSG_LOOT_MONEY_NOTIFY => 0x163,
            Self::SMSG_LOOT_CLEAR_MONEY => 0x165,
            Self::SMSG_ITEM_PUSH_RESULT => 0x166,
            Self::SMSG_DUEL_REQUESTED => 0x167,
            Self::SMSG_DUEL_OUTOFBOUNDS => 0x168,
            Self::SMSG_DUEL_INBOUNDS => 0x169,
            Self::SMSG_DUEL_COMPLETE => 0x16a,
            Self::SMSG_DUEL_WINNER => 0x16b,
            Self::SMSG_MOUNTRESULT => 0x16e,
            Self::SMSG_DISMOUNTRESULT => 0x16f,
            Self::SMSG_MOUNTSPECIAL_ANIM => 0x172,
            Self::SMSG_PET_TAME_FAILURE => 0x173,
            Self::SMSG_PET_NAME_INVALID => 0x178,
            Self::SMSG_PET_SPELLS => 0x179,
            Self::SMSG_PET_MODE => 0x17a,
            Self::SMSG_GOSSIP_MESSAGE => 0x17d,
            Self::SMSG_GOSSIP_COMPLETE => 0x17e,
            Self::SMSG_NPC_TEXT_UPDATE => 0x180,
            Self::SMSG_QUESTGIVER_STATUS => 0x183,
            Self::SMSG_QUESTGIVER_QUEST_LIST => 0x185,
            Self::SMSG_QUESTGIVER_QUEST_DETAILS => 0x188,
            Self::SMSG_QUESTGIVER_REQUEST_ITEMS => 0x18b,
            Self::SMSG_QUESTGIVER_OFFER_REWARD => 0x18d,
            Self::SMSG_QUESTGIVER_QUEST_INVALID => 0x18f,
            Self::SMSG_QUESTGIVER_QUEST_COMPLETE => 0x191,
            Self::SMSG_QUESTGIVER_QUEST_FAILED => 0x192,
            Self::SMSG_QUESTLOG_FULL => 0x195,
            Self::SMSG_QUESTUPDATE_FAILED => 0x196,
            Self::SMSG_QUESTUPDATE_FAILEDTIMER => 0x197,
            Self::SMSG_QUESTUPDATE_COMPLETE => 0x198,
            Self::SMSG_QUESTUPDATE_ADD_KILL => 0x199,
            Self::SMSG_QUESTUPDATE_ADD_ITEM => 0x19a,
            Self::SMSG_QUEST_CONFIRM_ACCEPT => 0x19c,
            Self::SMSG_LIST_INVENTORY => 0x19f,
            Self::SMSG_SELL_ITEM => 0x1a1,
            Self::SMSG_BUY_ITEM => 0x1a4,
            Self::SMSG_BUY_FAILED => 0x1a5,
            Self::SMSG_SHOWTAXINODES => 0x1a9,
            Self::SMSG_TAXINODE_STATUS => 0x1ab,
            Self::SMSG_ACTIVATETAXIREPLY => 0x1ae,
            Self::SMSG_NEW_TAXI_PATH => 0x1af,
            Self::SMSG_TRAINER_LIST => 0x1b1,
            Self::SMSG_TRAINER_BUY_SUCCEEDED => 0x1b3,
            Self::SMSG_TRAINER_BUY_FAILED => 0x1b4,
            Self::SMSG_SHOW_BANK => 0x1b8,
            Self::SMSG_BUY_BANK_SLOT_RESULT => 0x1ba,
            Self::SMSG_PETITION_SHOWLIST => 0x1bc,
            Self::SMSG_PETITION_SHOW_SIGNATURES => 0x1bf,
            Self::SMSG_PETITION_SIGN_RESULTS => 0x1c1,
            Self::SMSG_TURN_IN_PETITION_RESULTS => 0x1c5,
            Self::SMSG_PETITION_QUERY_RESPONSE => 0x1c7,
            Self::SMSG_FISH_NOT_HOOKED => 0x1c8,
            Self::SMSG_FISH_ESCAPED => 0x1c9,
            Self::SMSG_NOTIFICATION => 0x1cb,
            Self::SMSG_PLAYED_TIME => 0x1cd,
            Self::SMSG_QUERY_TIME_RESPONSE => 0x1cf,
            Self::SMSG_LOG_XPGAIN => 0x1d0,
            Self::SMSG_LEVELUP_INFO => 0x1d4,
            Self::MSG_MINIMAP_PING => 0x1d5,
            Self::SMSG_RESISTLOG => 0x1d6,
            Self::SMSG_ENCHANTMENTLOG => 0x1d7,
            Self::SMSG_START_MIRROR_TIMER => 0x1d9,
            Self::SMSG_PAUSE_MIRROR_TIMER => 0x1da,
            Self::SMSG_STOP_MIRROR_TIMER => 0x1db,
            Self::SMSG_PONG => 0x1dd,
            Self::SMSG_CLEAR_COOLDOWN => 0x1de,
            Self::SMSG_GAMEOBJECT_PAGETEXT => 0x1df,
            Self::SMSG_SPELL_DELAYED => 0x1e2,
            Self::SMSG_ITEM_TIME_UPDATE => 0x1ea,
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE => 0x1eb,
            Self::SMSG_AUTH_CHALLENGE => 0x1ec,
            Self::SMSG_AUTH_RESPONSE => 0x1ee,
            Self::MSG_SAVE_GUILD_EMBLEM => 0x1f1,
            Self::SMSG_PLAY_SPELL_VISUAL => 0x1f3,
            Self::SMSG_PARTYKILLLOG => 0x1f5,
            Self::SMSG_PLAY_SPELL_IMPACT => 0x1f7,
            Self::SMSG_EXPLORATION_EXPERIENCE => 0x1f8,
            Self::MSG_RANDOM_ROLL => 0x1fb,
            Self::SMSG_ENVIRONMENTALDAMAGELOG => 0x1fc,
            Self::MSG_LOOKING_FOR_GROUP => 0x1ff,
            Self::SMSG_REMOVED_SPELL => 0x203,
            Self::SMSG_GMTICKET_CREATE => 0x206,
            Self::SMSG_GMTICKET_UPDATETEXT => 0x208,
            Self::SMSG_ACCOUNT_DATA_TIMES => 0x209,
            Self::SMSG_GMTICKET_GETTICKET => 0x212,
            Self::SMSG_GAMEOBJECT_SPAWN_ANIM => 0x214,
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM => 0x215,
            Self::MSG_CORPSE_QUERY => 0x216,
            Self::SMSG_GMTICKET_DELETETICKET => 0x218,
            Self::SMSG_CHAT_WRONG_FACTION => 0x219,
            Self::SMSG_GMTICKET_SYSTEMSTATUS => 0x21b,
            Self::SMSG_SET_REST_START => 0x21e,
            Self::SMSG_SPIRIT_HEALER_CONFIRM => 0x222,
            Self::SMSG_GOSSIP_POI => 0x224,
            Self::SMSG_LOGIN_VERIFY_WORLD => 0x236,
            Self::SMSG_MAIL_LIST_RESULT => 0x23b,
            Self::SMSG_BATTLEFIELD_LIST => 0x23d,
            Self::SMSG_ITEM_TEXT_QUERY_RESPONSE => 0x244,
            Self::SMSG_SPELLLOGMISS => 0x24b,
            Self::SMSG_SPELLLOGEXECUTE => 0x24c,
            Self::SMSG_PERIODICAURALOG => 0x24e,
            Self::SMSG_SPELLDAMAGESHIELD => 0x24f,
            Self::SMSG_SPELLNONMELEEDAMAGELOG => 0x250,
            Self::SMSG_ZONE_UNDER_ATTACK => 0x254,
            Self::MSG_AUCTION_HELLO => 0x255,
            Self::SMSG_AUCTION_LIST_RESULT => 0x25c,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT => 0x25d,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION => 0x25e,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION => 0x25f,
            Self::SMSG_PROCRESIST => 0x260,
            Self::SMSG_DISPEL_FAILED => 0x262,
            Self::SMSG_SPELLORDAMAGE_IMMUNE => 0x263,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT => 0x265,
            Self::SMSG_SET_FLAT_SPELL_MODIFIER => 0x266,
            Self::SMSG_SET_PCT_SPELL_MODIFIER => 0x267,
            Self::SMSG_CORPSE_RECLAIM_DELAY => 0x269,
            Self::MSG_LIST_STABLED_PETS => 0x26f,
            Self::SMSG_STABLE_RESULT => 0x273,
            Self::SMSG_PLAY_MUSIC => 0x277,
            Self::SMSG_PLAY_OBJECT_SOUND => 0x278,
            Self::SMSG_SPELLDISPELLOG => 0x27b,
            Self::MSG_QUERY_NEXT_MAIL_TIME => 0x284,
            Self::SMSG_RECEIVED_MAIL => 0x285,
            Self::SMSG_RAID_GROUP_ONLY => 0x286,
            Self::SMSG_PVP_CREDIT => 0x28c,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION => 0x28d,
            Self::SMSG_SERVER_MESSAGE => 0x291,
            Self::SMSG_MEETINGSTONE_SETQUEUE => 0x295,
            Self::SMSG_MEETINGSTONE_COMPLETE => 0x297,
            Self::SMSG_MEETINGSTONE_IN_PROGRESS => 0x298,
            Self::SMSG_MEETINGSTONE_MEMBER_ADDED => 0x299,
            Self::SMSG_CANCEL_AUTO_REPEAT => 0x29c,
            Self::SMSG_STANDSTATE_UPDATE => 0x29d,
            Self::SMSG_LOOT_ALL_PASSED => 0x29e,
            Self::SMSG_LOOT_ROLL_WON => 0x29f,
            Self::SMSG_LOOT_START_ROLL => 0x2a1,
            Self::SMSG_LOOT_ROLL => 0x2a2,
            Self::SMSG_LOOT_MASTER_LIST => 0x2a4,
            Self::SMSG_SET_FORCED_REACTIONS => 0x2a5,
            Self::SMSG_SPELL_FAILED_OTHER => 0x2a6,
            Self::SMSG_GAMEOBJECT_RESET_STATE => 0x2a7,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND => 0x2a9,
            Self::MSG_TALENT_WIPE_CONFIRM => 0x2aa,
            Self::SMSG_SUMMON_REQUEST => 0x2ab,
            Self::SMSG_MONSTER_MOVE_TRANSPORT => 0x2ae,
            Self::SMSG_PET_BROKEN => 0x2af,
            Self::SMSG_FEIGN_DEATH_RESISTED => 0x2b4,
            Self::SMSG_DUEL_COUNTDOWN => 0x2b7,
            Self::SMSG_AREA_TRIGGER_MESSAGE => 0x2b8,
            Self::SMSG_MEETINGSTONE_JOINFAILED => 0x2bb,
            Self::SMSG_PLAYER_SKINNED => 0x2bc,
            Self::SMSG_DURABILITY_DAMAGE_DEATH => 0x2bd,
            Self::SMSG_INIT_WORLD_STATES => 0x2c2,
            Self::SMSG_UPDATE_WORLD_STATE => 0x2c3,
            Self::SMSG_ITEM_NAME_QUERY_RESPONSE => 0x2c5,
            Self::SMSG_PET_ACTION_FEEDBACK => 0x2c6,
            Self::SMSG_CHAR_RENAME => 0x2c8,
            Self::SMSG_INSTANCE_SAVE_CREATED => 0x2cb,
            Self::SMSG_RAID_INSTANCE_INFO => 0x2cc,
            Self::SMSG_PLAY_SOUND => 0x2d2,
            Self::SMSG_BATTLEFIELD_STATUS => 0x2d4,
            Self::MSG_INSPECT_HONOR_STATS => 0x2d6,
            Self::SMSG_FORCE_WALK_SPEED_CHANGE => 0x2da,
            Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE => 0x2dc,
            Self::SMSG_FORCE_TURN_RATE_CHANGE => 0x2de,
            Self::MSG_PVP_LOG_DATA => 0x2e0,
            Self::SMSG_AREA_SPIRIT_HEALER_TIME => 0x2e4,
            Self::SMSG_GROUP_JOINED_BATTLEGROUND => 0x2e8,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS => 0x2e9,
            Self::SMSG_BINDER_CONFIRM => 0x2eb,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED => 0x2ec,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT => 0x2ed,
            Self::SMSG_PET_UNLEARN_CONFIRM => 0x2f1,
            Self::SMSG_PARTY_MEMBER_STATS_FULL => 0x2f2,
            Self::SMSG_WEATHER => 0x2f4,
            Self::SMSG_RAID_INSTANCE_MESSAGE => 0x2fa,
            Self::SMSG_CHAT_RESTRICTED => 0x2fd,
            Self::SMSG_SPLINE_SET_RUN_SPEED => 0x2fe,
            Self::SMSG_SPLINE_SET_RUN_BACK_SPEED => 0x2ff,
            Self::SMSG_SPLINE_SET_SWIM_SPEED => 0x300,
            Self::SMSG_SPLINE_SET_WALK_SPEED => 0x301,
            Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED => 0x302,
            Self::SMSG_SPLINE_SET_TURN_RATE => 0x303,
            Self::SMSG_SPLINE_MOVE_UNROOT => 0x304,
            Self::SMSG_SPLINE_MOVE_FEATHER_FALL => 0x305,
            Self::SMSG_SPLINE_MOVE_NORMAL_FALL => 0x306,
            Self::SMSG_SPLINE_MOVE_SET_HOVER => 0x307,
            Self::SMSG_SPLINE_MOVE_UNSET_HOVER => 0x308,
            Self::SMSG_SPLINE_MOVE_WATER_WALK => 0x309,
            Self::SMSG_SPLINE_MOVE_LAND_WALK => 0x30a,
            Self::SMSG_SPLINE_MOVE_START_SWIM => 0x30b,
            Self::SMSG_SPLINE_MOVE_STOP_SWIM => 0x30c,
            Self::SMSG_SPLINE_MOVE_SET_RUN_MODE => 0x30d,
            Self::SMSG_SPLINE_MOVE_SET_WALK_MODE => 0x30e,
            Self::SMSG_SPLINE_MOVE_ROOT => 0x31a,
            Self::SMSG_INVALIDATE_PLAYER => 0x31c,
            Self::SMSG_INSTANCE_RESET => 0x31e,
            Self::SMSG_INSTANCE_RESET_FAILED => 0x31f,
            Self::SMSG_UPDATE_LAST_INSTANCE => 0x320,
            Self::MSG_RAID_TARGET_UPDATE => 0x321,
            Self::MSG_RAID_READY_CHECK => 0x322,
            Self::SMSG_PET_ACTION_SOUND => 0x324,
            Self::SMSG_PET_DISMISS_SOUND => 0x325,
            Self::SMSG_GM_TICKET_STATUS_UPDATE => 0x328,
            Self::SMSG_UPDATE_INSTANCE_OWNERSHIP => 0x32b,
            Self::SMSG_SPELLINSTAKILLLOG => 0x32f,
            Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS => 0x330,
            Self::SMSG_EXPECTED_SPAM_RECORDS => 0x332,
            Self::SMSG_DEFENSE_MESSAGE => 0x33b,
        }
    }

}

impl ServerOpcode {
    pub fn new(opcode: u16) -> std::result::Result<Self, ServerOpcodeError> {
        match opcode {
            0xb5 => Ok(Self::MSG_MOVE_START_FORWARD),
            0xb6 => Ok(Self::MSG_MOVE_START_BACKWARD),
            0xb7 => Ok(Self::MSG_MOVE_STOP),
            0xb8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT),
            0xb9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT),
            0xba => Ok(Self::MSG_MOVE_STOP_STRAFE),
            0xbb => Ok(Self::MSG_MOVE_JUMP),
            0xbc => Ok(Self::MSG_MOVE_START_TURN_LEFT),
            0xbd => Ok(Self::MSG_MOVE_START_TURN_RIGHT),
            0xbe => Ok(Self::MSG_MOVE_STOP_TURN),
            0xbf => Ok(Self::MSG_MOVE_START_PITCH_UP),
            0xc0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN),
            0xc1 => Ok(Self::MSG_MOVE_STOP_PITCH),
            0xc2 => Ok(Self::MSG_MOVE_SET_RUN_MODE),
            0xc3 => Ok(Self::MSG_MOVE_SET_WALK_MODE),
            0xc7 => Ok(Self::MSG_MOVE_TELEPORT_ACK),
            0xc9 => Ok(Self::MSG_MOVE_FALL_LAND),
            0xca => Ok(Self::MSG_MOVE_START_SWIM),
            0xcb => Ok(Self::MSG_MOVE_STOP_SWIM),
            0xda => Ok(Self::MSG_MOVE_SET_FACING),
            0xdb => Ok(Self::MSG_MOVE_SET_PITCH),
            0xdc => Ok(Self::MSG_MOVE_WORLDPORT_ACK),
            0xee => Ok(Self::MSG_MOVE_HEARTBEAT),
            0x1c2 => Ok(Self::MSG_PETITION_DECLINE),
            0x1f2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE),
            0x276 => Ok(Self::MSG_QUEST_PUSH_RESULT),
            0x2c1 => Ok(Self::MSG_PETITION_RENAME),
            0x3a => Ok(Self::SMSG_CHAR_CREATE),
            0x3b => Ok(Self::SMSG_CHAR_ENUM),
            0x3c => Ok(Self::SMSG_CHAR_DELETE),
            0x3e => Ok(Self::SMSG_NEW_WORLD),
            0x3f => Ok(Self::SMSG_TRANSFER_PENDING),
            0x40 => Ok(Self::SMSG_TRANSFER_ABORTED),
            0x41 => Ok(Self::SMSG_CHARACTER_LOGIN_FAILED),
            0x42 => Ok(Self::SMSG_LOGIN_SETTIMESPEED),
            0x4c => Ok(Self::SMSG_LOGOUT_RESPONSE),
            0x4d => Ok(Self::SMSG_LOGOUT_COMPLETE),
            0x4f => Ok(Self::SMSG_LOGOUT_CANCEL_ACK),
            0x51 => Ok(Self::SMSG_NAME_QUERY_RESPONSE),
            0x53 => Ok(Self::SMSG_PET_NAME_QUERY_RESPONSE),
            0x55 => Ok(Self::SMSG_GUILD_QUERY_RESPONSE),
            0x58 => Ok(Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE),
            0x5b => Ok(Self::SMSG_PAGE_TEXT_QUERY_RESPONSE),
            0x5d => Ok(Self::SMSG_QUEST_QUERY_RESPONSE),
            0x5f => Ok(Self::SMSG_GAMEOBJECT_QUERY_RESPONSE),
            0x61 => Ok(Self::SMSG_CREATURE_QUERY_RESPONSE),
            0x63 => Ok(Self::SMSG_WHO),
            0x65 => Ok(Self::SMSG_WHOIS),
            0x67 => Ok(Self::SMSG_FRIEND_LIST),
            0x68 => Ok(Self::SMSG_FRIEND_STATUS),
            0x6b => Ok(Self::SMSG_IGNORE_LIST),
            0x6f => Ok(Self::SMSG_GROUP_INVITE),
            0x74 => Ok(Self::SMSG_GROUP_DECLINE),
            0x77 => Ok(Self::SMSG_GROUP_UNINVITE),
            0x79 => Ok(Self::SMSG_GROUP_SET_LEADER),
            0x7c => Ok(Self::SMSG_GROUP_DESTROYED),
            0x7d => Ok(Self::SMSG_GROUP_LIST),
            0x7e => Ok(Self::SMSG_PARTY_MEMBER_STATS),
            0x7f => Ok(Self::SMSG_PARTY_COMMAND_RESULT),
            0x83 => Ok(Self::SMSG_GUILD_INVITE),
            0x88 => Ok(Self::SMSG_GUILD_INFO),
            0x8a => Ok(Self::SMSG_GUILD_ROSTER),
            0x92 => Ok(Self::SMSG_GUILD_EVENT),
            0x93 => Ok(Self::SMSG_GUILD_COMMAND_RESULT),
            0x96 => Ok(Self::SMSG_MESSAGECHAT),
            0x99 => Ok(Self::SMSG_CHANNEL_NOTIFY),
            0x9b => Ok(Self::SMSG_CHANNEL_LIST),
            0xaa => Ok(Self::SMSG_DESTROY_OBJECT),
            0xae => Ok(Self::SMSG_READ_ITEM_OK),
            0xaf => Ok(Self::SMSG_READ_ITEM_FAILED),
            0xb0 => Ok(Self::SMSG_ITEM_COOLDOWN),
            0xb3 => Ok(Self::SMSG_GAMEOBJECT_CUSTOM_ANIM),
            0xdd => Ok(Self::SMSG_MONSTER_MOVE),
            0xde => Ok(Self::SMSG_MOVE_WATER_WALK),
            0xdf => Ok(Self::SMSG_MOVE_LAND_WALK),
            0xe2 => Ok(Self::SMSG_FORCE_RUN_SPEED_CHANGE),
            0xe4 => Ok(Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE),
            0xe6 => Ok(Self::SMSG_FORCE_SWIM_SPEED_CHANGE),
            0xe8 => Ok(Self::SMSG_FORCE_MOVE_ROOT),
            0xea => Ok(Self::SMSG_FORCE_MOVE_UNROOT),
            0xef => Ok(Self::SMSG_MOVE_KNOCK_BACK),
            0xf2 => Ok(Self::SMSG_MOVE_FEATHER_FALL),
            0xf3 => Ok(Self::SMSG_MOVE_NORMAL_FALL),
            0xf4 => Ok(Self::SMSG_MOVE_SET_HOVER),
            0xf5 => Ok(Self::SMSG_MOVE_UNSET_HOVER),
            0xfa => Ok(Self::SMSG_TRIGGER_CINEMATIC),
            0xfd => Ok(Self::SMSG_TUTORIAL_FLAGS),
            0x103 => Ok(Self::SMSG_EMOTE),
            0x105 => Ok(Self::SMSG_TEXT_EMOTE),
            0x112 => Ok(Self::SMSG_INVENTORY_CHANGE_FAILURE),
            0x113 => Ok(Self::SMSG_OPEN_CONTAINER),
            0x115 => Ok(Self::SMSG_INSPECT),
            0x120 => Ok(Self::SMSG_TRADE_STATUS),
            0x121 => Ok(Self::SMSG_TRADE_STATUS_EXTENDED),
            0x122 => Ok(Self::SMSG_INITIALIZE_FACTIONS),
            0x123 => Ok(Self::SMSG_SET_FACTION_VISIBLE),
            0x124 => Ok(Self::SMSG_SET_FACTION_STANDING),
            0x127 => Ok(Self::SMSG_SET_PROFICIENCY),
            0x129 => Ok(Self::SMSG_ACTION_BUTTONS),
            0x12a => Ok(Self::SMSG_INITIAL_SPELLS),
            0x12b => Ok(Self::SMSG_LEARNED_SPELL),
            0x12c => Ok(Self::SMSG_SUPERCEDED_SPELL),
            0x130 => Ok(Self::SMSG_CAST_RESULT),
            0x131 => Ok(Self::SMSG_SPELL_START),
            0x132 => Ok(Self::SMSG_SPELL_GO),
            0x133 => Ok(Self::SMSG_SPELL_FAILURE),
            0x134 => Ok(Self::SMSG_SPELL_COOLDOWN),
            0x135 => Ok(Self::SMSG_COOLDOWN_EVENT),
            0x137 => Ok(Self::SMSG_UPDATE_AURA_DURATION),
            0x138 => Ok(Self::SMSG_PET_CAST_FAILED),
            0x13c => Ok(Self::SMSG_AI_REACTION),
            0x143 => Ok(Self::SMSG_ATTACKSTART),
            0x144 => Ok(Self::SMSG_ATTACKSTOP),
            0x145 => Ok(Self::SMSG_ATTACKSWING_NOTINRANGE),
            0x146 => Ok(Self::SMSG_ATTACKSWING_BADFACING),
            0x147 => Ok(Self::SMSG_ATTACKSWING_NOTSTANDING),
            0x148 => Ok(Self::SMSG_ATTACKSWING_DEADTARGET),
            0x149 => Ok(Self::SMSG_ATTACKSWING_CANT_ATTACK),
            0x14a => Ok(Self::SMSG_ATTACKERSTATEUPDATE),
            0x14e => Ok(Self::SMSG_CANCEL_COMBAT),
            0x150 => Ok(Self::SMSG_SPELLHEALLOG),
            0x151 => Ok(Self::SMSG_SPELLENERGIZELOG),
            0x155 => Ok(Self::SMSG_BINDPOINTUPDATE),
            0x158 => Ok(Self::SMSG_PLAYERBOUND),
            0x159 => Ok(Self::SMSG_CLIENT_CONTROL_UPDATE),
            0x15b => Ok(Self::SMSG_RESURRECT_REQUEST),
            0x160 => Ok(Self::SMSG_LOOT_RESPONSE),
            0x161 => Ok(Self::SMSG_LOOT_RELEASE_RESPONSE),
            0x162 => Ok(Self::SMSG_LOOT_REMOVED),
            0x163 => Ok(Self::SMSG_LOOT_MONEY_NOTIFY),
            0x165 => Ok(Self::SMSG_LOOT_CLEAR_MONEY),
            0x166 => Ok(Self::SMSG_ITEM_PUSH_RESULT),
            0x167 => Ok(Self::SMSG_DUEL_REQUESTED),
            0x168 => Ok(Self::SMSG_DUEL_OUTOFBOUNDS),
            0x169 => Ok(Self::SMSG_DUEL_INBOUNDS),
            0x16a => Ok(Self::SMSG_DUEL_COMPLETE),
            0x16b => Ok(Self::SMSG_DUEL_WINNER),
            0x16e => Ok(Self::SMSG_MOUNTRESULT),
            0x16f => Ok(Self::SMSG_DISMOUNTRESULT),
            0x172 => Ok(Self::SMSG_MOUNTSPECIAL_ANIM),
            0x173 => Ok(Self::SMSG_PET_TAME_FAILURE),
            0x178 => Ok(Self::SMSG_PET_NAME_INVALID),
            0x179 => Ok(Self::SMSG_PET_SPELLS),
            0x17a => Ok(Self::SMSG_PET_MODE),
            0x17d => Ok(Self::SMSG_GOSSIP_MESSAGE),
            0x17e => Ok(Self::SMSG_GOSSIP_COMPLETE),
            0x180 => Ok(Self::SMSG_NPC_TEXT_UPDATE),
            0x183 => Ok(Self::SMSG_QUESTGIVER_STATUS),
            0x185 => Ok(Self::SMSG_QUESTGIVER_QUEST_LIST),
            0x188 => Ok(Self::SMSG_QUESTGIVER_QUEST_DETAILS),
            0x18b => Ok(Self::SMSG_QUESTGIVER_REQUEST_ITEMS),
            0x18d => Ok(Self::SMSG_QUESTGIVER_OFFER_REWARD),
            0x18f => Ok(Self::SMSG_QUESTGIVER_QUEST_INVALID),
            0x191 => Ok(Self::SMSG_QUESTGIVER_QUEST_COMPLETE),
            0x192 => Ok(Self::SMSG_QUESTGIVER_QUEST_FAILED),
            0x195 => Ok(Self::SMSG_QUESTLOG_FULL),
            0x196 => Ok(Self::SMSG_QUESTUPDATE_FAILED),
            0x197 => Ok(Self::SMSG_QUESTUPDATE_FAILEDTIMER),
            0x198 => Ok(Self::SMSG_QUESTUPDATE_COMPLETE),
            0x199 => Ok(Self::SMSG_QUESTUPDATE_ADD_KILL),
            0x19a => Ok(Self::SMSG_QUESTUPDATE_ADD_ITEM),
            0x19c => Ok(Self::SMSG_QUEST_CONFIRM_ACCEPT),
            0x19f => Ok(Self::SMSG_LIST_INVENTORY),
            0x1a1 => Ok(Self::SMSG_SELL_ITEM),
            0x1a4 => Ok(Self::SMSG_BUY_ITEM),
            0x1a5 => Ok(Self::SMSG_BUY_FAILED),
            0x1a9 => Ok(Self::SMSG_SHOWTAXINODES),
            0x1ab => Ok(Self::SMSG_TAXINODE_STATUS),
            0x1ae => Ok(Self::SMSG_ACTIVATETAXIREPLY),
            0x1af => Ok(Self::SMSG_NEW_TAXI_PATH),
            0x1b1 => Ok(Self::SMSG_TRAINER_LIST),
            0x1b3 => Ok(Self::SMSG_TRAINER_BUY_SUCCEEDED),
            0x1b4 => Ok(Self::SMSG_TRAINER_BUY_FAILED),
            0x1b8 => Ok(Self::SMSG_SHOW_BANK),
            0x1ba => Ok(Self::SMSG_BUY_BANK_SLOT_RESULT),
            0x1bc => Ok(Self::SMSG_PETITION_SHOWLIST),
            0x1bf => Ok(Self::SMSG_PETITION_SHOW_SIGNATURES),
            0x1c1 => Ok(Self::SMSG_PETITION_SIGN_RESULTS),
            0x1c5 => Ok(Self::SMSG_TURN_IN_PETITION_RESULTS),
            0x1c7 => Ok(Self::SMSG_PETITION_QUERY_RESPONSE),
            0x1c8 => Ok(Self::SMSG_FISH_NOT_HOOKED),
            0x1c9 => Ok(Self::SMSG_FISH_ESCAPED),
            0x1cb => Ok(Self::SMSG_NOTIFICATION),
            0x1cd => Ok(Self::SMSG_PLAYED_TIME),
            0x1cf => Ok(Self::SMSG_QUERY_TIME_RESPONSE),
            0x1d0 => Ok(Self::SMSG_LOG_XPGAIN),
            0x1d4 => Ok(Self::SMSG_LEVELUP_INFO),
            0x1d5 => Ok(Self::MSG_MINIMAP_PING),
            0x1d6 => Ok(Self::SMSG_RESISTLOG),
            0x1d7 => Ok(Self::SMSG_ENCHANTMENTLOG),
            0x1d9 => Ok(Self::SMSG_START_MIRROR_TIMER),
            0x1da => Ok(Self::SMSG_PAUSE_MIRROR_TIMER),
            0x1db => Ok(Self::SMSG_STOP_MIRROR_TIMER),
            0x1dd => Ok(Self::SMSG_PONG),
            0x1de => Ok(Self::SMSG_CLEAR_COOLDOWN),
            0x1df => Ok(Self::SMSG_GAMEOBJECT_PAGETEXT),
            0x1e2 => Ok(Self::SMSG_SPELL_DELAYED),
            0x1ea => Ok(Self::SMSG_ITEM_TIME_UPDATE),
            0x1eb => Ok(Self::SMSG_ITEM_ENCHANT_TIME_UPDATE),
            0x1ec => Ok(Self::SMSG_AUTH_CHALLENGE),
            0x1ee => Ok(Self::SMSG_AUTH_RESPONSE),
            0x1f1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM),
            0x1f3 => Ok(Self::SMSG_PLAY_SPELL_VISUAL),
            0x1f5 => Ok(Self::SMSG_PARTYKILLLOG),
            0x1f7 => Ok(Self::SMSG_PLAY_SPELL_IMPACT),
            0x1f8 => Ok(Self::SMSG_EXPLORATION_EXPERIENCE),
            0x1fb => Ok(Self::MSG_RANDOM_ROLL),
            0x1fc => Ok(Self::SMSG_ENVIRONMENTALDAMAGELOG),
            0x1ff => Ok(Self::MSG_LOOKING_FOR_GROUP),
            0x203 => Ok(Self::SMSG_REMOVED_SPELL),
            0x206 => Ok(Self::SMSG_GMTICKET_CREATE),
            0x208 => Ok(Self::SMSG_GMTICKET_UPDATETEXT),
            0x209 => Ok(Self::SMSG_ACCOUNT_DATA_TIMES),
            0x212 => Ok(Self::SMSG_GMTICKET_GETTICKET),
            0x214 => Ok(Self::SMSG_GAMEOBJECT_SPAWN_ANIM),
            0x215 => Ok(Self::SMSG_GAMEOBJECT_DESPAWN_ANIM),
            0x216 => Ok(Self::MSG_CORPSE_QUERY),
            0x218 => Ok(Self::SMSG_GMTICKET_DELETETICKET),
            0x219 => Ok(Self::SMSG_CHAT_WRONG_FACTION),
            0x21b => Ok(Self::SMSG_GMTICKET_SYSTEMSTATUS),
            0x21e => Ok(Self::SMSG_SET_REST_START),
            0x222 => Ok(Self::SMSG_SPIRIT_HEALER_CONFIRM),
            0x224 => Ok(Self::SMSG_GOSSIP_POI),
            0x236 => Ok(Self::SMSG_LOGIN_VERIFY_WORLD),
            0x23b => Ok(Self::SMSG_MAIL_LIST_RESULT),
            0x23d => Ok(Self::SMSG_BATTLEFIELD_LIST),
            0x244 => Ok(Self::SMSG_ITEM_TEXT_QUERY_RESPONSE),
            0x24b => Ok(Self::SMSG_SPELLLOGMISS),
            0x24c => Ok(Self::SMSG_SPELLLOGEXECUTE),
            0x24e => Ok(Self::SMSG_PERIODICAURALOG),
            0x24f => Ok(Self::SMSG_SPELLDAMAGESHIELD),
            0x250 => Ok(Self::SMSG_SPELLNONMELEEDAMAGELOG),
            0x254 => Ok(Self::SMSG_ZONE_UNDER_ATTACK),
            0x255 => Ok(Self::MSG_AUCTION_HELLO),
            0x25c => Ok(Self::SMSG_AUCTION_LIST_RESULT),
            0x25d => Ok(Self::SMSG_AUCTION_OWNER_LIST_RESULT),
            0x25e => Ok(Self::SMSG_AUCTION_BIDDER_NOTIFICATION),
            0x25f => Ok(Self::SMSG_AUCTION_OWNER_NOTIFICATION),
            0x260 => Ok(Self::SMSG_PROCRESIST),
            0x262 => Ok(Self::SMSG_DISPEL_FAILED),
            0x263 => Ok(Self::SMSG_SPELLORDAMAGE_IMMUNE),
            0x265 => Ok(Self::SMSG_AUCTION_BIDDER_LIST_RESULT),
            0x266 => Ok(Self::SMSG_SET_FLAT_SPELL_MODIFIER),
            0x267 => Ok(Self::SMSG_SET_PCT_SPELL_MODIFIER),
            0x269 => Ok(Self::SMSG_CORPSE_RECLAIM_DELAY),
            0x26f => Ok(Self::MSG_LIST_STABLED_PETS),
            0x273 => Ok(Self::SMSG_STABLE_RESULT),
            0x277 => Ok(Self::SMSG_PLAY_MUSIC),
            0x278 => Ok(Self::SMSG_PLAY_OBJECT_SOUND),
            0x27b => Ok(Self::SMSG_SPELLDISPELLOG),
            0x284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME),
            0x285 => Ok(Self::SMSG_RECEIVED_MAIL),
            0x286 => Ok(Self::SMSG_RAID_GROUP_ONLY),
            0x28c => Ok(Self::SMSG_PVP_CREDIT),
            0x28d => Ok(Self::SMSG_AUCTION_REMOVED_NOTIFICATION),
            0x291 => Ok(Self::SMSG_SERVER_MESSAGE),
            0x295 => Ok(Self::SMSG_MEETINGSTONE_SETQUEUE),
            0x297 => Ok(Self::SMSG_MEETINGSTONE_COMPLETE),
            0x298 => Ok(Self::SMSG_MEETINGSTONE_IN_PROGRESS),
            0x299 => Ok(Self::SMSG_MEETINGSTONE_MEMBER_ADDED),
            0x29c => Ok(Self::SMSG_CANCEL_AUTO_REPEAT),
            0x29d => Ok(Self::SMSG_STANDSTATE_UPDATE),
            0x29e => Ok(Self::SMSG_LOOT_ALL_PASSED),
            0x29f => Ok(Self::SMSG_LOOT_ROLL_WON),
            0x2a1 => Ok(Self::SMSG_LOOT_START_ROLL),
            0x2a2 => Ok(Self::SMSG_LOOT_ROLL),
            0x2a4 => Ok(Self::SMSG_LOOT_MASTER_LIST),
            0x2a5 => Ok(Self::SMSG_SET_FORCED_REACTIONS),
            0x2a6 => Ok(Self::SMSG_SPELL_FAILED_OTHER),
            0x2a7 => Ok(Self::SMSG_GAMEOBJECT_RESET_STATE),
            0x2a9 => Ok(Self::SMSG_CHAT_PLAYER_NOT_FOUND),
            0x2aa => Ok(Self::MSG_TALENT_WIPE_CONFIRM),
            0x2ab => Ok(Self::SMSG_SUMMON_REQUEST),
            0x2ae => Ok(Self::SMSG_MONSTER_MOVE_TRANSPORT),
            0x2af => Ok(Self::SMSG_PET_BROKEN),
            0x2b4 => Ok(Self::SMSG_FEIGN_DEATH_RESISTED),
            0x2b7 => Ok(Self::SMSG_DUEL_COUNTDOWN),
            0x2b8 => Ok(Self::SMSG_AREA_TRIGGER_MESSAGE),
            0x2bb => Ok(Self::SMSG_MEETINGSTONE_JOINFAILED),
            0x2bc => Ok(Self::SMSG_PLAYER_SKINNED),
            0x2bd => Ok(Self::SMSG_DURABILITY_DAMAGE_DEATH),
            0x2c2 => Ok(Self::SMSG_INIT_WORLD_STATES),
            0x2c3 => Ok(Self::SMSG_UPDATE_WORLD_STATE),
            0x2c5 => Ok(Self::SMSG_ITEM_NAME_QUERY_RESPONSE),
            0x2c6 => Ok(Self::SMSG_PET_ACTION_FEEDBACK),
            0x2c8 => Ok(Self::SMSG_CHAR_RENAME),
            0x2cb => Ok(Self::SMSG_INSTANCE_SAVE_CREATED),
            0x2cc => Ok(Self::SMSG_RAID_INSTANCE_INFO),
            0x2d2 => Ok(Self::SMSG_PLAY_SOUND),
            0x2d4 => Ok(Self::SMSG_BATTLEFIELD_STATUS),
            0x2d6 => Ok(Self::MSG_INSPECT_HONOR_STATS),
            0x2da => Ok(Self::SMSG_FORCE_WALK_SPEED_CHANGE),
            0x2dc => Ok(Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE),
            0x2de => Ok(Self::SMSG_FORCE_TURN_RATE_CHANGE),
            0x2e0 => Ok(Self::MSG_PVP_LOG_DATA),
            0x2e4 => Ok(Self::SMSG_AREA_SPIRIT_HEALER_TIME),
            0x2e8 => Ok(Self::SMSG_GROUP_JOINED_BATTLEGROUND),
            0x2e9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS),
            0x2eb => Ok(Self::SMSG_BINDER_CONFIRM),
            0x2ec => Ok(Self::SMSG_BATTLEGROUND_PLAYER_JOINED),
            0x2ed => Ok(Self::SMSG_BATTLEGROUND_PLAYER_LEFT),
            0x2f1 => Ok(Self::SMSG_PET_UNLEARN_CONFIRM),
            0x2f2 => Ok(Self::SMSG_PARTY_MEMBER_STATS_FULL),
            0x2f4 => Ok(Self::SMSG_WEATHER),
            0x2fa => Ok(Self::SMSG_RAID_INSTANCE_MESSAGE),
            0x2fd => Ok(Self::SMSG_CHAT_RESTRICTED),
            0x2fe => Ok(Self::SMSG_SPLINE_SET_RUN_SPEED),
            0x2ff => Ok(Self::SMSG_SPLINE_SET_RUN_BACK_SPEED),
            0x300 => Ok(Self::SMSG_SPLINE_SET_SWIM_SPEED),
            0x301 => Ok(Self::SMSG_SPLINE_SET_WALK_SPEED),
            0x302 => Ok(Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED),
            0x303 => Ok(Self::SMSG_SPLINE_SET_TURN_RATE),
            0x304 => Ok(Self::SMSG_SPLINE_MOVE_UNROOT),
            0x305 => Ok(Self::SMSG_SPLINE_MOVE_FEATHER_FALL),
            0x306 => Ok(Self::SMSG_SPLINE_MOVE_NORMAL_FALL),
            0x307 => Ok(Self::SMSG_SPLINE_MOVE_SET_HOVER),
            0x308 => Ok(Self::SMSG_SPLINE_MOVE_UNSET_HOVER),
            0x309 => Ok(Self::SMSG_SPLINE_MOVE_WATER_WALK),
            0x30a => Ok(Self::SMSG_SPLINE_MOVE_LAND_WALK),
            0x30b => Ok(Self::SMSG_SPLINE_MOVE_START_SWIM),
            0x30c => Ok(Self::SMSG_SPLINE_MOVE_STOP_SWIM),
            0x30d => Ok(Self::SMSG_SPLINE_MOVE_SET_RUN_MODE),
            0x30e => Ok(Self::SMSG_SPLINE_MOVE_SET_WALK_MODE),
            0x31a => Ok(Self::SMSG_SPLINE_MOVE_ROOT),
            0x31c => Ok(Self::SMSG_INVALIDATE_PLAYER),
            0x31e => Ok(Self::SMSG_INSTANCE_RESET),
            0x31f => Ok(Self::SMSG_INSTANCE_RESET_FAILED),
            0x320 => Ok(Self::SMSG_UPDATE_LAST_INSTANCE),
            0x321 => Ok(Self::MSG_RAID_TARGET_UPDATE),
            0x322 => Ok(Self::MSG_RAID_READY_CHECK),
            0x324 => Ok(Self::SMSG_PET_ACTION_SOUND),
            0x325 => Ok(Self::SMSG_PET_DISMISS_SOUND),
            0x328 => Ok(Self::SMSG_GM_TICKET_STATUS_UPDATE),
            0x32b => Ok(Self::SMSG_UPDATE_INSTANCE_OWNERSHIP),
            0x32f => Ok(Self::SMSG_SPELLINSTAKILLLOG),
            0x330 => Ok(Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS),
            0x332 => Ok(Self::SMSG_EXPECTED_SPAM_RECORDS),
            0x33b => Ok(Self::SMSG_DEFENSE_MESSAGE),
            opcode => Err(ServerOpcodeError::InvalidOpcode(opcode)),
        }
    }

}

impl From<&ServerOpcodeMessage> for ServerOpcode {
    fn from(e: &ServerOpcodeMessage) -> Self {
        match *e {
            ServerOpcodeMessage::MSG_MOVE_START_FORWARD(_) => Self::MSG_MOVE_START_FORWARD,
            ServerOpcodeMessage::MSG_MOVE_START_BACKWARD(_) => Self::MSG_MOVE_START_BACKWARD,
            ServerOpcodeMessage::MSG_MOVE_STOP(_) => Self::MSG_MOVE_STOP,
            ServerOpcodeMessage::MSG_MOVE_START_STRAFE_LEFT(_) => Self::MSG_MOVE_START_STRAFE_LEFT,
            ServerOpcodeMessage::MSG_MOVE_START_STRAFE_RIGHT(_) => Self::MSG_MOVE_START_STRAFE_RIGHT,
            ServerOpcodeMessage::MSG_MOVE_STOP_STRAFE(_) => Self::MSG_MOVE_STOP_STRAFE,
            ServerOpcodeMessage::MSG_MOVE_JUMP(_) => Self::MSG_MOVE_JUMP,
            ServerOpcodeMessage::MSG_MOVE_START_TURN_LEFT(_) => Self::MSG_MOVE_START_TURN_LEFT,
            ServerOpcodeMessage::MSG_MOVE_START_TURN_RIGHT(_) => Self::MSG_MOVE_START_TURN_RIGHT,
            ServerOpcodeMessage::MSG_MOVE_STOP_TURN(_) => Self::MSG_MOVE_STOP_TURN,
            ServerOpcodeMessage::MSG_MOVE_START_PITCH_UP(_) => Self::MSG_MOVE_START_PITCH_UP,
            ServerOpcodeMessage::MSG_MOVE_START_PITCH_DOWN(_) => Self::MSG_MOVE_START_PITCH_DOWN,
            ServerOpcodeMessage::MSG_MOVE_STOP_PITCH(_) => Self::MSG_MOVE_STOP_PITCH,
            ServerOpcodeMessage::MSG_MOVE_SET_RUN_MODE(_) => Self::MSG_MOVE_SET_RUN_MODE,
            ServerOpcodeMessage::MSG_MOVE_SET_WALK_MODE(_) => Self::MSG_MOVE_SET_WALK_MODE,
            ServerOpcodeMessage::MSG_MOVE_TELEPORT_ACK(_) => Self::MSG_MOVE_TELEPORT_ACK,
            ServerOpcodeMessage::MSG_MOVE_FALL_LAND(_) => Self::MSG_MOVE_FALL_LAND,
            ServerOpcodeMessage::MSG_MOVE_START_SWIM(_) => Self::MSG_MOVE_START_SWIM,
            ServerOpcodeMessage::MSG_MOVE_STOP_SWIM(_) => Self::MSG_MOVE_STOP_SWIM,
            ServerOpcodeMessage::MSG_MOVE_SET_FACING(_) => Self::MSG_MOVE_SET_FACING,
            ServerOpcodeMessage::MSG_MOVE_SET_PITCH(_) => Self::MSG_MOVE_SET_PITCH,
            ServerOpcodeMessage::MSG_MOVE_WORLDPORT_ACK(_) => Self::MSG_MOVE_WORLDPORT_ACK,
            ServerOpcodeMessage::MSG_MOVE_HEARTBEAT(_) => Self::MSG_MOVE_HEARTBEAT,
            ServerOpcodeMessage::MSG_PETITION_DECLINE(_) => Self::MSG_PETITION_DECLINE,
            ServerOpcodeMessage::MSG_TABARDVENDOR_ACTIVATE(_) => Self::MSG_TABARDVENDOR_ACTIVATE,
            ServerOpcodeMessage::MSG_QUEST_PUSH_RESULT(_) => Self::MSG_QUEST_PUSH_RESULT,
            ServerOpcodeMessage::MSG_PETITION_RENAME(_) => Self::MSG_PETITION_RENAME,
            ServerOpcodeMessage::SMSG_CHAR_CREATE(_) => Self::SMSG_CHAR_CREATE,
            ServerOpcodeMessage::SMSG_CHAR_ENUM(_) => Self::SMSG_CHAR_ENUM,
            ServerOpcodeMessage::SMSG_CHAR_DELETE(_) => Self::SMSG_CHAR_DELETE,
            ServerOpcodeMessage::SMSG_NEW_WORLD(_) => Self::SMSG_NEW_WORLD,
            ServerOpcodeMessage::SMSG_TRANSFER_PENDING(_) => Self::SMSG_TRANSFER_PENDING,
            ServerOpcodeMessage::SMSG_TRANSFER_ABORTED(_) => Self::SMSG_TRANSFER_ABORTED,
            ServerOpcodeMessage::SMSG_CHARACTER_LOGIN_FAILED(_) => Self::SMSG_CHARACTER_LOGIN_FAILED,
            ServerOpcodeMessage::SMSG_LOGIN_SETTIMESPEED(_) => Self::SMSG_LOGIN_SETTIMESPEED,
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(_) => Self::SMSG_LOGOUT_RESPONSE,
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE(_) => Self::SMSG_LOGOUT_COMPLETE,
            ServerOpcodeMessage::SMSG_LOGOUT_CANCEL_ACK(_) => Self::SMSG_LOGOUT_CANCEL_ACK,
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(_) => Self::SMSG_NAME_QUERY_RESPONSE,
            ServerOpcodeMessage::SMSG_PET_NAME_QUERY_RESPONSE(_) => Self::SMSG_PET_NAME_QUERY_RESPONSE,
            ServerOpcodeMessage::SMSG_GUILD_QUERY_RESPONSE(_) => Self::SMSG_GUILD_QUERY_RESPONSE,
            ServerOpcodeMessage::SMSG_ITEM_QUERY_SINGLE_RESPONSE(_) => Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE,
            ServerOpcodeMessage::SMSG_PAGE_TEXT_QUERY_RESPONSE(_) => Self::SMSG_PAGE_TEXT_QUERY_RESPONSE,
            ServerOpcodeMessage::SMSG_QUEST_QUERY_RESPONSE(_) => Self::SMSG_QUEST_QUERY_RESPONSE,
            ServerOpcodeMessage::SMSG_GAMEOBJECT_QUERY_RESPONSE(_) => Self::SMSG_GAMEOBJECT_QUERY_RESPONSE,
            ServerOpcodeMessage::SMSG_CREATURE_QUERY_RESPONSE(_) => Self::SMSG_CREATURE_QUERY_RESPONSE,
            ServerOpcodeMessage::SMSG_WHO(_) => Self::SMSG_WHO,
            ServerOpcodeMessage::SMSG_WHOIS(_) => Self::SMSG_WHOIS,
            ServerOpcodeMessage::SMSG_FRIEND_LIST(_) => Self::SMSG_FRIEND_LIST,
            ServerOpcodeMessage::SMSG_FRIEND_STATUS(_) => Self::SMSG_FRIEND_STATUS,
            ServerOpcodeMessage::SMSG_IGNORE_LIST(_) => Self::SMSG_IGNORE_LIST,
            ServerOpcodeMessage::SMSG_GROUP_INVITE(_) => Self::SMSG_GROUP_INVITE,
            ServerOpcodeMessage::SMSG_GROUP_DECLINE(_) => Self::SMSG_GROUP_DECLINE,
            ServerOpcodeMessage::SMSG_GROUP_UNINVITE(_) => Self::SMSG_GROUP_UNINVITE,
            ServerOpcodeMessage::SMSG_GROUP_SET_LEADER(_) => Self::SMSG_GROUP_SET_LEADER,
            ServerOpcodeMessage::SMSG_GROUP_DESTROYED(_) => Self::SMSG_GROUP_DESTROYED,
            ServerOpcodeMessage::SMSG_GROUP_LIST(_) => Self::SMSG_GROUP_LIST,
            ServerOpcodeMessage::SMSG_PARTY_MEMBER_STATS(_) => Self::SMSG_PARTY_MEMBER_STATS,
            ServerOpcodeMessage::SMSG_PARTY_COMMAND_RESULT(_) => Self::SMSG_PARTY_COMMAND_RESULT,
            ServerOpcodeMessage::SMSG_GUILD_INVITE(_) => Self::SMSG_GUILD_INVITE,
            ServerOpcodeMessage::SMSG_GUILD_INFO(_) => Self::SMSG_GUILD_INFO,
            ServerOpcodeMessage::SMSG_GUILD_ROSTER(_) => Self::SMSG_GUILD_ROSTER,
            ServerOpcodeMessage::SMSG_GUILD_EVENT(_) => Self::SMSG_GUILD_EVENT,
            ServerOpcodeMessage::SMSG_GUILD_COMMAND_RESULT(_) => Self::SMSG_GUILD_COMMAND_RESULT,
            ServerOpcodeMessage::SMSG_MESSAGECHAT(_) => Self::SMSG_MESSAGECHAT,
            ServerOpcodeMessage::SMSG_CHANNEL_NOTIFY(_) => Self::SMSG_CHANNEL_NOTIFY,
            ServerOpcodeMessage::SMSG_CHANNEL_LIST(_) => Self::SMSG_CHANNEL_LIST,
            ServerOpcodeMessage::SMSG_DESTROY_OBJECT(_) => Self::SMSG_DESTROY_OBJECT,
            ServerOpcodeMessage::SMSG_READ_ITEM_OK(_) => Self::SMSG_READ_ITEM_OK,
            ServerOpcodeMessage::SMSG_READ_ITEM_FAILED(_) => Self::SMSG_READ_ITEM_FAILED,
            ServerOpcodeMessage::SMSG_ITEM_COOLDOWN(_) => Self::SMSG_ITEM_COOLDOWN,
            ServerOpcodeMessage::SMSG_GAMEOBJECT_CUSTOM_ANIM(_) => Self::SMSG_GAMEOBJECT_CUSTOM_ANIM,
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(_) => Self::SMSG_MONSTER_MOVE,
            ServerOpcodeMessage::SMSG_MOVE_WATER_WALK(_) => Self::SMSG_MOVE_WATER_WALK,
            ServerOpcodeMessage::SMSG_MOVE_LAND_WALK(_) => Self::SMSG_MOVE_LAND_WALK,
            ServerOpcodeMessage::SMSG_FORCE_RUN_SPEED_CHANGE(_) => Self::SMSG_FORCE_RUN_SPEED_CHANGE,
            ServerOpcodeMessage::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(_) => Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE,
            ServerOpcodeMessage::SMSG_FORCE_SWIM_SPEED_CHANGE(_) => Self::SMSG_FORCE_SWIM_SPEED_CHANGE,
            ServerOpcodeMessage::SMSG_FORCE_MOVE_ROOT(_) => Self::SMSG_FORCE_MOVE_ROOT,
            ServerOpcodeMessage::SMSG_FORCE_MOVE_UNROOT(_) => Self::SMSG_FORCE_MOVE_UNROOT,
            ServerOpcodeMessage::SMSG_MOVE_KNOCK_BACK(_) => Self::SMSG_MOVE_KNOCK_BACK,
            ServerOpcodeMessage::SMSG_MOVE_FEATHER_FALL(_) => Self::SMSG_MOVE_FEATHER_FALL,
            ServerOpcodeMessage::SMSG_MOVE_NORMAL_FALL(_) => Self::SMSG_MOVE_NORMAL_FALL,
            ServerOpcodeMessage::SMSG_MOVE_SET_HOVER(_) => Self::SMSG_MOVE_SET_HOVER,
            ServerOpcodeMessage::SMSG_MOVE_UNSET_HOVER(_) => Self::SMSG_MOVE_UNSET_HOVER,
            ServerOpcodeMessage::SMSG_TRIGGER_CINEMATIC(_) => Self::SMSG_TRIGGER_CINEMATIC,
            ServerOpcodeMessage::SMSG_TUTORIAL_FLAGS(_) => Self::SMSG_TUTORIAL_FLAGS,
            ServerOpcodeMessage::SMSG_EMOTE(_) => Self::SMSG_EMOTE,
            ServerOpcodeMessage::SMSG_TEXT_EMOTE(_) => Self::SMSG_TEXT_EMOTE,
            ServerOpcodeMessage::SMSG_INVENTORY_CHANGE_FAILURE(_) => Self::SMSG_INVENTORY_CHANGE_FAILURE,
            ServerOpcodeMessage::SMSG_OPEN_CONTAINER(_) => Self::SMSG_OPEN_CONTAINER,
            ServerOpcodeMessage::SMSG_INSPECT(_) => Self::SMSG_INSPECT,
            ServerOpcodeMessage::SMSG_TRADE_STATUS(_) => Self::SMSG_TRADE_STATUS,
            ServerOpcodeMessage::SMSG_TRADE_STATUS_EXTENDED(_) => Self::SMSG_TRADE_STATUS_EXTENDED,
            ServerOpcodeMessage::SMSG_INITIALIZE_FACTIONS(_) => Self::SMSG_INITIALIZE_FACTIONS,
            ServerOpcodeMessage::SMSG_SET_FACTION_VISIBLE(_) => Self::SMSG_SET_FACTION_VISIBLE,
            ServerOpcodeMessage::SMSG_SET_FACTION_STANDING(_) => Self::SMSG_SET_FACTION_STANDING,
            ServerOpcodeMessage::SMSG_SET_PROFICIENCY(_) => Self::SMSG_SET_PROFICIENCY,
            ServerOpcodeMessage::SMSG_ACTION_BUTTONS(_) => Self::SMSG_ACTION_BUTTONS,
            ServerOpcodeMessage::SMSG_INITIAL_SPELLS(_) => Self::SMSG_INITIAL_SPELLS,
            ServerOpcodeMessage::SMSG_LEARNED_SPELL(_) => Self::SMSG_LEARNED_SPELL,
            ServerOpcodeMessage::SMSG_SUPERCEDED_SPELL(_) => Self::SMSG_SUPERCEDED_SPELL,
            ServerOpcodeMessage::SMSG_CAST_RESULT(_) => Self::SMSG_CAST_RESULT,
            ServerOpcodeMessage::SMSG_SPELL_START(_) => Self::SMSG_SPELL_START,
            ServerOpcodeMessage::SMSG_SPELL_GO(_) => Self::SMSG_SPELL_GO,
            ServerOpcodeMessage::SMSG_SPELL_FAILURE(_) => Self::SMSG_SPELL_FAILURE,
            ServerOpcodeMessage::SMSG_SPELL_COOLDOWN(_) => Self::SMSG_SPELL_COOLDOWN,
            ServerOpcodeMessage::SMSG_COOLDOWN_EVENT(_) => Self::SMSG_COOLDOWN_EVENT,
            ServerOpcodeMessage::SMSG_UPDATE_AURA_DURATION(_) => Self::SMSG_UPDATE_AURA_DURATION,
            ServerOpcodeMessage::SMSG_PET_CAST_FAILED(_) => Self::SMSG_PET_CAST_FAILED,
            ServerOpcodeMessage::SMSG_AI_REACTION(_) => Self::SMSG_AI_REACTION,
            ServerOpcodeMessage::SMSG_ATTACKSTART(_) => Self::SMSG_ATTACKSTART,
            ServerOpcodeMessage::SMSG_ATTACKSTOP(_) => Self::SMSG_ATTACKSTOP,
            ServerOpcodeMessage::SMSG_ATTACKSWING_NOTINRANGE(_) => Self::SMSG_ATTACKSWING_NOTINRANGE,
            ServerOpcodeMessage::SMSG_ATTACKSWING_BADFACING(_) => Self::SMSG_ATTACKSWING_BADFACING,
            ServerOpcodeMessage::SMSG_ATTACKSWING_NOTSTANDING(_) => Self::SMSG_ATTACKSWING_NOTSTANDING,
            ServerOpcodeMessage::SMSG_ATTACKSWING_DEADTARGET(_) => Self::SMSG_ATTACKSWING_DEADTARGET,
            ServerOpcodeMessage::SMSG_ATTACKSWING_CANT_ATTACK(_) => Self::SMSG_ATTACKSWING_CANT_ATTACK,
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(_) => Self::SMSG_ATTACKERSTATEUPDATE,
            ServerOpcodeMessage::SMSG_CANCEL_COMBAT(_) => Self::SMSG_CANCEL_COMBAT,
            ServerOpcodeMessage::SMSG_SPELLHEALLOG(_) => Self::SMSG_SPELLHEALLOG,
            ServerOpcodeMessage::SMSG_SPELLENERGIZELOG(_) => Self::SMSG_SPELLENERGIZELOG,
            ServerOpcodeMessage::SMSG_BINDPOINTUPDATE(_) => Self::SMSG_BINDPOINTUPDATE,
            ServerOpcodeMessage::SMSG_PLAYERBOUND(_) => Self::SMSG_PLAYERBOUND,
            ServerOpcodeMessage::SMSG_CLIENT_CONTROL_UPDATE(_) => Self::SMSG_CLIENT_CONTROL_UPDATE,
            ServerOpcodeMessage::SMSG_RESURRECT_REQUEST(_) => Self::SMSG_RESURRECT_REQUEST,
            ServerOpcodeMessage::SMSG_LOOT_RESPONSE(_) => Self::SMSG_LOOT_RESPONSE,
            ServerOpcodeMessage::SMSG_LOOT_RELEASE_RESPONSE(_) => Self::SMSG_LOOT_RELEASE_RESPONSE,
            ServerOpcodeMessage::SMSG_LOOT_REMOVED(_) => Self::SMSG_LOOT_REMOVED,
            ServerOpcodeMessage::SMSG_LOOT_MONEY_NOTIFY(_) => Self::SMSG_LOOT_MONEY_NOTIFY,
            ServerOpcodeMessage::SMSG_LOOT_CLEAR_MONEY(_) => Self::SMSG_LOOT_CLEAR_MONEY,
            ServerOpcodeMessage::SMSG_ITEM_PUSH_RESULT(_) => Self::SMSG_ITEM_PUSH_RESULT,
            ServerOpcodeMessage::SMSG_DUEL_REQUESTED(_) => Self::SMSG_DUEL_REQUESTED,
            ServerOpcodeMessage::SMSG_DUEL_OUTOFBOUNDS(_) => Self::SMSG_DUEL_OUTOFBOUNDS,
            ServerOpcodeMessage::SMSG_DUEL_INBOUNDS(_) => Self::SMSG_DUEL_INBOUNDS,
            ServerOpcodeMessage::SMSG_DUEL_COMPLETE(_) => Self::SMSG_DUEL_COMPLETE,
            ServerOpcodeMessage::SMSG_DUEL_WINNER(_) => Self::SMSG_DUEL_WINNER,
            ServerOpcodeMessage::SMSG_MOUNTRESULT(_) => Self::SMSG_MOUNTRESULT,
            ServerOpcodeMessage::SMSG_DISMOUNTRESULT(_) => Self::SMSG_DISMOUNTRESULT,
            ServerOpcodeMessage::SMSG_MOUNTSPECIAL_ANIM(_) => Self::SMSG_MOUNTSPECIAL_ANIM,
            ServerOpcodeMessage::SMSG_PET_TAME_FAILURE(_) => Self::SMSG_PET_TAME_FAILURE,
            ServerOpcodeMessage::SMSG_PET_NAME_INVALID(_) => Self::SMSG_PET_NAME_INVALID,
            ServerOpcodeMessage::SMSG_PET_SPELLS(_) => Self::SMSG_PET_SPELLS,
            ServerOpcodeMessage::SMSG_PET_MODE(_) => Self::SMSG_PET_MODE,
            ServerOpcodeMessage::SMSG_GOSSIP_MESSAGE(_) => Self::SMSG_GOSSIP_MESSAGE,
            ServerOpcodeMessage::SMSG_GOSSIP_COMPLETE(_) => Self::SMSG_GOSSIP_COMPLETE,
            ServerOpcodeMessage::SMSG_NPC_TEXT_UPDATE(_) => Self::SMSG_NPC_TEXT_UPDATE,
            ServerOpcodeMessage::SMSG_QUESTGIVER_STATUS(_) => Self::SMSG_QUESTGIVER_STATUS,
            ServerOpcodeMessage::SMSG_QUESTGIVER_QUEST_LIST(_) => Self::SMSG_QUESTGIVER_QUEST_LIST,
            ServerOpcodeMessage::SMSG_QUESTGIVER_QUEST_DETAILS(_) => Self::SMSG_QUESTGIVER_QUEST_DETAILS,
            ServerOpcodeMessage::SMSG_QUESTGIVER_REQUEST_ITEMS(_) => Self::SMSG_QUESTGIVER_REQUEST_ITEMS,
            ServerOpcodeMessage::SMSG_QUESTGIVER_OFFER_REWARD(_) => Self::SMSG_QUESTGIVER_OFFER_REWARD,
            ServerOpcodeMessage::SMSG_QUESTGIVER_QUEST_INVALID(_) => Self::SMSG_QUESTGIVER_QUEST_INVALID,
            ServerOpcodeMessage::SMSG_QUESTGIVER_QUEST_COMPLETE(_) => Self::SMSG_QUESTGIVER_QUEST_COMPLETE,
            ServerOpcodeMessage::SMSG_QUESTGIVER_QUEST_FAILED(_) => Self::SMSG_QUESTGIVER_QUEST_FAILED,
            ServerOpcodeMessage::SMSG_QUESTLOG_FULL(_) => Self::SMSG_QUESTLOG_FULL,
            ServerOpcodeMessage::SMSG_QUESTUPDATE_FAILED(_) => Self::SMSG_QUESTUPDATE_FAILED,
            ServerOpcodeMessage::SMSG_QUESTUPDATE_FAILEDTIMER(_) => Self::SMSG_QUESTUPDATE_FAILEDTIMER,
            ServerOpcodeMessage::SMSG_QUESTUPDATE_COMPLETE(_) => Self::SMSG_QUESTUPDATE_COMPLETE,
            ServerOpcodeMessage::SMSG_QUESTUPDATE_ADD_KILL(_) => Self::SMSG_QUESTUPDATE_ADD_KILL,
            ServerOpcodeMessage::SMSG_QUESTUPDATE_ADD_ITEM(_) => Self::SMSG_QUESTUPDATE_ADD_ITEM,
            ServerOpcodeMessage::SMSG_QUEST_CONFIRM_ACCEPT(_) => Self::SMSG_QUEST_CONFIRM_ACCEPT,
            ServerOpcodeMessage::SMSG_LIST_INVENTORY(_) => Self::SMSG_LIST_INVENTORY,
            ServerOpcodeMessage::SMSG_SELL_ITEM(_) => Self::SMSG_SELL_ITEM,
            ServerOpcodeMessage::SMSG_BUY_ITEM(_) => Self::SMSG_BUY_ITEM,
            ServerOpcodeMessage::SMSG_BUY_FAILED(_) => Self::SMSG_BUY_FAILED,
            ServerOpcodeMessage::SMSG_SHOWTAXINODES(_) => Self::SMSG_SHOWTAXINODES,
            ServerOpcodeMessage::SMSG_TAXINODE_STATUS(_) => Self::SMSG_TAXINODE_STATUS,
            ServerOpcodeMessage::SMSG_ACTIVATETAXIREPLY(_) => Self::SMSG_ACTIVATETAXIREPLY,
            ServerOpcodeMessage::SMSG_NEW_TAXI_PATH(_) => Self::SMSG_NEW_TAXI_PATH,
            ServerOpcodeMessage::SMSG_TRAINER_LIST(_) => Self::SMSG_TRAINER_LIST,
            ServerOpcodeMessage::SMSG_TRAINER_BUY_SUCCEEDED(_) => Self::SMSG_TRAINER_BUY_SUCCEEDED,
            ServerOpcodeMessage::SMSG_TRAINER_BUY_FAILED(_) => Self::SMSG_TRAINER_BUY_FAILED,
            ServerOpcodeMessage::SMSG_SHOW_BANK(_) => Self::SMSG_SHOW_BANK,
            ServerOpcodeMessage::SMSG_BUY_BANK_SLOT_RESULT(_) => Self::SMSG_BUY_BANK_SLOT_RESULT,
            ServerOpcodeMessage::SMSG_PETITION_SHOWLIST(_) => Self::SMSG_PETITION_SHOWLIST,
            ServerOpcodeMessage::SMSG_PETITION_SHOW_SIGNATURES(_) => Self::SMSG_PETITION_SHOW_SIGNATURES,
            ServerOpcodeMessage::SMSG_PETITION_SIGN_RESULTS(_) => Self::SMSG_PETITION_SIGN_RESULTS,
            ServerOpcodeMessage::SMSG_TURN_IN_PETITION_RESULTS(_) => Self::SMSG_TURN_IN_PETITION_RESULTS,
            ServerOpcodeMessage::SMSG_PETITION_QUERY_RESPONSE(_) => Self::SMSG_PETITION_QUERY_RESPONSE,
            ServerOpcodeMessage::SMSG_FISH_NOT_HOOKED(_) => Self::SMSG_FISH_NOT_HOOKED,
            ServerOpcodeMessage::SMSG_FISH_ESCAPED(_) => Self::SMSG_FISH_ESCAPED,
            ServerOpcodeMessage::SMSG_NOTIFICATION(_) => Self::SMSG_NOTIFICATION,
            ServerOpcodeMessage::SMSG_PLAYED_TIME(_) => Self::SMSG_PLAYED_TIME,
            ServerOpcodeMessage::SMSG_QUERY_TIME_RESPONSE(_) => Self::SMSG_QUERY_TIME_RESPONSE,
            ServerOpcodeMessage::SMSG_LOG_XPGAIN(_) => Self::SMSG_LOG_XPGAIN,
            ServerOpcodeMessage::SMSG_LEVELUP_INFO(_) => Self::SMSG_LEVELUP_INFO,
            ServerOpcodeMessage::MSG_MINIMAP_PING(_) => Self::MSG_MINIMAP_PING,
            ServerOpcodeMessage::SMSG_RESISTLOG(_) => Self::SMSG_RESISTLOG,
            ServerOpcodeMessage::SMSG_ENCHANTMENTLOG(_) => Self::SMSG_ENCHANTMENTLOG,
            ServerOpcodeMessage::SMSG_START_MIRROR_TIMER(_) => Self::SMSG_START_MIRROR_TIMER,
            ServerOpcodeMessage::SMSG_PAUSE_MIRROR_TIMER(_) => Self::SMSG_PAUSE_MIRROR_TIMER,
            ServerOpcodeMessage::SMSG_STOP_MIRROR_TIMER(_) => Self::SMSG_STOP_MIRROR_TIMER,
            ServerOpcodeMessage::SMSG_PONG(_) => Self::SMSG_PONG,
            ServerOpcodeMessage::SMSG_CLEAR_COOLDOWN(_) => Self::SMSG_CLEAR_COOLDOWN,
            ServerOpcodeMessage::SMSG_GAMEOBJECT_PAGETEXT(_) => Self::SMSG_GAMEOBJECT_PAGETEXT,
            ServerOpcodeMessage::SMSG_SPELL_DELAYED(_) => Self::SMSG_SPELL_DELAYED,
            ServerOpcodeMessage::SMSG_ITEM_TIME_UPDATE(_) => Self::SMSG_ITEM_TIME_UPDATE,
            ServerOpcodeMessage::SMSG_ITEM_ENCHANT_TIME_UPDATE(_) => Self::SMSG_ITEM_ENCHANT_TIME_UPDATE,
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(_) => Self::SMSG_AUTH_CHALLENGE,
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(_) => Self::SMSG_AUTH_RESPONSE,
            ServerOpcodeMessage::MSG_SAVE_GUILD_EMBLEM(_) => Self::MSG_SAVE_GUILD_EMBLEM,
            ServerOpcodeMessage::SMSG_PLAY_SPELL_VISUAL(_) => Self::SMSG_PLAY_SPELL_VISUAL,
            ServerOpcodeMessage::SMSG_PARTYKILLLOG(_) => Self::SMSG_PARTYKILLLOG,
            ServerOpcodeMessage::SMSG_PLAY_SPELL_IMPACT(_) => Self::SMSG_PLAY_SPELL_IMPACT,
            ServerOpcodeMessage::SMSG_EXPLORATION_EXPERIENCE(_) => Self::SMSG_EXPLORATION_EXPERIENCE,
            ServerOpcodeMessage::MSG_RANDOM_ROLL(_) => Self::MSG_RANDOM_ROLL,
            ServerOpcodeMessage::SMSG_ENVIRONMENTALDAMAGELOG(_) => Self::SMSG_ENVIRONMENTALDAMAGELOG,
            ServerOpcodeMessage::MSG_LOOKING_FOR_GROUP(_) => Self::MSG_LOOKING_FOR_GROUP,
            ServerOpcodeMessage::SMSG_REMOVED_SPELL(_) => Self::SMSG_REMOVED_SPELL,
            ServerOpcodeMessage::SMSG_GMTICKET_CREATE(_) => Self::SMSG_GMTICKET_CREATE,
            ServerOpcodeMessage::SMSG_GMTICKET_UPDATETEXT(_) => Self::SMSG_GMTICKET_UPDATETEXT,
            ServerOpcodeMessage::SMSG_ACCOUNT_DATA_TIMES(_) => Self::SMSG_ACCOUNT_DATA_TIMES,
            ServerOpcodeMessage::SMSG_GMTICKET_GETTICKET(_) => Self::SMSG_GMTICKET_GETTICKET,
            ServerOpcodeMessage::SMSG_GAMEOBJECT_SPAWN_ANIM(_) => Self::SMSG_GAMEOBJECT_SPAWN_ANIM,
            ServerOpcodeMessage::SMSG_GAMEOBJECT_DESPAWN_ANIM(_) => Self::SMSG_GAMEOBJECT_DESPAWN_ANIM,
            ServerOpcodeMessage::MSG_CORPSE_QUERY(_) => Self::MSG_CORPSE_QUERY,
            ServerOpcodeMessage::SMSG_GMTICKET_DELETETICKET(_) => Self::SMSG_GMTICKET_DELETETICKET,
            ServerOpcodeMessage::SMSG_CHAT_WRONG_FACTION(_) => Self::SMSG_CHAT_WRONG_FACTION,
            ServerOpcodeMessage::SMSG_GMTICKET_SYSTEMSTATUS(_) => Self::SMSG_GMTICKET_SYSTEMSTATUS,
            ServerOpcodeMessage::SMSG_SET_REST_START(_) => Self::SMSG_SET_REST_START,
            ServerOpcodeMessage::SMSG_SPIRIT_HEALER_CONFIRM(_) => Self::SMSG_SPIRIT_HEALER_CONFIRM,
            ServerOpcodeMessage::SMSG_GOSSIP_POI(_) => Self::SMSG_GOSSIP_POI,
            ServerOpcodeMessage::SMSG_LOGIN_VERIFY_WORLD(_) => Self::SMSG_LOGIN_VERIFY_WORLD,
            ServerOpcodeMessage::SMSG_MAIL_LIST_RESULT(_) => Self::SMSG_MAIL_LIST_RESULT,
            ServerOpcodeMessage::SMSG_BATTLEFIELD_LIST(_) => Self::SMSG_BATTLEFIELD_LIST,
            ServerOpcodeMessage::SMSG_ITEM_TEXT_QUERY_RESPONSE(_) => Self::SMSG_ITEM_TEXT_QUERY_RESPONSE,
            ServerOpcodeMessage::SMSG_SPELLLOGMISS(_) => Self::SMSG_SPELLLOGMISS,
            ServerOpcodeMessage::SMSG_SPELLLOGEXECUTE(_) => Self::SMSG_SPELLLOGEXECUTE,
            ServerOpcodeMessage::SMSG_PERIODICAURALOG(_) => Self::SMSG_PERIODICAURALOG,
            ServerOpcodeMessage::SMSG_SPELLDAMAGESHIELD(_) => Self::SMSG_SPELLDAMAGESHIELD,
            ServerOpcodeMessage::SMSG_SPELLNONMELEEDAMAGELOG(_) => Self::SMSG_SPELLNONMELEEDAMAGELOG,
            ServerOpcodeMessage::SMSG_ZONE_UNDER_ATTACK(_) => Self::SMSG_ZONE_UNDER_ATTACK,
            ServerOpcodeMessage::MSG_AUCTION_HELLO(_) => Self::MSG_AUCTION_HELLO,
            ServerOpcodeMessage::SMSG_AUCTION_LIST_RESULT(_) => Self::SMSG_AUCTION_LIST_RESULT,
            ServerOpcodeMessage::SMSG_AUCTION_OWNER_LIST_RESULT(_) => Self::SMSG_AUCTION_OWNER_LIST_RESULT,
            ServerOpcodeMessage::SMSG_AUCTION_BIDDER_NOTIFICATION(_) => Self::SMSG_AUCTION_BIDDER_NOTIFICATION,
            ServerOpcodeMessage::SMSG_AUCTION_OWNER_NOTIFICATION(_) => Self::SMSG_AUCTION_OWNER_NOTIFICATION,
            ServerOpcodeMessage::SMSG_PROCRESIST(_) => Self::SMSG_PROCRESIST,
            ServerOpcodeMessage::SMSG_DISPEL_FAILED(_) => Self::SMSG_DISPEL_FAILED,
            ServerOpcodeMessage::SMSG_SPELLORDAMAGE_IMMUNE(_) => Self::SMSG_SPELLORDAMAGE_IMMUNE,
            ServerOpcodeMessage::SMSG_AUCTION_BIDDER_LIST_RESULT(_) => Self::SMSG_AUCTION_BIDDER_LIST_RESULT,
            ServerOpcodeMessage::SMSG_SET_FLAT_SPELL_MODIFIER(_) => Self::SMSG_SET_FLAT_SPELL_MODIFIER,
            ServerOpcodeMessage::SMSG_SET_PCT_SPELL_MODIFIER(_) => Self::SMSG_SET_PCT_SPELL_MODIFIER,
            ServerOpcodeMessage::SMSG_CORPSE_RECLAIM_DELAY(_) => Self::SMSG_CORPSE_RECLAIM_DELAY,
            ServerOpcodeMessage::MSG_LIST_STABLED_PETS(_) => Self::MSG_LIST_STABLED_PETS,
            ServerOpcodeMessage::SMSG_STABLE_RESULT(_) => Self::SMSG_STABLE_RESULT,
            ServerOpcodeMessage::SMSG_PLAY_MUSIC(_) => Self::SMSG_PLAY_MUSIC,
            ServerOpcodeMessage::SMSG_PLAY_OBJECT_SOUND(_) => Self::SMSG_PLAY_OBJECT_SOUND,
            ServerOpcodeMessage::SMSG_SPELLDISPELLOG(_) => Self::SMSG_SPELLDISPELLOG,
            ServerOpcodeMessage::MSG_QUERY_NEXT_MAIL_TIME(_) => Self::MSG_QUERY_NEXT_MAIL_TIME,
            ServerOpcodeMessage::SMSG_RECEIVED_MAIL(_) => Self::SMSG_RECEIVED_MAIL,
            ServerOpcodeMessage::SMSG_RAID_GROUP_ONLY(_) => Self::SMSG_RAID_GROUP_ONLY,
            ServerOpcodeMessage::SMSG_PVP_CREDIT(_) => Self::SMSG_PVP_CREDIT,
            ServerOpcodeMessage::SMSG_AUCTION_REMOVED_NOTIFICATION(_) => Self::SMSG_AUCTION_REMOVED_NOTIFICATION,
            ServerOpcodeMessage::SMSG_SERVER_MESSAGE(_) => Self::SMSG_SERVER_MESSAGE,
            ServerOpcodeMessage::SMSG_MEETINGSTONE_SETQUEUE(_) => Self::SMSG_MEETINGSTONE_SETQUEUE,
            ServerOpcodeMessage::SMSG_MEETINGSTONE_COMPLETE(_) => Self::SMSG_MEETINGSTONE_COMPLETE,
            ServerOpcodeMessage::SMSG_MEETINGSTONE_IN_PROGRESS(_) => Self::SMSG_MEETINGSTONE_IN_PROGRESS,
            ServerOpcodeMessage::SMSG_MEETINGSTONE_MEMBER_ADDED(_) => Self::SMSG_MEETINGSTONE_MEMBER_ADDED,
            ServerOpcodeMessage::SMSG_CANCEL_AUTO_REPEAT(_) => Self::SMSG_CANCEL_AUTO_REPEAT,
            ServerOpcodeMessage::SMSG_STANDSTATE_UPDATE(_) => Self::SMSG_STANDSTATE_UPDATE,
            ServerOpcodeMessage::SMSG_LOOT_ALL_PASSED(_) => Self::SMSG_LOOT_ALL_PASSED,
            ServerOpcodeMessage::SMSG_LOOT_ROLL_WON(_) => Self::SMSG_LOOT_ROLL_WON,
            ServerOpcodeMessage::SMSG_LOOT_START_ROLL(_) => Self::SMSG_LOOT_START_ROLL,
            ServerOpcodeMessage::SMSG_LOOT_ROLL(_) => Self::SMSG_LOOT_ROLL,
            ServerOpcodeMessage::SMSG_LOOT_MASTER_LIST(_) => Self::SMSG_LOOT_MASTER_LIST,
            ServerOpcodeMessage::SMSG_SET_FORCED_REACTIONS(_) => Self::SMSG_SET_FORCED_REACTIONS,
            ServerOpcodeMessage::SMSG_SPELL_FAILED_OTHER(_) => Self::SMSG_SPELL_FAILED_OTHER,
            ServerOpcodeMessage::SMSG_GAMEOBJECT_RESET_STATE(_) => Self::SMSG_GAMEOBJECT_RESET_STATE,
            ServerOpcodeMessage::SMSG_CHAT_PLAYER_NOT_FOUND(_) => Self::SMSG_CHAT_PLAYER_NOT_FOUND,
            ServerOpcodeMessage::MSG_TALENT_WIPE_CONFIRM(_) => Self::MSG_TALENT_WIPE_CONFIRM,
            ServerOpcodeMessage::SMSG_SUMMON_REQUEST(_) => Self::SMSG_SUMMON_REQUEST,
            ServerOpcodeMessage::SMSG_MONSTER_MOVE_TRANSPORT(_) => Self::SMSG_MONSTER_MOVE_TRANSPORT,
            ServerOpcodeMessage::SMSG_PET_BROKEN(_) => Self::SMSG_PET_BROKEN,
            ServerOpcodeMessage::SMSG_FEIGN_DEATH_RESISTED(_) => Self::SMSG_FEIGN_DEATH_RESISTED,
            ServerOpcodeMessage::SMSG_DUEL_COUNTDOWN(_) => Self::SMSG_DUEL_COUNTDOWN,
            ServerOpcodeMessage::SMSG_AREA_TRIGGER_MESSAGE(_) => Self::SMSG_AREA_TRIGGER_MESSAGE,
            ServerOpcodeMessage::SMSG_MEETINGSTONE_JOINFAILED(_) => Self::SMSG_MEETINGSTONE_JOINFAILED,
            ServerOpcodeMessage::SMSG_PLAYER_SKINNED(_) => Self::SMSG_PLAYER_SKINNED,
            ServerOpcodeMessage::SMSG_DURABILITY_DAMAGE_DEATH(_) => Self::SMSG_DURABILITY_DAMAGE_DEATH,
            ServerOpcodeMessage::SMSG_INIT_WORLD_STATES(_) => Self::SMSG_INIT_WORLD_STATES,
            ServerOpcodeMessage::SMSG_UPDATE_WORLD_STATE(_) => Self::SMSG_UPDATE_WORLD_STATE,
            ServerOpcodeMessage::SMSG_ITEM_NAME_QUERY_RESPONSE(_) => Self::SMSG_ITEM_NAME_QUERY_RESPONSE,
            ServerOpcodeMessage::SMSG_PET_ACTION_FEEDBACK(_) => Self::SMSG_PET_ACTION_FEEDBACK,
            ServerOpcodeMessage::SMSG_CHAR_RENAME(_) => Self::SMSG_CHAR_RENAME,
            ServerOpcodeMessage::SMSG_INSTANCE_SAVE_CREATED(_) => Self::SMSG_INSTANCE_SAVE_CREATED,
            ServerOpcodeMessage::SMSG_RAID_INSTANCE_INFO(_) => Self::SMSG_RAID_INSTANCE_INFO,
            ServerOpcodeMessage::SMSG_PLAY_SOUND(_) => Self::SMSG_PLAY_SOUND,
            ServerOpcodeMessage::SMSG_BATTLEFIELD_STATUS(_) => Self::SMSG_BATTLEFIELD_STATUS,
            ServerOpcodeMessage::MSG_INSPECT_HONOR_STATS(_) => Self::MSG_INSPECT_HONOR_STATS,
            ServerOpcodeMessage::SMSG_FORCE_WALK_SPEED_CHANGE(_) => Self::SMSG_FORCE_WALK_SPEED_CHANGE,
            ServerOpcodeMessage::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(_) => Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE,
            ServerOpcodeMessage::SMSG_FORCE_TURN_RATE_CHANGE(_) => Self::SMSG_FORCE_TURN_RATE_CHANGE,
            ServerOpcodeMessage::MSG_PVP_LOG_DATA(_) => Self::MSG_PVP_LOG_DATA,
            ServerOpcodeMessage::SMSG_AREA_SPIRIT_HEALER_TIME(_) => Self::SMSG_AREA_SPIRIT_HEALER_TIME,
            ServerOpcodeMessage::SMSG_GROUP_JOINED_BATTLEGROUND(_) => Self::SMSG_GROUP_JOINED_BATTLEGROUND,
            ServerOpcodeMessage::MSG_BATTLEGROUND_PLAYER_POSITIONS(_) => Self::MSG_BATTLEGROUND_PLAYER_POSITIONS,
            ServerOpcodeMessage::SMSG_BINDER_CONFIRM(_) => Self::SMSG_BINDER_CONFIRM,
            ServerOpcodeMessage::SMSG_BATTLEGROUND_PLAYER_JOINED(_) => Self::SMSG_BATTLEGROUND_PLAYER_JOINED,
            ServerOpcodeMessage::SMSG_BATTLEGROUND_PLAYER_LEFT(_) => Self::SMSG_BATTLEGROUND_PLAYER_LEFT,
            ServerOpcodeMessage::SMSG_PET_UNLEARN_CONFIRM(_) => Self::SMSG_PET_UNLEARN_CONFIRM,
            ServerOpcodeMessage::SMSG_PARTY_MEMBER_STATS_FULL(_) => Self::SMSG_PARTY_MEMBER_STATS_FULL,
            ServerOpcodeMessage::SMSG_WEATHER(_) => Self::SMSG_WEATHER,
            ServerOpcodeMessage::SMSG_RAID_INSTANCE_MESSAGE(_) => Self::SMSG_RAID_INSTANCE_MESSAGE,
            ServerOpcodeMessage::SMSG_CHAT_RESTRICTED(_) => Self::SMSG_CHAT_RESTRICTED,
            ServerOpcodeMessage::SMSG_SPLINE_SET_RUN_SPEED(_) => Self::SMSG_SPLINE_SET_RUN_SPEED,
            ServerOpcodeMessage::SMSG_SPLINE_SET_RUN_BACK_SPEED(_) => Self::SMSG_SPLINE_SET_RUN_BACK_SPEED,
            ServerOpcodeMessage::SMSG_SPLINE_SET_SWIM_SPEED(_) => Self::SMSG_SPLINE_SET_SWIM_SPEED,
            ServerOpcodeMessage::SMSG_SPLINE_SET_WALK_SPEED(_) => Self::SMSG_SPLINE_SET_WALK_SPEED,
            ServerOpcodeMessage::SMSG_SPLINE_SET_SWIM_BACK_SPEED(_) => Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED,
            ServerOpcodeMessage::SMSG_SPLINE_SET_TURN_RATE(_) => Self::SMSG_SPLINE_SET_TURN_RATE,
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_UNROOT(_) => Self::SMSG_SPLINE_MOVE_UNROOT,
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_FEATHER_FALL(_) => Self::SMSG_SPLINE_MOVE_FEATHER_FALL,
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_NORMAL_FALL(_) => Self::SMSG_SPLINE_MOVE_NORMAL_FALL,
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_SET_HOVER(_) => Self::SMSG_SPLINE_MOVE_SET_HOVER,
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_UNSET_HOVER(_) => Self::SMSG_SPLINE_MOVE_UNSET_HOVER,
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_WATER_WALK(_) => Self::SMSG_SPLINE_MOVE_WATER_WALK,
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_LAND_WALK(_) => Self::SMSG_SPLINE_MOVE_LAND_WALK,
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_START_SWIM(_) => Self::SMSG_SPLINE_MOVE_START_SWIM,
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_STOP_SWIM(_) => Self::SMSG_SPLINE_MOVE_STOP_SWIM,
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_SET_RUN_MODE(_) => Self::SMSG_SPLINE_MOVE_SET_RUN_MODE,
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_SET_WALK_MODE(_) => Self::SMSG_SPLINE_MOVE_SET_WALK_MODE,
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_ROOT(_) => Self::SMSG_SPLINE_MOVE_ROOT,
            ServerOpcodeMessage::SMSG_INVALIDATE_PLAYER(_) => Self::SMSG_INVALIDATE_PLAYER,
            ServerOpcodeMessage::SMSG_INSTANCE_RESET(_) => Self::SMSG_INSTANCE_RESET,
            ServerOpcodeMessage::SMSG_INSTANCE_RESET_FAILED(_) => Self::SMSG_INSTANCE_RESET_FAILED,
            ServerOpcodeMessage::SMSG_UPDATE_LAST_INSTANCE(_) => Self::SMSG_UPDATE_LAST_INSTANCE,
            ServerOpcodeMessage::MSG_RAID_TARGET_UPDATE(_) => Self::MSG_RAID_TARGET_UPDATE,
            ServerOpcodeMessage::MSG_RAID_READY_CHECK(_) => Self::MSG_RAID_READY_CHECK,
            ServerOpcodeMessage::SMSG_PET_ACTION_SOUND(_) => Self::SMSG_PET_ACTION_SOUND,
            ServerOpcodeMessage::SMSG_PET_DISMISS_SOUND(_) => Self::SMSG_PET_DISMISS_SOUND,
            ServerOpcodeMessage::SMSG_GM_TICKET_STATUS_UPDATE(_) => Self::SMSG_GM_TICKET_STATUS_UPDATE,
            ServerOpcodeMessage::SMSG_UPDATE_INSTANCE_OWNERSHIP(_) => Self::SMSG_UPDATE_INSTANCE_OWNERSHIP,
            ServerOpcodeMessage::SMSG_SPELLINSTAKILLLOG(_) => Self::SMSG_SPELLINSTAKILLLOG,
            ServerOpcodeMessage::SMSG_SPELL_UPDATE_CHAIN_TARGETS(_) => Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS,
            ServerOpcodeMessage::SMSG_EXPECTED_SPAM_RECORDS(_) => Self::SMSG_EXPECTED_SPAM_RECORDS,
            ServerOpcodeMessage::SMSG_DEFENSE_MESSAGE(_) => Self::SMSG_DEFENSE_MESSAGE,
        }
    }
}

#[derive(Debug)]
pub enum ServerOpcodeError {
    Io(std::io::Error),
    InvalidOpcode(u16),
}

impl std::error::Error for ServerOpcodeError {
}

impl std::fmt::Display for ServerOpcodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode for Server: '{}'", i)),
        }
    }
}

impl From<std::io::Error> for ServerOpcodeError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug)]
pub enum ServerOpcodeMessage {
    MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD),
    MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD),
    MSG_MOVE_STOP(MSG_MOVE_STOP),
    MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT),
    MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT),
    MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE),
    MSG_MOVE_JUMP(MSG_MOVE_JUMP),
    MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT),
    MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT),
    MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN),
    MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP),
    MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN),
    MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH),
    MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE),
    MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE),
    MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK),
    MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND),
    MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM),
    MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM),
    MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING),
    MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH),
    MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK),
    MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT),
    MSG_PETITION_DECLINE(MSG_PETITION_DECLINE),
    MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE),
    MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT),
    MSG_PETITION_RENAME(MSG_PETITION_RENAME),
    SMSG_CHAR_CREATE(SMSG_CHAR_CREATE),
    SMSG_CHAR_ENUM(SMSG_CHAR_ENUM),
    SMSG_CHAR_DELETE(SMSG_CHAR_DELETE),
    SMSG_NEW_WORLD(SMSG_NEW_WORLD),
    SMSG_TRANSFER_PENDING(SMSG_TRANSFER_PENDING),
    SMSG_TRANSFER_ABORTED(SMSG_TRANSFER_ABORTED),
    SMSG_CHARACTER_LOGIN_FAILED(SMSG_CHARACTER_LOGIN_FAILED),
    SMSG_LOGIN_SETTIMESPEED(SMSG_LOGIN_SETTIMESPEED),
    SMSG_LOGOUT_RESPONSE(SMSG_LOGOUT_RESPONSE),
    SMSG_LOGOUT_COMPLETE(SMSG_LOGOUT_COMPLETE),
    SMSG_LOGOUT_CANCEL_ACK(SMSG_LOGOUT_CANCEL_ACK),
    SMSG_NAME_QUERY_RESPONSE(SMSG_NAME_QUERY_RESPONSE),
    SMSG_PET_NAME_QUERY_RESPONSE(SMSG_PET_NAME_QUERY_RESPONSE),
    SMSG_GUILD_QUERY_RESPONSE(SMSG_GUILD_QUERY_RESPONSE),
    SMSG_ITEM_QUERY_SINGLE_RESPONSE(SMSG_ITEM_QUERY_SINGLE_RESPONSE),
    SMSG_PAGE_TEXT_QUERY_RESPONSE(SMSG_PAGE_TEXT_QUERY_RESPONSE),
    SMSG_QUEST_QUERY_RESPONSE(SMSG_QUEST_QUERY_RESPONSE),
    SMSG_GAMEOBJECT_QUERY_RESPONSE(SMSG_GAMEOBJECT_QUERY_RESPONSE),
    SMSG_CREATURE_QUERY_RESPONSE(SMSG_CREATURE_QUERY_RESPONSE),
    SMSG_WHO(SMSG_WHO),
    SMSG_WHOIS(SMSG_WHOIS),
    SMSG_FRIEND_LIST(SMSG_FRIEND_LIST),
    SMSG_FRIEND_STATUS(SMSG_FRIEND_STATUS),
    SMSG_IGNORE_LIST(SMSG_IGNORE_LIST),
    SMSG_GROUP_INVITE(SMSG_GROUP_INVITE),
    SMSG_GROUP_DECLINE(SMSG_GROUP_DECLINE),
    SMSG_GROUP_UNINVITE(SMSG_GROUP_UNINVITE),
    SMSG_GROUP_SET_LEADER(SMSG_GROUP_SET_LEADER),
    SMSG_GROUP_DESTROYED(SMSG_GROUP_DESTROYED),
    SMSG_GROUP_LIST(SMSG_GROUP_LIST),
    SMSG_PARTY_MEMBER_STATS(SMSG_PARTY_MEMBER_STATS),
    SMSG_PARTY_COMMAND_RESULT(SMSG_PARTY_COMMAND_RESULT),
    SMSG_GUILD_INVITE(SMSG_GUILD_INVITE),
    SMSG_GUILD_INFO(SMSG_GUILD_INFO),
    SMSG_GUILD_ROSTER(SMSG_GUILD_ROSTER),
    SMSG_GUILD_EVENT(SMSG_GUILD_EVENT),
    SMSG_GUILD_COMMAND_RESULT(SMSG_GUILD_COMMAND_RESULT),
    SMSG_MESSAGECHAT(SMSG_MESSAGECHAT),
    SMSG_CHANNEL_NOTIFY(SMSG_CHANNEL_NOTIFY),
    SMSG_CHANNEL_LIST(SMSG_CHANNEL_LIST),
    SMSG_DESTROY_OBJECT(SMSG_DESTROY_OBJECT),
    SMSG_READ_ITEM_OK(SMSG_READ_ITEM_OK),
    SMSG_READ_ITEM_FAILED(SMSG_READ_ITEM_FAILED),
    SMSG_ITEM_COOLDOWN(SMSG_ITEM_COOLDOWN),
    SMSG_GAMEOBJECT_CUSTOM_ANIM(SMSG_GAMEOBJECT_CUSTOM_ANIM),
    SMSG_MONSTER_MOVE(SMSG_MONSTER_MOVE),
    SMSG_MOVE_WATER_WALK(SMSG_MOVE_WATER_WALK),
    SMSG_MOVE_LAND_WALK(SMSG_MOVE_LAND_WALK),
    SMSG_FORCE_RUN_SPEED_CHANGE(SMSG_FORCE_RUN_SPEED_CHANGE),
    SMSG_FORCE_RUN_BACK_SPEED_CHANGE(SMSG_FORCE_RUN_BACK_SPEED_CHANGE),
    SMSG_FORCE_SWIM_SPEED_CHANGE(SMSG_FORCE_SWIM_SPEED_CHANGE),
    SMSG_FORCE_MOVE_ROOT(SMSG_FORCE_MOVE_ROOT),
    SMSG_FORCE_MOVE_UNROOT(SMSG_FORCE_MOVE_UNROOT),
    SMSG_MOVE_KNOCK_BACK(SMSG_MOVE_KNOCK_BACK),
    SMSG_MOVE_FEATHER_FALL(SMSG_MOVE_FEATHER_FALL),
    SMSG_MOVE_NORMAL_FALL(SMSG_MOVE_NORMAL_FALL),
    SMSG_MOVE_SET_HOVER(SMSG_MOVE_SET_HOVER),
    SMSG_MOVE_UNSET_HOVER(SMSG_MOVE_UNSET_HOVER),
    SMSG_TRIGGER_CINEMATIC(SMSG_TRIGGER_CINEMATIC),
    SMSG_TUTORIAL_FLAGS(SMSG_TUTORIAL_FLAGS),
    SMSG_EMOTE(SMSG_EMOTE),
    SMSG_TEXT_EMOTE(SMSG_TEXT_EMOTE),
    SMSG_INVENTORY_CHANGE_FAILURE(SMSG_INVENTORY_CHANGE_FAILURE),
    SMSG_OPEN_CONTAINER(SMSG_OPEN_CONTAINER),
    SMSG_INSPECT(SMSG_INSPECT),
    SMSG_TRADE_STATUS(SMSG_TRADE_STATUS),
    SMSG_TRADE_STATUS_EXTENDED(SMSG_TRADE_STATUS_EXTENDED),
    SMSG_INITIALIZE_FACTIONS(SMSG_INITIALIZE_FACTIONS),
    SMSG_SET_FACTION_VISIBLE(SMSG_SET_FACTION_VISIBLE),
    SMSG_SET_FACTION_STANDING(SMSG_SET_FACTION_STANDING),
    SMSG_SET_PROFICIENCY(SMSG_SET_PROFICIENCY),
    SMSG_ACTION_BUTTONS(SMSG_ACTION_BUTTONS),
    SMSG_INITIAL_SPELLS(SMSG_INITIAL_SPELLS),
    SMSG_LEARNED_SPELL(SMSG_LEARNED_SPELL),
    SMSG_SUPERCEDED_SPELL(SMSG_SUPERCEDED_SPELL),
    SMSG_CAST_RESULT(SMSG_CAST_RESULT),
    SMSG_SPELL_START(SMSG_SPELL_START),
    SMSG_SPELL_GO(SMSG_SPELL_GO),
    SMSG_SPELL_FAILURE(SMSG_SPELL_FAILURE),
    SMSG_SPELL_COOLDOWN(SMSG_SPELL_COOLDOWN),
    SMSG_COOLDOWN_EVENT(SMSG_COOLDOWN_EVENT),
    SMSG_UPDATE_AURA_DURATION(SMSG_UPDATE_AURA_DURATION),
    SMSG_PET_CAST_FAILED(SMSG_PET_CAST_FAILED),
    SMSG_AI_REACTION(SMSG_AI_REACTION),
    SMSG_ATTACKSTART(SMSG_ATTACKSTART),
    SMSG_ATTACKSTOP(SMSG_ATTACKSTOP),
    SMSG_ATTACKSWING_NOTINRANGE(SMSG_ATTACKSWING_NOTINRANGE),
    SMSG_ATTACKSWING_BADFACING(SMSG_ATTACKSWING_BADFACING),
    SMSG_ATTACKSWING_NOTSTANDING(SMSG_ATTACKSWING_NOTSTANDING),
    SMSG_ATTACKSWING_DEADTARGET(SMSG_ATTACKSWING_DEADTARGET),
    SMSG_ATTACKSWING_CANT_ATTACK(SMSG_ATTACKSWING_CANT_ATTACK),
    SMSG_ATTACKERSTATEUPDATE(SMSG_ATTACKERSTATEUPDATE),
    SMSG_CANCEL_COMBAT(SMSG_CANCEL_COMBAT),
    SMSG_SPELLHEALLOG(SMSG_SPELLHEALLOG),
    SMSG_SPELLENERGIZELOG(SMSG_SPELLENERGIZELOG),
    SMSG_BINDPOINTUPDATE(SMSG_BINDPOINTUPDATE),
    SMSG_PLAYERBOUND(SMSG_PLAYERBOUND),
    SMSG_CLIENT_CONTROL_UPDATE(SMSG_CLIENT_CONTROL_UPDATE),
    SMSG_RESURRECT_REQUEST(SMSG_RESURRECT_REQUEST),
    SMSG_LOOT_RESPONSE(SMSG_LOOT_RESPONSE),
    SMSG_LOOT_RELEASE_RESPONSE(SMSG_LOOT_RELEASE_RESPONSE),
    SMSG_LOOT_REMOVED(SMSG_LOOT_REMOVED),
    SMSG_LOOT_MONEY_NOTIFY(SMSG_LOOT_MONEY_NOTIFY),
    SMSG_LOOT_CLEAR_MONEY(SMSG_LOOT_CLEAR_MONEY),
    SMSG_ITEM_PUSH_RESULT(SMSG_ITEM_PUSH_RESULT),
    SMSG_DUEL_REQUESTED(SMSG_DUEL_REQUESTED),
    SMSG_DUEL_OUTOFBOUNDS(SMSG_DUEL_OUTOFBOUNDS),
    SMSG_DUEL_INBOUNDS(SMSG_DUEL_INBOUNDS),
    SMSG_DUEL_COMPLETE(SMSG_DUEL_COMPLETE),
    SMSG_DUEL_WINNER(SMSG_DUEL_WINNER),
    SMSG_MOUNTRESULT(SMSG_MOUNTRESULT),
    SMSG_DISMOUNTRESULT(SMSG_DISMOUNTRESULT),
    SMSG_MOUNTSPECIAL_ANIM(SMSG_MOUNTSPECIAL_ANIM),
    SMSG_PET_TAME_FAILURE(SMSG_PET_TAME_FAILURE),
    SMSG_PET_NAME_INVALID(SMSG_PET_NAME_INVALID),
    SMSG_PET_SPELLS(SMSG_PET_SPELLS),
    SMSG_PET_MODE(SMSG_PET_MODE),
    SMSG_GOSSIP_MESSAGE(SMSG_GOSSIP_MESSAGE),
    SMSG_GOSSIP_COMPLETE(SMSG_GOSSIP_COMPLETE),
    SMSG_NPC_TEXT_UPDATE(SMSG_NPC_TEXT_UPDATE),
    SMSG_QUESTGIVER_STATUS(SMSG_QUESTGIVER_STATUS),
    SMSG_QUESTGIVER_QUEST_LIST(SMSG_QUESTGIVER_QUEST_LIST),
    SMSG_QUESTGIVER_QUEST_DETAILS(SMSG_QUESTGIVER_QUEST_DETAILS),
    SMSG_QUESTGIVER_REQUEST_ITEMS(SMSG_QUESTGIVER_REQUEST_ITEMS),
    SMSG_QUESTGIVER_OFFER_REWARD(SMSG_QUESTGIVER_OFFER_REWARD),
    SMSG_QUESTGIVER_QUEST_INVALID(SMSG_QUESTGIVER_QUEST_INVALID),
    SMSG_QUESTGIVER_QUEST_COMPLETE(SMSG_QUESTGIVER_QUEST_COMPLETE),
    SMSG_QUESTGIVER_QUEST_FAILED(SMSG_QUESTGIVER_QUEST_FAILED),
    SMSG_QUESTLOG_FULL(SMSG_QUESTLOG_FULL),
    SMSG_QUESTUPDATE_FAILED(SMSG_QUESTUPDATE_FAILED),
    SMSG_QUESTUPDATE_FAILEDTIMER(SMSG_QUESTUPDATE_FAILEDTIMER),
    SMSG_QUESTUPDATE_COMPLETE(SMSG_QUESTUPDATE_COMPLETE),
    SMSG_QUESTUPDATE_ADD_KILL(SMSG_QUESTUPDATE_ADD_KILL),
    SMSG_QUESTUPDATE_ADD_ITEM(SMSG_QUESTUPDATE_ADD_ITEM),
    SMSG_QUEST_CONFIRM_ACCEPT(SMSG_QUEST_CONFIRM_ACCEPT),
    SMSG_LIST_INVENTORY(SMSG_LIST_INVENTORY),
    SMSG_SELL_ITEM(SMSG_SELL_ITEM),
    SMSG_BUY_ITEM(SMSG_BUY_ITEM),
    SMSG_BUY_FAILED(SMSG_BUY_FAILED),
    SMSG_SHOWTAXINODES(SMSG_SHOWTAXINODES),
    SMSG_TAXINODE_STATUS(SMSG_TAXINODE_STATUS),
    SMSG_ACTIVATETAXIREPLY(SMSG_ACTIVATETAXIREPLY),
    SMSG_NEW_TAXI_PATH(SMSG_NEW_TAXI_PATH),
    SMSG_TRAINER_LIST(SMSG_TRAINER_LIST),
    SMSG_TRAINER_BUY_SUCCEEDED(SMSG_TRAINER_BUY_SUCCEEDED),
    SMSG_TRAINER_BUY_FAILED(SMSG_TRAINER_BUY_FAILED),
    SMSG_SHOW_BANK(SMSG_SHOW_BANK),
    SMSG_BUY_BANK_SLOT_RESULT(SMSG_BUY_BANK_SLOT_RESULT),
    SMSG_PETITION_SHOWLIST(SMSG_PETITION_SHOWLIST),
    SMSG_PETITION_SHOW_SIGNATURES(SMSG_PETITION_SHOW_SIGNATURES),
    SMSG_PETITION_SIGN_RESULTS(SMSG_PETITION_SIGN_RESULTS),
    SMSG_TURN_IN_PETITION_RESULTS(SMSG_TURN_IN_PETITION_RESULTS),
    SMSG_PETITION_QUERY_RESPONSE(SMSG_PETITION_QUERY_RESPONSE),
    SMSG_FISH_NOT_HOOKED(SMSG_FISH_NOT_HOOKED),
    SMSG_FISH_ESCAPED(SMSG_FISH_ESCAPED),
    SMSG_NOTIFICATION(SMSG_NOTIFICATION),
    SMSG_PLAYED_TIME(SMSG_PLAYED_TIME),
    SMSG_QUERY_TIME_RESPONSE(SMSG_QUERY_TIME_RESPONSE),
    SMSG_LOG_XPGAIN(SMSG_LOG_XPGAIN),
    SMSG_LEVELUP_INFO(SMSG_LEVELUP_INFO),
    MSG_MINIMAP_PING(MSG_MINIMAP_PING_Server),
    SMSG_RESISTLOG(SMSG_RESISTLOG),
    SMSG_ENCHANTMENTLOG(SMSG_ENCHANTMENTLOG),
    SMSG_START_MIRROR_TIMER(SMSG_START_MIRROR_TIMER),
    SMSG_PAUSE_MIRROR_TIMER(SMSG_PAUSE_MIRROR_TIMER),
    SMSG_STOP_MIRROR_TIMER(SMSG_STOP_MIRROR_TIMER),
    SMSG_PONG(SMSG_PONG),
    SMSG_CLEAR_COOLDOWN(SMSG_CLEAR_COOLDOWN),
    SMSG_GAMEOBJECT_PAGETEXT(SMSG_GAMEOBJECT_PAGETEXT),
    SMSG_SPELL_DELAYED(SMSG_SPELL_DELAYED),
    SMSG_ITEM_TIME_UPDATE(SMSG_ITEM_TIME_UPDATE),
    SMSG_ITEM_ENCHANT_TIME_UPDATE(SMSG_ITEM_ENCHANT_TIME_UPDATE),
    SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE),
    SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE),
    MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Server),
    SMSG_PLAY_SPELL_VISUAL(SMSG_PLAY_SPELL_VISUAL),
    SMSG_PARTYKILLLOG(SMSG_PARTYKILLLOG),
    SMSG_PLAY_SPELL_IMPACT(SMSG_PLAY_SPELL_IMPACT),
    SMSG_EXPLORATION_EXPERIENCE(SMSG_EXPLORATION_EXPERIENCE),
    MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Server),
    SMSG_ENVIRONMENTALDAMAGELOG(SMSG_ENVIRONMENTALDAMAGELOG),
    MSG_LOOKING_FOR_GROUP(MSG_LOOKING_FOR_GROUP_Server),
    SMSG_REMOVED_SPELL(SMSG_REMOVED_SPELL),
    SMSG_GMTICKET_CREATE(SMSG_GMTICKET_CREATE),
    SMSG_GMTICKET_UPDATETEXT(SMSG_GMTICKET_UPDATETEXT),
    SMSG_ACCOUNT_DATA_TIMES(SMSG_ACCOUNT_DATA_TIMES),
    SMSG_GMTICKET_GETTICKET(SMSG_GMTICKET_GETTICKET),
    SMSG_GAMEOBJECT_SPAWN_ANIM(SMSG_GAMEOBJECT_SPAWN_ANIM),
    SMSG_GAMEOBJECT_DESPAWN_ANIM(SMSG_GAMEOBJECT_DESPAWN_ANIM),
    MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Server),
    SMSG_GMTICKET_DELETETICKET(SMSG_GMTICKET_DELETETICKET),
    SMSG_CHAT_WRONG_FACTION(SMSG_CHAT_WRONG_FACTION),
    SMSG_GMTICKET_SYSTEMSTATUS(SMSG_GMTICKET_SYSTEMSTATUS),
    SMSG_SET_REST_START(SMSG_SET_REST_START),
    SMSG_SPIRIT_HEALER_CONFIRM(SMSG_SPIRIT_HEALER_CONFIRM),
    SMSG_GOSSIP_POI(SMSG_GOSSIP_POI),
    SMSG_LOGIN_VERIFY_WORLD(SMSG_LOGIN_VERIFY_WORLD),
    SMSG_MAIL_LIST_RESULT(SMSG_MAIL_LIST_RESULT),
    SMSG_BATTLEFIELD_LIST(SMSG_BATTLEFIELD_LIST),
    SMSG_ITEM_TEXT_QUERY_RESPONSE(SMSG_ITEM_TEXT_QUERY_RESPONSE),
    SMSG_SPELLLOGMISS(SMSG_SPELLLOGMISS),
    SMSG_SPELLLOGEXECUTE(SMSG_SPELLLOGEXECUTE),
    SMSG_PERIODICAURALOG(SMSG_PERIODICAURALOG),
    SMSG_SPELLDAMAGESHIELD(SMSG_SPELLDAMAGESHIELD),
    SMSG_SPELLNONMELEEDAMAGELOG(SMSG_SPELLNONMELEEDAMAGELOG),
    SMSG_ZONE_UNDER_ATTACK(SMSG_ZONE_UNDER_ATTACK),
    MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Server),
    SMSG_AUCTION_LIST_RESULT(SMSG_AUCTION_LIST_RESULT),
    SMSG_AUCTION_OWNER_LIST_RESULT(SMSG_AUCTION_OWNER_LIST_RESULT),
    SMSG_AUCTION_BIDDER_NOTIFICATION(SMSG_AUCTION_BIDDER_NOTIFICATION),
    SMSG_AUCTION_OWNER_NOTIFICATION(SMSG_AUCTION_OWNER_NOTIFICATION),
    SMSG_PROCRESIST(SMSG_PROCRESIST),
    SMSG_DISPEL_FAILED(SMSG_DISPEL_FAILED),
    SMSG_SPELLORDAMAGE_IMMUNE(SMSG_SPELLORDAMAGE_IMMUNE),
    SMSG_AUCTION_BIDDER_LIST_RESULT(SMSG_AUCTION_BIDDER_LIST_RESULT),
    SMSG_SET_FLAT_SPELL_MODIFIER(SMSG_SET_FLAT_SPELL_MODIFIER),
    SMSG_SET_PCT_SPELL_MODIFIER(SMSG_SET_PCT_SPELL_MODIFIER),
    SMSG_CORPSE_RECLAIM_DELAY(SMSG_CORPSE_RECLAIM_DELAY),
    MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Server),
    SMSG_STABLE_RESULT(SMSG_STABLE_RESULT),
    SMSG_PLAY_MUSIC(SMSG_PLAY_MUSIC),
    SMSG_PLAY_OBJECT_SOUND(SMSG_PLAY_OBJECT_SOUND),
    SMSG_SPELLDISPELLOG(SMSG_SPELLDISPELLOG),
    MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Server),
    SMSG_RECEIVED_MAIL(SMSG_RECEIVED_MAIL),
    SMSG_RAID_GROUP_ONLY(SMSG_RAID_GROUP_ONLY),
    SMSG_PVP_CREDIT(SMSG_PVP_CREDIT),
    SMSG_AUCTION_REMOVED_NOTIFICATION(SMSG_AUCTION_REMOVED_NOTIFICATION),
    SMSG_SERVER_MESSAGE(SMSG_SERVER_MESSAGE),
    SMSG_MEETINGSTONE_SETQUEUE(SMSG_MEETINGSTONE_SETQUEUE),
    SMSG_MEETINGSTONE_COMPLETE(SMSG_MEETINGSTONE_COMPLETE),
    SMSG_MEETINGSTONE_IN_PROGRESS(SMSG_MEETINGSTONE_IN_PROGRESS),
    SMSG_MEETINGSTONE_MEMBER_ADDED(SMSG_MEETINGSTONE_MEMBER_ADDED),
    SMSG_CANCEL_AUTO_REPEAT(SMSG_CANCEL_AUTO_REPEAT),
    SMSG_STANDSTATE_UPDATE(SMSG_STANDSTATE_UPDATE),
    SMSG_LOOT_ALL_PASSED(SMSG_LOOT_ALL_PASSED),
    SMSG_LOOT_ROLL_WON(SMSG_LOOT_ROLL_WON),
    SMSG_LOOT_START_ROLL(SMSG_LOOT_START_ROLL),
    SMSG_LOOT_ROLL(SMSG_LOOT_ROLL),
    SMSG_LOOT_MASTER_LIST(SMSG_LOOT_MASTER_LIST),
    SMSG_SET_FORCED_REACTIONS(SMSG_SET_FORCED_REACTIONS),
    SMSG_SPELL_FAILED_OTHER(SMSG_SPELL_FAILED_OTHER),
    SMSG_GAMEOBJECT_RESET_STATE(SMSG_GAMEOBJECT_RESET_STATE),
    SMSG_CHAT_PLAYER_NOT_FOUND(SMSG_CHAT_PLAYER_NOT_FOUND),
    MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Server),
    SMSG_SUMMON_REQUEST(SMSG_SUMMON_REQUEST),
    SMSG_MONSTER_MOVE_TRANSPORT(SMSG_MONSTER_MOVE_TRANSPORT),
    SMSG_PET_BROKEN(SMSG_PET_BROKEN),
    SMSG_FEIGN_DEATH_RESISTED(SMSG_FEIGN_DEATH_RESISTED),
    SMSG_DUEL_COUNTDOWN(SMSG_DUEL_COUNTDOWN),
    SMSG_AREA_TRIGGER_MESSAGE(SMSG_AREA_TRIGGER_MESSAGE),
    SMSG_MEETINGSTONE_JOINFAILED(SMSG_MEETINGSTONE_JOINFAILED),
    SMSG_PLAYER_SKINNED(SMSG_PLAYER_SKINNED),
    SMSG_DURABILITY_DAMAGE_DEATH(SMSG_DURABILITY_DAMAGE_DEATH),
    SMSG_INIT_WORLD_STATES(SMSG_INIT_WORLD_STATES),
    SMSG_UPDATE_WORLD_STATE(SMSG_UPDATE_WORLD_STATE),
    SMSG_ITEM_NAME_QUERY_RESPONSE(SMSG_ITEM_NAME_QUERY_RESPONSE),
    SMSG_PET_ACTION_FEEDBACK(SMSG_PET_ACTION_FEEDBACK),
    SMSG_CHAR_RENAME(SMSG_CHAR_RENAME),
    SMSG_INSTANCE_SAVE_CREATED(SMSG_INSTANCE_SAVE_CREATED),
    SMSG_RAID_INSTANCE_INFO(SMSG_RAID_INSTANCE_INFO),
    SMSG_PLAY_SOUND(SMSG_PLAY_SOUND),
    SMSG_BATTLEFIELD_STATUS(SMSG_BATTLEFIELD_STATUS),
    MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Server),
    SMSG_FORCE_WALK_SPEED_CHANGE(SMSG_FORCE_WALK_SPEED_CHANGE),
    SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(SMSG_FORCE_SWIM_BACK_SPEED_CHANGE),
    SMSG_FORCE_TURN_RATE_CHANGE(SMSG_FORCE_TURN_RATE_CHANGE),
    MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_Server),
    SMSG_AREA_SPIRIT_HEALER_TIME(SMSG_AREA_SPIRIT_HEALER_TIME),
    SMSG_GROUP_JOINED_BATTLEGROUND(SMSG_GROUP_JOINED_BATTLEGROUND),
    MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Server),
    SMSG_BINDER_CONFIRM(SMSG_BINDER_CONFIRM),
    SMSG_BATTLEGROUND_PLAYER_JOINED(SMSG_BATTLEGROUND_PLAYER_JOINED),
    SMSG_BATTLEGROUND_PLAYER_LEFT(SMSG_BATTLEGROUND_PLAYER_LEFT),
    SMSG_PET_UNLEARN_CONFIRM(SMSG_PET_UNLEARN_CONFIRM),
    SMSG_PARTY_MEMBER_STATS_FULL(SMSG_PARTY_MEMBER_STATS_FULL),
    SMSG_WEATHER(SMSG_WEATHER),
    SMSG_RAID_INSTANCE_MESSAGE(SMSG_RAID_INSTANCE_MESSAGE),
    SMSG_CHAT_RESTRICTED(SMSG_CHAT_RESTRICTED),
    SMSG_SPLINE_SET_RUN_SPEED(SMSG_SPLINE_SET_RUN_SPEED),
    SMSG_SPLINE_SET_RUN_BACK_SPEED(SMSG_SPLINE_SET_RUN_BACK_SPEED),
    SMSG_SPLINE_SET_SWIM_SPEED(SMSG_SPLINE_SET_SWIM_SPEED),
    SMSG_SPLINE_SET_WALK_SPEED(SMSG_SPLINE_SET_WALK_SPEED),
    SMSG_SPLINE_SET_SWIM_BACK_SPEED(SMSG_SPLINE_SET_SWIM_BACK_SPEED),
    SMSG_SPLINE_SET_TURN_RATE(SMSG_SPLINE_SET_TURN_RATE),
    SMSG_SPLINE_MOVE_UNROOT(SMSG_SPLINE_MOVE_UNROOT),
    SMSG_SPLINE_MOVE_FEATHER_FALL(SMSG_SPLINE_MOVE_FEATHER_FALL),
    SMSG_SPLINE_MOVE_NORMAL_FALL(SMSG_SPLINE_MOVE_NORMAL_FALL),
    SMSG_SPLINE_MOVE_SET_HOVER(SMSG_SPLINE_MOVE_SET_HOVER),
    SMSG_SPLINE_MOVE_UNSET_HOVER(SMSG_SPLINE_MOVE_UNSET_HOVER),
    SMSG_SPLINE_MOVE_WATER_WALK(SMSG_SPLINE_MOVE_WATER_WALK),
    SMSG_SPLINE_MOVE_LAND_WALK(SMSG_SPLINE_MOVE_LAND_WALK),
    SMSG_SPLINE_MOVE_START_SWIM(SMSG_SPLINE_MOVE_START_SWIM),
    SMSG_SPLINE_MOVE_STOP_SWIM(SMSG_SPLINE_MOVE_STOP_SWIM),
    SMSG_SPLINE_MOVE_SET_RUN_MODE(SMSG_SPLINE_MOVE_SET_RUN_MODE),
    SMSG_SPLINE_MOVE_SET_WALK_MODE(SMSG_SPLINE_MOVE_SET_WALK_MODE),
    SMSG_SPLINE_MOVE_ROOT(SMSG_SPLINE_MOVE_ROOT),
    SMSG_INVALIDATE_PLAYER(SMSG_INVALIDATE_PLAYER),
    SMSG_INSTANCE_RESET(SMSG_INSTANCE_RESET),
    SMSG_INSTANCE_RESET_FAILED(SMSG_INSTANCE_RESET_FAILED),
    SMSG_UPDATE_LAST_INSTANCE(SMSG_UPDATE_LAST_INSTANCE),
    MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Server),
    MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Server),
    SMSG_PET_ACTION_SOUND(SMSG_PET_ACTION_SOUND),
    SMSG_PET_DISMISS_SOUND(SMSG_PET_DISMISS_SOUND),
    SMSG_GM_TICKET_STATUS_UPDATE(SMSG_GM_TICKET_STATUS_UPDATE),
    SMSG_UPDATE_INSTANCE_OWNERSHIP(SMSG_UPDATE_INSTANCE_OWNERSHIP),
    SMSG_SPELLINSTAKILLLOG(SMSG_SPELLINSTAKILLLOG),
    SMSG_SPELL_UPDATE_CHAIN_TARGETS(SMSG_SPELL_UPDATE_CHAIN_TARGETS),
    SMSG_EXPECTED_SPAM_RECORDS(SMSG_EXPECTED_SPAM_RECORDS),
    SMSG_DEFENSE_MESSAGE(SMSG_DEFENSE_MESSAGE),
}

impl ServerOpcodeMessage {

    #[cfg(feature = "sync")]
    pub fn read_unencrypted<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let size = (crate::util::read_u16_be(r)? - 2) as u32;
        let opcode = crate::util::read_u16_le(r)?;
        match opcode {
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD::read_body(r, size)?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD::read_body(r, size)?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(MSG_MOVE_STOP::read_body(r, size)?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT::read_body(r, size)?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT::read_body(r, size)?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE::read_body(r, size)?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(MSG_MOVE_JUMP::read_body(r, size)?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT::read_body(r, size)?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT::read_body(r, size)?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN::read_body(r, size)?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP::read_body(r, size)?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN::read_body(r, size)?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH::read_body(r, size)?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE::read_body(r, size)?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE::read_body(r, size)?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK::read_body(r, size)?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND::read_body(r, size)?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM::read_body(r, size)?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM::read_body(r, size)?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING::read_body(r, size)?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH::read_body(r, size)?)),
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK::read_body(r, size)?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT::read_body(r, size)?)),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(MSG_PETITION_DECLINE::read_body(r, size)?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE::read_body(r, size)?)),
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT::read_body(r, size)?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(MSG_PETITION_RENAME::read_body(r, size)?)),
            0x003A => Ok(Self::SMSG_CHAR_CREATE(SMSG_CHAR_CREATE::read_body(r, size)?)),
            0x003B => Ok(Self::SMSG_CHAR_ENUM(SMSG_CHAR_ENUM::read_body(r, size)?)),
            0x003C => Ok(Self::SMSG_CHAR_DELETE(SMSG_CHAR_DELETE::read_body(r, size)?)),
            0x003E => Ok(Self::SMSG_NEW_WORLD(SMSG_NEW_WORLD::read_body(r, size)?)),
            0x003F => Ok(Self::SMSG_TRANSFER_PENDING(SMSG_TRANSFER_PENDING::read_body(r, size)?)),
            0x0040 => Ok(Self::SMSG_TRANSFER_ABORTED(SMSG_TRANSFER_ABORTED::read_body(r, size)?)),
            0x0041 => Ok(Self::SMSG_CHARACTER_LOGIN_FAILED(SMSG_CHARACTER_LOGIN_FAILED::read_body(r, size)?)),
            0x0042 => Ok(Self::SMSG_LOGIN_SETTIMESPEED(SMSG_LOGIN_SETTIMESPEED::read_body(r, size)?)),
            0x004C => Ok(Self::SMSG_LOGOUT_RESPONSE(SMSG_LOGOUT_RESPONSE::read_body(r, size)?)),
            0x004D => Ok(Self::SMSG_LOGOUT_COMPLETE(SMSG_LOGOUT_COMPLETE::read_body(r, size)?)),
            0x004F => Ok(Self::SMSG_LOGOUT_CANCEL_ACK(SMSG_LOGOUT_CANCEL_ACK::read_body(r, size)?)),
            0x0051 => Ok(Self::SMSG_NAME_QUERY_RESPONSE(SMSG_NAME_QUERY_RESPONSE::read_body(r, size)?)),
            0x0053 => Ok(Self::SMSG_PET_NAME_QUERY_RESPONSE(SMSG_PET_NAME_QUERY_RESPONSE::read_body(r, size)?)),
            0x0055 => Ok(Self::SMSG_GUILD_QUERY_RESPONSE(SMSG_GUILD_QUERY_RESPONSE::read_body(r, size)?)),
            0x0058 => Ok(Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(SMSG_ITEM_QUERY_SINGLE_RESPONSE::read_body(r, size)?)),
            0x005B => Ok(Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(SMSG_PAGE_TEXT_QUERY_RESPONSE::read_body(r, size)?)),
            0x005D => Ok(Self::SMSG_QUEST_QUERY_RESPONSE(SMSG_QUEST_QUERY_RESPONSE::read_body(r, size)?)),
            0x005F => Ok(Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(SMSG_GAMEOBJECT_QUERY_RESPONSE::read_body(r, size)?)),
            0x0061 => Ok(Self::SMSG_CREATURE_QUERY_RESPONSE(SMSG_CREATURE_QUERY_RESPONSE::read_body(r, size)?)),
            0x0063 => Ok(Self::SMSG_WHO(SMSG_WHO::read_body(r, size)?)),
            0x0065 => Ok(Self::SMSG_WHOIS(SMSG_WHOIS::read_body(r, size)?)),
            0x0067 => Ok(Self::SMSG_FRIEND_LIST(SMSG_FRIEND_LIST::read_body(r, size)?)),
            0x0068 => Ok(Self::SMSG_FRIEND_STATUS(SMSG_FRIEND_STATUS::read_body(r, size)?)),
            0x006B => Ok(Self::SMSG_IGNORE_LIST(SMSG_IGNORE_LIST::read_body(r, size)?)),
            0x006F => Ok(Self::SMSG_GROUP_INVITE(SMSG_GROUP_INVITE::read_body(r, size)?)),
            0x0074 => Ok(Self::SMSG_GROUP_DECLINE(SMSG_GROUP_DECLINE::read_body(r, size)?)),
            0x0077 => Ok(Self::SMSG_GROUP_UNINVITE(SMSG_GROUP_UNINVITE::read_body(r, size)?)),
            0x0079 => Ok(Self::SMSG_GROUP_SET_LEADER(SMSG_GROUP_SET_LEADER::read_body(r, size)?)),
            0x007C => Ok(Self::SMSG_GROUP_DESTROYED(SMSG_GROUP_DESTROYED::read_body(r, size)?)),
            0x007D => Ok(Self::SMSG_GROUP_LIST(SMSG_GROUP_LIST::read_body(r, size)?)),
            0x007E => Ok(Self::SMSG_PARTY_MEMBER_STATS(SMSG_PARTY_MEMBER_STATS::read_body(r, size)?)),
            0x007F => Ok(Self::SMSG_PARTY_COMMAND_RESULT(SMSG_PARTY_COMMAND_RESULT::read_body(r, size)?)),
            0x0083 => Ok(Self::SMSG_GUILD_INVITE(SMSG_GUILD_INVITE::read_body(r, size)?)),
            0x0088 => Ok(Self::SMSG_GUILD_INFO(SMSG_GUILD_INFO::read_body(r, size)?)),
            0x008A => Ok(Self::SMSG_GUILD_ROSTER(SMSG_GUILD_ROSTER::read_body(r, size)?)),
            0x0092 => Ok(Self::SMSG_GUILD_EVENT(SMSG_GUILD_EVENT::read_body(r, size)?)),
            0x0093 => Ok(Self::SMSG_GUILD_COMMAND_RESULT(SMSG_GUILD_COMMAND_RESULT::read_body(r, size)?)),
            0x0096 => Ok(Self::SMSG_MESSAGECHAT(SMSG_MESSAGECHAT::read_body(r, size)?)),
            0x0099 => Ok(Self::SMSG_CHANNEL_NOTIFY(SMSG_CHANNEL_NOTIFY::read_body(r, size)?)),
            0x009B => Ok(Self::SMSG_CHANNEL_LIST(SMSG_CHANNEL_LIST::read_body(r, size)?)),
            0x00AA => Ok(Self::SMSG_DESTROY_OBJECT(SMSG_DESTROY_OBJECT::read_body(r, size)?)),
            0x00AE => Ok(Self::SMSG_READ_ITEM_OK(SMSG_READ_ITEM_OK::read_body(r, size)?)),
            0x00AF => Ok(Self::SMSG_READ_ITEM_FAILED(SMSG_READ_ITEM_FAILED::read_body(r, size)?)),
            0x00B0 => Ok(Self::SMSG_ITEM_COOLDOWN(SMSG_ITEM_COOLDOWN::read_body(r, size)?)),
            0x00B3 => Ok(Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(SMSG_GAMEOBJECT_CUSTOM_ANIM::read_body(r, size)?)),
            0x00DD => Ok(Self::SMSG_MONSTER_MOVE(SMSG_MONSTER_MOVE::read_body(r, size)?)),
            0x00DE => Ok(Self::SMSG_MOVE_WATER_WALK(SMSG_MOVE_WATER_WALK::read_body(r, size)?)),
            0x00DF => Ok(Self::SMSG_MOVE_LAND_WALK(SMSG_MOVE_LAND_WALK::read_body(r, size)?)),
            0x00E2 => Ok(Self::SMSG_FORCE_RUN_SPEED_CHANGE(SMSG_FORCE_RUN_SPEED_CHANGE::read_body(r, size)?)),
            0x00E4 => Ok(Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(SMSG_FORCE_RUN_BACK_SPEED_CHANGE::read_body(r, size)?)),
            0x00E6 => Ok(Self::SMSG_FORCE_SWIM_SPEED_CHANGE(SMSG_FORCE_SWIM_SPEED_CHANGE::read_body(r, size)?)),
            0x00E8 => Ok(Self::SMSG_FORCE_MOVE_ROOT(SMSG_FORCE_MOVE_ROOT::read_body(r, size)?)),
            0x00EA => Ok(Self::SMSG_FORCE_MOVE_UNROOT(SMSG_FORCE_MOVE_UNROOT::read_body(r, size)?)),
            0x00EF => Ok(Self::SMSG_MOVE_KNOCK_BACK(SMSG_MOVE_KNOCK_BACK::read_body(r, size)?)),
            0x00F2 => Ok(Self::SMSG_MOVE_FEATHER_FALL(SMSG_MOVE_FEATHER_FALL::read_body(r, size)?)),
            0x00F3 => Ok(Self::SMSG_MOVE_NORMAL_FALL(SMSG_MOVE_NORMAL_FALL::read_body(r, size)?)),
            0x00F4 => Ok(Self::SMSG_MOVE_SET_HOVER(SMSG_MOVE_SET_HOVER::read_body(r, size)?)),
            0x00F5 => Ok(Self::SMSG_MOVE_UNSET_HOVER(SMSG_MOVE_UNSET_HOVER::read_body(r, size)?)),
            0x00FA => Ok(Self::SMSG_TRIGGER_CINEMATIC(SMSG_TRIGGER_CINEMATIC::read_body(r, size)?)),
            0x00FD => Ok(Self::SMSG_TUTORIAL_FLAGS(SMSG_TUTORIAL_FLAGS::read_body(r, size)?)),
            0x0103 => Ok(Self::SMSG_EMOTE(SMSG_EMOTE::read_body(r, size)?)),
            0x0105 => Ok(Self::SMSG_TEXT_EMOTE(SMSG_TEXT_EMOTE::read_body(r, size)?)),
            0x0112 => Ok(Self::SMSG_INVENTORY_CHANGE_FAILURE(SMSG_INVENTORY_CHANGE_FAILURE::read_body(r, size)?)),
            0x0113 => Ok(Self::SMSG_OPEN_CONTAINER(SMSG_OPEN_CONTAINER::read_body(r, size)?)),
            0x0115 => Ok(Self::SMSG_INSPECT(SMSG_INSPECT::read_body(r, size)?)),
            0x0120 => Ok(Self::SMSG_TRADE_STATUS(SMSG_TRADE_STATUS::read_body(r, size)?)),
            0x0121 => Ok(Self::SMSG_TRADE_STATUS_EXTENDED(SMSG_TRADE_STATUS_EXTENDED::read_body(r, size)?)),
            0x0122 => Ok(Self::SMSG_INITIALIZE_FACTIONS(SMSG_INITIALIZE_FACTIONS::read_body(r, size)?)),
            0x0123 => Ok(Self::SMSG_SET_FACTION_VISIBLE(SMSG_SET_FACTION_VISIBLE::read_body(r, size)?)),
            0x0124 => Ok(Self::SMSG_SET_FACTION_STANDING(SMSG_SET_FACTION_STANDING::read_body(r, size)?)),
            0x0127 => Ok(Self::SMSG_SET_PROFICIENCY(SMSG_SET_PROFICIENCY::read_body(r, size)?)),
            0x0129 => Ok(Self::SMSG_ACTION_BUTTONS(SMSG_ACTION_BUTTONS::read_body(r, size)?)),
            0x012A => Ok(Self::SMSG_INITIAL_SPELLS(SMSG_INITIAL_SPELLS::read_body(r, size)?)),
            0x012B => Ok(Self::SMSG_LEARNED_SPELL(SMSG_LEARNED_SPELL::read_body(r, size)?)),
            0x012C => Ok(Self::SMSG_SUPERCEDED_SPELL(SMSG_SUPERCEDED_SPELL::read_body(r, size)?)),
            0x0130 => Ok(Self::SMSG_CAST_RESULT(SMSG_CAST_RESULT::read_body(r, size)?)),
            0x0131 => Ok(Self::SMSG_SPELL_START(SMSG_SPELL_START::read_body(r, size)?)),
            0x0132 => Ok(Self::SMSG_SPELL_GO(SMSG_SPELL_GO::read_body(r, size)?)),
            0x0133 => Ok(Self::SMSG_SPELL_FAILURE(SMSG_SPELL_FAILURE::read_body(r, size)?)),
            0x0134 => Ok(Self::SMSG_SPELL_COOLDOWN(SMSG_SPELL_COOLDOWN::read_body(r, size)?)),
            0x0135 => Ok(Self::SMSG_COOLDOWN_EVENT(SMSG_COOLDOWN_EVENT::read_body(r, size)?)),
            0x0137 => Ok(Self::SMSG_UPDATE_AURA_DURATION(SMSG_UPDATE_AURA_DURATION::read_body(r, size)?)),
            0x0138 => Ok(Self::SMSG_PET_CAST_FAILED(SMSG_PET_CAST_FAILED::read_body(r, size)?)),
            0x013C => Ok(Self::SMSG_AI_REACTION(SMSG_AI_REACTION::read_body(r, size)?)),
            0x0143 => Ok(Self::SMSG_ATTACKSTART(SMSG_ATTACKSTART::read_body(r, size)?)),
            0x0144 => Ok(Self::SMSG_ATTACKSTOP(SMSG_ATTACKSTOP::read_body(r, size)?)),
            0x0145 => Ok(Self::SMSG_ATTACKSWING_NOTINRANGE(SMSG_ATTACKSWING_NOTINRANGE::read_body(r, size)?)),
            0x0146 => Ok(Self::SMSG_ATTACKSWING_BADFACING(SMSG_ATTACKSWING_BADFACING::read_body(r, size)?)),
            0x0147 => Ok(Self::SMSG_ATTACKSWING_NOTSTANDING(SMSG_ATTACKSWING_NOTSTANDING::read_body(r, size)?)),
            0x0148 => Ok(Self::SMSG_ATTACKSWING_DEADTARGET(SMSG_ATTACKSWING_DEADTARGET::read_body(r, size)?)),
            0x0149 => Ok(Self::SMSG_ATTACKSWING_CANT_ATTACK(SMSG_ATTACKSWING_CANT_ATTACK::read_body(r, size)?)),
            0x014A => Ok(Self::SMSG_ATTACKERSTATEUPDATE(SMSG_ATTACKERSTATEUPDATE::read_body(r, size)?)),
            0x014E => Ok(Self::SMSG_CANCEL_COMBAT(SMSG_CANCEL_COMBAT::read_body(r, size)?)),
            0x0150 => Ok(Self::SMSG_SPELLHEALLOG(SMSG_SPELLHEALLOG::read_body(r, size)?)),
            0x0151 => Ok(Self::SMSG_SPELLENERGIZELOG(SMSG_SPELLENERGIZELOG::read_body(r, size)?)),
            0x0155 => Ok(Self::SMSG_BINDPOINTUPDATE(SMSG_BINDPOINTUPDATE::read_body(r, size)?)),
            0x0158 => Ok(Self::SMSG_PLAYERBOUND(SMSG_PLAYERBOUND::read_body(r, size)?)),
            0x0159 => Ok(Self::SMSG_CLIENT_CONTROL_UPDATE(SMSG_CLIENT_CONTROL_UPDATE::read_body(r, size)?)),
            0x015B => Ok(Self::SMSG_RESURRECT_REQUEST(SMSG_RESURRECT_REQUEST::read_body(r, size)?)),
            0x0160 => Ok(Self::SMSG_LOOT_RESPONSE(SMSG_LOOT_RESPONSE::read_body(r, size)?)),
            0x0161 => Ok(Self::SMSG_LOOT_RELEASE_RESPONSE(SMSG_LOOT_RELEASE_RESPONSE::read_body(r, size)?)),
            0x0162 => Ok(Self::SMSG_LOOT_REMOVED(SMSG_LOOT_REMOVED::read_body(r, size)?)),
            0x0163 => Ok(Self::SMSG_LOOT_MONEY_NOTIFY(SMSG_LOOT_MONEY_NOTIFY::read_body(r, size)?)),
            0x0165 => Ok(Self::SMSG_LOOT_CLEAR_MONEY(SMSG_LOOT_CLEAR_MONEY::read_body(r, size)?)),
            0x0166 => Ok(Self::SMSG_ITEM_PUSH_RESULT(SMSG_ITEM_PUSH_RESULT::read_body(r, size)?)),
            0x0167 => Ok(Self::SMSG_DUEL_REQUESTED(SMSG_DUEL_REQUESTED::read_body(r, size)?)),
            0x0168 => Ok(Self::SMSG_DUEL_OUTOFBOUNDS(SMSG_DUEL_OUTOFBOUNDS::read_body(r, size)?)),
            0x0169 => Ok(Self::SMSG_DUEL_INBOUNDS(SMSG_DUEL_INBOUNDS::read_body(r, size)?)),
            0x016A => Ok(Self::SMSG_DUEL_COMPLETE(SMSG_DUEL_COMPLETE::read_body(r, size)?)),
            0x016B => Ok(Self::SMSG_DUEL_WINNER(SMSG_DUEL_WINNER::read_body(r, size)?)),
            0x016E => Ok(Self::SMSG_MOUNTRESULT(SMSG_MOUNTRESULT::read_body(r, size)?)),
            0x016F => Ok(Self::SMSG_DISMOUNTRESULT(SMSG_DISMOUNTRESULT::read_body(r, size)?)),
            0x0172 => Ok(Self::SMSG_MOUNTSPECIAL_ANIM(SMSG_MOUNTSPECIAL_ANIM::read_body(r, size)?)),
            0x0173 => Ok(Self::SMSG_PET_TAME_FAILURE(SMSG_PET_TAME_FAILURE::read_body(r, size)?)),
            0x0178 => Ok(Self::SMSG_PET_NAME_INVALID(SMSG_PET_NAME_INVALID::read_body(r, size)?)),
            0x0179 => Ok(Self::SMSG_PET_SPELLS(SMSG_PET_SPELLS::read_body(r, size)?)),
            0x017A => Ok(Self::SMSG_PET_MODE(SMSG_PET_MODE::read_body(r, size)?)),
            0x017D => Ok(Self::SMSG_GOSSIP_MESSAGE(SMSG_GOSSIP_MESSAGE::read_body(r, size)?)),
            0x017E => Ok(Self::SMSG_GOSSIP_COMPLETE(SMSG_GOSSIP_COMPLETE::read_body(r, size)?)),
            0x0180 => Ok(Self::SMSG_NPC_TEXT_UPDATE(SMSG_NPC_TEXT_UPDATE::read_body(r, size)?)),
            0x0183 => Ok(Self::SMSG_QUESTGIVER_STATUS(SMSG_QUESTGIVER_STATUS::read_body(r, size)?)),
            0x0185 => Ok(Self::SMSG_QUESTGIVER_QUEST_LIST(SMSG_QUESTGIVER_QUEST_LIST::read_body(r, size)?)),
            0x0188 => Ok(Self::SMSG_QUESTGIVER_QUEST_DETAILS(SMSG_QUESTGIVER_QUEST_DETAILS::read_body(r, size)?)),
            0x018B => Ok(Self::SMSG_QUESTGIVER_REQUEST_ITEMS(SMSG_QUESTGIVER_REQUEST_ITEMS::read_body(r, size)?)),
            0x018D => Ok(Self::SMSG_QUESTGIVER_OFFER_REWARD(SMSG_QUESTGIVER_OFFER_REWARD::read_body(r, size)?)),
            0x018F => Ok(Self::SMSG_QUESTGIVER_QUEST_INVALID(SMSG_QUESTGIVER_QUEST_INVALID::read_body(r, size)?)),
            0x0191 => Ok(Self::SMSG_QUESTGIVER_QUEST_COMPLETE(SMSG_QUESTGIVER_QUEST_COMPLETE::read_body(r, size)?)),
            0x0192 => Ok(Self::SMSG_QUESTGIVER_QUEST_FAILED(SMSG_QUESTGIVER_QUEST_FAILED::read_body(r, size)?)),
            0x0195 => Ok(Self::SMSG_QUESTLOG_FULL(SMSG_QUESTLOG_FULL::read_body(r, size)?)),
            0x0196 => Ok(Self::SMSG_QUESTUPDATE_FAILED(SMSG_QUESTUPDATE_FAILED::read_body(r, size)?)),
            0x0197 => Ok(Self::SMSG_QUESTUPDATE_FAILEDTIMER(SMSG_QUESTUPDATE_FAILEDTIMER::read_body(r, size)?)),
            0x0198 => Ok(Self::SMSG_QUESTUPDATE_COMPLETE(SMSG_QUESTUPDATE_COMPLETE::read_body(r, size)?)),
            0x0199 => Ok(Self::SMSG_QUESTUPDATE_ADD_KILL(SMSG_QUESTUPDATE_ADD_KILL::read_body(r, size)?)),
            0x019A => Ok(Self::SMSG_QUESTUPDATE_ADD_ITEM(SMSG_QUESTUPDATE_ADD_ITEM::read_body(r, size)?)),
            0x019C => Ok(Self::SMSG_QUEST_CONFIRM_ACCEPT(SMSG_QUEST_CONFIRM_ACCEPT::read_body(r, size)?)),
            0x019F => Ok(Self::SMSG_LIST_INVENTORY(SMSG_LIST_INVENTORY::read_body(r, size)?)),
            0x01A1 => Ok(Self::SMSG_SELL_ITEM(SMSG_SELL_ITEM::read_body(r, size)?)),
            0x01A4 => Ok(Self::SMSG_BUY_ITEM(SMSG_BUY_ITEM::read_body(r, size)?)),
            0x01A5 => Ok(Self::SMSG_BUY_FAILED(SMSG_BUY_FAILED::read_body(r, size)?)),
            0x01A9 => Ok(Self::SMSG_SHOWTAXINODES(SMSG_SHOWTAXINODES::read_body(r, size)?)),
            0x01AB => Ok(Self::SMSG_TAXINODE_STATUS(SMSG_TAXINODE_STATUS::read_body(r, size)?)),
            0x01AE => Ok(Self::SMSG_ACTIVATETAXIREPLY(SMSG_ACTIVATETAXIREPLY::read_body(r, size)?)),
            0x01AF => Ok(Self::SMSG_NEW_TAXI_PATH(SMSG_NEW_TAXI_PATH::read_body(r, size)?)),
            0x01B1 => Ok(Self::SMSG_TRAINER_LIST(SMSG_TRAINER_LIST::read_body(r, size)?)),
            0x01B3 => Ok(Self::SMSG_TRAINER_BUY_SUCCEEDED(SMSG_TRAINER_BUY_SUCCEEDED::read_body(r, size)?)),
            0x01B4 => Ok(Self::SMSG_TRAINER_BUY_FAILED(SMSG_TRAINER_BUY_FAILED::read_body(r, size)?)),
            0x01B8 => Ok(Self::SMSG_SHOW_BANK(SMSG_SHOW_BANK::read_body(r, size)?)),
            0x01BA => Ok(Self::SMSG_BUY_BANK_SLOT_RESULT(SMSG_BUY_BANK_SLOT_RESULT::read_body(r, size)?)),
            0x01BC => Ok(Self::SMSG_PETITION_SHOWLIST(SMSG_PETITION_SHOWLIST::read_body(r, size)?)),
            0x01BF => Ok(Self::SMSG_PETITION_SHOW_SIGNATURES(SMSG_PETITION_SHOW_SIGNATURES::read_body(r, size)?)),
            0x01C1 => Ok(Self::SMSG_PETITION_SIGN_RESULTS(SMSG_PETITION_SIGN_RESULTS::read_body(r, size)?)),
            0x01C5 => Ok(Self::SMSG_TURN_IN_PETITION_RESULTS(SMSG_TURN_IN_PETITION_RESULTS::read_body(r, size)?)),
            0x01C7 => Ok(Self::SMSG_PETITION_QUERY_RESPONSE(SMSG_PETITION_QUERY_RESPONSE::read_body(r, size)?)),
            0x01C8 => Ok(Self::SMSG_FISH_NOT_HOOKED(SMSG_FISH_NOT_HOOKED::read_body(r, size)?)),
            0x01C9 => Ok(Self::SMSG_FISH_ESCAPED(SMSG_FISH_ESCAPED::read_body(r, size)?)),
            0x01CB => Ok(Self::SMSG_NOTIFICATION(SMSG_NOTIFICATION::read_body(r, size)?)),
            0x01CD => Ok(Self::SMSG_PLAYED_TIME(SMSG_PLAYED_TIME::read_body(r, size)?)),
            0x01CF => Ok(Self::SMSG_QUERY_TIME_RESPONSE(SMSG_QUERY_TIME_RESPONSE::read_body(r, size)?)),
            0x01D0 => Ok(Self::SMSG_LOG_XPGAIN(SMSG_LOG_XPGAIN::read_body(r, size)?)),
            0x01D4 => Ok(Self::SMSG_LEVELUP_INFO(SMSG_LEVELUP_INFO::read_body(r, size)?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(MSG_MINIMAP_PING_Server::read_body(r, size)?)),
            0x01D6 => Ok(Self::SMSG_RESISTLOG(SMSG_RESISTLOG::read_body(r, size)?)),
            0x01D7 => Ok(Self::SMSG_ENCHANTMENTLOG(SMSG_ENCHANTMENTLOG::read_body(r, size)?)),
            0x01D9 => Ok(Self::SMSG_START_MIRROR_TIMER(SMSG_START_MIRROR_TIMER::read_body(r, size)?)),
            0x01DA => Ok(Self::SMSG_PAUSE_MIRROR_TIMER(SMSG_PAUSE_MIRROR_TIMER::read_body(r, size)?)),
            0x01DB => Ok(Self::SMSG_STOP_MIRROR_TIMER(SMSG_STOP_MIRROR_TIMER::read_body(r, size)?)),
            0x01DD => Ok(Self::SMSG_PONG(SMSG_PONG::read_body(r, size)?)),
            0x01DE => Ok(Self::SMSG_CLEAR_COOLDOWN(SMSG_CLEAR_COOLDOWN::read_body(r, size)?)),
            0x01DF => Ok(Self::SMSG_GAMEOBJECT_PAGETEXT(SMSG_GAMEOBJECT_PAGETEXT::read_body(r, size)?)),
            0x01E2 => Ok(Self::SMSG_SPELL_DELAYED(SMSG_SPELL_DELAYED::read_body(r, size)?)),
            0x01EA => Ok(Self::SMSG_ITEM_TIME_UPDATE(SMSG_ITEM_TIME_UPDATE::read_body(r, size)?)),
            0x01EB => Ok(Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(SMSG_ITEM_ENCHANT_TIME_UPDATE::read_body(r, size)?)),
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE::read_body(r, size)?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE::read_body(r, size)?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Server::read_body(r, size)?)),
            0x01F3 => Ok(Self::SMSG_PLAY_SPELL_VISUAL(SMSG_PLAY_SPELL_VISUAL::read_body(r, size)?)),
            0x01F5 => Ok(Self::SMSG_PARTYKILLLOG(SMSG_PARTYKILLLOG::read_body(r, size)?)),
            0x01F7 => Ok(Self::SMSG_PLAY_SPELL_IMPACT(SMSG_PLAY_SPELL_IMPACT::read_body(r, size)?)),
            0x01F8 => Ok(Self::SMSG_EXPLORATION_EXPERIENCE(SMSG_EXPLORATION_EXPERIENCE::read_body(r, size)?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Server::read_body(r, size)?)),
            0x01FC => Ok(Self::SMSG_ENVIRONMENTALDAMAGELOG(SMSG_ENVIRONMENTALDAMAGELOG::read_body(r, size)?)),
            0x01FF => Ok(Self::MSG_LOOKING_FOR_GROUP(MSG_LOOKING_FOR_GROUP_Server::read_body(r, size)?)),
            0x0203 => Ok(Self::SMSG_REMOVED_SPELL(SMSG_REMOVED_SPELL::read_body(r, size)?)),
            0x0206 => Ok(Self::SMSG_GMTICKET_CREATE(SMSG_GMTICKET_CREATE::read_body(r, size)?)),
            0x0208 => Ok(Self::SMSG_GMTICKET_UPDATETEXT(SMSG_GMTICKET_UPDATETEXT::read_body(r, size)?)),
            0x0209 => Ok(Self::SMSG_ACCOUNT_DATA_TIMES(SMSG_ACCOUNT_DATA_TIMES::read_body(r, size)?)),
            0x0212 => Ok(Self::SMSG_GMTICKET_GETTICKET(SMSG_GMTICKET_GETTICKET::read_body(r, size)?)),
            0x0214 => Ok(Self::SMSG_GAMEOBJECT_SPAWN_ANIM(SMSG_GAMEOBJECT_SPAWN_ANIM::read_body(r, size)?)),
            0x0215 => Ok(Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(SMSG_GAMEOBJECT_DESPAWN_ANIM::read_body(r, size)?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Server::read_body(r, size)?)),
            0x0218 => Ok(Self::SMSG_GMTICKET_DELETETICKET(SMSG_GMTICKET_DELETETICKET::read_body(r, size)?)),
            0x0219 => Ok(Self::SMSG_CHAT_WRONG_FACTION(SMSG_CHAT_WRONG_FACTION::read_body(r, size)?)),
            0x021B => Ok(Self::SMSG_GMTICKET_SYSTEMSTATUS(SMSG_GMTICKET_SYSTEMSTATUS::read_body(r, size)?)),
            0x021E => Ok(Self::SMSG_SET_REST_START(SMSG_SET_REST_START::read_body(r, size)?)),
            0x0222 => Ok(Self::SMSG_SPIRIT_HEALER_CONFIRM(SMSG_SPIRIT_HEALER_CONFIRM::read_body(r, size)?)),
            0x0224 => Ok(Self::SMSG_GOSSIP_POI(SMSG_GOSSIP_POI::read_body(r, size)?)),
            0x0236 => Ok(Self::SMSG_LOGIN_VERIFY_WORLD(SMSG_LOGIN_VERIFY_WORLD::read_body(r, size)?)),
            0x023B => Ok(Self::SMSG_MAIL_LIST_RESULT(SMSG_MAIL_LIST_RESULT::read_body(r, size)?)),
            0x023D => Ok(Self::SMSG_BATTLEFIELD_LIST(SMSG_BATTLEFIELD_LIST::read_body(r, size)?)),
            0x0244 => Ok(Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(SMSG_ITEM_TEXT_QUERY_RESPONSE::read_body(r, size)?)),
            0x024B => Ok(Self::SMSG_SPELLLOGMISS(SMSG_SPELLLOGMISS::read_body(r, size)?)),
            0x024C => Ok(Self::SMSG_SPELLLOGEXECUTE(SMSG_SPELLLOGEXECUTE::read_body(r, size)?)),
            0x024E => Ok(Self::SMSG_PERIODICAURALOG(SMSG_PERIODICAURALOG::read_body(r, size)?)),
            0x024F => Ok(Self::SMSG_SPELLDAMAGESHIELD(SMSG_SPELLDAMAGESHIELD::read_body(r, size)?)),
            0x0250 => Ok(Self::SMSG_SPELLNONMELEEDAMAGELOG(SMSG_SPELLNONMELEEDAMAGELOG::read_body(r, size)?)),
            0x0254 => Ok(Self::SMSG_ZONE_UNDER_ATTACK(SMSG_ZONE_UNDER_ATTACK::read_body(r, size)?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Server::read_body(r, size)?)),
            0x025C => Ok(Self::SMSG_AUCTION_LIST_RESULT(SMSG_AUCTION_LIST_RESULT::read_body(r, size)?)),
            0x025D => Ok(Self::SMSG_AUCTION_OWNER_LIST_RESULT(SMSG_AUCTION_OWNER_LIST_RESULT::read_body(r, size)?)),
            0x025E => Ok(Self::SMSG_AUCTION_BIDDER_NOTIFICATION(SMSG_AUCTION_BIDDER_NOTIFICATION::read_body(r, size)?)),
            0x025F => Ok(Self::SMSG_AUCTION_OWNER_NOTIFICATION(SMSG_AUCTION_OWNER_NOTIFICATION::read_body(r, size)?)),
            0x0260 => Ok(Self::SMSG_PROCRESIST(SMSG_PROCRESIST::read_body(r, size)?)),
            0x0262 => Ok(Self::SMSG_DISPEL_FAILED(SMSG_DISPEL_FAILED::read_body(r, size)?)),
            0x0263 => Ok(Self::SMSG_SPELLORDAMAGE_IMMUNE(SMSG_SPELLORDAMAGE_IMMUNE::read_body(r, size)?)),
            0x0265 => Ok(Self::SMSG_AUCTION_BIDDER_LIST_RESULT(SMSG_AUCTION_BIDDER_LIST_RESULT::read_body(r, size)?)),
            0x0266 => Ok(Self::SMSG_SET_FLAT_SPELL_MODIFIER(SMSG_SET_FLAT_SPELL_MODIFIER::read_body(r, size)?)),
            0x0267 => Ok(Self::SMSG_SET_PCT_SPELL_MODIFIER(SMSG_SET_PCT_SPELL_MODIFIER::read_body(r, size)?)),
            0x0269 => Ok(Self::SMSG_CORPSE_RECLAIM_DELAY(SMSG_CORPSE_RECLAIM_DELAY::read_body(r, size)?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Server::read_body(r, size)?)),
            0x0273 => Ok(Self::SMSG_STABLE_RESULT(SMSG_STABLE_RESULT::read_body(r, size)?)),
            0x0277 => Ok(Self::SMSG_PLAY_MUSIC(SMSG_PLAY_MUSIC::read_body(r, size)?)),
            0x0278 => Ok(Self::SMSG_PLAY_OBJECT_SOUND(SMSG_PLAY_OBJECT_SOUND::read_body(r, size)?)),
            0x027B => Ok(Self::SMSG_SPELLDISPELLOG(SMSG_SPELLDISPELLOG::read_body(r, size)?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Server::read_body(r, size)?)),
            0x0285 => Ok(Self::SMSG_RECEIVED_MAIL(SMSG_RECEIVED_MAIL::read_body(r, size)?)),
            0x0286 => Ok(Self::SMSG_RAID_GROUP_ONLY(SMSG_RAID_GROUP_ONLY::read_body(r, size)?)),
            0x028C => Ok(Self::SMSG_PVP_CREDIT(SMSG_PVP_CREDIT::read_body(r, size)?)),
            0x028D => Ok(Self::SMSG_AUCTION_REMOVED_NOTIFICATION(SMSG_AUCTION_REMOVED_NOTIFICATION::read_body(r, size)?)),
            0x0291 => Ok(Self::SMSG_SERVER_MESSAGE(SMSG_SERVER_MESSAGE::read_body(r, size)?)),
            0x0295 => Ok(Self::SMSG_MEETINGSTONE_SETQUEUE(SMSG_MEETINGSTONE_SETQUEUE::read_body(r, size)?)),
            0x0297 => Ok(Self::SMSG_MEETINGSTONE_COMPLETE(SMSG_MEETINGSTONE_COMPLETE::read_body(r, size)?)),
            0x0298 => Ok(Self::SMSG_MEETINGSTONE_IN_PROGRESS(SMSG_MEETINGSTONE_IN_PROGRESS::read_body(r, size)?)),
            0x0299 => Ok(Self::SMSG_MEETINGSTONE_MEMBER_ADDED(SMSG_MEETINGSTONE_MEMBER_ADDED::read_body(r, size)?)),
            0x029C => Ok(Self::SMSG_CANCEL_AUTO_REPEAT(SMSG_CANCEL_AUTO_REPEAT::read_body(r, size)?)),
            0x029D => Ok(Self::SMSG_STANDSTATE_UPDATE(SMSG_STANDSTATE_UPDATE::read_body(r, size)?)),
            0x029E => Ok(Self::SMSG_LOOT_ALL_PASSED(SMSG_LOOT_ALL_PASSED::read_body(r, size)?)),
            0x029F => Ok(Self::SMSG_LOOT_ROLL_WON(SMSG_LOOT_ROLL_WON::read_body(r, size)?)),
            0x02A1 => Ok(Self::SMSG_LOOT_START_ROLL(SMSG_LOOT_START_ROLL::read_body(r, size)?)),
            0x02A2 => Ok(Self::SMSG_LOOT_ROLL(SMSG_LOOT_ROLL::read_body(r, size)?)),
            0x02A4 => Ok(Self::SMSG_LOOT_MASTER_LIST(SMSG_LOOT_MASTER_LIST::read_body(r, size)?)),
            0x02A5 => Ok(Self::SMSG_SET_FORCED_REACTIONS(SMSG_SET_FORCED_REACTIONS::read_body(r, size)?)),
            0x02A6 => Ok(Self::SMSG_SPELL_FAILED_OTHER(SMSG_SPELL_FAILED_OTHER::read_body(r, size)?)),
            0x02A7 => Ok(Self::SMSG_GAMEOBJECT_RESET_STATE(SMSG_GAMEOBJECT_RESET_STATE::read_body(r, size)?)),
            0x02A9 => Ok(Self::SMSG_CHAT_PLAYER_NOT_FOUND(SMSG_CHAT_PLAYER_NOT_FOUND::read_body(r, size)?)),
            0x02AA => Ok(Self::MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Server::read_body(r, size)?)),
            0x02AB => Ok(Self::SMSG_SUMMON_REQUEST(SMSG_SUMMON_REQUEST::read_body(r, size)?)),
            0x02AE => Ok(Self::SMSG_MONSTER_MOVE_TRANSPORT(SMSG_MONSTER_MOVE_TRANSPORT::read_body(r, size)?)),
            0x02AF => Ok(Self::SMSG_PET_BROKEN(SMSG_PET_BROKEN::read_body(r, size)?)),
            0x02B4 => Ok(Self::SMSG_FEIGN_DEATH_RESISTED(SMSG_FEIGN_DEATH_RESISTED::read_body(r, size)?)),
            0x02B7 => Ok(Self::SMSG_DUEL_COUNTDOWN(SMSG_DUEL_COUNTDOWN::read_body(r, size)?)),
            0x02B8 => Ok(Self::SMSG_AREA_TRIGGER_MESSAGE(SMSG_AREA_TRIGGER_MESSAGE::read_body(r, size)?)),
            0x02BB => Ok(Self::SMSG_MEETINGSTONE_JOINFAILED(SMSG_MEETINGSTONE_JOINFAILED::read_body(r, size)?)),
            0x02BC => Ok(Self::SMSG_PLAYER_SKINNED(SMSG_PLAYER_SKINNED::read_body(r, size)?)),
            0x02BD => Ok(Self::SMSG_DURABILITY_DAMAGE_DEATH(SMSG_DURABILITY_DAMAGE_DEATH::read_body(r, size)?)),
            0x02C2 => Ok(Self::SMSG_INIT_WORLD_STATES(SMSG_INIT_WORLD_STATES::read_body(r, size)?)),
            0x02C3 => Ok(Self::SMSG_UPDATE_WORLD_STATE(SMSG_UPDATE_WORLD_STATE::read_body(r, size)?)),
            0x02C5 => Ok(Self::SMSG_ITEM_NAME_QUERY_RESPONSE(SMSG_ITEM_NAME_QUERY_RESPONSE::read_body(r, size)?)),
            0x02C6 => Ok(Self::SMSG_PET_ACTION_FEEDBACK(SMSG_PET_ACTION_FEEDBACK::read_body(r, size)?)),
            0x02C8 => Ok(Self::SMSG_CHAR_RENAME(SMSG_CHAR_RENAME::read_body(r, size)?)),
            0x02CB => Ok(Self::SMSG_INSTANCE_SAVE_CREATED(SMSG_INSTANCE_SAVE_CREATED::read_body(r, size)?)),
            0x02CC => Ok(Self::SMSG_RAID_INSTANCE_INFO(SMSG_RAID_INSTANCE_INFO::read_body(r, size)?)),
            0x02D2 => Ok(Self::SMSG_PLAY_SOUND(SMSG_PLAY_SOUND::read_body(r, size)?)),
            0x02D4 => Ok(Self::SMSG_BATTLEFIELD_STATUS(SMSG_BATTLEFIELD_STATUS::read_body(r, size)?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Server::read_body(r, size)?)),
            0x02DA => Ok(Self::SMSG_FORCE_WALK_SPEED_CHANGE(SMSG_FORCE_WALK_SPEED_CHANGE::read_body(r, size)?)),
            0x02DC => Ok(Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(SMSG_FORCE_SWIM_BACK_SPEED_CHANGE::read_body(r, size)?)),
            0x02DE => Ok(Self::SMSG_FORCE_TURN_RATE_CHANGE(SMSG_FORCE_TURN_RATE_CHANGE::read_body(r, size)?)),
            0x02E0 => Ok(Self::MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_Server::read_body(r, size)?)),
            0x02E4 => Ok(Self::SMSG_AREA_SPIRIT_HEALER_TIME(SMSG_AREA_SPIRIT_HEALER_TIME::read_body(r, size)?)),
            0x02E8 => Ok(Self::SMSG_GROUP_JOINED_BATTLEGROUND(SMSG_GROUP_JOINED_BATTLEGROUND::read_body(r, size)?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Server::read_body(r, size)?)),
            0x02EB => Ok(Self::SMSG_BINDER_CONFIRM(SMSG_BINDER_CONFIRM::read_body(r, size)?)),
            0x02EC => Ok(Self::SMSG_BATTLEGROUND_PLAYER_JOINED(SMSG_BATTLEGROUND_PLAYER_JOINED::read_body(r, size)?)),
            0x02ED => Ok(Self::SMSG_BATTLEGROUND_PLAYER_LEFT(SMSG_BATTLEGROUND_PLAYER_LEFT::read_body(r, size)?)),
            0x02F1 => Ok(Self::SMSG_PET_UNLEARN_CONFIRM(SMSG_PET_UNLEARN_CONFIRM::read_body(r, size)?)),
            0x02F2 => Ok(Self::SMSG_PARTY_MEMBER_STATS_FULL(SMSG_PARTY_MEMBER_STATS_FULL::read_body(r, size)?)),
            0x02F4 => Ok(Self::SMSG_WEATHER(SMSG_WEATHER::read_body(r, size)?)),
            0x02FA => Ok(Self::SMSG_RAID_INSTANCE_MESSAGE(SMSG_RAID_INSTANCE_MESSAGE::read_body(r, size)?)),
            0x02FD => Ok(Self::SMSG_CHAT_RESTRICTED(SMSG_CHAT_RESTRICTED::read_body(r, size)?)),
            0x02FE => Ok(Self::SMSG_SPLINE_SET_RUN_SPEED(SMSG_SPLINE_SET_RUN_SPEED::read_body(r, size)?)),
            0x02FF => Ok(Self::SMSG_SPLINE_SET_RUN_BACK_SPEED(SMSG_SPLINE_SET_RUN_BACK_SPEED::read_body(r, size)?)),
            0x0300 => Ok(Self::SMSG_SPLINE_SET_SWIM_SPEED(SMSG_SPLINE_SET_SWIM_SPEED::read_body(r, size)?)),
            0x0301 => Ok(Self::SMSG_SPLINE_SET_WALK_SPEED(SMSG_SPLINE_SET_WALK_SPEED::read_body(r, size)?)),
            0x0302 => Ok(Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED(SMSG_SPLINE_SET_SWIM_BACK_SPEED::read_body(r, size)?)),
            0x0303 => Ok(Self::SMSG_SPLINE_SET_TURN_RATE(SMSG_SPLINE_SET_TURN_RATE::read_body(r, size)?)),
            0x0304 => Ok(Self::SMSG_SPLINE_MOVE_UNROOT(SMSG_SPLINE_MOVE_UNROOT::read_body(r, size)?)),
            0x0305 => Ok(Self::SMSG_SPLINE_MOVE_FEATHER_FALL(SMSG_SPLINE_MOVE_FEATHER_FALL::read_body(r, size)?)),
            0x0306 => Ok(Self::SMSG_SPLINE_MOVE_NORMAL_FALL(SMSG_SPLINE_MOVE_NORMAL_FALL::read_body(r, size)?)),
            0x0307 => Ok(Self::SMSG_SPLINE_MOVE_SET_HOVER(SMSG_SPLINE_MOVE_SET_HOVER::read_body(r, size)?)),
            0x0308 => Ok(Self::SMSG_SPLINE_MOVE_UNSET_HOVER(SMSG_SPLINE_MOVE_UNSET_HOVER::read_body(r, size)?)),
            0x0309 => Ok(Self::SMSG_SPLINE_MOVE_WATER_WALK(SMSG_SPLINE_MOVE_WATER_WALK::read_body(r, size)?)),
            0x030A => Ok(Self::SMSG_SPLINE_MOVE_LAND_WALK(SMSG_SPLINE_MOVE_LAND_WALK::read_body(r, size)?)),
            0x030B => Ok(Self::SMSG_SPLINE_MOVE_START_SWIM(SMSG_SPLINE_MOVE_START_SWIM::read_body(r, size)?)),
            0x030C => Ok(Self::SMSG_SPLINE_MOVE_STOP_SWIM(SMSG_SPLINE_MOVE_STOP_SWIM::read_body(r, size)?)),
            0x030D => Ok(Self::SMSG_SPLINE_MOVE_SET_RUN_MODE(SMSG_SPLINE_MOVE_SET_RUN_MODE::read_body(r, size)?)),
            0x030E => Ok(Self::SMSG_SPLINE_MOVE_SET_WALK_MODE(SMSG_SPLINE_MOVE_SET_WALK_MODE::read_body(r, size)?)),
            0x031A => Ok(Self::SMSG_SPLINE_MOVE_ROOT(SMSG_SPLINE_MOVE_ROOT::read_body(r, size)?)),
            0x031C => Ok(Self::SMSG_INVALIDATE_PLAYER(SMSG_INVALIDATE_PLAYER::read_body(r, size)?)),
            0x031E => Ok(Self::SMSG_INSTANCE_RESET(SMSG_INSTANCE_RESET::read_body(r, size)?)),
            0x031F => Ok(Self::SMSG_INSTANCE_RESET_FAILED(SMSG_INSTANCE_RESET_FAILED::read_body(r, size)?)),
            0x0320 => Ok(Self::SMSG_UPDATE_LAST_INSTANCE(SMSG_UPDATE_LAST_INSTANCE::read_body(r, size)?)),
            0x0321 => Ok(Self::MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Server::read_body(r, size)?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Server::read_body(r, size)?)),
            0x0324 => Ok(Self::SMSG_PET_ACTION_SOUND(SMSG_PET_ACTION_SOUND::read_body(r, size)?)),
            0x0325 => Ok(Self::SMSG_PET_DISMISS_SOUND(SMSG_PET_DISMISS_SOUND::read_body(r, size)?)),
            0x0328 => Ok(Self::SMSG_GM_TICKET_STATUS_UPDATE(SMSG_GM_TICKET_STATUS_UPDATE::read_body(r, size)?)),
            0x032B => Ok(Self::SMSG_UPDATE_INSTANCE_OWNERSHIP(SMSG_UPDATE_INSTANCE_OWNERSHIP::read_body(r, size)?)),
            0x032F => Ok(Self::SMSG_SPELLINSTAKILLLOG(SMSG_SPELLINSTAKILLLOG::read_body(r, size)?)),
            0x0330 => Ok(Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS(SMSG_SPELL_UPDATE_CHAIN_TARGETS::read_body(r, size)?)),
            0x0332 => Ok(Self::SMSG_EXPECTED_SPAM_RECORDS(SMSG_EXPECTED_SPAM_RECORDS::read_body(r, size)?)),
            0x033B => Ok(Self::SMSG_DEFENSE_MESSAGE(SMSG_DEFENSE_MESSAGE::read_body(r, size)?)),
            _ => Err(ServerOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }
    #[cfg(feature = "sync")]
    pub fn read_encrypted<R: std::io::Read, D: Decrypter>(r: &mut R, d: &mut D) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let mut header = [0u8; 4];
        r.read_exact(&mut header)?;
        let header = d.decrypt_server_header(header);
        let header_size = (header.size - 2) as u32;
        match header.opcode {
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD::read_body(r, header_size)?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD::read_body(r, header_size)?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(MSG_MOVE_STOP::read_body(r, header_size)?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT::read_body(r, header_size)?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT::read_body(r, header_size)?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE::read_body(r, header_size)?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(MSG_MOVE_JUMP::read_body(r, header_size)?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT::read_body(r, header_size)?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT::read_body(r, header_size)?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN::read_body(r, header_size)?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP::read_body(r, header_size)?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN::read_body(r, header_size)?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH::read_body(r, header_size)?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE::read_body(r, header_size)?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE::read_body(r, header_size)?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK::read_body(r, header_size)?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND::read_body(r, header_size)?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM::read_body(r, header_size)?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM::read_body(r, header_size)?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING::read_body(r, header_size)?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH::read_body(r, header_size)?)),
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK::read_body(r, header_size)?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT::read_body(r, header_size)?)),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(MSG_PETITION_DECLINE::read_body(r, header_size)?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE::read_body(r, header_size)?)),
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT::read_body(r, header_size)?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(MSG_PETITION_RENAME::read_body(r, header_size)?)),
            0x003A => Ok(Self::SMSG_CHAR_CREATE(SMSG_CHAR_CREATE::read_body(r, header_size)?)),
            0x003B => Ok(Self::SMSG_CHAR_ENUM(SMSG_CHAR_ENUM::read_body(r, header_size)?)),
            0x003C => Ok(Self::SMSG_CHAR_DELETE(SMSG_CHAR_DELETE::read_body(r, header_size)?)),
            0x003E => Ok(Self::SMSG_NEW_WORLD(SMSG_NEW_WORLD::read_body(r, header_size)?)),
            0x003F => Ok(Self::SMSG_TRANSFER_PENDING(SMSG_TRANSFER_PENDING::read_body(r, header_size)?)),
            0x0040 => Ok(Self::SMSG_TRANSFER_ABORTED(SMSG_TRANSFER_ABORTED::read_body(r, header_size)?)),
            0x0041 => Ok(Self::SMSG_CHARACTER_LOGIN_FAILED(SMSG_CHARACTER_LOGIN_FAILED::read_body(r, header_size)?)),
            0x0042 => Ok(Self::SMSG_LOGIN_SETTIMESPEED(SMSG_LOGIN_SETTIMESPEED::read_body(r, header_size)?)),
            0x004C => Ok(Self::SMSG_LOGOUT_RESPONSE(SMSG_LOGOUT_RESPONSE::read_body(r, header_size)?)),
            0x004D => Ok(Self::SMSG_LOGOUT_COMPLETE(SMSG_LOGOUT_COMPLETE::read_body(r, header_size)?)),
            0x004F => Ok(Self::SMSG_LOGOUT_CANCEL_ACK(SMSG_LOGOUT_CANCEL_ACK::read_body(r, header_size)?)),
            0x0051 => Ok(Self::SMSG_NAME_QUERY_RESPONSE(SMSG_NAME_QUERY_RESPONSE::read_body(r, header_size)?)),
            0x0053 => Ok(Self::SMSG_PET_NAME_QUERY_RESPONSE(SMSG_PET_NAME_QUERY_RESPONSE::read_body(r, header_size)?)),
            0x0055 => Ok(Self::SMSG_GUILD_QUERY_RESPONSE(SMSG_GUILD_QUERY_RESPONSE::read_body(r, header_size)?)),
            0x0058 => Ok(Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(SMSG_ITEM_QUERY_SINGLE_RESPONSE::read_body(r, header_size)?)),
            0x005B => Ok(Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(SMSG_PAGE_TEXT_QUERY_RESPONSE::read_body(r, header_size)?)),
            0x005D => Ok(Self::SMSG_QUEST_QUERY_RESPONSE(SMSG_QUEST_QUERY_RESPONSE::read_body(r, header_size)?)),
            0x005F => Ok(Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(SMSG_GAMEOBJECT_QUERY_RESPONSE::read_body(r, header_size)?)),
            0x0061 => Ok(Self::SMSG_CREATURE_QUERY_RESPONSE(SMSG_CREATURE_QUERY_RESPONSE::read_body(r, header_size)?)),
            0x0063 => Ok(Self::SMSG_WHO(SMSG_WHO::read_body(r, header_size)?)),
            0x0065 => Ok(Self::SMSG_WHOIS(SMSG_WHOIS::read_body(r, header_size)?)),
            0x0067 => Ok(Self::SMSG_FRIEND_LIST(SMSG_FRIEND_LIST::read_body(r, header_size)?)),
            0x0068 => Ok(Self::SMSG_FRIEND_STATUS(SMSG_FRIEND_STATUS::read_body(r, header_size)?)),
            0x006B => Ok(Self::SMSG_IGNORE_LIST(SMSG_IGNORE_LIST::read_body(r, header_size)?)),
            0x006F => Ok(Self::SMSG_GROUP_INVITE(SMSG_GROUP_INVITE::read_body(r, header_size)?)),
            0x0074 => Ok(Self::SMSG_GROUP_DECLINE(SMSG_GROUP_DECLINE::read_body(r, header_size)?)),
            0x0077 => Ok(Self::SMSG_GROUP_UNINVITE(SMSG_GROUP_UNINVITE::read_body(r, header_size)?)),
            0x0079 => Ok(Self::SMSG_GROUP_SET_LEADER(SMSG_GROUP_SET_LEADER::read_body(r, header_size)?)),
            0x007C => Ok(Self::SMSG_GROUP_DESTROYED(SMSG_GROUP_DESTROYED::read_body(r, header_size)?)),
            0x007D => Ok(Self::SMSG_GROUP_LIST(SMSG_GROUP_LIST::read_body(r, header_size)?)),
            0x007E => Ok(Self::SMSG_PARTY_MEMBER_STATS(SMSG_PARTY_MEMBER_STATS::read_body(r, header_size)?)),
            0x007F => Ok(Self::SMSG_PARTY_COMMAND_RESULT(SMSG_PARTY_COMMAND_RESULT::read_body(r, header_size)?)),
            0x0083 => Ok(Self::SMSG_GUILD_INVITE(SMSG_GUILD_INVITE::read_body(r, header_size)?)),
            0x0088 => Ok(Self::SMSG_GUILD_INFO(SMSG_GUILD_INFO::read_body(r, header_size)?)),
            0x008A => Ok(Self::SMSG_GUILD_ROSTER(SMSG_GUILD_ROSTER::read_body(r, header_size)?)),
            0x0092 => Ok(Self::SMSG_GUILD_EVENT(SMSG_GUILD_EVENT::read_body(r, header_size)?)),
            0x0093 => Ok(Self::SMSG_GUILD_COMMAND_RESULT(SMSG_GUILD_COMMAND_RESULT::read_body(r, header_size)?)),
            0x0096 => Ok(Self::SMSG_MESSAGECHAT(SMSG_MESSAGECHAT::read_body(r, header_size)?)),
            0x0099 => Ok(Self::SMSG_CHANNEL_NOTIFY(SMSG_CHANNEL_NOTIFY::read_body(r, header_size)?)),
            0x009B => Ok(Self::SMSG_CHANNEL_LIST(SMSG_CHANNEL_LIST::read_body(r, header_size)?)),
            0x00AA => Ok(Self::SMSG_DESTROY_OBJECT(SMSG_DESTROY_OBJECT::read_body(r, header_size)?)),
            0x00AE => Ok(Self::SMSG_READ_ITEM_OK(SMSG_READ_ITEM_OK::read_body(r, header_size)?)),
            0x00AF => Ok(Self::SMSG_READ_ITEM_FAILED(SMSG_READ_ITEM_FAILED::read_body(r, header_size)?)),
            0x00B0 => Ok(Self::SMSG_ITEM_COOLDOWN(SMSG_ITEM_COOLDOWN::read_body(r, header_size)?)),
            0x00B3 => Ok(Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(SMSG_GAMEOBJECT_CUSTOM_ANIM::read_body(r, header_size)?)),
            0x00DD => Ok(Self::SMSG_MONSTER_MOVE(SMSG_MONSTER_MOVE::read_body(r, header_size)?)),
            0x00DE => Ok(Self::SMSG_MOVE_WATER_WALK(SMSG_MOVE_WATER_WALK::read_body(r, header_size)?)),
            0x00DF => Ok(Self::SMSG_MOVE_LAND_WALK(SMSG_MOVE_LAND_WALK::read_body(r, header_size)?)),
            0x00E2 => Ok(Self::SMSG_FORCE_RUN_SPEED_CHANGE(SMSG_FORCE_RUN_SPEED_CHANGE::read_body(r, header_size)?)),
            0x00E4 => Ok(Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(SMSG_FORCE_RUN_BACK_SPEED_CHANGE::read_body(r, header_size)?)),
            0x00E6 => Ok(Self::SMSG_FORCE_SWIM_SPEED_CHANGE(SMSG_FORCE_SWIM_SPEED_CHANGE::read_body(r, header_size)?)),
            0x00E8 => Ok(Self::SMSG_FORCE_MOVE_ROOT(SMSG_FORCE_MOVE_ROOT::read_body(r, header_size)?)),
            0x00EA => Ok(Self::SMSG_FORCE_MOVE_UNROOT(SMSG_FORCE_MOVE_UNROOT::read_body(r, header_size)?)),
            0x00EF => Ok(Self::SMSG_MOVE_KNOCK_BACK(SMSG_MOVE_KNOCK_BACK::read_body(r, header_size)?)),
            0x00F2 => Ok(Self::SMSG_MOVE_FEATHER_FALL(SMSG_MOVE_FEATHER_FALL::read_body(r, header_size)?)),
            0x00F3 => Ok(Self::SMSG_MOVE_NORMAL_FALL(SMSG_MOVE_NORMAL_FALL::read_body(r, header_size)?)),
            0x00F4 => Ok(Self::SMSG_MOVE_SET_HOVER(SMSG_MOVE_SET_HOVER::read_body(r, header_size)?)),
            0x00F5 => Ok(Self::SMSG_MOVE_UNSET_HOVER(SMSG_MOVE_UNSET_HOVER::read_body(r, header_size)?)),
            0x00FA => Ok(Self::SMSG_TRIGGER_CINEMATIC(SMSG_TRIGGER_CINEMATIC::read_body(r, header_size)?)),
            0x00FD => Ok(Self::SMSG_TUTORIAL_FLAGS(SMSG_TUTORIAL_FLAGS::read_body(r, header_size)?)),
            0x0103 => Ok(Self::SMSG_EMOTE(SMSG_EMOTE::read_body(r, header_size)?)),
            0x0105 => Ok(Self::SMSG_TEXT_EMOTE(SMSG_TEXT_EMOTE::read_body(r, header_size)?)),
            0x0112 => Ok(Self::SMSG_INVENTORY_CHANGE_FAILURE(SMSG_INVENTORY_CHANGE_FAILURE::read_body(r, header_size)?)),
            0x0113 => Ok(Self::SMSG_OPEN_CONTAINER(SMSG_OPEN_CONTAINER::read_body(r, header_size)?)),
            0x0115 => Ok(Self::SMSG_INSPECT(SMSG_INSPECT::read_body(r, header_size)?)),
            0x0120 => Ok(Self::SMSG_TRADE_STATUS(SMSG_TRADE_STATUS::read_body(r, header_size)?)),
            0x0121 => Ok(Self::SMSG_TRADE_STATUS_EXTENDED(SMSG_TRADE_STATUS_EXTENDED::read_body(r, header_size)?)),
            0x0122 => Ok(Self::SMSG_INITIALIZE_FACTIONS(SMSG_INITIALIZE_FACTIONS::read_body(r, header_size)?)),
            0x0123 => Ok(Self::SMSG_SET_FACTION_VISIBLE(SMSG_SET_FACTION_VISIBLE::read_body(r, header_size)?)),
            0x0124 => Ok(Self::SMSG_SET_FACTION_STANDING(SMSG_SET_FACTION_STANDING::read_body(r, header_size)?)),
            0x0127 => Ok(Self::SMSG_SET_PROFICIENCY(SMSG_SET_PROFICIENCY::read_body(r, header_size)?)),
            0x0129 => Ok(Self::SMSG_ACTION_BUTTONS(SMSG_ACTION_BUTTONS::read_body(r, header_size)?)),
            0x012A => Ok(Self::SMSG_INITIAL_SPELLS(SMSG_INITIAL_SPELLS::read_body(r, header_size)?)),
            0x012B => Ok(Self::SMSG_LEARNED_SPELL(SMSG_LEARNED_SPELL::read_body(r, header_size)?)),
            0x012C => Ok(Self::SMSG_SUPERCEDED_SPELL(SMSG_SUPERCEDED_SPELL::read_body(r, header_size)?)),
            0x0130 => Ok(Self::SMSG_CAST_RESULT(SMSG_CAST_RESULT::read_body(r, header_size)?)),
            0x0131 => Ok(Self::SMSG_SPELL_START(SMSG_SPELL_START::read_body(r, header_size)?)),
            0x0132 => Ok(Self::SMSG_SPELL_GO(SMSG_SPELL_GO::read_body(r, header_size)?)),
            0x0133 => Ok(Self::SMSG_SPELL_FAILURE(SMSG_SPELL_FAILURE::read_body(r, header_size)?)),
            0x0134 => Ok(Self::SMSG_SPELL_COOLDOWN(SMSG_SPELL_COOLDOWN::read_body(r, header_size)?)),
            0x0135 => Ok(Self::SMSG_COOLDOWN_EVENT(SMSG_COOLDOWN_EVENT::read_body(r, header_size)?)),
            0x0137 => Ok(Self::SMSG_UPDATE_AURA_DURATION(SMSG_UPDATE_AURA_DURATION::read_body(r, header_size)?)),
            0x0138 => Ok(Self::SMSG_PET_CAST_FAILED(SMSG_PET_CAST_FAILED::read_body(r, header_size)?)),
            0x013C => Ok(Self::SMSG_AI_REACTION(SMSG_AI_REACTION::read_body(r, header_size)?)),
            0x0143 => Ok(Self::SMSG_ATTACKSTART(SMSG_ATTACKSTART::read_body(r, header_size)?)),
            0x0144 => Ok(Self::SMSG_ATTACKSTOP(SMSG_ATTACKSTOP::read_body(r, header_size)?)),
            0x0145 => Ok(Self::SMSG_ATTACKSWING_NOTINRANGE(SMSG_ATTACKSWING_NOTINRANGE::read_body(r, header_size)?)),
            0x0146 => Ok(Self::SMSG_ATTACKSWING_BADFACING(SMSG_ATTACKSWING_BADFACING::read_body(r, header_size)?)),
            0x0147 => Ok(Self::SMSG_ATTACKSWING_NOTSTANDING(SMSG_ATTACKSWING_NOTSTANDING::read_body(r, header_size)?)),
            0x0148 => Ok(Self::SMSG_ATTACKSWING_DEADTARGET(SMSG_ATTACKSWING_DEADTARGET::read_body(r, header_size)?)),
            0x0149 => Ok(Self::SMSG_ATTACKSWING_CANT_ATTACK(SMSG_ATTACKSWING_CANT_ATTACK::read_body(r, header_size)?)),
            0x014A => Ok(Self::SMSG_ATTACKERSTATEUPDATE(SMSG_ATTACKERSTATEUPDATE::read_body(r, header_size)?)),
            0x014E => Ok(Self::SMSG_CANCEL_COMBAT(SMSG_CANCEL_COMBAT::read_body(r, header_size)?)),
            0x0150 => Ok(Self::SMSG_SPELLHEALLOG(SMSG_SPELLHEALLOG::read_body(r, header_size)?)),
            0x0151 => Ok(Self::SMSG_SPELLENERGIZELOG(SMSG_SPELLENERGIZELOG::read_body(r, header_size)?)),
            0x0155 => Ok(Self::SMSG_BINDPOINTUPDATE(SMSG_BINDPOINTUPDATE::read_body(r, header_size)?)),
            0x0158 => Ok(Self::SMSG_PLAYERBOUND(SMSG_PLAYERBOUND::read_body(r, header_size)?)),
            0x0159 => Ok(Self::SMSG_CLIENT_CONTROL_UPDATE(SMSG_CLIENT_CONTROL_UPDATE::read_body(r, header_size)?)),
            0x015B => Ok(Self::SMSG_RESURRECT_REQUEST(SMSG_RESURRECT_REQUEST::read_body(r, header_size)?)),
            0x0160 => Ok(Self::SMSG_LOOT_RESPONSE(SMSG_LOOT_RESPONSE::read_body(r, header_size)?)),
            0x0161 => Ok(Self::SMSG_LOOT_RELEASE_RESPONSE(SMSG_LOOT_RELEASE_RESPONSE::read_body(r, header_size)?)),
            0x0162 => Ok(Self::SMSG_LOOT_REMOVED(SMSG_LOOT_REMOVED::read_body(r, header_size)?)),
            0x0163 => Ok(Self::SMSG_LOOT_MONEY_NOTIFY(SMSG_LOOT_MONEY_NOTIFY::read_body(r, header_size)?)),
            0x0165 => Ok(Self::SMSG_LOOT_CLEAR_MONEY(SMSG_LOOT_CLEAR_MONEY::read_body(r, header_size)?)),
            0x0166 => Ok(Self::SMSG_ITEM_PUSH_RESULT(SMSG_ITEM_PUSH_RESULT::read_body(r, header_size)?)),
            0x0167 => Ok(Self::SMSG_DUEL_REQUESTED(SMSG_DUEL_REQUESTED::read_body(r, header_size)?)),
            0x0168 => Ok(Self::SMSG_DUEL_OUTOFBOUNDS(SMSG_DUEL_OUTOFBOUNDS::read_body(r, header_size)?)),
            0x0169 => Ok(Self::SMSG_DUEL_INBOUNDS(SMSG_DUEL_INBOUNDS::read_body(r, header_size)?)),
            0x016A => Ok(Self::SMSG_DUEL_COMPLETE(SMSG_DUEL_COMPLETE::read_body(r, header_size)?)),
            0x016B => Ok(Self::SMSG_DUEL_WINNER(SMSG_DUEL_WINNER::read_body(r, header_size)?)),
            0x016E => Ok(Self::SMSG_MOUNTRESULT(SMSG_MOUNTRESULT::read_body(r, header_size)?)),
            0x016F => Ok(Self::SMSG_DISMOUNTRESULT(SMSG_DISMOUNTRESULT::read_body(r, header_size)?)),
            0x0172 => Ok(Self::SMSG_MOUNTSPECIAL_ANIM(SMSG_MOUNTSPECIAL_ANIM::read_body(r, header_size)?)),
            0x0173 => Ok(Self::SMSG_PET_TAME_FAILURE(SMSG_PET_TAME_FAILURE::read_body(r, header_size)?)),
            0x0178 => Ok(Self::SMSG_PET_NAME_INVALID(SMSG_PET_NAME_INVALID::read_body(r, header_size)?)),
            0x0179 => Ok(Self::SMSG_PET_SPELLS(SMSG_PET_SPELLS::read_body(r, header_size)?)),
            0x017A => Ok(Self::SMSG_PET_MODE(SMSG_PET_MODE::read_body(r, header_size)?)),
            0x017D => Ok(Self::SMSG_GOSSIP_MESSAGE(SMSG_GOSSIP_MESSAGE::read_body(r, header_size)?)),
            0x017E => Ok(Self::SMSG_GOSSIP_COMPLETE(SMSG_GOSSIP_COMPLETE::read_body(r, header_size)?)),
            0x0180 => Ok(Self::SMSG_NPC_TEXT_UPDATE(SMSG_NPC_TEXT_UPDATE::read_body(r, header_size)?)),
            0x0183 => Ok(Self::SMSG_QUESTGIVER_STATUS(SMSG_QUESTGIVER_STATUS::read_body(r, header_size)?)),
            0x0185 => Ok(Self::SMSG_QUESTGIVER_QUEST_LIST(SMSG_QUESTGIVER_QUEST_LIST::read_body(r, header_size)?)),
            0x0188 => Ok(Self::SMSG_QUESTGIVER_QUEST_DETAILS(SMSG_QUESTGIVER_QUEST_DETAILS::read_body(r, header_size)?)),
            0x018B => Ok(Self::SMSG_QUESTGIVER_REQUEST_ITEMS(SMSG_QUESTGIVER_REQUEST_ITEMS::read_body(r, header_size)?)),
            0x018D => Ok(Self::SMSG_QUESTGIVER_OFFER_REWARD(SMSG_QUESTGIVER_OFFER_REWARD::read_body(r, header_size)?)),
            0x018F => Ok(Self::SMSG_QUESTGIVER_QUEST_INVALID(SMSG_QUESTGIVER_QUEST_INVALID::read_body(r, header_size)?)),
            0x0191 => Ok(Self::SMSG_QUESTGIVER_QUEST_COMPLETE(SMSG_QUESTGIVER_QUEST_COMPLETE::read_body(r, header_size)?)),
            0x0192 => Ok(Self::SMSG_QUESTGIVER_QUEST_FAILED(SMSG_QUESTGIVER_QUEST_FAILED::read_body(r, header_size)?)),
            0x0195 => Ok(Self::SMSG_QUESTLOG_FULL(SMSG_QUESTLOG_FULL::read_body(r, header_size)?)),
            0x0196 => Ok(Self::SMSG_QUESTUPDATE_FAILED(SMSG_QUESTUPDATE_FAILED::read_body(r, header_size)?)),
            0x0197 => Ok(Self::SMSG_QUESTUPDATE_FAILEDTIMER(SMSG_QUESTUPDATE_FAILEDTIMER::read_body(r, header_size)?)),
            0x0198 => Ok(Self::SMSG_QUESTUPDATE_COMPLETE(SMSG_QUESTUPDATE_COMPLETE::read_body(r, header_size)?)),
            0x0199 => Ok(Self::SMSG_QUESTUPDATE_ADD_KILL(SMSG_QUESTUPDATE_ADD_KILL::read_body(r, header_size)?)),
            0x019A => Ok(Self::SMSG_QUESTUPDATE_ADD_ITEM(SMSG_QUESTUPDATE_ADD_ITEM::read_body(r, header_size)?)),
            0x019C => Ok(Self::SMSG_QUEST_CONFIRM_ACCEPT(SMSG_QUEST_CONFIRM_ACCEPT::read_body(r, header_size)?)),
            0x019F => Ok(Self::SMSG_LIST_INVENTORY(SMSG_LIST_INVENTORY::read_body(r, header_size)?)),
            0x01A1 => Ok(Self::SMSG_SELL_ITEM(SMSG_SELL_ITEM::read_body(r, header_size)?)),
            0x01A4 => Ok(Self::SMSG_BUY_ITEM(SMSG_BUY_ITEM::read_body(r, header_size)?)),
            0x01A5 => Ok(Self::SMSG_BUY_FAILED(SMSG_BUY_FAILED::read_body(r, header_size)?)),
            0x01A9 => Ok(Self::SMSG_SHOWTAXINODES(SMSG_SHOWTAXINODES::read_body(r, header_size)?)),
            0x01AB => Ok(Self::SMSG_TAXINODE_STATUS(SMSG_TAXINODE_STATUS::read_body(r, header_size)?)),
            0x01AE => Ok(Self::SMSG_ACTIVATETAXIREPLY(SMSG_ACTIVATETAXIREPLY::read_body(r, header_size)?)),
            0x01AF => Ok(Self::SMSG_NEW_TAXI_PATH(SMSG_NEW_TAXI_PATH::read_body(r, header_size)?)),
            0x01B1 => Ok(Self::SMSG_TRAINER_LIST(SMSG_TRAINER_LIST::read_body(r, header_size)?)),
            0x01B3 => Ok(Self::SMSG_TRAINER_BUY_SUCCEEDED(SMSG_TRAINER_BUY_SUCCEEDED::read_body(r, header_size)?)),
            0x01B4 => Ok(Self::SMSG_TRAINER_BUY_FAILED(SMSG_TRAINER_BUY_FAILED::read_body(r, header_size)?)),
            0x01B8 => Ok(Self::SMSG_SHOW_BANK(SMSG_SHOW_BANK::read_body(r, header_size)?)),
            0x01BA => Ok(Self::SMSG_BUY_BANK_SLOT_RESULT(SMSG_BUY_BANK_SLOT_RESULT::read_body(r, header_size)?)),
            0x01BC => Ok(Self::SMSG_PETITION_SHOWLIST(SMSG_PETITION_SHOWLIST::read_body(r, header_size)?)),
            0x01BF => Ok(Self::SMSG_PETITION_SHOW_SIGNATURES(SMSG_PETITION_SHOW_SIGNATURES::read_body(r, header_size)?)),
            0x01C1 => Ok(Self::SMSG_PETITION_SIGN_RESULTS(SMSG_PETITION_SIGN_RESULTS::read_body(r, header_size)?)),
            0x01C5 => Ok(Self::SMSG_TURN_IN_PETITION_RESULTS(SMSG_TURN_IN_PETITION_RESULTS::read_body(r, header_size)?)),
            0x01C7 => Ok(Self::SMSG_PETITION_QUERY_RESPONSE(SMSG_PETITION_QUERY_RESPONSE::read_body(r, header_size)?)),
            0x01C8 => Ok(Self::SMSG_FISH_NOT_HOOKED(SMSG_FISH_NOT_HOOKED::read_body(r, header_size)?)),
            0x01C9 => Ok(Self::SMSG_FISH_ESCAPED(SMSG_FISH_ESCAPED::read_body(r, header_size)?)),
            0x01CB => Ok(Self::SMSG_NOTIFICATION(SMSG_NOTIFICATION::read_body(r, header_size)?)),
            0x01CD => Ok(Self::SMSG_PLAYED_TIME(SMSG_PLAYED_TIME::read_body(r, header_size)?)),
            0x01CF => Ok(Self::SMSG_QUERY_TIME_RESPONSE(SMSG_QUERY_TIME_RESPONSE::read_body(r, header_size)?)),
            0x01D0 => Ok(Self::SMSG_LOG_XPGAIN(SMSG_LOG_XPGAIN::read_body(r, header_size)?)),
            0x01D4 => Ok(Self::SMSG_LEVELUP_INFO(SMSG_LEVELUP_INFO::read_body(r, header_size)?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(MSG_MINIMAP_PING_Server::read_body(r, header_size)?)),
            0x01D6 => Ok(Self::SMSG_RESISTLOG(SMSG_RESISTLOG::read_body(r, header_size)?)),
            0x01D7 => Ok(Self::SMSG_ENCHANTMENTLOG(SMSG_ENCHANTMENTLOG::read_body(r, header_size)?)),
            0x01D9 => Ok(Self::SMSG_START_MIRROR_TIMER(SMSG_START_MIRROR_TIMER::read_body(r, header_size)?)),
            0x01DA => Ok(Self::SMSG_PAUSE_MIRROR_TIMER(SMSG_PAUSE_MIRROR_TIMER::read_body(r, header_size)?)),
            0x01DB => Ok(Self::SMSG_STOP_MIRROR_TIMER(SMSG_STOP_MIRROR_TIMER::read_body(r, header_size)?)),
            0x01DD => Ok(Self::SMSG_PONG(SMSG_PONG::read_body(r, header_size)?)),
            0x01DE => Ok(Self::SMSG_CLEAR_COOLDOWN(SMSG_CLEAR_COOLDOWN::read_body(r, header_size)?)),
            0x01DF => Ok(Self::SMSG_GAMEOBJECT_PAGETEXT(SMSG_GAMEOBJECT_PAGETEXT::read_body(r, header_size)?)),
            0x01E2 => Ok(Self::SMSG_SPELL_DELAYED(SMSG_SPELL_DELAYED::read_body(r, header_size)?)),
            0x01EA => Ok(Self::SMSG_ITEM_TIME_UPDATE(SMSG_ITEM_TIME_UPDATE::read_body(r, header_size)?)),
            0x01EB => Ok(Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(SMSG_ITEM_ENCHANT_TIME_UPDATE::read_body(r, header_size)?)),
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE::read_body(r, header_size)?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE::read_body(r, header_size)?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Server::read_body(r, header_size)?)),
            0x01F3 => Ok(Self::SMSG_PLAY_SPELL_VISUAL(SMSG_PLAY_SPELL_VISUAL::read_body(r, header_size)?)),
            0x01F5 => Ok(Self::SMSG_PARTYKILLLOG(SMSG_PARTYKILLLOG::read_body(r, header_size)?)),
            0x01F7 => Ok(Self::SMSG_PLAY_SPELL_IMPACT(SMSG_PLAY_SPELL_IMPACT::read_body(r, header_size)?)),
            0x01F8 => Ok(Self::SMSG_EXPLORATION_EXPERIENCE(SMSG_EXPLORATION_EXPERIENCE::read_body(r, header_size)?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Server::read_body(r, header_size)?)),
            0x01FC => Ok(Self::SMSG_ENVIRONMENTALDAMAGELOG(SMSG_ENVIRONMENTALDAMAGELOG::read_body(r, header_size)?)),
            0x01FF => Ok(Self::MSG_LOOKING_FOR_GROUP(MSG_LOOKING_FOR_GROUP_Server::read_body(r, header_size)?)),
            0x0203 => Ok(Self::SMSG_REMOVED_SPELL(SMSG_REMOVED_SPELL::read_body(r, header_size)?)),
            0x0206 => Ok(Self::SMSG_GMTICKET_CREATE(SMSG_GMTICKET_CREATE::read_body(r, header_size)?)),
            0x0208 => Ok(Self::SMSG_GMTICKET_UPDATETEXT(SMSG_GMTICKET_UPDATETEXT::read_body(r, header_size)?)),
            0x0209 => Ok(Self::SMSG_ACCOUNT_DATA_TIMES(SMSG_ACCOUNT_DATA_TIMES::read_body(r, header_size)?)),
            0x0212 => Ok(Self::SMSG_GMTICKET_GETTICKET(SMSG_GMTICKET_GETTICKET::read_body(r, header_size)?)),
            0x0214 => Ok(Self::SMSG_GAMEOBJECT_SPAWN_ANIM(SMSG_GAMEOBJECT_SPAWN_ANIM::read_body(r, header_size)?)),
            0x0215 => Ok(Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(SMSG_GAMEOBJECT_DESPAWN_ANIM::read_body(r, header_size)?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Server::read_body(r, header_size)?)),
            0x0218 => Ok(Self::SMSG_GMTICKET_DELETETICKET(SMSG_GMTICKET_DELETETICKET::read_body(r, header_size)?)),
            0x0219 => Ok(Self::SMSG_CHAT_WRONG_FACTION(SMSG_CHAT_WRONG_FACTION::read_body(r, header_size)?)),
            0x021B => Ok(Self::SMSG_GMTICKET_SYSTEMSTATUS(SMSG_GMTICKET_SYSTEMSTATUS::read_body(r, header_size)?)),
            0x021E => Ok(Self::SMSG_SET_REST_START(SMSG_SET_REST_START::read_body(r, header_size)?)),
            0x0222 => Ok(Self::SMSG_SPIRIT_HEALER_CONFIRM(SMSG_SPIRIT_HEALER_CONFIRM::read_body(r, header_size)?)),
            0x0224 => Ok(Self::SMSG_GOSSIP_POI(SMSG_GOSSIP_POI::read_body(r, header_size)?)),
            0x0236 => Ok(Self::SMSG_LOGIN_VERIFY_WORLD(SMSG_LOGIN_VERIFY_WORLD::read_body(r, header_size)?)),
            0x023B => Ok(Self::SMSG_MAIL_LIST_RESULT(SMSG_MAIL_LIST_RESULT::read_body(r, header_size)?)),
            0x023D => Ok(Self::SMSG_BATTLEFIELD_LIST(SMSG_BATTLEFIELD_LIST::read_body(r, header_size)?)),
            0x0244 => Ok(Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(SMSG_ITEM_TEXT_QUERY_RESPONSE::read_body(r, header_size)?)),
            0x024B => Ok(Self::SMSG_SPELLLOGMISS(SMSG_SPELLLOGMISS::read_body(r, header_size)?)),
            0x024C => Ok(Self::SMSG_SPELLLOGEXECUTE(SMSG_SPELLLOGEXECUTE::read_body(r, header_size)?)),
            0x024E => Ok(Self::SMSG_PERIODICAURALOG(SMSG_PERIODICAURALOG::read_body(r, header_size)?)),
            0x024F => Ok(Self::SMSG_SPELLDAMAGESHIELD(SMSG_SPELLDAMAGESHIELD::read_body(r, header_size)?)),
            0x0250 => Ok(Self::SMSG_SPELLNONMELEEDAMAGELOG(SMSG_SPELLNONMELEEDAMAGELOG::read_body(r, header_size)?)),
            0x0254 => Ok(Self::SMSG_ZONE_UNDER_ATTACK(SMSG_ZONE_UNDER_ATTACK::read_body(r, header_size)?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Server::read_body(r, header_size)?)),
            0x025C => Ok(Self::SMSG_AUCTION_LIST_RESULT(SMSG_AUCTION_LIST_RESULT::read_body(r, header_size)?)),
            0x025D => Ok(Self::SMSG_AUCTION_OWNER_LIST_RESULT(SMSG_AUCTION_OWNER_LIST_RESULT::read_body(r, header_size)?)),
            0x025E => Ok(Self::SMSG_AUCTION_BIDDER_NOTIFICATION(SMSG_AUCTION_BIDDER_NOTIFICATION::read_body(r, header_size)?)),
            0x025F => Ok(Self::SMSG_AUCTION_OWNER_NOTIFICATION(SMSG_AUCTION_OWNER_NOTIFICATION::read_body(r, header_size)?)),
            0x0260 => Ok(Self::SMSG_PROCRESIST(SMSG_PROCRESIST::read_body(r, header_size)?)),
            0x0262 => Ok(Self::SMSG_DISPEL_FAILED(SMSG_DISPEL_FAILED::read_body(r, header_size)?)),
            0x0263 => Ok(Self::SMSG_SPELLORDAMAGE_IMMUNE(SMSG_SPELLORDAMAGE_IMMUNE::read_body(r, header_size)?)),
            0x0265 => Ok(Self::SMSG_AUCTION_BIDDER_LIST_RESULT(SMSG_AUCTION_BIDDER_LIST_RESULT::read_body(r, header_size)?)),
            0x0266 => Ok(Self::SMSG_SET_FLAT_SPELL_MODIFIER(SMSG_SET_FLAT_SPELL_MODIFIER::read_body(r, header_size)?)),
            0x0267 => Ok(Self::SMSG_SET_PCT_SPELL_MODIFIER(SMSG_SET_PCT_SPELL_MODIFIER::read_body(r, header_size)?)),
            0x0269 => Ok(Self::SMSG_CORPSE_RECLAIM_DELAY(SMSG_CORPSE_RECLAIM_DELAY::read_body(r, header_size)?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Server::read_body(r, header_size)?)),
            0x0273 => Ok(Self::SMSG_STABLE_RESULT(SMSG_STABLE_RESULT::read_body(r, header_size)?)),
            0x0277 => Ok(Self::SMSG_PLAY_MUSIC(SMSG_PLAY_MUSIC::read_body(r, header_size)?)),
            0x0278 => Ok(Self::SMSG_PLAY_OBJECT_SOUND(SMSG_PLAY_OBJECT_SOUND::read_body(r, header_size)?)),
            0x027B => Ok(Self::SMSG_SPELLDISPELLOG(SMSG_SPELLDISPELLOG::read_body(r, header_size)?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Server::read_body(r, header_size)?)),
            0x0285 => Ok(Self::SMSG_RECEIVED_MAIL(SMSG_RECEIVED_MAIL::read_body(r, header_size)?)),
            0x0286 => Ok(Self::SMSG_RAID_GROUP_ONLY(SMSG_RAID_GROUP_ONLY::read_body(r, header_size)?)),
            0x028C => Ok(Self::SMSG_PVP_CREDIT(SMSG_PVP_CREDIT::read_body(r, header_size)?)),
            0x028D => Ok(Self::SMSG_AUCTION_REMOVED_NOTIFICATION(SMSG_AUCTION_REMOVED_NOTIFICATION::read_body(r, header_size)?)),
            0x0291 => Ok(Self::SMSG_SERVER_MESSAGE(SMSG_SERVER_MESSAGE::read_body(r, header_size)?)),
            0x0295 => Ok(Self::SMSG_MEETINGSTONE_SETQUEUE(SMSG_MEETINGSTONE_SETQUEUE::read_body(r, header_size)?)),
            0x0297 => Ok(Self::SMSG_MEETINGSTONE_COMPLETE(SMSG_MEETINGSTONE_COMPLETE::read_body(r, header_size)?)),
            0x0298 => Ok(Self::SMSG_MEETINGSTONE_IN_PROGRESS(SMSG_MEETINGSTONE_IN_PROGRESS::read_body(r, header_size)?)),
            0x0299 => Ok(Self::SMSG_MEETINGSTONE_MEMBER_ADDED(SMSG_MEETINGSTONE_MEMBER_ADDED::read_body(r, header_size)?)),
            0x029C => Ok(Self::SMSG_CANCEL_AUTO_REPEAT(SMSG_CANCEL_AUTO_REPEAT::read_body(r, header_size)?)),
            0x029D => Ok(Self::SMSG_STANDSTATE_UPDATE(SMSG_STANDSTATE_UPDATE::read_body(r, header_size)?)),
            0x029E => Ok(Self::SMSG_LOOT_ALL_PASSED(SMSG_LOOT_ALL_PASSED::read_body(r, header_size)?)),
            0x029F => Ok(Self::SMSG_LOOT_ROLL_WON(SMSG_LOOT_ROLL_WON::read_body(r, header_size)?)),
            0x02A1 => Ok(Self::SMSG_LOOT_START_ROLL(SMSG_LOOT_START_ROLL::read_body(r, header_size)?)),
            0x02A2 => Ok(Self::SMSG_LOOT_ROLL(SMSG_LOOT_ROLL::read_body(r, header_size)?)),
            0x02A4 => Ok(Self::SMSG_LOOT_MASTER_LIST(SMSG_LOOT_MASTER_LIST::read_body(r, header_size)?)),
            0x02A5 => Ok(Self::SMSG_SET_FORCED_REACTIONS(SMSG_SET_FORCED_REACTIONS::read_body(r, header_size)?)),
            0x02A6 => Ok(Self::SMSG_SPELL_FAILED_OTHER(SMSG_SPELL_FAILED_OTHER::read_body(r, header_size)?)),
            0x02A7 => Ok(Self::SMSG_GAMEOBJECT_RESET_STATE(SMSG_GAMEOBJECT_RESET_STATE::read_body(r, header_size)?)),
            0x02A9 => Ok(Self::SMSG_CHAT_PLAYER_NOT_FOUND(SMSG_CHAT_PLAYER_NOT_FOUND::read_body(r, header_size)?)),
            0x02AA => Ok(Self::MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Server::read_body(r, header_size)?)),
            0x02AB => Ok(Self::SMSG_SUMMON_REQUEST(SMSG_SUMMON_REQUEST::read_body(r, header_size)?)),
            0x02AE => Ok(Self::SMSG_MONSTER_MOVE_TRANSPORT(SMSG_MONSTER_MOVE_TRANSPORT::read_body(r, header_size)?)),
            0x02AF => Ok(Self::SMSG_PET_BROKEN(SMSG_PET_BROKEN::read_body(r, header_size)?)),
            0x02B4 => Ok(Self::SMSG_FEIGN_DEATH_RESISTED(SMSG_FEIGN_DEATH_RESISTED::read_body(r, header_size)?)),
            0x02B7 => Ok(Self::SMSG_DUEL_COUNTDOWN(SMSG_DUEL_COUNTDOWN::read_body(r, header_size)?)),
            0x02B8 => Ok(Self::SMSG_AREA_TRIGGER_MESSAGE(SMSG_AREA_TRIGGER_MESSAGE::read_body(r, header_size)?)),
            0x02BB => Ok(Self::SMSG_MEETINGSTONE_JOINFAILED(SMSG_MEETINGSTONE_JOINFAILED::read_body(r, header_size)?)),
            0x02BC => Ok(Self::SMSG_PLAYER_SKINNED(SMSG_PLAYER_SKINNED::read_body(r, header_size)?)),
            0x02BD => Ok(Self::SMSG_DURABILITY_DAMAGE_DEATH(SMSG_DURABILITY_DAMAGE_DEATH::read_body(r, header_size)?)),
            0x02C2 => Ok(Self::SMSG_INIT_WORLD_STATES(SMSG_INIT_WORLD_STATES::read_body(r, header_size)?)),
            0x02C3 => Ok(Self::SMSG_UPDATE_WORLD_STATE(SMSG_UPDATE_WORLD_STATE::read_body(r, header_size)?)),
            0x02C5 => Ok(Self::SMSG_ITEM_NAME_QUERY_RESPONSE(SMSG_ITEM_NAME_QUERY_RESPONSE::read_body(r, header_size)?)),
            0x02C6 => Ok(Self::SMSG_PET_ACTION_FEEDBACK(SMSG_PET_ACTION_FEEDBACK::read_body(r, header_size)?)),
            0x02C8 => Ok(Self::SMSG_CHAR_RENAME(SMSG_CHAR_RENAME::read_body(r, header_size)?)),
            0x02CB => Ok(Self::SMSG_INSTANCE_SAVE_CREATED(SMSG_INSTANCE_SAVE_CREATED::read_body(r, header_size)?)),
            0x02CC => Ok(Self::SMSG_RAID_INSTANCE_INFO(SMSG_RAID_INSTANCE_INFO::read_body(r, header_size)?)),
            0x02D2 => Ok(Self::SMSG_PLAY_SOUND(SMSG_PLAY_SOUND::read_body(r, header_size)?)),
            0x02D4 => Ok(Self::SMSG_BATTLEFIELD_STATUS(SMSG_BATTLEFIELD_STATUS::read_body(r, header_size)?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Server::read_body(r, header_size)?)),
            0x02DA => Ok(Self::SMSG_FORCE_WALK_SPEED_CHANGE(SMSG_FORCE_WALK_SPEED_CHANGE::read_body(r, header_size)?)),
            0x02DC => Ok(Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(SMSG_FORCE_SWIM_BACK_SPEED_CHANGE::read_body(r, header_size)?)),
            0x02DE => Ok(Self::SMSG_FORCE_TURN_RATE_CHANGE(SMSG_FORCE_TURN_RATE_CHANGE::read_body(r, header_size)?)),
            0x02E0 => Ok(Self::MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_Server::read_body(r, header_size)?)),
            0x02E4 => Ok(Self::SMSG_AREA_SPIRIT_HEALER_TIME(SMSG_AREA_SPIRIT_HEALER_TIME::read_body(r, header_size)?)),
            0x02E8 => Ok(Self::SMSG_GROUP_JOINED_BATTLEGROUND(SMSG_GROUP_JOINED_BATTLEGROUND::read_body(r, header_size)?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Server::read_body(r, header_size)?)),
            0x02EB => Ok(Self::SMSG_BINDER_CONFIRM(SMSG_BINDER_CONFIRM::read_body(r, header_size)?)),
            0x02EC => Ok(Self::SMSG_BATTLEGROUND_PLAYER_JOINED(SMSG_BATTLEGROUND_PLAYER_JOINED::read_body(r, header_size)?)),
            0x02ED => Ok(Self::SMSG_BATTLEGROUND_PLAYER_LEFT(SMSG_BATTLEGROUND_PLAYER_LEFT::read_body(r, header_size)?)),
            0x02F1 => Ok(Self::SMSG_PET_UNLEARN_CONFIRM(SMSG_PET_UNLEARN_CONFIRM::read_body(r, header_size)?)),
            0x02F2 => Ok(Self::SMSG_PARTY_MEMBER_STATS_FULL(SMSG_PARTY_MEMBER_STATS_FULL::read_body(r, header_size)?)),
            0x02F4 => Ok(Self::SMSG_WEATHER(SMSG_WEATHER::read_body(r, header_size)?)),
            0x02FA => Ok(Self::SMSG_RAID_INSTANCE_MESSAGE(SMSG_RAID_INSTANCE_MESSAGE::read_body(r, header_size)?)),
            0x02FD => Ok(Self::SMSG_CHAT_RESTRICTED(SMSG_CHAT_RESTRICTED::read_body(r, header_size)?)),
            0x02FE => Ok(Self::SMSG_SPLINE_SET_RUN_SPEED(SMSG_SPLINE_SET_RUN_SPEED::read_body(r, header_size)?)),
            0x02FF => Ok(Self::SMSG_SPLINE_SET_RUN_BACK_SPEED(SMSG_SPLINE_SET_RUN_BACK_SPEED::read_body(r, header_size)?)),
            0x0300 => Ok(Self::SMSG_SPLINE_SET_SWIM_SPEED(SMSG_SPLINE_SET_SWIM_SPEED::read_body(r, header_size)?)),
            0x0301 => Ok(Self::SMSG_SPLINE_SET_WALK_SPEED(SMSG_SPLINE_SET_WALK_SPEED::read_body(r, header_size)?)),
            0x0302 => Ok(Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED(SMSG_SPLINE_SET_SWIM_BACK_SPEED::read_body(r, header_size)?)),
            0x0303 => Ok(Self::SMSG_SPLINE_SET_TURN_RATE(SMSG_SPLINE_SET_TURN_RATE::read_body(r, header_size)?)),
            0x0304 => Ok(Self::SMSG_SPLINE_MOVE_UNROOT(SMSG_SPLINE_MOVE_UNROOT::read_body(r, header_size)?)),
            0x0305 => Ok(Self::SMSG_SPLINE_MOVE_FEATHER_FALL(SMSG_SPLINE_MOVE_FEATHER_FALL::read_body(r, header_size)?)),
            0x0306 => Ok(Self::SMSG_SPLINE_MOVE_NORMAL_FALL(SMSG_SPLINE_MOVE_NORMAL_FALL::read_body(r, header_size)?)),
            0x0307 => Ok(Self::SMSG_SPLINE_MOVE_SET_HOVER(SMSG_SPLINE_MOVE_SET_HOVER::read_body(r, header_size)?)),
            0x0308 => Ok(Self::SMSG_SPLINE_MOVE_UNSET_HOVER(SMSG_SPLINE_MOVE_UNSET_HOVER::read_body(r, header_size)?)),
            0x0309 => Ok(Self::SMSG_SPLINE_MOVE_WATER_WALK(SMSG_SPLINE_MOVE_WATER_WALK::read_body(r, header_size)?)),
            0x030A => Ok(Self::SMSG_SPLINE_MOVE_LAND_WALK(SMSG_SPLINE_MOVE_LAND_WALK::read_body(r, header_size)?)),
            0x030B => Ok(Self::SMSG_SPLINE_MOVE_START_SWIM(SMSG_SPLINE_MOVE_START_SWIM::read_body(r, header_size)?)),
            0x030C => Ok(Self::SMSG_SPLINE_MOVE_STOP_SWIM(SMSG_SPLINE_MOVE_STOP_SWIM::read_body(r, header_size)?)),
            0x030D => Ok(Self::SMSG_SPLINE_MOVE_SET_RUN_MODE(SMSG_SPLINE_MOVE_SET_RUN_MODE::read_body(r, header_size)?)),
            0x030E => Ok(Self::SMSG_SPLINE_MOVE_SET_WALK_MODE(SMSG_SPLINE_MOVE_SET_WALK_MODE::read_body(r, header_size)?)),
            0x031A => Ok(Self::SMSG_SPLINE_MOVE_ROOT(SMSG_SPLINE_MOVE_ROOT::read_body(r, header_size)?)),
            0x031C => Ok(Self::SMSG_INVALIDATE_PLAYER(SMSG_INVALIDATE_PLAYER::read_body(r, header_size)?)),
            0x031E => Ok(Self::SMSG_INSTANCE_RESET(SMSG_INSTANCE_RESET::read_body(r, header_size)?)),
            0x031F => Ok(Self::SMSG_INSTANCE_RESET_FAILED(SMSG_INSTANCE_RESET_FAILED::read_body(r, header_size)?)),
            0x0320 => Ok(Self::SMSG_UPDATE_LAST_INSTANCE(SMSG_UPDATE_LAST_INSTANCE::read_body(r, header_size)?)),
            0x0321 => Ok(Self::MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Server::read_body(r, header_size)?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Server::read_body(r, header_size)?)),
            0x0324 => Ok(Self::SMSG_PET_ACTION_SOUND(SMSG_PET_ACTION_SOUND::read_body(r, header_size)?)),
            0x0325 => Ok(Self::SMSG_PET_DISMISS_SOUND(SMSG_PET_DISMISS_SOUND::read_body(r, header_size)?)),
            0x0328 => Ok(Self::SMSG_GM_TICKET_STATUS_UPDATE(SMSG_GM_TICKET_STATUS_UPDATE::read_body(r, header_size)?)),
            0x032B => Ok(Self::SMSG_UPDATE_INSTANCE_OWNERSHIP(SMSG_UPDATE_INSTANCE_OWNERSHIP::read_body(r, header_size)?)),
            0x032F => Ok(Self::SMSG_SPELLINSTAKILLLOG(SMSG_SPELLINSTAKILLLOG::read_body(r, header_size)?)),
            0x0330 => Ok(Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS(SMSG_SPELL_UPDATE_CHAIN_TARGETS::read_body(r, header_size)?)),
            0x0332 => Ok(Self::SMSG_EXPECTED_SPAM_RECORDS(SMSG_EXPECTED_SPAM_RECORDS::read_body(r, header_size)?)),
            0x033B => Ok(Self::SMSG_DEFENSE_MESSAGE(SMSG_DEFENSE_MESSAGE::read_body(r, header_size)?)),
            _ => Err(ServerOpcodeMessageError::InvalidOpcode(header.opcode)),
        }
    }


    #[cfg(feature = "tokio")]
    pub async fn tokio_read_unencrypted<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let size = (crate::util::tokio_read_u16_be(r).await? - 2) as u32;
        let opcode = crate::util::tokio_read_u16_le(r).await?;
        match opcode {
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD::tokio_read_body(r, size).await?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD::tokio_read_body(r, size).await?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(MSG_MOVE_STOP::tokio_read_body(r, size).await?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT::tokio_read_body(r, size).await?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT::tokio_read_body(r, size).await?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE::tokio_read_body(r, size).await?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(MSG_MOVE_JUMP::tokio_read_body(r, size).await?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT::tokio_read_body(r, size).await?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT::tokio_read_body(r, size).await?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN::tokio_read_body(r, size).await?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP::tokio_read_body(r, size).await?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN::tokio_read_body(r, size).await?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH::tokio_read_body(r, size).await?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE::tokio_read_body(r, size).await?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE::tokio_read_body(r, size).await?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK::tokio_read_body(r, size).await?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND::tokio_read_body(r, size).await?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM::tokio_read_body(r, size).await?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM::tokio_read_body(r, size).await?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING::tokio_read_body(r, size).await?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH::tokio_read_body(r, size).await?)),
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK::tokio_read_body(r, size).await?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT::tokio_read_body(r, size).await?)),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(MSG_PETITION_DECLINE::tokio_read_body(r, size).await?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE::tokio_read_body(r, size).await?)),
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT::tokio_read_body(r, size).await?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(MSG_PETITION_RENAME::tokio_read_body(r, size).await?)),
            0x003A => Ok(Self::SMSG_CHAR_CREATE(SMSG_CHAR_CREATE::tokio_read_body(r, size).await?)),
            0x003B => Ok(Self::SMSG_CHAR_ENUM(SMSG_CHAR_ENUM::tokio_read_body(r, size).await?)),
            0x003C => Ok(Self::SMSG_CHAR_DELETE(SMSG_CHAR_DELETE::tokio_read_body(r, size).await?)),
            0x003E => Ok(Self::SMSG_NEW_WORLD(SMSG_NEW_WORLD::tokio_read_body(r, size).await?)),
            0x003F => Ok(Self::SMSG_TRANSFER_PENDING(SMSG_TRANSFER_PENDING::tokio_read_body(r, size).await?)),
            0x0040 => Ok(Self::SMSG_TRANSFER_ABORTED(SMSG_TRANSFER_ABORTED::tokio_read_body(r, size).await?)),
            0x0041 => Ok(Self::SMSG_CHARACTER_LOGIN_FAILED(SMSG_CHARACTER_LOGIN_FAILED::tokio_read_body(r, size).await?)),
            0x0042 => Ok(Self::SMSG_LOGIN_SETTIMESPEED(SMSG_LOGIN_SETTIMESPEED::tokio_read_body(r, size).await?)),
            0x004C => Ok(Self::SMSG_LOGOUT_RESPONSE(SMSG_LOGOUT_RESPONSE::tokio_read_body(r, size).await?)),
            0x004D => Ok(Self::SMSG_LOGOUT_COMPLETE(SMSG_LOGOUT_COMPLETE::tokio_read_body(r, size).await?)),
            0x004F => Ok(Self::SMSG_LOGOUT_CANCEL_ACK(SMSG_LOGOUT_CANCEL_ACK::tokio_read_body(r, size).await?)),
            0x0051 => Ok(Self::SMSG_NAME_QUERY_RESPONSE(SMSG_NAME_QUERY_RESPONSE::tokio_read_body(r, size).await?)),
            0x0053 => Ok(Self::SMSG_PET_NAME_QUERY_RESPONSE(SMSG_PET_NAME_QUERY_RESPONSE::tokio_read_body(r, size).await?)),
            0x0055 => Ok(Self::SMSG_GUILD_QUERY_RESPONSE(SMSG_GUILD_QUERY_RESPONSE::tokio_read_body(r, size).await?)),
            0x0058 => Ok(Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(SMSG_ITEM_QUERY_SINGLE_RESPONSE::tokio_read_body(r, size).await?)),
            0x005B => Ok(Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(SMSG_PAGE_TEXT_QUERY_RESPONSE::tokio_read_body(r, size).await?)),
            0x005D => Ok(Self::SMSG_QUEST_QUERY_RESPONSE(SMSG_QUEST_QUERY_RESPONSE::tokio_read_body(r, size).await?)),
            0x005F => Ok(Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(SMSG_GAMEOBJECT_QUERY_RESPONSE::tokio_read_body(r, size).await?)),
            0x0061 => Ok(Self::SMSG_CREATURE_QUERY_RESPONSE(SMSG_CREATURE_QUERY_RESPONSE::tokio_read_body(r, size).await?)),
            0x0063 => Ok(Self::SMSG_WHO(SMSG_WHO::tokio_read_body(r, size).await?)),
            0x0065 => Ok(Self::SMSG_WHOIS(SMSG_WHOIS::tokio_read_body(r, size).await?)),
            0x0067 => Ok(Self::SMSG_FRIEND_LIST(SMSG_FRIEND_LIST::tokio_read_body(r, size).await?)),
            0x0068 => Ok(Self::SMSG_FRIEND_STATUS(SMSG_FRIEND_STATUS::tokio_read_body(r, size).await?)),
            0x006B => Ok(Self::SMSG_IGNORE_LIST(SMSG_IGNORE_LIST::tokio_read_body(r, size).await?)),
            0x006F => Ok(Self::SMSG_GROUP_INVITE(SMSG_GROUP_INVITE::tokio_read_body(r, size).await?)),
            0x0074 => Ok(Self::SMSG_GROUP_DECLINE(SMSG_GROUP_DECLINE::tokio_read_body(r, size).await?)),
            0x0077 => Ok(Self::SMSG_GROUP_UNINVITE(SMSG_GROUP_UNINVITE::tokio_read_body(r, size).await?)),
            0x0079 => Ok(Self::SMSG_GROUP_SET_LEADER(SMSG_GROUP_SET_LEADER::tokio_read_body(r, size).await?)),
            0x007C => Ok(Self::SMSG_GROUP_DESTROYED(SMSG_GROUP_DESTROYED::tokio_read_body(r, size).await?)),
            0x007D => Ok(Self::SMSG_GROUP_LIST(SMSG_GROUP_LIST::tokio_read_body(r, size).await?)),
            0x007E => Ok(Self::SMSG_PARTY_MEMBER_STATS(SMSG_PARTY_MEMBER_STATS::tokio_read_body(r, size).await?)),
            0x007F => Ok(Self::SMSG_PARTY_COMMAND_RESULT(SMSG_PARTY_COMMAND_RESULT::tokio_read_body(r, size).await?)),
            0x0083 => Ok(Self::SMSG_GUILD_INVITE(SMSG_GUILD_INVITE::tokio_read_body(r, size).await?)),
            0x0088 => Ok(Self::SMSG_GUILD_INFO(SMSG_GUILD_INFO::tokio_read_body(r, size).await?)),
            0x008A => Ok(Self::SMSG_GUILD_ROSTER(SMSG_GUILD_ROSTER::tokio_read_body(r, size).await?)),
            0x0092 => Ok(Self::SMSG_GUILD_EVENT(SMSG_GUILD_EVENT::tokio_read_body(r, size).await?)),
            0x0093 => Ok(Self::SMSG_GUILD_COMMAND_RESULT(SMSG_GUILD_COMMAND_RESULT::tokio_read_body(r, size).await?)),
            0x0096 => Ok(Self::SMSG_MESSAGECHAT(SMSG_MESSAGECHAT::tokio_read_body(r, size).await?)),
            0x0099 => Ok(Self::SMSG_CHANNEL_NOTIFY(SMSG_CHANNEL_NOTIFY::tokio_read_body(r, size).await?)),
            0x009B => Ok(Self::SMSG_CHANNEL_LIST(SMSG_CHANNEL_LIST::tokio_read_body(r, size).await?)),
            0x00AA => Ok(Self::SMSG_DESTROY_OBJECT(SMSG_DESTROY_OBJECT::tokio_read_body(r, size).await?)),
            0x00AE => Ok(Self::SMSG_READ_ITEM_OK(SMSG_READ_ITEM_OK::tokio_read_body(r, size).await?)),
            0x00AF => Ok(Self::SMSG_READ_ITEM_FAILED(SMSG_READ_ITEM_FAILED::tokio_read_body(r, size).await?)),
            0x00B0 => Ok(Self::SMSG_ITEM_COOLDOWN(SMSG_ITEM_COOLDOWN::tokio_read_body(r, size).await?)),
            0x00B3 => Ok(Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(SMSG_GAMEOBJECT_CUSTOM_ANIM::tokio_read_body(r, size).await?)),
            0x00DD => Ok(Self::SMSG_MONSTER_MOVE(SMSG_MONSTER_MOVE::tokio_read_body(r, size).await?)),
            0x00DE => Ok(Self::SMSG_MOVE_WATER_WALK(SMSG_MOVE_WATER_WALK::tokio_read_body(r, size).await?)),
            0x00DF => Ok(Self::SMSG_MOVE_LAND_WALK(SMSG_MOVE_LAND_WALK::tokio_read_body(r, size).await?)),
            0x00E2 => Ok(Self::SMSG_FORCE_RUN_SPEED_CHANGE(SMSG_FORCE_RUN_SPEED_CHANGE::tokio_read_body(r, size).await?)),
            0x00E4 => Ok(Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(SMSG_FORCE_RUN_BACK_SPEED_CHANGE::tokio_read_body(r, size).await?)),
            0x00E6 => Ok(Self::SMSG_FORCE_SWIM_SPEED_CHANGE(SMSG_FORCE_SWIM_SPEED_CHANGE::tokio_read_body(r, size).await?)),
            0x00E8 => Ok(Self::SMSG_FORCE_MOVE_ROOT(SMSG_FORCE_MOVE_ROOT::tokio_read_body(r, size).await?)),
            0x00EA => Ok(Self::SMSG_FORCE_MOVE_UNROOT(SMSG_FORCE_MOVE_UNROOT::tokio_read_body(r, size).await?)),
            0x00EF => Ok(Self::SMSG_MOVE_KNOCK_BACK(SMSG_MOVE_KNOCK_BACK::tokio_read_body(r, size).await?)),
            0x00F2 => Ok(Self::SMSG_MOVE_FEATHER_FALL(SMSG_MOVE_FEATHER_FALL::tokio_read_body(r, size).await?)),
            0x00F3 => Ok(Self::SMSG_MOVE_NORMAL_FALL(SMSG_MOVE_NORMAL_FALL::tokio_read_body(r, size).await?)),
            0x00F4 => Ok(Self::SMSG_MOVE_SET_HOVER(SMSG_MOVE_SET_HOVER::tokio_read_body(r, size).await?)),
            0x00F5 => Ok(Self::SMSG_MOVE_UNSET_HOVER(SMSG_MOVE_UNSET_HOVER::tokio_read_body(r, size).await?)),
            0x00FA => Ok(Self::SMSG_TRIGGER_CINEMATIC(SMSG_TRIGGER_CINEMATIC::tokio_read_body(r, size).await?)),
            0x00FD => Ok(Self::SMSG_TUTORIAL_FLAGS(SMSG_TUTORIAL_FLAGS::tokio_read_body(r, size).await?)),
            0x0103 => Ok(Self::SMSG_EMOTE(SMSG_EMOTE::tokio_read_body(r, size).await?)),
            0x0105 => Ok(Self::SMSG_TEXT_EMOTE(SMSG_TEXT_EMOTE::tokio_read_body(r, size).await?)),
            0x0112 => Ok(Self::SMSG_INVENTORY_CHANGE_FAILURE(SMSG_INVENTORY_CHANGE_FAILURE::tokio_read_body(r, size).await?)),
            0x0113 => Ok(Self::SMSG_OPEN_CONTAINER(SMSG_OPEN_CONTAINER::tokio_read_body(r, size).await?)),
            0x0115 => Ok(Self::SMSG_INSPECT(SMSG_INSPECT::tokio_read_body(r, size).await?)),
            0x0120 => Ok(Self::SMSG_TRADE_STATUS(SMSG_TRADE_STATUS::tokio_read_body(r, size).await?)),
            0x0121 => Ok(Self::SMSG_TRADE_STATUS_EXTENDED(SMSG_TRADE_STATUS_EXTENDED::tokio_read_body(r, size).await?)),
            0x0122 => Ok(Self::SMSG_INITIALIZE_FACTIONS(SMSG_INITIALIZE_FACTIONS::tokio_read_body(r, size).await?)),
            0x0123 => Ok(Self::SMSG_SET_FACTION_VISIBLE(SMSG_SET_FACTION_VISIBLE::tokio_read_body(r, size).await?)),
            0x0124 => Ok(Self::SMSG_SET_FACTION_STANDING(SMSG_SET_FACTION_STANDING::tokio_read_body(r, size).await?)),
            0x0127 => Ok(Self::SMSG_SET_PROFICIENCY(SMSG_SET_PROFICIENCY::tokio_read_body(r, size).await?)),
            0x0129 => Ok(Self::SMSG_ACTION_BUTTONS(SMSG_ACTION_BUTTONS::tokio_read_body(r, size).await?)),
            0x012A => Ok(Self::SMSG_INITIAL_SPELLS(SMSG_INITIAL_SPELLS::tokio_read_body(r, size).await?)),
            0x012B => Ok(Self::SMSG_LEARNED_SPELL(SMSG_LEARNED_SPELL::tokio_read_body(r, size).await?)),
            0x012C => Ok(Self::SMSG_SUPERCEDED_SPELL(SMSG_SUPERCEDED_SPELL::tokio_read_body(r, size).await?)),
            0x0130 => Ok(Self::SMSG_CAST_RESULT(SMSG_CAST_RESULT::tokio_read_body(r, size).await?)),
            0x0131 => Ok(Self::SMSG_SPELL_START(SMSG_SPELL_START::tokio_read_body(r, size).await?)),
            0x0132 => Ok(Self::SMSG_SPELL_GO(SMSG_SPELL_GO::tokio_read_body(r, size).await?)),
            0x0133 => Ok(Self::SMSG_SPELL_FAILURE(SMSG_SPELL_FAILURE::tokio_read_body(r, size).await?)),
            0x0134 => Ok(Self::SMSG_SPELL_COOLDOWN(SMSG_SPELL_COOLDOWN::tokio_read_body(r, size).await?)),
            0x0135 => Ok(Self::SMSG_COOLDOWN_EVENT(SMSG_COOLDOWN_EVENT::tokio_read_body(r, size).await?)),
            0x0137 => Ok(Self::SMSG_UPDATE_AURA_DURATION(SMSG_UPDATE_AURA_DURATION::tokio_read_body(r, size).await?)),
            0x0138 => Ok(Self::SMSG_PET_CAST_FAILED(SMSG_PET_CAST_FAILED::tokio_read_body(r, size).await?)),
            0x013C => Ok(Self::SMSG_AI_REACTION(SMSG_AI_REACTION::tokio_read_body(r, size).await?)),
            0x0143 => Ok(Self::SMSG_ATTACKSTART(SMSG_ATTACKSTART::tokio_read_body(r, size).await?)),
            0x0144 => Ok(Self::SMSG_ATTACKSTOP(SMSG_ATTACKSTOP::tokio_read_body(r, size).await?)),
            0x0145 => Ok(Self::SMSG_ATTACKSWING_NOTINRANGE(SMSG_ATTACKSWING_NOTINRANGE::tokio_read_body(r, size).await?)),
            0x0146 => Ok(Self::SMSG_ATTACKSWING_BADFACING(SMSG_ATTACKSWING_BADFACING::tokio_read_body(r, size).await?)),
            0x0147 => Ok(Self::SMSG_ATTACKSWING_NOTSTANDING(SMSG_ATTACKSWING_NOTSTANDING::tokio_read_body(r, size).await?)),
            0x0148 => Ok(Self::SMSG_ATTACKSWING_DEADTARGET(SMSG_ATTACKSWING_DEADTARGET::tokio_read_body(r, size).await?)),
            0x0149 => Ok(Self::SMSG_ATTACKSWING_CANT_ATTACK(SMSG_ATTACKSWING_CANT_ATTACK::tokio_read_body(r, size).await?)),
            0x014A => Ok(Self::SMSG_ATTACKERSTATEUPDATE(SMSG_ATTACKERSTATEUPDATE::tokio_read_body(r, size).await?)),
            0x014E => Ok(Self::SMSG_CANCEL_COMBAT(SMSG_CANCEL_COMBAT::tokio_read_body(r, size).await?)),
            0x0150 => Ok(Self::SMSG_SPELLHEALLOG(SMSG_SPELLHEALLOG::tokio_read_body(r, size).await?)),
            0x0151 => Ok(Self::SMSG_SPELLENERGIZELOG(SMSG_SPELLENERGIZELOG::tokio_read_body(r, size).await?)),
            0x0155 => Ok(Self::SMSG_BINDPOINTUPDATE(SMSG_BINDPOINTUPDATE::tokio_read_body(r, size).await?)),
            0x0158 => Ok(Self::SMSG_PLAYERBOUND(SMSG_PLAYERBOUND::tokio_read_body(r, size).await?)),
            0x0159 => Ok(Self::SMSG_CLIENT_CONTROL_UPDATE(SMSG_CLIENT_CONTROL_UPDATE::tokio_read_body(r, size).await?)),
            0x015B => Ok(Self::SMSG_RESURRECT_REQUEST(SMSG_RESURRECT_REQUEST::tokio_read_body(r, size).await?)),
            0x0160 => Ok(Self::SMSG_LOOT_RESPONSE(SMSG_LOOT_RESPONSE::tokio_read_body(r, size).await?)),
            0x0161 => Ok(Self::SMSG_LOOT_RELEASE_RESPONSE(SMSG_LOOT_RELEASE_RESPONSE::tokio_read_body(r, size).await?)),
            0x0162 => Ok(Self::SMSG_LOOT_REMOVED(SMSG_LOOT_REMOVED::tokio_read_body(r, size).await?)),
            0x0163 => Ok(Self::SMSG_LOOT_MONEY_NOTIFY(SMSG_LOOT_MONEY_NOTIFY::tokio_read_body(r, size).await?)),
            0x0165 => Ok(Self::SMSG_LOOT_CLEAR_MONEY(SMSG_LOOT_CLEAR_MONEY::tokio_read_body(r, size).await?)),
            0x0166 => Ok(Self::SMSG_ITEM_PUSH_RESULT(SMSG_ITEM_PUSH_RESULT::tokio_read_body(r, size).await?)),
            0x0167 => Ok(Self::SMSG_DUEL_REQUESTED(SMSG_DUEL_REQUESTED::tokio_read_body(r, size).await?)),
            0x0168 => Ok(Self::SMSG_DUEL_OUTOFBOUNDS(SMSG_DUEL_OUTOFBOUNDS::tokio_read_body(r, size).await?)),
            0x0169 => Ok(Self::SMSG_DUEL_INBOUNDS(SMSG_DUEL_INBOUNDS::tokio_read_body(r, size).await?)),
            0x016A => Ok(Self::SMSG_DUEL_COMPLETE(SMSG_DUEL_COMPLETE::tokio_read_body(r, size).await?)),
            0x016B => Ok(Self::SMSG_DUEL_WINNER(SMSG_DUEL_WINNER::tokio_read_body(r, size).await?)),
            0x016E => Ok(Self::SMSG_MOUNTRESULT(SMSG_MOUNTRESULT::tokio_read_body(r, size).await?)),
            0x016F => Ok(Self::SMSG_DISMOUNTRESULT(SMSG_DISMOUNTRESULT::tokio_read_body(r, size).await?)),
            0x0172 => Ok(Self::SMSG_MOUNTSPECIAL_ANIM(SMSG_MOUNTSPECIAL_ANIM::tokio_read_body(r, size).await?)),
            0x0173 => Ok(Self::SMSG_PET_TAME_FAILURE(SMSG_PET_TAME_FAILURE::tokio_read_body(r, size).await?)),
            0x0178 => Ok(Self::SMSG_PET_NAME_INVALID(SMSG_PET_NAME_INVALID::tokio_read_body(r, size).await?)),
            0x0179 => Ok(Self::SMSG_PET_SPELLS(SMSG_PET_SPELLS::tokio_read_body(r, size).await?)),
            0x017A => Ok(Self::SMSG_PET_MODE(SMSG_PET_MODE::tokio_read_body(r, size).await?)),
            0x017D => Ok(Self::SMSG_GOSSIP_MESSAGE(SMSG_GOSSIP_MESSAGE::tokio_read_body(r, size).await?)),
            0x017E => Ok(Self::SMSG_GOSSIP_COMPLETE(SMSG_GOSSIP_COMPLETE::tokio_read_body(r, size).await?)),
            0x0180 => Ok(Self::SMSG_NPC_TEXT_UPDATE(SMSG_NPC_TEXT_UPDATE::tokio_read_body(r, size).await?)),
            0x0183 => Ok(Self::SMSG_QUESTGIVER_STATUS(SMSG_QUESTGIVER_STATUS::tokio_read_body(r, size).await?)),
            0x0185 => Ok(Self::SMSG_QUESTGIVER_QUEST_LIST(SMSG_QUESTGIVER_QUEST_LIST::tokio_read_body(r, size).await?)),
            0x0188 => Ok(Self::SMSG_QUESTGIVER_QUEST_DETAILS(SMSG_QUESTGIVER_QUEST_DETAILS::tokio_read_body(r, size).await?)),
            0x018B => Ok(Self::SMSG_QUESTGIVER_REQUEST_ITEMS(SMSG_QUESTGIVER_REQUEST_ITEMS::tokio_read_body(r, size).await?)),
            0x018D => Ok(Self::SMSG_QUESTGIVER_OFFER_REWARD(SMSG_QUESTGIVER_OFFER_REWARD::tokio_read_body(r, size).await?)),
            0x018F => Ok(Self::SMSG_QUESTGIVER_QUEST_INVALID(SMSG_QUESTGIVER_QUEST_INVALID::tokio_read_body(r, size).await?)),
            0x0191 => Ok(Self::SMSG_QUESTGIVER_QUEST_COMPLETE(SMSG_QUESTGIVER_QUEST_COMPLETE::tokio_read_body(r, size).await?)),
            0x0192 => Ok(Self::SMSG_QUESTGIVER_QUEST_FAILED(SMSG_QUESTGIVER_QUEST_FAILED::tokio_read_body(r, size).await?)),
            0x0195 => Ok(Self::SMSG_QUESTLOG_FULL(SMSG_QUESTLOG_FULL::tokio_read_body(r, size).await?)),
            0x0196 => Ok(Self::SMSG_QUESTUPDATE_FAILED(SMSG_QUESTUPDATE_FAILED::tokio_read_body(r, size).await?)),
            0x0197 => Ok(Self::SMSG_QUESTUPDATE_FAILEDTIMER(SMSG_QUESTUPDATE_FAILEDTIMER::tokio_read_body(r, size).await?)),
            0x0198 => Ok(Self::SMSG_QUESTUPDATE_COMPLETE(SMSG_QUESTUPDATE_COMPLETE::tokio_read_body(r, size).await?)),
            0x0199 => Ok(Self::SMSG_QUESTUPDATE_ADD_KILL(SMSG_QUESTUPDATE_ADD_KILL::tokio_read_body(r, size).await?)),
            0x019A => Ok(Self::SMSG_QUESTUPDATE_ADD_ITEM(SMSG_QUESTUPDATE_ADD_ITEM::tokio_read_body(r, size).await?)),
            0x019C => Ok(Self::SMSG_QUEST_CONFIRM_ACCEPT(SMSG_QUEST_CONFIRM_ACCEPT::tokio_read_body(r, size).await?)),
            0x019F => Ok(Self::SMSG_LIST_INVENTORY(SMSG_LIST_INVENTORY::tokio_read_body(r, size).await?)),
            0x01A1 => Ok(Self::SMSG_SELL_ITEM(SMSG_SELL_ITEM::tokio_read_body(r, size).await?)),
            0x01A4 => Ok(Self::SMSG_BUY_ITEM(SMSG_BUY_ITEM::tokio_read_body(r, size).await?)),
            0x01A5 => Ok(Self::SMSG_BUY_FAILED(SMSG_BUY_FAILED::tokio_read_body(r, size).await?)),
            0x01A9 => Ok(Self::SMSG_SHOWTAXINODES(SMSG_SHOWTAXINODES::tokio_read_body(r, size).await?)),
            0x01AB => Ok(Self::SMSG_TAXINODE_STATUS(SMSG_TAXINODE_STATUS::tokio_read_body(r, size).await?)),
            0x01AE => Ok(Self::SMSG_ACTIVATETAXIREPLY(SMSG_ACTIVATETAXIREPLY::tokio_read_body(r, size).await?)),
            0x01AF => Ok(Self::SMSG_NEW_TAXI_PATH(SMSG_NEW_TAXI_PATH::tokio_read_body(r, size).await?)),
            0x01B1 => Ok(Self::SMSG_TRAINER_LIST(SMSG_TRAINER_LIST::tokio_read_body(r, size).await?)),
            0x01B3 => Ok(Self::SMSG_TRAINER_BUY_SUCCEEDED(SMSG_TRAINER_BUY_SUCCEEDED::tokio_read_body(r, size).await?)),
            0x01B4 => Ok(Self::SMSG_TRAINER_BUY_FAILED(SMSG_TRAINER_BUY_FAILED::tokio_read_body(r, size).await?)),
            0x01B8 => Ok(Self::SMSG_SHOW_BANK(SMSG_SHOW_BANK::tokio_read_body(r, size).await?)),
            0x01BA => Ok(Self::SMSG_BUY_BANK_SLOT_RESULT(SMSG_BUY_BANK_SLOT_RESULT::tokio_read_body(r, size).await?)),
            0x01BC => Ok(Self::SMSG_PETITION_SHOWLIST(SMSG_PETITION_SHOWLIST::tokio_read_body(r, size).await?)),
            0x01BF => Ok(Self::SMSG_PETITION_SHOW_SIGNATURES(SMSG_PETITION_SHOW_SIGNATURES::tokio_read_body(r, size).await?)),
            0x01C1 => Ok(Self::SMSG_PETITION_SIGN_RESULTS(SMSG_PETITION_SIGN_RESULTS::tokio_read_body(r, size).await?)),
            0x01C5 => Ok(Self::SMSG_TURN_IN_PETITION_RESULTS(SMSG_TURN_IN_PETITION_RESULTS::tokio_read_body(r, size).await?)),
            0x01C7 => Ok(Self::SMSG_PETITION_QUERY_RESPONSE(SMSG_PETITION_QUERY_RESPONSE::tokio_read_body(r, size).await?)),
            0x01C8 => Ok(Self::SMSG_FISH_NOT_HOOKED(SMSG_FISH_NOT_HOOKED::tokio_read_body(r, size).await?)),
            0x01C9 => Ok(Self::SMSG_FISH_ESCAPED(SMSG_FISH_ESCAPED::tokio_read_body(r, size).await?)),
            0x01CB => Ok(Self::SMSG_NOTIFICATION(SMSG_NOTIFICATION::tokio_read_body(r, size).await?)),
            0x01CD => Ok(Self::SMSG_PLAYED_TIME(SMSG_PLAYED_TIME::tokio_read_body(r, size).await?)),
            0x01CF => Ok(Self::SMSG_QUERY_TIME_RESPONSE(SMSG_QUERY_TIME_RESPONSE::tokio_read_body(r, size).await?)),
            0x01D0 => Ok(Self::SMSG_LOG_XPGAIN(SMSG_LOG_XPGAIN::tokio_read_body(r, size).await?)),
            0x01D4 => Ok(Self::SMSG_LEVELUP_INFO(SMSG_LEVELUP_INFO::tokio_read_body(r, size).await?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(MSG_MINIMAP_PING_Server::tokio_read_body(r, size).await?)),
            0x01D6 => Ok(Self::SMSG_RESISTLOG(SMSG_RESISTLOG::tokio_read_body(r, size).await?)),
            0x01D7 => Ok(Self::SMSG_ENCHANTMENTLOG(SMSG_ENCHANTMENTLOG::tokio_read_body(r, size).await?)),
            0x01D9 => Ok(Self::SMSG_START_MIRROR_TIMER(SMSG_START_MIRROR_TIMER::tokio_read_body(r, size).await?)),
            0x01DA => Ok(Self::SMSG_PAUSE_MIRROR_TIMER(SMSG_PAUSE_MIRROR_TIMER::tokio_read_body(r, size).await?)),
            0x01DB => Ok(Self::SMSG_STOP_MIRROR_TIMER(SMSG_STOP_MIRROR_TIMER::tokio_read_body(r, size).await?)),
            0x01DD => Ok(Self::SMSG_PONG(SMSG_PONG::tokio_read_body(r, size).await?)),
            0x01DE => Ok(Self::SMSG_CLEAR_COOLDOWN(SMSG_CLEAR_COOLDOWN::tokio_read_body(r, size).await?)),
            0x01DF => Ok(Self::SMSG_GAMEOBJECT_PAGETEXT(SMSG_GAMEOBJECT_PAGETEXT::tokio_read_body(r, size).await?)),
            0x01E2 => Ok(Self::SMSG_SPELL_DELAYED(SMSG_SPELL_DELAYED::tokio_read_body(r, size).await?)),
            0x01EA => Ok(Self::SMSG_ITEM_TIME_UPDATE(SMSG_ITEM_TIME_UPDATE::tokio_read_body(r, size).await?)),
            0x01EB => Ok(Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(SMSG_ITEM_ENCHANT_TIME_UPDATE::tokio_read_body(r, size).await?)),
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE::tokio_read_body(r, size).await?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE::tokio_read_body(r, size).await?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Server::tokio_read_body(r, size).await?)),
            0x01F3 => Ok(Self::SMSG_PLAY_SPELL_VISUAL(SMSG_PLAY_SPELL_VISUAL::tokio_read_body(r, size).await?)),
            0x01F5 => Ok(Self::SMSG_PARTYKILLLOG(SMSG_PARTYKILLLOG::tokio_read_body(r, size).await?)),
            0x01F7 => Ok(Self::SMSG_PLAY_SPELL_IMPACT(SMSG_PLAY_SPELL_IMPACT::tokio_read_body(r, size).await?)),
            0x01F8 => Ok(Self::SMSG_EXPLORATION_EXPERIENCE(SMSG_EXPLORATION_EXPERIENCE::tokio_read_body(r, size).await?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Server::tokio_read_body(r, size).await?)),
            0x01FC => Ok(Self::SMSG_ENVIRONMENTALDAMAGELOG(SMSG_ENVIRONMENTALDAMAGELOG::tokio_read_body(r, size).await?)),
            0x01FF => Ok(Self::MSG_LOOKING_FOR_GROUP(MSG_LOOKING_FOR_GROUP_Server::tokio_read_body(r, size).await?)),
            0x0203 => Ok(Self::SMSG_REMOVED_SPELL(SMSG_REMOVED_SPELL::tokio_read_body(r, size).await?)),
            0x0206 => Ok(Self::SMSG_GMTICKET_CREATE(SMSG_GMTICKET_CREATE::tokio_read_body(r, size).await?)),
            0x0208 => Ok(Self::SMSG_GMTICKET_UPDATETEXT(SMSG_GMTICKET_UPDATETEXT::tokio_read_body(r, size).await?)),
            0x0209 => Ok(Self::SMSG_ACCOUNT_DATA_TIMES(SMSG_ACCOUNT_DATA_TIMES::tokio_read_body(r, size).await?)),
            0x0212 => Ok(Self::SMSG_GMTICKET_GETTICKET(SMSG_GMTICKET_GETTICKET::tokio_read_body(r, size).await?)),
            0x0214 => Ok(Self::SMSG_GAMEOBJECT_SPAWN_ANIM(SMSG_GAMEOBJECT_SPAWN_ANIM::tokio_read_body(r, size).await?)),
            0x0215 => Ok(Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(SMSG_GAMEOBJECT_DESPAWN_ANIM::tokio_read_body(r, size).await?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Server::tokio_read_body(r, size).await?)),
            0x0218 => Ok(Self::SMSG_GMTICKET_DELETETICKET(SMSG_GMTICKET_DELETETICKET::tokio_read_body(r, size).await?)),
            0x0219 => Ok(Self::SMSG_CHAT_WRONG_FACTION(SMSG_CHAT_WRONG_FACTION::tokio_read_body(r, size).await?)),
            0x021B => Ok(Self::SMSG_GMTICKET_SYSTEMSTATUS(SMSG_GMTICKET_SYSTEMSTATUS::tokio_read_body(r, size).await?)),
            0x021E => Ok(Self::SMSG_SET_REST_START(SMSG_SET_REST_START::tokio_read_body(r, size).await?)),
            0x0222 => Ok(Self::SMSG_SPIRIT_HEALER_CONFIRM(SMSG_SPIRIT_HEALER_CONFIRM::tokio_read_body(r, size).await?)),
            0x0224 => Ok(Self::SMSG_GOSSIP_POI(SMSG_GOSSIP_POI::tokio_read_body(r, size).await?)),
            0x0236 => Ok(Self::SMSG_LOGIN_VERIFY_WORLD(SMSG_LOGIN_VERIFY_WORLD::tokio_read_body(r, size).await?)),
            0x023B => Ok(Self::SMSG_MAIL_LIST_RESULT(SMSG_MAIL_LIST_RESULT::tokio_read_body(r, size).await?)),
            0x023D => Ok(Self::SMSG_BATTLEFIELD_LIST(SMSG_BATTLEFIELD_LIST::tokio_read_body(r, size).await?)),
            0x0244 => Ok(Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(SMSG_ITEM_TEXT_QUERY_RESPONSE::tokio_read_body(r, size).await?)),
            0x024B => Ok(Self::SMSG_SPELLLOGMISS(SMSG_SPELLLOGMISS::tokio_read_body(r, size).await?)),
            0x024C => Ok(Self::SMSG_SPELLLOGEXECUTE(SMSG_SPELLLOGEXECUTE::tokio_read_body(r, size).await?)),
            0x024E => Ok(Self::SMSG_PERIODICAURALOG(SMSG_PERIODICAURALOG::tokio_read_body(r, size).await?)),
            0x024F => Ok(Self::SMSG_SPELLDAMAGESHIELD(SMSG_SPELLDAMAGESHIELD::tokio_read_body(r, size).await?)),
            0x0250 => Ok(Self::SMSG_SPELLNONMELEEDAMAGELOG(SMSG_SPELLNONMELEEDAMAGELOG::tokio_read_body(r, size).await?)),
            0x0254 => Ok(Self::SMSG_ZONE_UNDER_ATTACK(SMSG_ZONE_UNDER_ATTACK::tokio_read_body(r, size).await?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Server::tokio_read_body(r, size).await?)),
            0x025C => Ok(Self::SMSG_AUCTION_LIST_RESULT(SMSG_AUCTION_LIST_RESULT::tokio_read_body(r, size).await?)),
            0x025D => Ok(Self::SMSG_AUCTION_OWNER_LIST_RESULT(SMSG_AUCTION_OWNER_LIST_RESULT::tokio_read_body(r, size).await?)),
            0x025E => Ok(Self::SMSG_AUCTION_BIDDER_NOTIFICATION(SMSG_AUCTION_BIDDER_NOTIFICATION::tokio_read_body(r, size).await?)),
            0x025F => Ok(Self::SMSG_AUCTION_OWNER_NOTIFICATION(SMSG_AUCTION_OWNER_NOTIFICATION::tokio_read_body(r, size).await?)),
            0x0260 => Ok(Self::SMSG_PROCRESIST(SMSG_PROCRESIST::tokio_read_body(r, size).await?)),
            0x0262 => Ok(Self::SMSG_DISPEL_FAILED(SMSG_DISPEL_FAILED::tokio_read_body(r, size).await?)),
            0x0263 => Ok(Self::SMSG_SPELLORDAMAGE_IMMUNE(SMSG_SPELLORDAMAGE_IMMUNE::tokio_read_body(r, size).await?)),
            0x0265 => Ok(Self::SMSG_AUCTION_BIDDER_LIST_RESULT(SMSG_AUCTION_BIDDER_LIST_RESULT::tokio_read_body(r, size).await?)),
            0x0266 => Ok(Self::SMSG_SET_FLAT_SPELL_MODIFIER(SMSG_SET_FLAT_SPELL_MODIFIER::tokio_read_body(r, size).await?)),
            0x0267 => Ok(Self::SMSG_SET_PCT_SPELL_MODIFIER(SMSG_SET_PCT_SPELL_MODIFIER::tokio_read_body(r, size).await?)),
            0x0269 => Ok(Self::SMSG_CORPSE_RECLAIM_DELAY(SMSG_CORPSE_RECLAIM_DELAY::tokio_read_body(r, size).await?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Server::tokio_read_body(r, size).await?)),
            0x0273 => Ok(Self::SMSG_STABLE_RESULT(SMSG_STABLE_RESULT::tokio_read_body(r, size).await?)),
            0x0277 => Ok(Self::SMSG_PLAY_MUSIC(SMSG_PLAY_MUSIC::tokio_read_body(r, size).await?)),
            0x0278 => Ok(Self::SMSG_PLAY_OBJECT_SOUND(SMSG_PLAY_OBJECT_SOUND::tokio_read_body(r, size).await?)),
            0x027B => Ok(Self::SMSG_SPELLDISPELLOG(SMSG_SPELLDISPELLOG::tokio_read_body(r, size).await?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Server::tokio_read_body(r, size).await?)),
            0x0285 => Ok(Self::SMSG_RECEIVED_MAIL(SMSG_RECEIVED_MAIL::tokio_read_body(r, size).await?)),
            0x0286 => Ok(Self::SMSG_RAID_GROUP_ONLY(SMSG_RAID_GROUP_ONLY::tokio_read_body(r, size).await?)),
            0x028C => Ok(Self::SMSG_PVP_CREDIT(SMSG_PVP_CREDIT::tokio_read_body(r, size).await?)),
            0x028D => Ok(Self::SMSG_AUCTION_REMOVED_NOTIFICATION(SMSG_AUCTION_REMOVED_NOTIFICATION::tokio_read_body(r, size).await?)),
            0x0291 => Ok(Self::SMSG_SERVER_MESSAGE(SMSG_SERVER_MESSAGE::tokio_read_body(r, size).await?)),
            0x0295 => Ok(Self::SMSG_MEETINGSTONE_SETQUEUE(SMSG_MEETINGSTONE_SETQUEUE::tokio_read_body(r, size).await?)),
            0x0297 => Ok(Self::SMSG_MEETINGSTONE_COMPLETE(SMSG_MEETINGSTONE_COMPLETE::tokio_read_body(r, size).await?)),
            0x0298 => Ok(Self::SMSG_MEETINGSTONE_IN_PROGRESS(SMSG_MEETINGSTONE_IN_PROGRESS::tokio_read_body(r, size).await?)),
            0x0299 => Ok(Self::SMSG_MEETINGSTONE_MEMBER_ADDED(SMSG_MEETINGSTONE_MEMBER_ADDED::tokio_read_body(r, size).await?)),
            0x029C => Ok(Self::SMSG_CANCEL_AUTO_REPEAT(SMSG_CANCEL_AUTO_REPEAT::tokio_read_body(r, size).await?)),
            0x029D => Ok(Self::SMSG_STANDSTATE_UPDATE(SMSG_STANDSTATE_UPDATE::tokio_read_body(r, size).await?)),
            0x029E => Ok(Self::SMSG_LOOT_ALL_PASSED(SMSG_LOOT_ALL_PASSED::tokio_read_body(r, size).await?)),
            0x029F => Ok(Self::SMSG_LOOT_ROLL_WON(SMSG_LOOT_ROLL_WON::tokio_read_body(r, size).await?)),
            0x02A1 => Ok(Self::SMSG_LOOT_START_ROLL(SMSG_LOOT_START_ROLL::tokio_read_body(r, size).await?)),
            0x02A2 => Ok(Self::SMSG_LOOT_ROLL(SMSG_LOOT_ROLL::tokio_read_body(r, size).await?)),
            0x02A4 => Ok(Self::SMSG_LOOT_MASTER_LIST(SMSG_LOOT_MASTER_LIST::tokio_read_body(r, size).await?)),
            0x02A5 => Ok(Self::SMSG_SET_FORCED_REACTIONS(SMSG_SET_FORCED_REACTIONS::tokio_read_body(r, size).await?)),
            0x02A6 => Ok(Self::SMSG_SPELL_FAILED_OTHER(SMSG_SPELL_FAILED_OTHER::tokio_read_body(r, size).await?)),
            0x02A7 => Ok(Self::SMSG_GAMEOBJECT_RESET_STATE(SMSG_GAMEOBJECT_RESET_STATE::tokio_read_body(r, size).await?)),
            0x02A9 => Ok(Self::SMSG_CHAT_PLAYER_NOT_FOUND(SMSG_CHAT_PLAYER_NOT_FOUND::tokio_read_body(r, size).await?)),
            0x02AA => Ok(Self::MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Server::tokio_read_body(r, size).await?)),
            0x02AB => Ok(Self::SMSG_SUMMON_REQUEST(SMSG_SUMMON_REQUEST::tokio_read_body(r, size).await?)),
            0x02AE => Ok(Self::SMSG_MONSTER_MOVE_TRANSPORT(SMSG_MONSTER_MOVE_TRANSPORT::tokio_read_body(r, size).await?)),
            0x02AF => Ok(Self::SMSG_PET_BROKEN(SMSG_PET_BROKEN::tokio_read_body(r, size).await?)),
            0x02B4 => Ok(Self::SMSG_FEIGN_DEATH_RESISTED(SMSG_FEIGN_DEATH_RESISTED::tokio_read_body(r, size).await?)),
            0x02B7 => Ok(Self::SMSG_DUEL_COUNTDOWN(SMSG_DUEL_COUNTDOWN::tokio_read_body(r, size).await?)),
            0x02B8 => Ok(Self::SMSG_AREA_TRIGGER_MESSAGE(SMSG_AREA_TRIGGER_MESSAGE::tokio_read_body(r, size).await?)),
            0x02BB => Ok(Self::SMSG_MEETINGSTONE_JOINFAILED(SMSG_MEETINGSTONE_JOINFAILED::tokio_read_body(r, size).await?)),
            0x02BC => Ok(Self::SMSG_PLAYER_SKINNED(SMSG_PLAYER_SKINNED::tokio_read_body(r, size).await?)),
            0x02BD => Ok(Self::SMSG_DURABILITY_DAMAGE_DEATH(SMSG_DURABILITY_DAMAGE_DEATH::tokio_read_body(r, size).await?)),
            0x02C2 => Ok(Self::SMSG_INIT_WORLD_STATES(SMSG_INIT_WORLD_STATES::tokio_read_body(r, size).await?)),
            0x02C3 => Ok(Self::SMSG_UPDATE_WORLD_STATE(SMSG_UPDATE_WORLD_STATE::tokio_read_body(r, size).await?)),
            0x02C5 => Ok(Self::SMSG_ITEM_NAME_QUERY_RESPONSE(SMSG_ITEM_NAME_QUERY_RESPONSE::tokio_read_body(r, size).await?)),
            0x02C6 => Ok(Self::SMSG_PET_ACTION_FEEDBACK(SMSG_PET_ACTION_FEEDBACK::tokio_read_body(r, size).await?)),
            0x02C8 => Ok(Self::SMSG_CHAR_RENAME(SMSG_CHAR_RENAME::tokio_read_body(r, size).await?)),
            0x02CB => Ok(Self::SMSG_INSTANCE_SAVE_CREATED(SMSG_INSTANCE_SAVE_CREATED::tokio_read_body(r, size).await?)),
            0x02CC => Ok(Self::SMSG_RAID_INSTANCE_INFO(SMSG_RAID_INSTANCE_INFO::tokio_read_body(r, size).await?)),
            0x02D2 => Ok(Self::SMSG_PLAY_SOUND(SMSG_PLAY_SOUND::tokio_read_body(r, size).await?)),
            0x02D4 => Ok(Self::SMSG_BATTLEFIELD_STATUS(SMSG_BATTLEFIELD_STATUS::tokio_read_body(r, size).await?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Server::tokio_read_body(r, size).await?)),
            0x02DA => Ok(Self::SMSG_FORCE_WALK_SPEED_CHANGE(SMSG_FORCE_WALK_SPEED_CHANGE::tokio_read_body(r, size).await?)),
            0x02DC => Ok(Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(SMSG_FORCE_SWIM_BACK_SPEED_CHANGE::tokio_read_body(r, size).await?)),
            0x02DE => Ok(Self::SMSG_FORCE_TURN_RATE_CHANGE(SMSG_FORCE_TURN_RATE_CHANGE::tokio_read_body(r, size).await?)),
            0x02E0 => Ok(Self::MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_Server::tokio_read_body(r, size).await?)),
            0x02E4 => Ok(Self::SMSG_AREA_SPIRIT_HEALER_TIME(SMSG_AREA_SPIRIT_HEALER_TIME::tokio_read_body(r, size).await?)),
            0x02E8 => Ok(Self::SMSG_GROUP_JOINED_BATTLEGROUND(SMSG_GROUP_JOINED_BATTLEGROUND::tokio_read_body(r, size).await?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Server::tokio_read_body(r, size).await?)),
            0x02EB => Ok(Self::SMSG_BINDER_CONFIRM(SMSG_BINDER_CONFIRM::tokio_read_body(r, size).await?)),
            0x02EC => Ok(Self::SMSG_BATTLEGROUND_PLAYER_JOINED(SMSG_BATTLEGROUND_PLAYER_JOINED::tokio_read_body(r, size).await?)),
            0x02ED => Ok(Self::SMSG_BATTLEGROUND_PLAYER_LEFT(SMSG_BATTLEGROUND_PLAYER_LEFT::tokio_read_body(r, size).await?)),
            0x02F1 => Ok(Self::SMSG_PET_UNLEARN_CONFIRM(SMSG_PET_UNLEARN_CONFIRM::tokio_read_body(r, size).await?)),
            0x02F2 => Ok(Self::SMSG_PARTY_MEMBER_STATS_FULL(SMSG_PARTY_MEMBER_STATS_FULL::tokio_read_body(r, size).await?)),
            0x02F4 => Ok(Self::SMSG_WEATHER(SMSG_WEATHER::tokio_read_body(r, size).await?)),
            0x02FA => Ok(Self::SMSG_RAID_INSTANCE_MESSAGE(SMSG_RAID_INSTANCE_MESSAGE::tokio_read_body(r, size).await?)),
            0x02FD => Ok(Self::SMSG_CHAT_RESTRICTED(SMSG_CHAT_RESTRICTED::tokio_read_body(r, size).await?)),
            0x02FE => Ok(Self::SMSG_SPLINE_SET_RUN_SPEED(SMSG_SPLINE_SET_RUN_SPEED::tokio_read_body(r, size).await?)),
            0x02FF => Ok(Self::SMSG_SPLINE_SET_RUN_BACK_SPEED(SMSG_SPLINE_SET_RUN_BACK_SPEED::tokio_read_body(r, size).await?)),
            0x0300 => Ok(Self::SMSG_SPLINE_SET_SWIM_SPEED(SMSG_SPLINE_SET_SWIM_SPEED::tokio_read_body(r, size).await?)),
            0x0301 => Ok(Self::SMSG_SPLINE_SET_WALK_SPEED(SMSG_SPLINE_SET_WALK_SPEED::tokio_read_body(r, size).await?)),
            0x0302 => Ok(Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED(SMSG_SPLINE_SET_SWIM_BACK_SPEED::tokio_read_body(r, size).await?)),
            0x0303 => Ok(Self::SMSG_SPLINE_SET_TURN_RATE(SMSG_SPLINE_SET_TURN_RATE::tokio_read_body(r, size).await?)),
            0x0304 => Ok(Self::SMSG_SPLINE_MOVE_UNROOT(SMSG_SPLINE_MOVE_UNROOT::tokio_read_body(r, size).await?)),
            0x0305 => Ok(Self::SMSG_SPLINE_MOVE_FEATHER_FALL(SMSG_SPLINE_MOVE_FEATHER_FALL::tokio_read_body(r, size).await?)),
            0x0306 => Ok(Self::SMSG_SPLINE_MOVE_NORMAL_FALL(SMSG_SPLINE_MOVE_NORMAL_FALL::tokio_read_body(r, size).await?)),
            0x0307 => Ok(Self::SMSG_SPLINE_MOVE_SET_HOVER(SMSG_SPLINE_MOVE_SET_HOVER::tokio_read_body(r, size).await?)),
            0x0308 => Ok(Self::SMSG_SPLINE_MOVE_UNSET_HOVER(SMSG_SPLINE_MOVE_UNSET_HOVER::tokio_read_body(r, size).await?)),
            0x0309 => Ok(Self::SMSG_SPLINE_MOVE_WATER_WALK(SMSG_SPLINE_MOVE_WATER_WALK::tokio_read_body(r, size).await?)),
            0x030A => Ok(Self::SMSG_SPLINE_MOVE_LAND_WALK(SMSG_SPLINE_MOVE_LAND_WALK::tokio_read_body(r, size).await?)),
            0x030B => Ok(Self::SMSG_SPLINE_MOVE_START_SWIM(SMSG_SPLINE_MOVE_START_SWIM::tokio_read_body(r, size).await?)),
            0x030C => Ok(Self::SMSG_SPLINE_MOVE_STOP_SWIM(SMSG_SPLINE_MOVE_STOP_SWIM::tokio_read_body(r, size).await?)),
            0x030D => Ok(Self::SMSG_SPLINE_MOVE_SET_RUN_MODE(SMSG_SPLINE_MOVE_SET_RUN_MODE::tokio_read_body(r, size).await?)),
            0x030E => Ok(Self::SMSG_SPLINE_MOVE_SET_WALK_MODE(SMSG_SPLINE_MOVE_SET_WALK_MODE::tokio_read_body(r, size).await?)),
            0x031A => Ok(Self::SMSG_SPLINE_MOVE_ROOT(SMSG_SPLINE_MOVE_ROOT::tokio_read_body(r, size).await?)),
            0x031C => Ok(Self::SMSG_INVALIDATE_PLAYER(SMSG_INVALIDATE_PLAYER::tokio_read_body(r, size).await?)),
            0x031E => Ok(Self::SMSG_INSTANCE_RESET(SMSG_INSTANCE_RESET::tokio_read_body(r, size).await?)),
            0x031F => Ok(Self::SMSG_INSTANCE_RESET_FAILED(SMSG_INSTANCE_RESET_FAILED::tokio_read_body(r, size).await?)),
            0x0320 => Ok(Self::SMSG_UPDATE_LAST_INSTANCE(SMSG_UPDATE_LAST_INSTANCE::tokio_read_body(r, size).await?)),
            0x0321 => Ok(Self::MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Server::tokio_read_body(r, size).await?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Server::tokio_read_body(r, size).await?)),
            0x0324 => Ok(Self::SMSG_PET_ACTION_SOUND(SMSG_PET_ACTION_SOUND::tokio_read_body(r, size).await?)),
            0x0325 => Ok(Self::SMSG_PET_DISMISS_SOUND(SMSG_PET_DISMISS_SOUND::tokio_read_body(r, size).await?)),
            0x0328 => Ok(Self::SMSG_GM_TICKET_STATUS_UPDATE(SMSG_GM_TICKET_STATUS_UPDATE::tokio_read_body(r, size).await?)),
            0x032B => Ok(Self::SMSG_UPDATE_INSTANCE_OWNERSHIP(SMSG_UPDATE_INSTANCE_OWNERSHIP::tokio_read_body(r, size).await?)),
            0x032F => Ok(Self::SMSG_SPELLINSTAKILLLOG(SMSG_SPELLINSTAKILLLOG::tokio_read_body(r, size).await?)),
            0x0330 => Ok(Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS(SMSG_SPELL_UPDATE_CHAIN_TARGETS::tokio_read_body(r, size).await?)),
            0x0332 => Ok(Self::SMSG_EXPECTED_SPAM_RECORDS(SMSG_EXPECTED_SPAM_RECORDS::tokio_read_body(r, size).await?)),
            0x033B => Ok(Self::SMSG_DEFENSE_MESSAGE(SMSG_DEFENSE_MESSAGE::tokio_read_body(r, size).await?)),
            _ => Err(ServerOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }
    #[cfg(feature = "tokio")]
    pub async fn tokio_read_encrypted<R: AsyncReadExt + Unpin + Send, D: Decrypter + Send>(r: &mut R, d: &mut D) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let mut header = [0u8; 4];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_server_header(header);
        let header_size = (header.size - 2) as u32;
        match header.opcode {
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD::tokio_read_body(r, header_size).await?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD::tokio_read_body(r, header_size).await?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(MSG_MOVE_STOP::tokio_read_body(r, header_size).await?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT::tokio_read_body(r, header_size).await?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT::tokio_read_body(r, header_size).await?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE::tokio_read_body(r, header_size).await?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(MSG_MOVE_JUMP::tokio_read_body(r, header_size).await?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT::tokio_read_body(r, header_size).await?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT::tokio_read_body(r, header_size).await?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN::tokio_read_body(r, header_size).await?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP::tokio_read_body(r, header_size).await?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN::tokio_read_body(r, header_size).await?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH::tokio_read_body(r, header_size).await?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE::tokio_read_body(r, header_size).await?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE::tokio_read_body(r, header_size).await?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK::tokio_read_body(r, header_size).await?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND::tokio_read_body(r, header_size).await?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM::tokio_read_body(r, header_size).await?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM::tokio_read_body(r, header_size).await?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING::tokio_read_body(r, header_size).await?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH::tokio_read_body(r, header_size).await?)),
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK::tokio_read_body(r, header_size).await?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT::tokio_read_body(r, header_size).await?)),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(MSG_PETITION_DECLINE::tokio_read_body(r, header_size).await?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE::tokio_read_body(r, header_size).await?)),
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT::tokio_read_body(r, header_size).await?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(MSG_PETITION_RENAME::tokio_read_body(r, header_size).await?)),
            0x003A => Ok(Self::SMSG_CHAR_CREATE(SMSG_CHAR_CREATE::tokio_read_body(r, header_size).await?)),
            0x003B => Ok(Self::SMSG_CHAR_ENUM(SMSG_CHAR_ENUM::tokio_read_body(r, header_size).await?)),
            0x003C => Ok(Self::SMSG_CHAR_DELETE(SMSG_CHAR_DELETE::tokio_read_body(r, header_size).await?)),
            0x003E => Ok(Self::SMSG_NEW_WORLD(SMSG_NEW_WORLD::tokio_read_body(r, header_size).await?)),
            0x003F => Ok(Self::SMSG_TRANSFER_PENDING(SMSG_TRANSFER_PENDING::tokio_read_body(r, header_size).await?)),
            0x0040 => Ok(Self::SMSG_TRANSFER_ABORTED(SMSG_TRANSFER_ABORTED::tokio_read_body(r, header_size).await?)),
            0x0041 => Ok(Self::SMSG_CHARACTER_LOGIN_FAILED(SMSG_CHARACTER_LOGIN_FAILED::tokio_read_body(r, header_size).await?)),
            0x0042 => Ok(Self::SMSG_LOGIN_SETTIMESPEED(SMSG_LOGIN_SETTIMESPEED::tokio_read_body(r, header_size).await?)),
            0x004C => Ok(Self::SMSG_LOGOUT_RESPONSE(SMSG_LOGOUT_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x004D => Ok(Self::SMSG_LOGOUT_COMPLETE(SMSG_LOGOUT_COMPLETE::tokio_read_body(r, header_size).await?)),
            0x004F => Ok(Self::SMSG_LOGOUT_CANCEL_ACK(SMSG_LOGOUT_CANCEL_ACK::tokio_read_body(r, header_size).await?)),
            0x0051 => Ok(Self::SMSG_NAME_QUERY_RESPONSE(SMSG_NAME_QUERY_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x0053 => Ok(Self::SMSG_PET_NAME_QUERY_RESPONSE(SMSG_PET_NAME_QUERY_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x0055 => Ok(Self::SMSG_GUILD_QUERY_RESPONSE(SMSG_GUILD_QUERY_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x0058 => Ok(Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(SMSG_ITEM_QUERY_SINGLE_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x005B => Ok(Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(SMSG_PAGE_TEXT_QUERY_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x005D => Ok(Self::SMSG_QUEST_QUERY_RESPONSE(SMSG_QUEST_QUERY_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x005F => Ok(Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(SMSG_GAMEOBJECT_QUERY_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x0061 => Ok(Self::SMSG_CREATURE_QUERY_RESPONSE(SMSG_CREATURE_QUERY_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x0063 => Ok(Self::SMSG_WHO(SMSG_WHO::tokio_read_body(r, header_size).await?)),
            0x0065 => Ok(Self::SMSG_WHOIS(SMSG_WHOIS::tokio_read_body(r, header_size).await?)),
            0x0067 => Ok(Self::SMSG_FRIEND_LIST(SMSG_FRIEND_LIST::tokio_read_body(r, header_size).await?)),
            0x0068 => Ok(Self::SMSG_FRIEND_STATUS(SMSG_FRIEND_STATUS::tokio_read_body(r, header_size).await?)),
            0x006B => Ok(Self::SMSG_IGNORE_LIST(SMSG_IGNORE_LIST::tokio_read_body(r, header_size).await?)),
            0x006F => Ok(Self::SMSG_GROUP_INVITE(SMSG_GROUP_INVITE::tokio_read_body(r, header_size).await?)),
            0x0074 => Ok(Self::SMSG_GROUP_DECLINE(SMSG_GROUP_DECLINE::tokio_read_body(r, header_size).await?)),
            0x0077 => Ok(Self::SMSG_GROUP_UNINVITE(SMSG_GROUP_UNINVITE::tokio_read_body(r, header_size).await?)),
            0x0079 => Ok(Self::SMSG_GROUP_SET_LEADER(SMSG_GROUP_SET_LEADER::tokio_read_body(r, header_size).await?)),
            0x007C => Ok(Self::SMSG_GROUP_DESTROYED(SMSG_GROUP_DESTROYED::tokio_read_body(r, header_size).await?)),
            0x007D => Ok(Self::SMSG_GROUP_LIST(SMSG_GROUP_LIST::tokio_read_body(r, header_size).await?)),
            0x007E => Ok(Self::SMSG_PARTY_MEMBER_STATS(SMSG_PARTY_MEMBER_STATS::tokio_read_body(r, header_size).await?)),
            0x007F => Ok(Self::SMSG_PARTY_COMMAND_RESULT(SMSG_PARTY_COMMAND_RESULT::tokio_read_body(r, header_size).await?)),
            0x0083 => Ok(Self::SMSG_GUILD_INVITE(SMSG_GUILD_INVITE::tokio_read_body(r, header_size).await?)),
            0x0088 => Ok(Self::SMSG_GUILD_INFO(SMSG_GUILD_INFO::tokio_read_body(r, header_size).await?)),
            0x008A => Ok(Self::SMSG_GUILD_ROSTER(SMSG_GUILD_ROSTER::tokio_read_body(r, header_size).await?)),
            0x0092 => Ok(Self::SMSG_GUILD_EVENT(SMSG_GUILD_EVENT::tokio_read_body(r, header_size).await?)),
            0x0093 => Ok(Self::SMSG_GUILD_COMMAND_RESULT(SMSG_GUILD_COMMAND_RESULT::tokio_read_body(r, header_size).await?)),
            0x0096 => Ok(Self::SMSG_MESSAGECHAT(SMSG_MESSAGECHAT::tokio_read_body(r, header_size).await?)),
            0x0099 => Ok(Self::SMSG_CHANNEL_NOTIFY(SMSG_CHANNEL_NOTIFY::tokio_read_body(r, header_size).await?)),
            0x009B => Ok(Self::SMSG_CHANNEL_LIST(SMSG_CHANNEL_LIST::tokio_read_body(r, header_size).await?)),
            0x00AA => Ok(Self::SMSG_DESTROY_OBJECT(SMSG_DESTROY_OBJECT::tokio_read_body(r, header_size).await?)),
            0x00AE => Ok(Self::SMSG_READ_ITEM_OK(SMSG_READ_ITEM_OK::tokio_read_body(r, header_size).await?)),
            0x00AF => Ok(Self::SMSG_READ_ITEM_FAILED(SMSG_READ_ITEM_FAILED::tokio_read_body(r, header_size).await?)),
            0x00B0 => Ok(Self::SMSG_ITEM_COOLDOWN(SMSG_ITEM_COOLDOWN::tokio_read_body(r, header_size).await?)),
            0x00B3 => Ok(Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(SMSG_GAMEOBJECT_CUSTOM_ANIM::tokio_read_body(r, header_size).await?)),
            0x00DD => Ok(Self::SMSG_MONSTER_MOVE(SMSG_MONSTER_MOVE::tokio_read_body(r, header_size).await?)),
            0x00DE => Ok(Self::SMSG_MOVE_WATER_WALK(SMSG_MOVE_WATER_WALK::tokio_read_body(r, header_size).await?)),
            0x00DF => Ok(Self::SMSG_MOVE_LAND_WALK(SMSG_MOVE_LAND_WALK::tokio_read_body(r, header_size).await?)),
            0x00E2 => Ok(Self::SMSG_FORCE_RUN_SPEED_CHANGE(SMSG_FORCE_RUN_SPEED_CHANGE::tokio_read_body(r, header_size).await?)),
            0x00E4 => Ok(Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(SMSG_FORCE_RUN_BACK_SPEED_CHANGE::tokio_read_body(r, header_size).await?)),
            0x00E6 => Ok(Self::SMSG_FORCE_SWIM_SPEED_CHANGE(SMSG_FORCE_SWIM_SPEED_CHANGE::tokio_read_body(r, header_size).await?)),
            0x00E8 => Ok(Self::SMSG_FORCE_MOVE_ROOT(SMSG_FORCE_MOVE_ROOT::tokio_read_body(r, header_size).await?)),
            0x00EA => Ok(Self::SMSG_FORCE_MOVE_UNROOT(SMSG_FORCE_MOVE_UNROOT::tokio_read_body(r, header_size).await?)),
            0x00EF => Ok(Self::SMSG_MOVE_KNOCK_BACK(SMSG_MOVE_KNOCK_BACK::tokio_read_body(r, header_size).await?)),
            0x00F2 => Ok(Self::SMSG_MOVE_FEATHER_FALL(SMSG_MOVE_FEATHER_FALL::tokio_read_body(r, header_size).await?)),
            0x00F3 => Ok(Self::SMSG_MOVE_NORMAL_FALL(SMSG_MOVE_NORMAL_FALL::tokio_read_body(r, header_size).await?)),
            0x00F4 => Ok(Self::SMSG_MOVE_SET_HOVER(SMSG_MOVE_SET_HOVER::tokio_read_body(r, header_size).await?)),
            0x00F5 => Ok(Self::SMSG_MOVE_UNSET_HOVER(SMSG_MOVE_UNSET_HOVER::tokio_read_body(r, header_size).await?)),
            0x00FA => Ok(Self::SMSG_TRIGGER_CINEMATIC(SMSG_TRIGGER_CINEMATIC::tokio_read_body(r, header_size).await?)),
            0x00FD => Ok(Self::SMSG_TUTORIAL_FLAGS(SMSG_TUTORIAL_FLAGS::tokio_read_body(r, header_size).await?)),
            0x0103 => Ok(Self::SMSG_EMOTE(SMSG_EMOTE::tokio_read_body(r, header_size).await?)),
            0x0105 => Ok(Self::SMSG_TEXT_EMOTE(SMSG_TEXT_EMOTE::tokio_read_body(r, header_size).await?)),
            0x0112 => Ok(Self::SMSG_INVENTORY_CHANGE_FAILURE(SMSG_INVENTORY_CHANGE_FAILURE::tokio_read_body(r, header_size).await?)),
            0x0113 => Ok(Self::SMSG_OPEN_CONTAINER(SMSG_OPEN_CONTAINER::tokio_read_body(r, header_size).await?)),
            0x0115 => Ok(Self::SMSG_INSPECT(SMSG_INSPECT::tokio_read_body(r, header_size).await?)),
            0x0120 => Ok(Self::SMSG_TRADE_STATUS(SMSG_TRADE_STATUS::tokio_read_body(r, header_size).await?)),
            0x0121 => Ok(Self::SMSG_TRADE_STATUS_EXTENDED(SMSG_TRADE_STATUS_EXTENDED::tokio_read_body(r, header_size).await?)),
            0x0122 => Ok(Self::SMSG_INITIALIZE_FACTIONS(SMSG_INITIALIZE_FACTIONS::tokio_read_body(r, header_size).await?)),
            0x0123 => Ok(Self::SMSG_SET_FACTION_VISIBLE(SMSG_SET_FACTION_VISIBLE::tokio_read_body(r, header_size).await?)),
            0x0124 => Ok(Self::SMSG_SET_FACTION_STANDING(SMSG_SET_FACTION_STANDING::tokio_read_body(r, header_size).await?)),
            0x0127 => Ok(Self::SMSG_SET_PROFICIENCY(SMSG_SET_PROFICIENCY::tokio_read_body(r, header_size).await?)),
            0x0129 => Ok(Self::SMSG_ACTION_BUTTONS(SMSG_ACTION_BUTTONS::tokio_read_body(r, header_size).await?)),
            0x012A => Ok(Self::SMSG_INITIAL_SPELLS(SMSG_INITIAL_SPELLS::tokio_read_body(r, header_size).await?)),
            0x012B => Ok(Self::SMSG_LEARNED_SPELL(SMSG_LEARNED_SPELL::tokio_read_body(r, header_size).await?)),
            0x012C => Ok(Self::SMSG_SUPERCEDED_SPELL(SMSG_SUPERCEDED_SPELL::tokio_read_body(r, header_size).await?)),
            0x0130 => Ok(Self::SMSG_CAST_RESULT(SMSG_CAST_RESULT::tokio_read_body(r, header_size).await?)),
            0x0131 => Ok(Self::SMSG_SPELL_START(SMSG_SPELL_START::tokio_read_body(r, header_size).await?)),
            0x0132 => Ok(Self::SMSG_SPELL_GO(SMSG_SPELL_GO::tokio_read_body(r, header_size).await?)),
            0x0133 => Ok(Self::SMSG_SPELL_FAILURE(SMSG_SPELL_FAILURE::tokio_read_body(r, header_size).await?)),
            0x0134 => Ok(Self::SMSG_SPELL_COOLDOWN(SMSG_SPELL_COOLDOWN::tokio_read_body(r, header_size).await?)),
            0x0135 => Ok(Self::SMSG_COOLDOWN_EVENT(SMSG_COOLDOWN_EVENT::tokio_read_body(r, header_size).await?)),
            0x0137 => Ok(Self::SMSG_UPDATE_AURA_DURATION(SMSG_UPDATE_AURA_DURATION::tokio_read_body(r, header_size).await?)),
            0x0138 => Ok(Self::SMSG_PET_CAST_FAILED(SMSG_PET_CAST_FAILED::tokio_read_body(r, header_size).await?)),
            0x013C => Ok(Self::SMSG_AI_REACTION(SMSG_AI_REACTION::tokio_read_body(r, header_size).await?)),
            0x0143 => Ok(Self::SMSG_ATTACKSTART(SMSG_ATTACKSTART::tokio_read_body(r, header_size).await?)),
            0x0144 => Ok(Self::SMSG_ATTACKSTOP(SMSG_ATTACKSTOP::tokio_read_body(r, header_size).await?)),
            0x0145 => Ok(Self::SMSG_ATTACKSWING_NOTINRANGE(SMSG_ATTACKSWING_NOTINRANGE::tokio_read_body(r, header_size).await?)),
            0x0146 => Ok(Self::SMSG_ATTACKSWING_BADFACING(SMSG_ATTACKSWING_BADFACING::tokio_read_body(r, header_size).await?)),
            0x0147 => Ok(Self::SMSG_ATTACKSWING_NOTSTANDING(SMSG_ATTACKSWING_NOTSTANDING::tokio_read_body(r, header_size).await?)),
            0x0148 => Ok(Self::SMSG_ATTACKSWING_DEADTARGET(SMSG_ATTACKSWING_DEADTARGET::tokio_read_body(r, header_size).await?)),
            0x0149 => Ok(Self::SMSG_ATTACKSWING_CANT_ATTACK(SMSG_ATTACKSWING_CANT_ATTACK::tokio_read_body(r, header_size).await?)),
            0x014A => Ok(Self::SMSG_ATTACKERSTATEUPDATE(SMSG_ATTACKERSTATEUPDATE::tokio_read_body(r, header_size).await?)),
            0x014E => Ok(Self::SMSG_CANCEL_COMBAT(SMSG_CANCEL_COMBAT::tokio_read_body(r, header_size).await?)),
            0x0150 => Ok(Self::SMSG_SPELLHEALLOG(SMSG_SPELLHEALLOG::tokio_read_body(r, header_size).await?)),
            0x0151 => Ok(Self::SMSG_SPELLENERGIZELOG(SMSG_SPELLENERGIZELOG::tokio_read_body(r, header_size).await?)),
            0x0155 => Ok(Self::SMSG_BINDPOINTUPDATE(SMSG_BINDPOINTUPDATE::tokio_read_body(r, header_size).await?)),
            0x0158 => Ok(Self::SMSG_PLAYERBOUND(SMSG_PLAYERBOUND::tokio_read_body(r, header_size).await?)),
            0x0159 => Ok(Self::SMSG_CLIENT_CONTROL_UPDATE(SMSG_CLIENT_CONTROL_UPDATE::tokio_read_body(r, header_size).await?)),
            0x015B => Ok(Self::SMSG_RESURRECT_REQUEST(SMSG_RESURRECT_REQUEST::tokio_read_body(r, header_size).await?)),
            0x0160 => Ok(Self::SMSG_LOOT_RESPONSE(SMSG_LOOT_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x0161 => Ok(Self::SMSG_LOOT_RELEASE_RESPONSE(SMSG_LOOT_RELEASE_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x0162 => Ok(Self::SMSG_LOOT_REMOVED(SMSG_LOOT_REMOVED::tokio_read_body(r, header_size).await?)),
            0x0163 => Ok(Self::SMSG_LOOT_MONEY_NOTIFY(SMSG_LOOT_MONEY_NOTIFY::tokio_read_body(r, header_size).await?)),
            0x0165 => Ok(Self::SMSG_LOOT_CLEAR_MONEY(SMSG_LOOT_CLEAR_MONEY::tokio_read_body(r, header_size).await?)),
            0x0166 => Ok(Self::SMSG_ITEM_PUSH_RESULT(SMSG_ITEM_PUSH_RESULT::tokio_read_body(r, header_size).await?)),
            0x0167 => Ok(Self::SMSG_DUEL_REQUESTED(SMSG_DUEL_REQUESTED::tokio_read_body(r, header_size).await?)),
            0x0168 => Ok(Self::SMSG_DUEL_OUTOFBOUNDS(SMSG_DUEL_OUTOFBOUNDS::tokio_read_body(r, header_size).await?)),
            0x0169 => Ok(Self::SMSG_DUEL_INBOUNDS(SMSG_DUEL_INBOUNDS::tokio_read_body(r, header_size).await?)),
            0x016A => Ok(Self::SMSG_DUEL_COMPLETE(SMSG_DUEL_COMPLETE::tokio_read_body(r, header_size).await?)),
            0x016B => Ok(Self::SMSG_DUEL_WINNER(SMSG_DUEL_WINNER::tokio_read_body(r, header_size).await?)),
            0x016E => Ok(Self::SMSG_MOUNTRESULT(SMSG_MOUNTRESULT::tokio_read_body(r, header_size).await?)),
            0x016F => Ok(Self::SMSG_DISMOUNTRESULT(SMSG_DISMOUNTRESULT::tokio_read_body(r, header_size).await?)),
            0x0172 => Ok(Self::SMSG_MOUNTSPECIAL_ANIM(SMSG_MOUNTSPECIAL_ANIM::tokio_read_body(r, header_size).await?)),
            0x0173 => Ok(Self::SMSG_PET_TAME_FAILURE(SMSG_PET_TAME_FAILURE::tokio_read_body(r, header_size).await?)),
            0x0178 => Ok(Self::SMSG_PET_NAME_INVALID(SMSG_PET_NAME_INVALID::tokio_read_body(r, header_size).await?)),
            0x0179 => Ok(Self::SMSG_PET_SPELLS(SMSG_PET_SPELLS::tokio_read_body(r, header_size).await?)),
            0x017A => Ok(Self::SMSG_PET_MODE(SMSG_PET_MODE::tokio_read_body(r, header_size).await?)),
            0x017D => Ok(Self::SMSG_GOSSIP_MESSAGE(SMSG_GOSSIP_MESSAGE::tokio_read_body(r, header_size).await?)),
            0x017E => Ok(Self::SMSG_GOSSIP_COMPLETE(SMSG_GOSSIP_COMPLETE::tokio_read_body(r, header_size).await?)),
            0x0180 => Ok(Self::SMSG_NPC_TEXT_UPDATE(SMSG_NPC_TEXT_UPDATE::tokio_read_body(r, header_size).await?)),
            0x0183 => Ok(Self::SMSG_QUESTGIVER_STATUS(SMSG_QUESTGIVER_STATUS::tokio_read_body(r, header_size).await?)),
            0x0185 => Ok(Self::SMSG_QUESTGIVER_QUEST_LIST(SMSG_QUESTGIVER_QUEST_LIST::tokio_read_body(r, header_size).await?)),
            0x0188 => Ok(Self::SMSG_QUESTGIVER_QUEST_DETAILS(SMSG_QUESTGIVER_QUEST_DETAILS::tokio_read_body(r, header_size).await?)),
            0x018B => Ok(Self::SMSG_QUESTGIVER_REQUEST_ITEMS(SMSG_QUESTGIVER_REQUEST_ITEMS::tokio_read_body(r, header_size).await?)),
            0x018D => Ok(Self::SMSG_QUESTGIVER_OFFER_REWARD(SMSG_QUESTGIVER_OFFER_REWARD::tokio_read_body(r, header_size).await?)),
            0x018F => Ok(Self::SMSG_QUESTGIVER_QUEST_INVALID(SMSG_QUESTGIVER_QUEST_INVALID::tokio_read_body(r, header_size).await?)),
            0x0191 => Ok(Self::SMSG_QUESTGIVER_QUEST_COMPLETE(SMSG_QUESTGIVER_QUEST_COMPLETE::tokio_read_body(r, header_size).await?)),
            0x0192 => Ok(Self::SMSG_QUESTGIVER_QUEST_FAILED(SMSG_QUESTGIVER_QUEST_FAILED::tokio_read_body(r, header_size).await?)),
            0x0195 => Ok(Self::SMSG_QUESTLOG_FULL(SMSG_QUESTLOG_FULL::tokio_read_body(r, header_size).await?)),
            0x0196 => Ok(Self::SMSG_QUESTUPDATE_FAILED(SMSG_QUESTUPDATE_FAILED::tokio_read_body(r, header_size).await?)),
            0x0197 => Ok(Self::SMSG_QUESTUPDATE_FAILEDTIMER(SMSG_QUESTUPDATE_FAILEDTIMER::tokio_read_body(r, header_size).await?)),
            0x0198 => Ok(Self::SMSG_QUESTUPDATE_COMPLETE(SMSG_QUESTUPDATE_COMPLETE::tokio_read_body(r, header_size).await?)),
            0x0199 => Ok(Self::SMSG_QUESTUPDATE_ADD_KILL(SMSG_QUESTUPDATE_ADD_KILL::tokio_read_body(r, header_size).await?)),
            0x019A => Ok(Self::SMSG_QUESTUPDATE_ADD_ITEM(SMSG_QUESTUPDATE_ADD_ITEM::tokio_read_body(r, header_size).await?)),
            0x019C => Ok(Self::SMSG_QUEST_CONFIRM_ACCEPT(SMSG_QUEST_CONFIRM_ACCEPT::tokio_read_body(r, header_size).await?)),
            0x019F => Ok(Self::SMSG_LIST_INVENTORY(SMSG_LIST_INVENTORY::tokio_read_body(r, header_size).await?)),
            0x01A1 => Ok(Self::SMSG_SELL_ITEM(SMSG_SELL_ITEM::tokio_read_body(r, header_size).await?)),
            0x01A4 => Ok(Self::SMSG_BUY_ITEM(SMSG_BUY_ITEM::tokio_read_body(r, header_size).await?)),
            0x01A5 => Ok(Self::SMSG_BUY_FAILED(SMSG_BUY_FAILED::tokio_read_body(r, header_size).await?)),
            0x01A9 => Ok(Self::SMSG_SHOWTAXINODES(SMSG_SHOWTAXINODES::tokio_read_body(r, header_size).await?)),
            0x01AB => Ok(Self::SMSG_TAXINODE_STATUS(SMSG_TAXINODE_STATUS::tokio_read_body(r, header_size).await?)),
            0x01AE => Ok(Self::SMSG_ACTIVATETAXIREPLY(SMSG_ACTIVATETAXIREPLY::tokio_read_body(r, header_size).await?)),
            0x01AF => Ok(Self::SMSG_NEW_TAXI_PATH(SMSG_NEW_TAXI_PATH::tokio_read_body(r, header_size).await?)),
            0x01B1 => Ok(Self::SMSG_TRAINER_LIST(SMSG_TRAINER_LIST::tokio_read_body(r, header_size).await?)),
            0x01B3 => Ok(Self::SMSG_TRAINER_BUY_SUCCEEDED(SMSG_TRAINER_BUY_SUCCEEDED::tokio_read_body(r, header_size).await?)),
            0x01B4 => Ok(Self::SMSG_TRAINER_BUY_FAILED(SMSG_TRAINER_BUY_FAILED::tokio_read_body(r, header_size).await?)),
            0x01B8 => Ok(Self::SMSG_SHOW_BANK(SMSG_SHOW_BANK::tokio_read_body(r, header_size).await?)),
            0x01BA => Ok(Self::SMSG_BUY_BANK_SLOT_RESULT(SMSG_BUY_BANK_SLOT_RESULT::tokio_read_body(r, header_size).await?)),
            0x01BC => Ok(Self::SMSG_PETITION_SHOWLIST(SMSG_PETITION_SHOWLIST::tokio_read_body(r, header_size).await?)),
            0x01BF => Ok(Self::SMSG_PETITION_SHOW_SIGNATURES(SMSG_PETITION_SHOW_SIGNATURES::tokio_read_body(r, header_size).await?)),
            0x01C1 => Ok(Self::SMSG_PETITION_SIGN_RESULTS(SMSG_PETITION_SIGN_RESULTS::tokio_read_body(r, header_size).await?)),
            0x01C5 => Ok(Self::SMSG_TURN_IN_PETITION_RESULTS(SMSG_TURN_IN_PETITION_RESULTS::tokio_read_body(r, header_size).await?)),
            0x01C7 => Ok(Self::SMSG_PETITION_QUERY_RESPONSE(SMSG_PETITION_QUERY_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x01C8 => Ok(Self::SMSG_FISH_NOT_HOOKED(SMSG_FISH_NOT_HOOKED::tokio_read_body(r, header_size).await?)),
            0x01C9 => Ok(Self::SMSG_FISH_ESCAPED(SMSG_FISH_ESCAPED::tokio_read_body(r, header_size).await?)),
            0x01CB => Ok(Self::SMSG_NOTIFICATION(SMSG_NOTIFICATION::tokio_read_body(r, header_size).await?)),
            0x01CD => Ok(Self::SMSG_PLAYED_TIME(SMSG_PLAYED_TIME::tokio_read_body(r, header_size).await?)),
            0x01CF => Ok(Self::SMSG_QUERY_TIME_RESPONSE(SMSG_QUERY_TIME_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x01D0 => Ok(Self::SMSG_LOG_XPGAIN(SMSG_LOG_XPGAIN::tokio_read_body(r, header_size).await?)),
            0x01D4 => Ok(Self::SMSG_LEVELUP_INFO(SMSG_LEVELUP_INFO::tokio_read_body(r, header_size).await?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(MSG_MINIMAP_PING_Server::tokio_read_body(r, header_size).await?)),
            0x01D6 => Ok(Self::SMSG_RESISTLOG(SMSG_RESISTLOG::tokio_read_body(r, header_size).await?)),
            0x01D7 => Ok(Self::SMSG_ENCHANTMENTLOG(SMSG_ENCHANTMENTLOG::tokio_read_body(r, header_size).await?)),
            0x01D9 => Ok(Self::SMSG_START_MIRROR_TIMER(SMSG_START_MIRROR_TIMER::tokio_read_body(r, header_size).await?)),
            0x01DA => Ok(Self::SMSG_PAUSE_MIRROR_TIMER(SMSG_PAUSE_MIRROR_TIMER::tokio_read_body(r, header_size).await?)),
            0x01DB => Ok(Self::SMSG_STOP_MIRROR_TIMER(SMSG_STOP_MIRROR_TIMER::tokio_read_body(r, header_size).await?)),
            0x01DD => Ok(Self::SMSG_PONG(SMSG_PONG::tokio_read_body(r, header_size).await?)),
            0x01DE => Ok(Self::SMSG_CLEAR_COOLDOWN(SMSG_CLEAR_COOLDOWN::tokio_read_body(r, header_size).await?)),
            0x01DF => Ok(Self::SMSG_GAMEOBJECT_PAGETEXT(SMSG_GAMEOBJECT_PAGETEXT::tokio_read_body(r, header_size).await?)),
            0x01E2 => Ok(Self::SMSG_SPELL_DELAYED(SMSG_SPELL_DELAYED::tokio_read_body(r, header_size).await?)),
            0x01EA => Ok(Self::SMSG_ITEM_TIME_UPDATE(SMSG_ITEM_TIME_UPDATE::tokio_read_body(r, header_size).await?)),
            0x01EB => Ok(Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(SMSG_ITEM_ENCHANT_TIME_UPDATE::tokio_read_body(r, header_size).await?)),
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE::tokio_read_body(r, header_size).await?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Server::tokio_read_body(r, header_size).await?)),
            0x01F3 => Ok(Self::SMSG_PLAY_SPELL_VISUAL(SMSG_PLAY_SPELL_VISUAL::tokio_read_body(r, header_size).await?)),
            0x01F5 => Ok(Self::SMSG_PARTYKILLLOG(SMSG_PARTYKILLLOG::tokio_read_body(r, header_size).await?)),
            0x01F7 => Ok(Self::SMSG_PLAY_SPELL_IMPACT(SMSG_PLAY_SPELL_IMPACT::tokio_read_body(r, header_size).await?)),
            0x01F8 => Ok(Self::SMSG_EXPLORATION_EXPERIENCE(SMSG_EXPLORATION_EXPERIENCE::tokio_read_body(r, header_size).await?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Server::tokio_read_body(r, header_size).await?)),
            0x01FC => Ok(Self::SMSG_ENVIRONMENTALDAMAGELOG(SMSG_ENVIRONMENTALDAMAGELOG::tokio_read_body(r, header_size).await?)),
            0x01FF => Ok(Self::MSG_LOOKING_FOR_GROUP(MSG_LOOKING_FOR_GROUP_Server::tokio_read_body(r, header_size).await?)),
            0x0203 => Ok(Self::SMSG_REMOVED_SPELL(SMSG_REMOVED_SPELL::tokio_read_body(r, header_size).await?)),
            0x0206 => Ok(Self::SMSG_GMTICKET_CREATE(SMSG_GMTICKET_CREATE::tokio_read_body(r, header_size).await?)),
            0x0208 => Ok(Self::SMSG_GMTICKET_UPDATETEXT(SMSG_GMTICKET_UPDATETEXT::tokio_read_body(r, header_size).await?)),
            0x0209 => Ok(Self::SMSG_ACCOUNT_DATA_TIMES(SMSG_ACCOUNT_DATA_TIMES::tokio_read_body(r, header_size).await?)),
            0x0212 => Ok(Self::SMSG_GMTICKET_GETTICKET(SMSG_GMTICKET_GETTICKET::tokio_read_body(r, header_size).await?)),
            0x0214 => Ok(Self::SMSG_GAMEOBJECT_SPAWN_ANIM(SMSG_GAMEOBJECT_SPAWN_ANIM::tokio_read_body(r, header_size).await?)),
            0x0215 => Ok(Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(SMSG_GAMEOBJECT_DESPAWN_ANIM::tokio_read_body(r, header_size).await?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Server::tokio_read_body(r, header_size).await?)),
            0x0218 => Ok(Self::SMSG_GMTICKET_DELETETICKET(SMSG_GMTICKET_DELETETICKET::tokio_read_body(r, header_size).await?)),
            0x0219 => Ok(Self::SMSG_CHAT_WRONG_FACTION(SMSG_CHAT_WRONG_FACTION::tokio_read_body(r, header_size).await?)),
            0x021B => Ok(Self::SMSG_GMTICKET_SYSTEMSTATUS(SMSG_GMTICKET_SYSTEMSTATUS::tokio_read_body(r, header_size).await?)),
            0x021E => Ok(Self::SMSG_SET_REST_START(SMSG_SET_REST_START::tokio_read_body(r, header_size).await?)),
            0x0222 => Ok(Self::SMSG_SPIRIT_HEALER_CONFIRM(SMSG_SPIRIT_HEALER_CONFIRM::tokio_read_body(r, header_size).await?)),
            0x0224 => Ok(Self::SMSG_GOSSIP_POI(SMSG_GOSSIP_POI::tokio_read_body(r, header_size).await?)),
            0x0236 => Ok(Self::SMSG_LOGIN_VERIFY_WORLD(SMSG_LOGIN_VERIFY_WORLD::tokio_read_body(r, header_size).await?)),
            0x023B => Ok(Self::SMSG_MAIL_LIST_RESULT(SMSG_MAIL_LIST_RESULT::tokio_read_body(r, header_size).await?)),
            0x023D => Ok(Self::SMSG_BATTLEFIELD_LIST(SMSG_BATTLEFIELD_LIST::tokio_read_body(r, header_size).await?)),
            0x0244 => Ok(Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(SMSG_ITEM_TEXT_QUERY_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x024B => Ok(Self::SMSG_SPELLLOGMISS(SMSG_SPELLLOGMISS::tokio_read_body(r, header_size).await?)),
            0x024C => Ok(Self::SMSG_SPELLLOGEXECUTE(SMSG_SPELLLOGEXECUTE::tokio_read_body(r, header_size).await?)),
            0x024E => Ok(Self::SMSG_PERIODICAURALOG(SMSG_PERIODICAURALOG::tokio_read_body(r, header_size).await?)),
            0x024F => Ok(Self::SMSG_SPELLDAMAGESHIELD(SMSG_SPELLDAMAGESHIELD::tokio_read_body(r, header_size).await?)),
            0x0250 => Ok(Self::SMSG_SPELLNONMELEEDAMAGELOG(SMSG_SPELLNONMELEEDAMAGELOG::tokio_read_body(r, header_size).await?)),
            0x0254 => Ok(Self::SMSG_ZONE_UNDER_ATTACK(SMSG_ZONE_UNDER_ATTACK::tokio_read_body(r, header_size).await?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Server::tokio_read_body(r, header_size).await?)),
            0x025C => Ok(Self::SMSG_AUCTION_LIST_RESULT(SMSG_AUCTION_LIST_RESULT::tokio_read_body(r, header_size).await?)),
            0x025D => Ok(Self::SMSG_AUCTION_OWNER_LIST_RESULT(SMSG_AUCTION_OWNER_LIST_RESULT::tokio_read_body(r, header_size).await?)),
            0x025E => Ok(Self::SMSG_AUCTION_BIDDER_NOTIFICATION(SMSG_AUCTION_BIDDER_NOTIFICATION::tokio_read_body(r, header_size).await?)),
            0x025F => Ok(Self::SMSG_AUCTION_OWNER_NOTIFICATION(SMSG_AUCTION_OWNER_NOTIFICATION::tokio_read_body(r, header_size).await?)),
            0x0260 => Ok(Self::SMSG_PROCRESIST(SMSG_PROCRESIST::tokio_read_body(r, header_size).await?)),
            0x0262 => Ok(Self::SMSG_DISPEL_FAILED(SMSG_DISPEL_FAILED::tokio_read_body(r, header_size).await?)),
            0x0263 => Ok(Self::SMSG_SPELLORDAMAGE_IMMUNE(SMSG_SPELLORDAMAGE_IMMUNE::tokio_read_body(r, header_size).await?)),
            0x0265 => Ok(Self::SMSG_AUCTION_BIDDER_LIST_RESULT(SMSG_AUCTION_BIDDER_LIST_RESULT::tokio_read_body(r, header_size).await?)),
            0x0266 => Ok(Self::SMSG_SET_FLAT_SPELL_MODIFIER(SMSG_SET_FLAT_SPELL_MODIFIER::tokio_read_body(r, header_size).await?)),
            0x0267 => Ok(Self::SMSG_SET_PCT_SPELL_MODIFIER(SMSG_SET_PCT_SPELL_MODIFIER::tokio_read_body(r, header_size).await?)),
            0x0269 => Ok(Self::SMSG_CORPSE_RECLAIM_DELAY(SMSG_CORPSE_RECLAIM_DELAY::tokio_read_body(r, header_size).await?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Server::tokio_read_body(r, header_size).await?)),
            0x0273 => Ok(Self::SMSG_STABLE_RESULT(SMSG_STABLE_RESULT::tokio_read_body(r, header_size).await?)),
            0x0277 => Ok(Self::SMSG_PLAY_MUSIC(SMSG_PLAY_MUSIC::tokio_read_body(r, header_size).await?)),
            0x0278 => Ok(Self::SMSG_PLAY_OBJECT_SOUND(SMSG_PLAY_OBJECT_SOUND::tokio_read_body(r, header_size).await?)),
            0x027B => Ok(Self::SMSG_SPELLDISPELLOG(SMSG_SPELLDISPELLOG::tokio_read_body(r, header_size).await?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Server::tokio_read_body(r, header_size).await?)),
            0x0285 => Ok(Self::SMSG_RECEIVED_MAIL(SMSG_RECEIVED_MAIL::tokio_read_body(r, header_size).await?)),
            0x0286 => Ok(Self::SMSG_RAID_GROUP_ONLY(SMSG_RAID_GROUP_ONLY::tokio_read_body(r, header_size).await?)),
            0x028C => Ok(Self::SMSG_PVP_CREDIT(SMSG_PVP_CREDIT::tokio_read_body(r, header_size).await?)),
            0x028D => Ok(Self::SMSG_AUCTION_REMOVED_NOTIFICATION(SMSG_AUCTION_REMOVED_NOTIFICATION::tokio_read_body(r, header_size).await?)),
            0x0291 => Ok(Self::SMSG_SERVER_MESSAGE(SMSG_SERVER_MESSAGE::tokio_read_body(r, header_size).await?)),
            0x0295 => Ok(Self::SMSG_MEETINGSTONE_SETQUEUE(SMSG_MEETINGSTONE_SETQUEUE::tokio_read_body(r, header_size).await?)),
            0x0297 => Ok(Self::SMSG_MEETINGSTONE_COMPLETE(SMSG_MEETINGSTONE_COMPLETE::tokio_read_body(r, header_size).await?)),
            0x0298 => Ok(Self::SMSG_MEETINGSTONE_IN_PROGRESS(SMSG_MEETINGSTONE_IN_PROGRESS::tokio_read_body(r, header_size).await?)),
            0x0299 => Ok(Self::SMSG_MEETINGSTONE_MEMBER_ADDED(SMSG_MEETINGSTONE_MEMBER_ADDED::tokio_read_body(r, header_size).await?)),
            0x029C => Ok(Self::SMSG_CANCEL_AUTO_REPEAT(SMSG_CANCEL_AUTO_REPEAT::tokio_read_body(r, header_size).await?)),
            0x029D => Ok(Self::SMSG_STANDSTATE_UPDATE(SMSG_STANDSTATE_UPDATE::tokio_read_body(r, header_size).await?)),
            0x029E => Ok(Self::SMSG_LOOT_ALL_PASSED(SMSG_LOOT_ALL_PASSED::tokio_read_body(r, header_size).await?)),
            0x029F => Ok(Self::SMSG_LOOT_ROLL_WON(SMSG_LOOT_ROLL_WON::tokio_read_body(r, header_size).await?)),
            0x02A1 => Ok(Self::SMSG_LOOT_START_ROLL(SMSG_LOOT_START_ROLL::tokio_read_body(r, header_size).await?)),
            0x02A2 => Ok(Self::SMSG_LOOT_ROLL(SMSG_LOOT_ROLL::tokio_read_body(r, header_size).await?)),
            0x02A4 => Ok(Self::SMSG_LOOT_MASTER_LIST(SMSG_LOOT_MASTER_LIST::tokio_read_body(r, header_size).await?)),
            0x02A5 => Ok(Self::SMSG_SET_FORCED_REACTIONS(SMSG_SET_FORCED_REACTIONS::tokio_read_body(r, header_size).await?)),
            0x02A6 => Ok(Self::SMSG_SPELL_FAILED_OTHER(SMSG_SPELL_FAILED_OTHER::tokio_read_body(r, header_size).await?)),
            0x02A7 => Ok(Self::SMSG_GAMEOBJECT_RESET_STATE(SMSG_GAMEOBJECT_RESET_STATE::tokio_read_body(r, header_size).await?)),
            0x02A9 => Ok(Self::SMSG_CHAT_PLAYER_NOT_FOUND(SMSG_CHAT_PLAYER_NOT_FOUND::tokio_read_body(r, header_size).await?)),
            0x02AA => Ok(Self::MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Server::tokio_read_body(r, header_size).await?)),
            0x02AB => Ok(Self::SMSG_SUMMON_REQUEST(SMSG_SUMMON_REQUEST::tokio_read_body(r, header_size).await?)),
            0x02AE => Ok(Self::SMSG_MONSTER_MOVE_TRANSPORT(SMSG_MONSTER_MOVE_TRANSPORT::tokio_read_body(r, header_size).await?)),
            0x02AF => Ok(Self::SMSG_PET_BROKEN(SMSG_PET_BROKEN::tokio_read_body(r, header_size).await?)),
            0x02B4 => Ok(Self::SMSG_FEIGN_DEATH_RESISTED(SMSG_FEIGN_DEATH_RESISTED::tokio_read_body(r, header_size).await?)),
            0x02B7 => Ok(Self::SMSG_DUEL_COUNTDOWN(SMSG_DUEL_COUNTDOWN::tokio_read_body(r, header_size).await?)),
            0x02B8 => Ok(Self::SMSG_AREA_TRIGGER_MESSAGE(SMSG_AREA_TRIGGER_MESSAGE::tokio_read_body(r, header_size).await?)),
            0x02BB => Ok(Self::SMSG_MEETINGSTONE_JOINFAILED(SMSG_MEETINGSTONE_JOINFAILED::tokio_read_body(r, header_size).await?)),
            0x02BC => Ok(Self::SMSG_PLAYER_SKINNED(SMSG_PLAYER_SKINNED::tokio_read_body(r, header_size).await?)),
            0x02BD => Ok(Self::SMSG_DURABILITY_DAMAGE_DEATH(SMSG_DURABILITY_DAMAGE_DEATH::tokio_read_body(r, header_size).await?)),
            0x02C2 => Ok(Self::SMSG_INIT_WORLD_STATES(SMSG_INIT_WORLD_STATES::tokio_read_body(r, header_size).await?)),
            0x02C3 => Ok(Self::SMSG_UPDATE_WORLD_STATE(SMSG_UPDATE_WORLD_STATE::tokio_read_body(r, header_size).await?)),
            0x02C5 => Ok(Self::SMSG_ITEM_NAME_QUERY_RESPONSE(SMSG_ITEM_NAME_QUERY_RESPONSE::tokio_read_body(r, header_size).await?)),
            0x02C6 => Ok(Self::SMSG_PET_ACTION_FEEDBACK(SMSG_PET_ACTION_FEEDBACK::tokio_read_body(r, header_size).await?)),
            0x02C8 => Ok(Self::SMSG_CHAR_RENAME(SMSG_CHAR_RENAME::tokio_read_body(r, header_size).await?)),
            0x02CB => Ok(Self::SMSG_INSTANCE_SAVE_CREATED(SMSG_INSTANCE_SAVE_CREATED::tokio_read_body(r, header_size).await?)),
            0x02CC => Ok(Self::SMSG_RAID_INSTANCE_INFO(SMSG_RAID_INSTANCE_INFO::tokio_read_body(r, header_size).await?)),
            0x02D2 => Ok(Self::SMSG_PLAY_SOUND(SMSG_PLAY_SOUND::tokio_read_body(r, header_size).await?)),
            0x02D4 => Ok(Self::SMSG_BATTLEFIELD_STATUS(SMSG_BATTLEFIELD_STATUS::tokio_read_body(r, header_size).await?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Server::tokio_read_body(r, header_size).await?)),
            0x02DA => Ok(Self::SMSG_FORCE_WALK_SPEED_CHANGE(SMSG_FORCE_WALK_SPEED_CHANGE::tokio_read_body(r, header_size).await?)),
            0x02DC => Ok(Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(SMSG_FORCE_SWIM_BACK_SPEED_CHANGE::tokio_read_body(r, header_size).await?)),
            0x02DE => Ok(Self::SMSG_FORCE_TURN_RATE_CHANGE(SMSG_FORCE_TURN_RATE_CHANGE::tokio_read_body(r, header_size).await?)),
            0x02E0 => Ok(Self::MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_Server::tokio_read_body(r, header_size).await?)),
            0x02E4 => Ok(Self::SMSG_AREA_SPIRIT_HEALER_TIME(SMSG_AREA_SPIRIT_HEALER_TIME::tokio_read_body(r, header_size).await?)),
            0x02E8 => Ok(Self::SMSG_GROUP_JOINED_BATTLEGROUND(SMSG_GROUP_JOINED_BATTLEGROUND::tokio_read_body(r, header_size).await?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Server::tokio_read_body(r, header_size).await?)),
            0x02EB => Ok(Self::SMSG_BINDER_CONFIRM(SMSG_BINDER_CONFIRM::tokio_read_body(r, header_size).await?)),
            0x02EC => Ok(Self::SMSG_BATTLEGROUND_PLAYER_JOINED(SMSG_BATTLEGROUND_PLAYER_JOINED::tokio_read_body(r, header_size).await?)),
            0x02ED => Ok(Self::SMSG_BATTLEGROUND_PLAYER_LEFT(SMSG_BATTLEGROUND_PLAYER_LEFT::tokio_read_body(r, header_size).await?)),
            0x02F1 => Ok(Self::SMSG_PET_UNLEARN_CONFIRM(SMSG_PET_UNLEARN_CONFIRM::tokio_read_body(r, header_size).await?)),
            0x02F2 => Ok(Self::SMSG_PARTY_MEMBER_STATS_FULL(SMSG_PARTY_MEMBER_STATS_FULL::tokio_read_body(r, header_size).await?)),
            0x02F4 => Ok(Self::SMSG_WEATHER(SMSG_WEATHER::tokio_read_body(r, header_size).await?)),
            0x02FA => Ok(Self::SMSG_RAID_INSTANCE_MESSAGE(SMSG_RAID_INSTANCE_MESSAGE::tokio_read_body(r, header_size).await?)),
            0x02FD => Ok(Self::SMSG_CHAT_RESTRICTED(SMSG_CHAT_RESTRICTED::tokio_read_body(r, header_size).await?)),
            0x02FE => Ok(Self::SMSG_SPLINE_SET_RUN_SPEED(SMSG_SPLINE_SET_RUN_SPEED::tokio_read_body(r, header_size).await?)),
            0x02FF => Ok(Self::SMSG_SPLINE_SET_RUN_BACK_SPEED(SMSG_SPLINE_SET_RUN_BACK_SPEED::tokio_read_body(r, header_size).await?)),
            0x0300 => Ok(Self::SMSG_SPLINE_SET_SWIM_SPEED(SMSG_SPLINE_SET_SWIM_SPEED::tokio_read_body(r, header_size).await?)),
            0x0301 => Ok(Self::SMSG_SPLINE_SET_WALK_SPEED(SMSG_SPLINE_SET_WALK_SPEED::tokio_read_body(r, header_size).await?)),
            0x0302 => Ok(Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED(SMSG_SPLINE_SET_SWIM_BACK_SPEED::tokio_read_body(r, header_size).await?)),
            0x0303 => Ok(Self::SMSG_SPLINE_SET_TURN_RATE(SMSG_SPLINE_SET_TURN_RATE::tokio_read_body(r, header_size).await?)),
            0x0304 => Ok(Self::SMSG_SPLINE_MOVE_UNROOT(SMSG_SPLINE_MOVE_UNROOT::tokio_read_body(r, header_size).await?)),
            0x0305 => Ok(Self::SMSG_SPLINE_MOVE_FEATHER_FALL(SMSG_SPLINE_MOVE_FEATHER_FALL::tokio_read_body(r, header_size).await?)),
            0x0306 => Ok(Self::SMSG_SPLINE_MOVE_NORMAL_FALL(SMSG_SPLINE_MOVE_NORMAL_FALL::tokio_read_body(r, header_size).await?)),
            0x0307 => Ok(Self::SMSG_SPLINE_MOVE_SET_HOVER(SMSG_SPLINE_MOVE_SET_HOVER::tokio_read_body(r, header_size).await?)),
            0x0308 => Ok(Self::SMSG_SPLINE_MOVE_UNSET_HOVER(SMSG_SPLINE_MOVE_UNSET_HOVER::tokio_read_body(r, header_size).await?)),
            0x0309 => Ok(Self::SMSG_SPLINE_MOVE_WATER_WALK(SMSG_SPLINE_MOVE_WATER_WALK::tokio_read_body(r, header_size).await?)),
            0x030A => Ok(Self::SMSG_SPLINE_MOVE_LAND_WALK(SMSG_SPLINE_MOVE_LAND_WALK::tokio_read_body(r, header_size).await?)),
            0x030B => Ok(Self::SMSG_SPLINE_MOVE_START_SWIM(SMSG_SPLINE_MOVE_START_SWIM::tokio_read_body(r, header_size).await?)),
            0x030C => Ok(Self::SMSG_SPLINE_MOVE_STOP_SWIM(SMSG_SPLINE_MOVE_STOP_SWIM::tokio_read_body(r, header_size).await?)),
            0x030D => Ok(Self::SMSG_SPLINE_MOVE_SET_RUN_MODE(SMSG_SPLINE_MOVE_SET_RUN_MODE::tokio_read_body(r, header_size).await?)),
            0x030E => Ok(Self::SMSG_SPLINE_MOVE_SET_WALK_MODE(SMSG_SPLINE_MOVE_SET_WALK_MODE::tokio_read_body(r, header_size).await?)),
            0x031A => Ok(Self::SMSG_SPLINE_MOVE_ROOT(SMSG_SPLINE_MOVE_ROOT::tokio_read_body(r, header_size).await?)),
            0x031C => Ok(Self::SMSG_INVALIDATE_PLAYER(SMSG_INVALIDATE_PLAYER::tokio_read_body(r, header_size).await?)),
            0x031E => Ok(Self::SMSG_INSTANCE_RESET(SMSG_INSTANCE_RESET::tokio_read_body(r, header_size).await?)),
            0x031F => Ok(Self::SMSG_INSTANCE_RESET_FAILED(SMSG_INSTANCE_RESET_FAILED::tokio_read_body(r, header_size).await?)),
            0x0320 => Ok(Self::SMSG_UPDATE_LAST_INSTANCE(SMSG_UPDATE_LAST_INSTANCE::tokio_read_body(r, header_size).await?)),
            0x0321 => Ok(Self::MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Server::tokio_read_body(r, header_size).await?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Server::tokio_read_body(r, header_size).await?)),
            0x0324 => Ok(Self::SMSG_PET_ACTION_SOUND(SMSG_PET_ACTION_SOUND::tokio_read_body(r, header_size).await?)),
            0x0325 => Ok(Self::SMSG_PET_DISMISS_SOUND(SMSG_PET_DISMISS_SOUND::tokio_read_body(r, header_size).await?)),
            0x0328 => Ok(Self::SMSG_GM_TICKET_STATUS_UPDATE(SMSG_GM_TICKET_STATUS_UPDATE::tokio_read_body(r, header_size).await?)),
            0x032B => Ok(Self::SMSG_UPDATE_INSTANCE_OWNERSHIP(SMSG_UPDATE_INSTANCE_OWNERSHIP::tokio_read_body(r, header_size).await?)),
            0x032F => Ok(Self::SMSG_SPELLINSTAKILLLOG(SMSG_SPELLINSTAKILLLOG::tokio_read_body(r, header_size).await?)),
            0x0330 => Ok(Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS(SMSG_SPELL_UPDATE_CHAIN_TARGETS::tokio_read_body(r, header_size).await?)),
            0x0332 => Ok(Self::SMSG_EXPECTED_SPAM_RECORDS(SMSG_EXPECTED_SPAM_RECORDS::tokio_read_body(r, header_size).await?)),
            0x033B => Ok(Self::SMSG_DEFENSE_MESSAGE(SMSG_DEFENSE_MESSAGE::tokio_read_body(r, header_size).await?)),
            _ => Err(ServerOpcodeMessageError::InvalidOpcode(header.opcode)),
        }
    }


    #[cfg(feature = "async-std")]
    pub async fn astd_read_unencrypted<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let size = (crate::util::astd_read_u16_be(r).await? - 2) as u32;
        let opcode = crate::util::astd_read_u16_le(r).await?;
        match opcode {
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD::astd_read_body(r, size).await?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD::astd_read_body(r, size).await?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(MSG_MOVE_STOP::astd_read_body(r, size).await?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT::astd_read_body(r, size).await?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT::astd_read_body(r, size).await?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE::astd_read_body(r, size).await?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(MSG_MOVE_JUMP::astd_read_body(r, size).await?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT::astd_read_body(r, size).await?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT::astd_read_body(r, size).await?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN::astd_read_body(r, size).await?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP::astd_read_body(r, size).await?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN::astd_read_body(r, size).await?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH::astd_read_body(r, size).await?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE::astd_read_body(r, size).await?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE::astd_read_body(r, size).await?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK::astd_read_body(r, size).await?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND::astd_read_body(r, size).await?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM::astd_read_body(r, size).await?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM::astd_read_body(r, size).await?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING::astd_read_body(r, size).await?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH::astd_read_body(r, size).await?)),
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK::astd_read_body(r, size).await?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT::astd_read_body(r, size).await?)),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(MSG_PETITION_DECLINE::astd_read_body(r, size).await?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE::astd_read_body(r, size).await?)),
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT::astd_read_body(r, size).await?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(MSG_PETITION_RENAME::astd_read_body(r, size).await?)),
            0x003A => Ok(Self::SMSG_CHAR_CREATE(SMSG_CHAR_CREATE::astd_read_body(r, size).await?)),
            0x003B => Ok(Self::SMSG_CHAR_ENUM(SMSG_CHAR_ENUM::astd_read_body(r, size).await?)),
            0x003C => Ok(Self::SMSG_CHAR_DELETE(SMSG_CHAR_DELETE::astd_read_body(r, size).await?)),
            0x003E => Ok(Self::SMSG_NEW_WORLD(SMSG_NEW_WORLD::astd_read_body(r, size).await?)),
            0x003F => Ok(Self::SMSG_TRANSFER_PENDING(SMSG_TRANSFER_PENDING::astd_read_body(r, size).await?)),
            0x0040 => Ok(Self::SMSG_TRANSFER_ABORTED(SMSG_TRANSFER_ABORTED::astd_read_body(r, size).await?)),
            0x0041 => Ok(Self::SMSG_CHARACTER_LOGIN_FAILED(SMSG_CHARACTER_LOGIN_FAILED::astd_read_body(r, size).await?)),
            0x0042 => Ok(Self::SMSG_LOGIN_SETTIMESPEED(SMSG_LOGIN_SETTIMESPEED::astd_read_body(r, size).await?)),
            0x004C => Ok(Self::SMSG_LOGOUT_RESPONSE(SMSG_LOGOUT_RESPONSE::astd_read_body(r, size).await?)),
            0x004D => Ok(Self::SMSG_LOGOUT_COMPLETE(SMSG_LOGOUT_COMPLETE::astd_read_body(r, size).await?)),
            0x004F => Ok(Self::SMSG_LOGOUT_CANCEL_ACK(SMSG_LOGOUT_CANCEL_ACK::astd_read_body(r, size).await?)),
            0x0051 => Ok(Self::SMSG_NAME_QUERY_RESPONSE(SMSG_NAME_QUERY_RESPONSE::astd_read_body(r, size).await?)),
            0x0053 => Ok(Self::SMSG_PET_NAME_QUERY_RESPONSE(SMSG_PET_NAME_QUERY_RESPONSE::astd_read_body(r, size).await?)),
            0x0055 => Ok(Self::SMSG_GUILD_QUERY_RESPONSE(SMSG_GUILD_QUERY_RESPONSE::astd_read_body(r, size).await?)),
            0x0058 => Ok(Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(SMSG_ITEM_QUERY_SINGLE_RESPONSE::astd_read_body(r, size).await?)),
            0x005B => Ok(Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(SMSG_PAGE_TEXT_QUERY_RESPONSE::astd_read_body(r, size).await?)),
            0x005D => Ok(Self::SMSG_QUEST_QUERY_RESPONSE(SMSG_QUEST_QUERY_RESPONSE::astd_read_body(r, size).await?)),
            0x005F => Ok(Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(SMSG_GAMEOBJECT_QUERY_RESPONSE::astd_read_body(r, size).await?)),
            0x0061 => Ok(Self::SMSG_CREATURE_QUERY_RESPONSE(SMSG_CREATURE_QUERY_RESPONSE::astd_read_body(r, size).await?)),
            0x0063 => Ok(Self::SMSG_WHO(SMSG_WHO::astd_read_body(r, size).await?)),
            0x0065 => Ok(Self::SMSG_WHOIS(SMSG_WHOIS::astd_read_body(r, size).await?)),
            0x0067 => Ok(Self::SMSG_FRIEND_LIST(SMSG_FRIEND_LIST::astd_read_body(r, size).await?)),
            0x0068 => Ok(Self::SMSG_FRIEND_STATUS(SMSG_FRIEND_STATUS::astd_read_body(r, size).await?)),
            0x006B => Ok(Self::SMSG_IGNORE_LIST(SMSG_IGNORE_LIST::astd_read_body(r, size).await?)),
            0x006F => Ok(Self::SMSG_GROUP_INVITE(SMSG_GROUP_INVITE::astd_read_body(r, size).await?)),
            0x0074 => Ok(Self::SMSG_GROUP_DECLINE(SMSG_GROUP_DECLINE::astd_read_body(r, size).await?)),
            0x0077 => Ok(Self::SMSG_GROUP_UNINVITE(SMSG_GROUP_UNINVITE::astd_read_body(r, size).await?)),
            0x0079 => Ok(Self::SMSG_GROUP_SET_LEADER(SMSG_GROUP_SET_LEADER::astd_read_body(r, size).await?)),
            0x007C => Ok(Self::SMSG_GROUP_DESTROYED(SMSG_GROUP_DESTROYED::astd_read_body(r, size).await?)),
            0x007D => Ok(Self::SMSG_GROUP_LIST(SMSG_GROUP_LIST::astd_read_body(r, size).await?)),
            0x007E => Ok(Self::SMSG_PARTY_MEMBER_STATS(SMSG_PARTY_MEMBER_STATS::astd_read_body(r, size).await?)),
            0x007F => Ok(Self::SMSG_PARTY_COMMAND_RESULT(SMSG_PARTY_COMMAND_RESULT::astd_read_body(r, size).await?)),
            0x0083 => Ok(Self::SMSG_GUILD_INVITE(SMSG_GUILD_INVITE::astd_read_body(r, size).await?)),
            0x0088 => Ok(Self::SMSG_GUILD_INFO(SMSG_GUILD_INFO::astd_read_body(r, size).await?)),
            0x008A => Ok(Self::SMSG_GUILD_ROSTER(SMSG_GUILD_ROSTER::astd_read_body(r, size).await?)),
            0x0092 => Ok(Self::SMSG_GUILD_EVENT(SMSG_GUILD_EVENT::astd_read_body(r, size).await?)),
            0x0093 => Ok(Self::SMSG_GUILD_COMMAND_RESULT(SMSG_GUILD_COMMAND_RESULT::astd_read_body(r, size).await?)),
            0x0096 => Ok(Self::SMSG_MESSAGECHAT(SMSG_MESSAGECHAT::astd_read_body(r, size).await?)),
            0x0099 => Ok(Self::SMSG_CHANNEL_NOTIFY(SMSG_CHANNEL_NOTIFY::astd_read_body(r, size).await?)),
            0x009B => Ok(Self::SMSG_CHANNEL_LIST(SMSG_CHANNEL_LIST::astd_read_body(r, size).await?)),
            0x00AA => Ok(Self::SMSG_DESTROY_OBJECT(SMSG_DESTROY_OBJECT::astd_read_body(r, size).await?)),
            0x00AE => Ok(Self::SMSG_READ_ITEM_OK(SMSG_READ_ITEM_OK::astd_read_body(r, size).await?)),
            0x00AF => Ok(Self::SMSG_READ_ITEM_FAILED(SMSG_READ_ITEM_FAILED::astd_read_body(r, size).await?)),
            0x00B0 => Ok(Self::SMSG_ITEM_COOLDOWN(SMSG_ITEM_COOLDOWN::astd_read_body(r, size).await?)),
            0x00B3 => Ok(Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(SMSG_GAMEOBJECT_CUSTOM_ANIM::astd_read_body(r, size).await?)),
            0x00DD => Ok(Self::SMSG_MONSTER_MOVE(SMSG_MONSTER_MOVE::astd_read_body(r, size).await?)),
            0x00DE => Ok(Self::SMSG_MOVE_WATER_WALK(SMSG_MOVE_WATER_WALK::astd_read_body(r, size).await?)),
            0x00DF => Ok(Self::SMSG_MOVE_LAND_WALK(SMSG_MOVE_LAND_WALK::astd_read_body(r, size).await?)),
            0x00E2 => Ok(Self::SMSG_FORCE_RUN_SPEED_CHANGE(SMSG_FORCE_RUN_SPEED_CHANGE::astd_read_body(r, size).await?)),
            0x00E4 => Ok(Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(SMSG_FORCE_RUN_BACK_SPEED_CHANGE::astd_read_body(r, size).await?)),
            0x00E6 => Ok(Self::SMSG_FORCE_SWIM_SPEED_CHANGE(SMSG_FORCE_SWIM_SPEED_CHANGE::astd_read_body(r, size).await?)),
            0x00E8 => Ok(Self::SMSG_FORCE_MOVE_ROOT(SMSG_FORCE_MOVE_ROOT::astd_read_body(r, size).await?)),
            0x00EA => Ok(Self::SMSG_FORCE_MOVE_UNROOT(SMSG_FORCE_MOVE_UNROOT::astd_read_body(r, size).await?)),
            0x00EF => Ok(Self::SMSG_MOVE_KNOCK_BACK(SMSG_MOVE_KNOCK_BACK::astd_read_body(r, size).await?)),
            0x00F2 => Ok(Self::SMSG_MOVE_FEATHER_FALL(SMSG_MOVE_FEATHER_FALL::astd_read_body(r, size).await?)),
            0x00F3 => Ok(Self::SMSG_MOVE_NORMAL_FALL(SMSG_MOVE_NORMAL_FALL::astd_read_body(r, size).await?)),
            0x00F4 => Ok(Self::SMSG_MOVE_SET_HOVER(SMSG_MOVE_SET_HOVER::astd_read_body(r, size).await?)),
            0x00F5 => Ok(Self::SMSG_MOVE_UNSET_HOVER(SMSG_MOVE_UNSET_HOVER::astd_read_body(r, size).await?)),
            0x00FA => Ok(Self::SMSG_TRIGGER_CINEMATIC(SMSG_TRIGGER_CINEMATIC::astd_read_body(r, size).await?)),
            0x00FD => Ok(Self::SMSG_TUTORIAL_FLAGS(SMSG_TUTORIAL_FLAGS::astd_read_body(r, size).await?)),
            0x0103 => Ok(Self::SMSG_EMOTE(SMSG_EMOTE::astd_read_body(r, size).await?)),
            0x0105 => Ok(Self::SMSG_TEXT_EMOTE(SMSG_TEXT_EMOTE::astd_read_body(r, size).await?)),
            0x0112 => Ok(Self::SMSG_INVENTORY_CHANGE_FAILURE(SMSG_INVENTORY_CHANGE_FAILURE::astd_read_body(r, size).await?)),
            0x0113 => Ok(Self::SMSG_OPEN_CONTAINER(SMSG_OPEN_CONTAINER::astd_read_body(r, size).await?)),
            0x0115 => Ok(Self::SMSG_INSPECT(SMSG_INSPECT::astd_read_body(r, size).await?)),
            0x0120 => Ok(Self::SMSG_TRADE_STATUS(SMSG_TRADE_STATUS::astd_read_body(r, size).await?)),
            0x0121 => Ok(Self::SMSG_TRADE_STATUS_EXTENDED(SMSG_TRADE_STATUS_EXTENDED::astd_read_body(r, size).await?)),
            0x0122 => Ok(Self::SMSG_INITIALIZE_FACTIONS(SMSG_INITIALIZE_FACTIONS::astd_read_body(r, size).await?)),
            0x0123 => Ok(Self::SMSG_SET_FACTION_VISIBLE(SMSG_SET_FACTION_VISIBLE::astd_read_body(r, size).await?)),
            0x0124 => Ok(Self::SMSG_SET_FACTION_STANDING(SMSG_SET_FACTION_STANDING::astd_read_body(r, size).await?)),
            0x0127 => Ok(Self::SMSG_SET_PROFICIENCY(SMSG_SET_PROFICIENCY::astd_read_body(r, size).await?)),
            0x0129 => Ok(Self::SMSG_ACTION_BUTTONS(SMSG_ACTION_BUTTONS::astd_read_body(r, size).await?)),
            0x012A => Ok(Self::SMSG_INITIAL_SPELLS(SMSG_INITIAL_SPELLS::astd_read_body(r, size).await?)),
            0x012B => Ok(Self::SMSG_LEARNED_SPELL(SMSG_LEARNED_SPELL::astd_read_body(r, size).await?)),
            0x012C => Ok(Self::SMSG_SUPERCEDED_SPELL(SMSG_SUPERCEDED_SPELL::astd_read_body(r, size).await?)),
            0x0130 => Ok(Self::SMSG_CAST_RESULT(SMSG_CAST_RESULT::astd_read_body(r, size).await?)),
            0x0131 => Ok(Self::SMSG_SPELL_START(SMSG_SPELL_START::astd_read_body(r, size).await?)),
            0x0132 => Ok(Self::SMSG_SPELL_GO(SMSG_SPELL_GO::astd_read_body(r, size).await?)),
            0x0133 => Ok(Self::SMSG_SPELL_FAILURE(SMSG_SPELL_FAILURE::astd_read_body(r, size).await?)),
            0x0134 => Ok(Self::SMSG_SPELL_COOLDOWN(SMSG_SPELL_COOLDOWN::astd_read_body(r, size).await?)),
            0x0135 => Ok(Self::SMSG_COOLDOWN_EVENT(SMSG_COOLDOWN_EVENT::astd_read_body(r, size).await?)),
            0x0137 => Ok(Self::SMSG_UPDATE_AURA_DURATION(SMSG_UPDATE_AURA_DURATION::astd_read_body(r, size).await?)),
            0x0138 => Ok(Self::SMSG_PET_CAST_FAILED(SMSG_PET_CAST_FAILED::astd_read_body(r, size).await?)),
            0x013C => Ok(Self::SMSG_AI_REACTION(SMSG_AI_REACTION::astd_read_body(r, size).await?)),
            0x0143 => Ok(Self::SMSG_ATTACKSTART(SMSG_ATTACKSTART::astd_read_body(r, size).await?)),
            0x0144 => Ok(Self::SMSG_ATTACKSTOP(SMSG_ATTACKSTOP::astd_read_body(r, size).await?)),
            0x0145 => Ok(Self::SMSG_ATTACKSWING_NOTINRANGE(SMSG_ATTACKSWING_NOTINRANGE::astd_read_body(r, size).await?)),
            0x0146 => Ok(Self::SMSG_ATTACKSWING_BADFACING(SMSG_ATTACKSWING_BADFACING::astd_read_body(r, size).await?)),
            0x0147 => Ok(Self::SMSG_ATTACKSWING_NOTSTANDING(SMSG_ATTACKSWING_NOTSTANDING::astd_read_body(r, size).await?)),
            0x0148 => Ok(Self::SMSG_ATTACKSWING_DEADTARGET(SMSG_ATTACKSWING_DEADTARGET::astd_read_body(r, size).await?)),
            0x0149 => Ok(Self::SMSG_ATTACKSWING_CANT_ATTACK(SMSG_ATTACKSWING_CANT_ATTACK::astd_read_body(r, size).await?)),
            0x014A => Ok(Self::SMSG_ATTACKERSTATEUPDATE(SMSG_ATTACKERSTATEUPDATE::astd_read_body(r, size).await?)),
            0x014E => Ok(Self::SMSG_CANCEL_COMBAT(SMSG_CANCEL_COMBAT::astd_read_body(r, size).await?)),
            0x0150 => Ok(Self::SMSG_SPELLHEALLOG(SMSG_SPELLHEALLOG::astd_read_body(r, size).await?)),
            0x0151 => Ok(Self::SMSG_SPELLENERGIZELOG(SMSG_SPELLENERGIZELOG::astd_read_body(r, size).await?)),
            0x0155 => Ok(Self::SMSG_BINDPOINTUPDATE(SMSG_BINDPOINTUPDATE::astd_read_body(r, size).await?)),
            0x0158 => Ok(Self::SMSG_PLAYERBOUND(SMSG_PLAYERBOUND::astd_read_body(r, size).await?)),
            0x0159 => Ok(Self::SMSG_CLIENT_CONTROL_UPDATE(SMSG_CLIENT_CONTROL_UPDATE::astd_read_body(r, size).await?)),
            0x015B => Ok(Self::SMSG_RESURRECT_REQUEST(SMSG_RESURRECT_REQUEST::astd_read_body(r, size).await?)),
            0x0160 => Ok(Self::SMSG_LOOT_RESPONSE(SMSG_LOOT_RESPONSE::astd_read_body(r, size).await?)),
            0x0161 => Ok(Self::SMSG_LOOT_RELEASE_RESPONSE(SMSG_LOOT_RELEASE_RESPONSE::astd_read_body(r, size).await?)),
            0x0162 => Ok(Self::SMSG_LOOT_REMOVED(SMSG_LOOT_REMOVED::astd_read_body(r, size).await?)),
            0x0163 => Ok(Self::SMSG_LOOT_MONEY_NOTIFY(SMSG_LOOT_MONEY_NOTIFY::astd_read_body(r, size).await?)),
            0x0165 => Ok(Self::SMSG_LOOT_CLEAR_MONEY(SMSG_LOOT_CLEAR_MONEY::astd_read_body(r, size).await?)),
            0x0166 => Ok(Self::SMSG_ITEM_PUSH_RESULT(SMSG_ITEM_PUSH_RESULT::astd_read_body(r, size).await?)),
            0x0167 => Ok(Self::SMSG_DUEL_REQUESTED(SMSG_DUEL_REQUESTED::astd_read_body(r, size).await?)),
            0x0168 => Ok(Self::SMSG_DUEL_OUTOFBOUNDS(SMSG_DUEL_OUTOFBOUNDS::astd_read_body(r, size).await?)),
            0x0169 => Ok(Self::SMSG_DUEL_INBOUNDS(SMSG_DUEL_INBOUNDS::astd_read_body(r, size).await?)),
            0x016A => Ok(Self::SMSG_DUEL_COMPLETE(SMSG_DUEL_COMPLETE::astd_read_body(r, size).await?)),
            0x016B => Ok(Self::SMSG_DUEL_WINNER(SMSG_DUEL_WINNER::astd_read_body(r, size).await?)),
            0x016E => Ok(Self::SMSG_MOUNTRESULT(SMSG_MOUNTRESULT::astd_read_body(r, size).await?)),
            0x016F => Ok(Self::SMSG_DISMOUNTRESULT(SMSG_DISMOUNTRESULT::astd_read_body(r, size).await?)),
            0x0172 => Ok(Self::SMSG_MOUNTSPECIAL_ANIM(SMSG_MOUNTSPECIAL_ANIM::astd_read_body(r, size).await?)),
            0x0173 => Ok(Self::SMSG_PET_TAME_FAILURE(SMSG_PET_TAME_FAILURE::astd_read_body(r, size).await?)),
            0x0178 => Ok(Self::SMSG_PET_NAME_INVALID(SMSG_PET_NAME_INVALID::astd_read_body(r, size).await?)),
            0x0179 => Ok(Self::SMSG_PET_SPELLS(SMSG_PET_SPELLS::astd_read_body(r, size).await?)),
            0x017A => Ok(Self::SMSG_PET_MODE(SMSG_PET_MODE::astd_read_body(r, size).await?)),
            0x017D => Ok(Self::SMSG_GOSSIP_MESSAGE(SMSG_GOSSIP_MESSAGE::astd_read_body(r, size).await?)),
            0x017E => Ok(Self::SMSG_GOSSIP_COMPLETE(SMSG_GOSSIP_COMPLETE::astd_read_body(r, size).await?)),
            0x0180 => Ok(Self::SMSG_NPC_TEXT_UPDATE(SMSG_NPC_TEXT_UPDATE::astd_read_body(r, size).await?)),
            0x0183 => Ok(Self::SMSG_QUESTGIVER_STATUS(SMSG_QUESTGIVER_STATUS::astd_read_body(r, size).await?)),
            0x0185 => Ok(Self::SMSG_QUESTGIVER_QUEST_LIST(SMSG_QUESTGIVER_QUEST_LIST::astd_read_body(r, size).await?)),
            0x0188 => Ok(Self::SMSG_QUESTGIVER_QUEST_DETAILS(SMSG_QUESTGIVER_QUEST_DETAILS::astd_read_body(r, size).await?)),
            0x018B => Ok(Self::SMSG_QUESTGIVER_REQUEST_ITEMS(SMSG_QUESTGIVER_REQUEST_ITEMS::astd_read_body(r, size).await?)),
            0x018D => Ok(Self::SMSG_QUESTGIVER_OFFER_REWARD(SMSG_QUESTGIVER_OFFER_REWARD::astd_read_body(r, size).await?)),
            0x018F => Ok(Self::SMSG_QUESTGIVER_QUEST_INVALID(SMSG_QUESTGIVER_QUEST_INVALID::astd_read_body(r, size).await?)),
            0x0191 => Ok(Self::SMSG_QUESTGIVER_QUEST_COMPLETE(SMSG_QUESTGIVER_QUEST_COMPLETE::astd_read_body(r, size).await?)),
            0x0192 => Ok(Self::SMSG_QUESTGIVER_QUEST_FAILED(SMSG_QUESTGIVER_QUEST_FAILED::astd_read_body(r, size).await?)),
            0x0195 => Ok(Self::SMSG_QUESTLOG_FULL(SMSG_QUESTLOG_FULL::astd_read_body(r, size).await?)),
            0x0196 => Ok(Self::SMSG_QUESTUPDATE_FAILED(SMSG_QUESTUPDATE_FAILED::astd_read_body(r, size).await?)),
            0x0197 => Ok(Self::SMSG_QUESTUPDATE_FAILEDTIMER(SMSG_QUESTUPDATE_FAILEDTIMER::astd_read_body(r, size).await?)),
            0x0198 => Ok(Self::SMSG_QUESTUPDATE_COMPLETE(SMSG_QUESTUPDATE_COMPLETE::astd_read_body(r, size).await?)),
            0x0199 => Ok(Self::SMSG_QUESTUPDATE_ADD_KILL(SMSG_QUESTUPDATE_ADD_KILL::astd_read_body(r, size).await?)),
            0x019A => Ok(Self::SMSG_QUESTUPDATE_ADD_ITEM(SMSG_QUESTUPDATE_ADD_ITEM::astd_read_body(r, size).await?)),
            0x019C => Ok(Self::SMSG_QUEST_CONFIRM_ACCEPT(SMSG_QUEST_CONFIRM_ACCEPT::astd_read_body(r, size).await?)),
            0x019F => Ok(Self::SMSG_LIST_INVENTORY(SMSG_LIST_INVENTORY::astd_read_body(r, size).await?)),
            0x01A1 => Ok(Self::SMSG_SELL_ITEM(SMSG_SELL_ITEM::astd_read_body(r, size).await?)),
            0x01A4 => Ok(Self::SMSG_BUY_ITEM(SMSG_BUY_ITEM::astd_read_body(r, size).await?)),
            0x01A5 => Ok(Self::SMSG_BUY_FAILED(SMSG_BUY_FAILED::astd_read_body(r, size).await?)),
            0x01A9 => Ok(Self::SMSG_SHOWTAXINODES(SMSG_SHOWTAXINODES::astd_read_body(r, size).await?)),
            0x01AB => Ok(Self::SMSG_TAXINODE_STATUS(SMSG_TAXINODE_STATUS::astd_read_body(r, size).await?)),
            0x01AE => Ok(Self::SMSG_ACTIVATETAXIREPLY(SMSG_ACTIVATETAXIREPLY::astd_read_body(r, size).await?)),
            0x01AF => Ok(Self::SMSG_NEW_TAXI_PATH(SMSG_NEW_TAXI_PATH::astd_read_body(r, size).await?)),
            0x01B1 => Ok(Self::SMSG_TRAINER_LIST(SMSG_TRAINER_LIST::astd_read_body(r, size).await?)),
            0x01B3 => Ok(Self::SMSG_TRAINER_BUY_SUCCEEDED(SMSG_TRAINER_BUY_SUCCEEDED::astd_read_body(r, size).await?)),
            0x01B4 => Ok(Self::SMSG_TRAINER_BUY_FAILED(SMSG_TRAINER_BUY_FAILED::astd_read_body(r, size).await?)),
            0x01B8 => Ok(Self::SMSG_SHOW_BANK(SMSG_SHOW_BANK::astd_read_body(r, size).await?)),
            0x01BA => Ok(Self::SMSG_BUY_BANK_SLOT_RESULT(SMSG_BUY_BANK_SLOT_RESULT::astd_read_body(r, size).await?)),
            0x01BC => Ok(Self::SMSG_PETITION_SHOWLIST(SMSG_PETITION_SHOWLIST::astd_read_body(r, size).await?)),
            0x01BF => Ok(Self::SMSG_PETITION_SHOW_SIGNATURES(SMSG_PETITION_SHOW_SIGNATURES::astd_read_body(r, size).await?)),
            0x01C1 => Ok(Self::SMSG_PETITION_SIGN_RESULTS(SMSG_PETITION_SIGN_RESULTS::astd_read_body(r, size).await?)),
            0x01C5 => Ok(Self::SMSG_TURN_IN_PETITION_RESULTS(SMSG_TURN_IN_PETITION_RESULTS::astd_read_body(r, size).await?)),
            0x01C7 => Ok(Self::SMSG_PETITION_QUERY_RESPONSE(SMSG_PETITION_QUERY_RESPONSE::astd_read_body(r, size).await?)),
            0x01C8 => Ok(Self::SMSG_FISH_NOT_HOOKED(SMSG_FISH_NOT_HOOKED::astd_read_body(r, size).await?)),
            0x01C9 => Ok(Self::SMSG_FISH_ESCAPED(SMSG_FISH_ESCAPED::astd_read_body(r, size).await?)),
            0x01CB => Ok(Self::SMSG_NOTIFICATION(SMSG_NOTIFICATION::astd_read_body(r, size).await?)),
            0x01CD => Ok(Self::SMSG_PLAYED_TIME(SMSG_PLAYED_TIME::astd_read_body(r, size).await?)),
            0x01CF => Ok(Self::SMSG_QUERY_TIME_RESPONSE(SMSG_QUERY_TIME_RESPONSE::astd_read_body(r, size).await?)),
            0x01D0 => Ok(Self::SMSG_LOG_XPGAIN(SMSG_LOG_XPGAIN::astd_read_body(r, size).await?)),
            0x01D4 => Ok(Self::SMSG_LEVELUP_INFO(SMSG_LEVELUP_INFO::astd_read_body(r, size).await?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(MSG_MINIMAP_PING_Server::astd_read_body(r, size).await?)),
            0x01D6 => Ok(Self::SMSG_RESISTLOG(SMSG_RESISTLOG::astd_read_body(r, size).await?)),
            0x01D7 => Ok(Self::SMSG_ENCHANTMENTLOG(SMSG_ENCHANTMENTLOG::astd_read_body(r, size).await?)),
            0x01D9 => Ok(Self::SMSG_START_MIRROR_TIMER(SMSG_START_MIRROR_TIMER::astd_read_body(r, size).await?)),
            0x01DA => Ok(Self::SMSG_PAUSE_MIRROR_TIMER(SMSG_PAUSE_MIRROR_TIMER::astd_read_body(r, size).await?)),
            0x01DB => Ok(Self::SMSG_STOP_MIRROR_TIMER(SMSG_STOP_MIRROR_TIMER::astd_read_body(r, size).await?)),
            0x01DD => Ok(Self::SMSG_PONG(SMSG_PONG::astd_read_body(r, size).await?)),
            0x01DE => Ok(Self::SMSG_CLEAR_COOLDOWN(SMSG_CLEAR_COOLDOWN::astd_read_body(r, size).await?)),
            0x01DF => Ok(Self::SMSG_GAMEOBJECT_PAGETEXT(SMSG_GAMEOBJECT_PAGETEXT::astd_read_body(r, size).await?)),
            0x01E2 => Ok(Self::SMSG_SPELL_DELAYED(SMSG_SPELL_DELAYED::astd_read_body(r, size).await?)),
            0x01EA => Ok(Self::SMSG_ITEM_TIME_UPDATE(SMSG_ITEM_TIME_UPDATE::astd_read_body(r, size).await?)),
            0x01EB => Ok(Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(SMSG_ITEM_ENCHANT_TIME_UPDATE::astd_read_body(r, size).await?)),
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE::astd_read_body(r, size).await?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE::astd_read_body(r, size).await?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Server::astd_read_body(r, size).await?)),
            0x01F3 => Ok(Self::SMSG_PLAY_SPELL_VISUAL(SMSG_PLAY_SPELL_VISUAL::astd_read_body(r, size).await?)),
            0x01F5 => Ok(Self::SMSG_PARTYKILLLOG(SMSG_PARTYKILLLOG::astd_read_body(r, size).await?)),
            0x01F7 => Ok(Self::SMSG_PLAY_SPELL_IMPACT(SMSG_PLAY_SPELL_IMPACT::astd_read_body(r, size).await?)),
            0x01F8 => Ok(Self::SMSG_EXPLORATION_EXPERIENCE(SMSG_EXPLORATION_EXPERIENCE::astd_read_body(r, size).await?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Server::astd_read_body(r, size).await?)),
            0x01FC => Ok(Self::SMSG_ENVIRONMENTALDAMAGELOG(SMSG_ENVIRONMENTALDAMAGELOG::astd_read_body(r, size).await?)),
            0x01FF => Ok(Self::MSG_LOOKING_FOR_GROUP(MSG_LOOKING_FOR_GROUP_Server::astd_read_body(r, size).await?)),
            0x0203 => Ok(Self::SMSG_REMOVED_SPELL(SMSG_REMOVED_SPELL::astd_read_body(r, size).await?)),
            0x0206 => Ok(Self::SMSG_GMTICKET_CREATE(SMSG_GMTICKET_CREATE::astd_read_body(r, size).await?)),
            0x0208 => Ok(Self::SMSG_GMTICKET_UPDATETEXT(SMSG_GMTICKET_UPDATETEXT::astd_read_body(r, size).await?)),
            0x0209 => Ok(Self::SMSG_ACCOUNT_DATA_TIMES(SMSG_ACCOUNT_DATA_TIMES::astd_read_body(r, size).await?)),
            0x0212 => Ok(Self::SMSG_GMTICKET_GETTICKET(SMSG_GMTICKET_GETTICKET::astd_read_body(r, size).await?)),
            0x0214 => Ok(Self::SMSG_GAMEOBJECT_SPAWN_ANIM(SMSG_GAMEOBJECT_SPAWN_ANIM::astd_read_body(r, size).await?)),
            0x0215 => Ok(Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(SMSG_GAMEOBJECT_DESPAWN_ANIM::astd_read_body(r, size).await?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Server::astd_read_body(r, size).await?)),
            0x0218 => Ok(Self::SMSG_GMTICKET_DELETETICKET(SMSG_GMTICKET_DELETETICKET::astd_read_body(r, size).await?)),
            0x0219 => Ok(Self::SMSG_CHAT_WRONG_FACTION(SMSG_CHAT_WRONG_FACTION::astd_read_body(r, size).await?)),
            0x021B => Ok(Self::SMSG_GMTICKET_SYSTEMSTATUS(SMSG_GMTICKET_SYSTEMSTATUS::astd_read_body(r, size).await?)),
            0x021E => Ok(Self::SMSG_SET_REST_START(SMSG_SET_REST_START::astd_read_body(r, size).await?)),
            0x0222 => Ok(Self::SMSG_SPIRIT_HEALER_CONFIRM(SMSG_SPIRIT_HEALER_CONFIRM::astd_read_body(r, size).await?)),
            0x0224 => Ok(Self::SMSG_GOSSIP_POI(SMSG_GOSSIP_POI::astd_read_body(r, size).await?)),
            0x0236 => Ok(Self::SMSG_LOGIN_VERIFY_WORLD(SMSG_LOGIN_VERIFY_WORLD::astd_read_body(r, size).await?)),
            0x023B => Ok(Self::SMSG_MAIL_LIST_RESULT(SMSG_MAIL_LIST_RESULT::astd_read_body(r, size).await?)),
            0x023D => Ok(Self::SMSG_BATTLEFIELD_LIST(SMSG_BATTLEFIELD_LIST::astd_read_body(r, size).await?)),
            0x0244 => Ok(Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(SMSG_ITEM_TEXT_QUERY_RESPONSE::astd_read_body(r, size).await?)),
            0x024B => Ok(Self::SMSG_SPELLLOGMISS(SMSG_SPELLLOGMISS::astd_read_body(r, size).await?)),
            0x024C => Ok(Self::SMSG_SPELLLOGEXECUTE(SMSG_SPELLLOGEXECUTE::astd_read_body(r, size).await?)),
            0x024E => Ok(Self::SMSG_PERIODICAURALOG(SMSG_PERIODICAURALOG::astd_read_body(r, size).await?)),
            0x024F => Ok(Self::SMSG_SPELLDAMAGESHIELD(SMSG_SPELLDAMAGESHIELD::astd_read_body(r, size).await?)),
            0x0250 => Ok(Self::SMSG_SPELLNONMELEEDAMAGELOG(SMSG_SPELLNONMELEEDAMAGELOG::astd_read_body(r, size).await?)),
            0x0254 => Ok(Self::SMSG_ZONE_UNDER_ATTACK(SMSG_ZONE_UNDER_ATTACK::astd_read_body(r, size).await?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Server::astd_read_body(r, size).await?)),
            0x025C => Ok(Self::SMSG_AUCTION_LIST_RESULT(SMSG_AUCTION_LIST_RESULT::astd_read_body(r, size).await?)),
            0x025D => Ok(Self::SMSG_AUCTION_OWNER_LIST_RESULT(SMSG_AUCTION_OWNER_LIST_RESULT::astd_read_body(r, size).await?)),
            0x025E => Ok(Self::SMSG_AUCTION_BIDDER_NOTIFICATION(SMSG_AUCTION_BIDDER_NOTIFICATION::astd_read_body(r, size).await?)),
            0x025F => Ok(Self::SMSG_AUCTION_OWNER_NOTIFICATION(SMSG_AUCTION_OWNER_NOTIFICATION::astd_read_body(r, size).await?)),
            0x0260 => Ok(Self::SMSG_PROCRESIST(SMSG_PROCRESIST::astd_read_body(r, size).await?)),
            0x0262 => Ok(Self::SMSG_DISPEL_FAILED(SMSG_DISPEL_FAILED::astd_read_body(r, size).await?)),
            0x0263 => Ok(Self::SMSG_SPELLORDAMAGE_IMMUNE(SMSG_SPELLORDAMAGE_IMMUNE::astd_read_body(r, size).await?)),
            0x0265 => Ok(Self::SMSG_AUCTION_BIDDER_LIST_RESULT(SMSG_AUCTION_BIDDER_LIST_RESULT::astd_read_body(r, size).await?)),
            0x0266 => Ok(Self::SMSG_SET_FLAT_SPELL_MODIFIER(SMSG_SET_FLAT_SPELL_MODIFIER::astd_read_body(r, size).await?)),
            0x0267 => Ok(Self::SMSG_SET_PCT_SPELL_MODIFIER(SMSG_SET_PCT_SPELL_MODIFIER::astd_read_body(r, size).await?)),
            0x0269 => Ok(Self::SMSG_CORPSE_RECLAIM_DELAY(SMSG_CORPSE_RECLAIM_DELAY::astd_read_body(r, size).await?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Server::astd_read_body(r, size).await?)),
            0x0273 => Ok(Self::SMSG_STABLE_RESULT(SMSG_STABLE_RESULT::astd_read_body(r, size).await?)),
            0x0277 => Ok(Self::SMSG_PLAY_MUSIC(SMSG_PLAY_MUSIC::astd_read_body(r, size).await?)),
            0x0278 => Ok(Self::SMSG_PLAY_OBJECT_SOUND(SMSG_PLAY_OBJECT_SOUND::astd_read_body(r, size).await?)),
            0x027B => Ok(Self::SMSG_SPELLDISPELLOG(SMSG_SPELLDISPELLOG::astd_read_body(r, size).await?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Server::astd_read_body(r, size).await?)),
            0x0285 => Ok(Self::SMSG_RECEIVED_MAIL(SMSG_RECEIVED_MAIL::astd_read_body(r, size).await?)),
            0x0286 => Ok(Self::SMSG_RAID_GROUP_ONLY(SMSG_RAID_GROUP_ONLY::astd_read_body(r, size).await?)),
            0x028C => Ok(Self::SMSG_PVP_CREDIT(SMSG_PVP_CREDIT::astd_read_body(r, size).await?)),
            0x028D => Ok(Self::SMSG_AUCTION_REMOVED_NOTIFICATION(SMSG_AUCTION_REMOVED_NOTIFICATION::astd_read_body(r, size).await?)),
            0x0291 => Ok(Self::SMSG_SERVER_MESSAGE(SMSG_SERVER_MESSAGE::astd_read_body(r, size).await?)),
            0x0295 => Ok(Self::SMSG_MEETINGSTONE_SETQUEUE(SMSG_MEETINGSTONE_SETQUEUE::astd_read_body(r, size).await?)),
            0x0297 => Ok(Self::SMSG_MEETINGSTONE_COMPLETE(SMSG_MEETINGSTONE_COMPLETE::astd_read_body(r, size).await?)),
            0x0298 => Ok(Self::SMSG_MEETINGSTONE_IN_PROGRESS(SMSG_MEETINGSTONE_IN_PROGRESS::astd_read_body(r, size).await?)),
            0x0299 => Ok(Self::SMSG_MEETINGSTONE_MEMBER_ADDED(SMSG_MEETINGSTONE_MEMBER_ADDED::astd_read_body(r, size).await?)),
            0x029C => Ok(Self::SMSG_CANCEL_AUTO_REPEAT(SMSG_CANCEL_AUTO_REPEAT::astd_read_body(r, size).await?)),
            0x029D => Ok(Self::SMSG_STANDSTATE_UPDATE(SMSG_STANDSTATE_UPDATE::astd_read_body(r, size).await?)),
            0x029E => Ok(Self::SMSG_LOOT_ALL_PASSED(SMSG_LOOT_ALL_PASSED::astd_read_body(r, size).await?)),
            0x029F => Ok(Self::SMSG_LOOT_ROLL_WON(SMSG_LOOT_ROLL_WON::astd_read_body(r, size).await?)),
            0x02A1 => Ok(Self::SMSG_LOOT_START_ROLL(SMSG_LOOT_START_ROLL::astd_read_body(r, size).await?)),
            0x02A2 => Ok(Self::SMSG_LOOT_ROLL(SMSG_LOOT_ROLL::astd_read_body(r, size).await?)),
            0x02A4 => Ok(Self::SMSG_LOOT_MASTER_LIST(SMSG_LOOT_MASTER_LIST::astd_read_body(r, size).await?)),
            0x02A5 => Ok(Self::SMSG_SET_FORCED_REACTIONS(SMSG_SET_FORCED_REACTIONS::astd_read_body(r, size).await?)),
            0x02A6 => Ok(Self::SMSG_SPELL_FAILED_OTHER(SMSG_SPELL_FAILED_OTHER::astd_read_body(r, size).await?)),
            0x02A7 => Ok(Self::SMSG_GAMEOBJECT_RESET_STATE(SMSG_GAMEOBJECT_RESET_STATE::astd_read_body(r, size).await?)),
            0x02A9 => Ok(Self::SMSG_CHAT_PLAYER_NOT_FOUND(SMSG_CHAT_PLAYER_NOT_FOUND::astd_read_body(r, size).await?)),
            0x02AA => Ok(Self::MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Server::astd_read_body(r, size).await?)),
            0x02AB => Ok(Self::SMSG_SUMMON_REQUEST(SMSG_SUMMON_REQUEST::astd_read_body(r, size).await?)),
            0x02AE => Ok(Self::SMSG_MONSTER_MOVE_TRANSPORT(SMSG_MONSTER_MOVE_TRANSPORT::astd_read_body(r, size).await?)),
            0x02AF => Ok(Self::SMSG_PET_BROKEN(SMSG_PET_BROKEN::astd_read_body(r, size).await?)),
            0x02B4 => Ok(Self::SMSG_FEIGN_DEATH_RESISTED(SMSG_FEIGN_DEATH_RESISTED::astd_read_body(r, size).await?)),
            0x02B7 => Ok(Self::SMSG_DUEL_COUNTDOWN(SMSG_DUEL_COUNTDOWN::astd_read_body(r, size).await?)),
            0x02B8 => Ok(Self::SMSG_AREA_TRIGGER_MESSAGE(SMSG_AREA_TRIGGER_MESSAGE::astd_read_body(r, size).await?)),
            0x02BB => Ok(Self::SMSG_MEETINGSTONE_JOINFAILED(SMSG_MEETINGSTONE_JOINFAILED::astd_read_body(r, size).await?)),
            0x02BC => Ok(Self::SMSG_PLAYER_SKINNED(SMSG_PLAYER_SKINNED::astd_read_body(r, size).await?)),
            0x02BD => Ok(Self::SMSG_DURABILITY_DAMAGE_DEATH(SMSG_DURABILITY_DAMAGE_DEATH::astd_read_body(r, size).await?)),
            0x02C2 => Ok(Self::SMSG_INIT_WORLD_STATES(SMSG_INIT_WORLD_STATES::astd_read_body(r, size).await?)),
            0x02C3 => Ok(Self::SMSG_UPDATE_WORLD_STATE(SMSG_UPDATE_WORLD_STATE::astd_read_body(r, size).await?)),
            0x02C5 => Ok(Self::SMSG_ITEM_NAME_QUERY_RESPONSE(SMSG_ITEM_NAME_QUERY_RESPONSE::astd_read_body(r, size).await?)),
            0x02C6 => Ok(Self::SMSG_PET_ACTION_FEEDBACK(SMSG_PET_ACTION_FEEDBACK::astd_read_body(r, size).await?)),
            0x02C8 => Ok(Self::SMSG_CHAR_RENAME(SMSG_CHAR_RENAME::astd_read_body(r, size).await?)),
            0x02CB => Ok(Self::SMSG_INSTANCE_SAVE_CREATED(SMSG_INSTANCE_SAVE_CREATED::astd_read_body(r, size).await?)),
            0x02CC => Ok(Self::SMSG_RAID_INSTANCE_INFO(SMSG_RAID_INSTANCE_INFO::astd_read_body(r, size).await?)),
            0x02D2 => Ok(Self::SMSG_PLAY_SOUND(SMSG_PLAY_SOUND::astd_read_body(r, size).await?)),
            0x02D4 => Ok(Self::SMSG_BATTLEFIELD_STATUS(SMSG_BATTLEFIELD_STATUS::astd_read_body(r, size).await?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Server::astd_read_body(r, size).await?)),
            0x02DA => Ok(Self::SMSG_FORCE_WALK_SPEED_CHANGE(SMSG_FORCE_WALK_SPEED_CHANGE::astd_read_body(r, size).await?)),
            0x02DC => Ok(Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(SMSG_FORCE_SWIM_BACK_SPEED_CHANGE::astd_read_body(r, size).await?)),
            0x02DE => Ok(Self::SMSG_FORCE_TURN_RATE_CHANGE(SMSG_FORCE_TURN_RATE_CHANGE::astd_read_body(r, size).await?)),
            0x02E0 => Ok(Self::MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_Server::astd_read_body(r, size).await?)),
            0x02E4 => Ok(Self::SMSG_AREA_SPIRIT_HEALER_TIME(SMSG_AREA_SPIRIT_HEALER_TIME::astd_read_body(r, size).await?)),
            0x02E8 => Ok(Self::SMSG_GROUP_JOINED_BATTLEGROUND(SMSG_GROUP_JOINED_BATTLEGROUND::astd_read_body(r, size).await?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Server::astd_read_body(r, size).await?)),
            0x02EB => Ok(Self::SMSG_BINDER_CONFIRM(SMSG_BINDER_CONFIRM::astd_read_body(r, size).await?)),
            0x02EC => Ok(Self::SMSG_BATTLEGROUND_PLAYER_JOINED(SMSG_BATTLEGROUND_PLAYER_JOINED::astd_read_body(r, size).await?)),
            0x02ED => Ok(Self::SMSG_BATTLEGROUND_PLAYER_LEFT(SMSG_BATTLEGROUND_PLAYER_LEFT::astd_read_body(r, size).await?)),
            0x02F1 => Ok(Self::SMSG_PET_UNLEARN_CONFIRM(SMSG_PET_UNLEARN_CONFIRM::astd_read_body(r, size).await?)),
            0x02F2 => Ok(Self::SMSG_PARTY_MEMBER_STATS_FULL(SMSG_PARTY_MEMBER_STATS_FULL::astd_read_body(r, size).await?)),
            0x02F4 => Ok(Self::SMSG_WEATHER(SMSG_WEATHER::astd_read_body(r, size).await?)),
            0x02FA => Ok(Self::SMSG_RAID_INSTANCE_MESSAGE(SMSG_RAID_INSTANCE_MESSAGE::astd_read_body(r, size).await?)),
            0x02FD => Ok(Self::SMSG_CHAT_RESTRICTED(SMSG_CHAT_RESTRICTED::astd_read_body(r, size).await?)),
            0x02FE => Ok(Self::SMSG_SPLINE_SET_RUN_SPEED(SMSG_SPLINE_SET_RUN_SPEED::astd_read_body(r, size).await?)),
            0x02FF => Ok(Self::SMSG_SPLINE_SET_RUN_BACK_SPEED(SMSG_SPLINE_SET_RUN_BACK_SPEED::astd_read_body(r, size).await?)),
            0x0300 => Ok(Self::SMSG_SPLINE_SET_SWIM_SPEED(SMSG_SPLINE_SET_SWIM_SPEED::astd_read_body(r, size).await?)),
            0x0301 => Ok(Self::SMSG_SPLINE_SET_WALK_SPEED(SMSG_SPLINE_SET_WALK_SPEED::astd_read_body(r, size).await?)),
            0x0302 => Ok(Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED(SMSG_SPLINE_SET_SWIM_BACK_SPEED::astd_read_body(r, size).await?)),
            0x0303 => Ok(Self::SMSG_SPLINE_SET_TURN_RATE(SMSG_SPLINE_SET_TURN_RATE::astd_read_body(r, size).await?)),
            0x0304 => Ok(Self::SMSG_SPLINE_MOVE_UNROOT(SMSG_SPLINE_MOVE_UNROOT::astd_read_body(r, size).await?)),
            0x0305 => Ok(Self::SMSG_SPLINE_MOVE_FEATHER_FALL(SMSG_SPLINE_MOVE_FEATHER_FALL::astd_read_body(r, size).await?)),
            0x0306 => Ok(Self::SMSG_SPLINE_MOVE_NORMAL_FALL(SMSG_SPLINE_MOVE_NORMAL_FALL::astd_read_body(r, size).await?)),
            0x0307 => Ok(Self::SMSG_SPLINE_MOVE_SET_HOVER(SMSG_SPLINE_MOVE_SET_HOVER::astd_read_body(r, size).await?)),
            0x0308 => Ok(Self::SMSG_SPLINE_MOVE_UNSET_HOVER(SMSG_SPLINE_MOVE_UNSET_HOVER::astd_read_body(r, size).await?)),
            0x0309 => Ok(Self::SMSG_SPLINE_MOVE_WATER_WALK(SMSG_SPLINE_MOVE_WATER_WALK::astd_read_body(r, size).await?)),
            0x030A => Ok(Self::SMSG_SPLINE_MOVE_LAND_WALK(SMSG_SPLINE_MOVE_LAND_WALK::astd_read_body(r, size).await?)),
            0x030B => Ok(Self::SMSG_SPLINE_MOVE_START_SWIM(SMSG_SPLINE_MOVE_START_SWIM::astd_read_body(r, size).await?)),
            0x030C => Ok(Self::SMSG_SPLINE_MOVE_STOP_SWIM(SMSG_SPLINE_MOVE_STOP_SWIM::astd_read_body(r, size).await?)),
            0x030D => Ok(Self::SMSG_SPLINE_MOVE_SET_RUN_MODE(SMSG_SPLINE_MOVE_SET_RUN_MODE::astd_read_body(r, size).await?)),
            0x030E => Ok(Self::SMSG_SPLINE_MOVE_SET_WALK_MODE(SMSG_SPLINE_MOVE_SET_WALK_MODE::astd_read_body(r, size).await?)),
            0x031A => Ok(Self::SMSG_SPLINE_MOVE_ROOT(SMSG_SPLINE_MOVE_ROOT::astd_read_body(r, size).await?)),
            0x031C => Ok(Self::SMSG_INVALIDATE_PLAYER(SMSG_INVALIDATE_PLAYER::astd_read_body(r, size).await?)),
            0x031E => Ok(Self::SMSG_INSTANCE_RESET(SMSG_INSTANCE_RESET::astd_read_body(r, size).await?)),
            0x031F => Ok(Self::SMSG_INSTANCE_RESET_FAILED(SMSG_INSTANCE_RESET_FAILED::astd_read_body(r, size).await?)),
            0x0320 => Ok(Self::SMSG_UPDATE_LAST_INSTANCE(SMSG_UPDATE_LAST_INSTANCE::astd_read_body(r, size).await?)),
            0x0321 => Ok(Self::MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Server::astd_read_body(r, size).await?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Server::astd_read_body(r, size).await?)),
            0x0324 => Ok(Self::SMSG_PET_ACTION_SOUND(SMSG_PET_ACTION_SOUND::astd_read_body(r, size).await?)),
            0x0325 => Ok(Self::SMSG_PET_DISMISS_SOUND(SMSG_PET_DISMISS_SOUND::astd_read_body(r, size).await?)),
            0x0328 => Ok(Self::SMSG_GM_TICKET_STATUS_UPDATE(SMSG_GM_TICKET_STATUS_UPDATE::astd_read_body(r, size).await?)),
            0x032B => Ok(Self::SMSG_UPDATE_INSTANCE_OWNERSHIP(SMSG_UPDATE_INSTANCE_OWNERSHIP::astd_read_body(r, size).await?)),
            0x032F => Ok(Self::SMSG_SPELLINSTAKILLLOG(SMSG_SPELLINSTAKILLLOG::astd_read_body(r, size).await?)),
            0x0330 => Ok(Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS(SMSG_SPELL_UPDATE_CHAIN_TARGETS::astd_read_body(r, size).await?)),
            0x0332 => Ok(Self::SMSG_EXPECTED_SPAM_RECORDS(SMSG_EXPECTED_SPAM_RECORDS::astd_read_body(r, size).await?)),
            0x033B => Ok(Self::SMSG_DEFENSE_MESSAGE(SMSG_DEFENSE_MESSAGE::astd_read_body(r, size).await?)),
            _ => Err(ServerOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }
    #[cfg(feature = "async-std")]
    pub async fn astd_read_encrypted<R: ReadExt + Unpin + Send, D: Decrypter + Send>(r: &mut R, d: &mut D) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let mut header = [0u8; 4];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_server_header(header);
        let header_size = (header.size - 2) as u32;
        match header.opcode {
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD::astd_read_body(r, header_size).await?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD::astd_read_body(r, header_size).await?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(MSG_MOVE_STOP::astd_read_body(r, header_size).await?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT::astd_read_body(r, header_size).await?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT::astd_read_body(r, header_size).await?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE::astd_read_body(r, header_size).await?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(MSG_MOVE_JUMP::astd_read_body(r, header_size).await?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT::astd_read_body(r, header_size).await?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT::astd_read_body(r, header_size).await?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN::astd_read_body(r, header_size).await?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP::astd_read_body(r, header_size).await?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN::astd_read_body(r, header_size).await?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH::astd_read_body(r, header_size).await?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE::astd_read_body(r, header_size).await?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE::astd_read_body(r, header_size).await?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK::astd_read_body(r, header_size).await?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND::astd_read_body(r, header_size).await?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM::astd_read_body(r, header_size).await?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM::astd_read_body(r, header_size).await?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING::astd_read_body(r, header_size).await?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH::astd_read_body(r, header_size).await?)),
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK::astd_read_body(r, header_size).await?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT::astd_read_body(r, header_size).await?)),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(MSG_PETITION_DECLINE::astd_read_body(r, header_size).await?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE::astd_read_body(r, header_size).await?)),
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT::astd_read_body(r, header_size).await?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(MSG_PETITION_RENAME::astd_read_body(r, header_size).await?)),
            0x003A => Ok(Self::SMSG_CHAR_CREATE(SMSG_CHAR_CREATE::astd_read_body(r, header_size).await?)),
            0x003B => Ok(Self::SMSG_CHAR_ENUM(SMSG_CHAR_ENUM::astd_read_body(r, header_size).await?)),
            0x003C => Ok(Self::SMSG_CHAR_DELETE(SMSG_CHAR_DELETE::astd_read_body(r, header_size).await?)),
            0x003E => Ok(Self::SMSG_NEW_WORLD(SMSG_NEW_WORLD::astd_read_body(r, header_size).await?)),
            0x003F => Ok(Self::SMSG_TRANSFER_PENDING(SMSG_TRANSFER_PENDING::astd_read_body(r, header_size).await?)),
            0x0040 => Ok(Self::SMSG_TRANSFER_ABORTED(SMSG_TRANSFER_ABORTED::astd_read_body(r, header_size).await?)),
            0x0041 => Ok(Self::SMSG_CHARACTER_LOGIN_FAILED(SMSG_CHARACTER_LOGIN_FAILED::astd_read_body(r, header_size).await?)),
            0x0042 => Ok(Self::SMSG_LOGIN_SETTIMESPEED(SMSG_LOGIN_SETTIMESPEED::astd_read_body(r, header_size).await?)),
            0x004C => Ok(Self::SMSG_LOGOUT_RESPONSE(SMSG_LOGOUT_RESPONSE::astd_read_body(r, header_size).await?)),
            0x004D => Ok(Self::SMSG_LOGOUT_COMPLETE(SMSG_LOGOUT_COMPLETE::astd_read_body(r, header_size).await?)),
            0x004F => Ok(Self::SMSG_LOGOUT_CANCEL_ACK(SMSG_LOGOUT_CANCEL_ACK::astd_read_body(r, header_size).await?)),
            0x0051 => Ok(Self::SMSG_NAME_QUERY_RESPONSE(SMSG_NAME_QUERY_RESPONSE::astd_read_body(r, header_size).await?)),
            0x0053 => Ok(Self::SMSG_PET_NAME_QUERY_RESPONSE(SMSG_PET_NAME_QUERY_RESPONSE::astd_read_body(r, header_size).await?)),
            0x0055 => Ok(Self::SMSG_GUILD_QUERY_RESPONSE(SMSG_GUILD_QUERY_RESPONSE::astd_read_body(r, header_size).await?)),
            0x0058 => Ok(Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(SMSG_ITEM_QUERY_SINGLE_RESPONSE::astd_read_body(r, header_size).await?)),
            0x005B => Ok(Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(SMSG_PAGE_TEXT_QUERY_RESPONSE::astd_read_body(r, header_size).await?)),
            0x005D => Ok(Self::SMSG_QUEST_QUERY_RESPONSE(SMSG_QUEST_QUERY_RESPONSE::astd_read_body(r, header_size).await?)),
            0x005F => Ok(Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(SMSG_GAMEOBJECT_QUERY_RESPONSE::astd_read_body(r, header_size).await?)),
            0x0061 => Ok(Self::SMSG_CREATURE_QUERY_RESPONSE(SMSG_CREATURE_QUERY_RESPONSE::astd_read_body(r, header_size).await?)),
            0x0063 => Ok(Self::SMSG_WHO(SMSG_WHO::astd_read_body(r, header_size).await?)),
            0x0065 => Ok(Self::SMSG_WHOIS(SMSG_WHOIS::astd_read_body(r, header_size).await?)),
            0x0067 => Ok(Self::SMSG_FRIEND_LIST(SMSG_FRIEND_LIST::astd_read_body(r, header_size).await?)),
            0x0068 => Ok(Self::SMSG_FRIEND_STATUS(SMSG_FRIEND_STATUS::astd_read_body(r, header_size).await?)),
            0x006B => Ok(Self::SMSG_IGNORE_LIST(SMSG_IGNORE_LIST::astd_read_body(r, header_size).await?)),
            0x006F => Ok(Self::SMSG_GROUP_INVITE(SMSG_GROUP_INVITE::astd_read_body(r, header_size).await?)),
            0x0074 => Ok(Self::SMSG_GROUP_DECLINE(SMSG_GROUP_DECLINE::astd_read_body(r, header_size).await?)),
            0x0077 => Ok(Self::SMSG_GROUP_UNINVITE(SMSG_GROUP_UNINVITE::astd_read_body(r, header_size).await?)),
            0x0079 => Ok(Self::SMSG_GROUP_SET_LEADER(SMSG_GROUP_SET_LEADER::astd_read_body(r, header_size).await?)),
            0x007C => Ok(Self::SMSG_GROUP_DESTROYED(SMSG_GROUP_DESTROYED::astd_read_body(r, header_size).await?)),
            0x007D => Ok(Self::SMSG_GROUP_LIST(SMSG_GROUP_LIST::astd_read_body(r, header_size).await?)),
            0x007E => Ok(Self::SMSG_PARTY_MEMBER_STATS(SMSG_PARTY_MEMBER_STATS::astd_read_body(r, header_size).await?)),
            0x007F => Ok(Self::SMSG_PARTY_COMMAND_RESULT(SMSG_PARTY_COMMAND_RESULT::astd_read_body(r, header_size).await?)),
            0x0083 => Ok(Self::SMSG_GUILD_INVITE(SMSG_GUILD_INVITE::astd_read_body(r, header_size).await?)),
            0x0088 => Ok(Self::SMSG_GUILD_INFO(SMSG_GUILD_INFO::astd_read_body(r, header_size).await?)),
            0x008A => Ok(Self::SMSG_GUILD_ROSTER(SMSG_GUILD_ROSTER::astd_read_body(r, header_size).await?)),
            0x0092 => Ok(Self::SMSG_GUILD_EVENT(SMSG_GUILD_EVENT::astd_read_body(r, header_size).await?)),
            0x0093 => Ok(Self::SMSG_GUILD_COMMAND_RESULT(SMSG_GUILD_COMMAND_RESULT::astd_read_body(r, header_size).await?)),
            0x0096 => Ok(Self::SMSG_MESSAGECHAT(SMSG_MESSAGECHAT::astd_read_body(r, header_size).await?)),
            0x0099 => Ok(Self::SMSG_CHANNEL_NOTIFY(SMSG_CHANNEL_NOTIFY::astd_read_body(r, header_size).await?)),
            0x009B => Ok(Self::SMSG_CHANNEL_LIST(SMSG_CHANNEL_LIST::astd_read_body(r, header_size).await?)),
            0x00AA => Ok(Self::SMSG_DESTROY_OBJECT(SMSG_DESTROY_OBJECT::astd_read_body(r, header_size).await?)),
            0x00AE => Ok(Self::SMSG_READ_ITEM_OK(SMSG_READ_ITEM_OK::astd_read_body(r, header_size).await?)),
            0x00AF => Ok(Self::SMSG_READ_ITEM_FAILED(SMSG_READ_ITEM_FAILED::astd_read_body(r, header_size).await?)),
            0x00B0 => Ok(Self::SMSG_ITEM_COOLDOWN(SMSG_ITEM_COOLDOWN::astd_read_body(r, header_size).await?)),
            0x00B3 => Ok(Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(SMSG_GAMEOBJECT_CUSTOM_ANIM::astd_read_body(r, header_size).await?)),
            0x00DD => Ok(Self::SMSG_MONSTER_MOVE(SMSG_MONSTER_MOVE::astd_read_body(r, header_size).await?)),
            0x00DE => Ok(Self::SMSG_MOVE_WATER_WALK(SMSG_MOVE_WATER_WALK::astd_read_body(r, header_size).await?)),
            0x00DF => Ok(Self::SMSG_MOVE_LAND_WALK(SMSG_MOVE_LAND_WALK::astd_read_body(r, header_size).await?)),
            0x00E2 => Ok(Self::SMSG_FORCE_RUN_SPEED_CHANGE(SMSG_FORCE_RUN_SPEED_CHANGE::astd_read_body(r, header_size).await?)),
            0x00E4 => Ok(Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(SMSG_FORCE_RUN_BACK_SPEED_CHANGE::astd_read_body(r, header_size).await?)),
            0x00E6 => Ok(Self::SMSG_FORCE_SWIM_SPEED_CHANGE(SMSG_FORCE_SWIM_SPEED_CHANGE::astd_read_body(r, header_size).await?)),
            0x00E8 => Ok(Self::SMSG_FORCE_MOVE_ROOT(SMSG_FORCE_MOVE_ROOT::astd_read_body(r, header_size).await?)),
            0x00EA => Ok(Self::SMSG_FORCE_MOVE_UNROOT(SMSG_FORCE_MOVE_UNROOT::astd_read_body(r, header_size).await?)),
            0x00EF => Ok(Self::SMSG_MOVE_KNOCK_BACK(SMSG_MOVE_KNOCK_BACK::astd_read_body(r, header_size).await?)),
            0x00F2 => Ok(Self::SMSG_MOVE_FEATHER_FALL(SMSG_MOVE_FEATHER_FALL::astd_read_body(r, header_size).await?)),
            0x00F3 => Ok(Self::SMSG_MOVE_NORMAL_FALL(SMSG_MOVE_NORMAL_FALL::astd_read_body(r, header_size).await?)),
            0x00F4 => Ok(Self::SMSG_MOVE_SET_HOVER(SMSG_MOVE_SET_HOVER::astd_read_body(r, header_size).await?)),
            0x00F5 => Ok(Self::SMSG_MOVE_UNSET_HOVER(SMSG_MOVE_UNSET_HOVER::astd_read_body(r, header_size).await?)),
            0x00FA => Ok(Self::SMSG_TRIGGER_CINEMATIC(SMSG_TRIGGER_CINEMATIC::astd_read_body(r, header_size).await?)),
            0x00FD => Ok(Self::SMSG_TUTORIAL_FLAGS(SMSG_TUTORIAL_FLAGS::astd_read_body(r, header_size).await?)),
            0x0103 => Ok(Self::SMSG_EMOTE(SMSG_EMOTE::astd_read_body(r, header_size).await?)),
            0x0105 => Ok(Self::SMSG_TEXT_EMOTE(SMSG_TEXT_EMOTE::astd_read_body(r, header_size).await?)),
            0x0112 => Ok(Self::SMSG_INVENTORY_CHANGE_FAILURE(SMSG_INVENTORY_CHANGE_FAILURE::astd_read_body(r, header_size).await?)),
            0x0113 => Ok(Self::SMSG_OPEN_CONTAINER(SMSG_OPEN_CONTAINER::astd_read_body(r, header_size).await?)),
            0x0115 => Ok(Self::SMSG_INSPECT(SMSG_INSPECT::astd_read_body(r, header_size).await?)),
            0x0120 => Ok(Self::SMSG_TRADE_STATUS(SMSG_TRADE_STATUS::astd_read_body(r, header_size).await?)),
            0x0121 => Ok(Self::SMSG_TRADE_STATUS_EXTENDED(SMSG_TRADE_STATUS_EXTENDED::astd_read_body(r, header_size).await?)),
            0x0122 => Ok(Self::SMSG_INITIALIZE_FACTIONS(SMSG_INITIALIZE_FACTIONS::astd_read_body(r, header_size).await?)),
            0x0123 => Ok(Self::SMSG_SET_FACTION_VISIBLE(SMSG_SET_FACTION_VISIBLE::astd_read_body(r, header_size).await?)),
            0x0124 => Ok(Self::SMSG_SET_FACTION_STANDING(SMSG_SET_FACTION_STANDING::astd_read_body(r, header_size).await?)),
            0x0127 => Ok(Self::SMSG_SET_PROFICIENCY(SMSG_SET_PROFICIENCY::astd_read_body(r, header_size).await?)),
            0x0129 => Ok(Self::SMSG_ACTION_BUTTONS(SMSG_ACTION_BUTTONS::astd_read_body(r, header_size).await?)),
            0x012A => Ok(Self::SMSG_INITIAL_SPELLS(SMSG_INITIAL_SPELLS::astd_read_body(r, header_size).await?)),
            0x012B => Ok(Self::SMSG_LEARNED_SPELL(SMSG_LEARNED_SPELL::astd_read_body(r, header_size).await?)),
            0x012C => Ok(Self::SMSG_SUPERCEDED_SPELL(SMSG_SUPERCEDED_SPELL::astd_read_body(r, header_size).await?)),
            0x0130 => Ok(Self::SMSG_CAST_RESULT(SMSG_CAST_RESULT::astd_read_body(r, header_size).await?)),
            0x0131 => Ok(Self::SMSG_SPELL_START(SMSG_SPELL_START::astd_read_body(r, header_size).await?)),
            0x0132 => Ok(Self::SMSG_SPELL_GO(SMSG_SPELL_GO::astd_read_body(r, header_size).await?)),
            0x0133 => Ok(Self::SMSG_SPELL_FAILURE(SMSG_SPELL_FAILURE::astd_read_body(r, header_size).await?)),
            0x0134 => Ok(Self::SMSG_SPELL_COOLDOWN(SMSG_SPELL_COOLDOWN::astd_read_body(r, header_size).await?)),
            0x0135 => Ok(Self::SMSG_COOLDOWN_EVENT(SMSG_COOLDOWN_EVENT::astd_read_body(r, header_size).await?)),
            0x0137 => Ok(Self::SMSG_UPDATE_AURA_DURATION(SMSG_UPDATE_AURA_DURATION::astd_read_body(r, header_size).await?)),
            0x0138 => Ok(Self::SMSG_PET_CAST_FAILED(SMSG_PET_CAST_FAILED::astd_read_body(r, header_size).await?)),
            0x013C => Ok(Self::SMSG_AI_REACTION(SMSG_AI_REACTION::astd_read_body(r, header_size).await?)),
            0x0143 => Ok(Self::SMSG_ATTACKSTART(SMSG_ATTACKSTART::astd_read_body(r, header_size).await?)),
            0x0144 => Ok(Self::SMSG_ATTACKSTOP(SMSG_ATTACKSTOP::astd_read_body(r, header_size).await?)),
            0x0145 => Ok(Self::SMSG_ATTACKSWING_NOTINRANGE(SMSG_ATTACKSWING_NOTINRANGE::astd_read_body(r, header_size).await?)),
            0x0146 => Ok(Self::SMSG_ATTACKSWING_BADFACING(SMSG_ATTACKSWING_BADFACING::astd_read_body(r, header_size).await?)),
            0x0147 => Ok(Self::SMSG_ATTACKSWING_NOTSTANDING(SMSG_ATTACKSWING_NOTSTANDING::astd_read_body(r, header_size).await?)),
            0x0148 => Ok(Self::SMSG_ATTACKSWING_DEADTARGET(SMSG_ATTACKSWING_DEADTARGET::astd_read_body(r, header_size).await?)),
            0x0149 => Ok(Self::SMSG_ATTACKSWING_CANT_ATTACK(SMSG_ATTACKSWING_CANT_ATTACK::astd_read_body(r, header_size).await?)),
            0x014A => Ok(Self::SMSG_ATTACKERSTATEUPDATE(SMSG_ATTACKERSTATEUPDATE::astd_read_body(r, header_size).await?)),
            0x014E => Ok(Self::SMSG_CANCEL_COMBAT(SMSG_CANCEL_COMBAT::astd_read_body(r, header_size).await?)),
            0x0150 => Ok(Self::SMSG_SPELLHEALLOG(SMSG_SPELLHEALLOG::astd_read_body(r, header_size).await?)),
            0x0151 => Ok(Self::SMSG_SPELLENERGIZELOG(SMSG_SPELLENERGIZELOG::astd_read_body(r, header_size).await?)),
            0x0155 => Ok(Self::SMSG_BINDPOINTUPDATE(SMSG_BINDPOINTUPDATE::astd_read_body(r, header_size).await?)),
            0x0158 => Ok(Self::SMSG_PLAYERBOUND(SMSG_PLAYERBOUND::astd_read_body(r, header_size).await?)),
            0x0159 => Ok(Self::SMSG_CLIENT_CONTROL_UPDATE(SMSG_CLIENT_CONTROL_UPDATE::astd_read_body(r, header_size).await?)),
            0x015B => Ok(Self::SMSG_RESURRECT_REQUEST(SMSG_RESURRECT_REQUEST::astd_read_body(r, header_size).await?)),
            0x0160 => Ok(Self::SMSG_LOOT_RESPONSE(SMSG_LOOT_RESPONSE::astd_read_body(r, header_size).await?)),
            0x0161 => Ok(Self::SMSG_LOOT_RELEASE_RESPONSE(SMSG_LOOT_RELEASE_RESPONSE::astd_read_body(r, header_size).await?)),
            0x0162 => Ok(Self::SMSG_LOOT_REMOVED(SMSG_LOOT_REMOVED::astd_read_body(r, header_size).await?)),
            0x0163 => Ok(Self::SMSG_LOOT_MONEY_NOTIFY(SMSG_LOOT_MONEY_NOTIFY::astd_read_body(r, header_size).await?)),
            0x0165 => Ok(Self::SMSG_LOOT_CLEAR_MONEY(SMSG_LOOT_CLEAR_MONEY::astd_read_body(r, header_size).await?)),
            0x0166 => Ok(Self::SMSG_ITEM_PUSH_RESULT(SMSG_ITEM_PUSH_RESULT::astd_read_body(r, header_size).await?)),
            0x0167 => Ok(Self::SMSG_DUEL_REQUESTED(SMSG_DUEL_REQUESTED::astd_read_body(r, header_size).await?)),
            0x0168 => Ok(Self::SMSG_DUEL_OUTOFBOUNDS(SMSG_DUEL_OUTOFBOUNDS::astd_read_body(r, header_size).await?)),
            0x0169 => Ok(Self::SMSG_DUEL_INBOUNDS(SMSG_DUEL_INBOUNDS::astd_read_body(r, header_size).await?)),
            0x016A => Ok(Self::SMSG_DUEL_COMPLETE(SMSG_DUEL_COMPLETE::astd_read_body(r, header_size).await?)),
            0x016B => Ok(Self::SMSG_DUEL_WINNER(SMSG_DUEL_WINNER::astd_read_body(r, header_size).await?)),
            0x016E => Ok(Self::SMSG_MOUNTRESULT(SMSG_MOUNTRESULT::astd_read_body(r, header_size).await?)),
            0x016F => Ok(Self::SMSG_DISMOUNTRESULT(SMSG_DISMOUNTRESULT::astd_read_body(r, header_size).await?)),
            0x0172 => Ok(Self::SMSG_MOUNTSPECIAL_ANIM(SMSG_MOUNTSPECIAL_ANIM::astd_read_body(r, header_size).await?)),
            0x0173 => Ok(Self::SMSG_PET_TAME_FAILURE(SMSG_PET_TAME_FAILURE::astd_read_body(r, header_size).await?)),
            0x0178 => Ok(Self::SMSG_PET_NAME_INVALID(SMSG_PET_NAME_INVALID::astd_read_body(r, header_size).await?)),
            0x0179 => Ok(Self::SMSG_PET_SPELLS(SMSG_PET_SPELLS::astd_read_body(r, header_size).await?)),
            0x017A => Ok(Self::SMSG_PET_MODE(SMSG_PET_MODE::astd_read_body(r, header_size).await?)),
            0x017D => Ok(Self::SMSG_GOSSIP_MESSAGE(SMSG_GOSSIP_MESSAGE::astd_read_body(r, header_size).await?)),
            0x017E => Ok(Self::SMSG_GOSSIP_COMPLETE(SMSG_GOSSIP_COMPLETE::astd_read_body(r, header_size).await?)),
            0x0180 => Ok(Self::SMSG_NPC_TEXT_UPDATE(SMSG_NPC_TEXT_UPDATE::astd_read_body(r, header_size).await?)),
            0x0183 => Ok(Self::SMSG_QUESTGIVER_STATUS(SMSG_QUESTGIVER_STATUS::astd_read_body(r, header_size).await?)),
            0x0185 => Ok(Self::SMSG_QUESTGIVER_QUEST_LIST(SMSG_QUESTGIVER_QUEST_LIST::astd_read_body(r, header_size).await?)),
            0x0188 => Ok(Self::SMSG_QUESTGIVER_QUEST_DETAILS(SMSG_QUESTGIVER_QUEST_DETAILS::astd_read_body(r, header_size).await?)),
            0x018B => Ok(Self::SMSG_QUESTGIVER_REQUEST_ITEMS(SMSG_QUESTGIVER_REQUEST_ITEMS::astd_read_body(r, header_size).await?)),
            0x018D => Ok(Self::SMSG_QUESTGIVER_OFFER_REWARD(SMSG_QUESTGIVER_OFFER_REWARD::astd_read_body(r, header_size).await?)),
            0x018F => Ok(Self::SMSG_QUESTGIVER_QUEST_INVALID(SMSG_QUESTGIVER_QUEST_INVALID::astd_read_body(r, header_size).await?)),
            0x0191 => Ok(Self::SMSG_QUESTGIVER_QUEST_COMPLETE(SMSG_QUESTGIVER_QUEST_COMPLETE::astd_read_body(r, header_size).await?)),
            0x0192 => Ok(Self::SMSG_QUESTGIVER_QUEST_FAILED(SMSG_QUESTGIVER_QUEST_FAILED::astd_read_body(r, header_size).await?)),
            0x0195 => Ok(Self::SMSG_QUESTLOG_FULL(SMSG_QUESTLOG_FULL::astd_read_body(r, header_size).await?)),
            0x0196 => Ok(Self::SMSG_QUESTUPDATE_FAILED(SMSG_QUESTUPDATE_FAILED::astd_read_body(r, header_size).await?)),
            0x0197 => Ok(Self::SMSG_QUESTUPDATE_FAILEDTIMER(SMSG_QUESTUPDATE_FAILEDTIMER::astd_read_body(r, header_size).await?)),
            0x0198 => Ok(Self::SMSG_QUESTUPDATE_COMPLETE(SMSG_QUESTUPDATE_COMPLETE::astd_read_body(r, header_size).await?)),
            0x0199 => Ok(Self::SMSG_QUESTUPDATE_ADD_KILL(SMSG_QUESTUPDATE_ADD_KILL::astd_read_body(r, header_size).await?)),
            0x019A => Ok(Self::SMSG_QUESTUPDATE_ADD_ITEM(SMSG_QUESTUPDATE_ADD_ITEM::astd_read_body(r, header_size).await?)),
            0x019C => Ok(Self::SMSG_QUEST_CONFIRM_ACCEPT(SMSG_QUEST_CONFIRM_ACCEPT::astd_read_body(r, header_size).await?)),
            0x019F => Ok(Self::SMSG_LIST_INVENTORY(SMSG_LIST_INVENTORY::astd_read_body(r, header_size).await?)),
            0x01A1 => Ok(Self::SMSG_SELL_ITEM(SMSG_SELL_ITEM::astd_read_body(r, header_size).await?)),
            0x01A4 => Ok(Self::SMSG_BUY_ITEM(SMSG_BUY_ITEM::astd_read_body(r, header_size).await?)),
            0x01A5 => Ok(Self::SMSG_BUY_FAILED(SMSG_BUY_FAILED::astd_read_body(r, header_size).await?)),
            0x01A9 => Ok(Self::SMSG_SHOWTAXINODES(SMSG_SHOWTAXINODES::astd_read_body(r, header_size).await?)),
            0x01AB => Ok(Self::SMSG_TAXINODE_STATUS(SMSG_TAXINODE_STATUS::astd_read_body(r, header_size).await?)),
            0x01AE => Ok(Self::SMSG_ACTIVATETAXIREPLY(SMSG_ACTIVATETAXIREPLY::astd_read_body(r, header_size).await?)),
            0x01AF => Ok(Self::SMSG_NEW_TAXI_PATH(SMSG_NEW_TAXI_PATH::astd_read_body(r, header_size).await?)),
            0x01B1 => Ok(Self::SMSG_TRAINER_LIST(SMSG_TRAINER_LIST::astd_read_body(r, header_size).await?)),
            0x01B3 => Ok(Self::SMSG_TRAINER_BUY_SUCCEEDED(SMSG_TRAINER_BUY_SUCCEEDED::astd_read_body(r, header_size).await?)),
            0x01B4 => Ok(Self::SMSG_TRAINER_BUY_FAILED(SMSG_TRAINER_BUY_FAILED::astd_read_body(r, header_size).await?)),
            0x01B8 => Ok(Self::SMSG_SHOW_BANK(SMSG_SHOW_BANK::astd_read_body(r, header_size).await?)),
            0x01BA => Ok(Self::SMSG_BUY_BANK_SLOT_RESULT(SMSG_BUY_BANK_SLOT_RESULT::astd_read_body(r, header_size).await?)),
            0x01BC => Ok(Self::SMSG_PETITION_SHOWLIST(SMSG_PETITION_SHOWLIST::astd_read_body(r, header_size).await?)),
            0x01BF => Ok(Self::SMSG_PETITION_SHOW_SIGNATURES(SMSG_PETITION_SHOW_SIGNATURES::astd_read_body(r, header_size).await?)),
            0x01C1 => Ok(Self::SMSG_PETITION_SIGN_RESULTS(SMSG_PETITION_SIGN_RESULTS::astd_read_body(r, header_size).await?)),
            0x01C5 => Ok(Self::SMSG_TURN_IN_PETITION_RESULTS(SMSG_TURN_IN_PETITION_RESULTS::astd_read_body(r, header_size).await?)),
            0x01C7 => Ok(Self::SMSG_PETITION_QUERY_RESPONSE(SMSG_PETITION_QUERY_RESPONSE::astd_read_body(r, header_size).await?)),
            0x01C8 => Ok(Self::SMSG_FISH_NOT_HOOKED(SMSG_FISH_NOT_HOOKED::astd_read_body(r, header_size).await?)),
            0x01C9 => Ok(Self::SMSG_FISH_ESCAPED(SMSG_FISH_ESCAPED::astd_read_body(r, header_size).await?)),
            0x01CB => Ok(Self::SMSG_NOTIFICATION(SMSG_NOTIFICATION::astd_read_body(r, header_size).await?)),
            0x01CD => Ok(Self::SMSG_PLAYED_TIME(SMSG_PLAYED_TIME::astd_read_body(r, header_size).await?)),
            0x01CF => Ok(Self::SMSG_QUERY_TIME_RESPONSE(SMSG_QUERY_TIME_RESPONSE::astd_read_body(r, header_size).await?)),
            0x01D0 => Ok(Self::SMSG_LOG_XPGAIN(SMSG_LOG_XPGAIN::astd_read_body(r, header_size).await?)),
            0x01D4 => Ok(Self::SMSG_LEVELUP_INFO(SMSG_LEVELUP_INFO::astd_read_body(r, header_size).await?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(MSG_MINIMAP_PING_Server::astd_read_body(r, header_size).await?)),
            0x01D6 => Ok(Self::SMSG_RESISTLOG(SMSG_RESISTLOG::astd_read_body(r, header_size).await?)),
            0x01D7 => Ok(Self::SMSG_ENCHANTMENTLOG(SMSG_ENCHANTMENTLOG::astd_read_body(r, header_size).await?)),
            0x01D9 => Ok(Self::SMSG_START_MIRROR_TIMER(SMSG_START_MIRROR_TIMER::astd_read_body(r, header_size).await?)),
            0x01DA => Ok(Self::SMSG_PAUSE_MIRROR_TIMER(SMSG_PAUSE_MIRROR_TIMER::astd_read_body(r, header_size).await?)),
            0x01DB => Ok(Self::SMSG_STOP_MIRROR_TIMER(SMSG_STOP_MIRROR_TIMER::astd_read_body(r, header_size).await?)),
            0x01DD => Ok(Self::SMSG_PONG(SMSG_PONG::astd_read_body(r, header_size).await?)),
            0x01DE => Ok(Self::SMSG_CLEAR_COOLDOWN(SMSG_CLEAR_COOLDOWN::astd_read_body(r, header_size).await?)),
            0x01DF => Ok(Self::SMSG_GAMEOBJECT_PAGETEXT(SMSG_GAMEOBJECT_PAGETEXT::astd_read_body(r, header_size).await?)),
            0x01E2 => Ok(Self::SMSG_SPELL_DELAYED(SMSG_SPELL_DELAYED::astd_read_body(r, header_size).await?)),
            0x01EA => Ok(Self::SMSG_ITEM_TIME_UPDATE(SMSG_ITEM_TIME_UPDATE::astd_read_body(r, header_size).await?)),
            0x01EB => Ok(Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(SMSG_ITEM_ENCHANT_TIME_UPDATE::astd_read_body(r, header_size).await?)),
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE::astd_read_body(r, header_size).await?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE::astd_read_body(r, header_size).await?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Server::astd_read_body(r, header_size).await?)),
            0x01F3 => Ok(Self::SMSG_PLAY_SPELL_VISUAL(SMSG_PLAY_SPELL_VISUAL::astd_read_body(r, header_size).await?)),
            0x01F5 => Ok(Self::SMSG_PARTYKILLLOG(SMSG_PARTYKILLLOG::astd_read_body(r, header_size).await?)),
            0x01F7 => Ok(Self::SMSG_PLAY_SPELL_IMPACT(SMSG_PLAY_SPELL_IMPACT::astd_read_body(r, header_size).await?)),
            0x01F8 => Ok(Self::SMSG_EXPLORATION_EXPERIENCE(SMSG_EXPLORATION_EXPERIENCE::astd_read_body(r, header_size).await?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Server::astd_read_body(r, header_size).await?)),
            0x01FC => Ok(Self::SMSG_ENVIRONMENTALDAMAGELOG(SMSG_ENVIRONMENTALDAMAGELOG::astd_read_body(r, header_size).await?)),
            0x01FF => Ok(Self::MSG_LOOKING_FOR_GROUP(MSG_LOOKING_FOR_GROUP_Server::astd_read_body(r, header_size).await?)),
            0x0203 => Ok(Self::SMSG_REMOVED_SPELL(SMSG_REMOVED_SPELL::astd_read_body(r, header_size).await?)),
            0x0206 => Ok(Self::SMSG_GMTICKET_CREATE(SMSG_GMTICKET_CREATE::astd_read_body(r, header_size).await?)),
            0x0208 => Ok(Self::SMSG_GMTICKET_UPDATETEXT(SMSG_GMTICKET_UPDATETEXT::astd_read_body(r, header_size).await?)),
            0x0209 => Ok(Self::SMSG_ACCOUNT_DATA_TIMES(SMSG_ACCOUNT_DATA_TIMES::astd_read_body(r, header_size).await?)),
            0x0212 => Ok(Self::SMSG_GMTICKET_GETTICKET(SMSG_GMTICKET_GETTICKET::astd_read_body(r, header_size).await?)),
            0x0214 => Ok(Self::SMSG_GAMEOBJECT_SPAWN_ANIM(SMSG_GAMEOBJECT_SPAWN_ANIM::astd_read_body(r, header_size).await?)),
            0x0215 => Ok(Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(SMSG_GAMEOBJECT_DESPAWN_ANIM::astd_read_body(r, header_size).await?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Server::astd_read_body(r, header_size).await?)),
            0x0218 => Ok(Self::SMSG_GMTICKET_DELETETICKET(SMSG_GMTICKET_DELETETICKET::astd_read_body(r, header_size).await?)),
            0x0219 => Ok(Self::SMSG_CHAT_WRONG_FACTION(SMSG_CHAT_WRONG_FACTION::astd_read_body(r, header_size).await?)),
            0x021B => Ok(Self::SMSG_GMTICKET_SYSTEMSTATUS(SMSG_GMTICKET_SYSTEMSTATUS::astd_read_body(r, header_size).await?)),
            0x021E => Ok(Self::SMSG_SET_REST_START(SMSG_SET_REST_START::astd_read_body(r, header_size).await?)),
            0x0222 => Ok(Self::SMSG_SPIRIT_HEALER_CONFIRM(SMSG_SPIRIT_HEALER_CONFIRM::astd_read_body(r, header_size).await?)),
            0x0224 => Ok(Self::SMSG_GOSSIP_POI(SMSG_GOSSIP_POI::astd_read_body(r, header_size).await?)),
            0x0236 => Ok(Self::SMSG_LOGIN_VERIFY_WORLD(SMSG_LOGIN_VERIFY_WORLD::astd_read_body(r, header_size).await?)),
            0x023B => Ok(Self::SMSG_MAIL_LIST_RESULT(SMSG_MAIL_LIST_RESULT::astd_read_body(r, header_size).await?)),
            0x023D => Ok(Self::SMSG_BATTLEFIELD_LIST(SMSG_BATTLEFIELD_LIST::astd_read_body(r, header_size).await?)),
            0x0244 => Ok(Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(SMSG_ITEM_TEXT_QUERY_RESPONSE::astd_read_body(r, header_size).await?)),
            0x024B => Ok(Self::SMSG_SPELLLOGMISS(SMSG_SPELLLOGMISS::astd_read_body(r, header_size).await?)),
            0x024C => Ok(Self::SMSG_SPELLLOGEXECUTE(SMSG_SPELLLOGEXECUTE::astd_read_body(r, header_size).await?)),
            0x024E => Ok(Self::SMSG_PERIODICAURALOG(SMSG_PERIODICAURALOG::astd_read_body(r, header_size).await?)),
            0x024F => Ok(Self::SMSG_SPELLDAMAGESHIELD(SMSG_SPELLDAMAGESHIELD::astd_read_body(r, header_size).await?)),
            0x0250 => Ok(Self::SMSG_SPELLNONMELEEDAMAGELOG(SMSG_SPELLNONMELEEDAMAGELOG::astd_read_body(r, header_size).await?)),
            0x0254 => Ok(Self::SMSG_ZONE_UNDER_ATTACK(SMSG_ZONE_UNDER_ATTACK::astd_read_body(r, header_size).await?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Server::astd_read_body(r, header_size).await?)),
            0x025C => Ok(Self::SMSG_AUCTION_LIST_RESULT(SMSG_AUCTION_LIST_RESULT::astd_read_body(r, header_size).await?)),
            0x025D => Ok(Self::SMSG_AUCTION_OWNER_LIST_RESULT(SMSG_AUCTION_OWNER_LIST_RESULT::astd_read_body(r, header_size).await?)),
            0x025E => Ok(Self::SMSG_AUCTION_BIDDER_NOTIFICATION(SMSG_AUCTION_BIDDER_NOTIFICATION::astd_read_body(r, header_size).await?)),
            0x025F => Ok(Self::SMSG_AUCTION_OWNER_NOTIFICATION(SMSG_AUCTION_OWNER_NOTIFICATION::astd_read_body(r, header_size).await?)),
            0x0260 => Ok(Self::SMSG_PROCRESIST(SMSG_PROCRESIST::astd_read_body(r, header_size).await?)),
            0x0262 => Ok(Self::SMSG_DISPEL_FAILED(SMSG_DISPEL_FAILED::astd_read_body(r, header_size).await?)),
            0x0263 => Ok(Self::SMSG_SPELLORDAMAGE_IMMUNE(SMSG_SPELLORDAMAGE_IMMUNE::astd_read_body(r, header_size).await?)),
            0x0265 => Ok(Self::SMSG_AUCTION_BIDDER_LIST_RESULT(SMSG_AUCTION_BIDDER_LIST_RESULT::astd_read_body(r, header_size).await?)),
            0x0266 => Ok(Self::SMSG_SET_FLAT_SPELL_MODIFIER(SMSG_SET_FLAT_SPELL_MODIFIER::astd_read_body(r, header_size).await?)),
            0x0267 => Ok(Self::SMSG_SET_PCT_SPELL_MODIFIER(SMSG_SET_PCT_SPELL_MODIFIER::astd_read_body(r, header_size).await?)),
            0x0269 => Ok(Self::SMSG_CORPSE_RECLAIM_DELAY(SMSG_CORPSE_RECLAIM_DELAY::astd_read_body(r, header_size).await?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Server::astd_read_body(r, header_size).await?)),
            0x0273 => Ok(Self::SMSG_STABLE_RESULT(SMSG_STABLE_RESULT::astd_read_body(r, header_size).await?)),
            0x0277 => Ok(Self::SMSG_PLAY_MUSIC(SMSG_PLAY_MUSIC::astd_read_body(r, header_size).await?)),
            0x0278 => Ok(Self::SMSG_PLAY_OBJECT_SOUND(SMSG_PLAY_OBJECT_SOUND::astd_read_body(r, header_size).await?)),
            0x027B => Ok(Self::SMSG_SPELLDISPELLOG(SMSG_SPELLDISPELLOG::astd_read_body(r, header_size).await?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Server::astd_read_body(r, header_size).await?)),
            0x0285 => Ok(Self::SMSG_RECEIVED_MAIL(SMSG_RECEIVED_MAIL::astd_read_body(r, header_size).await?)),
            0x0286 => Ok(Self::SMSG_RAID_GROUP_ONLY(SMSG_RAID_GROUP_ONLY::astd_read_body(r, header_size).await?)),
            0x028C => Ok(Self::SMSG_PVP_CREDIT(SMSG_PVP_CREDIT::astd_read_body(r, header_size).await?)),
            0x028D => Ok(Self::SMSG_AUCTION_REMOVED_NOTIFICATION(SMSG_AUCTION_REMOVED_NOTIFICATION::astd_read_body(r, header_size).await?)),
            0x0291 => Ok(Self::SMSG_SERVER_MESSAGE(SMSG_SERVER_MESSAGE::astd_read_body(r, header_size).await?)),
            0x0295 => Ok(Self::SMSG_MEETINGSTONE_SETQUEUE(SMSG_MEETINGSTONE_SETQUEUE::astd_read_body(r, header_size).await?)),
            0x0297 => Ok(Self::SMSG_MEETINGSTONE_COMPLETE(SMSG_MEETINGSTONE_COMPLETE::astd_read_body(r, header_size).await?)),
            0x0298 => Ok(Self::SMSG_MEETINGSTONE_IN_PROGRESS(SMSG_MEETINGSTONE_IN_PROGRESS::astd_read_body(r, header_size).await?)),
            0x0299 => Ok(Self::SMSG_MEETINGSTONE_MEMBER_ADDED(SMSG_MEETINGSTONE_MEMBER_ADDED::astd_read_body(r, header_size).await?)),
            0x029C => Ok(Self::SMSG_CANCEL_AUTO_REPEAT(SMSG_CANCEL_AUTO_REPEAT::astd_read_body(r, header_size).await?)),
            0x029D => Ok(Self::SMSG_STANDSTATE_UPDATE(SMSG_STANDSTATE_UPDATE::astd_read_body(r, header_size).await?)),
            0x029E => Ok(Self::SMSG_LOOT_ALL_PASSED(SMSG_LOOT_ALL_PASSED::astd_read_body(r, header_size).await?)),
            0x029F => Ok(Self::SMSG_LOOT_ROLL_WON(SMSG_LOOT_ROLL_WON::astd_read_body(r, header_size).await?)),
            0x02A1 => Ok(Self::SMSG_LOOT_START_ROLL(SMSG_LOOT_START_ROLL::astd_read_body(r, header_size).await?)),
            0x02A2 => Ok(Self::SMSG_LOOT_ROLL(SMSG_LOOT_ROLL::astd_read_body(r, header_size).await?)),
            0x02A4 => Ok(Self::SMSG_LOOT_MASTER_LIST(SMSG_LOOT_MASTER_LIST::astd_read_body(r, header_size).await?)),
            0x02A5 => Ok(Self::SMSG_SET_FORCED_REACTIONS(SMSG_SET_FORCED_REACTIONS::astd_read_body(r, header_size).await?)),
            0x02A6 => Ok(Self::SMSG_SPELL_FAILED_OTHER(SMSG_SPELL_FAILED_OTHER::astd_read_body(r, header_size).await?)),
            0x02A7 => Ok(Self::SMSG_GAMEOBJECT_RESET_STATE(SMSG_GAMEOBJECT_RESET_STATE::astd_read_body(r, header_size).await?)),
            0x02A9 => Ok(Self::SMSG_CHAT_PLAYER_NOT_FOUND(SMSG_CHAT_PLAYER_NOT_FOUND::astd_read_body(r, header_size).await?)),
            0x02AA => Ok(Self::MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Server::astd_read_body(r, header_size).await?)),
            0x02AB => Ok(Self::SMSG_SUMMON_REQUEST(SMSG_SUMMON_REQUEST::astd_read_body(r, header_size).await?)),
            0x02AE => Ok(Self::SMSG_MONSTER_MOVE_TRANSPORT(SMSG_MONSTER_MOVE_TRANSPORT::astd_read_body(r, header_size).await?)),
            0x02AF => Ok(Self::SMSG_PET_BROKEN(SMSG_PET_BROKEN::astd_read_body(r, header_size).await?)),
            0x02B4 => Ok(Self::SMSG_FEIGN_DEATH_RESISTED(SMSG_FEIGN_DEATH_RESISTED::astd_read_body(r, header_size).await?)),
            0x02B7 => Ok(Self::SMSG_DUEL_COUNTDOWN(SMSG_DUEL_COUNTDOWN::astd_read_body(r, header_size).await?)),
            0x02B8 => Ok(Self::SMSG_AREA_TRIGGER_MESSAGE(SMSG_AREA_TRIGGER_MESSAGE::astd_read_body(r, header_size).await?)),
            0x02BB => Ok(Self::SMSG_MEETINGSTONE_JOINFAILED(SMSG_MEETINGSTONE_JOINFAILED::astd_read_body(r, header_size).await?)),
            0x02BC => Ok(Self::SMSG_PLAYER_SKINNED(SMSG_PLAYER_SKINNED::astd_read_body(r, header_size).await?)),
            0x02BD => Ok(Self::SMSG_DURABILITY_DAMAGE_DEATH(SMSG_DURABILITY_DAMAGE_DEATH::astd_read_body(r, header_size).await?)),
            0x02C2 => Ok(Self::SMSG_INIT_WORLD_STATES(SMSG_INIT_WORLD_STATES::astd_read_body(r, header_size).await?)),
            0x02C3 => Ok(Self::SMSG_UPDATE_WORLD_STATE(SMSG_UPDATE_WORLD_STATE::astd_read_body(r, header_size).await?)),
            0x02C5 => Ok(Self::SMSG_ITEM_NAME_QUERY_RESPONSE(SMSG_ITEM_NAME_QUERY_RESPONSE::astd_read_body(r, header_size).await?)),
            0x02C6 => Ok(Self::SMSG_PET_ACTION_FEEDBACK(SMSG_PET_ACTION_FEEDBACK::astd_read_body(r, header_size).await?)),
            0x02C8 => Ok(Self::SMSG_CHAR_RENAME(SMSG_CHAR_RENAME::astd_read_body(r, header_size).await?)),
            0x02CB => Ok(Self::SMSG_INSTANCE_SAVE_CREATED(SMSG_INSTANCE_SAVE_CREATED::astd_read_body(r, header_size).await?)),
            0x02CC => Ok(Self::SMSG_RAID_INSTANCE_INFO(SMSG_RAID_INSTANCE_INFO::astd_read_body(r, header_size).await?)),
            0x02D2 => Ok(Self::SMSG_PLAY_SOUND(SMSG_PLAY_SOUND::astd_read_body(r, header_size).await?)),
            0x02D4 => Ok(Self::SMSG_BATTLEFIELD_STATUS(SMSG_BATTLEFIELD_STATUS::astd_read_body(r, header_size).await?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Server::astd_read_body(r, header_size).await?)),
            0x02DA => Ok(Self::SMSG_FORCE_WALK_SPEED_CHANGE(SMSG_FORCE_WALK_SPEED_CHANGE::astd_read_body(r, header_size).await?)),
            0x02DC => Ok(Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(SMSG_FORCE_SWIM_BACK_SPEED_CHANGE::astd_read_body(r, header_size).await?)),
            0x02DE => Ok(Self::SMSG_FORCE_TURN_RATE_CHANGE(SMSG_FORCE_TURN_RATE_CHANGE::astd_read_body(r, header_size).await?)),
            0x02E0 => Ok(Self::MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_Server::astd_read_body(r, header_size).await?)),
            0x02E4 => Ok(Self::SMSG_AREA_SPIRIT_HEALER_TIME(SMSG_AREA_SPIRIT_HEALER_TIME::astd_read_body(r, header_size).await?)),
            0x02E8 => Ok(Self::SMSG_GROUP_JOINED_BATTLEGROUND(SMSG_GROUP_JOINED_BATTLEGROUND::astd_read_body(r, header_size).await?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Server::astd_read_body(r, header_size).await?)),
            0x02EB => Ok(Self::SMSG_BINDER_CONFIRM(SMSG_BINDER_CONFIRM::astd_read_body(r, header_size).await?)),
            0x02EC => Ok(Self::SMSG_BATTLEGROUND_PLAYER_JOINED(SMSG_BATTLEGROUND_PLAYER_JOINED::astd_read_body(r, header_size).await?)),
            0x02ED => Ok(Self::SMSG_BATTLEGROUND_PLAYER_LEFT(SMSG_BATTLEGROUND_PLAYER_LEFT::astd_read_body(r, header_size).await?)),
            0x02F1 => Ok(Self::SMSG_PET_UNLEARN_CONFIRM(SMSG_PET_UNLEARN_CONFIRM::astd_read_body(r, header_size).await?)),
            0x02F2 => Ok(Self::SMSG_PARTY_MEMBER_STATS_FULL(SMSG_PARTY_MEMBER_STATS_FULL::astd_read_body(r, header_size).await?)),
            0x02F4 => Ok(Self::SMSG_WEATHER(SMSG_WEATHER::astd_read_body(r, header_size).await?)),
            0x02FA => Ok(Self::SMSG_RAID_INSTANCE_MESSAGE(SMSG_RAID_INSTANCE_MESSAGE::astd_read_body(r, header_size).await?)),
            0x02FD => Ok(Self::SMSG_CHAT_RESTRICTED(SMSG_CHAT_RESTRICTED::astd_read_body(r, header_size).await?)),
            0x02FE => Ok(Self::SMSG_SPLINE_SET_RUN_SPEED(SMSG_SPLINE_SET_RUN_SPEED::astd_read_body(r, header_size).await?)),
            0x02FF => Ok(Self::SMSG_SPLINE_SET_RUN_BACK_SPEED(SMSG_SPLINE_SET_RUN_BACK_SPEED::astd_read_body(r, header_size).await?)),
            0x0300 => Ok(Self::SMSG_SPLINE_SET_SWIM_SPEED(SMSG_SPLINE_SET_SWIM_SPEED::astd_read_body(r, header_size).await?)),
            0x0301 => Ok(Self::SMSG_SPLINE_SET_WALK_SPEED(SMSG_SPLINE_SET_WALK_SPEED::astd_read_body(r, header_size).await?)),
            0x0302 => Ok(Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED(SMSG_SPLINE_SET_SWIM_BACK_SPEED::astd_read_body(r, header_size).await?)),
            0x0303 => Ok(Self::SMSG_SPLINE_SET_TURN_RATE(SMSG_SPLINE_SET_TURN_RATE::astd_read_body(r, header_size).await?)),
            0x0304 => Ok(Self::SMSG_SPLINE_MOVE_UNROOT(SMSG_SPLINE_MOVE_UNROOT::astd_read_body(r, header_size).await?)),
            0x0305 => Ok(Self::SMSG_SPLINE_MOVE_FEATHER_FALL(SMSG_SPLINE_MOVE_FEATHER_FALL::astd_read_body(r, header_size).await?)),
            0x0306 => Ok(Self::SMSG_SPLINE_MOVE_NORMAL_FALL(SMSG_SPLINE_MOVE_NORMAL_FALL::astd_read_body(r, header_size).await?)),
            0x0307 => Ok(Self::SMSG_SPLINE_MOVE_SET_HOVER(SMSG_SPLINE_MOVE_SET_HOVER::astd_read_body(r, header_size).await?)),
            0x0308 => Ok(Self::SMSG_SPLINE_MOVE_UNSET_HOVER(SMSG_SPLINE_MOVE_UNSET_HOVER::astd_read_body(r, header_size).await?)),
            0x0309 => Ok(Self::SMSG_SPLINE_MOVE_WATER_WALK(SMSG_SPLINE_MOVE_WATER_WALK::astd_read_body(r, header_size).await?)),
            0x030A => Ok(Self::SMSG_SPLINE_MOVE_LAND_WALK(SMSG_SPLINE_MOVE_LAND_WALK::astd_read_body(r, header_size).await?)),
            0x030B => Ok(Self::SMSG_SPLINE_MOVE_START_SWIM(SMSG_SPLINE_MOVE_START_SWIM::astd_read_body(r, header_size).await?)),
            0x030C => Ok(Self::SMSG_SPLINE_MOVE_STOP_SWIM(SMSG_SPLINE_MOVE_STOP_SWIM::astd_read_body(r, header_size).await?)),
            0x030D => Ok(Self::SMSG_SPLINE_MOVE_SET_RUN_MODE(SMSG_SPLINE_MOVE_SET_RUN_MODE::astd_read_body(r, header_size).await?)),
            0x030E => Ok(Self::SMSG_SPLINE_MOVE_SET_WALK_MODE(SMSG_SPLINE_MOVE_SET_WALK_MODE::astd_read_body(r, header_size).await?)),
            0x031A => Ok(Self::SMSG_SPLINE_MOVE_ROOT(SMSG_SPLINE_MOVE_ROOT::astd_read_body(r, header_size).await?)),
            0x031C => Ok(Self::SMSG_INVALIDATE_PLAYER(SMSG_INVALIDATE_PLAYER::astd_read_body(r, header_size).await?)),
            0x031E => Ok(Self::SMSG_INSTANCE_RESET(SMSG_INSTANCE_RESET::astd_read_body(r, header_size).await?)),
            0x031F => Ok(Self::SMSG_INSTANCE_RESET_FAILED(SMSG_INSTANCE_RESET_FAILED::astd_read_body(r, header_size).await?)),
            0x0320 => Ok(Self::SMSG_UPDATE_LAST_INSTANCE(SMSG_UPDATE_LAST_INSTANCE::astd_read_body(r, header_size).await?)),
            0x0321 => Ok(Self::MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Server::astd_read_body(r, header_size).await?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Server::astd_read_body(r, header_size).await?)),
            0x0324 => Ok(Self::SMSG_PET_ACTION_SOUND(SMSG_PET_ACTION_SOUND::astd_read_body(r, header_size).await?)),
            0x0325 => Ok(Self::SMSG_PET_DISMISS_SOUND(SMSG_PET_DISMISS_SOUND::astd_read_body(r, header_size).await?)),
            0x0328 => Ok(Self::SMSG_GM_TICKET_STATUS_UPDATE(SMSG_GM_TICKET_STATUS_UPDATE::astd_read_body(r, header_size).await?)),
            0x032B => Ok(Self::SMSG_UPDATE_INSTANCE_OWNERSHIP(SMSG_UPDATE_INSTANCE_OWNERSHIP::astd_read_body(r, header_size).await?)),
            0x032F => Ok(Self::SMSG_SPELLINSTAKILLLOG(SMSG_SPELLINSTAKILLLOG::astd_read_body(r, header_size).await?)),
            0x0330 => Ok(Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS(SMSG_SPELL_UPDATE_CHAIN_TARGETS::astd_read_body(r, header_size).await?)),
            0x0332 => Ok(Self::SMSG_EXPECTED_SPAM_RECORDS(SMSG_EXPECTED_SPAM_RECORDS::astd_read_body(r, header_size).await?)),
            0x033B => Ok(Self::SMSG_DEFENSE_MESSAGE(SMSG_DEFENSE_MESSAGE::astd_read_body(r, header_size).await?)),
            _ => Err(ServerOpcodeMessageError::InvalidOpcode(header.opcode)),
        }
    }

}

#[derive(Debug)]
pub enum ServerOpcodeMessageError {
    Io(std::io::Error),
    InvalidOpcode(u16),
    MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULTError),
    MSG_PETITION_RENAME(MSG_PETITION_RENAMEError),
    SMSG_CHAR_CREATE(SMSG_CHAR_CREATEError),
    SMSG_CHAR_ENUM(SMSG_CHAR_ENUMError),
    SMSG_CHAR_DELETE(SMSG_CHAR_DELETEError),
    SMSG_TRANSFER_PENDING(SMSG_TRANSFER_PENDINGError),
    SMSG_TRANSFER_ABORTED(SMSG_TRANSFER_ABORTEDError),
    SMSG_CHARACTER_LOGIN_FAILED(SMSG_CHARACTER_LOGIN_FAILEDError),
    SMSG_LOGOUT_RESPONSE(SMSG_LOGOUT_RESPONSEError),
    SMSG_NAME_QUERY_RESPONSE(SMSG_NAME_QUERY_RESPONSEError),
    SMSG_PET_NAME_QUERY_RESPONSE(SMSG_PET_NAME_QUERY_RESPONSEError),
    SMSG_GUILD_QUERY_RESPONSE(SMSG_GUILD_QUERY_RESPONSEError),
    SMSG_ITEM_QUERY_SINGLE_RESPONSE(SMSG_ITEM_QUERY_SINGLE_RESPONSEError),
    SMSG_PAGE_TEXT_QUERY_RESPONSE(SMSG_PAGE_TEXT_QUERY_RESPONSEError),
    SMSG_QUEST_QUERY_RESPONSE(SMSG_QUEST_QUERY_RESPONSEError),
    SMSG_GAMEOBJECT_QUERY_RESPONSE(SMSG_GAMEOBJECT_QUERY_RESPONSEError),
    SMSG_CREATURE_QUERY_RESPONSE(SMSG_CREATURE_QUERY_RESPONSEError),
    SMSG_WHO(SMSG_WHOError),
    SMSG_WHOIS(SMSG_WHOISError),
    SMSG_FRIEND_LIST(SMSG_FRIEND_LISTError),
    SMSG_FRIEND_STATUS(SMSG_FRIEND_STATUSError),
    SMSG_GROUP_INVITE(SMSG_GROUP_INVITEError),
    SMSG_GROUP_DECLINE(SMSG_GROUP_DECLINEError),
    SMSG_GROUP_SET_LEADER(SMSG_GROUP_SET_LEADERError),
    SMSG_GROUP_LIST(SMSG_GROUP_LISTError),
    SMSG_PARTY_MEMBER_STATS(SMSG_PARTY_MEMBER_STATSError),
    SMSG_PARTY_COMMAND_RESULT(SMSG_PARTY_COMMAND_RESULTError),
    SMSG_GUILD_INVITE(SMSG_GUILD_INVITEError),
    SMSG_GUILD_INFO(SMSG_GUILD_INFOError),
    SMSG_GUILD_EVENT(SMSG_GUILD_EVENTError),
    SMSG_GUILD_COMMAND_RESULT(SMSG_GUILD_COMMAND_RESULTError),
    SMSG_MESSAGECHAT(SMSG_MESSAGECHATError),
    SMSG_CHANNEL_NOTIFY(SMSG_CHANNEL_NOTIFYError),
    SMSG_CHANNEL_LIST(SMSG_CHANNEL_LISTError),
    SMSG_MONSTER_MOVE(SMSG_MONSTER_MOVEError),
    SMSG_TRIGGER_CINEMATIC(SMSG_TRIGGER_CINEMATICError),
    SMSG_EMOTE(SMSG_EMOTEError),
    SMSG_TEXT_EMOTE(SMSG_TEXT_EMOTEError),
    SMSG_INVENTORY_CHANGE_FAILURE(SMSG_INVENTORY_CHANGE_FAILUREError),
    SMSG_TRADE_STATUS(SMSG_TRADE_STATUSError),
    SMSG_SET_PROFICIENCY(SMSG_SET_PROFICIENCYError),
    SMSG_CAST_RESULT(SMSG_CAST_RESULTError),
    SMSG_SPELL_START(SMSG_SPELL_STARTError),
    SMSG_SPELL_GO(SMSG_SPELL_GOError),
    SMSG_SPELL_FAILURE(SMSG_SPELL_FAILUREError),
    SMSG_PET_CAST_FAILED(SMSG_PET_CAST_FAILEDError),
    SMSG_AI_REACTION(SMSG_AI_REACTIONError),
    SMSG_SPELLENERGIZELOG(SMSG_SPELLENERGIZELOGError),
    SMSG_BINDPOINTUPDATE(SMSG_BINDPOINTUPDATEError),
    SMSG_PLAYERBOUND(SMSG_PLAYERBOUNDError),
    SMSG_RESURRECT_REQUEST(SMSG_RESURRECT_REQUESTError),
    SMSG_LOOT_RESPONSE(SMSG_LOOT_RESPONSEError),
    SMSG_ITEM_PUSH_RESULT(SMSG_ITEM_PUSH_RESULTError),
    SMSG_DUEL_WINNER(SMSG_DUEL_WINNERError),
    SMSG_MOUNTRESULT(SMSG_MOUNTRESULTError),
    SMSG_DISMOUNTRESULT(SMSG_DISMOUNTRESULTError),
    SMSG_PET_TAME_FAILURE(SMSG_PET_TAME_FAILUREError),
    SMSG_PET_SPELLS(SMSG_PET_SPELLSError),
    SMSG_PET_MODE(SMSG_PET_MODEError),
    SMSG_GOSSIP_MESSAGE(SMSG_GOSSIP_MESSAGEError),
    SMSG_NPC_TEXT_UPDATE(SMSG_NPC_TEXT_UPDATEError),
    SMSG_QUESTGIVER_STATUS(SMSG_QUESTGIVER_STATUSError),
    SMSG_QUESTGIVER_QUEST_LIST(SMSG_QUESTGIVER_QUEST_LISTError),
    SMSG_QUESTGIVER_QUEST_DETAILS(SMSG_QUESTGIVER_QUEST_DETAILSError),
    SMSG_QUESTGIVER_REQUEST_ITEMS(SMSG_QUESTGIVER_REQUEST_ITEMSError),
    SMSG_QUESTGIVER_OFFER_REWARD(SMSG_QUESTGIVER_OFFER_REWARDError),
    SMSG_QUESTGIVER_QUEST_INVALID(SMSG_QUESTGIVER_QUEST_INVALIDError),
    SMSG_QUESTGIVER_QUEST_FAILED(SMSG_QUESTGIVER_QUEST_FAILEDError),
    SMSG_QUEST_CONFIRM_ACCEPT(SMSG_QUEST_CONFIRM_ACCEPTError),
    SMSG_SELL_ITEM(SMSG_SELL_ITEMError),
    SMSG_BUY_FAILED(SMSG_BUY_FAILEDError),
    SMSG_ACTIVATETAXIREPLY(SMSG_ACTIVATETAXIREPLYError),
    SMSG_TRAINER_LIST(SMSG_TRAINER_LISTError),
    SMSG_TRAINER_BUY_FAILED(SMSG_TRAINER_BUY_FAILEDError),
    SMSG_BUY_BANK_SLOT_RESULT(SMSG_BUY_BANK_SLOT_RESULTError),
    SMSG_PETITION_SIGN_RESULTS(SMSG_PETITION_SIGN_RESULTSError),
    SMSG_TURN_IN_PETITION_RESULTS(SMSG_TURN_IN_PETITION_RESULTSError),
    SMSG_PETITION_QUERY_RESPONSE(SMSG_PETITION_QUERY_RESPONSEError),
    SMSG_NOTIFICATION(SMSG_NOTIFICATIONError),
    SMSG_LOG_XPGAIN(SMSG_LOG_XPGAINError),
    SMSG_START_MIRROR_TIMER(SMSG_START_MIRROR_TIMERError),
    SMSG_PAUSE_MIRROR_TIMER(SMSG_PAUSE_MIRROR_TIMERError),
    SMSG_STOP_MIRROR_TIMER(SMSG_STOP_MIRROR_TIMERError),
    SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSEError),
    MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_ServerError),
    SMSG_EXPLORATION_EXPERIENCE(SMSG_EXPLORATION_EXPERIENCEError),
    SMSG_ENVIRONMENTALDAMAGELOG(SMSG_ENVIRONMENTALDAMAGELOGError),
    SMSG_GMTICKET_CREATE(SMSG_GMTICKET_CREATEError),
    SMSG_GMTICKET_UPDATETEXT(SMSG_GMTICKET_UPDATETEXTError),
    SMSG_GMTICKET_GETTICKET(SMSG_GMTICKET_GETTICKETError),
    MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_ServerError),
    SMSG_GMTICKET_DELETETICKET(SMSG_GMTICKET_DELETETICKETError),
    SMSG_GOSSIP_POI(SMSG_GOSSIP_POIError),
    SMSG_LOGIN_VERIFY_WORLD(SMSG_LOGIN_VERIFY_WORLDError),
    SMSG_MAIL_LIST_RESULT(SMSG_MAIL_LIST_RESULTError),
    SMSG_BATTLEFIELD_LIST(SMSG_BATTLEFIELD_LISTError),
    SMSG_ITEM_TEXT_QUERY_RESPONSE(SMSG_ITEM_TEXT_QUERY_RESPONSEError),
    SMSG_SPELLLOGMISS(SMSG_SPELLLOGMISSError),
    SMSG_SPELLLOGEXECUTE(SMSG_SPELLLOGEXECUTEError),
    SMSG_PERIODICAURALOG(SMSG_PERIODICAURALOGError),
    SMSG_SPELLDAMAGESHIELD(SMSG_SPELLDAMAGESHIELDError),
    SMSG_SPELLNONMELEEDAMAGELOG(SMSG_SPELLNONMELEEDAMAGELOGError),
    SMSG_PROCRESIST(SMSG_PROCRESISTError),
    MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_ServerError),
    SMSG_STABLE_RESULT(SMSG_STABLE_RESULTError),
    SMSG_RAID_GROUP_ONLY(SMSG_RAID_GROUP_ONLYError),
    SMSG_PVP_CREDIT(SMSG_PVP_CREDITError),
    SMSG_SERVER_MESSAGE(SMSG_SERVER_MESSAGEError),
    SMSG_MEETINGSTONE_SETQUEUE(SMSG_MEETINGSTONE_SETQUEUEError),
    SMSG_STANDSTATE_UPDATE(SMSG_STANDSTATE_UPDATEError),
    SMSG_LOOT_ROLL_WON(SMSG_LOOT_ROLL_WONError),
    SMSG_LOOT_ROLL(SMSG_LOOT_ROLLError),
    SMSG_CHAT_PLAYER_NOT_FOUND(SMSG_CHAT_PLAYER_NOT_FOUNDError),
    SMSG_MONSTER_MOVE_TRANSPORT(SMSG_MONSTER_MOVE_TRANSPORTError),
    SMSG_AREA_TRIGGER_MESSAGE(SMSG_AREA_TRIGGER_MESSAGEError),
    SMSG_MEETINGSTONE_JOINFAILED(SMSG_MEETINGSTONE_JOINFAILEDError),
    SMSG_INIT_WORLD_STATES(SMSG_INIT_WORLD_STATESError),
    SMSG_ITEM_NAME_QUERY_RESPONSE(SMSG_ITEM_NAME_QUERY_RESPONSEError),
    SMSG_PET_ACTION_FEEDBACK(SMSG_PET_ACTION_FEEDBACKError),
    SMSG_CHAR_RENAME(SMSG_CHAR_RENAMEError),
    SMSG_RAID_INSTANCE_INFO(SMSG_RAID_INSTANCE_INFOError),
    SMSG_BATTLEFIELD_STATUS(SMSG_BATTLEFIELD_STATUSError),
    MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_ServerError),
    MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_ServerError),
    SMSG_GROUP_JOINED_BATTLEGROUND(SMSG_GROUP_JOINED_BATTLEGROUNDError),
    SMSG_PARTY_MEMBER_STATS_FULL(SMSG_PARTY_MEMBER_STATS_FULLError),
    SMSG_WEATHER(SMSG_WEATHERError),
    SMSG_RAID_INSTANCE_MESSAGE(SMSG_RAID_INSTANCE_MESSAGEError),
    SMSG_INSTANCE_RESET(SMSG_INSTANCE_RESETError),
    SMSG_INSTANCE_RESET_FAILED(SMSG_INSTANCE_RESET_FAILEDError),
    SMSG_UPDATE_LAST_INSTANCE(SMSG_UPDATE_LAST_INSTANCEError),
    MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_ServerError),
    SMSG_PET_ACTION_SOUND(SMSG_PET_ACTION_SOUNDError),
    SMSG_GM_TICKET_STATUS_UPDATE(SMSG_GM_TICKET_STATUS_UPDATEError),
    SMSG_EXPECTED_SPAM_RECORDS(SMSG_EXPECTED_SPAM_RECORDSError),
    SMSG_DEFENSE_MESSAGE(SMSG_DEFENSE_MESSAGEError),
}

impl std::error::Error for ServerOpcodeMessageError {}
impl std::fmt::Display for ServerOpcodeMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode received for ServerMessage: '{}'", i)),
            Self::MSG_QUEST_PUSH_RESULT(i) => i.fmt(f),
            Self::MSG_PETITION_RENAME(i) => i.fmt(f),
            Self::SMSG_CHAR_CREATE(i) => i.fmt(f),
            Self::SMSG_CHAR_ENUM(i) => i.fmt(f),
            Self::SMSG_CHAR_DELETE(i) => i.fmt(f),
            Self::SMSG_TRANSFER_PENDING(i) => i.fmt(f),
            Self::SMSG_TRANSFER_ABORTED(i) => i.fmt(f),
            Self::SMSG_CHARACTER_LOGIN_FAILED(i) => i.fmt(f),
            Self::SMSG_LOGOUT_RESPONSE(i) => i.fmt(f),
            Self::SMSG_NAME_QUERY_RESPONSE(i) => i.fmt(f),
            Self::SMSG_PET_NAME_QUERY_RESPONSE(i) => i.fmt(f),
            Self::SMSG_GUILD_QUERY_RESPONSE(i) => i.fmt(f),
            Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(i) => i.fmt(f),
            Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(i) => i.fmt(f),
            Self::SMSG_QUEST_QUERY_RESPONSE(i) => i.fmt(f),
            Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(i) => i.fmt(f),
            Self::SMSG_CREATURE_QUERY_RESPONSE(i) => i.fmt(f),
            Self::SMSG_WHO(i) => i.fmt(f),
            Self::SMSG_WHOIS(i) => i.fmt(f),
            Self::SMSG_FRIEND_LIST(i) => i.fmt(f),
            Self::SMSG_FRIEND_STATUS(i) => i.fmt(f),
            Self::SMSG_GROUP_INVITE(i) => i.fmt(f),
            Self::SMSG_GROUP_DECLINE(i) => i.fmt(f),
            Self::SMSG_GROUP_SET_LEADER(i) => i.fmt(f),
            Self::SMSG_GROUP_LIST(i) => i.fmt(f),
            Self::SMSG_PARTY_MEMBER_STATS(i) => i.fmt(f),
            Self::SMSG_PARTY_COMMAND_RESULT(i) => i.fmt(f),
            Self::SMSG_GUILD_INVITE(i) => i.fmt(f),
            Self::SMSG_GUILD_INFO(i) => i.fmt(f),
            Self::SMSG_GUILD_EVENT(i) => i.fmt(f),
            Self::SMSG_GUILD_COMMAND_RESULT(i) => i.fmt(f),
            Self::SMSG_MESSAGECHAT(i) => i.fmt(f),
            Self::SMSG_CHANNEL_NOTIFY(i) => i.fmt(f),
            Self::SMSG_CHANNEL_LIST(i) => i.fmt(f),
            Self::SMSG_MONSTER_MOVE(i) => i.fmt(f),
            Self::SMSG_TRIGGER_CINEMATIC(i) => i.fmt(f),
            Self::SMSG_EMOTE(i) => i.fmt(f),
            Self::SMSG_TEXT_EMOTE(i) => i.fmt(f),
            Self::SMSG_INVENTORY_CHANGE_FAILURE(i) => i.fmt(f),
            Self::SMSG_TRADE_STATUS(i) => i.fmt(f),
            Self::SMSG_SET_PROFICIENCY(i) => i.fmt(f),
            Self::SMSG_CAST_RESULT(i) => i.fmt(f),
            Self::SMSG_SPELL_START(i) => i.fmt(f),
            Self::SMSG_SPELL_GO(i) => i.fmt(f),
            Self::SMSG_SPELL_FAILURE(i) => i.fmt(f),
            Self::SMSG_PET_CAST_FAILED(i) => i.fmt(f),
            Self::SMSG_AI_REACTION(i) => i.fmt(f),
            Self::SMSG_SPELLENERGIZELOG(i) => i.fmt(f),
            Self::SMSG_BINDPOINTUPDATE(i) => i.fmt(f),
            Self::SMSG_PLAYERBOUND(i) => i.fmt(f),
            Self::SMSG_RESURRECT_REQUEST(i) => i.fmt(f),
            Self::SMSG_LOOT_RESPONSE(i) => i.fmt(f),
            Self::SMSG_ITEM_PUSH_RESULT(i) => i.fmt(f),
            Self::SMSG_DUEL_WINNER(i) => i.fmt(f),
            Self::SMSG_MOUNTRESULT(i) => i.fmt(f),
            Self::SMSG_DISMOUNTRESULT(i) => i.fmt(f),
            Self::SMSG_PET_TAME_FAILURE(i) => i.fmt(f),
            Self::SMSG_PET_SPELLS(i) => i.fmt(f),
            Self::SMSG_PET_MODE(i) => i.fmt(f),
            Self::SMSG_GOSSIP_MESSAGE(i) => i.fmt(f),
            Self::SMSG_NPC_TEXT_UPDATE(i) => i.fmt(f),
            Self::SMSG_QUESTGIVER_STATUS(i) => i.fmt(f),
            Self::SMSG_QUESTGIVER_QUEST_LIST(i) => i.fmt(f),
            Self::SMSG_QUESTGIVER_QUEST_DETAILS(i) => i.fmt(f),
            Self::SMSG_QUESTGIVER_REQUEST_ITEMS(i) => i.fmt(f),
            Self::SMSG_QUESTGIVER_OFFER_REWARD(i) => i.fmt(f),
            Self::SMSG_QUESTGIVER_QUEST_INVALID(i) => i.fmt(f),
            Self::SMSG_QUESTGIVER_QUEST_FAILED(i) => i.fmt(f),
            Self::SMSG_QUEST_CONFIRM_ACCEPT(i) => i.fmt(f),
            Self::SMSG_SELL_ITEM(i) => i.fmt(f),
            Self::SMSG_BUY_FAILED(i) => i.fmt(f),
            Self::SMSG_ACTIVATETAXIREPLY(i) => i.fmt(f),
            Self::SMSG_TRAINER_LIST(i) => i.fmt(f),
            Self::SMSG_TRAINER_BUY_FAILED(i) => i.fmt(f),
            Self::SMSG_BUY_BANK_SLOT_RESULT(i) => i.fmt(f),
            Self::SMSG_PETITION_SIGN_RESULTS(i) => i.fmt(f),
            Self::SMSG_TURN_IN_PETITION_RESULTS(i) => i.fmt(f),
            Self::SMSG_PETITION_QUERY_RESPONSE(i) => i.fmt(f),
            Self::SMSG_NOTIFICATION(i) => i.fmt(f),
            Self::SMSG_LOG_XPGAIN(i) => i.fmt(f),
            Self::SMSG_START_MIRROR_TIMER(i) => i.fmt(f),
            Self::SMSG_PAUSE_MIRROR_TIMER(i) => i.fmt(f),
            Self::SMSG_STOP_MIRROR_TIMER(i) => i.fmt(f),
            Self::SMSG_AUTH_RESPONSE(i) => i.fmt(f),
            Self::MSG_SAVE_GUILD_EMBLEM(i) => i.fmt(f),
            Self::SMSG_EXPLORATION_EXPERIENCE(i) => i.fmt(f),
            Self::SMSG_ENVIRONMENTALDAMAGELOG(i) => i.fmt(f),
            Self::SMSG_GMTICKET_CREATE(i) => i.fmt(f),
            Self::SMSG_GMTICKET_UPDATETEXT(i) => i.fmt(f),
            Self::SMSG_GMTICKET_GETTICKET(i) => i.fmt(f),
            Self::MSG_CORPSE_QUERY(i) => i.fmt(f),
            Self::SMSG_GMTICKET_DELETETICKET(i) => i.fmt(f),
            Self::SMSG_GOSSIP_POI(i) => i.fmt(f),
            Self::SMSG_LOGIN_VERIFY_WORLD(i) => i.fmt(f),
            Self::SMSG_MAIL_LIST_RESULT(i) => i.fmt(f),
            Self::SMSG_BATTLEFIELD_LIST(i) => i.fmt(f),
            Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(i) => i.fmt(f),
            Self::SMSG_SPELLLOGMISS(i) => i.fmt(f),
            Self::SMSG_SPELLLOGEXECUTE(i) => i.fmt(f),
            Self::SMSG_PERIODICAURALOG(i) => i.fmt(f),
            Self::SMSG_SPELLDAMAGESHIELD(i) => i.fmt(f),
            Self::SMSG_SPELLNONMELEEDAMAGELOG(i) => i.fmt(f),
            Self::SMSG_PROCRESIST(i) => i.fmt(f),
            Self::MSG_LIST_STABLED_PETS(i) => i.fmt(f),
            Self::SMSG_STABLE_RESULT(i) => i.fmt(f),
            Self::SMSG_RAID_GROUP_ONLY(i) => i.fmt(f),
            Self::SMSG_PVP_CREDIT(i) => i.fmt(f),
            Self::SMSG_SERVER_MESSAGE(i) => i.fmt(f),
            Self::SMSG_MEETINGSTONE_SETQUEUE(i) => i.fmt(f),
            Self::SMSG_STANDSTATE_UPDATE(i) => i.fmt(f),
            Self::SMSG_LOOT_ROLL_WON(i) => i.fmt(f),
            Self::SMSG_LOOT_ROLL(i) => i.fmt(f),
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(i) => i.fmt(f),
            Self::SMSG_MONSTER_MOVE_TRANSPORT(i) => i.fmt(f),
            Self::SMSG_AREA_TRIGGER_MESSAGE(i) => i.fmt(f),
            Self::SMSG_MEETINGSTONE_JOINFAILED(i) => i.fmt(f),
            Self::SMSG_INIT_WORLD_STATES(i) => i.fmt(f),
            Self::SMSG_ITEM_NAME_QUERY_RESPONSE(i) => i.fmt(f),
            Self::SMSG_PET_ACTION_FEEDBACK(i) => i.fmt(f),
            Self::SMSG_CHAR_RENAME(i) => i.fmt(f),
            Self::SMSG_RAID_INSTANCE_INFO(i) => i.fmt(f),
            Self::SMSG_BATTLEFIELD_STATUS(i) => i.fmt(f),
            Self::MSG_INSPECT_HONOR_STATS(i) => i.fmt(f),
            Self::MSG_PVP_LOG_DATA(i) => i.fmt(f),
            Self::SMSG_GROUP_JOINED_BATTLEGROUND(i) => i.fmt(f),
            Self::SMSG_PARTY_MEMBER_STATS_FULL(i) => i.fmt(f),
            Self::SMSG_WEATHER(i) => i.fmt(f),
            Self::SMSG_RAID_INSTANCE_MESSAGE(i) => i.fmt(f),
            Self::SMSG_INSTANCE_RESET(i) => i.fmt(f),
            Self::SMSG_INSTANCE_RESET_FAILED(i) => i.fmt(f),
            Self::SMSG_UPDATE_LAST_INSTANCE(i) => i.fmt(f),
            Self::MSG_RAID_TARGET_UPDATE(i) => i.fmt(f),
            Self::SMSG_PET_ACTION_SOUND(i) => i.fmt(f),
            Self::SMSG_GM_TICKET_STATUS_UPDATE(i) => i.fmt(f),
            Self::SMSG_EXPECTED_SPAM_RECORDS(i) => i.fmt(f),
            Self::SMSG_DEFENSE_MESSAGE(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for ServerOpcodeMessageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MSG_QUEST_PUSH_RESULTError> for ServerOpcodeMessageError {
    fn from(e: MSG_QUEST_PUSH_RESULTError) -> Self {
        match e {
            MSG_QUEST_PUSH_RESULTError::Io(i) => Self::Io(i),
            _ => Self::MSG_QUEST_PUSH_RESULT(e),
        }
    }
}

impl From<MSG_PETITION_RENAMEError> for ServerOpcodeMessageError {
    fn from(e: MSG_PETITION_RENAMEError) -> Self {
        match e {
            MSG_PETITION_RENAMEError::Io(i) => Self::Io(i),
            _ => Self::MSG_PETITION_RENAME(e),
        }
    }
}

impl From<SMSG_CHAR_CREATEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_CHAR_CREATEError) -> Self {
        match e {
            SMSG_CHAR_CREATEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_CHAR_CREATE(e),
        }
    }
}

impl From<SMSG_CHAR_ENUMError> for ServerOpcodeMessageError {
    fn from(e: SMSG_CHAR_ENUMError) -> Self {
        match e {
            SMSG_CHAR_ENUMError::Io(i) => Self::Io(i),
            _ => Self::SMSG_CHAR_ENUM(e),
        }
    }
}

impl From<SMSG_CHAR_DELETEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_CHAR_DELETEError) -> Self {
        match e {
            SMSG_CHAR_DELETEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_CHAR_DELETE(e),
        }
    }
}

impl From<SMSG_TRANSFER_PENDINGError> for ServerOpcodeMessageError {
    fn from(e: SMSG_TRANSFER_PENDINGError) -> Self {
        match e {
            SMSG_TRANSFER_PENDINGError::Io(i) => Self::Io(i),
            _ => Self::SMSG_TRANSFER_PENDING(e),
        }
    }
}

impl From<SMSG_TRANSFER_ABORTEDError> for ServerOpcodeMessageError {
    fn from(e: SMSG_TRANSFER_ABORTEDError) -> Self {
        match e {
            SMSG_TRANSFER_ABORTEDError::Io(i) => Self::Io(i),
            _ => Self::SMSG_TRANSFER_ABORTED(e),
        }
    }
}

impl From<SMSG_CHARACTER_LOGIN_FAILEDError> for ServerOpcodeMessageError {
    fn from(e: SMSG_CHARACTER_LOGIN_FAILEDError) -> Self {
        match e {
            SMSG_CHARACTER_LOGIN_FAILEDError::Io(i) => Self::Io(i),
            _ => Self::SMSG_CHARACTER_LOGIN_FAILED(e),
        }
    }
}

impl From<SMSG_LOGOUT_RESPONSEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_LOGOUT_RESPONSEError) -> Self {
        match e {
            SMSG_LOGOUT_RESPONSEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_LOGOUT_RESPONSE(e),
        }
    }
}

impl From<SMSG_NAME_QUERY_RESPONSEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_NAME_QUERY_RESPONSEError) -> Self {
        match e {
            SMSG_NAME_QUERY_RESPONSEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_NAME_QUERY_RESPONSE(e),
        }
    }
}

impl From<SMSG_PET_NAME_QUERY_RESPONSEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PET_NAME_QUERY_RESPONSEError) -> Self {
        match e {
            SMSG_PET_NAME_QUERY_RESPONSEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PET_NAME_QUERY_RESPONSE(e),
        }
    }
}

impl From<SMSG_GUILD_QUERY_RESPONSEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GUILD_QUERY_RESPONSEError) -> Self {
        match e {
            SMSG_GUILD_QUERY_RESPONSEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GUILD_QUERY_RESPONSE(e),
        }
    }
}

impl From<SMSG_ITEM_QUERY_SINGLE_RESPONSEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_ITEM_QUERY_SINGLE_RESPONSEError) -> Self {
        match e {
            SMSG_ITEM_QUERY_SINGLE_RESPONSEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(e),
        }
    }
}

impl From<SMSG_PAGE_TEXT_QUERY_RESPONSEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PAGE_TEXT_QUERY_RESPONSEError) -> Self {
        match e {
            SMSG_PAGE_TEXT_QUERY_RESPONSEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(e),
        }
    }
}

impl From<SMSG_QUEST_QUERY_RESPONSEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_QUEST_QUERY_RESPONSEError) -> Self {
        match e {
            SMSG_QUEST_QUERY_RESPONSEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_QUEST_QUERY_RESPONSE(e),
        }
    }
}

impl From<SMSG_GAMEOBJECT_QUERY_RESPONSEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GAMEOBJECT_QUERY_RESPONSEError) -> Self {
        match e {
            SMSG_GAMEOBJECT_QUERY_RESPONSEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(e),
        }
    }
}

impl From<SMSG_CREATURE_QUERY_RESPONSEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_CREATURE_QUERY_RESPONSEError) -> Self {
        match e {
            SMSG_CREATURE_QUERY_RESPONSEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_CREATURE_QUERY_RESPONSE(e),
        }
    }
}

impl From<SMSG_WHOError> for ServerOpcodeMessageError {
    fn from(e: SMSG_WHOError) -> Self {
        match e {
            SMSG_WHOError::Io(i) => Self::Io(i),
            _ => Self::SMSG_WHO(e),
        }
    }
}

impl From<SMSG_WHOISError> for ServerOpcodeMessageError {
    fn from(e: SMSG_WHOISError) -> Self {
        match e {
            SMSG_WHOISError::Io(i) => Self::Io(i),
            _ => Self::SMSG_WHOIS(e),
        }
    }
}

impl From<SMSG_FRIEND_LISTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_FRIEND_LISTError) -> Self {
        match e {
            SMSG_FRIEND_LISTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_FRIEND_LIST(e),
        }
    }
}

impl From<SMSG_FRIEND_STATUSError> for ServerOpcodeMessageError {
    fn from(e: SMSG_FRIEND_STATUSError) -> Self {
        match e {
            SMSG_FRIEND_STATUSError::Io(i) => Self::Io(i),
            _ => Self::SMSG_FRIEND_STATUS(e),
        }
    }
}

impl From<SMSG_GROUP_INVITEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GROUP_INVITEError) -> Self {
        match e {
            SMSG_GROUP_INVITEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GROUP_INVITE(e),
        }
    }
}

impl From<SMSG_GROUP_DECLINEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GROUP_DECLINEError) -> Self {
        match e {
            SMSG_GROUP_DECLINEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GROUP_DECLINE(e),
        }
    }
}

impl From<SMSG_GROUP_SET_LEADERError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GROUP_SET_LEADERError) -> Self {
        match e {
            SMSG_GROUP_SET_LEADERError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GROUP_SET_LEADER(e),
        }
    }
}

impl From<SMSG_GROUP_LISTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GROUP_LISTError) -> Self {
        match e {
            SMSG_GROUP_LISTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GROUP_LIST(e),
        }
    }
}

impl From<SMSG_PARTY_MEMBER_STATSError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PARTY_MEMBER_STATSError) -> Self {
        match e {
            SMSG_PARTY_MEMBER_STATSError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PARTY_MEMBER_STATS(e),
        }
    }
}

impl From<SMSG_PARTY_COMMAND_RESULTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PARTY_COMMAND_RESULTError) -> Self {
        match e {
            SMSG_PARTY_COMMAND_RESULTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PARTY_COMMAND_RESULT(e),
        }
    }
}

impl From<SMSG_GUILD_INVITEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GUILD_INVITEError) -> Self {
        match e {
            SMSG_GUILD_INVITEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GUILD_INVITE(e),
        }
    }
}

impl From<SMSG_GUILD_INFOError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GUILD_INFOError) -> Self {
        match e {
            SMSG_GUILD_INFOError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GUILD_INFO(e),
        }
    }
}

impl From<SMSG_GUILD_EVENTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GUILD_EVENTError) -> Self {
        match e {
            SMSG_GUILD_EVENTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GUILD_EVENT(e),
        }
    }
}

impl From<SMSG_GUILD_COMMAND_RESULTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GUILD_COMMAND_RESULTError) -> Self {
        match e {
            SMSG_GUILD_COMMAND_RESULTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GUILD_COMMAND_RESULT(e),
        }
    }
}

impl From<SMSG_MESSAGECHATError> for ServerOpcodeMessageError {
    fn from(e: SMSG_MESSAGECHATError) -> Self {
        match e {
            SMSG_MESSAGECHATError::Io(i) => Self::Io(i),
            _ => Self::SMSG_MESSAGECHAT(e),
        }
    }
}

impl From<SMSG_CHANNEL_NOTIFYError> for ServerOpcodeMessageError {
    fn from(e: SMSG_CHANNEL_NOTIFYError) -> Self {
        match e {
            SMSG_CHANNEL_NOTIFYError::Io(i) => Self::Io(i),
            _ => Self::SMSG_CHANNEL_NOTIFY(e),
        }
    }
}

impl From<SMSG_CHANNEL_LISTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_CHANNEL_LISTError) -> Self {
        match e {
            SMSG_CHANNEL_LISTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_CHANNEL_LIST(e),
        }
    }
}

impl From<SMSG_MONSTER_MOVEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_MONSTER_MOVEError) -> Self {
        match e {
            SMSG_MONSTER_MOVEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_MONSTER_MOVE(e),
        }
    }
}

impl From<SMSG_TRIGGER_CINEMATICError> for ServerOpcodeMessageError {
    fn from(e: SMSG_TRIGGER_CINEMATICError) -> Self {
        match e {
            SMSG_TRIGGER_CINEMATICError::Io(i) => Self::Io(i),
            _ => Self::SMSG_TRIGGER_CINEMATIC(e),
        }
    }
}

impl From<SMSG_EMOTEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_EMOTEError) -> Self {
        match e {
            SMSG_EMOTEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_EMOTE(e),
        }
    }
}

impl From<SMSG_TEXT_EMOTEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_TEXT_EMOTEError) -> Self {
        match e {
            SMSG_TEXT_EMOTEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_TEXT_EMOTE(e),
        }
    }
}

impl From<SMSG_INVENTORY_CHANGE_FAILUREError> for ServerOpcodeMessageError {
    fn from(e: SMSG_INVENTORY_CHANGE_FAILUREError) -> Self {
        match e {
            SMSG_INVENTORY_CHANGE_FAILUREError::Io(i) => Self::Io(i),
            _ => Self::SMSG_INVENTORY_CHANGE_FAILURE(e),
        }
    }
}

impl From<SMSG_TRADE_STATUSError> for ServerOpcodeMessageError {
    fn from(e: SMSG_TRADE_STATUSError) -> Self {
        match e {
            SMSG_TRADE_STATUSError::Io(i) => Self::Io(i),
            _ => Self::SMSG_TRADE_STATUS(e),
        }
    }
}

impl From<SMSG_SET_PROFICIENCYError> for ServerOpcodeMessageError {
    fn from(e: SMSG_SET_PROFICIENCYError) -> Self {
        match e {
            SMSG_SET_PROFICIENCYError::Io(i) => Self::Io(i),
            _ => Self::SMSG_SET_PROFICIENCY(e),
        }
    }
}

impl From<SMSG_CAST_RESULTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_CAST_RESULTError) -> Self {
        match e {
            SMSG_CAST_RESULTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_CAST_RESULT(e),
        }
    }
}

impl From<SMSG_SPELL_STARTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_SPELL_STARTError) -> Self {
        match e {
            SMSG_SPELL_STARTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_SPELL_START(e),
        }
    }
}

impl From<SMSG_SPELL_GOError> for ServerOpcodeMessageError {
    fn from(e: SMSG_SPELL_GOError) -> Self {
        match e {
            SMSG_SPELL_GOError::Io(i) => Self::Io(i),
            _ => Self::SMSG_SPELL_GO(e),
        }
    }
}

impl From<SMSG_SPELL_FAILUREError> for ServerOpcodeMessageError {
    fn from(e: SMSG_SPELL_FAILUREError) -> Self {
        match e {
            SMSG_SPELL_FAILUREError::Io(i) => Self::Io(i),
            _ => Self::SMSG_SPELL_FAILURE(e),
        }
    }
}

impl From<SMSG_PET_CAST_FAILEDError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PET_CAST_FAILEDError) -> Self {
        match e {
            SMSG_PET_CAST_FAILEDError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PET_CAST_FAILED(e),
        }
    }
}

impl From<SMSG_AI_REACTIONError> for ServerOpcodeMessageError {
    fn from(e: SMSG_AI_REACTIONError) -> Self {
        match e {
            SMSG_AI_REACTIONError::Io(i) => Self::Io(i),
            _ => Self::SMSG_AI_REACTION(e),
        }
    }
}

impl From<SMSG_SPELLENERGIZELOGError> for ServerOpcodeMessageError {
    fn from(e: SMSG_SPELLENERGIZELOGError) -> Self {
        match e {
            SMSG_SPELLENERGIZELOGError::Io(i) => Self::Io(i),
            _ => Self::SMSG_SPELLENERGIZELOG(e),
        }
    }
}

impl From<SMSG_BINDPOINTUPDATEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_BINDPOINTUPDATEError) -> Self {
        match e {
            SMSG_BINDPOINTUPDATEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_BINDPOINTUPDATE(e),
        }
    }
}

impl From<SMSG_PLAYERBOUNDError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PLAYERBOUNDError) -> Self {
        match e {
            SMSG_PLAYERBOUNDError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PLAYERBOUND(e),
        }
    }
}

impl From<SMSG_RESURRECT_REQUESTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_RESURRECT_REQUESTError) -> Self {
        match e {
            SMSG_RESURRECT_REQUESTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_RESURRECT_REQUEST(e),
        }
    }
}

impl From<SMSG_LOOT_RESPONSEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_LOOT_RESPONSEError) -> Self {
        match e {
            SMSG_LOOT_RESPONSEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_LOOT_RESPONSE(e),
        }
    }
}

impl From<SMSG_ITEM_PUSH_RESULTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_ITEM_PUSH_RESULTError) -> Self {
        match e {
            SMSG_ITEM_PUSH_RESULTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_ITEM_PUSH_RESULT(e),
        }
    }
}

impl From<SMSG_DUEL_WINNERError> for ServerOpcodeMessageError {
    fn from(e: SMSG_DUEL_WINNERError) -> Self {
        match e {
            SMSG_DUEL_WINNERError::Io(i) => Self::Io(i),
            _ => Self::SMSG_DUEL_WINNER(e),
        }
    }
}

impl From<SMSG_MOUNTRESULTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_MOUNTRESULTError) -> Self {
        match e {
            SMSG_MOUNTRESULTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_MOUNTRESULT(e),
        }
    }
}

impl From<SMSG_DISMOUNTRESULTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_DISMOUNTRESULTError) -> Self {
        match e {
            SMSG_DISMOUNTRESULTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_DISMOUNTRESULT(e),
        }
    }
}

impl From<SMSG_PET_TAME_FAILUREError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PET_TAME_FAILUREError) -> Self {
        match e {
            SMSG_PET_TAME_FAILUREError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PET_TAME_FAILURE(e),
        }
    }
}

impl From<SMSG_PET_SPELLSError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PET_SPELLSError) -> Self {
        match e {
            SMSG_PET_SPELLSError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PET_SPELLS(e),
        }
    }
}

impl From<SMSG_PET_MODEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PET_MODEError) -> Self {
        match e {
            SMSG_PET_MODEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PET_MODE(e),
        }
    }
}

impl From<SMSG_GOSSIP_MESSAGEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GOSSIP_MESSAGEError) -> Self {
        match e {
            SMSG_GOSSIP_MESSAGEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GOSSIP_MESSAGE(e),
        }
    }
}

impl From<SMSG_NPC_TEXT_UPDATEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_NPC_TEXT_UPDATEError) -> Self {
        match e {
            SMSG_NPC_TEXT_UPDATEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_NPC_TEXT_UPDATE(e),
        }
    }
}

impl From<SMSG_QUESTGIVER_STATUSError> for ServerOpcodeMessageError {
    fn from(e: SMSG_QUESTGIVER_STATUSError) -> Self {
        match e {
            SMSG_QUESTGIVER_STATUSError::Io(i) => Self::Io(i),
            _ => Self::SMSG_QUESTGIVER_STATUS(e),
        }
    }
}

impl From<SMSG_QUESTGIVER_QUEST_LISTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_QUESTGIVER_QUEST_LISTError) -> Self {
        match e {
            SMSG_QUESTGIVER_QUEST_LISTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_QUESTGIVER_QUEST_LIST(e),
        }
    }
}

impl From<SMSG_QUESTGIVER_QUEST_DETAILSError> for ServerOpcodeMessageError {
    fn from(e: SMSG_QUESTGIVER_QUEST_DETAILSError) -> Self {
        match e {
            SMSG_QUESTGIVER_QUEST_DETAILSError::Io(i) => Self::Io(i),
            _ => Self::SMSG_QUESTGIVER_QUEST_DETAILS(e),
        }
    }
}

impl From<SMSG_QUESTGIVER_REQUEST_ITEMSError> for ServerOpcodeMessageError {
    fn from(e: SMSG_QUESTGIVER_REQUEST_ITEMSError) -> Self {
        match e {
            SMSG_QUESTGIVER_REQUEST_ITEMSError::Io(i) => Self::Io(i),
            _ => Self::SMSG_QUESTGIVER_REQUEST_ITEMS(e),
        }
    }
}

impl From<SMSG_QUESTGIVER_OFFER_REWARDError> for ServerOpcodeMessageError {
    fn from(e: SMSG_QUESTGIVER_OFFER_REWARDError) -> Self {
        match e {
            SMSG_QUESTGIVER_OFFER_REWARDError::Io(i) => Self::Io(i),
            _ => Self::SMSG_QUESTGIVER_OFFER_REWARD(e),
        }
    }
}

impl From<SMSG_QUESTGIVER_QUEST_INVALIDError> for ServerOpcodeMessageError {
    fn from(e: SMSG_QUESTGIVER_QUEST_INVALIDError) -> Self {
        match e {
            SMSG_QUESTGIVER_QUEST_INVALIDError::Io(i) => Self::Io(i),
            _ => Self::SMSG_QUESTGIVER_QUEST_INVALID(e),
        }
    }
}

impl From<SMSG_QUESTGIVER_QUEST_FAILEDError> for ServerOpcodeMessageError {
    fn from(e: SMSG_QUESTGIVER_QUEST_FAILEDError) -> Self {
        match e {
            SMSG_QUESTGIVER_QUEST_FAILEDError::Io(i) => Self::Io(i),
            _ => Self::SMSG_QUESTGIVER_QUEST_FAILED(e),
        }
    }
}

impl From<SMSG_QUEST_CONFIRM_ACCEPTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_QUEST_CONFIRM_ACCEPTError) -> Self {
        match e {
            SMSG_QUEST_CONFIRM_ACCEPTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_QUEST_CONFIRM_ACCEPT(e),
        }
    }
}

impl From<SMSG_SELL_ITEMError> for ServerOpcodeMessageError {
    fn from(e: SMSG_SELL_ITEMError) -> Self {
        match e {
            SMSG_SELL_ITEMError::Io(i) => Self::Io(i),
            _ => Self::SMSG_SELL_ITEM(e),
        }
    }
}

impl From<SMSG_BUY_FAILEDError> for ServerOpcodeMessageError {
    fn from(e: SMSG_BUY_FAILEDError) -> Self {
        match e {
            SMSG_BUY_FAILEDError::Io(i) => Self::Io(i),
            _ => Self::SMSG_BUY_FAILED(e),
        }
    }
}

impl From<SMSG_ACTIVATETAXIREPLYError> for ServerOpcodeMessageError {
    fn from(e: SMSG_ACTIVATETAXIREPLYError) -> Self {
        match e {
            SMSG_ACTIVATETAXIREPLYError::Io(i) => Self::Io(i),
            _ => Self::SMSG_ACTIVATETAXIREPLY(e),
        }
    }
}

impl From<SMSG_TRAINER_LISTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_TRAINER_LISTError) -> Self {
        match e {
            SMSG_TRAINER_LISTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_TRAINER_LIST(e),
        }
    }
}

impl From<SMSG_TRAINER_BUY_FAILEDError> for ServerOpcodeMessageError {
    fn from(e: SMSG_TRAINER_BUY_FAILEDError) -> Self {
        match e {
            SMSG_TRAINER_BUY_FAILEDError::Io(i) => Self::Io(i),
            _ => Self::SMSG_TRAINER_BUY_FAILED(e),
        }
    }
}

impl From<SMSG_BUY_BANK_SLOT_RESULTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_BUY_BANK_SLOT_RESULTError) -> Self {
        match e {
            SMSG_BUY_BANK_SLOT_RESULTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_BUY_BANK_SLOT_RESULT(e),
        }
    }
}

impl From<SMSG_PETITION_SIGN_RESULTSError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PETITION_SIGN_RESULTSError) -> Self {
        match e {
            SMSG_PETITION_SIGN_RESULTSError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PETITION_SIGN_RESULTS(e),
        }
    }
}

impl From<SMSG_TURN_IN_PETITION_RESULTSError> for ServerOpcodeMessageError {
    fn from(e: SMSG_TURN_IN_PETITION_RESULTSError) -> Self {
        match e {
            SMSG_TURN_IN_PETITION_RESULTSError::Io(i) => Self::Io(i),
            _ => Self::SMSG_TURN_IN_PETITION_RESULTS(e),
        }
    }
}

impl From<SMSG_PETITION_QUERY_RESPONSEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PETITION_QUERY_RESPONSEError) -> Self {
        match e {
            SMSG_PETITION_QUERY_RESPONSEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PETITION_QUERY_RESPONSE(e),
        }
    }
}

impl From<SMSG_NOTIFICATIONError> for ServerOpcodeMessageError {
    fn from(e: SMSG_NOTIFICATIONError) -> Self {
        match e {
            SMSG_NOTIFICATIONError::Io(i) => Self::Io(i),
            _ => Self::SMSG_NOTIFICATION(e),
        }
    }
}

impl From<SMSG_LOG_XPGAINError> for ServerOpcodeMessageError {
    fn from(e: SMSG_LOG_XPGAINError) -> Self {
        match e {
            SMSG_LOG_XPGAINError::Io(i) => Self::Io(i),
            _ => Self::SMSG_LOG_XPGAIN(e),
        }
    }
}

impl From<SMSG_START_MIRROR_TIMERError> for ServerOpcodeMessageError {
    fn from(e: SMSG_START_MIRROR_TIMERError) -> Self {
        match e {
            SMSG_START_MIRROR_TIMERError::Io(i) => Self::Io(i),
            _ => Self::SMSG_START_MIRROR_TIMER(e),
        }
    }
}

impl From<SMSG_PAUSE_MIRROR_TIMERError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PAUSE_MIRROR_TIMERError) -> Self {
        match e {
            SMSG_PAUSE_MIRROR_TIMERError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PAUSE_MIRROR_TIMER(e),
        }
    }
}

impl From<SMSG_STOP_MIRROR_TIMERError> for ServerOpcodeMessageError {
    fn from(e: SMSG_STOP_MIRROR_TIMERError) -> Self {
        match e {
            SMSG_STOP_MIRROR_TIMERError::Io(i) => Self::Io(i),
            _ => Self::SMSG_STOP_MIRROR_TIMER(e),
        }
    }
}

impl From<SMSG_AUTH_RESPONSEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_AUTH_RESPONSEError) -> Self {
        match e {
            SMSG_AUTH_RESPONSEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_AUTH_RESPONSE(e),
        }
    }
}

impl From<MSG_SAVE_GUILD_EMBLEM_ServerError> for ServerOpcodeMessageError {
    fn from(e: MSG_SAVE_GUILD_EMBLEM_ServerError) -> Self {
        match e {
            MSG_SAVE_GUILD_EMBLEM_ServerError::Io(i) => Self::Io(i),
            _ => Self::MSG_SAVE_GUILD_EMBLEM(e),
        }
    }
}

impl From<SMSG_EXPLORATION_EXPERIENCEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_EXPLORATION_EXPERIENCEError) -> Self {
        match e {
            SMSG_EXPLORATION_EXPERIENCEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_EXPLORATION_EXPERIENCE(e),
        }
    }
}

impl From<SMSG_ENVIRONMENTALDAMAGELOGError> for ServerOpcodeMessageError {
    fn from(e: SMSG_ENVIRONMENTALDAMAGELOGError) -> Self {
        match e {
            SMSG_ENVIRONMENTALDAMAGELOGError::Io(i) => Self::Io(i),
            _ => Self::SMSG_ENVIRONMENTALDAMAGELOG(e),
        }
    }
}

impl From<SMSG_GMTICKET_CREATEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GMTICKET_CREATEError) -> Self {
        match e {
            SMSG_GMTICKET_CREATEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GMTICKET_CREATE(e),
        }
    }
}

impl From<SMSG_GMTICKET_UPDATETEXTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GMTICKET_UPDATETEXTError) -> Self {
        match e {
            SMSG_GMTICKET_UPDATETEXTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GMTICKET_UPDATETEXT(e),
        }
    }
}

impl From<SMSG_GMTICKET_GETTICKETError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GMTICKET_GETTICKETError) -> Self {
        match e {
            SMSG_GMTICKET_GETTICKETError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GMTICKET_GETTICKET(e),
        }
    }
}

impl From<MSG_CORPSE_QUERY_ServerError> for ServerOpcodeMessageError {
    fn from(e: MSG_CORPSE_QUERY_ServerError) -> Self {
        match e {
            MSG_CORPSE_QUERY_ServerError::Io(i) => Self::Io(i),
            _ => Self::MSG_CORPSE_QUERY(e),
        }
    }
}

impl From<SMSG_GMTICKET_DELETETICKETError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GMTICKET_DELETETICKETError) -> Self {
        match e {
            SMSG_GMTICKET_DELETETICKETError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GMTICKET_DELETETICKET(e),
        }
    }
}

impl From<SMSG_GOSSIP_POIError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GOSSIP_POIError) -> Self {
        match e {
            SMSG_GOSSIP_POIError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GOSSIP_POI(e),
        }
    }
}

impl From<SMSG_LOGIN_VERIFY_WORLDError> for ServerOpcodeMessageError {
    fn from(e: SMSG_LOGIN_VERIFY_WORLDError) -> Self {
        match e {
            SMSG_LOGIN_VERIFY_WORLDError::Io(i) => Self::Io(i),
            _ => Self::SMSG_LOGIN_VERIFY_WORLD(e),
        }
    }
}

impl From<SMSG_MAIL_LIST_RESULTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_MAIL_LIST_RESULTError) -> Self {
        match e {
            SMSG_MAIL_LIST_RESULTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_MAIL_LIST_RESULT(e),
        }
    }
}

impl From<SMSG_BATTLEFIELD_LISTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_BATTLEFIELD_LISTError) -> Self {
        match e {
            SMSG_BATTLEFIELD_LISTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_BATTLEFIELD_LIST(e),
        }
    }
}

impl From<SMSG_ITEM_TEXT_QUERY_RESPONSEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_ITEM_TEXT_QUERY_RESPONSEError) -> Self {
        match e {
            SMSG_ITEM_TEXT_QUERY_RESPONSEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(e),
        }
    }
}

impl From<SMSG_SPELLLOGMISSError> for ServerOpcodeMessageError {
    fn from(e: SMSG_SPELLLOGMISSError) -> Self {
        match e {
            SMSG_SPELLLOGMISSError::Io(i) => Self::Io(i),
            _ => Self::SMSG_SPELLLOGMISS(e),
        }
    }
}

impl From<SMSG_SPELLLOGEXECUTEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_SPELLLOGEXECUTEError) -> Self {
        match e {
            SMSG_SPELLLOGEXECUTEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_SPELLLOGEXECUTE(e),
        }
    }
}

impl From<SMSG_PERIODICAURALOGError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PERIODICAURALOGError) -> Self {
        match e {
            SMSG_PERIODICAURALOGError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PERIODICAURALOG(e),
        }
    }
}

impl From<SMSG_SPELLDAMAGESHIELDError> for ServerOpcodeMessageError {
    fn from(e: SMSG_SPELLDAMAGESHIELDError) -> Self {
        match e {
            SMSG_SPELLDAMAGESHIELDError::Io(i) => Self::Io(i),
            _ => Self::SMSG_SPELLDAMAGESHIELD(e),
        }
    }
}

impl From<SMSG_SPELLNONMELEEDAMAGELOGError> for ServerOpcodeMessageError {
    fn from(e: SMSG_SPELLNONMELEEDAMAGELOGError) -> Self {
        match e {
            SMSG_SPELLNONMELEEDAMAGELOGError::Io(i) => Self::Io(i),
            _ => Self::SMSG_SPELLNONMELEEDAMAGELOG(e),
        }
    }
}

impl From<SMSG_PROCRESISTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PROCRESISTError) -> Self {
        match e {
            SMSG_PROCRESISTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PROCRESIST(e),
        }
    }
}

impl From<MSG_LIST_STABLED_PETS_ServerError> for ServerOpcodeMessageError {
    fn from(e: MSG_LIST_STABLED_PETS_ServerError) -> Self {
        match e {
            MSG_LIST_STABLED_PETS_ServerError::Io(i) => Self::Io(i),
            _ => Self::MSG_LIST_STABLED_PETS(e),
        }
    }
}

impl From<SMSG_STABLE_RESULTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_STABLE_RESULTError) -> Self {
        match e {
            SMSG_STABLE_RESULTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_STABLE_RESULT(e),
        }
    }
}

impl From<SMSG_RAID_GROUP_ONLYError> for ServerOpcodeMessageError {
    fn from(e: SMSG_RAID_GROUP_ONLYError) -> Self {
        match e {
            SMSG_RAID_GROUP_ONLYError::Io(i) => Self::Io(i),
            _ => Self::SMSG_RAID_GROUP_ONLY(e),
        }
    }
}

impl From<SMSG_PVP_CREDITError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PVP_CREDITError) -> Self {
        match e {
            SMSG_PVP_CREDITError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PVP_CREDIT(e),
        }
    }
}

impl From<SMSG_SERVER_MESSAGEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_SERVER_MESSAGEError) -> Self {
        match e {
            SMSG_SERVER_MESSAGEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_SERVER_MESSAGE(e),
        }
    }
}

impl From<SMSG_MEETINGSTONE_SETQUEUEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_MEETINGSTONE_SETQUEUEError) -> Self {
        match e {
            SMSG_MEETINGSTONE_SETQUEUEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_MEETINGSTONE_SETQUEUE(e),
        }
    }
}

impl From<SMSG_STANDSTATE_UPDATEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_STANDSTATE_UPDATEError) -> Self {
        match e {
            SMSG_STANDSTATE_UPDATEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_STANDSTATE_UPDATE(e),
        }
    }
}

impl From<SMSG_LOOT_ROLL_WONError> for ServerOpcodeMessageError {
    fn from(e: SMSG_LOOT_ROLL_WONError) -> Self {
        match e {
            SMSG_LOOT_ROLL_WONError::Io(i) => Self::Io(i),
            _ => Self::SMSG_LOOT_ROLL_WON(e),
        }
    }
}

impl From<SMSG_LOOT_ROLLError> for ServerOpcodeMessageError {
    fn from(e: SMSG_LOOT_ROLLError) -> Self {
        match e {
            SMSG_LOOT_ROLLError::Io(i) => Self::Io(i),
            _ => Self::SMSG_LOOT_ROLL(e),
        }
    }
}

impl From<SMSG_CHAT_PLAYER_NOT_FOUNDError> for ServerOpcodeMessageError {
    fn from(e: SMSG_CHAT_PLAYER_NOT_FOUNDError) -> Self {
        match e {
            SMSG_CHAT_PLAYER_NOT_FOUNDError::Io(i) => Self::Io(i),
            _ => Self::SMSG_CHAT_PLAYER_NOT_FOUND(e),
        }
    }
}

impl From<SMSG_MONSTER_MOVE_TRANSPORTError> for ServerOpcodeMessageError {
    fn from(e: SMSG_MONSTER_MOVE_TRANSPORTError) -> Self {
        match e {
            SMSG_MONSTER_MOVE_TRANSPORTError::Io(i) => Self::Io(i),
            _ => Self::SMSG_MONSTER_MOVE_TRANSPORT(e),
        }
    }
}

impl From<SMSG_AREA_TRIGGER_MESSAGEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_AREA_TRIGGER_MESSAGEError) -> Self {
        match e {
            SMSG_AREA_TRIGGER_MESSAGEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_AREA_TRIGGER_MESSAGE(e),
        }
    }
}

impl From<SMSG_MEETINGSTONE_JOINFAILEDError> for ServerOpcodeMessageError {
    fn from(e: SMSG_MEETINGSTONE_JOINFAILEDError) -> Self {
        match e {
            SMSG_MEETINGSTONE_JOINFAILEDError::Io(i) => Self::Io(i),
            _ => Self::SMSG_MEETINGSTONE_JOINFAILED(e),
        }
    }
}

impl From<SMSG_INIT_WORLD_STATESError> for ServerOpcodeMessageError {
    fn from(e: SMSG_INIT_WORLD_STATESError) -> Self {
        match e {
            SMSG_INIT_WORLD_STATESError::Io(i) => Self::Io(i),
            _ => Self::SMSG_INIT_WORLD_STATES(e),
        }
    }
}

impl From<SMSG_ITEM_NAME_QUERY_RESPONSEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_ITEM_NAME_QUERY_RESPONSEError) -> Self {
        match e {
            SMSG_ITEM_NAME_QUERY_RESPONSEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_ITEM_NAME_QUERY_RESPONSE(e),
        }
    }
}

impl From<SMSG_PET_ACTION_FEEDBACKError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PET_ACTION_FEEDBACKError) -> Self {
        match e {
            SMSG_PET_ACTION_FEEDBACKError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PET_ACTION_FEEDBACK(e),
        }
    }
}

impl From<SMSG_CHAR_RENAMEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_CHAR_RENAMEError) -> Self {
        match e {
            SMSG_CHAR_RENAMEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_CHAR_RENAME(e),
        }
    }
}

impl From<SMSG_RAID_INSTANCE_INFOError> for ServerOpcodeMessageError {
    fn from(e: SMSG_RAID_INSTANCE_INFOError) -> Self {
        match e {
            SMSG_RAID_INSTANCE_INFOError::Io(i) => Self::Io(i),
            _ => Self::SMSG_RAID_INSTANCE_INFO(e),
        }
    }
}

impl From<SMSG_BATTLEFIELD_STATUSError> for ServerOpcodeMessageError {
    fn from(e: SMSG_BATTLEFIELD_STATUSError) -> Self {
        match e {
            SMSG_BATTLEFIELD_STATUSError::Io(i) => Self::Io(i),
            _ => Self::SMSG_BATTLEFIELD_STATUS(e),
        }
    }
}

impl From<MSG_INSPECT_HONOR_STATS_ServerError> for ServerOpcodeMessageError {
    fn from(e: MSG_INSPECT_HONOR_STATS_ServerError) -> Self {
        match e {
            MSG_INSPECT_HONOR_STATS_ServerError::Io(i) => Self::Io(i),
            _ => Self::MSG_INSPECT_HONOR_STATS(e),
        }
    }
}

impl From<MSG_PVP_LOG_DATA_ServerError> for ServerOpcodeMessageError {
    fn from(e: MSG_PVP_LOG_DATA_ServerError) -> Self {
        match e {
            MSG_PVP_LOG_DATA_ServerError::Io(i) => Self::Io(i),
            _ => Self::MSG_PVP_LOG_DATA(e),
        }
    }
}

impl From<SMSG_GROUP_JOINED_BATTLEGROUNDError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GROUP_JOINED_BATTLEGROUNDError) -> Self {
        match e {
            SMSG_GROUP_JOINED_BATTLEGROUNDError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GROUP_JOINED_BATTLEGROUND(e),
        }
    }
}

impl From<SMSG_PARTY_MEMBER_STATS_FULLError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PARTY_MEMBER_STATS_FULLError) -> Self {
        match e {
            SMSG_PARTY_MEMBER_STATS_FULLError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PARTY_MEMBER_STATS_FULL(e),
        }
    }
}

impl From<SMSG_WEATHERError> for ServerOpcodeMessageError {
    fn from(e: SMSG_WEATHERError) -> Self {
        match e {
            SMSG_WEATHERError::Io(i) => Self::Io(i),
            _ => Self::SMSG_WEATHER(e),
        }
    }
}

impl From<SMSG_RAID_INSTANCE_MESSAGEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_RAID_INSTANCE_MESSAGEError) -> Self {
        match e {
            SMSG_RAID_INSTANCE_MESSAGEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_RAID_INSTANCE_MESSAGE(e),
        }
    }
}

impl From<SMSG_INSTANCE_RESETError> for ServerOpcodeMessageError {
    fn from(e: SMSG_INSTANCE_RESETError) -> Self {
        match e {
            SMSG_INSTANCE_RESETError::Io(i) => Self::Io(i),
            _ => Self::SMSG_INSTANCE_RESET(e),
        }
    }
}

impl From<SMSG_INSTANCE_RESET_FAILEDError> for ServerOpcodeMessageError {
    fn from(e: SMSG_INSTANCE_RESET_FAILEDError) -> Self {
        match e {
            SMSG_INSTANCE_RESET_FAILEDError::Io(i) => Self::Io(i),
            _ => Self::SMSG_INSTANCE_RESET_FAILED(e),
        }
    }
}

impl From<SMSG_UPDATE_LAST_INSTANCEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_UPDATE_LAST_INSTANCEError) -> Self {
        match e {
            SMSG_UPDATE_LAST_INSTANCEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_UPDATE_LAST_INSTANCE(e),
        }
    }
}

impl From<MSG_RAID_TARGET_UPDATE_ServerError> for ServerOpcodeMessageError {
    fn from(e: MSG_RAID_TARGET_UPDATE_ServerError) -> Self {
        match e {
            MSG_RAID_TARGET_UPDATE_ServerError::Io(i) => Self::Io(i),
            _ => Self::MSG_RAID_TARGET_UPDATE(e),
        }
    }
}

impl From<SMSG_PET_ACTION_SOUNDError> for ServerOpcodeMessageError {
    fn from(e: SMSG_PET_ACTION_SOUNDError) -> Self {
        match e {
            SMSG_PET_ACTION_SOUNDError::Io(i) => Self::Io(i),
            _ => Self::SMSG_PET_ACTION_SOUND(e),
        }
    }
}

impl From<SMSG_GM_TICKET_STATUS_UPDATEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_GM_TICKET_STATUS_UPDATEError) -> Self {
        match e {
            SMSG_GM_TICKET_STATUS_UPDATEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_GM_TICKET_STATUS_UPDATE(e),
        }
    }
}

impl From<SMSG_EXPECTED_SPAM_RECORDSError> for ServerOpcodeMessageError {
    fn from(e: SMSG_EXPECTED_SPAM_RECORDSError) -> Self {
        match e {
            SMSG_EXPECTED_SPAM_RECORDSError::Io(i) => Self::Io(i),
            _ => Self::SMSG_EXPECTED_SPAM_RECORDS(e),
        }
    }
}

impl From<SMSG_DEFENSE_MESSAGEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_DEFENSE_MESSAGEError) -> Self {
        match e {
            SMSG_DEFENSE_MESSAGEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_DEFENSE_MESSAGE(e),
        }
    }
}

