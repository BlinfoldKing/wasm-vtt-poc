use crate::app::App;
use crate::cmd::RunCommand;
use crate::entities::user::CreateUserRequest;
use crate::error::Error;

pub struct SetupRunner;

impl RunCommand<SetupRunner> for App {
    async fn run(&mut self) -> Result<(), Error> {
        let usecase = self.usecase.lock().unwrap();
        if let Some(setting) = usecase.setting.get_setting("initialized")? {
            if setting.value == "true" {
                println!("ouroboros already initialized");
                return Ok(());
            }
        }

        tracing::info!("initializing...");
        tracing::info!("create user...");
        usecase.user.create_user(CreateUserRequest {
            username: "admin".to_string(),
            password: "pass".to_string(),
        })?;
        println!("finalize setting");
        usecase
            .setting
            .set_setting("initialized".to_string(), "true".to_string())?;

        Ok(())
    }
}
