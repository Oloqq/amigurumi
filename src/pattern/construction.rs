use std::fs;
use std::path::PathBuf;

use super::{Pattern, Stitch};

impl Pattern {
    pub fn from_file(path: PathBuf) -> Self {
        println!("{path:?}");
        let content = fs::read_to_string(&path).expect("File not found");
        let extension = path.extension().expect("Unrecognized format");
        match extension.to_str().unwrap() {
            "yaml" => Self::from_yaml_str(content.as_str()),
            _ => panic!("Unrecognized format"),
        }
    }

    pub fn from_yaml_str(content: &str) -> Self {
        serde_yaml::from_str(&content).expect("Could not parse yaml into pattern")
    }
}

pub struct PatternBuilder {
    starting_ring: usize,
    rounds: Vec<Vec<Stitch>>,
    has_error: Option<(usize, String)>,
    pub warnings: Vec<(usize, String)>,
}

impl PatternBuilder {
    pub fn new(starting_ring: usize) -> Self {
        Self {
            starting_ring,
            rounds: vec![],
            has_error: None,
            warnings: vec![],
        }
    }

    fn stitches_to_fill(&self) -> usize {
        match self.rounds.last() {
            Some(round) => round.len(),
            None => self.starting_ring,
        }
    }

    fn _error(&mut self, msg: String) {
        if self.has_error.is_none() {
            self.has_error = Some((self.rounds.len() + 1, msg));
        }
    }

    fn warn(&mut self, msg: String) {
        self.warnings.push((self.rounds.len() + 1, msg));
    }

    pub fn round_like(mut self, repeat_this: &Vec<Stitch>) -> Self {
        let stitches = self.stitches_to_fill();
        let repeats = stitches / repeat_this.len();
        let leftover = stitches % repeat_this.len();
        if leftover != 0 {
            self.warn(format!("Pattern won't be fully repeated in the row. Length of previous round: {}, length of the pattern: {}", stitches, repeat_this.len()))
        }
        let full_reps = repeat_this.iter().cycle().take(repeat_this.len() * repeats);
        let partial_rep = repeat_this.iter().take(leftover);

        self.rounds
            .push(full_reps.chain(partial_rep).cloned().collect());

        self
    }

    pub fn full_rounds(mut self, num: usize) -> Self {
        for _ in 0..num {
            self.rounds.push(
                (0..self.stitches_to_fill())
                    .map(|_| Stitch::Single)
                    .collect(),
            );
        }
        self
    }

    pub fn build(self) -> Result<Pattern, (usize, String)> {
        if let Some(error) = self.has_error {
            return Err(error);
        }
        let last_round = match self.rounds.last() {
            Some(round) => round,
            None => return Err((0, "Pattern must have at least one round".into())),
        };

        Ok(Pattern {
            starting_circle: self.starting_ring,
            ending_circle: last_round.len(),
            rounds: self.rounds,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use Stitch::*;

    #[test]
    fn test_detects_no_rounds() {
        assert!(PatternBuilder::new(6).build().is_err());
    }

    #[test]
    fn test_full_round() {
        let mut p = PatternBuilder::new(6);
        assert_eq!(p.rounds.len(), 0);
        p = p.full_rounds(2);
        assert_eq!(p.rounds.len(), 2);
        assert_eq!(p.rounds[0].len(), 6);
        assert_eq!(p.rounds[1].len(), 6);
        let pat = p.build().unwrap();
        assert_eq!(pat.ending_circle, 6);
    }

    #[test]
    fn test_round_like() {
        let mut p = PatternBuilder::new(6);
        let single_6 = vec![Single, Single, Single, Single, Single, Single];
        p = p.round_like(&single_6);
        assert_eq!(p.rounds.len(), 1);
        assert_eq!(p.rounds[0], single_6);

        p = p.round_like(&vec![Single, Single, Single]);
        assert_eq!(p.rounds.len(), 2);
        assert_eq!(p.rounds[1], single_6);

        p = p.round_like(&vec![Single, Single, Increase]);
        assert_eq!(p.rounds.len(), 3);
        assert_eq!(
            p.rounds[2],
            vec![Single, Single, Increase, Single, Single, Increase]
        );
    }

    #[test]
    fn test_round_like_with_leftovers() {
        let mut p = PatternBuilder::new(3);
        p = p.round_like(&vec![Single, Single]);
        assert_eq!(p.rounds.len(), 1);
        assert_eq!(p.rounds[0], vec![Single, Single, Single]);
        assert_eq!(p.warnings.len(), 1);
        assert!(p.warnings[0].0 == 1)
    }

    #[test]
    #[ignore]
    fn test_round_like_with_decrease() {
        let mut p = PatternBuilder::new(3);
        p = p.round_like(&vec![Single, Decrease]);
        assert_eq!(p.rounds.len(), 1);
        assert_eq!(p.rounds[0], vec![Single, Decrease]);
        assert_eq!(p.warnings.len(), 0);
    }

    #[test]
    #[ignore]
    fn test_decrease_overflowing_the_round() {
        todo!()
    }
}
