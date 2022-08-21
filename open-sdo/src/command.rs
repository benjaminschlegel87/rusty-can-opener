use super::abort::{add_abort, SdoAbortCode};
use super::CanFrame;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Checks the Command byte and reroutes the msg to the appropriate module
/// This is the server view:
/// Every request is checked if it is either a read or a write
/// On the first read request:
/// - The request just is a Index/SubIndex + Read: Client does not know what kind of transfer is necessary
/// - Server checks Index/Subindex
/// a) Does Index/Subindex exist? if not -> abort
/// b) Is Read allowed on that object? if not -> abort
/// c) Determine the size of the object
///     4 bytes or smaller => Expedited
///     5 bytes or more => Segmented or Block
/// - Generate Response with either:
/// a) Payload für expedited and Command which indicates 1,2,3 or 4 bytes payload and Expedited Bit 1 is set
/// b) Total length >= 5 for segemented/block and Command Expedited Bit is unset and flag Data set size is indicated is set(Bit 0)
/// Draw state diagram for this and check cargo docs if we can add it here
#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum RequestCommand {
    Read = 0x40,
    SegReadToggle0 = 0x60,
    SegReadToggle1 = 0x70,
    ExpWriteB1 = 0x2F,
    ExpWriteB2 = 0x2B,
    ExpWriteB3 = 0x27,
    ExpWriteB4 = 0x23,
    SegWriteInit = 0x21, // Seg Write Init is a Write Req 0x20 + 0x01 for "Size is indicated"
}

#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ResponseCommand {
    ExpReadRespB1 = 0x4F, // Response on a valid Read Req Expedited with size 1 Bytes
    ExpReadRespB2 = 0x4B, // Response on a valid Read Req Expedited with size 2 Bytes
    ExpReadRespB3 = 0x47, // Response on a valid Read Req Expedited with size 3 Bytes
    ExpReadRespB4 = 0x43, // Response on a valid Read Req Expedited with size 4 Bytes
    SucWriteResp = 0x60,  // Reponse on a valid Write Req
    SegReadInitalLen = 0x41, // Initial Seg Read Response: Indicates the Length of the Transfer
    SegReadToggle0NotLast = 0x00, // Transfer ongoing: ToggleBit 0: Response on a segmented Read: All 7 Payload bytes are filled
    SegReadToggle1NotLast = 0x10, // Transfer ongoing: ToggleBit 1: Response on a segmented Read: All 7 Payload bytes are filled
    SegReadToggle0Last = 0x01, // Last Response of Transfer: ToggleBit 0: Response on a segmented Read: All 7 Payload bytes are filled
    SegReadToggle1Last = 0x11, // Last Response of Transfer: ToggleBit 1: Response on a segmented Read: All 7 Payload bytes are filled
    SegReadToggle0Valid6 = 0x03, // Last Response of Transfer: ToggleBit 0: Response on a segmented Read: Only 6 Bytes valid
    SegReadToggle1Valid6 = 0x13, // Last Response of Transfer: ToggleBit 1: Response on a segmented Read: Only 6 Bytes valid
    SegReadToggle0Valid5 = 0x05, // Last Response of Transfer: ToggleBit 0: Response on a segmented Read: Only 5 Bytes valid
    SegReadToggle1Valid5 = 0x15, // Last Response of Transfer: ToggleBit 1: Response on a segmented Read: Only 5 Bytes valid
    SegReadToggle0Valid4 = 0x07, // Last Response of Transfer: ToggleBit 0: Response on a segmented Read: Only 4 Bytes valid
    SegReadToggle1Valid4 = 0x17, // Last Response of Transfer: ToggleBit 1: Response on a segmented Read: Only 4 Bytes valid
    SegReadToggle0Valid3 = 0x09, // Last Response of Transfer: ToggleBit 0: Response on a segmented Read: Only 3 Bytes valid
    SegReadToggle1Valid3 = 0x19, // Last Response of Transfer: ToggleBit 1: Response on a segmented Read: Only 3 Bytes valid
    SegReadToggle0Valid2 = 0x0B, // Last Response of Transfer: ToggleBit 0: Response on a segmented Read: Only 2 Bytes valid
    SegReadToggle1Valid2 = 0x1B, // Last Response of Transfer: ToggleBit 1: Response on a segmented Read: Only 2 Bytes valid
    SegReadToggle0Valid1 = 0x0D, // Last Response of Transfer: ToggleBit 0: Response on a segmented Read: Only 1 Bytes valid
    SegReadToggle1Valid1 = 0x1D, // Last Response of Transfer: ToggleBit 1: Response on a segmented Read: Only 1 Bytes valid
    AbortTransfer = 0x80,
}

/// # POV Server
/// => incoming req
/// => outgoing resp
/// ## Exp Read
/// => Cmd: [0x40 => ; B7-5: 0b010 ; B4-0: 0x00] Index SubIndex 0x00u32
/// =< 4Byte Resp Cmd: [0x43 => ; B7-5: 0b010 ; B4: 0 ; 3-2: 0b00 ; B1: 1 ; B0: 0] Index SubIndex Data
/// =< 3Byte Resp Cmd: [0x47 => ; B7-5: 0b010 ; B4: 0 ; 3-2: 0b01 ; B1: 1 ; B0: 0] Index SubIndex Data
/// =< 2Byte Resp Cmd: [0x4B => ; B7-5: 0b010 ; B4: 0 ; 3-2: 0b10 ; B1: 1 ; B0: 0] Index SubIndex Data
/// =< 1Byte Resp Cmd: [0x4F => ; B7-5: 0b010 ; B4: 0 ; 3-2: 0b11 ; B1: 1 ; B0: 0] Index SubIndex Data
/// OR
/// =< Abort Resp CMD: [0x80 => ; B7-5: 0b100 ; B4-0: 0] Index Subindex AbortCode
///
/// ## Exp Write
/// => 4Byte Req Cmd: [0x23 => ; B7-5: 0b001 ; B4: 0 ; 3-2: 0b11 ; B1: 1 ; B0: 0] Index SubIndex Data
/// => 3Byte Req Cmd: [0x27 => ; B7-5: 0b001 ; B4: 0 ; 3-2: 0b10 ; B1: 1 ; B0: 0] Index SubIndex Data
/// => 2Byte Req Cmd: [0x2B => ; B7-5: 0b001 ; B4: 0 ; 3-2: 0b01 ; B1: 1 ; B0: 0] Index SubIndex Data
/// => 1Byte Req Cmd: [0x2F => ; B7-5: 0b001 ; B4: 0 ; 3-2: 0b00 ; B1: 1 ; B0: 0] Index SubIndex Data
/// <= Suc Resp  Cmd: [0x60 => ; B7-5: 0b011 ; B4-0: 0] Index Subindx 0x00u32
/// OR
/// =< Abort Resp CMD: [0x80 => ; B7-5: 0b100 ; B4-0: 0] Index Subindex AbortCode
///
/// ## Seg Read
/// starts with the same request as Exp Read
/// => Cmd: [0x40 => ; B7-5: 0b010 ; B4-0: 0x00] Index SubIndex 0x00u32
///                                                                Not Exp   LenIndic
/// =< Length Resp Cmd: [0x41 => ; B7-5: 0b010 ; B4: 0 ; 3-2: 0b00 ; B1: 0 ; B0: 1] Index SubIndex Data
/// OR
/// =< Abort Resp CMD: [0x80 => ; B7-5: 0b100 ; B4-0: 0] Index Subindex AbortCode
/// =>
impl RequestCommand {
    pub fn extract_command_from_byte0(byte0: u8) -> Result<Self, ()> {
        if let Ok(res) = RequestCommand::try_from_primitive(byte0) {
            Ok(res)
        } else {
            // Unknown Command Specifier was given
            Err(())
        }
    }
}
impl ResponseCommand {
    pub fn generate_byte0_from_command(cmd: Self) -> u8 {
        ResponseCommand::into(cmd)
    }
}
/// Should be a method on the Sdo Server
/// We should specialize the CanFrame Types for different services
/// Create a SdoFrame Type that can only be constructed via new() with
/// a DLC=8 set. Sdo Transfers use in any case DLC=8
/// impl From<CanFrame> for SdoFrame ....
pub fn handle_sdo_server_msg(msg: CanFrame) -> Option<CanFrame> {
    // INVARIANCE:
    // Higher Instance alreasy checked that 0x67F >=can_id > 0x600 and in valid SDo Range
    let place_server_req_id = 0x604; // Listens as Node Id 4
    let place_server_response_id = 0x584;
    if msg.can_id == place_server_req_id {
        // It is a request for this server
        let mut resp = CanFrame::new(place_server_response_id);
        resp.data = handle_server_request(msg.data);
        resp.dlc = 8;
        Some(msg)
    } else {
        // Msg was not for this server
        // Return no response
        None
    }
}
pub fn handle_server_request(req: [u8; 8]) -> [u8; 8] {
    // This is the function that is called if the CAN-ID matches the Server ID
    // For now this is a static function - Might make sense to make it a method on
    // a SdoServer struct

    // Reuse req as response because we need Index and Subindex to stay the same anyway
    // copy operation
    let mut response = req;
    let (_meta, resp_data) = response.split_at_mut(4); // Split in two 4 bytes slices: First Is Cmd,Index ; Second is Payload/Abort/Len
    let req_cmd = RequestCommand::try_from_primitive(req[0]);
    if let Ok(_cmd) = req_cmd {
        // Valid Command
        // Work i

        // DO STUFF

        // last step -> set Response Cmd Byte
        response[0] = ResponseCommand::generate_byte0_from_command(ResponseCommand::ExpReadRespB1);
        response
    } else {
        // Command of Sdo Request is unknown
        // TODO Reset internal Sdo Server State if necessary
        // i.e.: Terminate ongoing segmented transfer on this server
        // Unwrap()
        // This operation is unfailedable as we Split a 8 Byte array in two at Index 4 => data is always [u8;4] what the fn expects
        add_abort(
            resp_data.try_into().unwrap(),
            SdoAbortCode::UnknownAbortCode,
        );
        return response;
    }
}
