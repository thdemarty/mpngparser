use crate::mpng;
use byteorder::{BigEndian, ReadBytesExt};
use std::{fs::File, io::Read, path::Path, process::exit};

// Magic number for mini PNG files
const MPNG_MAGIC: [u8; 8] = [0x4d, 0x69, 0x6e, 0x69, 0x2d, 0x50, 0x4e, 0x47];


#[derive(Debug)]
pub enum ParsingError {
    InvalidBlockTag,
    InvalidMagicNumber,
    InvalidHeaderBlockLength,
    InvalidHeaderPixelType,
    InvalidDataBlockConsistency,
}

#[derive(Debug, Clone)]
struct MPNGBuilder {
    pub header: Option<mpng::MPNGHeader>,
    pub comment: Option<mpng::MPNGComment>,
    pub data: Option<mpng::MPNGData>,
}

impl MPNGBuilder {
    fn new() -> Self {
        MPNGBuilder {
            header: None,
            comment: None,
            data: None,
        }
    }

    fn build(self) -> Result<mpng::MPNG, &'static str> {
        let header = self.header.ok_or("Missing header block")?;
        let data = self.data.ok_or("Missing data block")?;
        Ok(mpng::MPNG {
            header,
            comment: self.comment,
            data,
        })
    }

    fn set_header(&mut self, header: mpng::MPNGHeader) {
        self.header = Some(header);
    }

    fn set_comment(&mut self, comment: mpng::MPNGComment) {
        self.comment = Some(comment);
    }

    fn set_data(&mut self, data: mpng::MPNGData) {
        self.data = Some(data);
    }
}

pub struct Parser {
    reader: std::io::BufReader<std::fs::File>,
    builder: MPNGBuilder,
}

impl Parser {
    pub fn new(filepath: &Path) -> Self {
        let file = File::open(filepath).unwrap();
        let reader = std::io::BufReader::new(file);
        let builder = MPNGBuilder::new();
        Parser { reader, builder }
    }

    pub fn parse(&mut self) -> Result<mpng::MPNG, ParsingError> {
        // Check first 8 bytes of file
        let mut magic = [0; 8];
        self.reader.read_exact(&mut magic).unwrap();

        if magic != MPNG_MAGIC {
            return Err(ParsingError::InvalidMagicNumber);
        }

        loop {
            match self.read_block() {
                Ok(true) => continue,
                Ok(false) => break,
                Err(e) => return Err(e),
            }
        }

        // Build the MPNG struct
        let mpng = self.builder.clone().build();
        if mpng.is_err() {
            eprintln!("{}", mpng.err().unwrap());
            exit(1);
        } else {
            return Ok(mpng.unwrap());
        }
    }

    fn parse_header(&mut self) -> Result<bool, ParsingError> {
        let length = self.reader.read_u32::<BigEndian>().unwrap();
        if length != 9 {
            return Err(ParsingError::InvalidHeaderBlockLength);
        }
        
        let width = self.reader.read_u32::<BigEndian>().unwrap();
        let height = self.reader.read_u32::<BigEndian>().unwrap();
        let pixel_type = self.reader.read_u8().unwrap();
        if pixel_type > 3 {
            return Err(ParsingError::InvalidHeaderPixelType);
        }


        self.builder.set_header(mpng::MPNGHeader {
            width,
            height,
            pixel_type: match pixel_type {
                0 => mpng::PixelType::BlackAndWhite,
                1 => mpng::PixelType::GreyScale,
                2 => mpng::PixelType::Palette,
                3 => mpng::PixelType::TrueColor,
                _ => unreachable!(),
            },
        });

        return Ok(true);
    }

    fn parse_comment(&mut self) -> Result<bool, ParsingError> {
        let length = self.reader.read_u32::<BigEndian>().unwrap();
        let mut data = vec![0; length as usize];
        self.reader.read_exact(&mut data).unwrap();

        let text = String::from_utf8(data).unwrap();
        self.builder.set_comment(mpng::MPNGComment { text });

        return Ok(true);
    }

    fn parse_data(&mut self) -> Result<bool, ParsingError> {
        let length = self.reader.read_u32::<BigEndian>().unwrap();
        
        match self.builder.header.unwrap().width * self.builder.header.unwrap().height / 8 == length {
            true => {
                let mut data_buffer = vec![0; length as usize];
                self.reader.read_exact(&mut data_buffer).unwrap();
                self.builder.set_data(mpng::MPNGData { data: data_buffer });
                Ok(true)
            },
            false => { Err(ParsingError::InvalidDataBlockConsistency) }
            
        }

       
    }

    /// Read a block and orient the parser into the correct state
    /// return true if a block follows after this one else return false
    /// raise an error if the block is invalid
    fn read_block(&mut self) -> Result<bool, ParsingError> {
        let another_block = self.reader.read_u8();

        match another_block {
            Ok(block_type) => {
                // print block type as char
                match block_type {
                    b'H' => self.parse_header(),
                    b'C' => self.parse_comment(),
                    b'D' => self.parse_data(),
                    _ => {
                        eprintln!( "Invalid block tag: {:?}", block_type as char);
                        return Err(ParsingError::InvalidBlockTag)
                    }
                    
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    return Ok(false);
                } else {
                    panic!("Error reading block type: {:?}", e);
                }
            }
        }
    }
}
