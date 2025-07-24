use std::io;
use std::fs;
use chrono::Utc;

// Structure pour représenter un fichier
struct Fichier {
    nom: String,
    contenu: String,
}

// Implémentation des méthodes pour la structure Fichier
impl Fichier {
    // Afficher les informations du fichier
    fn afficher(&self) {
        println!("Fichier: {}", self.nom);
        println!("Contenu: {}", self.contenu);
        let maintenant = Utc::now();
        println!("Consulté le: {}", maintenant.format("%d/%m/%Y %H:%M:%S"));
        println!("-------------------");
    }

    // Modifier le contenu du fichier
    fn modifier(&mut self, nouveau_contenu: String) {
        self.contenu = nouveau_contenu;
        println!("Fichier '{}' modifié avec succès!", self.nom);
    }

    // Écrire le fichier sur le disque
    fn ecrire_sur_disque(&self) {
        match fs::write(&self.nom, &self.contenu) {
            Ok(_) => println!("Fichier '{}' sauvegardé sur disque!", self.nom),
            Err(_) => println!("Erreur lors de la sauvegarde du fichier '{}'", self.nom),
        }
    }
}

// Structure pour gérer une collection de fichiers
struct GestionnaireFichiers {
    fichiers: Vec<Fichier>,
}

impl GestionnaireFichiers {
    // Créer un nouveau gestionnaire
    fn nouveau() -> GestionnaireFichiers {
        GestionnaireFichiers {
            fichiers: Vec::new(),
        }
    }

    // Ajouter un fichier
    fn ajouter_fichier(&mut self, nom: String, contenu: String) {
        let fichier = Fichier { nom, contenu };
        self.fichiers.push(fichier);
        println!("Fichier ajouté avec succès!");
    }

    // Lister tous les fichiers
    fn lister_fichiers(&self) {
        if self.fichiers.len() == 0 {
            println!("Aucun fichier dans le gestionnaire.");
        } else {
            println!("Liste des fichiers:");
            for (i, fichier) in self.fichiers.iter().enumerate() {
                println!("{}. {}", i + 1, fichier.nom);
            }
        }
    }

    // Afficher un fichier spécifique
    fn afficher_fichier(&self, index: usize) {
        if index < self.fichiers.len() {
            self.fichiers[index].afficher();
        } else {
            println!("Fichier non trouvé!");
        }
    }

    // Modifier un fichier
    fn modifier_fichier(&mut self, index: usize, nouveau_contenu: String) {
        if index < self.fichiers.len() {
            self.fichiers[index].modifier(nouveau_contenu);
        } else {
            println!("Fichier non trouvé!");
        }
    }

    // Supprimer un fichier
    fn supprimer_fichier(&mut self, index: usize) {
        if index < self.fichiers.len() {
            let fichier_supprime = self.fichiers.remove(index);
            println!("Fichier '{}' supprimé définitivement!", fichier_supprime.nom);
        } else {
            println!("Fichier non trouvé!");
        }
    }

    // Sauvegarder tous les fichiers sur disque
    fn sauvegarder_tous(&self) {
        for fichier in &self.fichiers {
            fichier.ecrire_sur_disque();
        }
    }

    // Lire un fichier depuis le disque
    fn lire_depuis_disque(&mut self, nom_fichier: String) {
        match fs::read_to_string(&nom_fichier) {
            Ok(contenu) => {
                self.ajouter_fichier(nom_fichier, contenu);
                println!("Fichier lu et ajouté avec succès!");
            }
            Err(_) => println!("Erreur: impossible de lire le fichier '{}'", nom_fichier),
        }
    }
}

fn main() {
    let mut gestionnaire = GestionnaireFichiers::nouveau();

    println!("=== GESTIONNAIRE DE FICHIERS ===");
    let maintenant = Utc::now();
    println!("Démarré le: {}", maintenant.format("%d/%m/%Y %H:%M:%S"));
    println!();

    loop {
        // Afficher le menu
        let options = [
            "Créer un nouveau fichier",
            "Lister tous les fichiers",
            "Afficher un fichier",
            "Modifier un fichier",
            "Supprimer un fichier",
            "Lire un fichier depuis le disque",
            "Sauvegarder tous les fichiers",
            "Quitter"
        ];

        println!("Menu:");
        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        println!("Veuillez saisir votre choix:");
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");

        let choix: usize = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez saisir un numéro valide!");
                continue;
            }
        };

        match choix {
            1 => {
                // Créer un nouveau fichier
                println!("Nom du fichier:");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).expect("Erreur de lecture");
                let nom = nom.trim().to_string();

                println!("Contenu du fichier:");
                let mut contenu = String::new();
                io::stdin().read_line(&mut contenu).expect("Erreur de lecture");
                let contenu = contenu.trim().to_string();

                gestionnaire.ajouter_fichier(nom, contenu);
            }
            2 => {
                // Lister tous les fichiers
                gestionnaire.lister_fichiers();
            }
            3 => {
                // Afficher un fichier
                gestionnaire.lister_fichiers();
                if gestionnaire.fichiers.len() > 0 {
                    println!("Numéro du fichier à afficher:");
                    let mut index_input = String::new();
                    io::stdin().read_line(&mut index_input).expect("Erreur de lecture");

                    match index_input.trim().parse::<usize>() {
                        Ok(index) if index > 0 => {
                            gestionnaire.afficher_fichier(index - 1);
                        }
                        _ => println!("Numéro invalide!"),
                    }
                }
            }
            4 => {
                // Modifier un fichier
                gestionnaire.lister_fichiers();
                if gestionnaire.fichiers.len() > 0 {
                    println!("Numéro du fichier à modifier:");
                    let mut index_input = String::new();
                    io::stdin().read_line(&mut index_input).expect("Erreur de lecture");

                    match index_input.trim().parse::<usize>() {
                        Ok(index) if index > 0 => {
                            println!("Nouveau contenu:");
                            let mut nouveau_contenu = String::new();
                            io::stdin().read_line(&mut nouveau_contenu).expect("Erreur de lecture");
                            gestionnaire.modifier_fichier(index - 1, nouveau_contenu.trim().to_string());
                        }
                        _ => println!("Numéro invalide!"),
                    }
                }
            }
            5 => {
                // Supprimer un fichier
                gestionnaire.lister_fichiers();
                if gestionnaire.fichiers.len() > 0 {
                    println!("Numéro du fichier à supprimer:");
                    let mut index_input = String::new();
                    io::stdin().read_line(&mut index_input).expect("Erreur de lecture");

                    match index_input.trim().parse::<usize>() {
                        Ok(index) if index > 0 => {
                            gestionnaire.supprimer_fichier(index - 1);
                        }
                        _ => println!("Numéro invalide!"),
                    }
                }
            }
            6 => {
                // Lire un fichier depuis le disque
                println!("Nom du fichier à lire:");
                let mut nom_fichier = String::new();
                io::stdin().read_line(&mut nom_fichier).expect("Erreur de lecture");
                gestionnaire.lire_depuis_disque(nom_fichier.trim().to_string());
            }
            7 => {
                // Sauvegarder tous les fichiers
                gestionnaire.sauvegarder_tous();
            }
            8 => {
                // Quitter
                println!("Au revoir!");
                break;
            }
            _ => {
                println!("Choix invalide! Veuillez choisir entre 1 et 8.");
            }
        }

        println!(); // Ligne vide pour la lisibilité
    }
}
