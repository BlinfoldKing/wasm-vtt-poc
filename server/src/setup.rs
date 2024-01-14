use crate::app::App;
use crate::entities::user::CreateUserRequest;
use crate::error::Error;
use crate::repository::sqlite::SqliteRepository;
use crate::usecases::setting::SettingUsecase;
use crate::usecases::user::UserUsecase;

impl App {
    pub fn setup(&self) -> Result<(), Error> {
        let user_usecase = UserUsecase::<SqliteRepository>::new(self.config.clone())?;
        let setting_usecase = SettingUsecase::<SqliteRepository>::new(self.config.clone())?;

        if let Some(setting) = setting_usecase.get_setting("initialized")? {
            if setting.value == "true" {
                println!("ouroboros already initialized");
                return Ok(());
            }
        }

        println!("initializing...");
        println!("create user...");
        user_usecase.create_user(CreateUserRequest {
            username: "admin".to_string(),
            password: "pass".to_string(),
        })?;
        println!("finalize setting");
        setting_usecase.set_setting("initialized".to_string(), "true".to_string())?;

        Ok(())
    }
}
