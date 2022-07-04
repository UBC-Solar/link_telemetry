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
    Unsigned(u8),
    Incremental(f32),
    Signed16(i16),
    Signed8(i8),
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
