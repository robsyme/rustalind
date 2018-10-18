use std::io::BufRead;
use std::io;

pub struct Record {
    id: String,
    seq: String,
}

impl Record {
    pub fn new() -> Self {
        Record {
            id: String::new(),
            seq: String::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.id.is_empty() && self.seq.is_empty()
    }

    fn clear(&mut self) {
        self.id.clear();
        self.seq.clear();
    }

    pub fn gc_count(&self) -> usize {
        self.seq
            .chars()
            .filter(|base| *base == 'C' || *base == 'G')
            .count()
    }

    pub fn gc_percent(&self) -> f64 {
        self.gc_count() as f64 / self.seq.len() as f64
    }

    pub fn id(&self) -> &String {
        &self.id
    }
}

/// A FASTA reader.
pub struct FastaReader<R: io::Read> {
    reader: io::BufReader<R>,
    line: String,
}

impl<R: io::Read> FastaReader<R> {
    pub fn new(reader: R) -> Self {
        FastaReader {
            reader: io::BufReader::new(reader),
            line: Default::default(),
        }
    }

    pub fn read(&mut self, record: &mut Record) -> io::Result<()> {
        record.clear();

        if self.line.is_empty() {
            self.reader.read_line(&mut self.line)?;
            if self.line.is_empty() {
                return Ok(());
            }
        }

        // If the line isn't a header...
        if !self.line.starts_with('>') {
            // Loop over the lines until we find something interesting.
            loop {
                self.line.clear();
                self.reader.read_line(&mut self.line)?;
                if self.line.starts_with('>') {
                    break;
                } else if self.line.is_empty() {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput,
                    "Could not find a valid fasta entry in the file"))
                }
            }
            return Err(io::Error::new(io::ErrorKind::Other, "Expected '>' at record start"));
        }

        record.id = self.line[1..]
            .trim_right()
            .splitn(2, ' ')
            .nth(0)
            .map(|s| s.to_owned())
            .unwrap();

        loop {
            self.line.clear();
            self.reader.read_line(&mut self.line)?;
            if self.line.is_empty() || self.line.starts_with('>') {
                break;
            }
            record.seq.push_str(self.line.trim_right());
        }

        Ok(())
    }
}

impl<T: io::Read> Iterator for FastaReader<T> {
    type Item = io::Result<Record>;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut record = Record::new();
        match self.read(&mut record) {
            // If the read failed, return the error.
            Err(err) => Some(Err(err)),
            // If no data was written to record, we've reached EOF and can stop the iteration by returning None.
            Ok(()) if record.is_empty() => None,
            // Otherwise, return the record
            Ok(()) => Some(Ok(record)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_FASTA_FORMAT: &'static [u8] = b">id desc
ACCGTAGGCTGA
CCGTAGGCTGAA
CGTAGGCTGAAA
GTAGGCTGAAAA
CCCC
>id2
ATTGTTGTTTTA
ATTGTTGTTTTA
ATTGTTGTTTTA
GGGG
";

    const EMPTY_FASTA_FILE: &'static [u8] = b"";

    const ERRONEOUS_FASTA_FORMAT_01: &'static [u8] = b"
these lines at the beginning
are just junk that should
be ignored

>id desc
ACCGTAGGCTGA
CCGTAGGCTGAA
CGTAGGCTGAAA
GTAGGCTGAAAA
CCCC
>id2
ATTGTTGTTTTA
ATTGTTGTTTTA
ATTGTTGTTTTA
GGGG
";

    #[test]
    fn blank_fasta() {
        let mut blank_reader = FastaReader::new(EMPTY_FASTA_FILE);
        assert!(blank_reader.next().is_none());
    }

    #[test]
    fn erroneous_fasta() {
        let reader = FastaReader::new(ERRONEOUS_FASTA_FORMAT_01);
        let record_count = reader
            .filter_map(Result::ok)
            .count();
        assert_eq!(2, record_count);
    }

    #[test]
    fn test_gc_counting() {
        let a: Vec<usize> = FastaReader::new(BASIC_FASTA_FORMAT)
            .filter_map(Result::ok)
            .map(|rec: Record| rec.gc_count())
            .collect();
        assert_eq!(a, vec![29, 10]);
    }
}