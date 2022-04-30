## Client Version 1.12

### Wowm Representation
```rust,ignore
flag FactionFlag : u8 {
    VISIBLE = 0x01;
    AT_WAR = 0x02;
    HIDDEN = 0x04;
    INVISIBLE_FORCED = 0x08;
    PEACE_FORCED = 0x10;
    INACTIVE = 0x20;
    RIVAL = 0x40;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `VISIBLE` | 1 (0x01) |  | makes visible in client (set or can be set at interaction with target of this faction) |
| `AT_WAR` | 2 (0x02) |  | enable AtWar-button in client. player controlled (except opposition team always war state), Flag only set on initial creation |
| `HIDDEN` | 4 (0x04) |  | hidden faction from reputation pane in client (player can gain reputation, but this update not sent to client) |
| `INVISIBLE_FORCED` | 8 (0x08) |  | always overwrite FACTION_FLAG_VISIBLE and hide faction in rep.list, used for hide opposite team factions |
| `PEACE_FORCED` | 16 (0x10) |  | always overwrite FACTION_FLAG_AT_WAR, used for prevent war with own team factions |
| `INACTIVE` | 32 (0x20) |  | player controlled, state stored in characters.data ( CMSG_SET_FACTION_INACTIVE ) |
| `RIVAL` | 64 (0x40) |  | flag for the two competing outland factions |