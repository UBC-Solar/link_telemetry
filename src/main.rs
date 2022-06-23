use can::StandardFrame;
use std::time::Instant;

mod can;

fn main() {
    let test_string = String::from("DDDDDDDD062600FF00FF00FF00FF8\n");

    let start_time = Instant::now();

    let frame = StandardFrame::new(test_string);

    println!("{:#?}, time taken: {:?}", frame, start_time.elapsed());
}
