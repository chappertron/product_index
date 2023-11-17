use anyhow::Result;
use clap::Parser;
use product_index::product_index_Nd;

use io::Write;
use std::io;

fn main() -> Result<()> {
    let ProductIndex { index, lengths } = ProductIndex::parse();

    let indicies = product_index_Nd(index, &lengths)?;

    // Lock to prevent repeated locking.
    let mut std_out = io::stdout().lock();

    for i in indicies {
        // TODO: Remove trailing whitespace?
        write!(std_out, "{i} ")?;
    }

    writeln!(std_out,)?;

    Ok(())
}

#[derive(Parser, Debug)]
/// A command line tool to get the indicies to that correspond to a Cartesian product.
struct ProductIndex {
    /// 1D Index to convert to N-Dimensional.
    index: usize,
    /// Length of each of the N arrays.
    #[arg(required = true)]
    lengths: Vec<usize>,
}
