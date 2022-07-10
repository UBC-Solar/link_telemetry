#[derive(Debug)]
pub struct Measurement {
    msg_id: u32,
    name: String,
    source: String,
    class: String,
    value: MeasurementValue,
}

#[derive(Debug)]
pub enum MeasurementValue {
    IEEE32Float(f32),
    Bool(bool),
    Unsigned8(u8),
    Unsigned16(u16),
    Incremental(f32),
    Signed16(i16),
    Signed8(i8),
    Invalid
}

impl Measurement {
    pub fn new(
        msg_id: u32,
        name: String,
        source: String,
        class: String,
        value: MeasurementValue,
    ) -> Self {
        Self {
            msg_id,
            name,
            source,
            class,
            value,
        }
    }
}
