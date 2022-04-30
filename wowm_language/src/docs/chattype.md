## Client Version 1.12

### Wowm Representation
```rust,ignore
enum ChatType : u8 {
    SAY = 0x00;
    PARTY = 0x01;
    RAID = 0x02;
    GUILD = 0x03;
    OFFICER = 0x04;
    YELL = 0x05;
    WHISPER = 0x06;
    WHISPER_INFORM = 0x07;
    EMOTE = 0x08;
    TEXT_EMOTE = 0x09;
    SYSTEM = 0x0A;
    MONSTER_SAY = 0x0B;
    MONSTER_YELL = 0x0C;
    MONSTER_EMOTE = 0x0D;
    CHANNEL = 0x0E;
    CHANNEL_JOIN = 0x0F;
    CHANNEL_LEAVE = 0x10;
    CHANNEL_LIST = 0x11;
    CHANNEL_NOTICE = 0x12;
    CHANNEL_NOTICE_USER = 0x13;
    AFK = 0x14;
    DND = 0x15;
    IGNORED = 0x16;
    SKILL = 0x17;
    LOOT = 0x18;
    MONSTER_WHISPER = 0x1A;
    BG_SYSTEM_NEUTRAL = 0x52;
    BG_SYSTEM_ALLIANCE = 0x53;
    BG_SYSTEM_HORDE = 0x54;
    RAID_LEADER = 0x57;
    RAID_WARNING = 0x58;
    RAID_BOSS_WHISPER = 0x59;
    RAID_BOSS_EMOTE = 0x5A;
    BATTLEGROUND = 0x5C;
    BATTLEGROUND_LEADER = 0x5D;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `SAY` | 0 (0x00) |  |  |
| `PARTY` | 1 (0x01) |  |  |
| `RAID` | 2 (0x02) |  |  |
| `GUILD` | 3 (0x03) |  |  |
| `OFFICER` | 4 (0x04) |  |  |
| `YELL` | 5 (0x05) |  |  |
| `WHISPER` | 6 (0x06) |  |  |
| `WHISPER_INFORM` | 7 (0x07) |  |  |
| `EMOTE` | 8 (0x08) |  |  |
| `TEXT_EMOTE` | 9 (0x09) |  |  |
| `SYSTEM` | 10 (0x0A) |  |  |
| `MONSTER_SAY` | 11 (0x0B) |  |  |
| `MONSTER_YELL` | 12 (0x0C) |  |  |
| `MONSTER_EMOTE` | 13 (0x0D) |  |  |
| `CHANNEL` | 14 (0x0E) |  |  |
| `CHANNEL_JOIN` | 15 (0x0F) |  |  |
| `CHANNEL_LEAVE` | 16 (0x10) |  |  |
| `CHANNEL_LIST` | 17 (0x11) |  |  |
| `CHANNEL_NOTICE` | 18 (0x12) |  |  |
| `CHANNEL_NOTICE_USER` | 19 (0x13) |  |  |
| `AFK` | 20 (0x14) |  |  |
| `DND` | 21 (0x15) |  |  |
| `IGNORED` | 22 (0x16) |  |  |
| `SKILL` | 23 (0x17) |  |  |
| `LOOT` | 24 (0x18) |  |  |
| `MONSTER_WHISPER` | 26 (0x1A) |  |  |
| `BG_SYSTEM_NEUTRAL` | 82 (0x52) |  |  |
| `BG_SYSTEM_ALLIANCE` | 83 (0x53) |  |  |
| `BG_SYSTEM_HORDE` | 84 (0x54) |  |  |
| `RAID_LEADER` | 87 (0x57) |  |  |
| `RAID_WARNING` | 88 (0x58) |  |  |
| `RAID_BOSS_WHISPER` | 89 (0x59) |  |  |
| `RAID_BOSS_EMOTE` | 90 (0x5A) |  |  |
| `BATTLEGROUND` | 92 (0x5C) |  |  |
| `BATTLEGROUND_LEADER` | 93 (0x5D) |  |  |