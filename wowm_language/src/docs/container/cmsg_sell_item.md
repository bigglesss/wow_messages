## Client Version 1.12

```rust,ignore
cmsg CMSG_SELL_ITEM = 0x1A0 {
    Guid vendor_guid;    
    Guid item_guid;    
    u8 amount;    
}

```