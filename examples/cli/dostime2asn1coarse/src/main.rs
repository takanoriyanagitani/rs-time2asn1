use std::io::{self, Read};
use std::process::ExitCode;

use rs_time2asn1::{CompactTimeCoarse, DosTime, RawDosTime};

fn reader2rawdostime16bits<R: Read>(mut rdr: R) -> Result<u16, io::Error> {
    let mut buf = [0; 2];
    rdr.read_exact(&mut buf)?;
    Ok(u16::from_be_bytes(buf))
}

fn raw2dostime(raw: u16) -> Result<DosTime, io::Error> {
    let raw_dos_time = RawDosTime { value: raw };
    raw_dos_time
        .try_into()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn dostime2der(dos: DosTime) -> Result<Vec<u8>, io::Error> {
    let compact_time_coarse: CompactTimeCoarse = dos.into();
    compact_time_coarse.to_der_bytes().map_err(io::Error::other)
}

fn raw2der(raw: u16) -> Result<Vec<u8>, io::Error> {
    let dos_time = raw2dostime(raw)?;
    dostime2der(dos_time)
}

fn der2writer<W: io::Write>(mut wtr: W, der: Vec<u8>) -> Result<(), io::Error> {
    wtr.write_all(&der)
}

fn sub() -> Result<(), io::Error> {
    let raw_dos_time_value = reader2rawdostime16bits(io::stdin())?;
    let der_output = raw2der(raw_dos_time_value)?;
    der2writer(io::stdout(), der_output)?;
    Ok(())
}

fn main() -> ExitCode {
    match sub() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
