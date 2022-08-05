# MSG_MOVE_TELEPORT_ACK_Server

## Client Version 1.12

### Description

Can be response to [CMSG_TELEPORT_TO_UNIT](./cmsg_teleport_to_unit.md).

Can also be a response to [MSG_MOVE_TELEPORT_ACK_Client](./msg_move_teleport_ack_client.md) after being sent.

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm#L11).
```rust,ignore
smsg MSG_MOVE_TELEPORT_ACK_Server = 0x00C7 {
    PackedGuid guid;
    u32 movement_counter;
    MovementInfo info;
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

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | - / - | [PackedGuid](../spec/packed-guid.md) | guid |  |  |
| - | 4 / Little | u32 | movement_counter |  |  |
| - | ? / - | [MovementInfo](movementinfo.md) | info |  |  |
