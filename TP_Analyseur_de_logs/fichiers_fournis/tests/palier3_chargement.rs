//! Palier 3 — Chargement d'un journal. Lancer : cargo test --test palier3_chargement
use analyseur_logs::*;

const JOURNAL: &str = "# commentaire
2026-10-06 08:00:01 [INFO] serveur: démarrage

2026-10-06 08:16:40 [ERROR] auth: compte bloqué
ligne cassée
2026-10-06 09:00:00 [FATAL] bdd: panne
";

#[test]
fn charger_texte_separe_valides_et_invalides() {
    let (entrees, erreurs) = charger_texte(JOURNAL);
    assert_eq!(entrees.len(), 2, "commentaire et ligne vide ignorés");
    assert_eq!(entrees[1].module, "auth", "l'ordre du fichier est conservé");
    assert_eq!(
        erreurs,
        vec![
            ErreurLog::FormatInvalide("ligne cassée".into()),
            ErreurLog::FormatInvalide("2026-10-06 09:00:00 [FATAL] bdd: panne".into()),
        ]
    );
}

#[test]
fn charger_texte_vide() {
    let (entrees, erreurs) = charger_texte("");
    assert!(entrees.is_empty() && erreurs.is_empty());
}

#[test]
fn charger_le_fichier_fourni() {
    let (entrees, erreurs) = charger_fichier("logs/serveur.log").unwrap();
    assert_eq!(entrees.len(), 27);
    assert_eq!(erreurs.len(), 4);
}

#[test]
fn fichier_absent() {
    assert_eq!(
        charger_fichier("absent.log").unwrap_err(),
        ErreurLog::Fichier("absent.log".into())
    );
}
