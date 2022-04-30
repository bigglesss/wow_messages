## Client Version 1.12

### Wowm Representation
```rust,ignore
enum SpellCastResult : u8 {
    AFFECTING_COMBAT = 0x00;
    ALREADY_AT_FULL_HEALTH = 0x01;
    ALREADY_AT_FULL_MANA = 0x02;
    ALREADY_BEING_TAMED = 0x03;
    ALREADY_HAVE_CHARM = 0x04;
    ALREADY_HAVE_SUMMON = 0x05;
    ALREADY_OPEN = 0x06;
    MORE_POWERFUL_SPELL_ACTIVE = 0x07;
    BAD_IMPLICIT_TARGETS = 0x09;
    BAD_TARGETS = 0x0A;
    CANT_BE_CHARMED = 0x0B;
    CANT_BE_DISENCHANTED = 0x0C;
    CANT_BE_PROSPECTED = 0x0D;
    CANT_CAST_ON_TAPPED = 0x0E;
    CANT_DUEL_WHILE_INVISIBLE = 0x0F;
    CANT_DUEL_WHILE_STEALTHED = 0x10;
    CANT_TOO_CLOSE_TO_ENEMY = 0x11;
    CANT_DO_THAT_YET = 0x12;
    CASTER_DEAD = 0x13;
    CHARMED = 0x14;
    CHEST_IN_USE = 0x15;
    CONFUSED = 0x16;
    DONT_REPORT = 0x17;
    EQUIPPED_ITEM = 0x18;
    EQUIPPED_ITEM_CLASS = 0x19;
    EQUIPPED_ITEM_CLASS_MAINHAND = 0x1A;
    EQUIPPED_ITEM_CLASS_OFFHAND = 0x1B;
    ERROR = 0x1C;
    FIZZLE = 0x1D;
    FLEEING = 0x1E;
    FOOD_LOWLEVEL = 0x1F;
    HIGHLEVEL = 0x20;
    IMMUNE = 0x22;
    INTERRUPTED = 0x23;
    INTERRUPTED_COMBAT = 0x24;
    ITEM_ALREADY_ENCHANTED = 0x25;
    ITEM_GONE = 0x26;
    ENCHANT_NOT_EXISTING_ITEM = 0x27;
    ITEM_NOT_READY = 0x28;
    LEVEL_REQUIREMENT = 0x29;
    LINE_OF_SIGHT = 0x2A;
    LOWLEVEL = 0x2B;
    SKILL_NOT_HIGH_ENOUGH = 0x2C;
    MAINHAND_EMPTY = 0x2D;
    MOVING = 0x2E;
    NEED_AMMO = 0x2F;
    NEED_REQUIRES_SOMETHING = 0x30;
    NEED_EXOTIC_AMMO = 0x31;
    NOPATH = 0x32;
    NOT_BEHIND = 0x33;
    NOT_FISHABLE = 0x34;
    NOT_HERE = 0x35;
    NOT_INFRONT = 0x36;
    NOT_IN_CONTROL = 0x37;
    NOT_KNOWN = 0x38;
    NOT_MOUNTED = 0x39;
    NOT_ON_TAXI = 0x3A;
    NOT_ON_TRANSPORT = 0x3B;
    NOT_READY = 0x3C;
    NOT_SHAPESHIFT = 0x3D;
    NOT_STANDING = 0x3E;
    NOT_TRADEABLE = 0x3F;
    NOT_TRADING = 0x40;
    NOT_UNSHEATHED = 0x41;
    NOT_WHILE_GHOST = 0x42;
    NO_AMMO = 0x43;
    NO_CHARGES_REMAIN = 0x44;
    NO_CHAMPION = 0x45;
    NO_COMBO_POINTS = 0x46;
    NO_DUELING = 0x47;
    NO_ENDURANCE = 0x48;
    NO_FISH = 0x49;
    NO_ITEMS_WHILE_SHAPESHIFTED = 0x4A;
    NO_MOUNTS_ALLOWED = 0x4B;
    NO_PET = 0x4C;
    NO_POWER = 0x4D;
    NOTHING_TO_DISPEL = 0x4E;
    NOTHING_TO_STEAL = 0x4F;
    ONLY_ABOVEWATER = 0x50;
    ONLY_DAYTIME = 0x51;
    ONLY_INDOORS = 0x52;
    ONLY_MOUNTED = 0x53;
    ONLY_NIGHTTIME = 0x54;
    ONLY_OUTDOORS = 0x55;
    ONLY_SHAPESHIFT = 0x56;
    ONLY_STEALTHED = 0x57;
    ONLY_UNDERWATER = 0x58;
    OUT_OF_RANGE = 0x59;
    PACIFIED = 0x5A;
    POSSESSED = 0x5B;
    REQUIRES_AREA = 0x5D;
    REQUIRES_SPELL_FOCUS = 0x5E;
    ROOTED = 0x5F;
    SILENCED = 0x60;
    SPELL_IN_PROGRESS = 0x61;
    SPELL_LEARNED = 0x62;
    SPELL_UNAVAILABLE = 0x63;
    STUNNED = 0x64;
    TARGETS_DEAD = 0x65;
    TARGET_AFFECTING_COMBAT = 0x66;
    TARGET_AURASTATE = 0x67;
    TARGET_DUELING = 0x68;
    TARGET_ENEMY = 0x69;
    TARGET_ENRAGED = 0x6A;
    TARGET_FRIENDLY = 0x6B;
    TARGET_IN_COMBAT = 0x6C;
    TARGET_IS_PLAYER = 0x6D;
    TARGET_NOT_DEAD = 0x6E;
    TARGET_NOT_IN_PARTY = 0x6F;
    TARGET_NOT_LOOTED = 0x70;
    TARGET_NOT_PLAYER = 0x71;
    TARGET_NO_POCKETS = 0x72;
    TARGET_NO_WEAPONS = 0x73;
    TARGET_UNSKINNABLE = 0x74;
    THIRST_SATIATED = 0x75;
    TOO_CLOSE = 0x76;
    TOO_MANY_OF_ITEM = 0x77;
    TRAINING_POINTS = 0x79;
    TRY_AGAIN = 0x7A;
    UNIT_NOT_BEHIND = 0x7B;
    UNIT_NOT_INFRONT = 0x7C;
    WRONG_PET_FOOD = 0x7D;
    NOT_WHILE_FATIGUED = 0x7E;
    TARGET_NOT_IN_INSTANCE = 0x7F;
    NOT_WHILE_TRADING = 0x80;
    TARGET_NOT_IN_RAID = 0x81;
    DISENCHANT_WHILE_LOOTING = 0x82;
    PROSPECT_WHILE_LOOTING = 0x83;
    TARGET_FREEFORALL = 0x85;
    NO_EDIBLE_CORPSES = 0x86;
    ONLY_BATTLEGROUNDS = 0x87;
    TARGET_NOT_GHOST = 0x88;
    TOO_MANY_SKILLS = 0x89;
    CANT_USE_NEW_ITEM = 0x8A;
    WRONG_WEATHER = 0x8B;
    DAMAGE_IMMUNE = 0x8C;
    PREVENTED_BY_MECHANIC = 0x8D;
    PLAY_TIME = 0x8E;
    REPUTATION = 0x8F;
    MIN_SKILL = 0x90;
    UNKNOWN = 0x91;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `AFFECTING_COMBAT` | 0 (0x00) |  |  |
| `ALREADY_AT_FULL_HEALTH` | 1 (0x01) |  |  |
| `ALREADY_AT_FULL_MANA` | 2 (0x02) |  |  |
| `ALREADY_BEING_TAMED` | 3 (0x03) |  |  |
| `ALREADY_HAVE_CHARM` | 4 (0x04) |  |  |
| `ALREADY_HAVE_SUMMON` | 5 (0x05) |  |  |
| `ALREADY_OPEN` | 6 (0x06) |  |  |
| `MORE_POWERFUL_SPELL_ACTIVE` | 7 (0x07) |  |  |
| `BAD_IMPLICIT_TARGETS` | 9 (0x09) |  |  |
| `BAD_TARGETS` | 10 (0x0A) |  |  |
| `CANT_BE_CHARMED` | 11 (0x0B) |  |  |
| `CANT_BE_DISENCHANTED` | 12 (0x0C) |  |  |
| `CANT_BE_PROSPECTED` | 13 (0x0D) |  |  |
| `CANT_CAST_ON_TAPPED` | 14 (0x0E) |  |  |
| `CANT_DUEL_WHILE_INVISIBLE` | 15 (0x0F) |  |  |
| `CANT_DUEL_WHILE_STEALTHED` | 16 (0x10) |  |  |
| `CANT_TOO_CLOSE_TO_ENEMY` | 17 (0x11) |  |  |
| `CANT_DO_THAT_YET` | 18 (0x12) |  |  |
| `CASTER_DEAD` | 19 (0x13) |  |  |
| `CHARMED` | 20 (0x14) |  |  |
| `CHEST_IN_USE` | 21 (0x15) |  |  |
| `CONFUSED` | 22 (0x16) |  |  |
| `DONT_REPORT` | 23 (0x17) |  |  |
| `EQUIPPED_ITEM` | 24 (0x18) |  |  |
| `EQUIPPED_ITEM_CLASS` | 25 (0x19) |  |  |
| `EQUIPPED_ITEM_CLASS_MAINHAND` | 26 (0x1A) |  |  |
| `EQUIPPED_ITEM_CLASS_OFFHAND` | 27 (0x1B) |  |  |
| `ERROR` | 28 (0x1C) |  |  |
| `FIZZLE` | 29 (0x1D) |  |  |
| `FLEEING` | 30 (0x1E) |  |  |
| `FOOD_LOWLEVEL` | 31 (0x1F) |  |  |
| `HIGHLEVEL` | 32 (0x20) |  |  |
| `IMMUNE` | 34 (0x22) |  |  |
| `INTERRUPTED` | 35 (0x23) |  |  |
| `INTERRUPTED_COMBAT` | 36 (0x24) |  |  |
| `ITEM_ALREADY_ENCHANTED` | 37 (0x25) |  |  |
| `ITEM_GONE` | 38 (0x26) |  |  |
| `ENCHANT_NOT_EXISTING_ITEM` | 39 (0x27) |  |  |
| `ITEM_NOT_READY` | 40 (0x28) |  |  |
| `LEVEL_REQUIREMENT` | 41 (0x29) |  |  |
| `LINE_OF_SIGHT` | 42 (0x2A) |  |  |
| `LOWLEVEL` | 43 (0x2B) |  |  |
| `SKILL_NOT_HIGH_ENOUGH` | 44 (0x2C) |  |  |
| `MAINHAND_EMPTY` | 45 (0x2D) |  |  |
| `MOVING` | 46 (0x2E) |  |  |
| `NEED_AMMO` | 47 (0x2F) |  |  |
| `NEED_REQUIRES_SOMETHING` | 48 (0x30) |  |  |
| `NEED_EXOTIC_AMMO` | 49 (0x31) |  |  |
| `NOPATH` | 50 (0x32) |  |  |
| `NOT_BEHIND` | 51 (0x33) |  |  |
| `NOT_FISHABLE` | 52 (0x34) |  |  |
| `NOT_HERE` | 53 (0x35) |  |  |
| `NOT_INFRONT` | 54 (0x36) |  |  |
| `NOT_IN_CONTROL` | 55 (0x37) |  |  |
| `NOT_KNOWN` | 56 (0x38) |  |  |
| `NOT_MOUNTED` | 57 (0x39) |  |  |
| `NOT_ON_TAXI` | 58 (0x3A) |  |  |
| `NOT_ON_TRANSPORT` | 59 (0x3B) |  |  |
| `NOT_READY` | 60 (0x3C) |  |  |
| `NOT_SHAPESHIFT` | 61 (0x3D) |  |  |
| `NOT_STANDING` | 62 (0x3E) |  |  |
| `NOT_TRADEABLE` | 63 (0x3F) |  | rogues trying 'enchant' other's weapon with poison |
| `NOT_TRADING` | 64 (0x40) |  |  |
| `NOT_UNSHEATHED` | 65 (0x41) |  |  |
| `NOT_WHILE_GHOST` | 66 (0x42) |  |  |
| `NO_AMMO` | 67 (0x43) |  |  |
| `NO_CHARGES_REMAIN` | 68 (0x44) |  |  |
| `NO_CHAMPION` | 69 (0x45) |  |  |
| `NO_COMBO_POINTS` | 70 (0x46) |  |  |
| `NO_DUELING` | 71 (0x47) |  |  |
| `NO_ENDURANCE` | 72 (0x48) |  |  |
| `NO_FISH` | 73 (0x49) |  |  |
| `NO_ITEMS_WHILE_SHAPESHIFTED` | 74 (0x4A) |  |  |
| `NO_MOUNTS_ALLOWED` | 75 (0x4B) |  |  |
| `NO_PET` | 76 (0x4C) |  |  |
| `NO_POWER` | 77 (0x4D) |  |  |
| `NOTHING_TO_DISPEL` | 78 (0x4E) |  |  |
| `NOTHING_TO_STEAL` | 79 (0x4F) |  |  |
| `ONLY_ABOVEWATER` | 80 (0x50) |  |  |
| `ONLY_DAYTIME` | 81 (0x51) |  |  |
| `ONLY_INDOORS` | 82 (0x52) |  |  |
| `ONLY_MOUNTED` | 83 (0x53) |  |  |
| `ONLY_NIGHTTIME` | 84 (0x54) |  |  |
| `ONLY_OUTDOORS` | 85 (0x55) |  |  |
| `ONLY_SHAPESHIFT` | 86 (0x56) |  |  |
| `ONLY_STEALTHED` | 87 (0x57) |  |  |
| `ONLY_UNDERWATER` | 88 (0x58) |  |  |
| `OUT_OF_RANGE` | 89 (0x59) |  |  |
| `PACIFIED` | 90 (0x5A) |  |  |
| `POSSESSED` | 91 (0x5B) |  |  |
| `REQUIRES_AREA` | 93 (0x5D) |  |  |
| `REQUIRES_SPELL_FOCUS` | 94 (0x5E) |  |  |
| `ROOTED` | 95 (0x5F) |  |  |
| `SILENCED` | 96 (0x60) |  |  |
| `SPELL_IN_PROGRESS` | 97 (0x61) |  |  |
| `SPELL_LEARNED` | 98 (0x62) |  |  |
| `SPELL_UNAVAILABLE` | 99 (0x63) |  |  |
| `STUNNED` | 100 (0x64) |  |  |
| `TARGETS_DEAD` | 101 (0x65) |  |  |
| `TARGET_AFFECTING_COMBAT` | 102 (0x66) |  |  |
| `TARGET_AURASTATE` | 103 (0x67) |  |  |
| `TARGET_DUELING` | 104 (0x68) |  |  |
| `TARGET_ENEMY` | 105 (0x69) |  |  |
| `TARGET_ENRAGED` | 106 (0x6A) |  |  |
| `TARGET_FRIENDLY` | 107 (0x6B) |  |  |
| `TARGET_IN_COMBAT` | 108 (0x6C) |  |  |
| `TARGET_IS_PLAYER` | 109 (0x6D) |  |  |
| `TARGET_NOT_DEAD` | 110 (0x6E) |  |  |
| `TARGET_NOT_IN_PARTY` | 111 (0x6F) |  |  |
| `TARGET_NOT_LOOTED` | 112 (0x70) |  |  |
| `TARGET_NOT_PLAYER` | 113 (0x71) |  |  |
| `TARGET_NO_POCKETS` | 114 (0x72) |  |  |
| `TARGET_NO_WEAPONS` | 115 (0x73) |  |  |
| `TARGET_UNSKINNABLE` | 116 (0x74) |  |  |
| `THIRST_SATIATED` | 117 (0x75) |  |  |
| `TOO_CLOSE` | 118 (0x76) |  |  |
| `TOO_MANY_OF_ITEM` | 119 (0x77) |  |  |
| `TRAINING_POINTS` | 121 (0x79) |  |  |
| `TRY_AGAIN` | 122 (0x7A) |  |  |
| `UNIT_NOT_BEHIND` | 123 (0x7B) |  |  |
| `UNIT_NOT_INFRONT` | 124 (0x7C) |  |  |
| `WRONG_PET_FOOD` | 125 (0x7D) |  |  |
| `NOT_WHILE_FATIGUED` | 126 (0x7E) |  |  |
| `TARGET_NOT_IN_INSTANCE` | 127 (0x7F) |  |  |
| `NOT_WHILE_TRADING` | 128 (0x80) |  |  |
| `TARGET_NOT_IN_RAID` | 129 (0x81) |  |  |
| `DISENCHANT_WHILE_LOOTING` | 130 (0x82) |  |  |
| `PROSPECT_WHILE_LOOTING` | 131 (0x83) |  |  |
| `TARGET_FREEFORALL` | 133 (0x85) |  |  |
| `NO_EDIBLE_CORPSES` | 134 (0x86) |  |  |
| `ONLY_BATTLEGROUNDS` | 135 (0x87) |  |  |
| `TARGET_NOT_GHOST` | 136 (0x88) |  |  |
| `TOO_MANY_SKILLS` | 137 (0x89) |  |  |
| `CANT_USE_NEW_ITEM` | 138 (0x8A) |  |  |
| `WRONG_WEATHER` | 139 (0x8B) |  |  |
| `DAMAGE_IMMUNE` | 140 (0x8C) |  |  |
| `PREVENTED_BY_MECHANIC` | 141 (0x8D) |  |  |
| `PLAY_TIME` | 142 (0x8E) |  |  |
| `REPUTATION` | 143 (0x8F) |  |  |
| `MIN_SKILL` | 144 (0x90) |  |  |
| `UNKNOWN` | 145 (0x91) |  |  |