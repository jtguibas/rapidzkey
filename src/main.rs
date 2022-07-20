use ark_bn254::Bn254;
use ark_circom::generate_random_zkey;
use ark_circom::{CircomBuilder, CircomConfig};
use clap::{App, Arg};
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::fs::File;
use std::time::Instant;

fn main() {
    let matches = App::new("rapidzkey")
        .version("0.1.0")
        .about("fast zkey generation from circom r1cs")
        .arg(
            Arg::with_name("r1cs_path")
                .help("r1cs file path")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("zkey_path")
                .help("output zkey file path")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let r1cs_path = matches.value_of("r1cs_path").unwrap();
    let zkey_path = matches.value_of("zkey_path").unwrap();

    let start = Instant::now();

    let cfg = CircomConfig::<Bn254>::new_without_witness(r1cs_path).unwrap();
    let builder = CircomBuilder::new(cfg);
    let circuit = builder.setup();

    let mut rng = ChaCha8Rng::seed_from_u64(1337);
    let mut writer = File::create(zkey_path).unwrap();
    let _ = generate_random_zkey(circuit.clone(), &mut rng, &mut writer).unwrap();

    let duration = start.elapsed();
    println!("zkey generation took: {:?}", duration);
}
