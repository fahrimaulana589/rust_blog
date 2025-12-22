use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, CustomizeConnection};

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug)]
pub struct ConnectionOptions {
    pub enable_wal: bool,
    pub enable_foreign_keys: bool,
    pub busy_timeout: Option<u64>,
}

impl CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for ConnectionOptions {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        (|| {
            if self.enable_wal {
                conn.batch_execute("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
            }
            if self.enable_foreign_keys {
                conn.batch_execute("PRAGMA foreign_keys = ON;")?;
            }
            if let Some(timeout) = self.busy_timeout {
                conn.batch_execute(&format!("PRAGMA busy_timeout = {};", timeout))?;
            }
            conn.batch_execute("PRAGMA mmap_size = 100000;")?;
            Ok(())
        })()
        .map_err(diesel::r2d2::Error::QueryError)
    }
}

pub fn establish_connection(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .connection_customizer(Box::new(ConnectionOptions {
            enable_wal: true,
            enable_foreign_keys: true,
            busy_timeout: Some(5000),
        }))
        .build(manager)
        .expect("Failed to create pool.")
}
