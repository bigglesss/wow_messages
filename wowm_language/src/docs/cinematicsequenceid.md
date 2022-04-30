## Client Version 1.12

### Wowm Representation
```rust,ignore
enum CinematicSequenceId : u32 {
    HUMAN = 81;
    ORC = 21;
    DWARF = 41;
    NIGHT_ELF = 61;
    UNDEAD = 2;
    TAUREN = 141;
    GNOME = 101;
    TROLL = 121;
    GOBLIN = 0;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `HUMAN` | 81 (0x51) |  |  |
| `ORC` | 21 (0x15) |  |  |
| `DWARF` | 41 (0x29) |  |  |
| `NIGHT_ELF` | 61 (0x3D) |  |  |
| `UNDEAD` | 2 (0x02) |  |  |
| `TAUREN` | 141 (0x8D) |  |  |
| `GNOME` | 101 (0x65) |  |  |
| `TROLL` | 121 (0x79) |  |  |
| `GOBLIN` | 0 (0x00) |  |  |