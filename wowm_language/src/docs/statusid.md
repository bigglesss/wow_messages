## Client Version 1.12

### Wowm Representation
```rust,ignore
enum StatusId : u8 {
    NONE = 0;
    WAIT_QUEUE = 1;
    WAIT_JOIN = 2;
    IN_PROGRESS = 3;
    WAIT_LEAVE = 4;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  | first status, should mean bg is not instance |
| `WAIT_QUEUE` | 1 (0x01) |  | means bg is empty and waiting for queue |
| `WAIT_JOIN` | 2 (0x02) |  | this means, that BG has already started and it is waiting for more players |
| `IN_PROGRESS` | 3 (0x03) |  | means bg is running |
| `WAIT_LEAVE` | 4 (0x04) |  | means some faction has won BG and it is ending |