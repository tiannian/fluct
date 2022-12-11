use primitive_types::H160;

/// Recorder for address
pub trait AddressRecorder {
    fn record_address(&mut self, address: H160);
}
