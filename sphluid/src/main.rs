use anyhow::Result;
use indicatif::ProgressBar;

use sphluid::universe::Universe;

fn main() -> Result<()> {
    let history_file = "history.nc";

    let mut uni = Universe::<f64>::new(history_file.as_ref())?;

    let bar = ProgressBar::new(100);

    uni.snapshot(&history_file)?;
    for i in 1..100 {
        uni.vacuum();
        if i % 1 == 0 {
            uni.snapshot(&history_file)?;
        }
        bar.inc(1);
    }

    Ok(())
}
