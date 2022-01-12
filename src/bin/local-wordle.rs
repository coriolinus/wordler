use wordler::{
    oracle::memory_oracle::MemoryOracle, petitioner::human_petitioner::HumanPetitioner, wordle,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let answer = wordle::<MemoryOracle, HumanPetitioner>(true)?;
    println!("you win! ({})", answer);
    Ok(())
}
