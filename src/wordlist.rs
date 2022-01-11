use std::{
    io::{BufRead, Read},
    path::PathBuf,
};

const WORDS_SOURCE: &str = "https://github.com/dwyl/english-words/raw/master/words_alpha.txt";
const BUFFER_SIZE: usize = 4096;
const COMPRESSION_LEVEL: i32 = 11; // Maximum, slowest, compression
const WINDOW_SIZE: i32 = 24; // Most dense, most memory-intensive compression window
const SPINNER_STYLE: spinners::Spinners = spinners::Spinners::Line;

// It would be really great if we could `impl Drop` for SpinnerReader` to stop the spinner, but
// the spinner implementaion doesn't permit that.
struct SpinnerReader<R> {
    inner: R,
    read_so_far: usize,
    total_expected: Option<usize>,
    message: String,
    spinner: spinners::Spinner,
}

impl<R> SpinnerReader<R> {
    fn new(inner: R, message: String, total_expected: Option<usize>) -> Self {
        let sr = SpinnerReader {
            inner,
            message: message,
            read_so_far: 0,
            total_expected,
            spinner: spinners::Spinner::new(&SPINNER_STYLE, String::new()),
        };
        sr.update_message();
        sr
    }

    fn update_message(&self) {
        let message;
        if let Some(total) = self.total_expected {
            message = format!("{} ({}/{})", self.message, self.read_so_far, total);
        } else {
            message = format!("{} ({})", self.message, self.read_so_far);
        }
        self.spinner.message(message);
    }
}

impl<R: Read> Read for SpinnerReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let result = self.inner.read(buf);
        if let Ok(qty) = result {
            self.read_so_far += qty;
            self.update_message();
        }
        result
    }
}

/// Produce the cache path
fn cache_path() -> Result<PathBuf, Error> {
    let dir = dirs::cache_dir().ok_or(Error::NoCacheDir)?.join("wordler");
    if !dir.exists() {
        std::fs::create_dir_all(&dir).map_err(Error::CreateCacheDir)?;
    }
    Ok(dir.join("words.br"))
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
    let response = ureq::get(WORDS_SOURCE).call()?;
    let length = response
        .header("Content-Length")
        .map(|l| l.parse().ok())
        .flatten();
    let mut reader = SpinnerReader::new(
        response.into_reader(),
        "downloading word list".into(),
        length,
    );

    let params = brotli::enc::BrotliEncoderParams {
        mode: brotli::enc::backward_references::BrotliEncoderMode::BROTLI_MODE_TEXT,
        quality: COMPRESSION_LEVEL,
        lgwin: WINDOW_SIZE,
        ..Default::default()
    };
    brotli::enc::BrotliCompress(&mut reader, &mut writer, &params).map_err(Error::Encode)?;

    // clean up the spinner
    reader.spinner.stop();
    println!();

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
    #[error("could not create the cache directory")]
    CreateCacheDir(#[source] std::io::Error),
    #[error("could not open the cache file")]
    OpenCache(#[source] std::io::Error),
    #[error("could not download the word list")]
    Download(#[from] ureq::Error),
    #[error("could not brotli-compress the cache file")]
    Encode(#[source] std::io::Error),
    #[error("cache not valid after download")]
    InvalidCache(#[source] Box<Error>),
}
