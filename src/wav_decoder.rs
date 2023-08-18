use crate::*;
use std::io::{Read, Seek};
use hound::{WavReader, Error};

pub struct WavDecoder<R: Read + Seek> {
    reader: WavReader<R>,
}

impl<R: Read + Seek> WavDecoder<R> {
    pub fn new(reader: R) -> Result<Self, Error> {
        let reader = WavReader::new(reader)?;

        Ok(Self { reader })
    }

    pub fn channels(&self) -> usize {
        self.reader.spec().channels as usize
    }

    pub fn sample_rate(&self) -> usize {
        self.reader.spec().sample_rate as usize
    }
}

impl<R: Read + Seek> Iterator for WavDecoder<R> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.reader.samples::<i32>().next().map(|result| {
            Sample::from(&(result.unwrap() as i16))
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn it_decodes_wav_files() {
        let bytes = include_bytes!("../examples/wav_file.wav");
        let decoder = WavDecoder::new(Cursor::new(bytes)).unwrap();

        assert_eq!(decoder.collect::<Vec<_>>().len(), 345_472);
    }
}
