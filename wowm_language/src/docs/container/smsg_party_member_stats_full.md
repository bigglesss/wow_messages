## Client Version 1.12

```rust,ignore
smsg SMSG_PARTY_MEMBER_STATS_FULL = 0x2F2 {
    PackedGuid player;    
    GroupUpdateFlags mask;    
    if (mask & FLAG_STATUS) {        
        GroupMemberOnlineStatus status;        
    }    
    if (mask & FLAG_CUR_HP) {        
        u16 current_health;        
    }    
    if (mask & FLAG_MAX_HP) {        
        u16 max_health;        
    }    
    if (mask & FLAG_POWER_TYPE) {        
        Power power;        
    }    
    if (mask & FLAG_CUR_POWER) {        
        u16 current_power;        
    }    
    if (mask & FLAG_MAX_POWER) {        
        u16 max_power;        
    }    
    if (mask & FLAG_LEVEL) {        
        u16 level;        
    }    
    if (mask & FLAG_ZONE) {        
        Area area;        
    }    
    if (mask & FLAG_POSITION) {        
        u16 position_x;        
        u16 position_y;        
    }    
    if (mask & FLAG_AURAS) {        
        AuraMask auras;        
    }    
    if (mask & FLAG_PET_NAME) {        
        CString pet_name;        
    }    
    if (mask & FLAG_PET_MODEL_ID) {        
        u16 pet_display_id;        
    }    
    if (mask & FLAG_PET_CUR_HP) {        
        u16 pet_current_health;        
    }    
    if (mask & FLAG_PET_MAX_HP) {        
        u16 pet_max_health;        
    }    
    if (mask & FLAG_PET_POWER_TYPE) {        
        Power pet_power_type;        
    }    
    if (mask & FLAG_PET_CUR_POWER) {        
        u16 pet_current_power;        
    }    
    if (mask & FLAG_PET_MAX_POWER) {        
        u16 pet_max_power;        
    }    
    if (mask & FLAG_PET_AURAS) {        
        AuraMask pet_auras;        
    }    
}

```