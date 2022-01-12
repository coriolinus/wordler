use wordler::{
    oracle::memory_oracle::MemoryOracle, petitioner::dict_solver::DictSolver, wordle_config,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let answer = wordle_config::<MemoryOracle, DictSolver, _, _>(
        true,
        |oracle| oracle.max_guesses = Some(6),
        |_| {},
    )?;
    println!("bot solver wins! ({})", answer);
    Ok(())
}
