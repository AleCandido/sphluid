use anyhow::Result;
use indicatif::ProgressBar;

use sphluid::io::create;
use sphluid::universe::init;

fn main() -> Result<()> {
    let mut uni = init::random::<f64, 3>(1e4 as usize);

    let history_file = "history.nc";
    create(&history_file, &uni)?;

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
