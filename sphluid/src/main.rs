use anyhow::Result;
use sphluid::io::{append, create};

fn main() -> Result<()> {
    create()?;
    append()
}
