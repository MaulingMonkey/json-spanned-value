use crate::Shared;

use std::convert::*;
use std::io::{self, Read};
use std::rc::Rc;



/// A specialized [io::Read]er designed to identify JSON token boundaries and
/// make their position accessible.  This results in some very odd design
/// choices.
pub(crate) struct Reader<B: Buffer> {
    buf:    B,
    shared: Rc<Shared>,
    mode:   Mode,
}

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Normal,
    String,
    StringSlash,

    InSingleLineComment,
    InMultiLineComment,
}

impl<B: Buffer> Reader<B> {
    pub(crate) fn new(buf: B, shared: Rc<Shared>) -> Self {
        let mut r = Self { buf, shared, mode: Mode::Normal };
        r.advance_start_from(0);
        r
    }

    pub(crate) fn advance_start_from(&mut self, mut pos: usize) {
        assert_eq!(self.mode, Mode::Normal);
        let shared = &*self.shared;
        let src = self.buf.as_bytes();

        // We often need to seek "ahead" to find the start of the next token.
        // Doing so in here as an I/O side effect is very odd, but avoids
        // the need to statically store a reference to &'nonstatic [u8].
        // 
        // NOTE WELL:  shared.start == shared.pos - 1 is legal and common!  This
        // means arrays, strings, etc. all include the opening character.

        if shared.start.get().0 > pos { return; } // Avoid O(nn) behavior for "     ..."

        while let Some(ch) = src.get(pos) {
            if b": \r\n\t".contains(ch) {
                // Whitespace or ":" that's either not part of a token, or mid-string where we don't care
                pos += 1;
            } else if *ch == b'/' && shared.settings.allow_comments {
                // Probably a comment
                match src.get(pos+1) {
                    Some(b'/') => {
                        pos += 2; // b"//"
                        while let Some(ch) = src.get(pos) {
                            pos += 1;
                            if *ch == b'\n' { break }
                        }
                    },
                    Some(b'*') => {
                        pos += 2; // b"/*"
                        while let Some(ch) = src.get(pos) {
                            if *ch == b'*' && src.get(pos+1) == Some(&b'/') {
                                pos += 2;
                                break;
                            } else {
                                pos += 1;
                            }
                        }
                    },
                    _other => break,
                }
            } else {
                break;
            }
        }

        shared.start.set((pos, *src.get(pos).unwrap_or(&b'\0')));
    }
}

impl<B: Buffer> Read for Reader<B> {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if out.is_empty() { return Ok(0) }

        // serde_json is kind enough to only ever request a single byte at
        // a time.  Since serde_json is the only thing that should be using this
        // reader, we simplify logic some by only ever returning a single byte
        // at a time, even if more were available.  This also lets us accurately
        // track serde_json's exact cursor position.

        let src = self.buf.as_bytes();
        let pos = self.shared.pos.get();
        let mut ch = if let Some(n) = src.get(pos) { *n } else { return Ok(0) };
        // The current seek position is used to determine where many tokens end
        self.shared.pos.set(pos + 1);

        // Possibly skip comments and trailing commas.  To preserve
        // serde_json::Error line/column -> byte offset mappings, we do so by
        // replacing said content with whitespace.

        match self.mode {
            Mode::Normal => {
                self.advance_start_from(pos);
                let src = self.buf.as_bytes();
                match ch {
                    b'\"' => self.mode = Mode::String,
                    b',' if self.shared.settings.allow_trailing_comma => {
                        self.advance_start_from(pos+1);
                        match self.shared.start.get().1 {
                            b']'    => ch = b' ',
                            b'}'    => ch = b' ',
                            _other  => {},
                        }
                    },
                    b'/' if self.shared.settings.allow_comments => {
                        ch = b' ';
                        match src.get(pos+1) {
                            Some(b'/')  => self.mode = Mode::InSingleLineComment,
                            Some(b'*')  => self.mode = Mode::InMultiLineComment,
                            _other      => ch = b'/', // Okay, not actually a comment I guess
                        }
                    },
                    _other => {},
                }
            },
            Mode::String => match ch {
                b'\\'   => self.mode = Mode::StringSlash,
                b'\"'   => self.mode = Mode::Normal,
                _other  => {},
            },
            Mode::StringSlash => self.mode = Mode::String, // \n, \u1234, ...
            Mode::InSingleLineComment => match ch {
                b'\n' => self.mode = Mode::Normal,
                _other => ch = b' ',
            },
            Mode::InMultiLineComment => {
                if ch == b'/' && src[pos-1] == b'*' { self.mode = Mode::Normal }
                ch = b' ';
            },
        }

        out[0] = ch;
        Ok(1)
    }
}

impl<T: AsRef<[u8]>> Buffer for T {}
#[doc(hidden)] pub trait Buffer : AsRef<[u8]> {
    fn as_bytes(&self) -> &[u8] { self.as_ref() }
    fn len(&self) -> usize { self.as_bytes().len() }
}
