# SpellSchool

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/spell/spell_common.wowm:33`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/spell_common.wowm#L33).

```rust,ignore
enum SpellSchool : u8 {
    NORMAL = 0;
    HOLY = 1;
    FIRE = 2;
    NATURE = 3;
    FROST = 4;
    SHADOW = 5;
    ARCANE = 6;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NORMAL` | 0 (0x00) |  | Physical, Armor |
| `HOLY` | 1 (0x01) |  |  |
| `FIRE` | 2 (0x02) |  |  |
| `NATURE` | 3 (0x03) |  |  |
| `FROST` | 4 (0x04) |  |  |
| `SHADOW` | 5 (0x05) |  |  |
| `ARCANE` | 6 (0x06) |  |  |

Used in:
* [AuraLog](auralog.md)
* [SMSG_SPELLDAMAGESHIELD](smsg_spelldamageshield.md)
* [SMSG_SPELLNONMELEEDAMAGELOG](smsg_spellnonmeleedamagelog.md)

