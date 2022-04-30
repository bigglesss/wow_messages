## Protocol Version 2, Protocol Version 3

### Wowm Representation
```rust,ignore
flag RealmFlag : u8 {
    NONE = 0x00;
    INVALID = 0x01;
    OFFLINE = 0x02;
    FORCE_BLUE_RECOMMENDED = 0x20;
    FORCE_GREEN_RECOMMENDED = 0x40;
    FORCE_RED_FULL = 0x80;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  |  |
| `INVALID` | 1 (0x01) |  |  |
| `OFFLINE` | 2 (0x02) |  |  |
| `FORCE_BLUE_RECOMMENDED` | 32 (0x20) |  |  |
| `FORCE_GREEN_RECOMMENDED` | 64 (0x40) |  |  |
| `FORCE_RED_FULL` | 128 (0x80) |  |  |
## Protocol Version 8

### Wowm Representation
```rust,ignore
flag RealmFlag : u8 {
    NONE = 0x00;
    INVALID = 0x01;
    OFFLINE = 0x02;
    SPECIFY_BUILD = 0x04;
    FORCE_BLUE_RECOMMENDED = 0x20;
    FORCE_GREEN_RECOMMENDED = 0x40;
    FORCE_RED_FULL = 0x80;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  |  |
| `INVALID` | 1 (0x01) |  |  |
| `OFFLINE` | 2 (0x02) |  |  |
| `SPECIFY_BUILD` | 4 (0x04) |  |  |
| `FORCE_BLUE_RECOMMENDED` | 32 (0x20) |  |  |
| `FORCE_GREEN_RECOMMENDED` | 64 (0x40) |  |  |
| `FORCE_RED_FULL` | 128 (0x80) |  |  |