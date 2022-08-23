use super::raw_frame::*;
// Is is_sdo_msg method necessary? Should it also check dlc and we provide a From<> trait
// Error Types
#[derive(Debug)]
pub struct SdoFrame {
    frame: CANopenFrame<8>,
}
impl TryFrom<CANopenFrame<8>> for SdoFrame {
    type Error = CANopenFrameError;

    fn try_from(value: CANopenFrame<8>) -> Result<Self, Self::Error> {
        if SDO_ID_RANGE.contains(&value.get_id()) {
            Ok(Self { frame: value })
        } else {
            Err(CANopenFrameError::SdoWrongId)
        }
    }
}
impl From<SdoFrame> for CANopenFrame<8> {
    /// From a SdoFrame we can always create a [CANopenFrame] infallible
    fn from(sdo_frame: SdoFrame) -> Self {
        sdo_frame.frame
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn into_sdo() {
        let raw_msg =
            CANopenFrame::<8>::new(0x604, [0x43, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00])
                .expect("sdo msg not created");
        let sdo_msg: SdoFrame =
            SdoFrame::try_from(raw_msg).expect("This should be a valid SDO Frame");
        // raw_msg: borrow of moved value
        // check this works as expected
        // raw_msg.is_emcy_msg();
        let out_msg = CANopenFrame::from(sdo_msg);
        assert_eq!(0x43, out_msg.get_data()[0]);
    }
}
