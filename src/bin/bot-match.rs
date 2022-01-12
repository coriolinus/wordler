use wordler::{
    oracle::{memory_oracle::MemoryOracle, Disposition},
    petitioner::dict_solver::DictSolver,
    print_feedback, wordle_config,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let answer = wordle_config::<MemoryOracle, DictSolver, _, _>(
        true,
        |oracle| oracle.max_guesses = Some(6),
        |_| {},
    )?;
    #[cfg(not(feature = "pretty_feedback"))]
    println!("bot solver wins! ({})", answer);
    #[cfg(feature = "pretty_feedback")]
    {
        let all_correct = vec![Disposition::Correct; answer.chars().count()];
        print_feedback(&answer, &all_correct);
    }
    Ok(())
}
