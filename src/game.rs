use chrono::DateTime;
use chrono::Local;
use chrono::TimeZone;
use chrono;
use error::VeloError;
use id::Id;
use postgres;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::error::Error;
use std::time::Duration;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Game {
    pub start_time: DateTime<Local>,
    pub duration: Duration,
    pub winner: Id,
    pub loser: Id,
}

impl Game {
    /// Create new *Game* from string *tommi_line*.
    /// Starting time and duration are rounded down to nearest full second.
    pub fn new(tommi_line: &str) -> Result<Game, String> {
        println!("Converting {}", tommi_line);

        let tokens: Vec<&str> = tommi_line.split(',').collect();

        if tokens.len() != 5 {
            return Err(format!("Line should have exatcly 5 commas"));
        }

        let timestamp = tokens[3]
            .parse::<f64>()
            .map_err(|e| e.description().to_string())?;
        let duration = tokens[4]
            .parse::<f64>()
            .map_err(|e| e.description().to_string())?;
        let winner = tokens[2];
        let loser = if tokens[0] != winner {
            tokens[0]
        } else {
            tokens[1]
        };

        let winner = Id::new(winner).map_err(|e| e.0)?;
        let loser = Id::new(loser).map_err(|e| e.0)?;

        Ok(Game {
               start_time: Local.ymd(2001, 1, 1).and_hms(0, 0, 0) +
                           chrono::Duration::seconds(timestamp as i64),
               duration: Duration::from_secs(duration as u64),
               winner,
               loser,
           })
    }

    pub fn get_all(conn: &postgres::Connection) -> Result<Vec<Game>, VeloError> {
        let mut ret = vec![];
        for row in conn.query("SELECT * FROM games order by start_time DESC", &[])
                .map_err(|e| VeloError::DbError(e.description().into()))?
                .iter() {
            let winner_str: String = row.get(1);
            let loser_str: String = row.get(2);
            if let (Ok(winner), Ok(loser)) = (Id::new(&winner_str[..]), Id::new(&loser_str[..])) {
                ret.push(Game {
                             winner,
                             loser,
                             start_time: row.get(3),
                             duration: Duration::from_secs(row.get::<usize, i32>(4) as u64),
                         });
            } else {
                // TODO: Logging
                println!("Could not add game between {} and {}",
                         winner_str,
                         loser_str);
            }
        }
        Ok(ret)
    }
}

impl Serialize for Game {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("Game", 4)?;
        state.serialize_field("winner", &self.winner)?;
        state.serialize_field("loser", &self.loser)?;
        state.serialize_field("start_time", &self.start_time)?;
        state.serialize_field("duration", &self.duration.as_secs())?;
        state.end()
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
            Ok(result) => {
                assert_eq!(Game {
                               start_time: Local.ymd(2001, 1, 1).and_hms(0, 0, 0) +
                                           chrono::Duration::seconds(516099153),
                               duration: Duration::from_secs(337),
                               winner: Id::new("1hMduK6YEqJeAeZvd2bI9mI5bWSnRSZihsH5XdjdpViWPZiGK5cH8L0JVkbTEb0A").unwrap(),
                               loser: Id::new("muBF4sNpDCwLqLiVne7M8WtW6DJg1OQrbumx1HpBmkfVVsv7c1iNhHf3SBNNQd6s").unwrap(),
                           },
                           result)
            }
            Err(err) => panic!("Should not fail: {}", err),
        }
    }
}
