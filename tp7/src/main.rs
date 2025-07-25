
mod client;
mod server;

// Ce fichier main.rs est le point d'entrée par défaut du projet Rust.
// Dans ce TP, il ne fait qu'afficher 'Hello, world!' car les vraies fonctionnalités sont dans les binaires dns_client et dns_server.
// Pour utiliser le client ou le serveur DNS, il faut lancer les binaires correspondants.
// Pour lancer le client, il faut utiliser la commande `cargo run --bin dns_client`.
// Pour lancer le serveur, il faut utiliser la commande `cargo run --bin dns_server`.
// Pour lancer le serveur avec des logs détaillés, il faut utiliser la commande `cargo run --bin dns_server -- --verbose`.
// Pour lancer le client avec des logs détaillés, il faut utiliser la commande `cargo run --bin dns_client -- --verbose`.
// Pour lancer le client avec des logs détaillés, il faut utiliser la commande `cargo run --bin dns_client -- --verbose`.

// Tout est expliqué dans le readme.md

fn main() {
    println!("Hello, world!");
}
