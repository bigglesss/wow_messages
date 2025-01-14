/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/common_movement_2_4_3.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement_2_4_3.wowm#L3):
/// ```text
/// flag MovementFlags : u32 {
///     NONE = 0x00000000;
///     FORWARD = 0x00000001;
///     BACKWARD = 0x00000002;
///     STRAFE_LEFT = 0x00000004;
///     STRAFE_RIGHT = 0x00000008;
///     TURN_LEFT = 0x00000010;
///     TURN_RIGHT = 0x00000020;
///     PITCH_UP = 0x00000040;
///     PITCH_DOWN = 0x00000080;
///     WALK_MODE = 0x00000100;
///     ON_TRANSPORT = 0x00000200;
///     LEVITATING = 0x00000400;
///     FIXED_Z = 0x00000800;
///     ROOT = 0x00001000;
///     JUMPING = 0x00002000;
///     FALLINGFAR = 0x00004000;
///     SWIMMING = 0x00200000;
///     ASCENDING = 0x00400000;
///     CAN_FLY = 0x00800000;
///     FLYING = 0x01000000;
///     ONTRANSPORT = 0x02000000;
///     SPLINE_ELEVATION = 0x04000000;
///     SPLINE_ENABLED = 0x08000000;
///     WATERWALKING = 0x10000000;
///     SAFE_FALL = 0x20000000;
///     HOVER = 0x40000000;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
pub struct MovementFlags {
    inner: u32,
}

impl MovementFlags {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub(crate) const NONE: u32 = 0x00;
    pub(crate) const FORWARD: u32 = 0x01;
    pub(crate) const BACKWARD: u32 = 0x02;
    pub(crate) const STRAFE_LEFT: u32 = 0x04;
    pub(crate) const STRAFE_RIGHT: u32 = 0x08;
    pub(crate) const TURN_LEFT: u32 = 0x10;
    pub(crate) const TURN_RIGHT: u32 = 0x20;
    pub(crate) const PITCH_UP: u32 = 0x40;
    pub(crate) const PITCH_DOWN: u32 = 0x80;
    pub(crate) const WALK_MODE: u32 = 0x100;
    pub(crate) const ON_TRANSPORT: u32 = 0x200;
    pub(crate) const LEVITATING: u32 = 0x400;
    pub(crate) const FIXED_Z: u32 = 0x800;
    pub(crate) const ROOT: u32 = 0x1000;
    pub(crate) const JUMPING: u32 = 0x2000;
    pub(crate) const FALLINGFAR: u32 = 0x4000;
    pub(crate) const SWIMMING: u32 = 0x200000;
    pub(crate) const ASCENDING: u32 = 0x400000;
    pub(crate) const CAN_FLY: u32 = 0x800000;
    pub(crate) const FLYING: u32 = 0x1000000;
    pub(crate) const ONTRANSPORT: u32 = 0x2000000;
    pub(crate) const SPLINE_ELEVATION: u32 = 0x4000000;
    pub(crate) const SPLINE_ENABLED: u32 = 0x8000000;
    pub(crate) const WATERWALKING: u32 = 0x10000000;
    pub(crate) const SAFE_FALL: u32 = 0x20000000;
    pub(crate) const HOVER: u32 = 0x40000000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::FORWARD
                | Self::BACKWARD
                | Self::STRAFE_LEFT
                | Self::STRAFE_RIGHT
                | Self::TURN_LEFT
                | Self::TURN_RIGHT
                | Self::PITCH_UP
                | Self::PITCH_DOWN
                | Self::WALK_MODE
                | Self::ON_TRANSPORT
                | Self::LEVITATING
                | Self::FIXED_Z
                | Self::ROOT
                | Self::JUMPING
                | Self::FALLINGFAR
                | Self::SWIMMING
                | Self::ASCENDING
                | Self::CAN_FLY
                | Self::FLYING
                | Self::ONTRANSPORT
                | Self::SPLINE_ELEVATION
                | Self::SPLINE_ENABLED
                | Self::WATERWALKING
                | Self::SAFE_FALL
                | Self::HOVER
        }
    }

    pub const fn is_FORWARD(&self) -> bool {
        (self.inner & Self::FORWARD) != 0
    }

    pub const fn new_FORWARD() -> Self {
        Self { inner: Self::FORWARD }
    }

    pub fn set_FORWARD(&mut self) -> Self {
        self.inner |= Self::FORWARD;
        *self
    }

    pub fn clear_FORWARD(&mut self) -> Self {
        self.inner &= Self::FORWARD.reverse_bits();
        *self
    }

    pub const fn is_BACKWARD(&self) -> bool {
        (self.inner & Self::BACKWARD) != 0
    }

    pub const fn new_BACKWARD() -> Self {
        Self { inner: Self::BACKWARD }
    }

    pub fn set_BACKWARD(&mut self) -> Self {
        self.inner |= Self::BACKWARD;
        *self
    }

    pub fn clear_BACKWARD(&mut self) -> Self {
        self.inner &= Self::BACKWARD.reverse_bits();
        *self
    }

    pub const fn is_STRAFE_LEFT(&self) -> bool {
        (self.inner & Self::STRAFE_LEFT) != 0
    }

    pub const fn new_STRAFE_LEFT() -> Self {
        Self { inner: Self::STRAFE_LEFT }
    }

    pub fn set_STRAFE_LEFT(&mut self) -> Self {
        self.inner |= Self::STRAFE_LEFT;
        *self
    }

    pub fn clear_STRAFE_LEFT(&mut self) -> Self {
        self.inner &= Self::STRAFE_LEFT.reverse_bits();
        *self
    }

    pub const fn is_STRAFE_RIGHT(&self) -> bool {
        (self.inner & Self::STRAFE_RIGHT) != 0
    }

    pub const fn new_STRAFE_RIGHT() -> Self {
        Self { inner: Self::STRAFE_RIGHT }
    }

    pub fn set_STRAFE_RIGHT(&mut self) -> Self {
        self.inner |= Self::STRAFE_RIGHT;
        *self
    }

    pub fn clear_STRAFE_RIGHT(&mut self) -> Self {
        self.inner &= Self::STRAFE_RIGHT.reverse_bits();
        *self
    }

    pub const fn is_TURN_LEFT(&self) -> bool {
        (self.inner & Self::TURN_LEFT) != 0
    }

    pub const fn new_TURN_LEFT() -> Self {
        Self { inner: Self::TURN_LEFT }
    }

    pub fn set_TURN_LEFT(&mut self) -> Self {
        self.inner |= Self::TURN_LEFT;
        *self
    }

    pub fn clear_TURN_LEFT(&mut self) -> Self {
        self.inner &= Self::TURN_LEFT.reverse_bits();
        *self
    }

    pub const fn is_TURN_RIGHT(&self) -> bool {
        (self.inner & Self::TURN_RIGHT) != 0
    }

    pub const fn new_TURN_RIGHT() -> Self {
        Self { inner: Self::TURN_RIGHT }
    }

    pub fn set_TURN_RIGHT(&mut self) -> Self {
        self.inner |= Self::TURN_RIGHT;
        *self
    }

    pub fn clear_TURN_RIGHT(&mut self) -> Self {
        self.inner &= Self::TURN_RIGHT.reverse_bits();
        *self
    }

    pub const fn is_PITCH_UP(&self) -> bool {
        (self.inner & Self::PITCH_UP) != 0
    }

    pub const fn new_PITCH_UP() -> Self {
        Self { inner: Self::PITCH_UP }
    }

    pub fn set_PITCH_UP(&mut self) -> Self {
        self.inner |= Self::PITCH_UP;
        *self
    }

    pub fn clear_PITCH_UP(&mut self) -> Self {
        self.inner &= Self::PITCH_UP.reverse_bits();
        *self
    }

    pub const fn is_PITCH_DOWN(&self) -> bool {
        (self.inner & Self::PITCH_DOWN) != 0
    }

    pub const fn new_PITCH_DOWN() -> Self {
        Self { inner: Self::PITCH_DOWN }
    }

    pub fn set_PITCH_DOWN(&mut self) -> Self {
        self.inner |= Self::PITCH_DOWN;
        *self
    }

    pub fn clear_PITCH_DOWN(&mut self) -> Self {
        self.inner &= Self::PITCH_DOWN.reverse_bits();
        *self
    }

    pub const fn is_WALK_MODE(&self) -> bool {
        (self.inner & Self::WALK_MODE) != 0
    }

    pub const fn new_WALK_MODE() -> Self {
        Self { inner: Self::WALK_MODE }
    }

    pub fn set_WALK_MODE(&mut self) -> Self {
        self.inner |= Self::WALK_MODE;
        *self
    }

    pub fn clear_WALK_MODE(&mut self) -> Self {
        self.inner &= Self::WALK_MODE.reverse_bits();
        *self
    }

    pub const fn is_ON_TRANSPORT(&self) -> bool {
        (self.inner & Self::ON_TRANSPORT) != 0
    }

    pub const fn new_ON_TRANSPORT() -> Self {
        Self { inner: Self::ON_TRANSPORT }
    }

    pub fn set_ON_TRANSPORT(&mut self) -> Self {
        self.inner |= Self::ON_TRANSPORT;
        *self
    }

    pub fn clear_ON_TRANSPORT(&mut self) -> Self {
        self.inner &= Self::ON_TRANSPORT.reverse_bits();
        *self
    }

    pub const fn is_LEVITATING(&self) -> bool {
        (self.inner & Self::LEVITATING) != 0
    }

    pub const fn new_LEVITATING() -> Self {
        Self { inner: Self::LEVITATING }
    }

    pub fn set_LEVITATING(&mut self) -> Self {
        self.inner |= Self::LEVITATING;
        *self
    }

    pub fn clear_LEVITATING(&mut self) -> Self {
        self.inner &= Self::LEVITATING.reverse_bits();
        *self
    }

    pub const fn is_FIXED_Z(&self) -> bool {
        (self.inner & Self::FIXED_Z) != 0
    }

    pub const fn new_FIXED_Z() -> Self {
        Self { inner: Self::FIXED_Z }
    }

    pub fn set_FIXED_Z(&mut self) -> Self {
        self.inner |= Self::FIXED_Z;
        *self
    }

    pub fn clear_FIXED_Z(&mut self) -> Self {
        self.inner &= Self::FIXED_Z.reverse_bits();
        *self
    }

    pub const fn is_ROOT(&self) -> bool {
        (self.inner & Self::ROOT) != 0
    }

    pub const fn new_ROOT() -> Self {
        Self { inner: Self::ROOT }
    }

    pub fn set_ROOT(&mut self) -> Self {
        self.inner |= Self::ROOT;
        *self
    }

    pub fn clear_ROOT(&mut self) -> Self {
        self.inner &= Self::ROOT.reverse_bits();
        *self
    }

    pub const fn is_JUMPING(&self) -> bool {
        (self.inner & Self::JUMPING) != 0
    }

    pub const fn new_JUMPING() -> Self {
        Self { inner: Self::JUMPING }
    }

    pub fn set_JUMPING(&mut self) -> Self {
        self.inner |= Self::JUMPING;
        *self
    }

    pub fn clear_JUMPING(&mut self) -> Self {
        self.inner &= Self::JUMPING.reverse_bits();
        *self
    }

    pub const fn is_FALLINGFAR(&self) -> bool {
        (self.inner & Self::FALLINGFAR) != 0
    }

    pub const fn new_FALLINGFAR() -> Self {
        Self { inner: Self::FALLINGFAR }
    }

    pub fn set_FALLINGFAR(&mut self) -> Self {
        self.inner |= Self::FALLINGFAR;
        *self
    }

    pub fn clear_FALLINGFAR(&mut self) -> Self {
        self.inner &= Self::FALLINGFAR.reverse_bits();
        *self
    }

    pub const fn is_SWIMMING(&self) -> bool {
        (self.inner & Self::SWIMMING) != 0
    }

    pub const fn new_SWIMMING() -> Self {
        Self { inner: Self::SWIMMING }
    }

    pub fn set_SWIMMING(&mut self) -> Self {
        self.inner |= Self::SWIMMING;
        *self
    }

    pub fn clear_SWIMMING(&mut self) -> Self {
        self.inner &= Self::SWIMMING.reverse_bits();
        *self
    }

    pub const fn is_ASCENDING(&self) -> bool {
        (self.inner & Self::ASCENDING) != 0
    }

    pub const fn new_ASCENDING() -> Self {
        Self { inner: Self::ASCENDING }
    }

    pub fn set_ASCENDING(&mut self) -> Self {
        self.inner |= Self::ASCENDING;
        *self
    }

    pub fn clear_ASCENDING(&mut self) -> Self {
        self.inner &= Self::ASCENDING.reverse_bits();
        *self
    }

    pub const fn is_CAN_FLY(&self) -> bool {
        (self.inner & Self::CAN_FLY) != 0
    }

    pub const fn new_CAN_FLY() -> Self {
        Self { inner: Self::CAN_FLY }
    }

    pub fn set_CAN_FLY(&mut self) -> Self {
        self.inner |= Self::CAN_FLY;
        *self
    }

    pub fn clear_CAN_FLY(&mut self) -> Self {
        self.inner &= Self::CAN_FLY.reverse_bits();
        *self
    }

    pub const fn is_FLYING(&self) -> bool {
        (self.inner & Self::FLYING) != 0
    }

    pub const fn new_FLYING() -> Self {
        Self { inner: Self::FLYING }
    }

    pub fn set_FLYING(&mut self) -> Self {
        self.inner |= Self::FLYING;
        *self
    }

    pub fn clear_FLYING(&mut self) -> Self {
        self.inner &= Self::FLYING.reverse_bits();
        *self
    }

    pub const fn is_ONTRANSPORT(&self) -> bool {
        (self.inner & Self::ONTRANSPORT) != 0
    }

    pub const fn new_ONTRANSPORT() -> Self {
        Self { inner: Self::ONTRANSPORT }
    }

    pub fn set_ONTRANSPORT(&mut self) -> Self {
        self.inner |= Self::ONTRANSPORT;
        *self
    }

    pub fn clear_ONTRANSPORT(&mut self) -> Self {
        self.inner &= Self::ONTRANSPORT.reverse_bits();
        *self
    }

    pub const fn is_SPLINE_ELEVATION(&self) -> bool {
        (self.inner & Self::SPLINE_ELEVATION) != 0
    }

    pub const fn new_SPLINE_ELEVATION() -> Self {
        Self { inner: Self::SPLINE_ELEVATION }
    }

    pub fn set_SPLINE_ELEVATION(&mut self) -> Self {
        self.inner |= Self::SPLINE_ELEVATION;
        *self
    }

    pub fn clear_SPLINE_ELEVATION(&mut self) -> Self {
        self.inner &= Self::SPLINE_ELEVATION.reverse_bits();
        *self
    }

    pub const fn is_SPLINE_ENABLED(&self) -> bool {
        (self.inner & Self::SPLINE_ENABLED) != 0
    }

    pub const fn new_SPLINE_ENABLED() -> Self {
        Self { inner: Self::SPLINE_ENABLED }
    }

    pub fn set_SPLINE_ENABLED(&mut self) -> Self {
        self.inner |= Self::SPLINE_ENABLED;
        *self
    }

    pub fn clear_SPLINE_ENABLED(&mut self) -> Self {
        self.inner &= Self::SPLINE_ENABLED.reverse_bits();
        *self
    }

    pub const fn is_WATERWALKING(&self) -> bool {
        (self.inner & Self::WATERWALKING) != 0
    }

    pub const fn new_WATERWALKING() -> Self {
        Self { inner: Self::WATERWALKING }
    }

    pub fn set_WATERWALKING(&mut self) -> Self {
        self.inner |= Self::WATERWALKING;
        *self
    }

    pub fn clear_WATERWALKING(&mut self) -> Self {
        self.inner &= Self::WATERWALKING.reverse_bits();
        *self
    }

    pub const fn is_SAFE_FALL(&self) -> bool {
        (self.inner & Self::SAFE_FALL) != 0
    }

    pub const fn new_SAFE_FALL() -> Self {
        Self { inner: Self::SAFE_FALL }
    }

    pub fn set_SAFE_FALL(&mut self) -> Self {
        self.inner |= Self::SAFE_FALL;
        *self
    }

    pub fn clear_SAFE_FALL(&mut self) -> Self {
        self.inner &= Self::SAFE_FALL.reverse_bits();
        *self
    }

    pub const fn is_HOVER(&self) -> bool {
        (self.inner & Self::HOVER) != 0
    }

    pub const fn new_HOVER() -> Self {
        Self { inner: Self::HOVER }
    }

    pub fn set_HOVER(&mut self) -> Self {
        self.inner |= Self::HOVER;
        *self
    }

    pub fn clear_HOVER(&mut self) -> Self {
        self.inner &= Self::HOVER.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}

