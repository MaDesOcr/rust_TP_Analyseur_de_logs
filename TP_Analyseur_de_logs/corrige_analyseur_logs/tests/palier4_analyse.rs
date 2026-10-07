//! Palier 4 — Analyse. Lancer : cargo test --test palier4_analyse
use analyseur_logs::*;

fn jeu() -> Vec<EntreeLog> {
    let lignes = [
        "2026-10-06 08:00:00 [INFO] serveur: démarrage",
        "2026-10-06 08:10:00 [WARN] auth: mot de passe invalide",
        "2026-10-06 08:11:00 [ERROR] auth: compte bloqué",
        "2026-10-06 09:00:00 [DEBUG] cache: purge",
        "2026-10-06 09:30:00 [ERROR] bdd: délai dépassé",
        "2026-10-06 10:00:00 [INFO] Auth: Connexion de alice",
        "2026-10-06 11:00:00 [ERROR] api: erreur 500",
    ];
    lignes.iter().map(|l| analyser_ligne(l).unwrap()).collect()
}

fn modules(entrees: &[EntreeLog]) -> Vec<&str> {
    entrees.iter().map(|e| e.module.as_str()).collect()
}

#[test]
fn compte_par_niveau() {
    assert_eq!(compter_par_niveau(&jeu()), [1, 2, 1, 3]);
    assert_eq!(compter_par_niveau(&[]), [0, 0, 0, 0]);
}

#[test]
fn filtre_par_niveau_minimum() {
    let j = jeu();
    assert_eq!(filtrer_par_niveau(&j, Niveau::Warn).len(), 4);
    assert_eq!(filtrer_par_niveau(&j, Niveau::Error).len(), 3);
    assert_eq!(filtrer_par_niveau(&j, Niveau::Debug).len(), 7);
}

#[test]
fn filtre_par_module_sans_casse() {
    let j = jeu();
    assert_eq!(filtrer_par_module(&j, "auth").len(), 3);
    assert_eq!(filtrer_par_module(&j, " AUTH ").len(), 3);
    assert!(filtrer_par_module(&j, "inconnu").is_empty());
}

#[test]
fn recherche_dans_les_messages() {
    let j = jeu();
    assert_eq!(modules(&rechercher(&j, "CONNEXION")), vec!["Auth"]);
    assert_eq!(rechercher(&j, "é").len(), 3);
    assert!(rechercher(&j, "  ").is_empty(), "un mot vide ne donne rien");
}

#[test]
fn dernieres_erreurs_dans_l_ordre() {
    let j = jeu();
    assert_eq!(modules(&dernieres_erreurs(&j, 2)), vec!["bdd", "api"]);
    assert_eq!(
        dernieres_erreurs(&j, 10).len(),
        3,
        "pas plus que le nombre d'erreurs"
    );
    assert!(dernieres_erreurs(&j, 0).is_empty());
}

#[test]
fn compte_par_module_trie() {
    let attendu = vec![
        ("auth".to_string(), 2),
        ("Auth".to_string(), 1),
        ("api".to_string(), 1),
        ("bdd".to_string(), 1),
        ("cache".to_string(), 1),
        ("serveur".to_string(), 1),
    ];
    assert_eq!(compter_par_module(&jeu()), attendu);
}

#[test]
fn rapport_complet() {
    let attendu = "=== Rapport d'analyse ===
Entrées valides : 7
Lignes invalides : 2
Période : 2026-10-06 08:00:00 -> 2026-10-06 11:00:00

Répartition par niveau :
  DEBUG : 1
  INFO : 2
  WARN : 1
  ERROR : 3

Modules :
  auth : 2
  Auth : 1
  api : 1
  bdd : 1
  cache : 1
  serveur : 1

Dernières erreurs :
  2026-10-06 08:11:00 [ERROR] auth: compte bloqué
  2026-10-06 09:30:00 [ERROR] bdd: délai dépassé
  2026-10-06 11:00:00 [ERROR] api: erreur 500
";
    assert_eq!(generer_rapport(&jeu(), 2), attendu);
}

#[test]
fn rapport_vide() {
    let r = generer_rapport(&[], 0);
    assert!(r.contains("Période : aucune entrée"));
    assert!(r.contains("Dernières erreurs :\n  aucune\n"));
}
