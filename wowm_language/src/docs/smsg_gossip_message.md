## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_GOSSIP_MESSAGE = 0x017D {
    Guid guid;
    u32 title_text_id;
    u32 amount_of_gossip_items;
    GossipItem[amount_of_gossip_items] gossips;
    u32 amount_of_quests;
    QuestItem[amount_of_quests] quests;
}
```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |
| 0x0C | 4 / Little | u32 | title_text_id |  |
| 0x10 | 4 / Little | u32 | amount_of_gossip_items |  |
| 0x14 | ? / - | [GossipItem](gossipitem.md)[amount_of_gossip_items] | gossips |  |
| - | 4 / Little | u32 | amount_of_quests |  |
| - | ? / - | [QuestItem](questitem.md)[amount_of_quests] | quests |  |