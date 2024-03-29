#[doc = "Register `RXDOUBLE` reader"]
pub type R = crate::R<RXDOUBLErs>;
#[doc = "Field `RXDATA0` reader - RX Data 0"]
pub type RXDATA0_R = crate::FieldReader;
#[doc = "Field `RXDATA1` reader - RX Data 1"]
pub type RXDATA1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RX Data 0"]
    #[inline(always)]
    pub fn rxdata0(&self) -> RXDATA0_R {
        RXDATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX Data 1"]
    #[inline(always)]
    pub fn rxdata1(&self) -> RXDATA1_R {
        RXDATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "RX FIFO Double Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdouble::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDOUBLErs;
impl crate::RegisterSpec for RXDOUBLErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdouble::R`](R) reader structure"]
impl crate::Readable for RXDOUBLErs {}
#[doc = "`reset()` method sets RXDOUBLE to value 0"]
impl crate::Resettable for RXDOUBLErs {
    const RESET_VALUE: u32 = 0;
}
