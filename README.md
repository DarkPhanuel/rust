# RUST - Apprentissage Fondamentaux
# 21/07/2025

## Introduction au langage Rust
Rust est un langage de programmation système moderne, utilisé pour l'embarqué et le développement web. Il offre une sécurité mémoire supérieure à C++ pour les applications critiques, grâce à son système de gestion de la propriété (ownership) qui évite les fuites mémoire et les erreurs de segmentation.

## A. Installation et environnement de développement

### Vérification de l'installation
Pour vérifier la version de Rust et qu'il soit bien installé :
```bash
rustc --version
```
Pour vérifier Cargo (gestionnaire de paquets de Rust, permet de créer des nouveaux projets, compiler, gérer dépendances) :
```bash
cargo --version
```

Rust est un langage compilé (son compilateur est rustc et son extension est .rs). Il génère un .exe ou un .out après compilation.

### Créer un nouveau projet
```bash
cargo new tp0
```

### Commandes Cargo essentielles
```bash
cargo build       # Compiler en mode debug
cargo run         # Compiler et exécuter
cargo check       # Vérifier le code sans compiler
cargo test        # Exécuter les tests unitaires
cargo update      # Mettre à jour les dépendances
cargo doc --open  # Générer et ouvrir la documentation
```

**Note :** Les dépendances sont à ajouter dans le fichier `Cargo.toml`. Le fichier `main.rs` est le point d'entrée du programme.

## B. Syntaxe et éléments de base

Dans le fichier `main.rs`, `fn main()` est la fonction principale.

### 1) Variables
`let` pour déclarer une variable

**Exemple avec entier non signé sur 32 bits et affichage dans la console via println :**
```rust
let age: u32 = 30; // u32 = entier non signé sur 32 bits (valeur positive)
// Si :u32 n'est pas spécifié, par défaut, le compilateur déduit que c'est un i32
println!("J'ai {} ans.", age);

// Variable mutable
let mut solde: f64 = 1000.0;
```

**Par convention de RUST :** il faut utiliser le snake_case, ne jamais commencer par un chiffre, ni espace, ni tirets. Ajouter un underscore `_` à une variable inutilisée pour éviter les warnings.

### 2) Fonctions
`fn` définit une fonction

**Déclarer une fonction (exemple avec addition) :**
```rust
fn addition(n1: i32, n2: i32) -> i32 { // il faut bien spécifier le type de retour avec ->
    return n1 + n2;
}
// Appeler et afficher la fonction dans la fonction main
let resultat = addition(1, 2);
println!("La somme est {}", resultat);
```

**Référence :** `&str` est un type de chaîne de caractère (référence)
```rust
fn say_hello(nom: &str) {
    println!("Bonjour, {}", nom);
}
// Appeler la fonction dans main
say_hello("Hugo");
```

### 3) Conditions et boucles
**Condition if :**
```rust
let nombre = 16;
if nombre % 2 == 0 {
    println!("Pair");
} else {
    println!("Impair");
}
```

**Boucle for :**
```rust
for i in 1..=10 {
    // intervalle inclusif (ici 10 inclus)
    println!(" i vaut {}", i);
}
for i in 1..6 {
    // intervalle exclusif (ici 6 est exclu)
    println!(" i vaut {}", i);
}
let voitures = ["jeep", "renault", "bmw"];
for voiture in voitures {
    // itérer sur un tableau
    println!("Voiture : {}", voiture);
}
for (i, voiture) in voitures.iter().enumerate() {
    // créer un itérateur sur la collection avec iter() 
    // et créer une séquence de index,valeur avec enumerate()
    println!("Index {} : {}", i, voiture);
}
```

**Exemple de vecteur :** un vecteur, contrairement à un tableau classique, a sa taille qui croît ou diminue de manière dynamique en fonction des besoins
```rust
let noms = vec![String::from("Kevin"), String::from("Noureddine")];
for (i, nom) in noms.iter().enumerate() {
    println!("Nom {}: {}", i, nom);
}
```

### 4) Pattern matching
**match : pattern-matching :**
```rust
let choix = "1";
match choix {
    "1" => println!("Option 1"),
    "2" => println!("Option 2"),
    _ => println!("Option invalide"),
}

// Gestion d'erreurs avec match
let mut choix = String::new();
let choix: usize = match choix.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Veuillez saisir un numéro valide");
        return;
    }
};
```

### 5) Gestion des entrées utilisateur
```rust
use std::io; // Import nécessaire en haut du fichier

// Fonction utilitaire pour lire les entrées
fn lire_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    input.trim().to_string()
}

// Dans main()
println!("Entrez votre choix :");
let choix = lire_input();
```

## C. Structures de données et POO

### Définition d'une structure
```rust
struct Personne {
    nom: String,
    age: u32,
}
```

### Implémentation des méthodes
```rust
impl Personne {
    // Méthode avec référence immutable
    fn afficher(&self) {
        println!("Nom : {}, Âge : {}", self.nom, self.age);
    }

    // Méthode avec référence mutable
    fn vieillir(&mut self) {
        self.age += 1;
    }

    // Méthode qui consomme l'instance (move)
    fn renommer(self, nouveau_nom: &str) -> Personne {
        Personne {
            nom: nouveau_nom.to_string(),
            age: self.age,
        }
    }
}
```

## D. Mécanismes avancés du langage

### 1) Système de propriété (Ownership)
- **`&self`** : référence immutable (lecture seule)
- **`&mut self`** : référence mutable (modification)
- **`self`** : transfert de propriété (move semantics)

### 2) Gestion de la mémoire
Rust gère automatiquement la mémoire sans garbage collector grâce au système d'ownership, évitant les fuites mémoire et les accès invalides.

### 3) Gestion d'erreurs robuste
- Utilisation de `Result<T, E>` et `Option<T>`
- Pattern matching pour gérer les cas d'erreur
- Méthodes comme `unwrap_or()` pour valeurs par défaut

### 4) Collections
- **Vecteurs** : `Vec<T>` pour des collections dynamiques
- **Tableaux** : taille fixe définie à la compilation
- Méthodes d'itération avec `iter()` et `enumerate()`

## E. Projet d'application - Système de gestion bancaire

### Objectif du TP
Développer une application console de gestion de comptes bancaires pour appliquer les concepts Rust appris.

### Structure principale
```rust
struct CompteBancaire {
    nom: String,
    solde: f64,
}

impl CompteBancaire {
    fn afficher(&self) { /* ... */ }
    fn deposer(&mut self, montant: f64) { /* ... */ }
    fn retirer(&mut self, montant: f64) { /* ... */ }
    fn renommer(self, nouveau_nom: &str) -> CompteBancaire { /* ... */ }
    fn fermer(self) { /* ... */ }
}
```

### Fonctionnalités implémentées
1. **Créer un compte** - Application des structs et Vec
2. **Afficher les comptes** - Itération sur collections
3. **Déposer de l'argent** - Références mutables et validation
4. **Retirer de l'argent** - Logique conditionnelle et gestion d'erreurs
5. **Renommer un compte** - Move semantics et ownership
6. **Fermer un compte** - Manipulation de vecteurs
7. **Menu interactif** - Boucles, pattern matching, et gestion d'entrées

### Concepts Rust appliqués dans le TP
- **Structures et méthodes** pour modéliser les comptes
- **Ownership et borrowing** pour la gestion mémoire
- **Pattern matching** pour le menu et la gestion d'erreurs
- **Collections** avec `Vec<CompteBancaire>`
- **Gestion d'entrées utilisateur** avec validation
- **Formatage d'affichage** avec `println!` et spécificateurs de format

Ce TP illustre l'utilisation concrète des concepts Rust dans un projet complet et fonctionnel.