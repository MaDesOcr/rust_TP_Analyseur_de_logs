//! Statistiques, filtres et rapport.

use crate::entree::EntreeLog;
use crate::niveau::Niveau;

/// Nombre d'entrées par niveau, dans l'ordre [DEBUG, INFO, WARN, ERROR].
pub fn compter_par_niveau(entrees: &[EntreeLog]) -> [usize; 4] {
    let mut compteurs = [0; 4];
    for e in entrees {
        compteurs[e.niveau.indice()] += 1;
    }
    compteurs
}

/// Copies des entrées dont le niveau est au moins `niveau_min` (ex. Warn → WARN et ERROR).
pub fn filtrer_par_niveau(entrees: &[EntreeLog], niveau_min: Niveau) -> Vec<EntreeLog> {
    let mut resultat = Vec::new();
    for e in entrees {
        if e.niveau >= niveau_min {
            resultat.push(e.clone());
        }
    }
    resultat
}

/// Copies des entrées d'un module (comparaison sans tenir compte des majuscules).
pub fn filtrer_par_module(entrees: &[EntreeLog], module: &str) -> Vec<EntreeLog> {
    let cherche = module.trim().to_lowercase();
    let mut resultat = Vec::new();
    for e in entrees {
        if e.module.to_lowercase() == cherche {
            resultat.push(e.clone());
        }
    }
    resultat
}

/// Copies des entrées dont le message contient `mot` (sans tenir compte des majuscules).
/// Un mot vide ne donne aucun résultat.
pub fn rechercher(entrees: &[EntreeLog], mot: &str) -> Vec<EntreeLog> {
    let cherche = mot.trim().to_lowercase();
    let mut resultat = Vec::new();
    if cherche.is_empty() {
        return resultat;
    }
    for e in entrees {
        if e.message.to_lowercase().contains(&cherche) {
            resultat.push(e.clone());
        }
    }
    resultat
}

/// Les `n` dernières entrées de niveau ERROR, dans l'ordre du fichier.
pub fn dernieres_erreurs(entrees: &[EntreeLog], n: usize) -> Vec<EntreeLog> {
    let erreurs = filtrer_par_niveau(entrees, Niveau::Error);
    let debut = erreurs.len().saturating_sub(n);
    erreurs[debut..].to_vec()
}

/// Nombre d'entrées par module, trié par nombre décroissant,
/// puis par nom de module en cas d'égalité.
pub fn compter_par_module(entrees: &[EntreeLog]) -> Vec<(String, usize)> {
    let mut compteurs: Vec<(String, usize)> = Vec::new();
    for e in entrees {
        let mut trouve = false;
        for (module, n) in compteurs.iter_mut() {
            if *module == e.module {
                *n += 1;
                trouve = true;
                break;
            }
        }
        if !trouve {
            compteurs.push((e.module.clone(), 1));
        }
    }
    compteurs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    compteurs
}

/// Rapport texte complet (voir le sujet pour le format exact).
pub fn generer_rapport(entrees: &[EntreeLog], nb_lignes_invalides: usize) -> String {
    let mut r = String::new();
    r.push_str("=== Rapport d'analyse ===\n");
    r.push_str(&format!("Entrées valides : {}\n", entrees.len()));
    r.push_str(&format!("Lignes invalides : {nb_lignes_invalides}\n"));
    match (entrees.first(), entrees.last()) {
        (Some(premiere), Some(derniere)) => r.push_str(&format!(
            "Période : {} {} -> {} {}\n",
            premiere.date, premiere.heure, derniere.date, derniere.heure
        )),
        _ => r.push_str("Période : aucune entrée\n"),
    }

    r.push_str("\nRépartition par niveau :\n");
    let compteurs = compter_par_niveau(entrees);
    for niveau in Niveau::TOUS {
        r.push_str(&format!("  {} : {}\n", niveau, compteurs[niveau.indice()]));
    }

    r.push_str("\nModules :\n");
    for (module, n) in compter_par_module(entrees) {
        r.push_str(&format!("  {module} : {n}\n"));
    }

    r.push_str("\nDernières erreurs :\n");
    let erreurs = dernieres_erreurs(entrees, 5);
    if erreurs.is_empty() {
        r.push_str("  aucune\n");
    }
    for e in erreurs {
        r.push_str(&format!("  {e}\n"));
    }
    r
}
