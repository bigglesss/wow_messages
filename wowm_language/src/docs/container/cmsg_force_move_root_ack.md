## Client Version 1.12

```rust,ignore
cmsg CMSG_FORCE_MOVE_ROOT_ACK = 0xE9 {
    Guid guid;    
    u32 movement_counter;    
    MovementInfo movement_info;    
}

```