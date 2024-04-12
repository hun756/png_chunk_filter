use crate::chunk::Chunk;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read as _, Write as _};

pub fn process_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;

    let mut png_filter = PngFilter::new(input_file, output_file);
    png_filter.filter()
}

struct PngFilter<R: std::io::Read, W: std::io::Write> {
    reader: BufReader<R>,
    writer: BufWriter<W>,
}

impl<R: std::io::Read, W: std::io::Write> PngFilter<R, W> {
    fn new(reader: R, writer: W) -> Self {
        Self {
            reader: BufReader::new(reader),
            writer: BufWriter::new(writer),
        }
    }

    fn filter(&mut self) -> io::Result<()> {
        let mut buffer = Vec::new();
        self.reader.read_to_end(&mut buffer)?;

        let png_signature: &[u8] = b"\x89PNG\r\n\x1a\n";
        if !buffer.starts_with(png_signature) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a PNG file"));
        }

        self.writer.write_all(png_signature)?;

        let mut position: usize = 8;
        while position + 12 <= buffer.len() {
            if let Some(chunk) = Chunk::new(&buffer[position..]) {
                if chunk.is_critical() {
                    self.writer
                        .write_all(&buffer[position..position + 8 + chunk.length as usize + 4])?;
                    if chunk.chunk_type == *b"IEND" {
                        break;
                    }
                }
                position += 8 + chunk.length as usize + 4;
            } else {
                break;
            }
        }

        Ok(())
    }
}
