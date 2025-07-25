# TP9 - Chat WebSocket Rust

Ce projet c'est un chat en Rust qui marche avec WebSocket. Il y a un serveur et des clients qui peuvent discuter en temps réel. C'est fait pour un TP, donc c'est pas Facebook Messenger, mais ça marche !

## Ce que ça fait
- Un serveur qui accepte plusieurs clients en même temps
- Les clients peuvent envoyer des messages texte (et même du binaire, mais bon...)
- On peut faire /ping pour tester la connexion, et /quit pour partir
- Quand quelqu'un rejoint ou quitte, tout le monde le voit

## Comment lancer

### 1. D'abord, il faut compiler (faut avoir Rust installé)
```sh
cargo build
```

### 2. Lancer le serveur
Dans un terminal :
```sh
cargo run --bin server
```
Par défaut il écoute sur 127.0.0.1:8080

### 3. Lancer un client
Dans un autre terminal (tu peux en ouvrir plusieurs pour tester le chat) :
```sh
cargo run --bin client -- --username TonPseudo
```

Tu peux aussi préciser l'URL si tu veux (par défaut c'est ws://127.0.0.1:8080) :
```sh
cargo run --bin client -- --url ws://127.0.0.1:8080 --username TonPseudo
```

### 4. Utilisation
- Tape un message et appuie sur Entrée pour l'envoyer
- Tape `/ping` pour tester si le serveur répond
- Tape `/quit` pour quitter le chat

## Remarques
- Si tu veux tester à plusieurs, ouvre plusieurs terminaux et lance plusieurs clients
- Les messages système s'affichent quand quelqu'un rejoint ou quitte
- Si tu veux envoyer du binaire... bon courage, c'est pas super utile ici
- C'était joseph à la cuisine comme d'habitude

Voilà, amuse-toi bien avec ce chat Rust ! 