## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_QUESTGIVER_OFFER_REWARD = 0x018D {
    Guid npc;
    u32 quest_id;
    CString title;
    CString offer_reward_text;
    u32 enable_next;
    u32 amount_of_emotes;
    NpcTextUpdateEmote[amount_of_emotes] emotes;
    u32 amount_of_choice_item_rewards;
    QuestItemReward[amount_of_choice_item_rewards] choice_item_rewards;
    u32 amount_of_item_rewards;
    QuestItemReward[amount_of_item_rewards] item_rewards;
    u32 money_reward;
    u32 reward_spell;
    u32 reward_spell_cast;
}
```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | npc |  |
| 0x0C | 4 / Little | u32 | quest_id |  |
| 0x10 | - / - | CString | title |  |
| - | - / - | CString | offer_reward_text |  |
| - | 4 / Little | u32 | enable_next |  |
| - | 4 / Little | u32 | amount_of_emotes |  |
| - | ? / - | [NpcTextUpdateEmote](npctextupdateemote.md)[amount_of_emotes] | emotes |  |
| - | 4 / Little | u32 | amount_of_choice_item_rewards |  |
| - | ? / - | [QuestItemReward](questitemreward.md)[amount_of_choice_item_rewards] | choice_item_rewards |  |
| - | 4 / Little | u32 | amount_of_item_rewards |  |
| - | ? / - | [QuestItemReward](questitemreward.md)[amount_of_item_rewards] | item_rewards |  |
| - | 4 / Little | u32 | money_reward |  |
| - | 4 / Little | u32 | reward_spell |  |
| - | 4 / Little | u32 | reward_spell_cast |  |