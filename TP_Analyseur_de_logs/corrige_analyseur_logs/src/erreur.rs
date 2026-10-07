//! Erreurs de l'analyseur.

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ErreurLog {
    /// Ligne mal formée (on conserve la ligne d'origine).
    FormatInvalide(String),
    /// Fichier impossible à lire (on conserve le chemin).
    Fichier(String),
}

impl fmt::Display for ErreurLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErreurLog::FormatInvalide(ligne) => write!(f, "ligne invalide : {ligne}"),
            ErreurLog::Fichier(chemin) => write!(f, "impossible de lire le fichier {chemin}"),
        }
    }
}
