use std::io;

struct CompteBancaire {
    nom: String,
    solde: f64,
}

impl CompteBancaire {
    fn afficher(&self) {
        println!("Compte de {} : {:.2} €", self.nom, self.solde);
    }

    fn deposer(&mut self, montant: f64) {
        if montant <= 0.0 {
            println!("⚠️  Montant invalide. Dépôt refusé.");
        } else {
            self.solde += montant;
            println!("✅ +{:.2} € déposés sur le compte de {}.", montant, self.nom);
        }
    }

    fn retirer(&mut self, montant: f64) {
        if montant <= 0.0 {
            println!("⚠️  Le montant doit être positif.");
        } else if self.solde >= montant {
            self.solde -= montant;
            println!("✅ -{:.2} € retirés du compte de {}.", montant, self.nom);
        } else {
            println!("❌ Solde insuffisant !");
        }
    }

    fn renommer(self, nouveau_nom: &str) -> CompteBancaire {
        CompteBancaire {
            nom: nouveau_nom.to_string(),
            solde: self.solde,
        }
    }

    fn fermer(self) {
        println!(
            "🧾 Le compte de {} est fermé. Dernier solde : {:.2} €",
            self.nom, self.solde
        );
    }
}

// Fonction utilitaire pour lire une ligne depuis l'utilisateur
fn lire_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    input.trim().to_string()
}

fn main() {
    let mut comptes: Vec<CompteBancaire> = Vec::new();

    loop {
        println!("\n--- MENU ---");
        println!("1. Créer un compte");
        println!("2. Afficher les comptes");
        println!("3. Déposer de l'argent");
        println!("4. Retirer de l'argent");
        println!("5. Renommer un compte");
        println!("6. Fermer un compte");
        println!("7. Quitter");

        println!("Entrez votre choix :");
        let choix = lire_input();

        match choix.as_str() {
            "1" => {
                println!("Nom du nouveau compte :");
                let nom = lire_input();
                comptes.push(CompteBancaire { nom, solde: 0.0 });
                println!("✅ Compte créé avec succès !");
            }

            "2" => {
                println!("\n--- Comptes enregistrés ---");
                for (i, compte) in comptes.iter().enumerate() {
                    print!("{} - ", i + 1);
                    compte.afficher();
                }
            }

            "3" => {
                println!("Numéro du compte pour déposer :");
                let index = lire_input().parse::<usize>().unwrap_or(0);
                if let Some(compte) = comptes.get_mut(index - 1) {
                    println!("Montant à déposer :");
                    let montant = lire_input().parse::<f64>().unwrap_or(-1.0);
                    compte.deposer(montant);
                } else {
                    println!("❌ Compte non trouvé !");
                }
            }

            "4" => {
                println!("Numéro du compte pour retirer :");
                let index = lire_input().parse::<usize>().unwrap_or(0);
                if let Some(compte) = comptes.get_mut(index - 1) {
                    println!("Montant à retirer :");
                    let montant = lire_input().parse::<f64>().unwrap_or(-1.0);
                    compte.retirer(montant);
                } else {
                    println!("❌ Compte non trouvé !");
                }
            }

            "5" => {
                println!("Numéro du compte à renommer :");
                let index = lire_input().parse::<usize>().unwrap_or(0);
                if index == 0 || index > comptes.len() {
                    println!("❌ Compte invalide.");
                    continue;
                }

                println!("Nouveau nom du compte :");
                let nouveau_nom = lire_input();

                // remplacement dans le vecteur
                let ancien = comptes.remove(index - 1);
                let nouveau = ancien.renommer(&nouveau_nom);
                comptes.insert(index - 1, nouveau);

                println!("✅ Compte renommé avec succès !");
            }

            "6" => {
                println!("Numéro du compte à fermer :");
                let index = lire_input().parse::<usize>().unwrap_or(0);
                if index == 0 || index > comptes.len() {
                    println!("❌ Compte invalide.");
                    continue;
                }

                let compte = comptes.remove(index - 1);
                compte.fermer();
            }

            "7" => {
                println!("👋 Merci d’avoir utilisé notre système bancaire !");
                break;
            }

            _ => {
                println!("❌ Option invalide. Veuillez choisir un numéro entre 1 et 7.");
            }
        }
    }
}
