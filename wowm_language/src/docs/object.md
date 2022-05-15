## Client Version 1.12

### Wowm Representation
```rust,ignore
struct Object {
    UpdateType update_type;
    if (update_type == VALUES) {
        PackedGuid guid1;
        UpdateMask mask1;
    }
    else if (update_type == MOVEMENT) {
        PackedGuid guid2;
        MovementBlock movement1;
    }
    else if (update_type == CREATE_OBJECT
        || update_type == CREATE_OBJECT2) {
        PackedGuid guid3;
        ObjectType object_type;
        MovementBlock movement2;
        UpdateMask mask2;
    }
    else if (update_type == OUT_OF_RANGE_OBJECTS
        || update_type == NEAR_OBJECTS) {
        u32 count;
        PackedGuid[count] guids;
    }
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | ? / - | [UpdateType](updatetype.md) | update_type |  |

If update_type is equal to `VALUES`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | guid1 |  |
| - | - / - | [UpdateMask](../spec/update-mask.md) | mask1 |  |

Else If update_type is equal to `MOVEMENT`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | guid2 |  |
| - | ? / - | [MovementBlock](movementblock.md) | movement1 |  |

Else If update_type is equal to `CREATE_OBJECT` **or** 
is equal to `CREATE_OBJECT2`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | guid3 |  |
| - | ? / - | [ObjectType](objecttype.md) | object_type |  |
| - | ? / - | [MovementBlock](movementblock.md) | movement2 |  |
| - | - / - | [UpdateMask](../spec/update-mask.md) | mask2 |  |

Else If update_type is equal to `OUT_OF_RANGE_OBJECTS` **or** 
is equal to `NEAR_OBJECTS`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | u32 | count |  |
| - | ? / - | [PackedGuid](../spec/packed-guid.md)[count] | guids |  |