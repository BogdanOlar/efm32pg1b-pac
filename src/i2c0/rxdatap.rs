///Register `RXDATAP` reader
pub type R = crate::R<RXDATAPrs>;
///Field `RXDATAP` reader - RX Data Peek
pub type RxdatapR = crate::FieldReader;
impl R {
    ///Bits 0:7 - RX Data Peek
    #[inline(always)]
    pub fn rxdatap(&self) -> RxdatapR {
        RxdatapR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDATAP")
            .field("rxdatap", &self.rxdatap())
            .finish()
    }
}
///Receive Buffer Data Peek Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdatap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RXDATAPrs;
impl crate::RegisterSpec for RXDATAPrs {
    type Ux = u32;
}
///`read()` method returns [`rxdatap::R`](R) reader structure
impl crate::Readable for RXDATAPrs {}
///`reset()` method sets RXDATAP to value 0
impl crate::Resettable for RXDATAPrs {
    const RESET_VALUE: u32 = 0;
}
