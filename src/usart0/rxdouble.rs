///Register `RXDOUBLE` reader
pub type R = crate::R<RXDOUBLErs>;
///Field `RXDATA0` reader - RX Data 0
pub type Rxdata0R = crate::FieldReader;
///Field `RXDATA1` reader - RX Data 1
pub type Rxdata1R = crate::FieldReader;
impl R {
    ///Bits 0:7 - RX Data 0
    #[inline(always)]
    pub fn rxdata0(&self) -> Rxdata0R {
        Rxdata0R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - RX Data 1
    #[inline(always)]
    pub fn rxdata1(&self) -> Rxdata1R {
        Rxdata1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXDOUBLErs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
///RX FIFO Double Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdouble::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct RXDOUBLErs;
impl crate::RegisterSpec for RXDOUBLErs {
    type Ux = u32;
}
///`read()` method returns [`rxdouble::R`](R) reader structure
impl crate::Readable for RXDOUBLErs {}
///`reset()` method sets RXDOUBLE to value 0
impl crate::Resettable for RXDOUBLErs {
    const RESET_VALUE: u32 = 0;
}
