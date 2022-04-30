## Client Version 1.12

```rust,ignore
flag MovementFlags : u32 {
    NONE = 0x00000000;    
    FORWARD = 0x00000001;    
    BACKWARD = 0x00000002;    
    STRAFE_LEFT = 0x00000004;    
    STRAFE_RIGHT = 0x00000008;    
    TURN_LEFT = 0x00000010;    
    TURN_RIGHT = 0x00000020;    
    PITCH_UP = 0x00000040;    
    PITCH_DOWN = 0x00000080;    
    WALK_MODE = 0x00000100;    
    ON_TRANSPORT = 0x00000200;    
    LEVITATING = 0x00000400;    
    FIXED_Z = 0x00000800;    
    ROOT = 0x00001000;    
    JUMPING = 0x00002000;    
    FALLINGFAR = 0x00004000;    
    SWIMMING = 0x00200000;    
    SPLINE_ENABLED = 0x00400000;    
    CAN_FLY = 0x00800000;    
    FLYING = 0x01000000;    
    ONTRANSPORT = 0x02000000;    
    SPLINE_ELEVATION = 0x04000000;    
    WATERWALKING = 0x10000000;    
    SAFE_FALL = 0x20000000;    
    HOVER = 0x40000000;    
}

```