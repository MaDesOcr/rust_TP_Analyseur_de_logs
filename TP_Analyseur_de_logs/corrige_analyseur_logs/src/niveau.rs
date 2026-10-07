//! Niveau de gravité d'une entrée de log.

use std::fmt;

/// Du moins grave au plus grave : l'ordre de déclaration permet les comparaisons (<, >=…).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Niveau {
    Debug,
    Info,
    Warn,
    Error,
}

impl Niveau {
    /// Tous les niveaux, du moins grave au plus grave.
    pub const TOUS: [Niveau; 4] = [Niveau::Debug, Niveau::Info, Niveau::Warn, Niveau::Error];

    /// Reconnaît « DEBUG », « INFO », « WARN » ou « ERROR », sans tenir compte
    /// des majuscules ni des espaces autour. Tout autre texte donne None.
    pub fn depuis_texte(texte: &str) -> Option<Niveau> {
        match texte.trim().to_uppercase().as_str() {
            "DEBUG" => Some(Niveau::Debug),
            "INFO" => Some(Niveau::Info),
            "WARN" => Some(Niveau::Warn),
            "ERROR" => Some(Niveau::Error),
            _ => None,
        }
    }

    /// Libellé en majuscules : « DEBUG », « INFO », « WARN », « ERROR ».
    pub fn libelle(&self) -> &'static str {
        match self {
            Niveau::Debug => "DEBUG",
            Niveau::Info => "INFO",
            Niveau::Warn => "WARN",
            Niveau::Error => "ERROR",
        }
    }

    /// Position dans Niveau::TOUS (0 pour Debug … 3 pour Error).
    pub fn indice(&self) -> usize {
        match self {
            Niveau::Debug => 0,
            Niveau::Info => 1,
            Niveau::Warn => 2,
            Niveau::Error => 3,
        }
    }
}

impl fmt::Display for Niveau {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.libelle())
    }
}
