mod tokenizer;
mod parser;
mod vm;
mod storage;
mod repl;
mod errors;

use anyhow::Result;

fn main() -> Result<()> {
    repl::start_repl()
}