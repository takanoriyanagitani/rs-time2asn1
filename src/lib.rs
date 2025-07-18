use std::io;

use der::Choice;
use der::Encode;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RawDosTime {
    pub value: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DosTime {
    hours: u8,
    minutes: u8,
    two_second_intervals: u8,
}

impl DosTime {
    pub fn hours(&self) -> u8 {
        self.hours
    }

    pub fn minutes(&self) -> u8 {
        self.minutes
    }

    pub fn two_second_intervals(&self) -> u8 {
        self.two_second_intervals
    }
}

impl TryFrom<RawDosTime> for DosTime {
    type Error = &'static str;

    fn try_from(raw_time: RawDosTime) -> Result<Self, Self::Error> {
        let hours = ((raw_time.value >> 11) & 0x1F) as u8;
        let minutes = ((raw_time.value >> 5) & 0x3F) as u8;
        let two_second_intervals = (raw_time.value & 0x1F) as u8;

        if hours > 23 || minutes > 59 || two_second_intervals > 29 {
            return Err("Invalid DOS time value");
        }

        Ok(DosTime {
            hours,
            minutes,
            two_second_intervals,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Choice)]
pub enum CompactTimeCoarse {
    #[asn1(tag_mode = "IMPLICIT", context_specific = "0")]
    H0(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "1")]
    H1(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "2")]
    H2(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "3")]
    H3(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "4")]
    H4(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "5")]
    H5(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "6")]
    H6(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "7")]
    H7(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "8")]
    H8(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "9")]
    H9(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "10")]
    H10(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "11")]
    H11(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "12")]
    H12(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "13")]
    H13(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "14")]
    H14(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "15")]
    H15(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "16")]
    H16(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "17")]
    H17(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "18")]
    H18(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "19")]
    H19(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "20")]
    H20(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "21")]
    H21(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "22")]
    H22(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "23")]
    H23(u8),
}

impl CompactTimeCoarse {
    pub fn to_der_bytes(&self) -> Result<Vec<u8>, io::Error> {
        self.to_der().map_err(io::Error::other)
    }
}

impl From<DosTime> for CompactTimeCoarse {
    fn from(dos_time: DosTime) -> Self {
        match dos_time.hours() {
            0 => CompactTimeCoarse::H0(dos_time.minutes()),
            1 => CompactTimeCoarse::H1(dos_time.minutes()),
            2 => CompactTimeCoarse::H2(dos_time.minutes()),
            3 => CompactTimeCoarse::H3(dos_time.minutes()),
            4 => CompactTimeCoarse::H4(dos_time.minutes()),
            5 => CompactTimeCoarse::H5(dos_time.minutes()),
            6 => CompactTimeCoarse::H6(dos_time.minutes()),
            7 => CompactTimeCoarse::H7(dos_time.minutes()),
            8 => CompactTimeCoarse::H8(dos_time.minutes()),
            9 => CompactTimeCoarse::H9(dos_time.minutes()),
            10 => CompactTimeCoarse::H10(dos_time.minutes()),
            11 => CompactTimeCoarse::H11(dos_time.minutes()),
            12 => CompactTimeCoarse::H12(dos_time.minutes()),
            13 => CompactTimeCoarse::H13(dos_time.minutes()),
            14 => CompactTimeCoarse::H14(dos_time.minutes()),
            15 => CompactTimeCoarse::H15(dos_time.minutes()),
            16 => CompactTimeCoarse::H16(dos_time.minutes()),
            17 => CompactTimeCoarse::H17(dos_time.minutes()),
            18 => CompactTimeCoarse::H18(dos_time.minutes()),
            19 => CompactTimeCoarse::H19(dos_time.minutes()),
            20 => CompactTimeCoarse::H20(dos_time.minutes()),
            21 => CompactTimeCoarse::H21(dos_time.minutes()),
            22 => CompactTimeCoarse::H22(dos_time.minutes()),
            23 => CompactTimeCoarse::H23(dos_time.minutes()),
            _ => unreachable!(), // DosTime hours are validated to be 0-23
        }
    }
}
