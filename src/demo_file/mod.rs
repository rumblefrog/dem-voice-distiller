use std::io::{
    Cursor,
    Result as IOResult,
    Read,
    Seek,
    SeekFrom,
};

use byteorder::{
    ReadBytesExt,
    LittleEndian,
    ByteOrder,
};

use crate::errors::{
    Result,
    Error,
};

use crate::demo_header::DemoHeader;

struct DemoFile<T> {
    /// A single source of reads
    cursor: Cursor<T>,

    header_size: i32,
}

impl<T> DemoFile<T>
where
    T: AsRef<[u8]>,
{
    pub fn new(inner: T) -> Self {
        Self {
            cursor: Cursor::new(inner),
            header_size: 0,
        }
    }

    /// Rewind to the start & read
    pub fn header(&mut self) -> Result<DemoHeader> {
        self.cursor.seek(SeekFrom::Start(0))?;

        let header_magic = self.cursor.read_mstring(8)?;

        if header_magic != "HL2DEMO" {
            return Err(Error::InvalidHeader);
        }

        Ok(DemoHeader {
            header: header_magic,
            demo_protocol: self.cursor.read_i32::<LittleEndian>()?,
            network_protocol: self.cursor.read_i32::<LittleEndian>()?,
            server_name: self.cursor.read_mstring(260)?,
            client_name: self.cursor.read_mstring(260)?,
            map_name: self.cursor.read_mstring(260)?,
            game_dir: self.cursor.read_mstring(260)?,
            playback_time: self.cursor.read_f32::<LittleEndian>()?,
            ticks: self.cursor.read_i32::<LittleEndian>()?,
            frames: self.cursor.read_i32::<LittleEndian>()?,
            signon_len: self.cursor.read_i32::<LittleEndian>()?,
        })
    }
}

trait ReadMemoryString {
    /// Reads a fixed size memory-mapped string
    /// 
    /// Note: Valve likes to memory-mapped structures which requires
    /// fixed-sized fields
    fn read_mstring(&mut self, len: usize) -> IOResult<String>;
}

impl<T> ReadMemoryString for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn read_mstring(&mut self, len: usize) -> IOResult<String> {
        let mut buf = vec![0; len];
        let mut str_vec = vec![0; len];

        self.read_exact(&mut buf)?;

        for c in buf {
            if c == 0 {
                break;
            } else {
                str_vec.push(c);
            }
        }

        Ok(String::from_utf8_lossy(&str_vec).into_owned())
    }
}
