## Client Version 1.12

```rust,ignore
smsg SMSG_FRIEND_LIST = 0x67 {
    u8 amount_of_friends;    
    Friend[amount_of_friends] friends;    
}

```