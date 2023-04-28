use derive_more::Display;
use error_stack;
use error_stack::Context;

#[derive(Debug, Display)]
pub enum KUploadError {
    KubeconfigNotExist,
    #[display(fmt = "KubeError ({})", _0)]
    KubeError(String),
    UnknownError,
}
impl Context for KUploadError {}

pub type Result<T> = error_stack::Result<T, KUploadError>;
