use crate::felt::FeltTrait;
extern crate chrono;

/// this highly depends, when you start the generator!
pub fn time_multiplier<F: FeltTrait>() -> F {
    let now = chrono::Local::now();
    let midnight = chrono::NaiveTime::from_hms(0, 0, 0);
    let time_since_midnight = now.time() - midnight;
    
    F::new(time_since_midnight.num_milliseconds() as u64)
}

// pub fn time_modified_seed(seed: F) -> MegaLehmerSeed<F> {
//     MegaLehmerSeed { seed: seed }
// }
// ...
