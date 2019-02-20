use std::ops::{BitAnd, BitOr, BitXor};

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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TranslatedCodon {
    A, // Ala - Alanine
    C, // Cys - Cysteine
    D, // Asp - Aspartic Acid
    E, // Glu - Glutamic Acid
    F, // Phe - Phenylalanine
    G, // Gly - Glycine
    H, // His - Histidine
    I, // Ile - Isoleucine
    K, // Lys - Lysine
    L, // Leu - Leucine
    M, // Met - Methionine
    N, // Asn - Asparagine
    P, // Pro - Proline
    Q, // Gln - Glutamine
    R, // Arg - Arginine
    S, // Ser - Serine
    T, // Thr - Threonine
    V, // Val - Valine
    W, // Trp - Tryptophan
    Y, // Tyr - Tyrosine
    X, // Any amino acid
    Stop, // Terminator
}

impl From<char> for TranslatedCodon {
    fn from(c: char) -> Self {
        match c {
            'A' => TranslatedCodon::A,
            'C' => TranslatedCodon::C,
            'D' => TranslatedCodon::D,
            'E' => TranslatedCodon::E,
            'F' => TranslatedCodon::F,
            'G' => TranslatedCodon::G,
            'H' => TranslatedCodon::H,
            'I' => TranslatedCodon::I,
            'K' => TranslatedCodon::K,
            'L' => TranslatedCodon::L,
            'M' => TranslatedCodon::M,
            'N' => TranslatedCodon::N,
            'P' => TranslatedCodon::P,
            'Q' => TranslatedCodon::Q,
            'R' => TranslatedCodon::R,
            'S' => TranslatedCodon::S,
            'T' => TranslatedCodon::T,
            'V' => TranslatedCodon::V,
            'W' => TranslatedCodon::W,
            'Y' => TranslatedCodon::Y,
            '*' => TranslatedCodon::Stop,
            _ => TranslatedCodon::X,
        }
    }
}

pub struct TranslationTable([TranslatedCodon; 64]);

impl TranslationTable {
    pub fn new(amino_acids: [char; 64]) -> Self {
        TranslationTable([
            TranslatedCodon::from(amino_acids[00]),
            TranslatedCodon::from(amino_acids[01]),
            TranslatedCodon::from(amino_acids[02]),
            TranslatedCodon::from(amino_acids[03]),
            TranslatedCodon::from(amino_acids[04]),
            TranslatedCodon::from(amino_acids[05]),
            TranslatedCodon::from(amino_acids[06]),
            TranslatedCodon::from(amino_acids[07]),
            TranslatedCodon::from(amino_acids[08]),
            TranslatedCodon::from(amino_acids[09]),
            TranslatedCodon::from(amino_acids[10]),
            TranslatedCodon::from(amino_acids[11]),
            TranslatedCodon::from(amino_acids[12]),
            TranslatedCodon::from(amino_acids[13]),
            TranslatedCodon::from(amino_acids[14]),
            TranslatedCodon::from(amino_acids[15]),
            TranslatedCodon::from(amino_acids[16]),
            TranslatedCodon::from(amino_acids[17]),
            TranslatedCodon::from(amino_acids[18]),
            TranslatedCodon::from(amino_acids[19]),
            TranslatedCodon::from(amino_acids[20]),
            TranslatedCodon::from(amino_acids[21]),
            TranslatedCodon::from(amino_acids[22]),
            TranslatedCodon::from(amino_acids[23]),
            TranslatedCodon::from(amino_acids[24]),
            TranslatedCodon::from(amino_acids[25]),
            TranslatedCodon::from(amino_acids[26]),
            TranslatedCodon::from(amino_acids[27]),
            TranslatedCodon::from(amino_acids[28]),
            TranslatedCodon::from(amino_acids[29]),
            TranslatedCodon::from(amino_acids[30]),
            TranslatedCodon::from(amino_acids[31]),
            TranslatedCodon::from(amino_acids[32]),
            TranslatedCodon::from(amino_acids[33]),
            TranslatedCodon::from(amino_acids[34]),
            TranslatedCodon::from(amino_acids[35]),
            TranslatedCodon::from(amino_acids[36]),
            TranslatedCodon::from(amino_acids[37]),
            TranslatedCodon::from(amino_acids[38]),
            TranslatedCodon::from(amino_acids[39]),
            TranslatedCodon::from(amino_acids[40]),
            TranslatedCodon::from(amino_acids[41]),
            TranslatedCodon::from(amino_acids[42]),
            TranslatedCodon::from(amino_acids[43]),
            TranslatedCodon::from(amino_acids[44]),
            TranslatedCodon::from(amino_acids[45]),
            TranslatedCodon::from(amino_acids[46]),
            TranslatedCodon::from(amino_acids[47]),
            TranslatedCodon::from(amino_acids[48]),
            TranslatedCodon::from(amino_acids[49]),
            TranslatedCodon::from(amino_acids[50]),
            TranslatedCodon::from(amino_acids[51]),
            TranslatedCodon::from(amino_acids[52]),
            TranslatedCodon::from(amino_acids[53]),
            TranslatedCodon::from(amino_acids[54]),
            TranslatedCodon::from(amino_acids[55]),
            TranslatedCodon::from(amino_acids[56]),
            TranslatedCodon::from(amino_acids[57]),
            TranslatedCodon::from(amino_acids[58]),
            TranslatedCodon::from(amino_acids[59]),
            TranslatedCodon::from(amino_acids[60]),
            TranslatedCodon::from(amino_acids[61]),
            TranslatedCodon::from(amino_acids[62]),
            TranslatedCodon::from(amino_acids[63]),
        ])
    }

    pub fn get(&self, idx: usize) -> TranslatedCodon {
        self.0[idx].clone()
    }
}

#[derive(Debug)]
pub struct Codon(DNA, DNA, DNA);

impl Codon {
    pub fn get_translation_index(self) -> Option<usize> {
        // What does the first codon look like?
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
}


#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn codon_translation_index() {
        assert_eq!(Codon(DNA::A, DNA::T, DNA::G).get_translation_index(), Some(35));
    }

    #[test]
    fn codon_translation_table_construction() {
        let ncbi_default = TranslationTable::new(['F','F','L','L','S','S','S','S','Y','Y','*','*','C','C','*','W','L','L','L','L','P','P','P','P','H','H','Q','Q','R','R','R','R','I','I','I','M','T','T','T','T','N','N','K','K','S','S','R','R','V','V','V','V','A','A','A','A','D','D','E','E','G','G','G','G']);
        assert_eq!(ncbi_default.get(35), TranslatedCodon::M);
    }
}

// Here is where I define sequences and alphabets.
// All I really need at the moment is some code that allows for translation from DNA to protein.
// This actually seems slightly more difficult than I was imagining. I don't think I have a good
// handle on traits yet, which might prove helpful. I think that the best I can do is


// We can take some inspiration from the rust-bio implementation, and even perhaps improve on it.
// The rust-bio authors define functions to take an alphabet and

// Should a translation table be specific to a particular alphabet. A translation is basically
// a function that maps one alphabet to another. One wrinkle is that we need to map DNA bases ->
// codon triplets -> Protein amino acids.

// Would it be terribly slow to pack two nucleotides per byte? If we're only considering acgt, then
// we could actually do four bases per byte (2 bits each), but that would make it impossible to
// handle Ns, which occur too frequently to be ignored. Ideally, it would take three bits per base,
// but that might get a little bit tricky? Would it? If we take four bits, we could (theoretically)
// make the complement as simple as flipping all of the bits.

// The first two bits could specify ACGT, and the second two bits might specify the N-ness. For
// example, for the first two bits:
// 00 = A
// 01 = C
// 10 = G
// 11 = T

// 00 = not-n
// 01 = N
// 10 = N
// 11 = not-n


