extern crate core;

use std::io::Cursor;
use std::mem::size_of;
use anyhow::{anyhow, ensure, Result};
use bson::{Bson, Document, RawBson};
use bytes::Bytes;
use object::ReadRef;
use xz2::read::{XzDecoder, XzEncoder};
use xz2::stream::Action;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, Read};
    use crate::read;

    #[test]
    fn it_works() {
        let buf = {
            let mut buf = vec![];
            BufReader::new(File::open("/home/kisaragi/export/EmptyObject.7zbson.lzma").unwrap()).read_to_end(&mut buf).unwrap();
            buf
        };
        println!("{buf:x?}", buf = &buf);

        let a = bytes::Bytes::copy_from_slice(buf.as_slice());
        read(a).unwrap();
    }
}

fn read(bytes: Bytes) -> Result<Document> {
    let compress_level_header = 0x5d00_0020u32.to_be();
    let header_byte_5 = 0x00u8;
    let mut big_endian_uncompressed_size: u64 = 0xFFFF_FFFF_FFFF_FFFF;
    let mut compressed_body_size: u64 = 0xFFFF_FFFF_FFFF_FFFF;
    let unknown_byte = 0x00u8;
    // -----
    /*
    eprintln!("len: {l}", l = bytes.len());
    let mut index = 0;
    eprintln!("{index}");
    {
        let header: &u32 = bytes.read(&mut index).map_err(|_| anyhow!("decode fail (lv header)"))?;
        ensure!(compress_level_header == *header, "the compress level header was mismatch. expected: {compress_level_header:016x}, actual: {header:016x}");
        eprintln!("{index}");
    }
    {
        let _1: &u8 = bytes.read(&mut index).map_err(|_| anyhow!("decode fail byte(5)"))?;
        ensure!(header_byte_5 == *_1, "byte(5) was mismatch");
        eprintln!("{index}");
    }
    /* {
        let size: &u64 = bytes.read(&mut index).map_err(|_| anyhow!("decode fail (raw size)"))?;
        big_endian_uncompressed_size = (*size).to_be();
    } */
    index += 8;
    {
        let size: &u64 = bytes.read(&mut index).map_err(|_| anyhow!("decode fail (body size)"))?;
        compressed_body_size = *size;
        eprintln!("{index}");
    }
    {
        let _1: &u8 = bytes.read(&mut index).map_err(|_| anyhow!("decode fail"))?;
        ensure!(unknown_byte == *_1, "byte was mismatch");
        eprintln!("{index}");
    }
    */
    let mut decoder = xz2::stream::Stream::new_lzma_decoder(1 << 24)?;
    let mut buf = vec![];
    let index = 24;
    decoder.process_vec(&bytes.as_ref()[index..], &mut buf, Action::Finish)?;
    let bson = Document::from_reader(Cursor::new(buf))?;
    Ok(bson)
}
