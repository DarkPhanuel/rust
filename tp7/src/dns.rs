use std::collections::HashMap;
use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub struct DnsHeader {
    pub id: u16,
    pub flags: u16,
    pub question_count: u16,
    pub answer_count: u16,
    pub authority_count: u16,
    pub additional_count: u16,
}

#[allow(dead_code)]
impl DnsHeader {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            flags: 0x0100, // Requête standard avec récursion souhaitée
            question_count: 1,
            answer_count: 0,
            authority_count: 0,
            additional_count: 0,
        }
    }

    pub fn new_response(id: u16, answer_count: u16) -> Self {
        Self {
            id,
            flags: 0x8180, // Réponse avec récursion disponible
            question_count: 1,
            answer_count,
            authority_count: 0,
            additional_count: 0,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(12);
        bytes.extend_from_slice(&self.id.to_be_bytes());
        bytes.extend_from_slice(&self.flags.to_be_bytes());
        bytes.extend_from_slice(&self.question_count.to_be_bytes());
        bytes.extend_from_slice(&self.answer_count.to_be_bytes());
        bytes.extend_from_slice(&self.authority_count.to_be_bytes());
        bytes.extend_from_slice(&self.additional_count.to_be_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 12 {
            return Err("En-tête trop court");
        }

        Ok(Self {
            id: u16::from_be_bytes([bytes[0], bytes[1]]),
            flags: u16::from_be_bytes([bytes[2], bytes[3]]),
            question_count: u16::from_be_bytes([bytes[4], bytes[5]]),
            answer_count: u16::from_be_bytes([bytes[6], bytes[7]]),
            authority_count: u16::from_be_bytes([bytes[8], bytes[9]]),
            additional_count: u16::from_be_bytes([bytes[10], bytes[11]]),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DnsQuestion {
    pub name: String,
    pub qtype: u16,
    pub qclass: u16,
}

#[allow(dead_code)]
impl DnsQuestion {
    pub fn new(name: String) -> Self {
        Self {
            name,
            qtype: 1,    // Enregistrement A
            qclass: 1,   // Classe IN
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Encodage du nom de domaine
        for label in self.name.split('.') {
            if !label.is_empty() {
                bytes.push(label.len() as u8);
                bytes.extend_from_slice(label.as_bytes());
            }
        }
        bytes.push(0); // Fin du nom

        bytes.extend_from_slice(&self.qtype.to_be_bytes());
        bytes.extend_from_slice(&self.qclass.to_be_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        let name = Self::decode_name(bytes, offset)?;

        if *offset + 4 > bytes.len() {
            return Err("Question trop courte");
        }

        let qtype = u16::from_be_bytes([bytes[*offset], bytes[*offset + 1]]);
        let qclass = u16::from_be_bytes([bytes[*offset + 2], bytes[*offset + 3]]);
        *offset += 4;

        Ok(Self { name, qtype, qclass })
    }

    fn decode_name(bytes: &[u8], offset: &mut usize) -> Result<String, &'static str> {
        let mut name_parts = Vec::new();
        let mut pos = *offset;
        let mut jumped = false;
        let mut jumps = 0;

        loop {
            if jumps > 5 { // Éviter les boucles infinies
                return Err("Trop de sauts de compression");
            }

            if pos >= bytes.len() {
                return Err("Encodage de nom invalide");
            }

            let length = bytes[pos] as usize;

            // Vérifier si c'est un pointeur de compression (les 2 bits de poids fort sont 11)
            if length & 0xC0 == 0xC0 {
                if pos + 1 >= bytes.len() {
                    return Err("Pointeur de compression invalide");
                }

                if !jumped {
                    *offset = pos + 2;
                }

                // Calculer l'offset du pointeur (14 bits)
                let pointer_offset = ((length & 0x3F) as usize) << 8 | (bytes[pos + 1] as usize);
                pos = pointer_offset;
                jumped = true;
                jumps += 1;
                continue;
            }

            pos += 1;

            if length == 0 {
                break;
            }

            if pos + length > bytes.len() {
                return Err("Encodage de nom invalide");
            }

            let label = String::from_utf8_lossy(&bytes[pos..pos + length]);
            name_parts.push(label.to_string());
            pos += length;
        }

        if !jumped {
            *offset = pos;
        }

        Ok(name_parts.join("."))
    }
}

#[derive(Debug, Clone)]
pub struct DnsAnswer {
    pub name: String,
    pub atype: u16,
    pub class: u16,
    pub ttl: u32,
    pub data: Vec<u8>,
}

#[allow(dead_code)]
impl DnsAnswer {
    pub fn new_a_record(name: String, ip: Ipv4Addr, ttl: u32) -> Self {
        Self {
            name,
            atype: 1, // Enregistrement A
            class: 1, // Classe IN
            ttl,
            data: ip.octets().to_vec(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Encodage du nom de domaine
        for label in self.name.split('.') {
            if !label.is_empty() {
                bytes.push(label.len() as u8);
                bytes.extend_from_slice(label.as_bytes());
            }
        }
        bytes.push(0); // Fin du nom

        bytes.extend_from_slice(&self.atype.to_be_bytes());
        bytes.extend_from_slice(&self.class.to_be_bytes());
        bytes.extend_from_slice(&self.ttl.to_be_bytes());
        bytes.extend_from_slice(&(self.data.len() as u16).to_be_bytes());
        bytes.extend_from_slice(&self.data);

        bytes
    }

    pub fn from_bytes(bytes: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        let name = DnsQuestion::decode_name(bytes, offset)?;

        if *offset + 10 > bytes.len() {
            return Err("Réponse trop courte");
        }

        let atype = u16::from_be_bytes([bytes[*offset], bytes[*offset + 1]]);
        let class = u16::from_be_bytes([bytes[*offset + 2], bytes[*offset + 3]]);
        let ttl = u32::from_be_bytes([
            bytes[*offset + 4], bytes[*offset + 5],
            bytes[*offset + 6], bytes[*offset + 7]
        ]);
        let data_len = u16::from_be_bytes([bytes[*offset + 8], bytes[*offset + 9]]) as usize;
        *offset += 10;

        if *offset + data_len > bytes.len() {
            return Err("Données de réponse trop courtes");
        }

        let data = bytes[*offset..*offset + data_len].to_vec();
        *offset += data_len;

        Ok(Self { name, atype, class, ttl, data })
    }

    #[allow(dead_code)]
    pub fn get_ip(&self) -> Option<Ipv4Addr> {
        if self.atype == 1 && self.data.len() == 4 {
            Some(Ipv4Addr::from([self.data[0], self.data[1], self.data[2], self.data[3]]))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct DnsMessage {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsAnswer>,
}

#[allow(dead_code)]
impl DnsMessage {
    pub fn new_query(id: u16, domain: String) -> Self {
        Self {
            header: DnsHeader::new(id),
            questions: vec![DnsQuestion::new(domain)],
            answers: Vec::new(),
        }
    }

    pub fn new_response(id: u16, question: DnsQuestion, answers: Vec<DnsAnswer>) -> Self {
        Self {
            header: DnsHeader::new_response(id, answers.len() as u16),
            questions: vec![question],
            answers,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.header.to_bytes());

        for question in &self.questions {
            bytes.extend_from_slice(&question.to_bytes());
        }

        for answer in &self.answers {
            bytes.extend_from_slice(&answer.to_bytes());
        }

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        let header = DnsHeader::from_bytes(bytes)?;
        let mut offset = 12;

        let mut questions = Vec::new();
        for _ in 0..header.question_count {
            questions.push(DnsQuestion::from_bytes(bytes, &mut offset)?);
        }

        let mut answers = Vec::new();
        for _ in 0..header.answer_count {
            answers.push(DnsAnswer::from_bytes(bytes, &mut offset)?);
        }

        Ok(Self { header, questions, answers })
    }
}

pub struct DnsDatabase {
    records: HashMap<String, Ipv4Addr>,
}

#[allow(dead_code)]
impl DnsDatabase {
    pub fn new() -> Self {
        let mut records = HashMap::new();
        records.insert("example.com".to_string(), Ipv4Addr::new(93, 184, 216, 34));
        records.insert("google.com".to_string(), Ipv4Addr::new(142, 250, 185, 110));
        records.insert("github.com".to_string(), Ipv4Addr::new(140, 82, 112, 3));
        records.insert("localhost".to_string(), Ipv4Addr::new(127, 0, 0, 1));
        records.insert("test.local".to_string(), Ipv4Addr::new(192, 168, 1, 100));

        Self { records }
    }

    pub fn lookup(&self, domain: &str) -> Option<Ipv4Addr> {
        self.records.get(domain).copied()
    }

    #[allow(dead_code)]
    pub fn add_record(&mut self, domain: String, ip: Ipv4Addr) {
        self.records.insert(domain, ip);
    }

    pub fn all_records(&self) -> &HashMap<String, Ipv4Addr> {
        &self.records
    }
}
