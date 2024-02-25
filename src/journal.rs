use std::sync::atomic::{AtomicU64, Ordering};

use rusqlite::Connection;
use teloxide::prelude::UserId;

use std::error::Error;
use std::sync::{Arc, Mutex};
use rusqlite::Error::SqliteFailure;
use crate::debug;

/// The database for the bot
pub struct Journal {
    connection: Arc<Mutex<Connection>>,
    users: AtomicU64,
}

impl Journal {
    /// Connects to the database. File `users.db` must be presented in the local directory. If it is not,
    /// throws an error.
    pub fn new(db_path: &str) -> Result<Self, Box<dyn Error>> {
        let connection = Connection::open(db_path)?;
        match connection.execute(
            "CREATE TABLE users (
                id  INTEGER,
                PRIMARY KEY ( ID )\
            )",
            [],
        ) {
            Ok(_) => {
                debug::print_debug_message("Database succesfully created");
            },
            Err(ref error) => {
                match error {
                    SqliteFailure(error, message) => {
                        if let Some(msg) = message {
                            if !(msg == "table users already exists") {
                                debug::print_debug_error(error);
                                debug::print_debug_message("[ ERROR ] Can't create database");
                            }
                        }
                    }
                    _ => {
                        debug::print_debug_error(error);
                    },
                }
            }
        }

        Ok(
            Self {
                connection: Arc::new(Mutex::new(connection)),
                users: AtomicU64::new(0),
            }
        )
    }

    /// Returns the number of users in the database
    pub fn users(&self) -> u64 {
        self.users.load(Ordering::Relaxed)
    }

    /// Updates the number of users in the database.
    pub fn update(&self) {
        self.users.store(
            self.count_users().expect("Can't count users") as u64,
            Ordering::Relaxed,
        );
    }

    /// Checks if the user is in the database.
    pub fn contains_user(&self, id: UserId) -> bool {
        let mut exist = false;

        {
            let db = self.connection.lock().unwrap();
            let mut prepared = db.prepare("SELECT id FROM users").unwrap();

            let users_iter = prepared.query_map([], |row| {
                Ok(
                    row.get::<usize, u64>(0)?
                )
            }).unwrap();


            for user in users_iter {
                if user.unwrap() == id.0 { exist = true }
            }
        }

        exist
    }

    pub fn add_user(&self, id: UserId) -> Result<(), Box<dyn Error>> {
        self.connection.lock().unwrap().execute(
            "INSERT INTO users VALUES ( ? )",
            [id.0],
        )?;

        self.users.fetch_add(1, Ordering::Relaxed);

        Ok(())
    }

    pub fn count_users(&self) -> Result<usize, Box<dyn Error>> {
        let db = self.connection.lock().unwrap();
        let mut prepared = db.prepare("SELECT id FROM users")?;

        let users_iter = prepared.query_map([], |row| {
            Ok(
                row.get::<usize, u64>(0)?
            )
        })?;

        let mut count = 0;

        for _ in users_iter { count += 1; }

        Ok(count)
    }
}
