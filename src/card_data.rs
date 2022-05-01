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
                    effects: row.get(2).unwrap(),
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
    effects: String,
}

impl CardData {
    pub fn spawn_as_entity(&mut self, commands: &mut CommandBuffer) {}
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

                    let mut params = params_str.split(",").into_iter().map(|val| {
                        println!("{}", val);
                        val.trim().parse::<i32>().unwrap()
                    });

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

        Err("Fail".to_string())
    }
}

struct CardCode;

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
            effects: "damage(6);".to_string(),
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
}
