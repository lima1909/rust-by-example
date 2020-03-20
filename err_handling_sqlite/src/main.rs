use rusqlite::Connection;
use rusqlite::{params, NO_PARAMS};

// sudo apt-get install libsqlite3-dev
// sudo apt-get install sqlite3 (?)

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {
    let conn = Connection::open_in_memory().unwrap();
    // let conn = Connection::open("my.db").unwrap();

    // only handle the Err and ignore the Ok value
    if let Err(msg) = conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
        NO_PARAMS,
    ) {
        println!("error by creating table: {}", msg);
        return;
    };

    let me = Person {
        id: 0,
        name: "Mario".to_string(),
        data: Some(vec![1, 2, 3]),
    };
    match conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params![me.name, me.data],
    ) {
        Ok(n) => println!("insert: {} rows", n),
        Err(msg) => println!("err by insert: {}", msg),
    };

    let mut stmt = match conn.prepare("SELECT id, name, data FROM person") {
        Ok(stmt) => stmt,
        Err(msg) => {
            println!("err by executing select statement: {}", msg);
            // return is importand
            // in the error case, the stmt has no valid value
            // the Err returns the msg object, that is not compatible with stmt
            return;
        }
    };

    match stmt.query_map(params![], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    }) {
        Ok(person_iter) => {
            for person in person_iter {
                println!("Found person {:?}", person.unwrap());
            }
        }
        Err(msg) => println!("err by select: {}", msg),
    };
}
