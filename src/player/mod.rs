use regex::Regex;

pub type Id = String;
pub type Elo = i16; // realistic range: 500-3000

const UNDEFINED_ELO: Elo = -1;

pub struct Player {
    name: String,
    id: Id,
    elo: Elo
}

impl Player {
    pub fn parse(tommi_line: &str) -> Player {
        lazy_static! {
            // data from Velo: 64-character alphanumeric id, name
            static ref PARSER_RE: Regex = Regex::new("^([[:alnum:]]{64}),([^,]*)$").unwrap();
        }

        // FIXME: should this have error handling? probably only run once to init db?
        let captures = PARSER_RE.captures(tommi_line).unwrap();
        
        let id = String::from(&captures[1]);
        let name = String::from(&captures[2]);

        Player {
            name: name,
            id: id,
            elo: UNDEFINED_ELO
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
        let player: Player = Player::parse(TEST_LINE);

        assert_eq!("Ismo", player.name);
        assert_eq!("yBsgFK65Je24kPStpG60mySQAstqtZytURNqUPb8fXbWNTD93tNCMkl2Jhzv7ymy", player.id);
        assert_eq!(UNDEFINED_ELO, player.elo);
    }
}