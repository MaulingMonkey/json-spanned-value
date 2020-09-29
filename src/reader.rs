use crate::Shared;

use std::convert::*;
use std::io::{self, Read};
use std::rc::Rc;



pub(crate) struct Reader<B: Buffer> {
    pub(crate) buf:    B,
    pub(crate) shared: Rc<Shared>,
}

impl<B: Buffer> Read for Reader<B> {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if out.is_empty() { return Ok(0) }

        let shared = &*self.shared;
        let src = self.buf.as_bytes();
        let pos1 = shared.pos.get().0;
        let next = if let Some(n) = src.get(pos1) { *n } else { return Ok(0) };
        let pos2 = pos1 + 1;

        shared.last_read.set(next);
        shared.prev.set(shared.pos.get());
        shared.pos.set((pos2, next));
        out[0] = next;

        if shared.start.get().0 <= pos1 {
            let start = if next == b'\"' {
                pos1
            } else {
                let mut start = pos1;
                while b": \r\n\t".contains(src.get(start).unwrap_or(&b'\0')) { start += 1; }
                start
            };
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
