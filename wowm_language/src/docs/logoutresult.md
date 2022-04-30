## Client Version 1.12

### Wowm Representation
```rust,ignore
enum LogoutResult : u32 {
    SUCCESS = 0;
    FAILURE_IN_COMBAT = 1;
    FAILURE_FROZEN_BY_GM = 2;
    FAILURE_JUMPING_OR_FALLING = 3;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `SUCCESS` | 0 (0x00) |  |  |
| `FAILURE_IN_COMBAT` | 1 (0x01) |  |  |
| `FAILURE_FROZEN_BY_GM` | 2 (0x02) |  | vmangos checks for aura 9454. Has FIXME - Need the correct value. |
| `FAILURE_JUMPING_OR_FALLING` | 3 (0x03) |  |  |