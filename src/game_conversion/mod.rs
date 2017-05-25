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

pub fn to_struct(tommi_line: &str) -> Game {
    println!("Converting {}", tommi_line);
    Game {
        start_time: Local::now(),
        duration: Duration::minutes(1),
        winner: "ASDF".into(),
        loser: "FOOBAR".into(),
    }
}

#[cfg(test)]
mod tests {

    use chrono::Duration;
    use game_conversion::to_struct;

    #[test]
    fn it_works() {
        assert_eq!(Duration::minutes(1), to_struct(&"kekeke").duration);
    }
}
