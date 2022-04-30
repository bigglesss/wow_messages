## Client Version 1.12

### Wowm Representation
```rust,ignore
enum PartyResult : u8 {
    SUCCESS = 0;
    BAD_PLAYER_NAME = 1;
    TARGET_NOT_IN_GROUP = 2;
    GROUP_FULL = 3;
    ALREADY_IN_GROUP = 4;
    NOT_IN_GROUP = 5;
    NOT_LEADER = 6;
    PLAYER_WRONG_FACTION = 7;
    IGNORING_YOU = 8;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `SUCCESS` | 0 (0x00) |  |  |
| `BAD_PLAYER_NAME` | 1 (0x01) |  |  |
| `TARGET_NOT_IN_GROUP` | 2 (0x02) |  |  |
| `GROUP_FULL` | 3 (0x03) |  |  |
| `ALREADY_IN_GROUP` | 4 (0x04) |  |  |
| `NOT_IN_GROUP` | 5 (0x05) |  |  |
| `NOT_LEADER` | 6 (0x06) |  |  |
| `PLAYER_WRONG_FACTION` | 7 (0x07) |  |  |
| `IGNORING_YOU` | 8 (0x08) |  |  |