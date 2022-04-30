## Client Version 1.12

### Wowm Representation
```rust,ignore
flag GroupMemberOnlineStatus : u8 {
    OFFLINE = 0x0000;
    ONLINE = 0x0001;
    PVP = 0x0002;
    DEAD = 0x0004;
    GHOST = 0x0008;
    PVP_FFA = 0x0010;
    ZONE_OUT = 0x0020;
    AFK = 0x0040;
    DND = 0x0080;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `OFFLINE` | 0 (0x00) |  |  |
| `ONLINE` | 1 (0x01) |  | Lua_UnitIsConnected |
| `PVP` | 2 (0x02) |  | Lua_UnitIsPVP |
| `DEAD` | 4 (0x04) |  | Lua_UnitIsDead |
| `GHOST` | 8 (0x08) |  | Lua_UnitIsGhost |
| `PVP_FFA` | 16 (0x10) |  | Lua_UnitIsPVPFreeForAll |
| `ZONE_OUT` | 32 (0x20) |  | used in calls from Lua_GetPlayerMapPosition/Lua_GetBattlefieldFlagPosition |
| `AFK` | 64 (0x40) |  | Lua_UnitIsAFK |
| `DND` | 128 (0x80) |  | Lua_UnitIsDND |