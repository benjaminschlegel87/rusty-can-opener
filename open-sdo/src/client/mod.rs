/// Design Considerations:
/// Blocking or NB API?
///
/// "Driver Design":
/// Holds the "Client" a local handle to the driver and the client also "sends" the data?
/// - We would need a global CAN queue?
/// - Write/reader impl Object either hold by the client or provided externally in Write/read call
/// Does the client return CANopen Frames/Packages ?
pub struct SdoClient {}
