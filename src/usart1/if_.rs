#[doc = "Register `IF` reader"]
pub type R = crate::R<IFrs>;
#[doc = "Field `TXC` reader - TX Complete Interrupt Flag"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXBL` reader - TX Buffer Level Interrupt Flag"]
pub type TxblR = crate::BitReader;
#[doc = "Field `RXDATAV` reader - RX Data Valid Interrupt Flag"]
pub type RxdatavR = crate::BitReader;
#[doc = "Field `RXFULL` reader - RX Buffer Full Interrupt Flag"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `RXOF` reader - RX Overflow Interrupt Flag"]
pub type RxofR = crate::BitReader;
#[doc = "Field `RXUF` reader - RX Underflow Interrupt Flag"]
pub type RxufR = crate::BitReader;
#[doc = "Field `TXOF` reader - TX Overflow Interrupt Flag"]
pub type TxofR = crate::BitReader;
#[doc = "Field `TXUF` reader - TX Underflow Interrupt Flag"]
pub type TxufR = crate::BitReader;
#[doc = "Field `PERR` reader - Parity Error Interrupt Flag"]
pub type PerrR = crate::BitReader;
#[doc = "Field `FERR` reader - Framing Error Interrupt Flag"]
pub type FerrR = crate::BitReader;
#[doc = "Field `MPAF` reader - Multi-Processor Address Frame Interrupt Flag"]
pub type MpafR = crate::BitReader;
#[doc = "Field `SSM` reader - Slave-Select in Master Mode Interrupt Flag"]
pub type SsmR = crate::BitReader;
#[doc = "Field `CCF` reader - Collision Check Fail Interrupt Flag"]
pub type CcfR = crate::BitReader;
#[doc = "Field `TXIDLE` reader - TX Idle Interrupt Flag"]
pub type TxidleR = crate::BitReader;
#[doc = "Field `TCMP0` reader - Timer Comparator 0 Interrupt Flag"]
pub type Tcmp0R = crate::BitReader;
#[doc = "Field `TCMP1` reader - Timer Comparator 1 Interrupt Flag"]
pub type Tcmp1R = crate::BitReader;
#[doc = "Field `TCMP2` reader - Timer Comparator 2 Interrupt Flag"]
pub type Tcmp2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn txbl(&self) -> TxblR {
        TxblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RxdatavR {
        RxdatavR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX Buffer Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&self) -> RxofR {
        RxofR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn txuf(&self) -> TxufR {
        TxufR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multi-Processor Address Frame Interrupt Flag"]
    #[inline(always)]
    pub fn mpaf(&self) -> MpafR {
        MpafR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Slave-Select in Master Mode Interrupt Flag"]
    #[inline(always)]
    pub fn ssm(&self) -> SsmR {
        SsmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Collision Check Fail Interrupt Flag"]
    #[inline(always)]
    pub fn ccf(&self) -> CcfR {
        CcfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX Idle Interrupt Flag"]
    #[inline(always)]
    pub fn txidle(&self) -> TxidleR {
        TxidleR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Timer Comparator 0 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp0(&self) -> Tcmp0R {
        Tcmp0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer Comparator 1 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp1(&self) -> Tcmp1R {
        Tcmp1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer Comparator 2 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp2(&self) -> Tcmp2R {
        Tcmp2R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IFrs {}
#[doc = "`reset()` method sets IF to value 0x02"]
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0x02;
}
