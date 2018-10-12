//! Sonic Scraper - Music tag scraper library
//!
//! This is primarily the tagging backend for
//! [sonic], but is written in a way that should
//! be useful for other tools, working with
//! audio files and their metadata, as well.
//!
//! [sonic]: https://github.com/sonic/

/// Metadata that can be known about a song
pub struct Metadata<'parsed> {
    title: Option<&'parsed str>,
    artist: Option<&'parsed str,
    composer: Option<&'parsed str>,
    album: Option<&'parsed str>,
    genre: Option<&'parsed str>,
    bpm: Option<u32>,
}
