use thiserror::Error;
use std::env;

// create a new error that wraps the vauth::LogInError

#[derive(Error, Debug)]
pub enum AuditorError {
    #[error("The VEEAM_API_PASSWORD environmental variable is missing")]
    EnvError(#[from] env::VarError),
    #[error("IP Address is not valid")]
    IpAddressError,
    #[error("Username cannot be empty")]
    UsernameEmpty,
    #[error("Password cannot be empty")]
    PasswordEmpty,
    #[error("IP Address cannot be empty")]
    IpAddressEmpty,
    #[error("No refresh token")]
    NoRefreshToken,
    #[error("Error in sending request `{0:?}`")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Status Code Error `{0}`")]
    StatusCodeError(reqwest::StatusCode),
    #[error("Error in parsing response `{0:?}`")]
    SerdeError(#[from] serde_json::Error),
    #[error("Anyhow Error `{0:?}`")]
    AnyhowError(#[from] anyhow::Error)
}
