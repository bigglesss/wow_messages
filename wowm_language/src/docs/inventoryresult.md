## Client Version 1.12

### Comment

Any values greater than maximum enum value show as 'bag full'

### Wowm Representation
```rust,ignore
enum InventoryResult : u8 {
    OK = 0;
    CANT_EQUIP_LEVEL_I = 1;
    CANT_EQUIP_SKILL = 2;
    ITEM_DOESNT_GO_TO_SLOT = 3;
    BAG_FULL = 4;
    NONEMPTY_BAG_OVER_OTHER_BAG = 5;
    CANT_TRADE_EQUIP_BAGS = 6;
    ONLY_AMMO_CAN_GO_HERE = 7;
    NO_REQUIRED_PROFICIENCY = 8;
    NO_EQUIPMENT_SLOT_AVAILABLE = 9;
    YOU_CAN_NEVER_USE_THAT_ITEM = 10;
    YOU_CAN_NEVER_USE_THAT_ITEM2 = 11;
    NO_EQUIPMENT_SLOT_AVAILABLE2 = 12;
    CANT_EQUIP_WITH_TWOHANDED = 13;
    CANT_DUAL_WIELD = 14;
    ITEM_DOESNT_GO_INTO_BAG = 15;
    ITEM_DOESNT_GO_INTO_BAG2 = 16;
    CANT_CARRY_MORE_OF_THIS = 17;
    NO_EQUIPMENT_SLOT_AVAILABLE3 = 18;
    ITEM_CANT_STACK = 19;
    ITEM_CANT_BE_EQUIPPED = 20;
    ITEMS_CANT_BE_SWAPPED = 21;
    SLOT_IS_EMPTY = 22;
    ITEM_NOT_FOUND = 23;
    CANT_DROP_SOULBOUND = 24;
    OUT_OF_RANGE = 25;
    TRIED_TO_SPLIT_MORE_THAN_COUNT = 26;
    COULDNT_SPLIT_ITEMS = 27;
    MISSING_REAGENT = 28;
    NOT_ENOUGH_MONEY = 29;
    NOT_A_BAG = 30;
    CAN_ONLY_DO_WITH_EMPTY_BAGS = 31;
    DONT_OWN_THAT_ITEM = 32;
    CAN_EQUIP_ONLY1_QUIVER = 33;
    MUST_PURCHASE_THAT_BAG_SLOT = 34;
    TOO_FAR_AWAY_FROM_BANK = 35;
    ITEM_LOCKED = 36;
    YOU_ARE_STUNNED = 37;
    YOU_ARE_DEAD = 38;
    CANT_DO_RIGHT_NOW = 39;
    INT_BAG_ERROR = 40;
    CAN_EQUIP_ONLY1_BOLT = 41;
    CAN_EQUIP_ONLY1_AMMOPOUCH = 42;
    STACKABLE_CANT_BE_WRAPPED = 43;
    EQUIPPED_CANT_BE_WRAPPED = 44;
    WRAPPED_CANT_BE_WRAPPED = 45;
    BOUND_CANT_BE_WRAPPED = 46;
    UNIQUE_CANT_BE_WRAPPED = 47;
    BAGS_CANT_BE_WRAPPED = 48;
    ALREADY_LOOTED = 49;
    INVENTORY_FULL = 50;
    BANK_FULL = 51;
    ITEM_IS_CURRENTLY_SOLD_OUT = 52;
    BAG_FULL3 = 53;
    ITEM_NOT_FOUND2 = 54;
    ITEM_CANT_STACK2 = 55;
    BAG_FULL4 = 56;
    ITEM_SOLD_OUT = 57;
    OBJECT_IS_BUSY = 58;
    NONE = 59;
    NOT_IN_COMBAT = 60;
    NOT_WHILE_DISARMED = 61;
    BAG_FULL6 = 62;
    CANT_EQUIP_RANK = 63;
    CANT_EQUIP_REPUTATION = 64;
    TOO_MANY_SPECIAL_BAGS = 65;
    LOOT_CANT_LOOT_THAT_NOW = 66;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `OK` | 0 (0x00) |  |  |
| `CANT_EQUIP_LEVEL_I` | 1 (0x01) |  |  |
| `CANT_EQUIP_SKILL` | 2 (0x02) |  |  |
| `ITEM_DOESNT_GO_TO_SLOT` | 3 (0x03) |  |  |
| `BAG_FULL` | 4 (0x04) |  |  |
| `NONEMPTY_BAG_OVER_OTHER_BAG` | 5 (0x05) |  |  |
| `CANT_TRADE_EQUIP_BAGS` | 6 (0x06) |  |  |
| `ONLY_AMMO_CAN_GO_HERE` | 7 (0x07) |  |  |
| `NO_REQUIRED_PROFICIENCY` | 8 (0x08) |  |  |
| `NO_EQUIPMENT_SLOT_AVAILABLE` | 9 (0x09) |  |  |
| `YOU_CAN_NEVER_USE_THAT_ITEM` | 10 (0x0A) |  |  |
| `YOU_CAN_NEVER_USE_THAT_ITEM2` | 11 (0x0B) |  |  |
| `NO_EQUIPMENT_SLOT_AVAILABLE2` | 12 (0x0C) |  |  |
| `CANT_EQUIP_WITH_TWOHANDED` | 13 (0x0D) |  |  |
| `CANT_DUAL_WIELD` | 14 (0x0E) |  |  |
| `ITEM_DOESNT_GO_INTO_BAG` | 15 (0x0F) |  |  |
| `ITEM_DOESNT_GO_INTO_BAG2` | 16 (0x10) |  |  |
| `CANT_CARRY_MORE_OF_THIS` | 17 (0x11) |  |  |
| `NO_EQUIPMENT_SLOT_AVAILABLE3` | 18 (0x12) |  |  |
| `ITEM_CANT_STACK` | 19 (0x13) |  |  |
| `ITEM_CANT_BE_EQUIPPED` | 20 (0x14) |  |  |
| `ITEMS_CANT_BE_SWAPPED` | 21 (0x15) |  |  |
| `SLOT_IS_EMPTY` | 22 (0x16) |  |  |
| `ITEM_NOT_FOUND` | 23 (0x17) |  |  |
| `CANT_DROP_SOULBOUND` | 24 (0x18) |  |  |
| `OUT_OF_RANGE` | 25 (0x19) |  |  |
| `TRIED_TO_SPLIT_MORE_THAN_COUNT` | 26 (0x1A) |  |  |
| `COULDNT_SPLIT_ITEMS` | 27 (0x1B) |  |  |
| `MISSING_REAGENT` | 28 (0x1C) |  |  |
| `NOT_ENOUGH_MONEY` | 29 (0x1D) |  |  |
| `NOT_A_BAG` | 30 (0x1E) |  |  |
| `CAN_ONLY_DO_WITH_EMPTY_BAGS` | 31 (0x1F) |  |  |
| `DONT_OWN_THAT_ITEM` | 32 (0x20) |  |  |
| `CAN_EQUIP_ONLY1_QUIVER` | 33 (0x21) |  |  |
| `MUST_PURCHASE_THAT_BAG_SLOT` | 34 (0x22) |  |  |
| `TOO_FAR_AWAY_FROM_BANK` | 35 (0x23) |  |  |
| `ITEM_LOCKED` | 36 (0x24) |  |  |
| `YOU_ARE_STUNNED` | 37 (0x25) |  |  |
| `YOU_ARE_DEAD` | 38 (0x26) |  |  |
| `CANT_DO_RIGHT_NOW` | 39 (0x27) |  |  |
| `INT_BAG_ERROR` | 40 (0x28) |  |  |
| `CAN_EQUIP_ONLY1_BOLT` | 41 (0x29) |  |  |
| `CAN_EQUIP_ONLY1_AMMOPOUCH` | 42 (0x2A) |  |  |
| `STACKABLE_CANT_BE_WRAPPED` | 43 (0x2B) |  |  |
| `EQUIPPED_CANT_BE_WRAPPED` | 44 (0x2C) |  |  |
| `WRAPPED_CANT_BE_WRAPPED` | 45 (0x2D) |  |  |
| `BOUND_CANT_BE_WRAPPED` | 46 (0x2E) |  |  |
| `UNIQUE_CANT_BE_WRAPPED` | 47 (0x2F) |  |  |
| `BAGS_CANT_BE_WRAPPED` | 48 (0x30) |  |  |
| `ALREADY_LOOTED` | 49 (0x31) |  |  |
| `INVENTORY_FULL` | 50 (0x32) |  |  |
| `BANK_FULL` | 51 (0x33) |  |  |
| `ITEM_IS_CURRENTLY_SOLD_OUT` | 52 (0x34) |  |  |
| `BAG_FULL3` | 53 (0x35) |  |  |
| `ITEM_NOT_FOUND2` | 54 (0x36) |  |  |
| `ITEM_CANT_STACK2` | 55 (0x37) |  |  |
| `BAG_FULL4` | 56 (0x38) |  |  |
| `ITEM_SOLD_OUT` | 57 (0x39) |  |  |
| `OBJECT_IS_BUSY` | 58 (0x3A) |  |  |
| `NONE` | 59 (0x3B) |  |  |
| `NOT_IN_COMBAT` | 60 (0x3C) |  |  |
| `NOT_WHILE_DISARMED` | 61 (0x3D) |  |  |
| `BAG_FULL6` | 62 (0x3E) |  |  |
| `CANT_EQUIP_RANK` | 63 (0x3F) |  |  |
| `CANT_EQUIP_REPUTATION` | 64 (0x40) |  |  |
| `TOO_MANY_SPECIAL_BAGS` | 65 (0x41) |  |  |
| `LOOT_CANT_LOOT_THAT_NOW` | 66 (0x42) |  |  |