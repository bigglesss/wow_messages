## Client Version 1.12

```rust,ignore
smsg SMSG_LOGIN_SETTIMESPEED = 0x42 {
    u32 secs_to_time_bit_field;    
    f32 game_speed;    
}

```