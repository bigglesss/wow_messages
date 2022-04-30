## Client Version 1.12

### Comment

cmangos has one instance of this be u32, but both vmangos/mangoszero are u8

### Wowm Representation
```rust,ignore
enum QuestPartyMessage : u8 {
    SHARING_QUEST = 0;
    CANT_TAKE_QUEST = 1;
    ACCEPT_QUEST = 2;
    DECLINE_QUEST = 3;
    TOO_FAR = 4;
    BUSY = 5;
    LOG_FULL = 6;
    HAVE_QUEST = 7;
    FINISH_QUEST = 8;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `SHARING_QUEST` | 0 (0x00) |  | ERR_QUEST_PUSH_SUCCESS_S |
| `CANT_TAKE_QUEST` | 1 (0x01) |  | ERR_QUEST_PUSH_INVALID_S |
| `ACCEPT_QUEST` | 2 (0x02) |  | ERR_QUEST_PUSH_ACCEPTED_S |
| `DECLINE_QUEST` | 3 (0x03) |  | ERR_QUEST_PUSH_DECLINED_S |
| `TOO_FAR` | 4 (0x04) |  | removed in 3.x |
| `BUSY` | 5 (0x05) |  | ERR_QUEST_PUSH_BUSY_S |
| `LOG_FULL` | 6 (0x06) |  | ERR_QUEST_PUSH_LOG_FULL_S |
| `HAVE_QUEST` | 7 (0x07) |  | ERR_QUEST_PUSH_ONQUEST_S |
| `FINISH_QUEST` | 8 (0x08) |  | ERR_QUEST_PUSH_ALREADY_DONE_S |