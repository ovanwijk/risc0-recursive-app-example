

use risc0_zkvm::{guest::env, serde};

fn main() {
   
    let (image_id, counter): ([u32; 8], u64) = env::read();
    
    if counter == 0 {        
        env::commit(&(image_id, 1u64));
        return;
    }    
    env::verify(image_id, &serde::to_vec(&(image_id, counter)).unwrap()).unwrap();    
    env::commit(&(image_id, (counter + 1) as u64));
    println!("{:?}", env::cycle_count());
    
}
