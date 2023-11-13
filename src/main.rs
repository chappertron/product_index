use anyhow::Result;
use clap::Parser;
use product::product_index_Nd;

use io::Write;
use std::io;

fn main() -> Result<()> {
    let ProductIndex { index, dimensions } = ProductIndex::parse();

    let indicies = product_index_Nd(index, &dimensions)?;

    let mut std_out = io::stdout().lock();

    for i in indicies {
        // TODO: Remove trailing whitespace?
        write!(std_out, "{i} ")?;
    }
    writeln!(std_out,)?;

    Ok(())
}

#[derive(Parser, Debug)]
struct ProductIndex {
    index: usize,
    dimensions: Vec<usize>,
}
