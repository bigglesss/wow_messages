## Client Version 1.12

```rust,ignore
smsg SMSG_QUESTGIVER_QUEST_FAILED = 0x192 {
    u32 quest_id;    
    QuestFailedReason reason;    
}

```