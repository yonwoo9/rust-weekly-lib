use core::str;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpResponse, HttpServer};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
use chrono::{Duration, Utc};
use error::AuthError;
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
mod error;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

async fn login(login: web::Json<LoginRequest>) -> Result<HttpResponse, AuthError> {
    // 1. 检查用户提供的凭证是否正确
    if login.username != "admin" || login.password != "password" {
        return Err(AuthError::InvalidCredentials);
    }

    // 2. 创建token
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: login.username.clone(),
        exp: expiration as usize,
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .map_err(AuthError::TokenCreationError)?;

    // 3. 返回token
    Ok(HttpResponse::Ok().json(serde_json::json!({"token": token})))
}

async fn protected(_token: String) -> Result<HttpResponse, AuthError> {
    Ok(HttpResponse::Ok().json("this is a protected route"))
}

fn verify_token(token: &str) -> Result<Claims, AuthError> {
    // jwt token 验证
    let validation = Validation::new(Algorithm::HS256);
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &validation,
    ) {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => match err.kind() {
            // 不同的错误类型
            jsonwebtoken::errors::ErrorKind::InvalidToken => Err(AuthError::InvalidTokenFormat),
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => Err(AuthError::TokenExpired),
            _ => Err(AuthError::TokenValidationError(err)),
        },
    }
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    match verify_token(token) {
        Ok(_claims) => Ok(req),
        Err(auth_error) => {
            let error = Error::from(auth_error);
            Err((error, req))
        }
    }
}

// 1、get token
// curl -XPOST http://localhost:8080/login -d '{"username": "admin", "password": "password"}' -H 'Content-Type: application/json'
// 2、use token
// curl http://localhost:8080/protected -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbiIsImV4cCI6MTcyMjI1MTU5Nn0.BFv88IDx-FWWH00KbYQiAgISbltxupfcWDbtShWSmto' -H 'Content-Type: application/json'

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let auth = HttpAuthentication::bearer(validator);
        App::new().route("/login", web::post().to(login)).service(
            web::scope("/protected")
                .wrap(auth)
                .route("", web::get().to(protected)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
