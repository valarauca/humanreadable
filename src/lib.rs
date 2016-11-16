//!Human Readable Crate
//!
//!This crate implements both IEC and SI prefixes for integer data types.
//!This will allow for very large file numbers to be displayed in a human
//!parseable way.
//!
//!This crate no dependencies, including std. So it can be used in `#![no_std]` projects. 
//!
//!The difference between IEC and SI is thus:
//!
//!* SI uses base 1000
//!
//!* IEC uses base 1024
//!
//!The enums contains within respect the naming convention sticking to `KiB`,`GiB` for IEC types.
//!and using `KB`, or `GB` type enum names for SI types.
//!

#![no_std]

use core::convert::Into;
use core::fmt;

const iec_prefix: [u64; 7] = [
    1,
    1024,
    1024*1024,
    1024*1024*1024,
    1024*1024*1024*1024,
    1024*1024*1024*1024*1024,
    1024*1024*1024*1024*1024*1024
];

///
///Find Position within prefix array
///
#[inline(always)]
fn iec_position(x: u64) -> usize {
    for item in 0..6 {
        if x >= iec_prefix[item] && x < iec_prefix[item+1] {
            return item;
        }
    }
    7
}
#[test]
fn test_iec_position() {
    assert_eq!(iec_position(5),0);
    assert_eq!(iec_position(5000),1);
    assert_eq!(iec_position(1073741824),3);
}


///Holds a value that represents the fractional portion of a IEC/JEDEC
///binary prefix.
///
///Please note: these are not SI prefixes. They are defined by powers of
///1024 not 1000 like SI.
#[derive(Clone,Copy,PartialEq,PartialOrd)]
pub enum IEC {
    B(f64),
    KiB(f64),
    MiB(f64),
    GiB(f64),
    TiB(f64),
    PiB(f64),
    EiB(f64),
}

impl IEC {
    
    #[inline(always)]
    pub fn new(x: u64) -> IEC {
        let index = iec_position(x);
        let item = iec_prefix[index] as f64;
        let thing = x as f64;
        let item = thing/item;
        match index {
            0 => IEC::B(item),
            1 => IEC::KiB(item),
            2 => IEC::MiB(item),
            3 => IEC::GiB(item),
            4 => IEC::TiB(item),
            5 => IEC::PiB(item),
            6 => IEC::EiB(item),
            _ => unreachable!()
        }
    }

    #[inline(always)]
    pub fn get_val(&self) -> f64 {
        match self {
            &IEC::B(x) => x,
            &IEC::KiB(x) => x,
            &IEC::MiB(x) => x,
            &IEC::GiB(x) => x,
            &IEC::TiB(x) => x,
            &IEC::PiB(x) => x,
            &IEC::EiB(x) => x,
        }
    }
}

macro_rules! into_trait {
    ($code: ty) => {
        impl Into<IEC> for $code {
            fn into(self) -> IEC {
                IEC::new(self as u64)
            }
        }
    };
}

into_trait!(u8);
into_trait!(u16);
into_trait!(u32);
into_trait!(usize);
into_trait!(u64);
into_trait!(i8);
into_trait!(i16);
into_trait!(i32);
into_trait!(isize);
into_trait!(i64);

impl fmt::Display for IEC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &IEC::B(x) => write!(f,"{:.*}B",2,x),
            &IEC::KiB(x) => write!(f,"{:.*}KiB",2,x),
            &IEC::MiB(x) => write!(f,"{:.*}MiB",2,x),
            &IEC::GiB(x) => write!(f,"{:.*}GiB",2,x),
            &IEC::TiB(x) => write!(f,"{:.*}TiB",2,x),
            &IEC::PiB(x) => write!(f,"{:.*}PiB",2,x),
            &IEC::EiB(x) => write!(f,"{:.*}EiB",2,x),
        }
    }
}
impl fmt::Debug for IEC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &IEC::B(x) => write!(f,"{:.*}B",2,x),
            &IEC::KiB(x) => write!(f,"{:.*}KiB",2,x),
            &IEC::MiB(x) => write!(f,"{:.*}MiB",2,x),
            &IEC::GiB(x) => write!(f,"{:.*}GiB",2,x),
            &IEC::TiB(x) => write!(f,"{:.*}TiB",2,x),
            &IEC::PiB(x) => write!(f,"{:.*}PiB",2,x),
            &IEC::EiB(x) => write!(f,"{:.*}EiB",2,x),
        }
    }
}
        

