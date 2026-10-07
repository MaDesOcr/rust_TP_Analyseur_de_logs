//! Une entrée de log et son analyse.

use crate::erreur::ErreurLog;
use crate::niveau::Niveau;
use std::fmt;

/// Une ligne de log analysée.
/// Format : « AAAA-MM-JJ HH:MM:SS [NIVEAU] module: message »
#[derive(Debug, Clone, PartialEq)]
pub struct EntreeLog {
    pub date: String,
    pub heure: String,
    pub niveau: Niveau,
    pub module: String,
    pub message: String,
}

/// Affiche l'entrée dans le format d'origine.
impl fmt::Display for EntreeLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} [{}] {}: {}",
            self.date, self.heure, self.niveau, self.module, self.message
        )
    }
}

/// Vrai si `texte` a exactement la forme du modèle, où « 9 » représente un chiffre
/// et tout autre caractère doit être présent tel quel. Ex. : modele = "9999-99-99".
fn respecte_modele(texte: &str, modele: &str) -> bool {
    if texte.len() != modele.len() {
        return false;
    }
    for (c, m) in texte.chars().zip(modele.chars()) {
        let ok = if m == '9' { c.is_ascii_digit() } else { c == m };
        if !ok {
            return false;
        }
    }
    true
}

/// Analyse une ligne de log. Toute ligne qui ne respecte pas le format
/// donne ErreurLog::FormatInvalide avec la ligne d'origine (sans espaces autour).
pub fn analyser_ligne(ligne: &str) -> Result<EntreeLog, ErreurLog> {
    let ligne = ligne.trim();
    let invalide = ErreurLog::FormatInvalide(ligne.to_string());

    let morceaux: Vec<&str> = ligne.splitn(4, ' ').collect();
    if morceaux.len() != 4 {
        return Err(invalide);
    }
    let (date, heure, crochets, reste) = (morceaux[0], morceaux[1], morceaux[2], morceaux[3]);

    if !respecte_modele(date, "9999-99-99") || !respecte_modele(heure, "99:99:99") {
        return Err(invalide);
    }

    if !crochets.starts_with('[') || !crochets.ends_with(']') || crochets.len() < 3 {
        return Err(invalide);
    }
    let niveau = match Niveau::depuis_texte(&crochets[1..crochets.len() - 1]) {
        Some(n) => n,
        None => return Err(invalide),
    };

    let (module, message) = match reste.split_once(": ") {
        Some((m, msg)) => (m.trim(), msg.trim()),
        None => return Err(invalide),
    };
    if module.is_empty() || module.contains(' ') || message.is_empty() {
        return Err(invalide);
    }

    Ok(EntreeLog {
        date: date.to_string(),
        heure: heure.to_string(),
        niveau,
        module: module.to_string(),
        message: message.to_string(),
    })
}
