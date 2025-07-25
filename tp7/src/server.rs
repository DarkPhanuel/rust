use anyhow::Result;
use clap::Parser;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};

mod dns;
use dns::*;

#[derive(Parser)]
#[command(name = "dns_server")]
#[command(about = "Serveur DNS simple")]
struct Args {
    /// Port d'écoute (par défaut: 5353)
    #[arg(short, long, default_value = "8053")]
    port: u16,

    /// Adresse d'écoute (par défaut: 0.0.0.0)
    #[arg(short, long, default_value = "0.0.0.0")]
    address: String,

    /// Mode verbeux
    #[arg(short, long)]
    verbose: bool,
}

struct DnsServer {
    socket: UdpSocket,
    database: Arc<Mutex<DnsDatabase>>,
    verbose: bool,
}

impl DnsServer {
    fn new(addr: SocketAddr, verbose: bool) -> Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        let database = Arc::new(Mutex::new(DnsDatabase::new()));

        Ok(Self {
            socket,
            database,
            verbose,
        })
    }

    fn handle_query(&self, requete: DnsMessage, client_addr: SocketAddr) -> Result<()> {
        if self.verbose {
            println!("Requête reçue de {} (ID: {})", client_addr, requete.header.id);
        }

        if requete.questions.is_empty() {
            if self.verbose {
                println!("  ❌ Aucune question dans la requête");
            }
            return Ok(());
        }

        let question = &requete.questions[0];
        let domaine = &question.name;

        if self.verbose {
            println!("  🔍 Recherche de '{}'", domaine);
        }

        let database = self.database.lock().unwrap();
        let reponse = match database.lookup(domaine) {
            Some(ip) => {
                if self.verbose {
                    println!("  ✅ Trouvé: {} -> {}", domaine, ip);
                }
                let answer = DnsAnswer::new_a_record(domaine.clone(), ip, 3600);
                DnsMessage::new_response(requete.header.id, question.clone(), vec![answer])
            }
            None => {
                if self.verbose {
                    println!("  ❌ Non trouvé: {}", domaine);
                }
                // Réponse vide (NXDOMAIN)
                let mut reponse = DnsMessage::new_response(requete.header.id, question.clone(), vec![]);
                reponse.header.flags |= 0x0003; // NXDOMAIN
                reponse
            }
        };

        let reponse_bytes = reponse.to_bytes();
        self.socket.send_to(&reponse_bytes, client_addr)?;

        if self.verbose {
            println!("  📤 Réponse envoyée ({} bytes)", reponse_bytes.len());
        }

        Ok(())
    }

    fn run(&self) -> Result<()> {
        println!("🚀 Serveur DNS démarré sur {}", self.socket.local_addr()?);
        println!("📋 Domaines configurés:");

        let database = self.database.lock().unwrap();
        let mut domains: Vec<_> = database.all_records().iter().collect();
        domains.sort_by_key(|(domain, _)| *domain);

        for (domain, ip) in domains {
            println!("   {} -> {}", domain, ip);
        }
        drop(database);

        println!("\n⏳ En attente de requêtes...\n");

        let mut buffer = [0u8; 512];

        loop {
            match self.socket.recv_from(&mut buffer) {
                Ok((size, client_addr)) => {
                    if self.verbose {
                        println!("📨 Paquet reçu de {} ({} bytes)", client_addr, size);
                    }

                    match DnsMessage::from_bytes(&buffer[..size]) {
                        Ok(requete) => {
                            if let Err(e) = self.handle_query(requete, client_addr) {
                                eprintln!("❌ Erreur lors du traitement: {}", e);
                            }
                        }
                        Err(e) => {
                            if self.verbose {
                                println!("  ❌ Erreur de parsing: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Erreur de réception: {}", e);
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let addr = format!("{}:{}", args.address, args.port);
    let socket_addr: SocketAddr = addr.parse()
        .map_err(|_| anyhow::anyhow!("Adresse invalide: {}", addr))?;

    println!("🔧 Configuration du serveur DNS");
    println!("   Adresse: {}", socket_addr);
    println!("   Mode verbeux: {}", args.verbose);
    println!();

    let server = DnsServer::new(socket_addr, args.verbose)?;
    server.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_database() {
        let db = DnsDatabase::new();

        assert_eq!(db.lookup("example.com"), Some(Ipv4Addr::new(93, 184, 216, 34)));
        assert_eq!(db.lookup("nonexistent.com"), None);
    }

    #[test]
    fn test_dns_answer_creation() {
        let answer = DnsAnswer::new_a_record(
            "test.com".to_string(),
            Ipv4Addr::new(192, 168, 1, 1),
            3600
        );

        assert_eq!(answer.name, "test.com");
        assert_eq!(answer.get_ip(), Some(Ipv4Addr::new(192, 168, 1, 1)));
    }
}
