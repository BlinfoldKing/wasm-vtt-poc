use serde::Serialize;

#[derive(Serialize)]
pub struct Ping<'a> {
    pub ping: &'a str,
}
