use std::ops::RangeInclusive;

#[derive(Debug, PartialEq)]
pub struct CANopenFrame {
    cob_id: u16,
    dlc: u8,
    data: [u8; 8],
}

impl CANopenFrame {
    // 0 is fine because it is used from NMT Master to send NMT command to all nodes present
    const NMT_ID_RANGE: RangeInclusive<u16> = 0..=0x07F;
    const EMCY_ID_RANGE: RangeInclusive<u16> = 0x081..=0x0FF;
    const SDO_ID_RANGE: RangeInclusive<u16> = 0x581..=0x67F;
    const HB_ID_RANGE: RangeInclusive<u16> = 0x701..=0x77F;
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
    pub fn is_nmt_msg(&mut self) -> bool {
        CANopenFrame::NMT_ID_RANGE.contains(&self.cob_id)
    }
    pub fn is_emcy_msg(&mut self) -> bool {
        CANopenFrame::EMCY_ID_RANGE.contains(&self.cob_id)
    }
    pub fn is_sdo_msg(&mut self) -> bool {
        CANopenFrame::SDO_ID_RANGE.contains(&self.cob_id)
    }
    pub fn is_hb_msg(&mut self) -> bool {
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
}
