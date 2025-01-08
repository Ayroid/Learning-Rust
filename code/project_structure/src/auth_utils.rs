pub fn login(cred: models::Credentials) {
    super::database::get_user()
}

pub mod models {
    pub struct Credentials {
        pub username: String,
        pub password: String,
    }
}
