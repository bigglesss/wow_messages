## Protocol Version 2

### Wowm Representation
```rust,ignore
slogin CMD_AUTH_LOGON_CHALLENGE_Server = 0x00 {
    u8 protocol_version = 0;
    LoginResult login_result;
    if (login_result == SUCCESS) {
        u8[32] server_public_key;
        u8 generator_length;
        u8[generator_length] generator;
        u8 large_safe_prime_length;
        u8[large_safe_prime_length] large_safe_prime;
        u8[32] salt;
        u8[16] crc_salt;
    }
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 1 / - | u8 | protocol_version |  |
| 0x01 | ? / - | [LoginResult](loginresult.md) | login_result |  |

If login_result is equal to `SUCCESS`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | u8[32] | server_public_key |  |
| - | 1 / - | u8 | generator_length |  |
| - | ? / - | u8[generator_length] | generator |  |
| - | 1 / - | u8 | large_safe_prime_length |  |
| - | ? / - | u8[large_safe_prime_length] | large_safe_prime |  |
| - | ? / - | u8[32] | salt |  |
| - | ? / - | u8[16] | crc_salt |  |
### Examples
```c
0, // opcode (0)
0, // protocol_version: u8
0, // login_result: LoginResult SUCCESS (0x00)
73, 216, 194, 188, 104, 92, 43, 206, 74, 244, 250, 7, 10, 71, 147, 120, 88, 120, 
70, 181, 131, 212, 65, 130, 158, 36, 216, 135, 206, 218, 52, 70, // server_public_key: u8[32]
1, // generator_length: u8
7, // generator: u8[generator_length]
32, // large_safe_prime_length: u8
183, 155, 62, 42, 135, 130, 60, 171, 143, 94, 191, 191, 142, 177, 1, 8, 83, 80, 6, 
41, 139, 91, 173, 189, 91, 83, 225, 137, 94, 100, 75, 137, // large_safe_prime: u8[large_safe_prime_length]
199, 9, 135, 125, 140, 101, 82, 102, 165, 125, 184, 101, 61, 110, 166, 43, 181, 84, 
242, 11, 207, 116, 214, 74, 119, 167, 211, 61, 243, 48, 144, 135, // salt: u8[32]
186, 163, 30, 153, 160, 11, 33, 87, 252, 55, 63, 179, 105, 205, 210, 241, // crc_salt: u8[16]
```
## Protocol Version 3

### Wowm Representation
```rust,ignore
slogin CMD_AUTH_LOGON_CHALLENGE_Server = 0x00 {
    u8 protocol_version = 0;
    LoginResult login_result;
    if (login_result == SUCCESS) {
        u8[32] server_public_key;
        u8 generator_length;
        u8[generator_length] generator;
        u8 large_safe_prime_length;
        u8[large_safe_prime_length] large_safe_prime;
        u8[32] salt;
        u8[16] crc_salt;
        SecurityFlag security_flag;
        if (security_flag == PIN) {
            u32 pin_grid_seed;
            u8[16] pin_salt;
        }
    }
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 1 / - | u8 | protocol_version |  |
| 0x01 | ? / - | [LoginResult](loginresult.md) | login_result |  |

If login_result is equal to `SUCCESS`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | u8[32] | server_public_key |  |
| - | 1 / - | u8 | generator_length |  |
| - | ? / - | u8[generator_length] | generator |  |
| - | 1 / - | u8 | large_safe_prime_length |  |
| - | ? / - | u8[large_safe_prime_length] | large_safe_prime |  |
| - | ? / - | u8[32] | salt |  |
| - | ? / - | u8[16] | crc_salt |  |
| - | ? / - | [SecurityFlag](securityflag.md) | security_flag |  |

If security_flag is equal to `PIN`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | u32 | pin_grid_seed |  |
| - | ? / - | u8[16] | pin_salt |  |
### Examples
```c
0, // opcode (0)
0, // protocol_version: u8
0, // login_result: LoginResult SUCCESS (0x00)
73, 216, 194, 188, 104, 92, 43, 206, 74, 244, 250, 7, 10, 71, 147, 120, 88, 120, 
70, 181, 131, 212, 65, 130, 158, 36, 216, 135, 206, 218, 52, 70, // server_public_key: u8[32]
1, // generator_length: u8
7, // generator: u8[generator_length]
32, // large_safe_prime_length: u8
183, 155, 62, 42, 135, 130, 60, 171, 143, 94, 191, 191, 142, 177, 1, 8, 83, 80, 6, 
41, 139, 91, 173, 189, 91, 83, 225, 137, 94, 100, 75, 137, // large_safe_prime: u8[large_safe_prime_length]
199, 9, 135, 125, 140, 101, 82, 102, 165, 125, 184, 101, 61, 110, 166, 43, 181, 84, 
242, 11, 207, 116, 214, 74, 119, 167, 211, 61, 243, 48, 144, 135, // salt: u8[32]
186, 163, 30, 153, 160, 11, 33, 87, 252, 55, 63, 179, 105, 205, 210, 241, // crc_salt: u8[16]
1, // security_flag: SecurityFlag PIN (0x1)
239, 190, 173, 222, // pin_grid_seed: u32
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // pin_salt: u8[16]
```
```c
0, // opcode (0)
0, // protocol_version: u8
0, // login_result: LoginResult SUCCESS (0x00)
73, 216, 194, 188, 104, 92, 43, 206, 74, 244, 250, 7, 10, 71, 147, 120, 88, 120, 
70, 181, 131, 212, 65, 130, 158, 36, 216, 135, 206, 218, 52, 70, // server_public_key: u8[32]
1, // generator_length: u8
7, // generator: u8[generator_length]
32, // large_safe_prime_length: u8
183, 155, 62, 42, 135, 130, 60, 171, 143, 94, 191, 191, 142, 177, 1, 8, 83, 80, 6, 
41, 139, 91, 173, 189, 91, 83, 225, 137, 94, 100, 75, 137, // large_safe_prime: u8[large_safe_prime_length]
199, 9, 135, 125, 140, 101, 82, 102, 165, 125, 184, 101, 61, 110, 166, 43, 181, 84, 
242, 11, 207, 116, 214, 74, 119, 167, 211, 61, 243, 48, 144, 135, // salt: u8[32]
186, 163, 30, 153, 160, 11, 33, 87, 252, 55, 63, 179, 105, 205, 210, 241, // crc_salt: u8[16]
0, // security_flag: SecurityFlag NONE (0x0)
```
## Protocol Version 8

### Wowm Representation
```rust,ignore
slogin CMD_AUTH_LOGON_CHALLENGE_Server = 0x00 {
    u8 protocol_version = 0;
    LoginResult login_result;
    if (login_result == SUCCESS) {
        u8[32] server_public_key;
        u8 generator_length;
        u8[generator_length] generator;
        u8 large_safe_prime_length;
        u8[large_safe_prime_length] large_safe_prime;
        u8[32] salt;
        u8[16] crc_salt;
        SecurityFlag security_flag;
        if (security_flag & PIN) {
            u32 pin_grid_seed;
            u8[16] pin_salt;
        }
        if (security_flag & UNKNOWN0) {
            u8 unknown0;
            u8 unknown1;
            u8 unknown2;
            u8 unknown3;
            u64 unknown4;
        }
        if (security_flag & AUTHENTICATOR) {
            u8 unknown5;
        }
    }
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 1 / - | u8 | protocol_version |  |
| 0x01 | ? / - | [LoginResult](loginresult.md) | login_result |  |

If login_result is equal to `SUCCESS`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | u8[32] | server_public_key |  |
| - | 1 / - | u8 | generator_length |  |
| - | ? / - | u8[generator_length] | generator |  |
| - | 1 / - | u8 | large_safe_prime_length |  |
| - | ? / - | u8[large_safe_prime_length] | large_safe_prime |  |
| - | ? / - | u8[32] | salt |  |
| - | ? / - | u8[16] | crc_salt |  |
| - | ? / - | [SecurityFlag](securityflag.md) | security_flag |  |

If security_flag contains `PIN`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | u32 | pin_grid_seed |  |
| - | ? / - | u8[16] | pin_salt |  |

If security_flag contains `UNKNOWN0`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 1 / - | u8 | unknown0 |  |
| - | 1 / - | u8 | unknown1 |  |
| - | 1 / - | u8 | unknown2 |  |
| - | 1 / - | u8 | unknown3 |  |
| - | 8 / Little | u64 | unknown4 |  |

If security_flag contains `AUTHENTICATOR`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 1 / - | u8 | unknown5 |  |
### Examples
```c
0, // opcode (0)
0, // protocol_version: u8
0, // login_result: LoginResult SUCCESS (0x00)
73, 216, 194, 188, 104, 92, 43, 206, 74, 244, 250, 7, 10, 71, 147, 120, 88, 120, 
70, 181, 131, 212, 65, 130, 158, 36, 216, 135, 206, 218, 52, 70, // server_public_key: u8[32]
1, // generator_length: u8
7, // generator: u8[generator_length]
32, // large_safe_prime_length: u8
183, 155, 62, 42, 135, 130, 60, 171, 143, 94, 191, 191, 142, 177, 1, 8, 83, 80, 6, 
41, 139, 91, 173, 189, 91, 83, 225, 137, 94, 100, 75, 137, // large_safe_prime: u8[large_safe_prime_length]
199, 9, 135, 125, 140, 101, 82, 102, 165, 125, 184, 101, 61, 110, 166, 43, 181, 84, 
242, 11, 207, 116, 214, 74, 119, 167, 211, 61, 243, 48, 144, 135, // salt: u8[32]
186, 163, 30, 153, 160, 11, 33, 87, 252, 55, 63, 179, 105, 205, 210, 241, // crc_salt: u8[16]
0, // security_flag: SecurityFlag  NONE (0)
```
```c
0, // opcode (0)
0, // protocol_version: u8
0, // login_result: LoginResult SUCCESS (0x00)
73, 216, 194, 188, 104, 92, 43, 206, 74, 244, 250, 7, 10, 71, 147, 120, 88, 120, 
70, 181, 131, 212, 65, 130, 158, 36, 216, 135, 206, 218, 52, 70, // server_public_key: u8[32]
1, // generator_length: u8
7, // generator: u8[generator_length]
32, // large_safe_prime_length: u8
183, 155, 62, 42, 135, 130, 60, 171, 143, 94, 191, 191, 142, 177, 1, 8, 83, 80, 6, 
41, 139, 91, 173, 189, 91, 83, 225, 137, 94, 100, 75, 137, // large_safe_prime: u8[large_safe_prime_length]
199, 9, 135, 125, 140, 101, 82, 102, 165, 125, 184, 101, 61, 110, 166, 43, 181, 84, 
242, 11, 207, 116, 214, 74, 119, 167, 211, 61, 243, 48, 144, 135, // salt: u8[32]
186, 163, 30, 153, 160, 11, 33, 87, 252, 55, 63, 179, 105, 205, 210, 241, // crc_salt: u8[16]
1, // security_flag: SecurityFlag  PIN (1)
239, 190, 173, 222, // pin_grid_seed: u32
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // pin_salt: u8[16]
```
```c
0, // opcode (0)
0, // protocol_version: u8
0, // login_result: LoginResult SUCCESS (0x00)
73, 216, 194, 188, 104, 92, 43, 206, 74, 244, 250, 7, 10, 71, 147, 120, 88, 120, 
70, 181, 131, 212, 65, 130, 158, 36, 216, 135, 206, 218, 52, 70, // server_public_key: u8[32]
1, // generator_length: u8
7, // generator: u8[generator_length]
32, // large_safe_prime_length: u8
183, 155, 62, 42, 135, 130, 60, 171, 143, 94, 191, 191, 142, 177, 1, 8, 83, 80, 6, 
41, 139, 91, 173, 189, 91, 83, 225, 137, 94, 100, 75, 137, // large_safe_prime: u8[large_safe_prime_length]
199, 9, 135, 125, 140, 101, 82, 102, 165, 125, 184, 101, 61, 110, 166, 43, 181, 84, 
242, 11, 207, 116, 214, 74, 119, 167, 211, 61, 243, 48, 144, 135, // salt: u8[32]
186, 163, 30, 153, 160, 11, 33, 87, 252, 55, 63, 179, 105, 205, 210, 241, // crc_salt: u8[16]
4, // security_flag: SecurityFlag  AUTHENTICATOR (4)
1, // unknown5: u8
```
```c
0, // opcode (0)
0, // protocol_version: u8
0, // login_result: LoginResult SUCCESS (0x00)
73, 216, 194, 188, 104, 92, 43, 206, 74, 244, 250, 7, 10, 71, 147, 120, 88, 120, 
70, 181, 131, 212, 65, 130, 158, 36, 216, 135, 206, 218, 52, 70, // server_public_key: u8[32]
1, // generator_length: u8
7, // generator: u8[generator_length]
32, // large_safe_prime_length: u8
183, 155, 62, 42, 135, 130, 60, 171, 143, 94, 191, 191, 142, 177, 1, 8, 83, 80, 6, 
41, 139, 91, 173, 189, 91, 83, 225, 137, 94, 100, 75, 137, // large_safe_prime: u8[large_safe_prime_length]
199, 9, 135, 125, 140, 101, 82, 102, 165, 125, 184, 101, 61, 110, 166, 43, 181, 84, 
242, 11, 207, 116, 214, 74, 119, 167, 211, 61, 243, 48, 144, 135, // salt: u8[32]
186, 163, 30, 153, 160, 11, 33, 87, 252, 55, 63, 179, 105, 205, 210, 241, // crc_salt: u8[16]
2, // security_flag: SecurityFlag  UNKNOWN0 (2)
255, // unknown0: u8
238, // unknown1: u8
221, // unknown2: u8
204, // unknown3: u8
222, 202, 250, 239, 190, 173, 222, 0, // unknown4: u64
```
```c
0, // opcode (0)
0, // protocol_version: u8
5, // login_result: LoginResult FAIL_INCORRECT_PASSWORD (0x05)
```
```c
0, // opcode (0)
0, // protocol_version: u8
0, // login_result: LoginResult SUCCESS (0x00)
73, 216, 194, 188, 104, 92, 43, 206, 74, 244, 250, 7, 10, 71, 147, 120, 88, 120, 
70, 181, 131, 212, 65, 130, 158, 36, 216, 135, 206, 218, 52, 70, // server_public_key: u8[32]
1, // generator_length: u8
7, // generator: u8[generator_length]
32, // large_safe_prime_length: u8
183, 155, 62, 42, 135, 130, 60, 171, 143, 94, 191, 191, 142, 177, 1, 8, 83, 80, 6, 
41, 139, 91, 173, 189, 91, 83, 225, 137, 94, 100, 75, 137, // large_safe_prime: u8[large_safe_prime_length]
199, 9, 135, 125, 140, 101, 82, 102, 165, 125, 184, 101, 61, 110, 166, 43, 181, 84, 
242, 11, 207, 116, 214, 74, 119, 167, 211, 61, 243, 48, 144, 135, // salt: u8[32]
186, 163, 30, 153, 160, 11, 33, 87, 252, 55, 63, 179, 105, 205, 210, 241, // crc_salt: u8[16]
6, // security_flag: SecurityFlag  UNKNOWN0| AUTHENTICATOR (6)
255, // unknown0: u8
238, // unknown1: u8
221, // unknown2: u8
204, // unknown3: u8
222, 202, 250, 239, 190, 173, 222, 0, // unknown4: u64
1, // unknown5: u8
```
```c
0, // opcode (0)
0, // protocol_version: u8
5, // login_result: LoginResult FAIL_INCORRECT_PASSWORD (0x05)
```