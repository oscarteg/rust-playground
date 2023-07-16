use crate::jwt::error::Error::WrongCredentialsError;
use auth::{with_auth, Role};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use warp::{reject, reply, Filter, Rejection, Reply};

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;
type Users = Arc<HashMap<String, User>>;

#[derive(Clone)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub pw: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub pw: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[tokio::main]
async fn main() {
    let users = Arc::new(init_users());

    let login_route = warp::path!("login")
        .and(warp::post())
        .and(with_users(users.clone()))
        .and(warp::body::json())
        .and_then(login_handler);

    let user_route = warp::path!("user")
        .and(with_auth(Role::User))
        .and_then(user_handler);
    let admin_route = warp::path!("admin")
        .and(with_auth(Role::Admin))
        .and_then(admin_handler);

    let routes = login_route
        .or(user_route)
        .or(admin_route)
        .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    warp::any().map(move || users.clone())
}

pub async fn login_handler(users: Users, body: LoginRequest) -> WebResult<impl Reply> {
    match users
        .iter()
        .find(|(_uid, user)| user.email == body.email && user.pw == body.pw)
    {
        Some((uid, user)) => {
            let token =
                auth::create_jwt(uid, &Role::from_str(&user.role)).map_err(reject::custom)?;
            Ok(reply::json(&LoginResponse { token }))
        }
        None => Err(reject::custom(WrongCredentialsError)),
    }
}

pub async fn user_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello User {}", uid))
}

pub async fn admin_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello Admin {}", uid))
}

fn init_users() -> HashMap<String, User> {
    let mut map = HashMap::new();
    map.insert(
        String::from("1"),
        User {
            uid: String::from("1"),
            email: String::from("user@userland.com"),
            pw: String::from("1234"),
            role: String::from("User"),
        },
    );

    map.insert(
        String::from("2"),
        User {
            uid: String::from("2"),
            email: String::from("admin@adminaty.com"),
            pw: String::from("4321"),
            role: String::from("Admin"),
        },
    );
    map
}

mod error {
    use serde::Serialize;
    use std::convert::Infallible;
    use thiserror::Error;
    use warp::{http::StatusCode, Rejection, Reply};

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("wrong credentials")]
        WrongCredentialsError,
        #[error("jwt token not valid")]
        JWTTokenError,
        #[error("jwt token creation error")]
        JWTTokenCreationError,
        #[error("no auth header")]
        NoAuthHeaderError,
        #[error("invalid auth header")]
        InvalidAuthHeaderError,
        #[error("no permission")]
        NoPermissionError,
    }

    #[derive(Serialize, Debug)]
    struct ErrorResponse {
        message: String,
        status: String,
    }

    impl warp::reject::Reject for Error {}

    pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
        let (code, message) = if err.is_not_found() {
            (StatusCode::NOT_FOUND, "Not Found".to_string())
        } else if let Some(e) = err.find::<Error>() {
            match e {
                Error::WrongCredentialsError => (StatusCode::FORBIDDEN, e.to_string()),
                Error::NoPermissionError => (StatusCode::UNAUTHORIZED, e.to_string()),
                Error::JWTTokenError => (StatusCode::UNAUTHORIZED, e.to_string()),
                Error::JWTTokenCreationError => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                ),
                _ => (StatusCode::BAD_REQUEST, e.to_string()),
            }
        } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
            (
                StatusCode::METHOD_NOT_ALLOWED,
                "Method Not Allowed".to_string(),
            )
        } else {
            eprintln!("unhandled error: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            )
        };

        let json = warp::reply::json(&ErrorResponse {
            status: code.to_string(),
            message,
        });

        Ok(warp::reply::with_status(json, code))
    }
}

mod auth {
    use std::fmt;

    use crate::jwt::error::Error;
    use crate::jwt::{Result, WebResult};
    use chrono::prelude::*;
    use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
    use serde::{Deserialize, Serialize};
    use warp::{
        filters::header::headers_cloned,
        http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
        reject, Filter, Rejection,
    };

    const BEARER: &str = "Bearer ";
    const JWT_SECRET: &[u8] = b"secret";

    #[derive(Clone, PartialEq)]
    pub enum Role {
        User,
        Admin,
    }

    impl Role {
        pub fn from_str(role: &str) -> Role {
            match role {
                "Admin" => Role::Admin,
                _ => Role::User,
            }
        }
    }

    impl fmt::Display for Role {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Role::User => write!(f, "User"),
                Role::Admin => write!(f, "Admin"),
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Claims {
        sub: String,
        role: String,
        exp: usize,
    }

    pub fn with_auth(role: Role) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
        headers_cloned()
            .map(move |headers: HeaderMap<HeaderValue>| (role.clone(), headers))
            .and_then(authorize)
    }

    pub fn create_jwt(uid: &str, role: &Role) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(60))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: uid.to_owned(),
            role: role.to_string(),
            exp: expiration as usize,
        };
        let header = Header::new(Algorithm::HS512);
        encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
            .map_err(|_| Error::JWTTokenCreationError)
    }

    async fn authorize((role, headers): (Role, HeaderMap<HeaderValue>)) -> WebResult<String> {
        match jwt_from_header(&headers) {
            Ok(jwt) => {
                let decoded = decode::<Claims>(
                    &jwt,
                    &DecodingKey::from_secret(JWT_SECRET),
                    &Validation::new(Algorithm::HS512),
                )
                .map_err(|_| reject::custom(Error::JWTTokenError))?;

                if role == Role::Admin && Role::from_str(&decoded.claims.role) != Role::Admin {
                    return Err(reject::custom(Error::NoPermissionError));
                }

                Ok(decoded.claims.sub)
            }
            Err(e) => Err(reject::custom(e)),
        }
    }

    fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String> {
        let header = match headers.get(AUTHORIZATION) {
            Some(v) => v,
            None => return Err(Error::NoAuthHeaderError),
        };
        let auth_header = match std::str::from_utf8(header.as_bytes()) {
            Ok(v) => v,
            Err(_) => return Err(Error::NoAuthHeaderError),
        };
        if !auth_header.starts_with(BEARER) {
            return Err(Error::InvalidAuthHeaderError);
        }
        Ok(auth_header.trim_start_matches(BEARER).to_owned())
    }
}
