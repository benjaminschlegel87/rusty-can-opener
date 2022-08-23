use std::ops::RangeInclusive;

#[derive(Debug, PartialEq)]
pub struct CANopenFrame {
    cob_id: u16,
    dlc: u8,
    data: [u8; 8],
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
    frame: CANopenFrame,
}
impl TryFrom<CANopenFrame> for SdoFrame {
    type Error = CANopenFrameError;

    fn try_from(value: CANopenFrame) -> Result<Self, Self::Error> {
        if CANopenFrame::SDO_ID_RANGE.contains(&value.cob_id) {
            if value.dlc != CANopenFrame::HB_REQUIRED_DLC {
                Err(CANopenFrameError::SdoWrongDlc)
            } else {
                Ok(Self { frame: value })
            }
        } else {
            Err(CANopenFrameError::SdoWrongId)
        }
    }
}
impl From<SdoFrame> for CANopenFrame {
    /// From a SdoFrame we can always create a [CANopenFrame] infallible
    fn from(sdo_frame: SdoFrame) -> Self {
        CANopenFrame {
            cob_id: sdo_frame.frame.cob_id,
            dlc: sdo_frame.frame.dlc,
            data: sdo_frame.frame.data,
        }
    }
}
macro_rules! ImplApi {
    ($ty:ident) => {
        impl $ty {
            pub fn get_id(&self) -> u16 {
                self.frame.cob_id
            }
            pub fn get_data(&self) -> &[u8] {
                &self.frame.data
            }
        }
    };
}
ImplApi!(SdoFrame);

impl CANopenFrame {
    // TODO Research correctness of ID ranges and DLC requirements
    const NMT_ID_RANGE: RangeInclusive<u16> = 0..=0x07F;
    const EMCY_ID_RANGE: RangeInclusive<u16> = 0x081..=0x0FF;
    const SDO_ID_RANGE: RangeInclusive<u16> = 0x581..=0x67F;
    const HB_ID_RANGE: RangeInclusive<u16> = 0x701..=0x77F;

    const SDO_REQUIRED_DLC: u8 = 8;
    const EMCY_REQUIRED_DLC: u8 = 8;
    const HB_REQUIRED_DLC: u8 = 1;
    const NMT_REQUIRED_DLC: u8 = 2;

    pub fn new(can_id: u32, dlc: u8, data: [u8; 8]) -> Option<Self> {
        if dlc > 8 {
            None
        } else {
            if can_id > 0x77F {
                None
            } else {
                // Safe because in this branch can_id is < 0x77F
                let cob_id = can_id as u16;
                Some(Self { cob_id, dlc, data })
            }
        }
    }
    pub fn is_nmt_msg(&self) -> bool {
        CANopenFrame::NMT_ID_RANGE.contains(&self.cob_id)
    }
    pub fn is_emcy_msg(&self) -> bool {
        CANopenFrame::EMCY_ID_RANGE.contains(&self.cob_id)
    }
    pub fn is_sdo_msg(&self) -> bool {
        CANopenFrame::SDO_ID_RANGE.contains(&self.cob_id)
    }
    pub fn is_hb_msg(&self) -> bool {
        CANopenFrame::HB_ID_RANGE.contains(&self.cob_id)
    }
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_api_test() {
        assert_eq!(
            None,
            CANopenFrame::new(0x604, 9, [0x43, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00])
        );
        assert_eq!(
            None,
            CANopenFrame::new(0x780, 8, [0x43, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00])
        );

        let mut sdo_msg =
            CANopenFrame::new(0x604, 8, [0x43, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00])
                .expect("sdo msg not created");
        assert_eq!(sdo_msg.is_sdo_msg(), true);
        assert_eq!(sdo_msg.is_emcy_msg(), false);
        assert_eq!(sdo_msg.is_hb_msg(), false);
        assert_eq!(sdo_msg.is_nmt_msg(), false);

        let mut nmt_msg =
            CANopenFrame::new(0x000, 2, [0x7F, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
                .expect("sdo msg not created");
        assert_eq!(nmt_msg.is_sdo_msg(), false);
        assert_eq!(nmt_msg.is_emcy_msg(), false);
        assert_eq!(nmt_msg.is_hb_msg(), false);
        assert_eq!(nmt_msg.is_nmt_msg(), true);

        let mut emcy_msg =
            CANopenFrame::new(0x086, 2, [0x7F, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
                .expect("sdo msg not created");
        assert_eq!(emcy_msg.is_sdo_msg(), false);
        assert_eq!(emcy_msg.is_emcy_msg(), true);
        assert_eq!(emcy_msg.is_hb_msg(), false);
        assert_eq!(emcy_msg.is_nmt_msg(), false);

        let mut hb_msg =
            CANopenFrame::new(0x706, 2, [0x7F, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
                .expect("hb msg not created");
        assert_eq!(hb_msg.is_sdo_msg(), false);
        assert_eq!(hb_msg.is_emcy_msg(), false);
        assert_eq!(hb_msg.is_hb_msg(), true);
        assert_eq!(hb_msg.is_nmt_msg(), false);
    }
    #[test]
    fn into_sdo() {
        let raw_msg = CANopenFrame::new(0x604, 8, [0x43, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00])
            .expect("sdo msg not created");
        let sdo_msg: SdoFrame =
            SdoFrame::try_from(raw_msg).expect("This should be a valid SDO Frame");
        // raw_msg: borrow of moved value
        // check this works as expected
        // raw_msg.is_emcy_msg();
    }
}
