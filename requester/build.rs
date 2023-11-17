use ethers::prelude::Abigen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Abigen::new("ValueIncrementer", "./src/resources/ValueIncrementer.json")?
        .generate()?
        .write_to_file("./src/value_incrementer.rs")?;

    Ok(())
}
