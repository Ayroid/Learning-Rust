use project_structure::auth_utils::models::Credentials;
use project_structure::authenticate;

fn main() {
    let cred = Credentials {
        username: String::from("ayroid"),
        password: String::from("password"),
    };

    authenticate(cred);
}
