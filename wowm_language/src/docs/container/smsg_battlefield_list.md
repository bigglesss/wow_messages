## Client Version 1.12

```rust,ignore
smsg SMSG_BATTLEFIELD_LIST = 0x23D {
    Guid battlemaster;    
    Map map;    
    u8 unknown1;    
    u32 unknown2;    
    u8 unknown3;    
    u32 number_of_battlegrounds;    
    u32[number_of_battlegrounds] battlegrounds;    
}

```