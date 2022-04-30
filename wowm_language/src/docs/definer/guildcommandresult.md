## Client Version 1.12

```rust,ignore
enum GuildCommandResult : u8 {
    PLAYER_NO_MORE_IN_GUILD = 0x00;    
    GUILD_INTERNAL = 0x01;    
    ALREADY_IN_GUILD = 0x02;    
    ALREADY_IN_GUILD_S = 0x03;    
    INVITED_TO_GUILD = 0x04;    
    ALREADY_INVITED_TO_GUILD_S = 0x05;    
    GUILD_NAME_INVALID = 0x06;    
    GUILD_NAME_EXISTS_S = 0x07;    
    GUILD_LEADER_LEAVE = 0x08;    
    GUILD_PERMISSIONS = 0x08;    
    GUILD_PLAYER_NOT_IN_GUILD = 0x09;    
    GUILD_PLAYER_NOT_IN_GUILD_S = 0x0A;    
    GUILD_PLAYER_NOT_FOUND_S = 0x0B;    
    GUILD_NOT_ALLIED = 0x0C;    
    GUILD_RANK_TOO_HIGH_S = 0x0D;    
    GUILD_RANK_TOO_LOW_S = 0x0E;    
}

```