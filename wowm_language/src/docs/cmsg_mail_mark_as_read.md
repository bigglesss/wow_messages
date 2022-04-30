## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_MAIL_MARK_AS_READ = 0x0247 {
    Guid mailbox_guid;
    u32 mail_id;
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
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | mailbox_guid |  |
| 0x0E | 4 / Little | u32 | mail_id |  |