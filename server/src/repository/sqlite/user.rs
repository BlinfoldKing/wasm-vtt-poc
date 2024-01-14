use rusqlite::OptionalExtension;

use super::SqliteRepository;
use crate::{
    entities::user::User,
    error::{Error, ErrorKind},
    repository::user::UserRepository,
};

impl UserRepository for SqliteRepository {
    fn save_user(&self, user: User) -> Result<User, Error> {
        self.conn()?
            .execute(
                "INSERT INTO users (id, username, password_hash) VALUES (?1, ?2, ?3)",
                (&user.id, &user.username, &user.password_hash),
            )
            .map_err(|err| Error::new(err.to_string(), ErrorKind::InternalError))?;

        Ok(user)
    }

    fn get_user_by_id(&self, id: uuid::Uuid) -> Result<Option<User>, Error> {
        self.conn()?
            .query_row(
                "SELECT id, username, password_hash FROM users WHERE id = ?1",
                [id],
                |row| {
                    Ok(User {
                        id: row.get(0)?,
                        username: row.get(1)?,
                        password_hash: row.get(2)?,
                    })
                },
            )
            .optional()
            .map_err(|err| Error::new(err.to_string(), ErrorKind::InternalError))
    }

    fn get_users(&self) -> Result<Vec<User>, Error> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare("SELECT id, username, password_hash FROM users")
            .map_err(|err| Error::new(err.to_string(), ErrorKind::InternalError))?;

        let user_iter = stmt
            .query_map([], |row| {
                Ok(User {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    password_hash: row.get(2)?,
                })
            })
            .map_err(|err| Error::new(err.to_string(), ErrorKind::InternalError))?;

        Ok(user_iter.map(|user| user.unwrap()).collect())
    }
}
