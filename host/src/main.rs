
// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
// use methods::{
//     MERKLE_PROOF_ELF, MERKLE_PROOF_ID
// };
// use rayon::prelude::*;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, Receipt};
use methods::{RECURSIVE_COUNTER_ELF, RECURSIVE_COUNTER_ID};
use std::time::Instant;






fn main() {
    

    
    //Setup the initial counter value and pass the image ID
    let mut env = ExecutorEnv::builder()
        .write(&(&RECURSIVE_COUNTER_ID, 0u64))
        .unwrap()
        .build()
        .unwrap();

    let elf = RECURSIVE_COUNTER_ELF;

    let prover = default_prover();
    //To make this work we need succinct prover options.
    let prover_opts: ProverOpts = ProverOpts::succinct();
    let mut receipt = prover.prove_with_opts(env, elf, &prover_opts).unwrap().receipt;
    
    let mut journal: ([u32;8], u64) = receipt.journal.decode().unwrap();
    
    let mut counter = 0;
    loop {
      
        let start = Instant::now();
        //continue the counter
        env = ExecutorEnv::builder()        
        .add_assumption(receipt)
        .write(&(RECURSIVE_COUNTER_ID, journal.1))
        .unwrap()
        .build()
        .unwrap();
        receipt = prover.prove_with_opts(env, elf, &prover_opts).unwrap().receipt;
       
        journal = receipt.journal.decode().unwrap();
        
        receipt.verify(RECURSIVE_COUNTER_ID).unwrap();
        counter += 1;
        println!("RUN: {:?} - ZK value: {:?}", counter, journal.1);
        let duration = start.elapsed();
        println!("Time taken: {:?}", duration);
        if counter >= 257 {
            return
        }
    }
    
    
    
}
