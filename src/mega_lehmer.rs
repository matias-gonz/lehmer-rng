use crate::felt::FeltTrait;
use multiplier_generator::time_multiplier;
use seed_generators::time_seed;

pub struct MegaLehmer<F: FeltTrait> {
    multiplier: F,
    last_gen: F,
}

impl<F: FeltTrait> MegaLehmer<F> {
    pub fn new(seed: Option<F>, multiplier: Option<F>) -> MegaLehmer<F> {
        let seed = match seed {
            Some(seed) => seed,
            None => time_seed(),
        };
        let multiplier = match multiplier {
            Some(multiplier) => multiplier,
            None => time_multiplier(),
        };

        MegaLehmer {
            multiplier: multiplier,
            last_gen: seed,
        }
    }

    pub fn gen(&mut self) -> F {
        self.last_gen = self.last_gen * self.multiplier;
        self.last_gen
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::felt::Felt;

    type Felt17 = Felt<17>;

    #[test]
    fn test_gen() {
        let mut lehmer = MegaLehmer::new(Some(Felt17::new(1)), Some(Felt17::new(2)));
        assert_eq!(lehmer.gen(), Felt17::new(2));
        assert_eq!(lehmer.gen(), Felt17::new(4));
        assert_eq!(lehmer.gen(), Felt17::new(8));
        assert_eq!(lehmer.gen(), Felt17::new(16));
        assert_eq!(lehmer.gen(), Felt17::new(15));
        assert_eq!(lehmer.gen(), Felt17::new(13));
        assert_eq!(lehmer.gen(), Felt17::new(9));
    }
}
