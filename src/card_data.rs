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
                "SELECT CardID, Name, Cost, Effects FROM Cards WHERE CardID={}",
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
                })
            })
            .unwrap();

        card_iter.nth(0).unwrap().unwrap()
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct CardData {
    id: i32,
    name: String,
    cost: Option<i32>,
    effects: String,
}

impl CardData {
    pub fn spawn_as_entity(&mut self, commands: &mut CommandBuffer) -> Result<Entity, String> {
        if let Ok(card_effects) = get_card_effects_from_text(self.effects.to_owned()) {
            let entity = commands.push((
                (),
                Card {
                    name: self.name.to_owned(),
                },
            ));

            if let Some(cost) = self.cost {
                commands.add_component(entity, EnergyCost { amount: cost });
            }

            let mut is_targeted = false;

            card_effects.iter().for_each(|effect| match effect {
                CardEffect::DealDamage(target, amount) => match target {
                    Target::Hero => {
                        eprintln!("Effect Not Implemented: Self Damage");
                    }

                    Target::Enemy => {
                        is_targeted = true;
                        commands.add_component(entity, DealsDamage { amount: *amount });
                    }
                },

                CardEffect::Block(amount) => {
                    commands.add_component(entity, AddsBlock { amount: *amount })
                }

                CardEffect::InflictVulnerability(target, amount) => match target {
                    Target::Enemy => {
                        is_targeted = true;
                        commands.add_component(entity, InflictVulnerability { amount: *amount });
                    }

                    Target::Hero => {
                        eprintln!("Effect Not Implemented: Self Inflicted Vulnerability")
                    }
                },

                _ => {
                    eprintln!(
                        "Unimplemented Card Effect!\n{:?}\nIn Card: {}",
                        effect, self.name
                    )
                }
            });

            if is_targeted {
                commands.add_component(entity, SelectTarget);
            }

            Ok(entity)
        } else {
            Err("Failed to Parse".to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Target {
    Hero,
    Enemy,
}

#[derive(Debug, Clone, PartialEq)]
enum CardEffect {
    DealDamage(Target, i32),
    Block(i32),
    InflictVulnerability(Target, i32),
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
                        "deal" => {
                            return Ok(CardEffect::DealDamage(
                                Target::Enemy,
                                params.nth(0).unwrap(),
                            ));
                        }

                        "block" => {
                            return Ok(CardEffect::Block(params.nth(0).unwrap()));
                        }

                        "vuln" => {
                            return Ok(CardEffect::InflictVulnerability(
                                Target::Enemy,
                                params.nth(0).unwrap(),
                            ))
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
            effects: "deal(6);".to_string(),
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_deal_damage_string() {
        let amount = 4;
        let effect_string = format!("deal({})", amount);
        let actual = CardEffect::from_string(effect_string).unwrap();

        let expected = CardEffect::DealDamage(Target::Enemy, amount);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_multiple_commands_in_string() {
        let expected = Ok(vec![
            CardEffect::DealDamage(Target::Enemy, 3),
            CardEffect::Block(4),
            CardEffect::InflictVulnerability(Target::Enemy, 5),
        ]);
        let effect_string = "deal(3);block(4)\n;\nvuln(5);";

        let actual = get_card_effects_from_text(effect_string.to_string());

        assert_eq!(actual, expected)
    }
}
