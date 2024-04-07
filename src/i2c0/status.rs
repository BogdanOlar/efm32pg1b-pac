#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUSrs>;
#[doc = "Field `PSTART` reader - Pending START"]
pub type PstartR = crate::BitReader;
#[doc = "Field `PSTOP` reader - Pending STOP"]
pub type PstopR = crate::BitReader;
#[doc = "Field `PACK` reader - Pending ACK"]
pub type PackR = crate::BitReader;
#[doc = "Field `PNACK` reader - Pending NACK"]
pub type PnackR = crate::BitReader;
#[doc = "Field `PCONT` reader - Pending Continue"]
pub type PcontR = crate::BitReader;
#[doc = "Field `PABORT` reader - Pending Abort"]
pub type PabortR = crate::BitReader;
#[doc = "Field `TXC` reader - TX Complete"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXBL` reader - TX Buffer Level"]
pub type TxblR = crate::BitReader;
#[doc = "Field `RXDATAV` reader - RX Data Valid"]
pub type RxdatavR = crate::BitReader;
#[doc = "Field `RXFULL` reader - RX FIFO Full"]
pub type RxfullR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Pending START"]
    #[inline(always)]
    pub fn pstart(&self) -> PstartR {
        PstartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending STOP"]
    #[inline(always)]
    pub fn pstop(&self) -> PstopR {
        PstopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending ACK"]
    #[inline(always)]
    pub fn pack(&self) -> PackR {
        PackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pending NACK"]
    #[inline(always)]
    pub fn pnack(&self) -> PnackR {
        PnackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending Continue"]
    #[inline(always)]
    pub fn pcont(&self) -> PcontR {
        PcontR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending Abort"]
    #[inline(always)]
    pub fn pabort(&self) -> PabortR {
        PabortR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX Buffer Level"]
    #[inline(always)]
    pub fn txbl(&self) -> TxblR {
        TxblR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX Data Valid"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RxdatavR {
        RxdatavR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RX FIFO Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUSrs;
impl crate::RegisterSpec for STATUSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUSrs {}
#[doc = "`reset()` method sets STATUS to value 0x80"]
impl crate::Resettable for STATUSrs {
    const RESET_VALUE: u32 = 0x80;
}
