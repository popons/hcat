use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

fn to_lines(path: impl AsRef<Path>) -> Result<Vec<String>, Error> {
    let f = File::open(path)?;
    let r = BufReader::new(f);
    let lines = r.lines();
    Ok(lines.collect::<Result<Vec<String>, Error>>()?)
}

fn main() -> Result<(), Error> {
    let xs = std::env::args().collect::<Vec<String>>();
    let p = xs
        .get(1)
        .ok_or(Error::new(ErrorKind::Other, "arg invalid"))?;
    let xs = to_lines(p)?;
    for x in xs.iter() {
        println!("{}", x);
    }
    Ok(())
}
