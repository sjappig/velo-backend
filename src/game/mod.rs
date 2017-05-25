use chrono::DateTime;
use chrono::Duration;
use chrono::Local;

pub type Id = String;

pub struct Game {
    start_time: DateTime<Local>,
    duration: Duration,
    winner: Id,
    loser: Id,
}

impl Game {
    pub fn new(tommi_line: &str) -> Result<Game, String> {
        println!("Converting {}", tommi_line);

        Err(format!("Line was not valid: {}", tommi_line))

//        Ok(Game {
//            start_time: Local::now(),
//            duration: Duration::minutes(1),
//            winner: "ASDF".into(),
//            loser: "FOOBAR".into(),
//        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn invalid_line_creates_error() {
        let invalid_line = &"thislineisinvalid";

        match Game::new(&invalid_line) {
            Ok(_) => panic!("Shouldn't succeed"),
            Err(err) => assert!(err.contains(invalid_line)),
        }
    }
}
