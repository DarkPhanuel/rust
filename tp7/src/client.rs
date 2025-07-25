use anyhow::Result;
use clap::Parser;
use rand::random;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

mod dns;
use dns::*;

#[derive(Parser)]
#[command(name = "dns_client")]
#[command(about = "Client DNS simple")]
struct Args {
    /// Nom de domaine à résoudre
    domain: String,

    /// Serveur DNS à utiliser (par défaut: 127.0.0.1:5353)
    #[arg(short, long, default_value = "127.0.0.1:8053")]
    server: String,

    /// Timeout en secondes
    #[arg(short, long, default_value = "5")]
    timeout: u64,

    /// Mode verbeux
    #[arg(short, long)]
    verbose: bool,
}

struct DnsClient {
    socket: UdpSocket,
    server_addr: SocketAddr,
    verbose: bool,
}

impl DnsClient {
    fn new(server_addr: SocketAddr, timeout: Duration, verbose: bool) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_read_timeout(Some(timeout))?;
        socket.set_write_timeout(Some(timeout))?;

        Ok(Self {
            socket,
            server_addr,
            verbose,
        })
    }

    fn resolve(&self, domain: &str) -> Result<Option<std::net::Ipv4Addr>> {
        // Générer un ID aléatoire pour la requête
        let requete_id = random::<u16>();

        // Construire la requête DNS
        let requete = DnsMessage::new_query(requete_id, domain.to_string());
        let requete_bytes = requete.to_bytes();

        if self.verbose {
            println!("Envoi de la requête DNS pour '{}' (ID: {})", domain, requete_id);
            println!("Taille de la requête: {} bytes", requete_bytes.len());
        }

        // Envoyer la requête
        self.socket.send_to(&requete_bytes, self.server_addr)?;

        // Recevoir la réponse
        let mut buffer = [0u8; 512];
        let result = self.socket.recv_from(&mut buffer);

        match result {
            Ok((size, from)) => {
                if self.verbose {
                    println!("Réponse reçue de {} ({} bytes)", from, size);
                }
                // Parser la réponse
                let reponse = match DnsMessage::from_bytes(&buffer[..size]) {
                    Ok(resp) => resp,
                    Err(e) => {
                        if self.verbose {
                            println!("  ❌ Erreur de parsing détaillée: {}", e);
                            println!("  🔍 Données reçues (hex): {}",
                                     buffer[..size].iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" "));
                        }
                        return Err(anyhow::anyhow!("Erreur de parsing: {}", e));
                    }
                };

                if reponse.header.id != requete_id {
                    return Err(anyhow::anyhow!("ID de réponse incorrect"));
                }

                if self.verbose {
                    println!("Réponse parsée - {} réponse(s)", reponse.answers.len());
                }

                // Extraire l'adresse IP de la première réponse A
                for answer in &reponse.answers {
                    if let Some(ip) = answer.get_ip() {
                        if self.verbose {
                            println!("  📍 Adresse trouvée: {}", ip);
                        }
                        return Ok(Some(ip));
                    }
                }

                Ok(None)
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                if self.verbose {
                    println!("⏰ Timeout atteint, aucune réponse du serveur.");
                }
                Ok(None)
            }
            Err(e) => Err(e.into()),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Parser l'adresse du serveur
    let server_addr: SocketAddr = args.server.parse()
        .map_err(|_| anyhow::anyhow!("Adresse serveur invalide: {}", args.server))?;

    let timeout = Duration::from_secs(args.timeout);

    println!("Client DNS - Résolution de '{}'", args.domain);
    println!("Serveur: {}", server_addr);
    println!("Timeout: {:?}", timeout);
    println!();

    // Créer le client
    let client = DnsClient::new(server_addr, timeout, args.verbose)?;

    // Effectuer la résolution
    match client.resolve(&args.domain) {
        Ok(Some(ip)) => {
            println!("✅ Résolution réussie:");
            println!("   {} -> {}", args.domain, ip);
        }
        Ok(None) => {
            println!("❌ Aucune adresse trouvée pour '{}'", args.domain);
        }
        Err(e) => {
            println!("❌ Erreur lors de la résolution: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_message_query() {
        let requete = DnsMessage::new_query(12345, "example.com".to_string());
        let bytes = requete.to_bytes();

        // Vérifier que le message peut être sérialisé et désérialisé
        let parsed = DnsMessage::from_bytes(&bytes).unwrap();
        assert_eq!(parsed.header.id, 12345);
        assert_eq!(parsed.questions[0].name, "example.com");
    }
}
