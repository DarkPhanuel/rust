# TP7 – Client et Serveur DNS Simples (Rust)

## C'est quoi ce projet ?

Ce projet c'est un TP où on fait un client DNS (qui demande l'adresse IP d'un nom de domaine) et un serveur DNS (qui répond à ces questions pour quelques domaines connus). C'est fait en Rust, et ça utilise les sockets UDP, donc c'est un peu bas niveau, mais pas trop compliqué non plus.

## À quoi ça sert ?

- Comprendre comment marche le DNS (Domain Name System) en vrai, pas juste en théorie.
- Apprendre à envoyer et recevoir des paquets UDP en Rust.
- Manipuler des messages binaires (sérialisation/désérialisation).
- Voir comment un serveur répond à une requête réseau.

## Notions utilisées

- **UDP** : Protocole réseau sans connexion, rapide mais pas fiable (on s'en sert pour le DNS dans la vraie vie).
- **DNS** : Système qui traduit les noms de domaine (genre google.com) en adresses IP.
- **RFC 1035** : C'est le document officiel qui explique comment sont formatés les messages DNS (en-têtes, questions, réponses, etc.).
- **Rust** : Langage de programmation moderne, rapide, sûr (mais parfois un peu strict sur la gestion de la mémoire).
- **Sérialisation/Désérialisation** : Transformer des structures Rust en suite d'octets (et inversement) pour les envoyer sur le réseau.
- **Mutex/Arc** : Pour partager la base de données DNS entre plusieurs threads (même si ici on n'a pas de vrai multi-thread, c'est prêt pour).

## Comment ça marche ?

- Le **serveur** écoute sur un port UDP (par défaut 8053) et attend des requêtes DNS. Il connaît quelques domaines (genre example.com, google.com, etc.) et répond avec l'adresse IP correspondante. Si le domaine n'est pas connu, il dit "je sais pas" (NXDOMAIN).
- Le **client** envoie une requête DNS pour un domaine au serveur, attend la réponse, et affiche l'adresse IP si trouvée.

## Comment lancer ?

1. Compiler le projet :
   ```bash
   cargo build --release
   ```

2. Lancer le serveur (dans un terminal) :
   ```bash
   cargo run --bin dns_server
   ```
   (Tu peux changer le port avec `--port 8053` si besoin)

3. Lancer le client (dans un autre terminal) :
   ```bash
   cargo run --bin dns_client -- example.com
   ```
   (Tu peux mettre un autre domaine connu du serveur)

## Domaines connus par le serveur

- example.com
- google.com
- github.com
- localhost
- test.local

## Sources / Documentation

- [RFC 1035 – Domain names - implementation and specification](https://datatracker.ietf.org/doc/html/rfc1035)
- [La doc Rust sur std::net::UdpSocket](https://doc.rust-lang.org/std/net/struct.UdpSocket.html)
- [Rust Book (le livre officiel)](https://doc.rust-lang.org/book/)
- [Wikipedia – Domain Name System](https://fr.wikipedia.org/wiki/Domain_Name_System)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)

## Remarques

- Ce n'est pas un vrai serveur DNS complet, c'est juste pour apprendre.
- Il n'y a pas de gestion IPv6, ni de cache, ni de récursivité.
- Si toi qui lis ce readme, tu veux t'amuser, tu peux ajouter d'autres domaines dans le code !
- C'était joseph au clavier ! _°°_