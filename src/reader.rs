use crate::Shared;

use std::convert::*;
use std::io::{self, Read};
use std::rc::Rc;



/// A specialized [io::Read]er designed to identify JSON token boundaries and
/// make their position accessible.  This results in some very odd design
/// choices.
pub(crate) struct Reader<B: Buffer> {
    pub(crate) buf:    B,
    pub(crate) shared: Rc<Shared>,
}

impl<B: Buffer> Read for Reader<B> {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if out.is_empty() { return Ok(0) }

        // serde_json is kind enough to only ever request a single byte at
        // a time.  Since serde_json is the only thing that should be using this
        // reader, we simplify logic some by only ever returning a single byte
        // at a time, even if more were available.  This also lets us accurately
        // track serde_json's exact cursor position.

        let shared = &*self.shared;
        let src = self.buf.as_bytes();
        let pos1 = shared.pos.get();
        out[0] = if let Some(n) = src.get(pos1) { *n } else { return Ok(0) };
        let pos2 = pos1 + 1;

        // The current seek position is used to determine where many tokens end
        shared.pos.set(pos2);

        // We often need to seek "ahead" to find the start of the next token.
        // Doing so in here as an I/O side effect is very odd, but avoids
        // the need to statically store a reference to &'nonstatic [u8].
        // 
        // NOTE WELL:  shared.start == shared.pos - 1 is legal and common!  This
        // means arrays, strings, etc. all include the opening character.

        if shared.start.get().0 <= pos1 { // Avoid O(nn) behavior for "     ..."
            let mut start = pos1;
            while b": \r\n\t".contains(src.get(start).unwrap_or(&b'\0')) { start += 1; }
            shared.start.set((start, src[start]));
        }

        Ok(1)
    }
}

impl<T: AsRef<[u8]>> Buffer for T {}
pub(crate) trait Buffer : AsRef<[u8]> {
    fn as_bytes(&self) -> &[u8] { self.as_ref() }
    fn len(&self) -> usize { self.as_bytes().len() }
}
