# SMSG_SPELL_GO

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/spell/smsg_spell_go.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_go.wowm#L3).
```rust,ignore
smsg SMSG_SPELL_GO = 0x0132 {
    PackedGuid cast_item;
    PackedGuid caster;
    u32 spell;
    CastFlags flags;
    u8 amount_of_hits;
    Guid[amount_of_hits] hits;
    u8 amount_of_misses;
    SpellMiss[amount_of_misses] misses;
    SpellCastTargets targets;
    if (flags & AMMO) {
        u32 ammo_display_id;
        u32 ammo_inventory_type;
    }
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | - / - | [PackedGuid](../spec/packed-guid.md) | cast_item |  | cmangos/vmangos/mangoszero: if cast item is used, set this to guid of cast item, otherwise set it to same as caster. |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | caster |  |  |
| - | 4 / Little | u32 | spell |  |  |
| - | 2 / - | [CastFlags](castflags.md) | flags |  |  |
| - | 1 / - | u8 | amount_of_hits |  |  |
| - | ? / - | [Guid](../spec/packed-guid.md)[amount_of_hits] | hits |  |  |
| - | 1 / - | u8 | amount_of_misses |  |  |
| - | ? / - | [SpellMiss](spellmiss.md)[amount_of_misses] | misses |  |  |
| - | - / - | [SpellCastTargets](spellcasttargets.md) | targets |  |  |

If flags contains `AMMO`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / Little | u32 | ammo_display_id |  |  |
| - | 4 / Little | u32 | ammo_inventory_type |  |  |

