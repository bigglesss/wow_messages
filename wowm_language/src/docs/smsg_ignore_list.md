## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_IGNORE_LIST = 0x006B {
    u8 amount_of_ignored;
    u64[amount_of_ignored] ignored;
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
| 0x04 | 1 / - | u8 | amount_of_ignored |  |
| 0x05 | ? / - | u64[amount_of_ignored] | ignored |  |
### Examples
```c
0, 11, // size
107, 0, // opcode (107)
1, // amount_of_ignored: u8
239, 190, 173, 222, 254, 15, 220, 186, // ignored: u64[amount_of_ignored]
```
```c
0, 19, // size
107, 0, // opcode (107)
2, // amount_of_ignored: u8
239, 190, 173, 222, 254, 15, 220, 186, 239, 190, 173, 222, 0, 0, 0, 0, // ignored: u64[amount_of_ignored]
```