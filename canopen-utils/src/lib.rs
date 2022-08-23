use std::ops::RangeInclusive;

const SDO_REQUIRED_DLC: usize = 8;
const EMCY_REQUIRED_DLC: usize = 8;
const HB_REQUIRED_DLC: usize = 1;
const NMT_REQUIRED_DLC: usize = 2;

#[derive(Debug, PartialEq)]

pub struct CANopenFrame<const DLC: usize> {
    cob_id: u16,
    data: [u8; DLC],
}

#[derive(Debug)]
pub enum CANopenFrameError {
    SdoWrongDlc,
    SdoWrongId,
}
// Is is_sdo_msg method necessary? Should it also check dlc and we provide a From<> trait
// Error Types
#[derive(Debug)]
pub struct SdoFrame {
    frame: CANopenFrame<8>,
}
impl TryFrom<CANopenFrame<8>> for SdoFrame {
    type Error = CANopenFrameError;

    fn try_from(value: CANopenFrame<8>) -> Result<Self, Self::Error> {
        if CANopenFrame::<8>::SDO_ID_RANGE.contains(&value.cob_id) {
            Ok(Self { frame: value })
        } else {
            Err(CANopenFrameError::SdoWrongId)
        }
    }
}
impl From<SdoFrame> for CANopenFrame<8> {
    /// From a SdoFrame we can always create a [CANopenFrame] infallible
    fn from(sdo_frame: SdoFrame) -> Self {
        CANopenFrame {
            cob_id: sdo_frame.frame.cob_id,
            data: sdo_frame.frame.data,
        }
    }
}

impl<const DLC: usize> CANopenFrame<DLC> {
    // TODO Research correctness of ID ranges and DLC requirements
    const NMT_ID_RANGE: RangeInclusive<u16> = 0..=0x07F;
    const EMCY_ID_RANGE: RangeInclusive<u16> = 0x081..=0x0FF;
    const SDO_ID_RANGE: RangeInclusive<u16> = 0x581..=0x67F;
    const HB_ID_RANGE: RangeInclusive<u16> = 0x701..=0x77F;
}
macro_rules! ImplRawFrameApi {
    ($size:expr) => {
        impl CANopenFrame<$size> {
            pub fn new(can_id: u32, data: [u8; $size]) -> Option<Self> {
                if can_id > 0x77F {
                    None
                } else {
                    // Safe because in this branch can_id is < 0x77F
                    let cob_id = can_id as u16;
                    Some(Self { cob_id, data })
                }
            }
            pub fn get_data(&self) -> &[u8] {
                &self.data
            }
        }
    };
}
ImplRawFrameApi!(HB_REQUIRED_DLC);
ImplRawFrameApi!(NMT_REQUIRED_DLC);
ImplRawFrameApi!(3);
ImplRawFrameApi!(4);
ImplRawFrameApi!(5);
ImplRawFrameApi!(6);
ImplRawFrameApi!(7);
ImplRawFrameApi!(SDO_REQUIRED_DLC);

impl CANopenFrame<8> {
    pub fn is_emcy_msg(&self) -> bool {
        CANopenFrame::<8>::EMCY_ID_RANGE.contains(&self.cob_id)
    }
    pub fn is_sdo_msg(&self) -> bool {
        CANopenFrame::<8>::SDO_ID_RANGE.contains(&self.cob_id)
    }
}
impl CANopenFrame<2> {
    pub fn is_nmt_msg(&self) -> bool {
        CANopenFrame::<2>::NMT_ID_RANGE.contains(&self.cob_id)
    }
}
impl CANopenFrame<1> {
    pub fn is_hb_msg(&self) -> bool {
        CANopenFrame::<1>::HB_ID_RANGE.contains(&self.cob_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_api_test() {
        assert_eq!(
            None,
            CANopenFrame::<8>::new(0x780, [0x43, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00])
        );

        let sdo_msg =
            CANopenFrame::<8>::new(0x604, [0x43, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00])
                .expect("sdo msg not created");
        assert_eq!(sdo_msg.is_sdo_msg(), true);
        assert_eq!(sdo_msg.is_emcy_msg(), false);

        let nmt_msg = CANopenFrame::<2>::new(0x000, [0x7F, 0x00]).expect("sdo msg not created");
        assert_eq!(nmt_msg.is_nmt_msg(), true);

        let emcy_msg =
            CANopenFrame::<8>::new(0x086, [0x7F, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
                .expect("sdo msg not created");
        assert_eq!(emcy_msg.is_sdo_msg(), false);
        assert_eq!(emcy_msg.is_emcy_msg(), true);

        let hb_msg = CANopenFrame::<1>::new(0x706, [0x7F]).expect("hb msg not created");
        assert_eq!(hb_msg.is_hb_msg(), true);
    }
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
