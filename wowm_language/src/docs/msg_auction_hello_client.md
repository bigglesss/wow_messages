## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg MSG_AUCTION_HELLO_Client = 0x0255 {
    Guid auctioneer;
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
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | auctioneer |  |
### Examples
```c
0, 12, // size
85, 2, 0, 0, // opcode (597)
239, 190, 173, 222, 0, 0, 0, 0, // auctioneer: Guid
```