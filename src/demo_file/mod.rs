use std::io::Cursor;

struct DemoFile<T> {
    /// A single source of reads
    cursor: Cursor<T>,
}

impl<T> DemoFile<T> {
    pub fn new(inner: T) -> Self {
        Self {
            cursor: Cursor::new(inner)
        }
    }

    pub fn read_header(&mut self) {

    }
}
