use std::io::Write;

use crate::petitioner::Petitioner;

#[derive(Debug)]
pub struct HumanPetitioner;

impl Petitioner for HumanPetitioner {
    fn new(word_length: usize) -> Result<Box<Self>, super::Error> {
        println!("You must guess a word of {} characters.", word_length);
        Ok(Box::new(Self))
    }

    fn prepare_guess(&mut self) -> Result<String, super::Error> {
        let mut stdout = ezio::stdio::stdout();
        stdout.write_all(b"> ").expect("can write to stdout");
        stdout.flush().expect("can write to stdout");
        Ok(ezio::stdio::read_line())
    }

    fn feedback(&mut self, _feedback: crate::oracle::Feedback) -> Result<(), super::Error> {
        Ok(())
    }
}
