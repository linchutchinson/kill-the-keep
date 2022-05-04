# Card Commands

Inside of the SQLite DB cards.db, all of the card data for this game is stored and can be edited. It uses a small set of custom commands to define card effects. These commands are defined one per line with a semicolon at the end of the line and a pair of parentheses that can contain parameters where necessary.

Here is a list of the currently implemented commands and their effects. New commands are added
via the card_data.rs file.

## Command List

### Targeting Commands
Use one of these commands with every card in order to specify the target of the card's effects.

* one_enemy() == Mark an effect as targeting a single enemy. For example, when combined with the deal(x) command, the card would deal damage to a single targeted enemy.

* self() == Mark an effect as targeting the player character. This command is not currently implemented and is only here for completeness when building cards like Block.

* all_enemies() == Mark an effect as targeting all enemy monsters. For example, when combined with deal(x), the amount of damage specified will be dealt to all enemies.

### Other Commands

* deal(x) == Deal X damage to a target determined by a targeting command.

* deal_block() == Deal an amount of damage equal to your current block.

* block(x) == Gain x Block

* vuln(x) == Inflict X vulnerability on the targeted combatant(s).

* weak(x) == Inflict X Weakness on the targeted combatant(s).
