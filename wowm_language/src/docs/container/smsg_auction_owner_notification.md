## Client Version 1.12

```rust,ignore
smsg SMSG_AUCTION_OWNER_NOTIFICATION = 0x25F {
    u32 auction_id;    
    u32 bid;    
    u32 auction_out_bid;    
    Guid bidder;    
    u32 item_entry;    
    u32 item_random_property_id;    
}

```