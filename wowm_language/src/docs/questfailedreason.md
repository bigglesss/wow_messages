## Client Version 1.12

### Wowm Representation
```rust,ignore
enum QuestFailedReason : u32 {
    DONT_HAVE_REQ = 0;
    QUEST_FAILED_LOW_LEVEL = 1;
    QUEST_FAILED_REQS = 2;
    QUEST_FAILED_INVENTORY_FULL = 4;
    QUEST_FAILED_WRONG_RACE = 6;
    QUEST_ONLY_ONE_TIMED = 12;
    QUEST_ALREADY_ON = 13;
    QUEST_FAILED_DUPLICATE_ITEM = 17;
    QUEST_FAILED_MISSING_ITEMS = 20;
    QUEST_FAILED_NOT_ENOUGH_MONEY = 22;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `DONT_HAVE_REQ` | 0 (0x00) |  | this is default case |
| `QUEST_FAILED_LOW_LEVEL` | 1 (0x01) |  | You are not high enough level for that quest. |
| `QUEST_FAILED_REQS` | 2 (0x02) |  | You don't meet the requirements for that quest. |
| `QUEST_FAILED_INVENTORY_FULL` | 4 (0x04) |  | Inventory is full. (Also 50. From SMSG_QUESTGIVER_QUEST_FAILED) |
| `QUEST_FAILED_WRONG_RACE` | 6 (0x06) |  | That quest is not available to your race. |
| `QUEST_ONLY_ONE_TIMED` | 12 (0x0C) |  | You can only be on one timed quest at a time. |
| `QUEST_ALREADY_ON` | 13 (0x0D) |  | You are already on that quest. |
| `QUEST_FAILED_DUPLICATE_ITEM` | 17 (0x11) |  | Duplicate item found. (From SMSG_QUESTGIVER_QUEST_FAILED) |
| `QUEST_FAILED_MISSING_ITEMS` | 20 (0x14) |  | You don't have the required items with you. Check storage. |
| `QUEST_FAILED_NOT_ENOUGH_MONEY` | 22 (0x16) |  | You don't have enough money for that quest. |