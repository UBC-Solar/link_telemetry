#[derive(Debug)]
pub struct StandardFrame {
    timestamp: u32,
    id: u32,
    data: Vec<u8>,
    length: u32,
}

impl StandardFrame {
    pub fn new(stream_message: String) -> Result<Self, &'static str> {
        let verified = Self::verify(&stream_message);

        if verified {
            // separate stream message into parts
            let timestamp = Self::interpret(stream_message.get(0..8).unwrap());
            let id = Self::interpret(stream_message.get(8..12).unwrap());
            let length = Self::interpret(stream_message.get(28..29).unwrap());

            let data = stream_message
                .get(12..28)
                .unwrap()
                .as_bytes()
                .chunks(2)
                .map(|c| std::str::from_utf8(c).unwrap())
                .map(|c| u8::from_str_radix(c, 16).unwrap())
                .collect::<Vec<u8>>();

            Ok(Self {
                timestamp,
                id,
                data,
                length,
            })
        } else {
            Err("stream message argument is malformed")
        }
    }

    pub fn verify(stream_message: &str) -> bool {
        stream_message.len() == 30
    }

    pub fn interpret(slice: &str) -> u32 {
        slice
            .chars()
            .rev()
            .map(|c| c.to_digit(16).unwrap())
            .enumerate()
            .map(|(idx, digit)| 16u32.pow(idx as u32) * digit)
            .sum::<u32>()
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn payload(&self) -> u64 {
        self.payload
    }
}
