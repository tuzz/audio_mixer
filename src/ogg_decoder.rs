use crate::*;
use std::io::{Read, Seek};
use std::vec::IntoIter;
use lewton::{VorbisError, inside_ogg::OggStreamReader};

pub struct OggDecoder<R: Read + Seek> {
    reader: OggStreamReader<R>,
    packet: IntoIter<i16>,
}

impl<R: Read + Seek> OggDecoder<R> {
    pub fn new(reader: R) -> Result<Self, VorbisError> {
        let reader = OggStreamReader::new(reader)?;

        Ok(Self { reader, packet: vec![].into_iter() })
    }

    pub fn channels(&self) -> usize {
        self.reader.ident_hdr.audio_channels as usize
    }

    pub fn sample_rate(&self) -> usize {
        self.reader.ident_hdr.audio_sample_rate as usize
    }
}

impl<R: Read + Seek> Iterator for OggDecoder<R> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if let Some(sample) = self.packet.next() {
            return Some(Sample::from::<i16>(&sample));
        }

        while let Ok(Some(packet)) = self.reader.read_dec_packet_itl() {
            self.packet = packet.into_iter();

            if let Some(sample) = self.packet.next() {
                return Some(Sample::from::<i16>(&sample));
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn it_decodes_ogg_files() {
        let bytes = include_bytes!("../examples/ogg_file.ogg");
        let decoder = OggDecoder::new(Cursor::new(bytes)).unwrap();

        assert_eq!(decoder.collect::<Vec<_>>().len(), 345_456);
    }
}
