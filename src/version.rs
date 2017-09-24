
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version([u16; 3]);

impl Version {
    #[inline]
    pub fn new(major: u16, minor: u16, patch: u16) -> Version {
        Version([major, minor, patch])
    }
}

impl From<u32> for Version {
    fn from(val: u32) -> Version {
        Version([
            ((val & 0xffc00000) >> 22) as u16,
            ((val & 0x003ff000) >> 12) as u16,
            (val & 0x00000fff) as u16
        ])
    }
}

impl From<Version> for u32 {
    fn from(ver: Version) -> u32 {
        (ver.0[0] as u32) << 22 | (ver.0[1] as u32) << 12 | (ver.0[2] as u32)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.0[0], self.0[1], self.0[2])
    }
}
