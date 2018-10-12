//! Sonic Scraper - Music tag scraper library
//!
//! This is primarily the tagging backend for
//! [sonic], but is written in a way that should
//! be useful for other tools, working with
//! audio files and their metadata, as well.
//!
//! [sonic]: https://github.com/sonic/

/// A scraper accumilates information about songs
///
/// It's initialised over a set of files
/// and then tries to read as much
/// metadata from these files as possible.
/// Each song is assigned a unique hash
/// that is reproducible between runs.
pub struct Scraper {

}
