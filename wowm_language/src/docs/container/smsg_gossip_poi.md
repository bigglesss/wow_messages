## Client Version 1.12

```rust,ignore
smsg SMSG_GOSSIP_POI = 0x224 {
    u32 flags;    
    f32 position_x;    
    f32 position_y;    
    u32 icon;    
    u32 data;    
    CString location_name;    
}

```