## Client Version 1.12

```rust,ignore
smsg SMSG_PET_MODE = 0x17A {
    Guid guid;    
    PetReactState react_state;    
    PetCommandState command_state;    
    u8 unknown1;    
    u8 pet_enabled;    
}

```