use honggfuzz::fuzz;
use paris::Logger;

fn main() {
    let mut log = Logger::new();
    loop {
        fuzz!(|data: &[u8]| {
            let input: &str = std::str::from_utf8(data).unwrap();
            log.info(input);
        });
    }
}