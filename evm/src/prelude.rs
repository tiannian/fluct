use primitive_types::H160;

/// Recorder for address
pub trait AddressRecorder {
    fn record_address(&mut self, address: H160);
}

/* impl<'a, T: AddressRecorder> AddressRecorder for &'a mut T { */
/*     fn record_address(&mut self, address: H160) { */
/*         self.record_address(address); */
/*     } */
/* } */
/*  */
