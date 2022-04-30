## Client Version 1.12

```rust,ignore
struct SpellCastTargets {
    SpellCastTargetFlags target_flags;    
    if (target_flags & UNIT) {        
        PackedGuid unit_target1;        
    }    
    if (target_flags & UNIT_ENEMY) {        
        PackedGuid unit_target2;        
    }    
    if (target_flags & GAMEOBJECT) {        
        PackedGuid object_target1;        
    }    
    if (target_flags & LOCKED) {        
        PackedGuid object_target2;        
    }    
    if (target_flags & ITEM) {        
        PackedGuid item_target1;        
    }    
    if (target_flags & TRADE_ITEM) {        
        PackedGuid item_target2;        
    }    
    if (target_flags & SOURCE_LOCATION) {        
        f32 position_x1;        
        f32 position_y1;        
        f32 position_z1;        
    }    
    if (target_flags & DEST_LOCATION) {        
        f32 position_x2;        
        f32 position_y2;        
        f32 position_z2;        
    }    
    if (target_flags & STRING) {        
        CString target_string;        
    }    
    if (target_flags & CORPSE_ALLY) {        
        PackedGuid corpse_target1;        
    }    
    if (target_flags & CORPSE_ENEMY) {        
        PackedGuid corpse_target2;        
    }    
}

```