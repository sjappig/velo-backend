use chrono::DateTime;
use chrono::Duration;
use chrono::Local;
use chrono::TimeZone;
use std::error::Error;

pub type Id = String;

#[derive(PartialEq, Debug)]
pub struct Game {
    start_time: DateTime<Local>,
    duration: Duration,
    winner: Id,
    loser: Id,
}

impl Game {
    //! Create new *Game* from string *tommi_line*.
    //! Starting time and duration are rounded down to nearest full second.
    pub fn new(tommi_line: &str) -> Result<Game, String> {
        println!("Converting {}", tommi_line);

        let tokens: Vec<&str> = tommi_line.split(',').collect();
        
        if tokens.len() != 5 {
            return Err(format!("Line should have exatcly 5 commas"));
        }

        let timestamp = tokens[3].parse::<f64>().map_err(|e| {
            e.description().to_string()
        })?;
        let duration = tokens[4].parse::<f64>().map_err(|e| {
            e.description().to_string()
        })?;
        let winner = tokens[2];
        let loser = if tokens[0] != winner { tokens[0] } else { tokens[1] };
 
        Ok(Game {
            start_time: Local.ymd(2001, 1, 1).and_hms(0,0,0) + Duration::seconds(timestamp as i64),
            duration: Duration::seconds(duration as i64),
            winner: winner.into(),
            loser: loser.into(),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn invalid_line_creates_error() {
        let invalid_line = &"thislineisinvalid";

        match Game::new(&invalid_line) {
            Ok(_) => panic!("Should not succeed"),
            Err(err) => assert!(!err.is_empty()),
        }
    }

    #[test]
    fn valid_line_creates_game() {
        let valid_line = "muBF4sNpDCwLqLiVne7M8WtW6DJg1OQrbumx1HpBmkfVVsv7c1iNhHf3SBNNQd6s,\
                          1hMduK6YEqJeAeZvd2bI9mI5bWSnRSZihsH5XdjdpViWPZiGK5cH8L0JVkbTEb0A,\
                          1hMduK6YEqJeAeZvd2bI9mI5bWSnRSZihsH5XdjdpViWPZiGK5cH8L0JVkbTEb0A,\
                          516099153.307128,\
                          337.512281000614";

        match Game::new(&valid_line) {
            Ok(result) => assert_eq!(Game {
                                        start_time: Local.ymd(2001, 1, 1).and_hms(0,0,0) + Duration::seconds(516099153),
                                        duration: Duration::seconds(337),
                                        winner: "1hMduK6YEqJeAeZvd2bI9mI5bWSnRSZihsH5XdjdpViWPZiGK5cH8L0JVkbTEb0A".into(),
                                        loser: "muBF4sNpDCwLqLiVne7M8WtW6DJg1OQrbumx1HpBmkfVVsv7c1iNhHf3SBNNQd6s".into(),
                                     }, result),
            Err(err) => panic!("Should not fail: {}", err),
        }
    }
}
