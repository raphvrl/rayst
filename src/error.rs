use std::fmt;

#[derive(Debug)]
pub enum RaystError {
    InvalidInput(String),
    RenderError(String),
    IoError(std::io::Error),
    ImageError(image::ImageError),
}

impl fmt::Display for RaystError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RaystError::InvalidInput(msg) => write!(f, "Entrée invalide: {}", msg),
            RaystError::RenderError(msg) => write!(f, "Erreur de rendu: {}", msg),
            RaystError::IoError(err) => write!(f, "Erreur fichier: {}", err),
            RaystError::ImageError(err) => write!(f, "Erreur image: {}", err),
        }
    }
}

impl std::error::Error for RaystError {}

impl From<std::io::Error> for RaystError {
    fn from(err: std::io::Error) -> Self {
        RaystError::IoError(err)
    }
}

impl From<image::ImageError> for RaystError {
    fn from(err: image::ImageError) -> Self {
        RaystError::ImageError(err)
    }
}

pub type Result<T> = std::result::Result<T, RaystError>;
