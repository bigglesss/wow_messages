## Client Version 1.12

```rust,ignore
smsg SMSG_GUILD_EVENT = 0x92 {
    GuildEvent event;    
    u8 amount_of_events;    
    CString[amount_of_events] event_descriptions;    
}

```