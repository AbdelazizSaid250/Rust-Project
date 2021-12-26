use std::fmt::{Display, Formatter};
use std::fmt;

use actix_web::ResponseError;
use diesel::result::Error as DieselError;
use paperclip::actix::web::HttpResponse;
use serde::{Deserialize, Serialize};
use validator::{ValidationErrors, ValidationErrorsKind};

#[derive(Debug)]
pub enum Errors {
    BadReq(Vec<ErrorCode>),
    BadRequest(ErrorCode),
    InternalServerError(ErrorCode),
    NotFound(ErrorCode),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrorCode {
    pub error_code: String,
    pub message: String,
}

impl ErrorCode {
    pub fn validate_errors(error: ValidationErrors, errors: &mut Vec<ErrorCode>) {
        for value in error.errors().values() {
            match value {
                ValidationErrorsKind::Struct(_) => {}
                ValidationErrorsKind::List(_) => {}
                ValidationErrorsKind::Field(validation_error_vec) => {
                    for validation_error in validation_error_vec {
                        let error_code = validation_error.clone().code.to_string();
                        let message = validation_error
                            .clone()
                            .message
                            .expect("Validation Error")
                            .to_string();
                        errors.push(ErrorCode {
                            error_code,
                            message,
                        });
                    }
                }
            }
        }
    }
}

impl Display for Errors {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl ResponseError for Errors {
    fn error_response(&self) -> HttpResponse {
        match self {
            Errors::BadReq(errors) => HttpResponse::BadRequest().json(errors),
            Errors::BadRequest(error) => HttpResponse::BadRequest().json(error),
            Errors::NotFound(errors) => HttpResponse::NotFound().json(errors),
            Errors::InternalServerError(errors) => HttpResponse::InternalServerError().json(errors),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum StateCode {
    InternalServerError,
    NotFound,
    DBError,
    PaginationError,
    DuplicationError,
}

impl StateCode {
    pub fn get_code(&self) -> &'static str {
        match self {
            Self::InternalServerError => "internal-server-error",
            Self::NotFound => "not-found",
            Self::DBError => "db-error",
            Self::PaginationError => "pagination-error",
            Self::DuplicationError => "duplication-error",
        }
    }
    pub fn get_message(&self) -> &'static str {
        match self {
            Self::InternalServerError => "Internal server error",
            Self::NotFound => "Cannot find the object in the Server.",
            Self::DBError => "There is an error in dealing with Database.",
            Self::PaginationError => "Paginated Data is not valid.",
            Self::DuplicationError => "The object is duplicated.",
        }
    }
}

impl From<StateCode> for ErrorCode {
    fn from(item: StateCode) -> Self {
        Self {
            error_code: item.get_code().to_string(),
            message: item.get_message().to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    DBError(DieselError),
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    HttpRequest(String),
    DuplicationError,
    DeletedDuplicationError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::DBError(error) => write!(f, "{}", error),
            Error::BadRequest(error) => write!(f, "{}", error),
            Error::InternalServerError(error) => write!(f, "{}", error),
            Error::NotFound(error) => write!(f, "{}", error),
            Error::HttpRequest(error) => write!(f, "{}", error),
            Error::DuplicationError => write!(f, "The object is duplicated"),
            Error::DeletedDuplicationError => write!(f, "The deleted object is duplicated."),
        }
    }
}

impl From<DieselError> for Error {
    fn from(err: DieselError) -> Self {
        Error::DBError(err)
    }
}

impl From<String> for Error {
    fn from(req: String) -> Self {
        Error::HttpRequest(req)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::BadRequest(error) => HttpResponse::BadRequest().json(error),
            Error::NotFound(message) => HttpResponse::NotFound().json(message),
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        }
    }
}
