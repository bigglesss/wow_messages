## Client Version 1.12

### Wowm Representation
```rust,ignore
enum MonsterMoveType : u8 {
    NORMAL = 0;
    STOP = 1;
    FACING_SPOT = 2;
    FACING_TARGET = 3;
    FACING_ANGLE = 4;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NORMAL` | 0 (0x00) |  |  |
| `STOP` | 1 (0x01) |  |  |
| `FACING_SPOT` | 2 (0x02) |  |  |
| `FACING_TARGET` | 3 (0x03) |  |  |
| `FACING_ANGLE` | 4 (0x04) |  |  |