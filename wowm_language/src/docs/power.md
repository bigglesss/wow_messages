## Client Version 1.12

### Wowm Representation
```rust,ignore
enum Power : u8 {
    MANA = 0;
    RAGE = 1;
    FOCUS = 2;
    ENERGY = 3;
    HAPPINESS = 4;
    MAX_POWERS = 5;
    ALL = 127;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `MANA` | 0 (0x00) |  | The most common one, mobs usually have this or rage |
| `RAGE` | 1 (0x01) |  | This is what warriors use to cast their spells |
| `FOCUS` | 2 (0x02) |  | Used by hunters after Cataclysm (4.x) |
| `ENERGY` | 3 (0x03) |  | Used by rouges to do their spells |
| `HAPPINESS` | 4 (0x04) |  | Hunter's pet's happiness affect their damage |
| `MAX_POWERS` | 5 (0x05) |  |  |
| `ALL` | 127 (0x7F) |  | default for class? - need check for TBC |