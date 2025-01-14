# MSG_MOVE_TELEPORT_ACK_Client

## Client Version 1.12

### Description

Response to [MSG_MOVE_TELEPORT_ACK_Server](./msg_move_teleport_ack_server.md), at which point [MSG_MOVE_TELEPORT_ACK_Server](./msg_move_teleport_ack_server.md) should be sent to observing players.

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm#L3).
```rust,ignore
cmsg MSG_MOVE_TELEPORT_ACK_Client = 0x00C7 {
    Guid guid;
    u32 movement_counter;
    u32 time_in_msecs;
}
```
### Header

CMSG have a header of 6 bytes.

#### CMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |  |
| 0x0E | 4 / Little | u32 | movement_counter |  |  |
| 0x12 | 4 / Little | u32 | time_in_msecs |  |  |

