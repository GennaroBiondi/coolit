use std::path::Path;

use anyhow::{Context, Result, bail};

/// Sequence of binary data.
#[derive(Debug, Clone)]
pub struct Chunk {
    inner: Vec<u8>,
}

impl Chunk {
    /// Creates a new [`Chunk`].
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Creates a new [`Chunk`], initializing it with the given bytes.
    pub const fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { inner: bytes }
    }

    /// Returns a reference to the [`Chunk`]'s bytes.
    pub fn as_inner(&self) -> &[u8] {
        &self.inner
    }

    /// Returns a mutable reference to the [`Chunk`]'s bytes.
    pub fn as_inner_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }

    /// Returns the [`Chunk`]'s bytes, consuming it.
    pub fn into_inner(self) -> Vec<u8> {
        self.inner
    }

    /// Divides the [`Chunk`] into a [`Chunks`] and returns it.
    pub fn divided(&self, split_size: usize) -> Chunks {
        let mut chunks = Chunks::new();

        for chunk in self.inner.chunks(split_size) {
            chunks.push(Chunk::from_bytes(chunk.to_vec()));
        }

        chunks
    }
}

/// Ordered group of chunks.
#[derive(Debug, Clone)]
pub struct Chunks {
    inner: Vec<Chunk>,
}

impl Chunks {
    /// Creates new [`Chunks`].
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Append a [`Chunk`] to the back of the [`Chunks`] collection.
    pub fn push(&mut self, chunk: Chunk) {
        self.inner.push(chunk);
    }

    /// Returns a reference to the inner chunks.
    pub fn as_inner(&self) -> &[Chunk] {
        &self.inner
    }

    /// Returns a mutable reference to the inner chunks.
    pub fn as_inner_mut(&mut self) -> &mut [Chunk] {
        &mut self.inner
    }

    /// Returns the inner chunks, consuming the [`Chunks`].
    pub fn into_inner(self) -> Vec<Chunk> {
        self.inner
    }

    /// Dumps all the chunks by creating a directory and a file for each chunk, containing its data.
    ///
    /// # Errors
    ///
    /// Will error if the path already exists, or any I/O error happens.
    pub fn to_filesystem(&self, path: &Path) -> Result<()> {
        use std::{
            fs::{self, File},
            io::Write,
        };

        if path.exists() {
            bail!("path already exists")
        }

        fs::create_dir(path).context("creating chunks directory")?;

        for (index, chunk) in self.inner.iter().enumerate() {
            let new_chunk_name = format!("chunk_{:04}.chunk", index);

            let mut file =
                File::create(path.join(new_chunk_name)).context("failed to create chunk file")?;
            file.write_all(&chunk.inner)
                .context("failed to write to chunk file")?;
            file.flush().context("failed to flush chunk file")?;
        }

        Ok(())
    }

    /// Collects all the chunks in a directory and returns them.
    ///
    /// # Errors
    ///
    /// Will error if any I/O bound error happens.
    pub fn from_filesystem(source_dir: &Path) -> Result<Self> {
        use std::fs;

        let mut entries: Vec<_> = fs::read_dir(source_dir)
            .context("failed to iterate over chunk directory")?
            .map(|entry| entry.map(|e| e.path()))
            .collect::<Result<_, _>>()?;

        entries.retain(|path| path.is_file());
        entries.sort();

        let mut chunks = Self::new();

        for entry in entries {
            let data = fs::read(entry).context("failed to read chunk file")?;
            let data = Chunk::from_bytes(data);
            chunks.inner.push(data);
        }

        Ok(chunks)
    }

    /// Unifies all the [`Chunk`]s in the [`Chunks`] into a single one [`Chunk`] and returns
    /// it.
    pub fn unified(&self) -> Chunk {
        Chunk::from_bytes(
            self.inner
                .iter()
                .flat_map(|chunk| chunk.inner.iter())
                .copied()
                .collect(),
        )
    }
}
