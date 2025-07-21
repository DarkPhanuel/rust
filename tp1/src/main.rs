use std::io;

fn main() {
    let mut solde: f32 = 1000.0;
    let comptes = vec!["Kevin", "Nourdine", "Fatou"];
    let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];

    println!("--- MENU ---");
    for (i, option) in options.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }

    println!("Entrez le numéro de votre choix :");

    let mut choix = String::new();
    io::stdin().read_line(&mut choix).expect("Erreur de lecture");

    let choix: usize = match choix.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez entrer un nombre valide.");
            return;
        }
    };

    if choix == 0 || choix > options.len() {
        println!("Choix invalide !");
        return;
    }

    println!("Vous avez choisi : {}", options[choix - 1]);

    if choix == 1 {
        println!("Votre solde est : {:.2} €", solde);
    }

    if choix == 2 {
        println!("Montant à retirer :");
        let mut montant = String::new();
        io::stdin().read_line(&mut montant).expect("Erreur de lecture");

        let montant: f32 = match montant.trim().parse() {
            Ok(valeur) => valeur,
            Err(_) => {
                println!("Montant invalide !");
                return;
            }
        };

        if montant > solde {
            println!("Fonds insuffisants !");
        } else {
            solde = solde - montant;
            println!("Retrait effectué. Nouveau solde : {:.2} €", solde);
        }
    }

    if choix == 3 {
        println!("Liste des comptes :");
        for (i, compte) in comptes.iter().enumerate() {
            println!("{}. {}", i + 1, compte);
        }
    }

    if choix == 4 {
        println!("Merci et à bientôt !");
    }
}
