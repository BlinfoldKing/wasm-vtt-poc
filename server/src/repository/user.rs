use crate::entities::user::User;
use crate::error::Error;

use super::Repository;

pub trait UserRepository
where
    Self: Repository,
{
    fn save_user(&self, user: User) -> Result<User, Error>;
    fn get_user_by_id(&self, id: uuid::Uuid) -> Result<Option<User>, Error>;
    fn get_users(&self) -> Result<Vec<User>, Error>;
}
