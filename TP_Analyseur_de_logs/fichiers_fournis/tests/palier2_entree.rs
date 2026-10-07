//! Palier 2 — Analyse d'une ligne. Lancer : cargo test --test palier2_entree
use analyseur_logs::*;

fn invalide(ligne: &str) -> Result<EntreeLog, ErreurLog> {
    Err(ErreurLog::FormatInvalide(ligne.to_string()))
}

#[test]
fn ligne_valide() {
    let e = analyser_ligne("2026-10-06 08:16:40 [ERROR] auth: compte bob bloqué").unwrap();
    assert_eq!(e.date, "2026-10-06");
    assert_eq!(e.heure, "08:16:40");
    assert_eq!(e.niveau, Niveau::Error);
    assert_eq!(e.module, "auth");
    assert_eq!(e.message, "compte bob bloqué");
}

#[test]
fn espaces_autour_de_la_ligne_ignores() {
    let e = analyser_ligne("   2026-10-06 09:00:00 [info] api: GET /  ").unwrap();
    assert_eq!(e.niveau, Niveau::Info);
    assert_eq!(e.message, "GET /");
}

#[test]
fn le_message_peut_contenir_deux_points() {
    let e = analyser_ligne("2026-10-06 09:00:00 [WARN] api: délai : 1840 ms").unwrap();
    assert_eq!(e.module, "api");
    assert_eq!(e.message, "délai : 1840 ms");
}

#[test]
fn affichage_dans_le_format_d_origine() {
    let ligne = "2026-10-06 08:00:01 [INFO] serveur: démarrage";
    assert_eq!(analyser_ligne(ligne).unwrap().to_string(), ligne);
}

#[test]
fn date_ou_heure_invalide() {
    let l = "06/10/2026 10:00:00 [INFO] api: test";
    assert_eq!(analyser_ligne(l), invalide(l));
    let l = "2026-10-06 10h00 [INFO] api: test";
    assert_eq!(analyser_ligne(l), invalide(l));
}

#[test]
fn niveau_invalide() {
    let l = "2026-10-06 10:30:00 [CRITICAL] disque: plein";
    assert_eq!(analyser_ligne(l), invalide(l));
    let l = "2026-10-06 10:30:00 INFO disque: plein";
    assert_eq!(
        analyser_ligne(l),
        invalide(l),
        "le niveau doit être entre crochets"
    );
}

#[test]
fn module_ou_message_invalide() {
    let l = "2026-10-06 11:00:00 [INFO] api GET /sante 200";
    assert_eq!(
        analyser_ligne(l),
        invalide(l),
        "il manque « : » après le module"
    );
    let l = "2026-10-06 11:00:00 [INFO] api:";
    assert_eq!(analyser_ligne(l), invalide(l), "message vide");
}

#[test]
fn ligne_trop_courte() {
    assert_eq!(analyser_ligne("bonjour"), invalide("bonjour"));
    assert_eq!(analyser_ligne(""), invalide(""));
}

#[test]
fn affichage_des_erreurs() {
    assert_eq!(
        ErreurLog::FormatInvalide("xyz".into()).to_string(),
        "ligne invalide : xyz"
    );
    assert_eq!(
        ErreurLog::Fichier("a.log".into()).to_string(),
        "impossible de lire le fichier a.log"
    );
}
