use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum AuthError {
    #[display(fmt = "Invalid credentials")]
    InvalidCredentials, // 无效的凭证

    #[display(fmt = "Token expired")]
    TokenExpired, // token 过期

    #[display(fmt = "Invalid token format")]
    InvalidTokenFormat, // 无效的 token 格式

    #[allow(dead_code)] // 保留 暂未使用
    #[display(fmt = "Unauthorized")]
    Unauthorized, // 未授权

    #[display(fmt = "Token creation error")]
    TokenCreationError(#[error(not(source))] jsonwebtoken::errors::Error), // token 创建错误

    #[display(fmt = "Token validation error")]
    TokenValidationError(#[error(not(source))] jsonwebtoken::errors::Error), // token 验证错误

    #[allow(dead_code)] // 保留 暂未使用
    #[display(fmt = "Internal server error")]
    InternalServerError, // 内部服务器错误
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::InvalidCredentials => {
                HttpResponse::Unauthorized().json("Invalid credentials")
            }
            AuthError::TokenExpired => HttpResponse::Unauthorized().json("Token expired"),
            AuthError::InvalidTokenFormat => {
                HttpResponse::BadRequest().json("Invalid token format")
            }
            AuthError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            AuthError::TokenCreationError(_) => {
                HttpResponse::InternalServerError().json("Could not create token")
            }
            AuthError::TokenValidationError(_) => {
                HttpResponse::Unauthorized().json("Invalid token")
            }
            AuthError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AuthError::InvalidCredentials => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthError::TokenExpired => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthError::InvalidTokenFormat => actix_web::http::StatusCode::BAD_REQUEST,
            AuthError::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthError::TokenCreationError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::TokenValidationError(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthError::InternalServerError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
