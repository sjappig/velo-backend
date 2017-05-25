use regex::Regex;

pub type Id = String;
pub type Elo = i16; // realistic range: 500-3000

const UNDEFINED_ELO: Elo = -1;

#[derive(Debug, Serialize, PartialEq)]
pub struct Player {
    pub name: String,
    pub id: Id,
    pub elo: Elo,
}

pub fn get_all() -> Vec<Player> {
    return vec![ Player{ name: "Ismo".into(), id: "123".into(), elo: 123 } ];
}


impl Player {
    pub fn parse(tommi_line: &str) -> Result<Player, String> {
        lazy_static! {
            // data from Velo: 64-character alphanumeric id, name
            static ref PARSER_RE: Regex = Regex::new("^([[:alnum:]]{64}),([^,]*)$").unwrap();
        }

        match PARSER_RE.captures(tommi_line) {
            Some(captures) => {
                let id = String::from(&captures[1]);
                let name = String::from(&captures[2]);

                Ok(Player {
                       name: name,
                       id: id,
                       elo: UNDEFINED_ELO,
                   })
            }
            None => Err(format!("Could not parse the player line: {}", tommi_line)),
        }
    }
}

#[cfg(test)]
mod tests {
    use player::Player;
    use player::UNDEFINED_ELO;

    const TEST_LINE: &str = "yBsgFK65Je24kPStpG60mySQAstqtZytURNqUPb8fXbWNTD93tNCMkl2Jhzv7ymy,Ismo";

    #[test]
    fn it_parses_a_tommi_line() {
        let player: Player = Player::parse(TEST_LINE).unwrap();

        assert_eq!("Ismo", player.name);
        assert_eq!("yBsgFK65Je24kPStpG60mySQAstqtZytURNqUPb8fXbWNTD93tNCMkl2Jhzv7ymy",
                   player.id);
        assert_eq!(UNDEFINED_ELO, player.elo);
    }

    #[test]
    fn invalid_line_creates_error() {
        match Player::parse(&"THIS LINE IS INVALID") {
            Ok(_) => panic!("Parsing an invalid line should not succeed"),
            Err(err) => assert!(!err.is_empty()),
        }
    }
}
