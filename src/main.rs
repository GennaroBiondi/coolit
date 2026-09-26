#![doc = include_str!("../README.md")]

use anyhow::{Context, Ok, Result, bail};
use clap::Parser;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

/// Contains types to work with chunk data
mod chunks;

mod memory_unit;

pub use chunks::{Chunk, Chunks};
pub use memory_unit::MemoryUnit;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short = 'i', long = "input")]
    source: PathBuf,

    #[arg(short = 'o', long = "output")]
    destination: PathBuf,

    #[arg(short = 'l', long = "limit", conflicts_with = "unpack")]
    size_limit: Option<MemoryUnit>,

    #[arg(short = 'u', long = "unpack", conflicts_with = "size_limit")]
    unpack: bool,
}

fn pack(source: &Path, destination: &Path, byte_size_limit: u64) -> Result<()> {
    if !source.exists() {
        bail!("source file doesn't exist")
    }

    if destination.exists() {
        bail!("destination already exists")
    }

    if !source.is_file() {
        bail!("can't pack anything other than files")
    }

    // let file = File::open(source).context("failed to open source file")?;
    // let mut reader = BufReader::new(file);

    let chunk_data = std::fs::read(source)?;
    let chunk = Chunk::from_bytes(chunk_data);

    let chunks = chunk.divided(byte_size_limit as usize);

    // loop {
    //     let mut chunk_data = vec![0u8; byte_size_limit as usize];
    //     let bytes_read = reader
    //         .read(&mut chunk_data)
    //         .context("failed to read chunk data")?;
    //
    //     // EOF reached.
    //     if bytes_read == 0 {
    //         break;
    //     }
    //
    //     let pushing_chunk = Chunk::from_bytes(chunk_data);
    //     chunks.push(pushing_chunk);
    // }

    chunks
        .to_filesystem(destination)
        .context("failed to dump chunks to filesystem")?;

    Ok(())
}

fn unpack(source: &Path, destination: &Path) -> Result<()> {
    use std::io::Write;

    if !source.exists() {
        bail!("source file doesn't exist")
    }

    if destination.exists() {
        bail!("destination already exists")
    }

    if !source.is_dir() {
        bail!("can't unpack anything other than directories")
    }

    let chunks =
        Chunks::from_filesystem(source).context("failed to collect chunks from filesystem")?;
    let mut file = File::create(destination).context("failed to create output file")?;

    let unpacked_file_data = chunks.unified().into_inner();
    file.write_all(&unpacked_file_data)
        .context("failed to write to output file")?;

    Ok(())
}

fn main() -> Result<()> {
    let args = Arguments::parse();

    if args.unpack {
        unpack(&args.source, &args.destination)?;
    } else {
        let size_limit = args
            .size_limit
            .context("missing size limit (use --limit or --unpack)")?;

        pack(&args.source, &args.destination, size_limit.as_byte_amount())?;
    }

    Ok(())
}
