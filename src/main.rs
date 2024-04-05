#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
use rusqlite::Connection;

// short_url generation 
fn to_base62(val: &u64) -> String {
    let base62_chars: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut result = String::new();

    let mut n = val.clone();
    loop {
        let remainder = (n % 62) as usize;
        result.push(base62_chars[remainder]);

        n /= 62;
        if n == 0 {
            break;
        }
    }

    result.chars().rev().collect()
}

// DB INTERFACE
#[derive(Debug)]
struct Url {
    short_url_integer: u64,
    short_url_string: String,
    original_url: String,
}

fn create_db(name: &str) -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(name)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS urls (
            ID INTEGER PRIMARY KEY,
            short_url_integer INTEGER NOT NULL UNIQUE,
            short_url_string TEXT NOT NULL UNIQUE,
            original_url TEXT NOT NULL UNIQUE
        )",
        (),
    )?;

    Ok(conn)
}

fn db_get_url_by_short_url_string(conn: &Connection, short_url: &str) -> Result<Option<Url>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT short_url_integer, short_url_string, original_url FROM urls WHERE short_url_string = ?")?;
    let mut rows = stmt.query([short_url])?;

    match rows.next()? {
        Some(row) => {
            let short_url_integer: u64 = row.get(0)?;
            let short_url_string: String = row.get(1)?;
            let original_url: String = row.get(2)?;
            Ok(Some(Url { short_url_integer, short_url_string, original_url }))
        }
        None => Ok(None),
    }
}

fn db_add_url(conn: &Connection, url: &Url) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT OR IGNORE INTO urls (short_url_integer, short_url_string, original_url) VALUES (?, ?, ?)",
        (&url.short_url_integer, &url.short_url_string, &url.original_url),
    )?;

    Ok(())
}

fn db_get_highest_short_url(conn: &Connection) -> Result<Option<u64>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT MAX(short_url_integer) FROM urls")?;
    let highest_id: Option<u64> = stmt.query_row([], |row| row.get(0)).unwrap_or(None);
    Ok(highest_id)
}

fn db_get_url_by_original_url(conn: &Connection, original_url: &str) -> Result<Option<Url>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT short_url_integer, short_url_string, original_url FROM urls WHERE original_url = ?")?;
    let mut rows = stmt.query([original_url])?;

    match rows.next()? {
        Some(row) => {
            let short_url_integer: u64 = row.get(0)?;
            let short_url_string: String = row.get(1)?;
            let original_url: String = row.get(2)?;
            Ok(Some(Url {
                short_url_integer,
                short_url_string,
                original_url,
            }))
        }
        None => Ok(None),
    }
}


// ROCKET

#[get("/")]
fn index() -> &'static str {
    "Welcome to Pitico, your very very simple URL shortener"
}

#[get("/register/<original_url>")]
fn register_value(original_url: &str) -> String {
    let conn = create_db("pitico.db").unwrap();

    let registered_url = db_get_url_by_original_url(&conn, &original_url);
    match registered_url {
        Ok(url) => {
            match url {
                Some(url) => {
                    format!("URL \"{}\" already registered under {}", url.original_url, url.short_url_string)
                }
                None => {
                    let highest_short_url = db_get_highest_short_url(&conn).unwrap();
                    let short_url_integer = match highest_short_url {
                        Some(short_url) => short_url + 1,
                        None => 1,
                    };
                    let short_url_string = to_base62(&short_url_integer);
                    let url = Url {
                        short_url_integer,
                        short_url_string: short_url_string.to_string(),
                        original_url: original_url.to_string(),
                    };
                    db_add_url(&conn, &url).unwrap();
                    format!("URL registered under: {}", short_url_string.clone())
                }
            }
        }
        Err(error) => {
            format!("Error registering URL: {}", error)
        }
    }
}

#[get("/register")]
fn register() -> &'static str {
    "Please provide an URL to be shortened"
}

#[get("/<short_url>")]
fn redirect(short_url: &str) -> Redirect {
    let conn = create_db("pitico.db").unwrap();
    let url = db_get_url_by_short_url_string(&conn, &short_url).unwrap();
    match url {
        Some(url) => {
            println!("Redirecting to: {}", url.original_url);
            Redirect::to(format!("http://{}", url.original_url))
        },
        None => {
            Redirect::to(format!("/url_not_found/{}", short_url))
        },
    }
}

#[get("/url_not_found/<short_url>")]
fn url_not_found(short_url: &str) -> String {
    format!("Pitico URL {} not found", short_url)
}

#[launch]
fn rocket() -> _ {
    create_db("pitico.db").unwrap();
    rocket::build().mount("/", routes![
        index,
        register,
        register_value,
        redirect,
        url_not_found,
        ])
}