## Client Version 1.12

```rust,ignore
smsg MSG_RAID_TARGET_UPDATE_Server = 0x321 {
    RaidTargetUpdateType update_type;    
    if (update_type == FULL) {        
        RaidTargetUpdate[8] raid_targets;        
    }    
    else if (update_type == PARTIAL) {        
        RaidTargetUpdate raid_target;        
    }    
}

```