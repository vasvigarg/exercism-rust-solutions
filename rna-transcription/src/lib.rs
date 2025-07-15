#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    sequence: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    sequence: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, ch) in dna.chars().enumerate() {
            if !matches!(ch, 'A' | 'C' | 'G' | 'T') {
                return Err(i);
            }
        }
        Ok(Dna {
            sequence: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        let transcribed = self
            .sequence
            .chars()
            .map(|ch| match ch {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect();
        Rna { sequence: transcribed }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, ch) in rna.chars().enumerate() {
            if !matches!(ch, 'A' | 'C' | 'G' | 'U') {
                return Err(i);
            }
        }
        Ok(Rna {
            sequence: rna.to_string(),
        })
    }
}
