use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct MatrixConfig {
    pub home_server: String,
    pub login: String,
    pub password: String,
}

impl MatrixConfig {

}