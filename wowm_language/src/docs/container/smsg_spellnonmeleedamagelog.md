## Client Version 1.12

```rust,ignore
smsg SMSG_SPELLNONMELEEDAMAGELOG = 0x250 {
    PackedGuid target;    
    PackedGuid attacker;    
    u32 spell;    
    u32 damage;    
    SpellSchool school;    
    u32 absorbed_damage;    
    u32 resisted;    
    u8 periodic_log;    
    u8 unused;    
    u32 blocked;    
    u32 hit_info;    
    u8 extend_flag;    
}

```