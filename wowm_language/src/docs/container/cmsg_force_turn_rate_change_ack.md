## Client Version 1.12

```rust,ignore
cmsg CMSG_FORCE_TURN_RATE_CHANGE_ACK = 0x2DF {
    Guid guid;    
    u32 counter;    
    MovementInfo movement_info;    
    f32 new_speed;    
}

```