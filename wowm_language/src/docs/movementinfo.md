## Client Version 1.12

### Wowm Representation
```rust,ignore
struct MovementInfo {
    MovementFlags flags;
    u32 timestamp;
    f32 position_x;
    f32 position_y;
    f32 position_z;
    f32 orientation;
    if (flags & ON_TRANSPORT) {
        TransportInfo transport;
    }
    if (flags & SWIMMING) {
        f32 pitch;
    }
    f32 fall_time;
    if (flags & JUMPING) {
        f32 z_speed;
        f32 cos_angle;
        f32 sin_angle;
        f32 xy_speed;
    }
    if (flags & SPLINE_ELEVATION) {
        f32 spline_elevation;
    }
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | ? / - | [MovementFlags](movementflags.md) | flags |  |
| - | 4 / Little | u32 | timestamp |  |
| - | 4 / Little | f32 | position_x |  |
| - | 4 / Little | f32 | position_y |  |
| - | 4 / Little | f32 | position_z |  |
| - | 4 / Little | f32 | orientation |  |

If flags contains `ON_TRANSPORT`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | [TransportInfo](transportinfo.md) | transport |  |

If flags contains `SWIMMING`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | f32 | pitch |  |
| - | 4 / Little | f32 | fall_time |  |

If flags contains `JUMPING`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | f32 | z_speed |  |
| - | 4 / Little | f32 | cos_angle |  |
| - | 4 / Little | f32 | sin_angle |  |
| - | 4 / Little | f32 | xy_speed |  |

If flags contains `SPLINE_ELEVATION`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | f32 | spline_elevation |  |