use ed25519_dalek::SignatureError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SessionIdError {
    #[error("signature error: {0:?} {1}:{2}")]
    Signature(SignatureError, String, u32),
    #[error("convert error: {0}:{1}")]
    Convert(String, u32),
    #[error("required error: {0}:{1}")]
    Required(String, u32),
}

#[macro_export]
macro_rules! signature {
    () => {
        |v| errors::SessionIdError::Signature(v, file!().to_string(), line!())
    };
}

#[macro_export]
macro_rules! required {
    () => {
        || errors::SessionIdError::Required(file!().to_string(), line!())
    };
}

#[macro_export]
macro_rules! convert {
    () => {
        anyhow::Error::from(errors::SessionIdError::Convert(
            file!().to_string(),
            line!(),
        ))
    };
    ( $v:expr ) => {
        anyhow::anyhow!(errors::SessionIdError::Convert(
            format!("{},   {}", $v, file!().to_string()),
            line!()
        ))
    };
}
pub(crate) use convert;
pub(crate) use required;
pub(crate) use signature;
