//! From CiA:
//! Service data object (SDO)
//! Service data objects (SDOs) enable access to all entries of a CANopen object dictionary.
//! One SDO consists of two CAN data frames with different CAN-Identifiers.
//! This is a confirmed communication service. With an SDO, a peer-to-peer client-server communication
//!  between two CANopen devices can be established on the broadcast medium CAN. The owner of the accessed
//!  object dictionary acts as a server of the SDO. The device that accesses the object dictionary
//!  of the other device is the SDO client.
//!
//!

/// SDO Client implementation
pub mod client;

/// SDO Server implementation
pub mod server;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
