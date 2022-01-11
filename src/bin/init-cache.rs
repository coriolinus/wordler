use wordler::wordlist::load;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let words = load()?;
    println!("cached {} words", words.count());
    Ok(())
}
