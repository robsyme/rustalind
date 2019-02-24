//use seq::translation::{TranslationTable, TranslatedCodon};
use super::nuc::DNA;
use seq::translation::{TranslationTable, TranslatedCodon};
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub struct Codon(DNA, DNA, DNA);

impl Codon {
    pub fn get_translation_index(&self) -> Option<usize> {
        match self {
            Codon(DNA::T, DNA::T, DNA::T) => Some(00),
            Codon(DNA::T, DNA::T, DNA::C) => Some(01),
            Codon(DNA::T, DNA::T, DNA::A) => Some(02),
            Codon(DNA::T, DNA::T, DNA::G) => Some(03),
            Codon(DNA::T, DNA::C, DNA::T) => Some(04),
            Codon(DNA::T, DNA::C, DNA::C) => Some(05),
            Codon(DNA::T, DNA::C, DNA::A) => Some(06),
            Codon(DNA::T, DNA::C, DNA::G) => Some(07),
            Codon(DNA::T, DNA::A, DNA::T) => Some(08),
            Codon(DNA::T, DNA::A, DNA::C) => Some(09),
            Codon(DNA::T, DNA::A, DNA::A) => Some(10),
            Codon(DNA::T, DNA::A, DNA::G) => Some(11),
            Codon(DNA::T, DNA::G, DNA::T) => Some(12),
            Codon(DNA::T, DNA::G, DNA::C) => Some(13),
            Codon(DNA::T, DNA::G, DNA::A) => Some(14),
            Codon(DNA::T, DNA::G, DNA::G) => Some(15),
            Codon(DNA::C, DNA::T, DNA::T) => Some(16),
            Codon(DNA::C, DNA::T, DNA::C) => Some(17),
            Codon(DNA::C, DNA::T, DNA::A) => Some(18),
            Codon(DNA::C, DNA::T, DNA::G) => Some(19),
            Codon(DNA::C, DNA::C, DNA::T) => Some(20),
            Codon(DNA::C, DNA::C, DNA::C) => Some(21),
            Codon(DNA::C, DNA::C, DNA::A) => Some(22),
            Codon(DNA::C, DNA::C, DNA::G) => Some(23),
            Codon(DNA::C, DNA::A, DNA::T) => Some(24),
            Codon(DNA::C, DNA::A, DNA::C) => Some(25),
            Codon(DNA::C, DNA::A, DNA::A) => Some(26),
            Codon(DNA::C, DNA::A, DNA::G) => Some(27),
            Codon(DNA::C, DNA::G, DNA::T) => Some(28),
            Codon(DNA::C, DNA::G, DNA::C) => Some(29),
            Codon(DNA::C, DNA::G, DNA::A) => Some(30),
            Codon(DNA::C, DNA::G, DNA::G) => Some(31),
            Codon(DNA::A, DNA::T, DNA::T) => Some(32),
            Codon(DNA::A, DNA::T, DNA::C) => Some(33),
            Codon(DNA::A, DNA::T, DNA::A) => Some(34),
            Codon(DNA::A, DNA::T, DNA::G) => Some(35),
            Codon(DNA::A, DNA::C, DNA::T) => Some(36),
            Codon(DNA::A, DNA::C, DNA::C) => Some(37),
            Codon(DNA::A, DNA::C, DNA::A) => Some(38),
            Codon(DNA::A, DNA::C, DNA::G) => Some(39),
            Codon(DNA::A, DNA::A, DNA::T) => Some(40),
            Codon(DNA::A, DNA::A, DNA::C) => Some(41),
            Codon(DNA::A, DNA::A, DNA::A) => Some(42),
            Codon(DNA::A, DNA::A, DNA::G) => Some(43),
            Codon(DNA::A, DNA::G, DNA::T) => Some(44),
            Codon(DNA::A, DNA::G, DNA::C) => Some(45),
            Codon(DNA::A, DNA::G, DNA::A) => Some(46),
            Codon(DNA::A, DNA::G, DNA::G) => Some(47),
            Codon(DNA::G, DNA::T, DNA::T) => Some(48),
            Codon(DNA::G, DNA::T, DNA::C) => Some(49),
            Codon(DNA::G, DNA::T, DNA::A) => Some(50),
            Codon(DNA::G, DNA::T, DNA::G) => Some(51),
            Codon(DNA::G, DNA::C, DNA::T) => Some(52),
            Codon(DNA::G, DNA::C, DNA::C) => Some(53),
            Codon(DNA::G, DNA::C, DNA::A) => Some(54),
            Codon(DNA::G, DNA::C, DNA::G) => Some(55),
            Codon(DNA::G, DNA::A, DNA::T) => Some(56),
            Codon(DNA::G, DNA::A, DNA::C) => Some(57),
            Codon(DNA::G, DNA::A, DNA::A) => Some(58),
            Codon(DNA::G, DNA::A, DNA::G) => Some(59),
            Codon(DNA::G, DNA::G, DNA::T) => Some(60),
            Codon(DNA::G, DNA::G, DNA::C) => Some(61),
            Codon(DNA::G, DNA::G, DNA::A) => Some(62),
            Codon(DNA::G, DNA::G, DNA::G) => Some(63),
            _ => None
        }
    }

    pub fn translate(&self, table: &TranslationTable) -> TranslatedCodon {
        match self.get_translation_index() {
            Some(idx) => table.get(idx),
            None => TranslatedCodon::X,
        }
    }
}

impl TryFrom<&[char]> for Codon {
    type Error = ();

    fn try_from(value: &[char]) -> Result<Self, Self::Error> {
        match value {
            [a, b, c] => Ok(Codon(DNA::from(a), DNA::from(b), DNA::from(c))),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::DNA::{T,A,C,G};
    use ::seq::translation::ncbi_translation_tables::STANDARD;
    use ::seq::translation::TranslatedCodon;

    #[test]
    fn test_table_integration() {
        let methionine = Codon(A,T,G).translate(&STANDARD);
        assert_eq!(methionine, TranslatedCodon::M);
    }
}