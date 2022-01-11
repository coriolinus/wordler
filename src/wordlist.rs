use std::io::BufRead;
use std::path::PathBuf;

const WORDS_SOURCE: &str = "https://github.com/dwyl/english-words/raw/master/words_alpha.txt";
const BUFFER_SIZE: usize = 4096;
const COMPRESSION_LEVEL: i32 = 11; // Maximum, slowest, compression
const WINDOW_SIZE: i32 = 24; // Most dense, most memory-intensive compression window

/// Produce the cache path
fn cache_path() -> Result<PathBuf, Error> {
    let dir = dirs::cache_dir().ok_or(Error::NoCacheDir)?;
    Ok(dir.join("wordler").join("words.br"))
}

/// Read the cache from the canonical path without attempting to fetch it
fn read_cache() -> Result<impl Iterator<Item = String>, Error> {
    let path = cache_path()?;
    let reader = std::fs::File::open(path).map_err(Error::OpenCache)?;
    let reader = brotli::Decompressor::new(reader, BUFFER_SIZE);
    let reader = std::io::BufReader::new(reader);
    Ok(reader
        .lines()
        .map(|line| line.expect("brotli read failed in the middle of a line")))
}

/// Download the cache from the source, clobbering any existing data
fn create_cache() -> Result<(), Error> {
    let path = cache_path()?;
    let mut writer = std::fs::File::create(path).map_err(Error::OpenCache)?;
    let mut reader = ureq::get(WORDS_SOURCE).call()?.into_reader();

    let params = brotli::enc::BrotliEncoderParams {
        mode: brotli::enc::backward_references::BrotliEncoderMode::BROTLI_MODE_TEXT,
        quality: COMPRESSION_LEVEL,
        lgwin: WINDOW_SIZE,
        ..Default::default()
    };
    brotli::enc::BrotliCompress(&mut reader, &mut writer, &params).map_err(Error::Encode)?;
    Ok(())
}

/// Load the word list from cache, or download it fresh from the source and cache it.
pub fn load() -> Result<impl Iterator<Item = String>, Error> {
    let words = read_cache();
    if words.is_ok() {
        return words;
    }
    create_cache()?;
    read_cache().map_err(|err| Error::InvalidCache(Box::new(err)))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no cache directory could be constructed")]
    NoCacheDir,
    #[error("could not open the cache file")]
    OpenCache(#[source] std::io::Error),
    #[error("could not download the word list")]
    Download(#[from] ureq::Error),
    #[error("could not brotli-compress the cache file")]
    Encode(#[source] std::io::Error),
    #[error("cache not valid after download")]
    InvalidCache(#[source] Box<Error>),
}
