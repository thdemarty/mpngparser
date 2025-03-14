use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PixelType {
    BlackAndWhite = 0,
    GreyScale = 1,
    Palette = 2,
    TrueColor = 3,
}
#[derive(Debug, Clone, Copy)]
pub struct MPNGHeader {
    pub width: u32,
    pub height: u32,
    pub pixel_type: PixelType,
}
#[derive(Debug, Clone)]
pub struct MPNGComment {
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct MPNGData {
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct MPNG {
    pub header: MPNGHeader,
    pub comment: Option<MPNGComment>,
    pub data: MPNGData,
}

impl fmt::Display for MPNG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Print as follow
        // Width: 8
        // Height: 10
        // Pixel Type: 0 # Black & White
        // Comment: La lettre A
        write!(f, "Width: {}\n", self.header.width)?;
        write!(f, "Height: {}\n", self.header.height)?;
        match self.header.pixel_type {
            PixelType::BlackAndWhite => write!(f, "Pixel Type: 0 (Black & White)\n")?,
            PixelType::GreyScale => write!(f, "Pixel Type: 1 (Grey Scale)\n")?,
            PixelType::Palette => write!(f, "Pixel Type: 2 (Palette)\n")?,
            PixelType::TrueColor => write!(f, "Pixel Type: 3 (True Color)\n")?,
        }
        write!(f, "Comment: {}\n", self.comment.as_ref().unwrap().text)?;
        self.print();
        Ok(())
    }
}

impl MPNG {
    pub fn print(&self) {
        // display each bytes as bit

        let mut counter = 0;
        for byte in &self.data.data {
            for i in 0..8 {
                let pixel_value = (byte >> i) & 1;
                if pixel_value == 1 {
                    print!("X");
                } else {
                    print!(" ");
                }
                
                counter += 1;

                if counter % self.header.width == 0 {
                    println!();
                }

            }
        }
    }
}