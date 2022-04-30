## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_CHAR_RENAME = 0x02C8 {
    WorldResult result;
    if (result == RESPONSE_SUCCESS) {
        Guid guid;
        CString name;
    }
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
| 0x04 | ? / - | [WorldResult](worldresult.md) | result |  |

If result is equal to `RESPONSE_SUCCESS`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |
| - | - / - | CString | name |  |