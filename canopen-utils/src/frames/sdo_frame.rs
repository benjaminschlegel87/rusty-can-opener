use super::raw_frame::*;
/// is_sdo_msg sollte wohl hier rein und wir bieten einen convert methode an
#[derive(Debug, PartialEq)]
pub struct SdoFrame {
    frame: CANopenFrame<8>,
}
#[derive(Debug, Default)]
pub struct SdoParts {
    cob_id: u16,
    cmd: u8,
    index: u16,
    subindex: u8,
    data: u32,
}
impl From<SdoFrame> for CANopenFrame<8> {
    /// From a SdoFrame we can always create a [CANopenFrame] infallible
    fn from(sdo_frame: SdoFrame) -> Self {
        sdo_frame.frame
    }
}
impl SdoFrame {
    pub fn try_convert(frame: CANopenFrame<8>) -> Result<Self, CANopenFrame<8>> {
        if SDO_ID_RANGE.contains(&frame.get_id()) == true {
            Ok(Self { frame })
        } else {
            Err(frame)
        }
    }
    pub fn to_parts(self) -> SdoParts {
        let payload = self.frame.get_data();
        let mut parts = SdoParts::default();
        parts.cob_id = self.frame.get_id();
        parts.cmd = payload[0];
        // Split slice at CANopen Index position
        let (_, remain) = payload.split_at(1);
        let (index_slice, _) = remain.split_at(2);
        // Make a u16 out byte [1][2] treated as little endian
        // here we know that slice.len() is 8 so this can be unwrapped
        // try_into() checks for a exact fit -> for Array [T; N]
        // N == slice.len() must be valid otherwise it returns Err
        let arr = index_slice.try_into();
        parts.index = u16::from_le_bytes(arr.unwrap());
        parts.subindex = payload[3];
        // Make a u32 out byte [4][5][6][7] treated as little endian
        // here we know that slice.len() is 8 so this can be unwrapped
        let (_, data_slice) = payload.split_at(4);
        parts.data = u32::from_le_bytes(data_slice.try_into().unwrap());
        parts
    }
    pub fn from_parts(parts: SdoParts) -> Self {
        let mut payload: [u8; 8] = [0u8; 8];
        payload[0] = parts.cmd;
        let index_slice: [u8; 2] = u16::to_le_bytes(parts.index);
        payload[1..=2].clone_from_slice(&index_slice);
        payload[3] = parts.subindex;
        let data_slice: [u8; 4] = u32::to_le_bytes(parts.data);
        payload[4..=7].clone_from_slice(&data_slice);

        // Only way to have SdoParts is by getting it from a SdoFrame
        // Only way to have a SdoFrame is to get it via try_convert()
        // We know that cob_id of SdoParts is always a valid COB-ID and
        // we can unwrap here
        Self {
            frame: CANopenFrame::<8>::new(parts.cob_id as u32, payload).unwrap(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn into_sdo_happy() {
        // Valid SDO Read Msg
        let raw_msg =
            CANopenFrame::<8>::new(0x604, [0x43, 0x00, 0x10, 0x90, 0x0E, 0xF0, 0x20, 0xEF])
                .expect("raw can msg not created");
        // Convert it to a SdoFrame
        let sdo_msg = SdoFrame::try_convert(raw_msg).expect("msg should convert to a sdo frame");
        assert_eq!(0x604, sdo_msg.frame.get_id());
        // Pull SdoFrame to SdoParts
        let parts = sdo_msg.to_parts();
        assert_eq!(0x604, parts.cob_id);
        assert_eq!(0x43, parts.cmd);
        assert_eq!(0x1000, parts.index);
        assert_eq!(0x90, parts.subindex);
        assert_eq!(0xEF20F00E, parts.data);
        let res_sdo_msg = SdoFrame::from_parts(parts);
        let ref_msg =
            CANopenFrame::<8>::new(0x604, [0x43, 0x00, 0x10, 0x90, 0x0E, 0xF0, 0x20, 0xEF])
                .expect("raw can msg not created");
        assert_eq!(ref_msg, res_sdo_msg.into());
    }

    #[test]
    fn into_sdo_range_check() {
        assert_eq!(
            Err(CANopenFrame::<8>::new(0x580, [0u8; 8]).unwrap()),
            SdoFrame::try_convert(CANopenFrame::<8>::new(0x580, [0u8; 8]).unwrap())
        );

        assert_eq!(
            Err(CANopenFrame::<8>::new(0x680, [0u8; 8]).unwrap()),
            SdoFrame::try_convert(CANopenFrame::<8>::new(0x680, [0u8; 8]).unwrap())
        );
        SdoFrame::try_convert(CANopenFrame::<8>::new(0x581, [0u8; 8]).unwrap())
            .expect("This must works");
        SdoFrame::try_convert(CANopenFrame::<8>::new(0x67F, [0u8; 8]).unwrap())
            .expect("This must works");
    }

    #[test]
    fn into_sdo_as_service_check() {
        let msg =
            CANopenFrame::<8>::new(0x601, [0u8; 8]).expect("valid msg given - should not fail");

        let o = match SdoFrame::try_convert(msg) {
            Ok(_sdo) => {
                // Works on SDO
                (false, None)
            }
            Err(raw_frame) => (true, Some(raw_frame)),
        };
        assert_eq!((false, None), o);
        // do stuff with msg
    }

    #[test]
    fn into_sdo_as_service_check_2() {
        let msg =
            CANopenFrame::<8>::new(0x082, [0u8; 8]).expect("valid msg given - should not fail");

        let sdo_checked = match SdoFrame::try_convert(msg) {
            Ok(_sdo) => {
                // Works on SDO
                (false, None)
            }
            Err(raw_frame) => (true, Some(raw_frame)),
        };
        assert_eq!(true, sdo_checked.0);
        assert_eq!(true, sdo_checked.1.unwrap().is_emcy_msg());
    }
}
