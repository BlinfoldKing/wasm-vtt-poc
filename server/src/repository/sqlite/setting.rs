use rusqlite::OptionalExtension;

use super::SqliteRepository;
use crate::{
    entities::setting::Setting,
    error::{Error, ErrorKind},
    repository::setting::SettingRepository,
};

impl SettingRepository for SqliteRepository {
    fn set_setting(&self, setting: Setting) -> Result<Setting, Error> {
        self.conn()?
            .execute(
                "INSERT INTO settings (key, value) VALUES (?1, ?2)",
                (&setting.key, &setting.value),
            )
            .map_err(|err| Error::new(err.to_string(), ErrorKind::InternalError))?;

        Ok(setting)
    }

    fn get_setting(&self, key: &str) -> Result<Option<Setting>, Error> {
        self.conn()?
            .query_row(
                "SELECT key, value FROM settings WHERE key = ?1",
                [key],
                |row| {
                    Ok(Setting {
                        key: row.get(0)?,
                        value: row.get(1)?,
                    })
                },
            )
            .optional()
            .map_err(|err| Error::new(err.to_string(), ErrorKind::InternalError))
    }
}
