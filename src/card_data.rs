pub use crate::prelude::*;
use rusqlite::Connection;

pub struct CardDB {
    connection: Connection,
}

impl CardDB {
    pub fn new() -> Self {
        CardDB {
            connection: Connection::open("./assets/cards.db").unwrap(),
        }
    }

    pub fn get_card_from_id(&mut self, id: i32) -> CardData {
        let mut stmt = self
            .connection
            .prepare(&format!(
                "SELECT CardID, Name, Cost, Effects, Type FROM Cards WHERE CardID={}",
                id
            ))
            .unwrap();

        let mut card_iter = stmt
            .query_map([], |row| {
                Ok(CardData {
                    id: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    cost: row.get(2).unwrap(),
                    effects: row.get(3).unwrap(),
                    card_type: row.get(4).unwrap(),
                })
            })
            .unwrap();

        card_iter.nth(0).unwrap().unwrap()
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct CardData {
    id: i32,
    pub name: String,
    card_type: String,
    cost: Option<i32>,
    effects: String,
}

impl CardData {
    pub fn spawn_as_entity(&self, commands: &mut CommandBuffer) -> Result<Entity, String> {
        let card_effects =
            get_card_effects_from_text(self.effects.to_owned()).expect("Failed to parse Card Text");

        let card_type = match self.card_type.as_str() {
            "A" => CardType::Attack,
            "S" => CardType::Skill,
            "P" => CardType::Power,
            _ => {
                eprintln!("Unexpected Type Code for a Card");
                CardType::Attack
            }
        };

        let entity = commands.push((
            (),
            Card {
                name: self.name.to_owned(),
                card_type,
            },
        ));

        if let Some(cost) = self.cost {
            commands.add_component(entity, EnergyCost { amount: cost });
        }

        let mut target_defined = false;
        let mut requirements = Vec::new();

        card_effects.iter().for_each(|effect| match effect {
            CardEffect::TargetType(target) => {
                target_defined = true;

                match target {
                    Target::Hero => {
                        warn!("Not Implemented: Self Targeting Cards that Aren't Block");
                    }

                    Target::Enemy => {
                        commands.add_component(entity, SelectTarget);
                    }

                    Target::AllEnemies => {
                        commands.add_component(entity, AllEnemies);
                    }
                }
            }

            CardEffect::DealDamage(amount) => {
                commands.add_component(entity, DealsDamage(*amount));
            }

            CardEffect::DealBlockAsDamage => {
                commands.add_component(entity, DealBlock);
            }

            CardEffect::Block(amount) => {
                commands.add_component(entity, AddsBlock { amount: *amount })
            }

            CardEffect::InflictStatus(status, amount) => {
                commands.add_component(
                    entity,
                    InflictsStatus {
                        status: *status,
                        amount: *amount,
                    },
                );
            }

            CardEffect::AddIDToDiscard(id) => commands.add_component(
                entity,
                AddCardToZone {
                    zone: CardZone::Discard,
                    id: *id,
                },
            ),

            CardEffect::Require(requirement) => {
                requirements.push(*requirement);
            }
        });

        if !target_defined {
            eprintln!("No target explicitly set for card: {}", self.name);
        }

        if requirements.len() > 0 {
            commands.add_component(entity, PlayConditions { requirements });
        }

        Ok(entity)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Target {
    Hero,
    Enemy,
    AllEnemies,
}

#[derive(Debug, Clone, PartialEq)]
enum CardEffect {
    TargetType(Target),
    DealDamage(i32),
    DealBlockAsDamage,
    Block(i32),
    InflictStatus(Status, i32),
    AddIDToDiscard(i32),
    Require(Requirement),
}

impl CardEffect {
    fn from_string(val: String) -> Result<Self, String> {
        if let Some(open_paren_pos) = val.find("(") {
            if let Some(close_paren_pos) = val.find(")") {
                if close_paren_pos > open_paren_pos {
                    let keyword = val[0..open_paren_pos].to_string();
                    let params_str = val[open_paren_pos + 1..close_paren_pos].to_string();

                    let mut params = params_str
                        .split(",")
                        .into_iter()
                        .map(|val| val.trim().parse::<i32>().unwrap());

                    match keyword.as_str() {
                        "one_enemy" => return Ok(CardEffect::TargetType(Target::Enemy)),
                        "self" => return Ok(CardEffect::TargetType(Target::Hero)),
                        "all_enemies" => return Ok(CardEffect::TargetType(Target::AllEnemies)),
                        "deal" => {
                            return Ok(CardEffect::DealDamage(params.nth(0).unwrap()));
                        }

                        "deal_block" => {
                            return Ok(CardEffect::DealBlockAsDamage);
                        }

                        "block" => {
                            return Ok(CardEffect::Block(params.nth(0).unwrap()));
                        }

                        "vuln" => {
                            return Ok(CardEffect::InflictStatus(
                                Status::Vulnerability,
                                params.nth(0).unwrap(),
                            ))
                        }

                        "weak" => {
                            return Ok(CardEffect::InflictStatus(
                                Status::Weakness,
                                params.nth(0).unwrap(),
                            ))
                        }

                        "add_to_discard" => {
                            return Ok(CardEffect::AddIDToDiscard(params.nth(0).unwrap()))
                        }

                        "require_only_attacks" => {
                            return Ok(CardEffect::Require(Requirement::AllCardsInHandAre(
                                CardType::Attack,
                            )))
                        }

                        _ => return Err(format!("could not parse command: {}", keyword)),
                    }
                }
            }
        }

        Err(format!("Could not successfully parse string: {}", val))
    }
}

fn get_card_effects_from_text(val: String) -> Result<Vec<CardEffect>, String> {
    let effects = val
        .split(';')
        .map(|command| command.trim())
        .filter(|command| *command != "")
        .map(|command| CardEffect::from_string(command.to_string()))
        .collect::<Result<Vec<CardEffect>, String>>();

    effects
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_by_id() {
        let mut db = CardDB::new();

        let actual = db.get_card_from_id(1);
        let expected = CardData {
            id: 1,
            name: "Strike".to_string(),
            cost: Some(1),
            card_type: "A".to_string(),
            effects: "one_enemy();\ndeal(6);".to_string(),
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_deal_damage_string() {
        let amount = 4;
        let effect_string = format!("deal({})", amount);
        let actual = CardEffect::from_string(effect_string).unwrap();

        let expected = CardEffect::DealDamage(amount);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_multiple_commands_in_string() {
        let expected = Ok(vec![
            CardEffect::DealDamage(3),
            CardEffect::Block(4),
            CardEffect::InflictStatus(Status::Vulnerability, 5),
        ]);
        let effect_string = "deal(3);block(4)\n;\nvuln(5);";

        let actual = get_card_effects_from_text(effect_string.to_string());

        assert_eq!(actual, expected)
    }
}
