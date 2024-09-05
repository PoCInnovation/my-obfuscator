mod strings;
mod print;
mod functions;
mod obfuscator_struct;
mod dead_code;
pub use obfuscator_struct::Obfuscator;
mod random_identifiers;

trait Shiftable {
    fn shift(&self, shift: i32) -> Self
    where
        Self: Sized;
}

impl Shiftable for std::ops::Range<usize> {
    fn shift(&self, shift: i32) -> Self {
        let sign = shift < 0;
        let shift: usize = shift.abs().try_into().unwrap();

        if sign {
            if shift > self.start {
                return 0..0;
            }
            (self.start - shift)..(self.end - shift)
        } else {
            (self.start + shift)..(self.end + shift)
        }
    }
}
