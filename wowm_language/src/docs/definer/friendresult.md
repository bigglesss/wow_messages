## Client Version 1.12

```rust,ignore
enum FriendResult : u8 {
    DB_ERROR = 0x00;    
    LIST_FULL = 0x01;    
    ONLINE = 0x02;    
    OFFLINE = 0x03;    
    NOT_FOUND = 0x04;    
    REMOVED = 0x05;    
    ADDED_ONLINE = 0x06;    
    ADDED_OFFLINE = 0x07;    
    ALREADY = 0x08;    
    SELF = 0x09;    
    ENEMY = 0x0A;    
    IGNORE_FULL = 0x0B;    
    IGNORE_SELF = 0x0C;    
    IGNORE_NOT_FOUND = 0x0D;    
    IGNORE_ALREADY = 0x0E;    
    IGNORE_ADDED = 0x0F;    
    IGNORE_REMOVED = 0x10;    
    IGNORE_AMBIGUOUS = 0x11;    
    MUTE_FULL = 0x12;    
    MUTE_SELF = 0x13;    
    MUTE_NOT_FOUND = 0x14;    
    MUTE_ALREADY = 0x15;    
    MUTE_ADDED = 0x16;    
    MUTE_REMOVED = 0x17;    
    MUTE_AMBIGUOUS = 0x18;    
    UNKNOWN19 = 0x19;    
    UNKNOWN20 = 0x1A;    
}

```