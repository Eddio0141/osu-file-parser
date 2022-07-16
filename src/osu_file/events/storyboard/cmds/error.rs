use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ContinuingSetError {
    #[error("continuing fields index out of bounds")]
    IndexOutOfBounds,
    #[error(
        "continuing fields 2nd field is none without it being the last item in the continuing fields")]
    InvalidSecondFieldOption,
}

#[derive(Debug, Error)]
#[error(
    "continuing fields 2nd field is none without it being the last item in the continuing fields"
)]
pub struct InvalidSecondFieldOption;
