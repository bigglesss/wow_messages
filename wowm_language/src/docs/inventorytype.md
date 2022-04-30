## Client Version 1.12

### Wowm Representation
```rust,ignore
enum InventoryType : u8 {
    NON_EQUIP = 0;
    HEAD = 1;
    NECK_OR_RELIC = 2;
    SHOULDERS = 3;
    BODY = 4;
    CHEST = 5;
    WAIST = 6;
    LEGS = 7;
    FEET = 8;
    WRISTS = 9;
    HANDS = 10;
    FINGER = 11;
    TRINKET = 12;
    WEAPON = 13;
    SHIELD = 14;
    RANGED = 15;
    CLOAK = 16;
    TWO_HANDED_WEAPON = 17;
    BAG = 18;
    TABARD = 19;
    ROBE = 20;
    WEAPON_MAIN_HAND = 21;
    WEAPON_OFF_HAND = 22;
    HOLDABLE = 23;
    AMMO = 24;
    THROWN = 25;
    RANGED_RIGHT = 26;
    QUIVER = 27;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NON_EQUIP` | 0 (0x00) |  |  |
| `HEAD` | 1 (0x01) |  |  |
| `NECK_OR_RELIC` | 2 (0x02) |  |  |
| `SHOULDERS` | 3 (0x03) |  |  |
| `BODY` | 4 (0x04) |  |  |
| `CHEST` | 5 (0x05) |  |  |
| `WAIST` | 6 (0x06) |  |  |
| `LEGS` | 7 (0x07) |  |  |
| `FEET` | 8 (0x08) |  |  |
| `WRISTS` | 9 (0x09) |  |  |
| `HANDS` | 10 (0x0A) |  |  |
| `FINGER` | 11 (0x0B) |  |  |
| `TRINKET` | 12 (0x0C) |  |  |
| `WEAPON` | 13 (0x0D) |  |  |
| `SHIELD` | 14 (0x0E) |  |  |
| `RANGED` | 15 (0x0F) |  |  |
| `CLOAK` | 16 (0x10) |  |  |
| `TWO_HANDED_WEAPON` | 17 (0x11) |  |  |
| `BAG` | 18 (0x12) |  |  |
| `TABARD` | 19 (0x13) |  |  |
| `ROBE` | 20 (0x14) |  |  |
| `WEAPON_MAIN_HAND` | 21 (0x15) |  |  |
| `WEAPON_OFF_HAND` | 22 (0x16) |  |  |
| `HOLDABLE` | 23 (0x17) |  |  |
| `AMMO` | 24 (0x18) |  |  |
| `THROWN` | 25 (0x19) |  |  |
| `RANGED_RIGHT` | 26 (0x1A) |  |  |
| `QUIVER` | 27 (0x1B) |  |  |