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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn signed(claims: &Value, secret: &[u8]) -> String {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            claims,
            &EncodingKey::from_secret(secret),
        )
        .unwrap()
    }

    fn future_exp() -> i64 {
        Utc::now().timestamp() + 3600
    }

    #[test]
    fn accepts_valid_token_and_rejects_invalid_tokens() {
        let claims = json!({"sub": "admin", "exp": future_exp()});
        assert_eq!(
            verify_token(&signed(&claims, b"secret")).unwrap().sub,
            "admin"
        );
        assert!(verify_token(&signed(&claims, b"wrong-secret")).is_err());
        assert!(verify_token("not-a-jwt").is_err());
        assert!(matches!(
            verify_token(&signed(
                &json!({"sub": "admin", "exp": Utc::now().timestamp() - 3600}),
                b"secret"
            )),
            Err(AuthError::TokenExpired)
        ));
        for claims in [
            json!({"sub": "admin"}),
            json!({"sub": "admin", "exp": "99999999999"}),
            json!({"sub": "admin", "exp": {"value": future_exp()}}),
        ] {
            assert!(verify_token(&signed(&claims, b"secret")).is_err());
        }
    }

    // CVE-2026-25537: enabled validation must reject a malformed optional claim.
    // Decode into Value so Claims' strict deserializer cannot mask the library bug.
    #[test]
    fn rejects_malformed_optional_time_claims() {
        for claim in ["nbf", "exp"] {
            let mut validation = Validation::new(Algorithm::HS256);
            validation.validate_nbf = true;
            validation.required_spec_claims.clear();

            let decode_value = |claims: &Value| {
                decode::<Value>(
                    signed(claims, b"secret"),
                    &DecodingKey::from_secret(b"secret"),
                    &validation,
                )
            };
            assert!(decode_value(&json!({"sub": "admin"})).is_ok());
            assert!(decode_value(&json!({
                "sub": "admin", "exp": future_exp(), "nbf": Utc::now().timestamp() - 60
            }))
            .is_ok());

            for malformed in [json!("99999999999"), json!({"value": 123}), json!([123])] {
                let mut claims = json!({"sub": "admin", "exp": future_exp()});
                claims[claim] = malformed;
                assert!(decode_value(&claims).is_err(), "accepted malformed {claim}");
            }
        }
    }

    #[actix_web::test]
    async fn login_and_bearer_middleware_round_trip() {
        use actix_web::{http::StatusCode, test};

        let app = test::init_service(
            App::new().route("/login", web::post().to(login)).service(
                web::scope("/protected")
                    .wrap(HttpAuthentication::bearer(validator))
                    .route("", web::get().to(protected)),
            ),
        )
        .await;
        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(json!({"username": "admin", "password": "password"}))
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        let token = body["token"].as_str().unwrap();
        let request = test::TestRequest::get()
            .uri("/protected")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();
        assert_eq!(
            test::call_service(&app, request).await.status(),
            StatusCode::OK
        );

        let request = test::TestRequest::get().uri("/protected").to_request();
        assert_eq!(
            test::call_service(&app, request).await.status(),
            StatusCode::UNAUTHORIZED
        );
        let bad_token = signed(&json!({"sub": "admin", "exp": future_exp()}), b"wrong");
        let request = test::TestRequest::get()
            .uri("/protected")
            .insert_header(("Authorization", format!("Bearer {bad_token}")))
            .to_request();
        assert_eq!(
            test::call_service(&app, request).await.status(),
            StatusCode::UNAUTHORIZED
        );

        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(json!({"username": "admin", "password": "wrong"}))
            .to_request();
        assert_eq!(
            test::call_service(&app, request).await.status(),
            StatusCode::UNAUTHORIZED
        );
    }
}
