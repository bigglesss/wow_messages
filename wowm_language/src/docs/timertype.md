## Client Version 1.12

### Wowm Representation
```rust,ignore
enum TimerType : u32 {
    FATIGUE = 0;
    BREATH = 1;
    FEIGNDEATH = 2;
    ENVIRONMENTAL = 3;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `FATIGUE` | 0 (0x00) |  |  |
| `BREATH` | 1 (0x01) |  |  |
| `FEIGNDEATH` | 2 (0x02) |  |  |
| `ENVIRONMENTAL` | 3 (0x03) |  | Might be a mangos only thing. |