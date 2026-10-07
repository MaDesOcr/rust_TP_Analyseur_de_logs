//! Palier 1 — Le niveau de gravité. Lancer : cargo test --test palier1_niveau
use analyseur_logs::*;

#[test]
fn depuis_texte_reconnait_les_quatre_niveaux() {
    assert_eq!(Niveau::depuis_texte("DEBUG"), Some(Niveau::Debug));
    assert_eq!(Niveau::depuis_texte("INFO"), Some(Niveau::Info));
    assert_eq!(Niveau::depuis_texte("WARN"), Some(Niveau::Warn));
    assert_eq!(Niveau::depuis_texte("ERROR"), Some(Niveau::Error));
}

#[test]
fn depuis_texte_ignore_casse_et_espaces() {
    assert_eq!(Niveau::depuis_texte(" warn "), Some(Niveau::Warn));
    assert_eq!(Niveau::depuis_texte("Error"), Some(Niveau::Error));
}

#[test]
fn depuis_texte_refuse_le_reste() {
    assert_eq!(Niveau::depuis_texte("CRITICAL"), None);
    assert_eq!(Niveau::depuis_texte(""), None);
}

#[test]
fn libelle_et_affichage() {
    assert_eq!(Niveau::Warn.libelle(), "WARN");
    assert_eq!(Niveau::Error.to_string(), "ERROR");
    assert_eq!(format!("[{}]", Niveau::Info), "[INFO]");
}

#[test]
fn indice() {
    assert_eq!(Niveau::Debug.indice(), 0);
    assert_eq!(Niveau::Error.indice(), 3);
    for (i, n) in Niveau::TOUS.iter().enumerate() {
        assert_eq!(n.indice(), i);
    }
}

#[test]
fn les_niveaux_sont_ordonnes() {
    assert!(Niveau::Debug < Niveau::Info);
    assert!(Niveau::Warn < Niveau::Error);
    assert!(Niveau::Error >= Niveau::Warn);
}
