use std::process::exit;
use std::fs;
use std::env;
use std::io::{Read, Write};
use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
enum Font {
    NotoSansMonoBold,
    Mplus1pBlack,
    RoundedXMplus1pBlack,
    IPAmjMinchou,
    AoyagiReisyoShimo,
    LinLibertineRBah
}

#[derive(Debug)]
enum FontParseError {
    Unexpected
}

impl FromStr for Font {
    type Err = FontParseError ;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "notosans-mono-bold"        | "NotoSansMonoBold"        => Ok(Self::NotoSansMonoBold),
            "mplus-1p-black"            | "Mplus1pBlack"            => Ok(Self::Mplus1pBlack),
            "rouded-x-mplus-1p-black"   | "RoundedXMplus1pBlack"    => Ok(Self::RoundedXMplus1pBlack),
            "ipamjm"                    | "IPAmjMinchou"            => Ok(Self::IPAmjMinchou),
            "aoyagireisyoshimo"         | "AoyagiReisyoShimo"       => Ok(Self::AoyagiReisyoShimo),
            "LinLibertine_RBah"         | "LinLibertineRBah"        => Ok(Self::LinLibertineRBah),
            _ => Err(FontParseError::Unexpected)
        }
    }
}

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotoSansMonoBold      => write!(f, "notosans-mono-bold"),    
            Self::Mplus1pBlack          => write!(f, "mplus-1p-black"),
            Self::RoundedXMplus1pBlack  => write!(f, "rounded-x-mplus-1p-black"),
            Self::IPAmjMinchou          => write!(f, "ipamjm"),
            Self::AoyagiReisyoShimo     => write!(f, "aoyagireisyoshimo"),
            Self::LinLibertineRBah      => write!(f, "LinLibertine_RBah"),
        }
    }
}

pub fn font_list() {
    println!("You can select font in:");
    println!(" - {}", Font::NotoSansMonoBold);
    println!(" - {}", Font::Mplus1pBlack);
    println!(" - {}", Font::RoundedXMplus1pBlack);
    println!(" - {}", Font::IPAmjMinchou);
    println!(" - {}", Font::AoyagiReisyoShimo);
    println!(" - {}", Font::LinLibertineRBah)
}

#[derive(Debug)]
pub struct Generator<'g> {
    font: Font,
    color: u32,
    back_color: u32,
    text: &'g str
}

impl<'g> Generator<'g> {
    pub fn new() -> Self {
        Generator {
            font: Font::NotoSansMonoBold,
            color: 0xEC71A1FF,
            back_color: 0x00000000,
            text: "emo"
        }
    }

    pub fn font(&mut self, font: &str) -> &mut Self {
        match font.parse() {
            Ok(font) => {
                self.font = font;
                return self;
            },
            Err(e) => {
                eprintln!("{} -> {:?}", font, e);
                eprintln!("Hint: you can select font from `$ emo --font-list`");
                exit(1);
            }
        }
    }

    pub fn color(&mut self, color: &str) -> &mut Self {
        match u32::from_str_radix(color, 16) {
            Ok(color) => {
                self.color = color;
                return self;
            },
            Err(e) => {
                eprintln!("{} -> {:?}", color, e);
                exit(1);
            }
        }
    }

    pub fn back_color(&mut self, color: &str) -> &mut Self {
        match u32::from_str_radix(color, 16) {
            Ok(color) => {
                self.back_color = color;
                return self;
            },
            Err(e) => {
                eprintln!("{} -> {:?}", color, e);
                exit(1);
            }
        }
    }

    pub fn text(&mut self, text: &'g str) -> &mut Self {
        self.text = text;
        self
    }

    pub fn gen(&self, path: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = fs::File::create(path.unwrap_or(&format!("{}/emoji/{}.png", env::var("HOME").unwrap(), self.text)))?;
        
        let resp = ureq::get("https://emoji-gen.ninja/emoji")
            .query("align", "center")
            .query("back_color", &format!("{:<08X}", self.back_color))
            .query("color", &format!("{:<08X}", self.color))
            .query("font", &format!("{}", self.font))
            .query("locale", "ja")
            .query("public_fg", "false")
            .query("size_fixed", "false")
            .query("stretch", "true")
            .query("text", self.text)
            .call();
    
        let mut buf = {
            let len = resp.header("Content-Length").unwrap().parse::<usize>().unwrap();
            vec![ 0; len]
        };
        let mut reader = resp.into_reader();
        reader.read_exact(&mut buf)?;
        file.write_all(&buf)?;

        Ok(())
    }
}
