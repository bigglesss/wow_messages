## Client Version 1.12

```rust,ignore
cmsg CMSG_BATTLEMASTER_JOIN = 0x2EE {
    Guid guid;    
    Map map;    
    u32 instance_id;    
    u8 join_as_group;    
}

```