## Client Version 1.12

### Wowm Representation
```rust,ignore
enum SpellMissInfo : u32 {
    NONE = 0;
    MISS = 1;
    RESIST = 2;
    DODGE = 3;
    PARRY = 4;
    BLOCK = 5;
    EVADE = 6;
    IMMUNE = 7;
    IMMUNE2 = 8;
    DEFLECT = 9;
    ABSORB = 10;
    REFLECT = 11;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  |  |
| `MISS` | 1 (0x01) |  |  |
| `RESIST` | 2 (0x02) |  |  |
| `DODGE` | 3 (0x03) |  |  |
| `PARRY` | 4 (0x04) |  |  |
| `BLOCK` | 5 (0x05) |  |  |
| `EVADE` | 6 (0x06) |  |  |
| `IMMUNE` | 7 (0x07) |  |  |
| `IMMUNE2` | 8 (0x08) |  |  |
| `DEFLECT` | 9 (0x09) |  |  |
| `ABSORB` | 10 (0x0A) |  |  |
| `REFLECT` | 11 (0x0B) |  |  |