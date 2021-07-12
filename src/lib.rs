use supporting_types::Sampling;
mod supporting_types;

pub fn main() {
    println!("Hello, world!");
    let _ = (Sampling::Nucleus(12.0), Sampling::Temperature(23.1));
}
