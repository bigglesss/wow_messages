# GossipItem

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/gossip/smsg_gossip_message.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/smsg_gossip_message.wowm#L3).
```rust,ignore
struct GossipItem {
    u32 id;
    u8 item_icon;
    Bool coded;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 4 / Little | u32 | id |  | vmangos: sets to loop index |
| 0x04 | 1 / - | u8 | item_icon |  |  |
| 0x05 | 1 / - | Bool | coded |  | vmangos: makes pop up box password |

