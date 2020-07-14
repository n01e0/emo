pub mod emoji;

use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
pub enum Font {
    NotoSansMonoBold,
    Mplus1pBlack,
    RoundedXMplus1pBlack,
    IPAmjMinchou,
    AoyagiReisyoShimo,
    LinLibertineRBah
}

#[derive(Debug)]
pub enum FontParseError {
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
