## Client Version 1.12

### Wowm Representation
```rust,ignore
flag CastFlags : u16 {
    NONE = 0x00000000;
    HIDDEN_COMBATLOG = 0x00000001;
    UNKNOWN2 = 0x00000002;
    UNKNOWN3 = 0x00000004;
    UNKNOWN4 = 0x00000008;
    UNKNOWN5 = 0x00000010;
    AMMO = 0x00000020;
    UNKNOWN7 = 0x00000040;
    UNKNOWN8 = 0x00000080;
    UNKNOWN9 = 0x00000100;
}
```
### Type
The basic type is `u16`, a 2 byte (16 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  |  |
| `HIDDEN_COMBATLOG` | 1 (0x01) |  | mangoszero/cmangos/vmangos: hide in combat log? |
| `UNKNOWN2` | 2 (0x02) |  |  |
| `UNKNOWN3` | 4 (0x04) |  |  |
| `UNKNOWN4` | 8 (0x08) |  |  |
| `UNKNOWN5` | 16 (0x10) |  |  |
| `AMMO` | 32 (0x20) |  | cmangos/vmangos/mangoszero: Projectiles visual |
| `UNKNOWN7` | 64 (0x40) |  | cmangos/vmangos/mangoszero: !0x41 mask used to call CGTradeSkillInfo::DoRecast |
| `UNKNOWN8` | 128 (0x80) |  |  |
| `UNKNOWN9` | 256 (0x100) |  |  |