# ServerMessageType

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/chat/smsg_server_message.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_server_message.wowm#L3).

```rust,ignore
enum ServerMessageType : u32 {
    SHUTDOWN_TIME = 1;
    RESTART_TIME = 2;
    CUSTOM = 3;
    SHUTDOWN_CANCELLED = 4;
    RESTART_CANCELLED = 5;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `SHUTDOWN_TIME` | 1 (0x01) |  |  |
| `RESTART_TIME` | 2 (0x02) |  |  |
| `CUSTOM` | 3 (0x03) |  |  |
| `SHUTDOWN_CANCELLED` | 4 (0x04) |  |  |
| `RESTART_CANCELLED` | 5 (0x05) |  |  |

Used in:
* [SMSG_SERVER_MESSAGE](smsg_server_message.md)

