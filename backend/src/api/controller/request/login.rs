use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginForm {
    #[serde(default)]
    pub user_name: String,
    pub password: String,
}

impl LoginForm {
    pub fn validate(&self) -> Result<(), String> {
        if self.user_name.is_empty() {
            return Err("user_name is empty".to_string())
        }
        if self.password.is_empty() {
            return Err("password is empty".to_string())
        }
        Ok(())
    }
}