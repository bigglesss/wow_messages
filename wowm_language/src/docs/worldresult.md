## Client Version 1.2, Client Version 1.12

### Wowm Representation
```rust,ignore
enum WorldResult : u32 {
    RESPONSE_SUCCESS = 0x00;
    RESPONSE_FAILURE = 0x01;
    RESPONSE_CANCELLED = 0x02;
    RESPONSE_DISCONNECTED = 0x03;
    RESPONSE_FAILED_TO_CONNECT = 0x04;
    RESPONSE_CONNECTED = 0x05;
    RESPONSE_VERSION_MISMATCH = 0x06;
    CSTATUS_CONNECTING = 0x07;
    CSTATUS_NEGOTIATING_SECURITY = 0x08;
    CSTATUS_NEGOTIATION_COMPLETE = 0x09;
    CSTATUS_NEGOTIATION_FAILED = 0x0A;
    CSTATUS_AUTHENTICATING = 0x0B;
    AUTH_OK = 0x0C;
    AUTH_FAILED = 0x0D;
    AUTH_REJECT = 0x0E;
    AUTH_BAD_SERVER_PROOF = 0x0F;
    AUTH_UNAVAILABLE = 0x10;
    AUTH_SYSTEM_ERROR = 0x11;
    AUTH_BILLING_ERROR = 0x12;
    AUTH_BILLING_EXPIRED = 0x13;
    AUTH_VERSION_MISMATCH = 0x14;
    AUTH_UNKNOWN_ACCOUNT = 0x15;
    AUTH_INCORRECT_PASSWORD = 0x16;
    AUTH_SESSION_EXPIRED = 0x17;
    AUTH_SERVER_SHUTTING_DOWN = 0x18;
    AUTH_ALREADY_LOGGING_IN = 0x19;
    AUTH_LOGIN_SERVER_NOT_FOUND = 0x1A;
    AUTH_WAIT_QUEUE = 0x1B;
    AUTH_BANNED = 0x1C;
    AUTH_ALREADY_ONLINE = 0x1D;
    AUTH_NO_TIME = 0x1E;
    AUTH_DB_BUSY = 0x1F;
    AUTH_SUSPENDED = 0x20;
    AUTH_PARENTAL_CONTROL = 0x21;
    REALM_LIST_IN_PROGRESS = 0x22;
    REALM_LIST_SUCCESS = 0x23;
    REALM_LIST_FAILED = 0x24;
    REALM_LIST_INVALID = 0x25;
    REALM_LIST_REALM_NOT_FOUND = 0x26;
    ACCOUNT_CREATE_IN_PROGRESS = 0x27;
    ACCOUNT_CREATE_SUCCESS = 0x28;
    ACCOUNT_CREATE_FAILED = 0x29;
    CHAR_LIST_RETRIEVING = 0x2A;
    CHAR_LIST_RETRIEVED = 0x2B;
    CHAR_LIST_FAILED = 0x2C;
    CHAR_CREATE_IN_PROGRESS = 0x2D;
    CHAR_CREATE_SUCCESS = 0x2E;
    CHAR_CREATE_ERROR = 0x2F;
    CHAR_CREATE_FAILED = 0x30;
    CHAR_CREATE_NAME_IN_USE = 0x31;
    CHAR_CREATE_DISABLED = 0x32;
    CHAR_CREATE_PVP_TEAMS_VIOLATION = 0x33;
    CHAR_CREATE_SERVER_LIMIT = 0x34;
    CHAR_CREATE_ACCOUNT_LIMIT = 0x35;
    CHAR_CREATE_SERVER_QUEUE = 0x36;
    CHAR_CREATE_ONLY_EXISTING = 0x37;
    CHAR_DELETE_IN_PROGRESS = 0x38;
    CHAR_DELETE_SUCCESS = 0x39;
    CHAR_DELETE_FAILED = 0x3A;
    CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER = 0x3B;
    CHAR_LOGIN_IN_PROGRESS = 0x3C;
    CHAR_LOGIN_SUCCESS = 0x3D;
    CHAR_LOGIN_NO_WORLD = 0x3E;
    CHAR_LOGIN_DUPLICATE_CHARACTER = 0x3F;
    CHAR_LOGIN_NO_INSTANCES = 0x40;
    CHAR_LOGIN_FAILED = 0x41;
    CHAR_LOGIN_DISABLED = 0x42;
    CHAR_LOGIN_NO_CHARACTER = 0x43;
    CHAR_LOGIN_LOCKED_FOR_TRANSFER = 0x44;
    CHAR_NAME_NO_NAME = 0x45;
    CHAR_NAME_TOO_SHORT = 0x46;
    CHAR_NAME_TOO_LONG = 0x47;
    CHAR_NAME_ONLY_LETTERS = 0x48;
    CHAR_NAME_MIXED_LANGUAGES = 0x49;
    CHAR_NAME_PROFANE = 0x4A;
    CHAR_NAME_RESERVED = 0x4B;
    CHAR_NAME_INVALID_APOSTROPHE = 0x4C;
    CHAR_NAME_MULTIPLE_APOSTROPHES = 0x4D;
    CHAR_NAME_THREE_CONSECUTIVE = 0x4E;
    CHAR_NAME_INVALID_SPACE = 0x4F;
    CHAR_NAME_SUCCESS = 0x50;
    CHAR_NAME_FAILURE = 0x51;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `RESPONSE_SUCCESS` | 0 (0x00) |  |  |
| `RESPONSE_FAILURE` | 1 (0x01) |  |  |
| `RESPONSE_CANCELLED` | 2 (0x02) |  |  |
| `RESPONSE_DISCONNECTED` | 3 (0x03) |  |  |
| `RESPONSE_FAILED_TO_CONNECT` | 4 (0x04) |  |  |
| `RESPONSE_CONNECTED` | 5 (0x05) |  |  |
| `RESPONSE_VERSION_MISMATCH` | 6 (0x06) |  |  |
| `CSTATUS_CONNECTING` | 7 (0x07) |  |  |
| `CSTATUS_NEGOTIATING_SECURITY` | 8 (0x08) |  |  |
| `CSTATUS_NEGOTIATION_COMPLETE` | 9 (0x09) |  |  |
| `CSTATUS_NEGOTIATION_FAILED` | 10 (0x0A) |  |  |
| `CSTATUS_AUTHENTICATING` | 11 (0x0B) |  |  |
| `AUTH_OK` | 12 (0x0C) |  |  |
| `AUTH_FAILED` | 13 (0x0D) |  |  |
| `AUTH_REJECT` | 14 (0x0E) |  |  |
| `AUTH_BAD_SERVER_PROOF` | 15 (0x0F) |  |  |
| `AUTH_UNAVAILABLE` | 16 (0x10) |  |  |
| `AUTH_SYSTEM_ERROR` | 17 (0x11) |  |  |
| `AUTH_BILLING_ERROR` | 18 (0x12) |  |  |
| `AUTH_BILLING_EXPIRED` | 19 (0x13) |  |  |
| `AUTH_VERSION_MISMATCH` | 20 (0x14) |  |  |
| `AUTH_UNKNOWN_ACCOUNT` | 21 (0x15) |  |  |
| `AUTH_INCORRECT_PASSWORD` | 22 (0x16) |  |  |
| `AUTH_SESSION_EXPIRED` | 23 (0x17) |  |  |
| `AUTH_SERVER_SHUTTING_DOWN` | 24 (0x18) |  |  |
| `AUTH_ALREADY_LOGGING_IN` | 25 (0x19) |  |  |
| `AUTH_LOGIN_SERVER_NOT_FOUND` | 26 (0x1A) |  |  |
| `AUTH_WAIT_QUEUE` | 27 (0x1B) |  |  |
| `AUTH_BANNED` | 28 (0x1C) |  |  |
| `AUTH_ALREADY_ONLINE` | 29 (0x1D) |  |  |
| `AUTH_NO_TIME` | 30 (0x1E) |  |  |
| `AUTH_DB_BUSY` | 31 (0x1F) |  |  |
| `AUTH_SUSPENDED` | 32 (0x20) |  |  |
| `AUTH_PARENTAL_CONTROL` | 33 (0x21) |  |  |
| `REALM_LIST_IN_PROGRESS` | 34 (0x22) |  |  |
| `REALM_LIST_SUCCESS` | 35 (0x23) |  |  |
| `REALM_LIST_FAILED` | 36 (0x24) |  |  |
| `REALM_LIST_INVALID` | 37 (0x25) |  |  |
| `REALM_LIST_REALM_NOT_FOUND` | 38 (0x26) |  |  |
| `ACCOUNT_CREATE_IN_PROGRESS` | 39 (0x27) |  |  |
| `ACCOUNT_CREATE_SUCCESS` | 40 (0x28) |  |  |
| `ACCOUNT_CREATE_FAILED` | 41 (0x29) |  |  |
| `CHAR_LIST_RETRIEVING` | 42 (0x2A) |  |  |
| `CHAR_LIST_RETRIEVED` | 43 (0x2B) |  |  |
| `CHAR_LIST_FAILED` | 44 (0x2C) |  |  |
| `CHAR_CREATE_IN_PROGRESS` | 45 (0x2D) |  |  |
| `CHAR_CREATE_SUCCESS` | 46 (0x2E) |  |  |
| `CHAR_CREATE_ERROR` | 47 (0x2F) |  |  |
| `CHAR_CREATE_FAILED` | 48 (0x30) |  |  |
| `CHAR_CREATE_NAME_IN_USE` | 49 (0x31) |  |  |
| `CHAR_CREATE_DISABLED` | 50 (0x32) |  |  |
| `CHAR_CREATE_PVP_TEAMS_VIOLATION` | 51 (0x33) |  |  |
| `CHAR_CREATE_SERVER_LIMIT` | 52 (0x34) |  |  |
| `CHAR_CREATE_ACCOUNT_LIMIT` | 53 (0x35) |  |  |
| `CHAR_CREATE_SERVER_QUEUE` | 54 (0x36) |  |  |
| `CHAR_CREATE_ONLY_EXISTING` | 55 (0x37) |  |  |
| `CHAR_DELETE_IN_PROGRESS` | 56 (0x38) |  |  |
| `CHAR_DELETE_SUCCESS` | 57 (0x39) |  |  |
| `CHAR_DELETE_FAILED` | 58 (0x3A) |  |  |
| `CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER` | 59 (0x3B) |  |  |
| `CHAR_LOGIN_IN_PROGRESS` | 60 (0x3C) |  |  |
| `CHAR_LOGIN_SUCCESS` | 61 (0x3D) |  |  |
| `CHAR_LOGIN_NO_WORLD` | 62 (0x3E) |  |  |
| `CHAR_LOGIN_DUPLICATE_CHARACTER` | 63 (0x3F) |  |  |
| `CHAR_LOGIN_NO_INSTANCES` | 64 (0x40) |  |  |
| `CHAR_LOGIN_FAILED` | 65 (0x41) |  |  |
| `CHAR_LOGIN_DISABLED` | 66 (0x42) |  |  |
| `CHAR_LOGIN_NO_CHARACTER` | 67 (0x43) |  |  |
| `CHAR_LOGIN_LOCKED_FOR_TRANSFER` | 68 (0x44) |  |  |
| `CHAR_NAME_NO_NAME` | 69 (0x45) |  |  |
| `CHAR_NAME_TOO_SHORT` | 70 (0x46) |  |  |
| `CHAR_NAME_TOO_LONG` | 71 (0x47) |  |  |
| `CHAR_NAME_ONLY_LETTERS` | 72 (0x48) |  |  |
| `CHAR_NAME_MIXED_LANGUAGES` | 73 (0x49) |  |  |
| `CHAR_NAME_PROFANE` | 74 (0x4A) |  |  |
| `CHAR_NAME_RESERVED` | 75 (0x4B) |  |  |
| `CHAR_NAME_INVALID_APOSTROPHE` | 76 (0x4C) |  |  |
| `CHAR_NAME_MULTIPLE_APOSTROPHES` | 77 (0x4D) |  |  |
| `CHAR_NAME_THREE_CONSECUTIVE` | 78 (0x4E) |  |  |
| `CHAR_NAME_INVALID_SPACE` | 79 (0x4F) |  |  |
| `CHAR_NAME_SUCCESS` | 80 (0x50) |  |  |
| `CHAR_NAME_FAILURE` | 81 (0x51) |  |  |