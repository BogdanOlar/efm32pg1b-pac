///Register `RXDOUBLEP` reader
pub type R = crate::R<RXDOUBLEPrs>;
///Field `RXDATAP0` reader - RX Data 0 Peek
pub type Rxdatap0R = crate::FieldReader;
///Field `RXDATAP1` reader - RX Data 1 Peek
pub type Rxdatap1R = crate::FieldReader;
impl R {
    ///Bits 0:7 - RX Data 0 Peek
    #[inline(always)]
    pub fn rxdatap0(&self) -> Rxdatap0R {
        Rxdatap0R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - RX Data 1 Peek
    #[inline(always)]
    pub fn rxdatap1(&self) -> Rxdatap1R {
        Rxdatap1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDOUBLEP")
            .field("rxdatap0", &self.rxdatap0())
            .field("rxdatap1", &self.rxdatap1())
            .finish()
    }
}
///Receive Buffer Double Data Peek Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdoublep::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RXDOUBLEPrs;
impl crate::RegisterSpec for RXDOUBLEPrs {
    type Ux = u32;
}
///`read()` method returns [`rxdoublep::R`](R) reader structure
impl crate::Readable for RXDOUBLEPrs {}
///`reset()` method sets RXDOUBLEP to value 0
impl crate::Resettable for RXDOUBLEPrs {
    const RESET_VALUE: u32 = 0;
}
