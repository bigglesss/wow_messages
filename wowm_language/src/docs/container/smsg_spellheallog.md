## Client Version 1.12

```rust,ignore
smsg SMSG_SPELLHEALLOG = 0x150 {
    PackedGuid victim_guid;    
    PackedGuid caster_guid;    
    u32 id;    
    u32 damage;    
    u8 critical;    
}

```