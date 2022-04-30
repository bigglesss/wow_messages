## Client Version 1.12

### Wowm Representation
```rust,ignore
enum Map : u32 {
    EASTERN_KINGDOMS = 0;
    KALIMDOR = 1;
    TESTING = 13;
    SCOTT_TEST = 25;
    CASH_TEST = 29;
    ALTERAC_VALLEY = 30;
    SHADOWFANG_KEEP = 33;
    STORMWIND_STOCKADE = 34;
    STORMWIND_PRISON = 35;
    DEADMINES = 36;
    AZSHARA_CRATER = 37;
    COLLINS_TEST = 42;
    WAILING_CAVERNS = 43;
    MONASTERY = 44;
    RAZORFEN_KRAUL = 47;
    BLACKFATHOM_DEEPS = 48;
    ULDAMAN = 70;
    GNOMERAGON = 90;
    SUNKEN_TEMPLE = 109;
    RAZORFEN_DOWNS = 129;
    EMERALD_DREAM = 169;
    SCARLET_MONASTERY = 189;
    ZUL_FARRAK = 209;
    BLACKROCK_SPIRE = 229;
    BLACKROCK_DEPTHS = 230;
    ONYXIAS_LAIR = 249;
    CAVERNS_OF_TIME = 269;
    SCHOLOMANCE = 289;
    ZUL_GURUB = 309;
    STRATHOLME = 329;
    MAURADON = 349;
    DEEPRUN_TRAM = 369;
    RAGEFIRE_CHASM = 389;
    MOLTEN_CORE = 409;
    DIRE_MAUL = 429;
    ALLIANCE_PVP_BARRACKS = 449;
    HORDE_PVP_BARRACKS = 450;
    DEVELOPMENT_LAND = 451;
    BLACKWING_LAIR = 469;
    WARSONG_GULCH = 489;
    RUINS_OF_AHN_QIRAJ = 509;
    ARATHI_BASIN = 529;
    AHN_QIRAJ_TEMPLE = 531;
    NAXXRAMAS = 533;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment | Display |
| --------- | -------- | ----------- | ------- | ------- |
| `EASTERN_KINGDOMS` | 0 (0x00) |  |  | Eastern Kingdoms |
| `KALIMDOR` | 1 (0x01) |  |  | Kalimdor |
| `TESTING` | 13 (0x0D) |  |  | Testing |
| `SCOTT_TEST` | 25 (0x19) |  |  | Scott Test |
| `CASH_TEST` | 29 (0x1D) |  |  | CashTest |
| `ALTERAC_VALLEY` | 30 (0x1E) |  |  | Alterac Valley |
| `SHADOWFANG_KEEP` | 33 (0x21) |  |  | Shadowfang Keep |
| `STORMWIND_STOCKADE` | 34 (0x22) |  |  | Stormwind Stockade |
| `STORMWIND_PRISON` | 35 (0x23) |  |  | Stormwind Prison |
| `DEADMINES` | 36 (0x24) |  |  | Deadmines |
| `AZSHARA_CRATER` | 37 (0x25) |  |  | Azshara Crater |
| `COLLINS_TEST` | 42 (0x2A) |  |  | Collin's Test |
| `WAILING_CAVERNS` | 43 (0x2B) |  |  | Wailing Caverns |
| `MONASTERY` | 44 (0x2C) |  |  | Monastery |
| `RAZORFEN_KRAUL` | 47 (0x2F) |  |  | Razorfen Kraul |
| `BLACKFATHOM_DEEPS` | 48 (0x30) |  |  | Blackfathom Deeps |
| `ULDAMAN` | 70 (0x46) |  |  | Uldaman |
| `GNOMERAGON` | 90 (0x5A) |  |  | Gnomeragon |
| `SUNKEN_TEMPLE` | 109 (0x6D) |  |  | SunkenTemple |
| `RAZORFEN_DOWNS` | 129 (0x81) |  |  | Razorfen Downs |
| `EMERALD_DREAM` | 169 (0xA9) |  |  | Emerald Dream |
| `SCARLET_MONASTERY` | 189 (0xBD) |  |  | Scarlet Monastery |
| `ZUL_FARRAK` | 209 (0xD1) |  |  | Zul'Farrak |
| `BLACKROCK_SPIRE` | 229 (0xE5) |  |  | Blackrock Spire |
| `BLACKROCK_DEPTHS` | 230 (0xE6) |  |  | Blackrock Depths |
| `ONYXIAS_LAIR` | 249 (0xF9) |  |  | Onyxia's Lair |
| `CAVERNS_OF_TIME` | 269 (0x10D) |  |  | Caverns of Time |
| `SCHOLOMANCE` | 289 (0x121) |  |  | Scholomance |
| `ZUL_GURUB` | 309 (0x135) |  |  | Zul'Gurub |
| `STRATHOLME` | 329 (0x149) |  |  | Stratholme |
| `MAURADON` | 349 (0x15D) |  |  | Mauradon |
| `DEEPRUN_TRAM` | 369 (0x171) |  |  | Deeprun Tram |
| `RAGEFIRE_CHASM` | 389 (0x185) |  |  | Ragefire Chasm |
| `MOLTEN_CORE` | 409 (0x199) |  |  | Molten Core |
| `DIRE_MAUL` | 429 (0x1AD) |  |  | Dire Maul |
| `ALLIANCE_PVP_BARRACKS` | 449 (0x1C1) |  |  | Alliance PVP Barracks |
| `HORDE_PVP_BARRACKS` | 450 (0x1C2) |  |  | Horde PVP Barracks |
| `DEVELOPMENT_LAND` | 451 (0x1C3) |  |  | Development Land |
| `BLACKWING_LAIR` | 469 (0x1D5) |  |  | Blackwing Lair |
| `WARSONG_GULCH` | 489 (0x1E9) |  |  | Warsong Gulch |
| `RUINS_OF_AHN_QIRAJ` | 509 (0x1FD) |  |  | Ruins of Ahn'Qiraj |
| `ARATHI_BASIN` | 529 (0x211) |  |  | Arathi Basin |
| `AHN_QIRAJ_TEMPLE` | 531 (0x213) |  |  | Ahn'Qiraj Temple |
| `NAXXRAMAS` | 533 (0x215) |  |  | Naxxramas |