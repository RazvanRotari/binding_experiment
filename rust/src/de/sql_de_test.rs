use rusqlite::{Connection, NO_PARAMS, Statement};

use traits::Deserializable;
use models::person::Player;

struct TestDriver {
    conn: Connection
}

impl TestDriver {
    fn new() -> Self {
        let result = Connection::open_in_memory().unwrap();
        TestDriver {
            conn: result
        }
    }

    fn set_up(&self) -> bool {
        self.conn.execute(
            "CREATE TABLE person (
                  id                INTEGER PRIMARY KEY,
                  name              TEXT NOT NULL,
                  nickname          TEXT,
                  age               INTEGER NOT NULL,
                  credits           REAL NOT NULL,
                  has_companion     BOOLEAN NOT NULL,
                  game_score        INTEGER NOT NULL)",
            rusqlite::params![],
        ).is_ok()
    }

    fn insert(&self, person: &Player) -> bool {
        self.conn.execute(
            "INSERT INTO person (name, nickname, age, credits, has_companion, game_score)
                      VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![person.name,  person.nickname, person.age, person.credits as f64, person.has_companion, person.game_score],
        ).is_ok()
    }

    fn query(&self, query: &str) -> Statement {
        self.conn.prepare(query).unwrap()
    }

    fn tear_own(&self) -> bool {
        let result = self.conn
            .execute("DROP TABLE person", rusqlite::params![]).is_ok();
        result
    }
}

#[test]
fn should_map_person_from_sql() {
    let test = TestDriver::new();
    test.set_up();
    let player = Player::new(0, String::from("Paul"), String::from("Paul"), 22,
                             -22f32, false, 100);

    test.insert(&player);

    let mut statement = test.query("SELECT * FROM person");

    let iter = statement.query_map(NO_PARAMS, |row| {
        Ok(Player::unmarshal(row).unwrap())
    });

    let option = iter.unwrap().next().unwrap();

    assert_eq!(option.is_ok(), true);

    let result: Player = option.ok().unwrap();

    assert_eq!(result, player);

    test.tear_own();
}

#[test]
fn should_map_name_from_sql() {
    let test = TestDriver::new();
    test.set_up();

    let player = Player::new(0, String::from("Paul"), String::from("Paul"), 22,
                             -22f32, false, 100);

    test.insert(&player);

    let mut statement = test.query("SELECT name FROM person");

    let iter = statement.query_map(NO_PARAMS, |row| {
        Ok(String::unmarshal(row).unwrap())
    });

    let option = iter.unwrap().next().unwrap();

    assert_eq!(option.is_ok(), true);

    let name: String = option.ok().unwrap();

    assert_eq!(name, "Paul".to_owned());

    test.tear_own();
}