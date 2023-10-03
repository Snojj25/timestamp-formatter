use timestamp::Timestamp;

#[macro_use]
extern crate afl;

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let _ = Timestamp::parse(s);
        }
    });
}
