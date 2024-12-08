///Register `RXDATAX` reader
pub type R = crate::R<RXDATAXrs>;
///Field `RXDATA` reader - RX Data
pub type RxdataR = crate::FieldReader<u16>;
///Field `PERR` reader - Receive Data Parity Error
pub type PerrR = crate::BitReader;
///Field `FERR` reader - Receive Data Framing Error
pub type FerrR = crate::BitReader;
impl R {
    ///Bits 0:8 - RX Data
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 14 - Receive Data Parity Error
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Receive Data Framing Error
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXDATAXrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
///Receive Buffer Data Extended Register
///
///You can [`read`](crate::Reg::read) this register and get [`rxdatax::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct RXDATAXrs;
impl crate::RegisterSpec for RXDATAXrs {
    type Ux = u32;
}
///`read()` method returns [`rxdatax::R`](R) reader structure
impl crate::Readable for RXDATAXrs {}
///`reset()` method sets RXDATAX to value 0
impl crate::Resettable for RXDATAXrs {
    const RESET_VALUE: u32 = 0;
}
