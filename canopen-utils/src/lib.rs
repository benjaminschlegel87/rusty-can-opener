pub struct CANopenFrame {
    cob_id: u16,
    dlc: u8,
    data: [u8; 8],
}

impl CANopenFrame {
    pub fn new(can_id: u32, dlc: u8, data: [u8; 8]) -> Option<Self> {
        if dlc > 8 {
            None
        } else {
            if can_id > 0x77F {
                None
            } else {
                let cob_id = u16::try_from(can_id);
                if let Ok(id) = cob_id {
                    Some(Self {
                        cob_id: id,
                        dlc,
                        data,
                    })
                } else {
                    None
                }
            }
        }
    }
    pub fn try_sdo_msg(&mut self) -> Option<Self> {
        match self.cob_id {
            0x581..=0x67F => Some(Self {
                cob_id: self.cob_id,
                dlc: self.dlc,
                data: self.data,
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_api_test() {
        let msg = CANopenFrame::new(0x604, 8, [0x43, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00])
            .expect("msg not created");
    }
}
