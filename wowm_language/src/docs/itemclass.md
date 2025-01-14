# ItemClass

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/common.wowm:98`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L98).

```rust,ignore
enum ItemClass : u8 {
    CONSUMABLE = 0;
    CONTAINER = 1;
    WEAPON = 2;
    RESERVED_1 = 3;
    ARMOR = 4;
    REAGENT = 5;
    PROJECTILE = 6;
    TRADE_GOODS = 7;
    RESERVED_2 = 8;
    RECIPE = 9;
    RESERVED_3 = 10;
    QUIVER = 11;
    QUEST = 12;
    KEY = 13;
    RESERVED_4 = 14;
    MISC = 15;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `CONSUMABLE` | 0 (0x00) |  |  |
| `CONTAINER` | 1 (0x01) |  |  |
| `WEAPON` | 2 (0x02) |  |  |
| `RESERVED_1` | 3 (0x03) |  |  |
| `ARMOR` | 4 (0x04) |  |  |
| `REAGENT` | 5 (0x05) |  |  |
| `PROJECTILE` | 6 (0x06) |  |  |
| `TRADE_GOODS` | 7 (0x07) |  |  |
| `RESERVED_2` | 8 (0x08) |  |  |
| `RECIPE` | 9 (0x09) |  |  |
| `RESERVED_3` | 10 (0x0A) |  |  |
| `QUIVER` | 11 (0x0B) |  |  |
| `QUEST` | 12 (0x0C) |  |  |
| `KEY` | 13 (0x0D) |  |  |
| `RESERVED_4` | 14 (0x0E) |  |  |
| `MISC` | 15 (0x0F) |  |  |

Used in:
* [SMSG_ITEM_QUERY_SINGLE_RESPONSE](smsg_item_query_single_response.md)
* [SMSG_SET_PROFICIENCY](smsg_set_proficiency.md)

