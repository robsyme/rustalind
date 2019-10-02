use std::ops::{BitAnd, BitOr, BitXor};
use std::convert::TryFrom;

///
/// We can probably encode everything in four bits like so:
///
/// A C T G
/// 1 0 0 0 → A
/// 0 1 0 0 → C
/// 0 0 1 0 → T
/// 0 0 0 1 → G
/// -------
/// 1 0 0 1 → R
/// 0 1 1 0 → Y
/// 0 1 0 1 → S
/// 1 0 1 0 → W
/// 0 0 1 1 → K
/// 1 1 0 0 → M
/// 0 1 1 1 → B
/// 1 0 1 1 → D
/// 1 1 1 0 → H
/// 1 1 0 1 → V
/// 1 1 1 1 → N
///
/// To take the complement or any base, we just rotate_left (or rotate_right) 2 bits.
///
/// 1 0 0 0 (A)
///
/// 0 0 1 0 (T)
///
/// To take the complement or any base, we just rotate_left (or rotate_right) 2 bits.
///
///
/// etc..
///
#[derive(Debug, Eq, PartialEq)]
pub enum DNA {
    // G A C T
    T = 0b0001, // 0 0 0 1
    C = 0b0010, // 0 0 1 0
    A = 0b0100, // 0 1 0 0
    G = 0b1000, // 1 0 0 0

    Y = 0b0011, // 3
    W = 0b0101, // 5
    M = 0b0110, // 6
    K = 0b1001, // 9
    S = 0b1010, // 10
    R = 0b1100, // 12

    H = 0b0111, // 7
    B = 0b1011, // 11
    D = 0b1101, // 13
    V = 0b1110, // 14

    N = 0b1111, // 15
    Gap = 0b00,  // No base
}

impl From<u8> for DNA {
    fn from(i: u8) -> Self {
        match i & 0b1111 {
            0  => DNA::Gap,
            1  => DNA::T,
            2  => DNA::C,
            3  => DNA::Y,
            4  => DNA::A,
            5  => DNA::W,
            6  => DNA::M,
            7  => DNA::H,
            8  => DNA::G,
            9  => DNA::K,
            10 => DNA::S,
            11 => DNA::B,
            12 => DNA::R,
            13 => DNA::D,
            14 => DNA::V,
            _  => DNA::N,
        }
    }
}

impl From<char> for DNA {
    fn from(c: char) -> Self {
        match c  {
            'a' => DNA::A,
            'A' => DNA::A,
            'c' => DNA::C,
            'C' => DNA::C,
            'g' => DNA::G,
            'G' => DNA::G,
            't' => DNA::T,
            'T' => DNA::T,
            _ => DNA::N,
        }
    }
}

impl From<&char> for DNA {
    fn from(c: &char) -> Self {
        Self::from(*c)
    }
}

impl BitAnd for DNA {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        DNA::from(self as u8 & rhs as u8)
    }
}

impl BitOr for DNA {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        DNA::from(self as u8 | rhs as u8)
    }
}

impl BitXor for DNA {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        DNA::from(self as u8 ^ rhs as u8)
    }
}

impl DNA {
    pub fn complement(self) -> Self {
        let bits = self as u8;
        DNA::from(bits << 2 | bits >> 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use seq::codon::Codon;
    use seq::translation::ncbi_translation_tables::STANDARD;
    use seq::translation::TranslatedCodon;

    #[test]
    fn dna_as_tagged_enums() {
        assert_eq!(DNA::T as u8, 1);
        assert_eq!(DNA::C as u8, 2);
        assert_eq!(DNA::A as u8, 4);
        assert_eq!(DNA::G as u8, 8);
    }

    #[test]
    fn dna_from_chars() {
        assert_eq!(DNA::from('A'), DNA::A);
        assert_eq!(DNA::from('a'), DNA::A);
        assert_eq!(DNA::from('c'), DNA::C);
        assert_eq!(DNA::from('C'), DNA::C);
        assert_eq!(DNA::from('g'), DNA::G);
        assert_eq!(DNA::from('G'), DNA::G);
        assert_eq!(DNA::from('t'), DNA::T);
        assert_eq!(DNA::from('T'), DNA::T);
        assert_eq!(DNA::from('k'), DNA::N);
        assert_eq!(DNA::from('Z'), DNA::N);
        assert_eq!(DNA::from('4'), DNA::N);
    }

    #[test]
    fn dna_bit_or() {
        assert_eq!(DNA::A | DNA::A, DNA::A);
        assert_eq!(DNA::C | DNA::C, DNA::C);
        assert_eq!(DNA::G | DNA::G, DNA::G);
        assert_eq!(DNA::T | DNA::T, DNA::T);

        // Testing IUPAC ambiguities
        assert_eq!(DNA::A | DNA::T, DNA::W);
        assert_eq!(DNA::T | DNA::A, DNA::W);
        assert_eq!(DNA::G | DNA::C, DNA::S);
        assert_eq!(DNA::C | DNA::G, DNA::S);
        assert_eq!(DNA::T | DNA::G, DNA::K);
        assert_eq!(DNA::G | DNA::T, DNA::K);
        assert_eq!(DNA::A | DNA::G, DNA::R);
        assert_eq!(DNA::G | DNA::A, DNA::R);
        assert_eq!(DNA::C | DNA::T, DNA::Y);
        assert_eq!(DNA::T | DNA::C, DNA::Y);
    }

    #[test]
    fn dna_bit_and() {
        assert_eq!(DNA::W & DNA::M, DNA::A);
        assert_eq!(DNA::Y & DNA::R, DNA::Gap);
        assert_eq!(DNA::S & DNA::Y, DNA::C);
        assert_eq!(DNA::N & DNA::N, DNA::N);
        assert_eq!(DNA::N & DNA::A, DNA::A);
    }

    #[test]
    fn dna_bit_xor() {
        assert_eq!(DNA::W ^ DNA::Y, DNA::M);
        assert_eq!(DNA::T ^ DNA::W, DNA::A);
        assert_eq!(DNA::T ^ DNA::S, DNA::B);
    }

    #[test]
    fn dna_complement() {
        assert_eq!(DNA::T.complement(), DNA::A);
        assert_eq!(DNA::A.complement(), DNA::T);
        assert_eq!(DNA::G.complement(), DNA::C);
        assert_eq!(DNA::C.complement(), DNA::G);
        assert_eq!(DNA::N.complement(), DNA::N);

        assert_eq!(DNA::W.complement(), DNA::W);
        assert_eq!(DNA::S.complement(), DNA::S);
        assert_eq!(DNA::K.complement(), DNA::M);
    }

    //noinspection ALL
    #[test]
    fn dna_from_string() {
        let input = String::from("ATGGCCATGGCGCCCAGAACTGAGATCAATAGTACCCGTATTAACGGGTGA");
        let aa_seq: Vec<TranslatedCodon> = input
            .chars()
            .collect::<Vec<char>>()[..]
            .chunks_exact(3)
            .map(Codon::try_from)
            .filter_map(Result::ok)
            .map(|codon| codon.translate(&STANDARD))
            .collect();
        assert_eq!(aa_seq, vec![
            TranslatedCodon::M,
            TranslatedCodon::A,
            TranslatedCodon::M,
            TranslatedCodon::A,
            TranslatedCodon::P,
            TranslatedCodon::R,
            TranslatedCodon::T,
            TranslatedCodon::E,
            TranslatedCodon::I,
            TranslatedCodon::N,
            TranslatedCodon::S,
            TranslatedCodon::T,
            TranslatedCodon::R,
            TranslatedCodon::I,
            TranslatedCodon::N,
            TranslatedCodon::G,
            TranslatedCodon::Stop]);
    }
}