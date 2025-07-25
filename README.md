# RUST - Formation Complète
*Apprentissage Fondamentaux et Programmation Réseau*  
**Dernière mise à jour :** 25/07/2025
**Auteur :** WOANA Joseph

## Table des Matières
1. [Introduction au langage Rust](#introduction-au-langage-rust)
2. [Installation et environnement de développement](#a-installation-et-environnement-de-développement)
3. [Syntaxe et éléments de base](#b-syntaxe-et-éléments-de-base)
4. [Structures de données et POO](#c-structures-de-données-et-poo)
5. [Mécanismes avancés du langage](#d-mécanismes-avancés-du-langage)
6. [Projet d'application - Système de gestion bancaire](#e-projet-dapplication---système-de-gestion-bancaire)
7. [Travaux Pratiques Avancés - Programmation Réseau](#travaux-pratiques-avancés---programmation-réseau)

---

## Introduction au langage Rust

Rust est un langage de programmation système moderne, utilisé pour l'embarqué et le développement web. Il offre une sécurité mémoire supérieure à C++ pour les applications critiques, grâce à son système de gestion de la propriété (ownership) qui évite les fuites mémoire et les erreurs de segmentation.

**Caractéristiques principales :**
- Langage compilé (compilateur : `rustc`, extension : `.rs`)
- Génère un `.exe` ou un `.out` après compilation
- Gestion mémoire automatique sans garbage collector
- Manipulation de mémoire sécurisée et performante

---

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

### Créer un nouveau projet
```bash
cargo new tp0
```

### Commandes Cargo essentielles
```bash
cargo build       # Compiler en mode debug (par défaut)
cargo run         # Compiler et exécuter le projet
cargo check       # Vérifier le code sans compiler
cargo test        # Exécuter les tests unitaires
cargo update      # Mettre à jour les dépendances
cargo doc --open  # Générer et ouvrir la documentation dans le navigateur web
```

**Configuration importante :**
- Les dépendances sont à ajouter dans le fichier `Cargo.toml` sous `[dependencies]`
- Le fichier `main.rs` est le point d'entrée du programme
- Pour manipuler des fichiers : `use std::fs::File`

---

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

**Convention de nommage RUST :**
- Utiliser le `snake_case` (et non `camelCase`)
- Ne jamais commencer par un chiffre
- Pas d'espaces ni de tirets
- Ajouter un underscore `_` à une variable inutilisée pour éviter les warnings

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

### 6) Formatage et affichage
```rust
// println! est une macro qui permet d'afficher dans la console (comme printf en C)
use chrono::Local;
let maintenant = Local::now();
println!("Format FR : {}", maintenant.format("%d/%m/%Y")); // Format JJ/MM/YYYY  
println!("Format FR : {}", maintenant.format("%d/%m/%Y %H:%M:%S"));
```

---

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

---

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

---

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

---

## Travaux Pratiques Avancés - Programmation Réseau

### TP 7: Client et Serveur DNS Simples

**Description :**  
Ce TP propose d'implémenter un client DNS basique capable de résoudre des noms de domaine en adresses IP, et un serveur DNS simple qui répond à des requêtes pour quelques domaines prédéfinis. Ce TP explorera la programmation UDP et le format des messages DNS.

**Prérequis :**
- Maîtrise de la programmation réseau UDP en Rust
- Compréhension du protocole DNS (structure des requêtes/réponses)
- Manipulation de données binaires

**Objectifs Pédagogiques :**
- Envoyer et recevoir des paquets UDP formatés
- Parser et construire des messages DNS (en-têtes, questions, réponses)
- Implémenter une logique de résolution DNS simple
- Comprendre le rôle du DNS dans l'infrastructure réseau

**Concepts Clés :**
- `UdpSocket`
- Format des messages DNS (RFC 1035)
- Sérialisation/désérialisation de données binaires
- Résolution de noms

---

### TP 8: Implémentation d'un Protocole Personnalisé

**Description :**  
Les étudiants concevront et implémenteront un protocole réseau simple (par exemple, un protocole de messagerie ou de transfert de fichiers) au-dessus de TCP ou UDP. Ils devront définir le format des messages, les règles d'échange et implémenter un client et un serveur conformes à ce protocole.

**Prérequis :**
- Excellente maîtrise de la programmation réseau en Rust (TCP et/ou UDP)
- Capacité à concevoir des spécifications de protocole
- Bonne compréhension de la sérialisation/désérialisation

**Objectifs Pédagogiques :**
- Concevoir un protocole réseau fonctionnel
- Implémenter un protocole à partir de zéro
- Gérer les états de session (si TCP)
- Développer des clients et serveurs interopérables
- Mettre en pratique toutes les connaissances acquises en programmation réseau

**Concepts Clés :**
- Définition de protocole (en-têtes, corps, codes d'opération)
- Sérialisation/désérialisation (ex: serde)
- Gestion des états
- Robustesse et gestion des erreurs de protocole

---

### TP 9: Serveur et Client WebSocket

**Description :**  
Ce TP vise à implémenter un serveur et un client WebSocket en Rust. Les WebSockets permettent une communication bidirectionnelle persistante entre un client et un serveur, idéale pour les applications en temps réel comme les chats ou les tableaux de bord interactifs. Les étudiants utiliseront une crate comme `tokio-tungstenite`.

**Prérequis :**
- Bonne maîtrise de la programmation asynchrone avec Tokio
- Compréhension des concepts de base de HTTP (pour la phase de handshake WebSocket)

**Objectifs Pédagogiques :**
- Comprendre le protocole WebSocket et son utilité
- Implémenter un serveur WebSocket capable de gérer plusieurs connexions
- Développer un client WebSocket pour interagir avec le serveur
- Gérer les messages WebSocket (texte, binaire)

**Concepts Clés :**
- WebSocket (protocole)
- `tokio-tungstenite` (crate)
- Handshake WebSocket
- Communication full-duplex
- Messages texte et binaires

---

## Ressources Complémentaires

### Documentation Officielle
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)

### Crates Utiles pour la Programmation Réseau
- `tokio` - Runtime asynchrone
- `tokio-tungstenite` - WebSockets
- `serde` - Sérialisation/désérialisation
- `chrono` - Manipulation des dates

### Commandes de Référence Rapide
```bash
# Gestion de projet
cargo new mon_projet
cargo build
cargo run
cargo test
cargo check

# Documentation
cargo doc --open

# Mise à jour
cargo update
```

---

*Ce document constitue un guide complet pour l'apprentissage de Rust, des fondamentaux à la programmation réseau avancée. Il illustre l'utilisation concrète des concepts Rust dans des projets complets et fonctionnels.*
