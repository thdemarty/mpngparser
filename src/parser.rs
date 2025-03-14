use crate::mpng;
use byteorder::{BigEndian, ReadBytesExt};
use std::{fs::File, io::Read, path::Path, process::exit};

// Magic number for mini PNG files
const MPNG_MAGIC: [u8; 8] = [0x4d, 0x69, 0x6e, 0x69, 0x2d, 0x50, 0x4e, 0x47];





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

    pub fn parse(&mut self) -> mpng::MPNG {
        // Check first 8 bytes of file
        let mut magic = [0; 8];
        self.reader.read_exact(&mut magic).unwrap();

        if magic != MPNG_MAGIC {
            eprintln!("Invalid MPNG file");
            exit(1);
        }

        while self.read_block() {
            continue;
        }

        // Build the MPNG struct
        let mpng = self.builder.clone().build();
        if mpng.is_err() {
            eprintln!("{}", mpng.err().unwrap());
            exit(1);
        } else {
            return mpng.unwrap();
        }
    }

    fn parse_header(&mut self) -> bool {
        let length = self.reader.read_u32::<BigEndian>().unwrap();
        if length != 9 {
            eprintln!("Invalid header block length: {}, must be 9.", length);
            exit(1);
        }
        
        let width = self.reader.read_u32::<BigEndian>().unwrap();
        let height = self.reader.read_u32::<BigEndian>().unwrap();
        let pixel_type = self.reader.read_u8().unwrap();
        if pixel_type > 3 {
            eprintln!("Invalid pixel type: {}", pixel_type);
            exit(1);
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

        return true;
    }

    fn parse_comment(&mut self) -> bool {
        let length = self.reader.read_u32::<BigEndian>().unwrap();
        let mut data = vec![0; length as usize];
        self.reader.read_exact(&mut data).unwrap();

        let text = String::from_utf8(data).unwrap();
        self.builder.set_comment(mpng::MPNGComment { text });

        return true;
    }

    fn parse_data(&mut self) -> bool {
        let length = self.reader.read_u32::<BigEndian>().unwrap();
        let mut data = vec![0; length as usize];

        // test data consistency
        assert!(self.builder.header.unwrap().width * self.builder.header.unwrap().height / 8 == length);

        self.reader.read_exact(&mut data).unwrap();

        self.builder.set_data(mpng::MPNGData { data });

        return false;
    }

    /// Read a block and orient the parser into the correct state
    /// return true if a block follows after this one else return false
    /// raise an error if the block is invalid
    fn read_block(&mut self) -> bool {
        let another_block = self.reader.read_u8();

        match another_block {
            Ok(block_type) => {
                // print block type as char
                match block_type {
                    b'H' => return self.parse_header(),
                    b'C' => return self.parse_comment(),
                    b'D' => return self.parse_data(),
                    0 => return false, // EOF
                    _ => {
                        eprintln!("Invalid block type: 0x{:x}", block_type);
                        std::process::exit(1);
                    }
                }
            }
            Err(_) => {
                eprintln!("Error reading block type");
                std::process::exit(1);
            }
        }
    }
}
