///Register `RXDATAXP` reader
pub type R = crate::R<RXDATAXPrs>;
///Field `RXDATAP` reader - RX Data Peek
pub type RxdatapR = crate::FieldReader<u16>;
///Field `PERRP` reader - Receive Data Parity Error Peek
pub type PerrpR = crate::BitReader;
///Field `FERRP` reader - Receive Data Framing Error Peek
pub type FerrpR = crate::BitReader;
impl R {
    ///Bits 0:8 - RX Data Peek
    #[inline(always)]
    pub fn rxdatap(&self) -> RxdatapR {
        RxdatapR::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 14 - Receive Data Parity Error Peek
    #[inline(always)]
    pub fn perrp(&self) -> PerrpR {
        PerrpR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Receive Data Framing Error Peek
    #[inline(always)]
    pub fn ferrp(&self) -> FerrpR {
        FerrpR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDATAXP")
            .field("rxdatap", &self.rxdatap())
            .field("perrp", &self.perrp())
            .field("ferrp", &self.ferrp())
            .finish()
    }
}
///Receive Buffer Data Extended Peek Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdataxp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RXDATAXPrs;
impl crate::RegisterSpec for RXDATAXPrs {
    type Ux = u32;
}
///`read()` method returns [`rxdataxp::R`](R) reader structure
impl crate::Readable for RXDATAXPrs {}
///`reset()` method sets RXDATAXP to value 0
impl crate::Resettable for RXDATAXPrs {
    const RESET_VALUE: u32 = 0;
}
