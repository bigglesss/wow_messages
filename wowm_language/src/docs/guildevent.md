## Client Version 1.12

### Wowm Representation
```rust,ignore
enum GuildEvent : u8 {
    PROMOTION = 0;
    DEMOTION = 1;
    MOTD = 2;
    JOINED = 3;
    LEFT = 4;
    REMOVED = 5;
    LEADER_IS = 6;
    LEADER_CHANGED = 7;
    DISBANDED = 8;
    TABARD_CHANGED = 9;
    UNKNOWN10 = 10;
    ROSTER_UPDATE = 11;
    SIGNED_ON = 12;
    SIGNED_OFF = 13;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `PROMOTION` | 0 (0x00) |  |  |
| `DEMOTION` | 1 (0x01) |  |  |
| `MOTD` | 2 (0x02) |  |  |
| `JOINED` | 3 (0x03) |  |  |
| `LEFT` | 4 (0x04) |  |  |
| `REMOVED` | 5 (0x05) |  |  |
| `LEADER_IS` | 6 (0x06) |  |  |
| `LEADER_CHANGED` | 7 (0x07) |  |  |
| `DISBANDED` | 8 (0x08) |  |  |
| `TABARD_CHANGED` | 9 (0x09) |  |  |
| `UNKNOWN10` | 10 (0x0A) |  |  |
| `ROSTER_UPDATE` | 11 (0x0B) |  |  |
| `SIGNED_ON` | 12 (0x0C) |  |  |
| `SIGNED_OFF` | 13 (0x0D) |  |  |