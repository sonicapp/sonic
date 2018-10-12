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
    /// The title of a song
    pub title: Option<&'parsed str>,
    /// Artists credited for the song
    pub artist: Option<Vec<&'parsed str>>,
    /// Composers of the song
    pub composer: Option<Vec<&'parsed str>>,
    /// The song's album
    pub album: Option<&'parsed str>,
    /// The songs genre
    pub genre: Option<&'parsed str>,
    /// BPM speed information
    pub bpm: Option<u32>,
    /// If the song is part of a compilation, this
    /// field should be set to its name.
    pub compilation: Option<&'parsed str>
}

