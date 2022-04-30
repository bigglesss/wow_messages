## Client Version 1.12

### Wowm Representation
```rust,ignore
enum GmTicketStatusResponse : u32 {
    UPDATED = 1;
    CLOSED = 2;
    SURVEY = 3;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `UPDATED` | 1 (0x01) |  |  |
| `CLOSED` | 2 (0x02) |  |  |
| `SURVEY` | 3 (0x03) |  |  |