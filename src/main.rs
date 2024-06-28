mod felt;
mod mega_lehmer;
mod multiplier_generator;
mod seed_generators;

use felt::Felt;
use multiplier_generator::time_multiplier;
use seed_generators::time_seed;

type Felt17 = Felt<17>;

fn main() {
    let user_seed = 1234567890 as u64;
    let felt_with_user_seed = time_seed::<Felt17>(Some(user_seed));
    println!("Felt USER seed: {:?}", felt_with_user_seed);

    let felt_with_time_seed = time_seed::<Felt17>(None);
    println!("Felt COMPUTER seed: {:?}", felt_with_time_seed);
    // -----------------------------------------------------------------
    let user_seed = 0987654321 as u64;
    let felt_with_user_seed = time_multiplier::<Felt17>(Some(user_seed));
    println!("Felt USER multiplier: {:?}", felt_with_user_seed);

    let felt_with_time_seed = time_multiplier::<Felt17>(None);
    println!("Felt COMPUTER multiplier: {:?}", felt_with_time_seed);
}
