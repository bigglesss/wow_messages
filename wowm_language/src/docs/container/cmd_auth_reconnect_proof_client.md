## Protocol Version 2, Protocol Version 8

```rust,ignore
clogin CMD_AUTH_RECONNECT_PROOF_Client = 0x3 {
    u8[16] proof_data;    
    u8[20] client_proof;    
    u8[20] client_checksum;    
    u8 key_count = 0;    
}

```