//! Chargement d'un journal (texte ou fichier).

use crate::entree::{EntreeLog, analyser_ligne};
use crate::erreur::ErreurLog;
use std::fs;

/// Analyse un contenu complet. Les lignes vides et les commentaires (commençant par « # »)
/// sont ignorés. Les lignes invalides ne bloquent pas le chargement : leurs erreurs
/// sont renvoyées à côté des entrées valides, dans l'ordre du fichier.
pub fn charger_texte(contenu: &str) -> (Vec<EntreeLog>, Vec<ErreurLog>) {
    let mut entrees = Vec::new();
    let mut erreurs = Vec::new();
    for ligne in contenu.lines() {
        let ligne = ligne.trim();
        if ligne.is_empty() || ligne.starts_with('#') {
            continue;
        }
        match analyser_ligne(ligne) {
            Ok(entree) => entrees.push(entree),
            Err(e) => erreurs.push(e),
        }
    }
    (entrees, erreurs)
}

/// Lit le fichier puis l'analyse avec `charger_texte`.
pub fn charger_fichier(chemin: &str) -> Result<(Vec<EntreeLog>, Vec<ErreurLog>), ErreurLog> {
    let contenu = fs::read_to_string(chemin).map_err(|_| ErreurLog::Fichier(chemin.to_string()))?;
    Ok(charger_texte(&contenu))
}
