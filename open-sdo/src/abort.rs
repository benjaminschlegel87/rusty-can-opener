pub enum SdoAbortCode {
    CommunicationTimeout,
    UnknownAbortCode,
}

impl From<u32> for SdoAbortCode {
    fn from(abort_code: u32) -> Self {
        match abort_code {
            0x0504_0000 => Self::CommunicationTimeout,
            _ => Self::UnknownAbortCode,
        }
    }
}
impl From<SdoAbortCode> for u32 {
    fn from(code: SdoAbortCode) -> Self {
        match code {
            SdoAbortCode::CommunicationTimeout => 0x0504_0000,
            SdoAbortCode::UnknownAbortCode => 0,
        }
    }
}

pub fn add_abort(buffer: &mut [u8; 4], code: SdoAbortCode) {
    let code_u32 = u32::from(code);
    let by: [u8; 4] = code_u32.to_le_bytes();
    *buffer = by;
}
