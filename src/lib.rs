use supporting_types::Sampling;
pub mod supporting_types;

pub fn main() {
    let _ = (Sampling::Nucleus(12.0), Sampling::Temperature(23.1));
}
