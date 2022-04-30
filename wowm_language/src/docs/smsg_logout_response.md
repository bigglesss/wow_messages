## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_LOGOUT_RESPONSE = 0x004C {
    LogoutResult reason;
    LogoutSpeed speed;
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
| 0x04 | ? / - | [LogoutResult](logoutresult.md) | reason |  |
| - | ? / - | [LogoutSpeed](logoutspeed.md) | speed |  |
### Examples
```c
0, 7, // size
76, 0, // opcode (76)
0, 0, 0, 0, // reason: LogoutResult SUCCESS (0)
1, // speed: LogoutSpeed INSTANT (1)
```