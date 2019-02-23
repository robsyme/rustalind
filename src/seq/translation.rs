#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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

impl From<TranslatedCodon> for char {
    fn from(codon: TranslatedCodon) -> Self {
        match codon {
            TranslatedCodon::A => 'A',
            TranslatedCodon::C => 'C',
            TranslatedCodon::D => 'D',
            TranslatedCodon::E => 'E',
            TranslatedCodon::F => 'F',
            TranslatedCodon::G => 'G',
            TranslatedCodon::H => 'H',
            TranslatedCodon::I => 'I',
            TranslatedCodon::K => 'K',
            TranslatedCodon::L => 'L',
            TranslatedCodon::M => 'M',
            TranslatedCodon::N => 'N',
            TranslatedCodon::P => 'P',
            TranslatedCodon::Q => 'Q',
            TranslatedCodon::R => 'R',
            TranslatedCodon::S => 'S',
            TranslatedCodon::T => 'T',
            TranslatedCodon::V => 'V',
            TranslatedCodon::W => 'W',
            TranslatedCodon::Y => 'Y',
            TranslatedCodon::Stop => '*',
            _ => 'X',
        }
    }
}

impl Default for TranslatedCodon {
    fn default() -> Self {
        TranslatedCodon::X
    }
}

pub struct TranslationTable([TranslatedCodon; 64]);

impl TranslationTable {
    pub fn get(&self, idx: usize) -> TranslatedCodon {
        match self.0.get(idx) {
            None => TranslatedCodon::default(),
            Some(codon) => codon.clone(),
        }
    }
}

pub mod ncbi_translation_tables {
    use seq::translation::{TranslationTable,TranslatedCodon::*};
    pub static STANDARD: TranslationTable = TranslationTable([F,F,L,L,S,S,S,S,Y,Y,Stop,Stop,C,C,Stop,W,L,L,L,L,P,P,P,P,H,H,Q,Q,R,R,R,R,I,I,I,M,T,T,T,T,N,N,K,K,S,S,R,R,V,V,V,V,A,A,A,A,D,D,E,E,G,G,G,G]);
    pub static VERTEBRATE_MITOCHONDRIAL: TranslationTable = TranslationTable([F,F,L,L,S,S,S,S,Y,Y,Stop,Stop,C,C,W,W,L,L,L,L,P,P,P,P,H,H,Q,Q,R,R,R,R,I,I,M,M,T,T,T,T,N,N,K,K,S,S,Stop,Stop,V,V,V,V,A,A,A,A,D,D,E,E,G,G,G,G]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::ncbi_translation_tables::*;

    #[test]
    fn test_get() {
        assert_eq!(STANDARD.get(0), TranslatedCodon::F);
        assert_eq!(STANDARD.get(9999), TranslatedCodon::X);
    }
}