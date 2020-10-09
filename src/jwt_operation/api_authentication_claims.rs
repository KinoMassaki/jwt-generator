use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApiAuthenticationClaims {
    sub: String,
    iat: String,
    roles: Vec<String>
}

impl ApiAuthenticationClaims {
    pub fn new(sub: String, iat: String, roles: &Vec<String>) -> Self {
        let mut claims = ApiAuthenticationClaims::default();
        claims.sub = sub;
        claims.iat = iat;

        macro_rules! roles {
            ($role: expr) => {{
                claims.roles.push($role)
            }}
        }

        for role in roles {
            roles!(role.to_string());
        }

        claims
    }
    /*pub fn new(sub: String, iat: String, path_to_roles_file: String) -> Self {
        let mut claims = ApiAuthenticationClaims::default();
        claims.sub = sub;
        claims.iat = iat;

        macro_rules! roles {
            ($role: expr) => {{
                claims.roles.push($role)
            }}
        }

        let imported_roles = read_files::get_key_from_file(path_to_roles_file);

        for role in imported_roles {
            roles!(role.to_string());
        }

        claims
    }*/
}
