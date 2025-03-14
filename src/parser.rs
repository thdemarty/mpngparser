use byteorder::{BigEndian, ReadBytesExt};
use std::{io::Read, process::exit};

// Magic number for mini PNG files
const MPNG_MAGIC: [u8; 8] = [0x4d, 0x69, 0x6e, 0x69, 0x2d, 0x50, 0x4e, 0x47];

#[derive(Debug, Clone, Copy)]
enum PixelType {
    BlackAndWhite = 0,
    GreyScale = 1,
    Palette = 2,
    TrueColor = 3,
}
#[derive(Debug, Clone, Copy)]
struct MPNGHeader {
    width: u32,
    height: u32,
    pixel_type: PixelType,
}
#[derive(Debug, Clone)]
struct MPNGComment {
    text: String,
}

#[derive(Debug, Clone)]
struct MPNGData {
    data: Vec<u8>,
}

#[derive(Debug)]
pub struct MPNG {
    header: MPNGHeader,
    comment: Option<MPNGComment>,
    data: MPNGData,
}

impl MPNG {
    pub fn print(&self) {
        // display each bytes as bit

        let mut counter = 0;
        for byte in &self.data.data {
            for i in 0..8 {
                let pixel_value = (byte >> i) & 1;
                if pixel_value == 1 {
                    print!("⬜");
                } else {
                    print!("⬛");
                }
                
                counter += 1;

                if counter % self.header.width == 0 {
                    println!();
                }

            }
        }
    }
}



#[derive(Debug, Clone)]
struct MPNGBuilder {
    header: Option<MPNGHeader>,
    comment: Option<MPNGComment>,
    data: Option<MPNGData>,
}

impl MPNGBuilder {
    fn new() -> Self {
        MPNGBuilder {
            header: None,
            comment: None,
            data: None,
        }
    }

    fn build(self) -> Result<MPNG, &'static str> {
        let header = self.header.ok_or("Missing header block")?;
        let data = self.data.ok_or("Missing data block")?;
        Ok(MPNG {
            header,
            comment: self.comment,
            data,
        })
    }

    fn set_header(&mut self, header: MPNGHeader) {
        self.header = Some(header);
    }

    fn set_comment(&mut self, comment: MPNGComment) {
        self.comment = Some(comment);
    }

    fn set_data(&mut self, data: MPNGData) {
        self.data = Some(data);
    }
}

pub struct Parser {
    reader: std::io::BufReader<std::fs::File>,
    builder: MPNGBuilder,
}

impl Parser {
    pub fn new(filename: &str) -> Self {
        let file = std::fs::File::open(filename).unwrap();
        let reader = std::io::BufReader::new(file);
        let builder = MPNGBuilder::new();
        Parser { reader, builder }
    }

    pub fn parse(&mut self) -> MPNG {
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


        self.builder.set_header(MPNGHeader {
            width,
            height,
            pixel_type: match pixel_type {
                0 => PixelType::BlackAndWhite,
                1 => PixelType::GreyScale,
                2 => PixelType::Palette,
                3 => PixelType::TrueColor,
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
        self.builder.set_comment(MPNGComment { text });

        return true;
    }

    fn parse_data(&mut self) -> bool {
        let length = self.reader.read_u32::<BigEndian>().unwrap();
        let mut data = vec![0; length as usize];
        self.reader.read_exact(&mut data).unwrap();

        self.builder.set_data(MPNGData { data });

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
