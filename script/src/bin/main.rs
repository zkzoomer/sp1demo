use alloy_sol_types::SolType;
use clap::Parser;
use sp1_sdk::{ProverClient, SP1Stdin};
use spdemo_lib::PublicValuesStruct;

pub const SPDEMO_ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,
    #[clap(long)]
    prove: bool,
    #[clap(long, default_value = "20")]
    x: u32,
    #[clap(long, default_value = "20")]
    y: u32,
}

fn main() {
    sp1_sdk::utils::setup_logger();

    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    let client = ProverClient::new();

    let mut stdin = SP1Stdin::new();
    stdin.write(&args.x);
    stdin.write(&args.y);

    println!("x: {}", args.x);
    println!("y: {}", args.y);

    if args.execute {
        let res = client.execute(SPDEMO_ELF, stdin).run();
        println!("Result: {res:?}");
        let (output, report) = res.unwrap();
        println!("Program executed successfully");

        let decoded = PublicValuesStruct::abi_decode(output.as_slice(), true).unwrap();
        let PublicValuesStruct { x, y, z } = decoded;
        println!("x: {}", x);
        println!("y: {}", y);
        println!("z: {}", z);

        let expected_z = spdemo_lib::f(x, y);
        assert_eq!(z, expected_z);
        println!("Value is correct!");

        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        let (pk, vk) = client.setup(SPDEMO_ELF);

        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
