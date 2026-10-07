//! Analyseur de logs — programme principal avec menu interactif.
//!
//! Utilisation : cargo run -- [chemin du fichier de logs]
//! Sans argument, le fichier logs/serveur.log est chargé.

use analyseur_logs::*;
use std::env;
use std::fs;
use std::io::{self, Write};

const FICHIER_PAR_DEFAUT: &str = "logs/serveur.log";

/// État du programme : le journal chargé.
struct Session {
    chemin: String,
    entrees: Vec<EntreeLog>,
    erreurs: Vec<ErreurLog>,
}

/// Affiche une invite et lit une ligne au clavier.
/// Renvoie None en fin d'entrée (Ctrl+Z puis Entrée sous Windows, Ctrl+D sous Linux).
fn demander(invite: &str) -> Option<String> {
    print!("{invite}");
    io::stdout().flush().expect("affichage impossible");
    let mut ligne = String::new();
    match io::stdin().read_line(&mut ligne) {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(ligne.trim().to_string()),
    }
}

/// Charge un fichier ; en cas d'échec, affiche l'erreur et renvoie None.
fn charger(chemin: &str) -> Option<Session> {
    match charger_fichier(chemin) {
        Ok((entrees, erreurs)) => {
            println!(
                "Fichier {chemin} chargé : {} entrées valides, {} lignes invalides.",
                entrees.len(),
                erreurs.len()
            );
            Some(Session {
                chemin: chemin.to_string(),
                entrees,
                erreurs,
            })
        }
        Err(e) => {
            println!("Erreur : {e}");
            None
        }
    }
}

/// Affiche une liste d'entrées, ou « Aucun résultat. ».
fn afficher(entrees: &[EntreeLog]) {
    if entrees.is_empty() {
        println!("Aucun résultat.");
        return;
    }
    for e in entrees {
        println!("  {e}");
    }
    println!("({} résultat(s))", entrees.len());
}

fn afficher_menu(session: &Option<Session>) {
    println!();
    println!("===== Analyseur de logs =====");
    match session {
        Some(s) => println!("Fichier : {} ({} entrées)", s.chemin, s.entrees.len()),
        None => println!("Aucun fichier chargé"),
    }
    println!("1. Charger un fichier");
    println!("2. Statistiques");
    println!("3. Filtrer par niveau minimum");
    println!("4. Filtrer par module");
    println!("5. Rechercher un mot dans les messages");
    println!("6. Dernières erreurs");
    println!("7. Afficher les lignes invalides");
    println!("8. Exporter le rapport dans un fichier");
    println!("0. Quitter");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let chemin_initial = match args.get(1) {
        Some(c) => c.clone(),
        None => String::from(FICHIER_PAR_DEFAUT),
    };
    let mut session = charger(&chemin_initial);

    loop {
        afficher_menu(&session);
        let choix = match demander("Votre choix : ") {
            Some(c) => c,
            None => break,
        };

        if choix == "0" {
            break;
        }
        if choix == "1" {
            let chemin = demander("Chemin du fichier : ").unwrap_or_default();
            if let Some(nouvelle) = charger(&chemin) {
                session = Some(nouvelle);
            }
            continue;
        }

        // Toutes les autres options exigent un fichier chargé.
        let s = match &session {
            Some(s) => s,
            None => {
                if ["2", "3", "4", "5", "6", "7", "8"].contains(&choix.as_str()) {
                    println!("Chargez d'abord un fichier (option 1).");
                } else {
                    println!("Choix invalide : « {choix} ».");
                }
                continue;
            }
        };

        match choix.as_str() {
            "2" => print!("{}", generer_rapport(&s.entrees, s.erreurs.len())),
            "3" => {
                let texte =
                    demander("Niveau minimum (DEBUG, INFO, WARN, ERROR) : ").unwrap_or_default();
                match Niveau::depuis_texte(&texte) {
                    Some(niveau) => afficher(&filtrer_par_niveau(&s.entrees, niveau)),
                    None => println!("Niveau inconnu : « {texte} »."),
                }
            }
            "4" => {
                let module = demander("Module : ").unwrap_or_default();
                afficher(&filtrer_par_module(&s.entrees, &module));
            }
            "5" => {
                let mot = demander("Mot recherché : ").unwrap_or_default();
                afficher(&rechercher(&s.entrees, &mot));
            }
            "6" => {
                let texte = demander("Combien d'erreurs afficher ? [5] : ").unwrap_or_default();
                let n = if texte.is_empty() {
                    Ok(5)
                } else {
                    texte.parse::<usize>()
                };
                match n {
                    Ok(n) => afficher(&dernieres_erreurs(&s.entrees, n)),
                    Err(_) => println!("Nombre invalide : « {texte} »."),
                }
            }
            "7" => {
                if s.erreurs.is_empty() {
                    println!("Aucune ligne invalide.");
                }
                for e in &s.erreurs {
                    println!("  {e}");
                }
            }
            "8" => {
                let destination =
                    demander("Fichier de sortie [rapport.txt] : ").unwrap_or_default();
                let destination = if destination.is_empty() {
                    String::from("rapport.txt")
                } else {
                    destination
                };
                let rapport = generer_rapport(&s.entrees, s.erreurs.len());
                match fs::write(&destination, rapport) {
                    Ok(()) => println!("Rapport écrit dans {destination}."),
                    Err(e) => println!("Écriture impossible : {e}"),
                }
            }
            _ => println!("Choix invalide : « {choix} »."),
        }
    }
    println!("Au revoir !");
}
