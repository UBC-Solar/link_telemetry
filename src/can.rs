#[derive(Debug)]
pub struct StandardFrame {
    timestamp: u32,
    id: u32,
    payload: u64,
    payload_bytes: Vec<u8>,
    length: u8,
}

impl StandardFrame {
    pub fn new(stream_message: String) -> Result<Self, &'static str> {
        let verified = Self::verify(&stream_message);

        if verified {
            // separate stream message into parts
            let timestamp = u32::from_str_radix(stream_message.get(0..8).unwrap(), 16).unwrap();
            let id = u32::from_str_radix(stream_message.get(8..12).unwrap(), 16).unwrap();
            let length = u8::from_str_radix(stream_message.get(28..29).unwrap(), 16).unwrap();

            let payload_bytes = stream_message
                .get(12..28)
                .unwrap()
                .as_bytes()
                .chunks(2)
                .map(|c| std::str::from_utf8(c).unwrap())
                .map(|c| u8::from_str_radix(c, 16).unwrap())
                .collect::<Vec<u8>>();

            let payload = u64::from_str_radix(stream_message.get(12..28).unwrap(), 16).unwrap();

            Ok(Self {
                timestamp,
                id,
                payload,
                payload_bytes,
                length,
            })
        } else {
            Err("unable to convert stream message into standard frame type")
        }
    }

    pub fn verify(stream_message: &str) -> bool {
        // TODO: improve this
        stream_message.len() == 30
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn payload(&self) -> u64 {
        self.payload
    }
}
