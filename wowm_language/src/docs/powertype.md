## Client Version 1.12

### Wowm Representation
```rust,ignore
enum PowerType : u32 {
    MANA = 0;
    RAGE = 1;
    FOCUS = 2;
    ENERGY = 3;
    HAPPINESS = 4;
    HEALTH = 0xFFFFFFFE;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `MANA` | 0 (0x00) |  | UNIT_FIELD_POWER1 |
| `RAGE` | 1 (0x01) |  | UNIT_FIELD_POWER2 |
| `FOCUS` | 2 (0x02) |  | UNIT_FIELD_POWER3 |
| `ENERGY` | 3 (0x03) |  | UNIT_FIELD_POWER4 |
| `HAPPINESS` | 4 (0x04) |  | UNIT_FIELD_POWER5 |
| `HEALTH` | 4294967294 (0xFFFFFFFE) |  | (-2 as signed value) |