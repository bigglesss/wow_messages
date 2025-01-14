# CMSG_GMTICKET_CREATE

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_create.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_create.wowm#L3).
```rust,ignore
cmsg CMSG_GMTICKET_CREATE = 0x0205 {
    GmTicketType category;
    Map map;
    Vector3d position;
    CString message;
    CString reserved_for_future_use;
    if (category == BEHAVIOR_HARASSMENT) {
        u32 chat_data_line_count;
        u32 chat_data_size_uncompressed;
        u8[-] compressed_chat_data;
    }
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
| 0x06 | 1 / - | [GmTicketType](gmtickettype.md) | category |  |  |
| 0x07 | 4 / - | [Map](map.md) | map |  |  |
| 0x0B | 12 / - | [Vector3d](vector3d.md) | position |  |  |
| 0x17 | - / - | CString | message |  |  |
| - | - / - | CString | reserved_for_future_use |  | cmangos/vmangos/mangoszero: Pre-TBC: 'Reserved for future use'<br/>cmangos/vmangos/mangoszero: Unused |

If category is equal to `BEHAVIOR_HARASSMENT`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / Little | u32 | chat_data_line_count |  |  |
| - | 4 / Little | u32 | chat_data_size_uncompressed |  |  |
| - | ? / - | u8[-] | compressed_chat_data |  |  |

