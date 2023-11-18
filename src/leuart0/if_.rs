#[doc = "Register `IF` reader"]
pub type R = crate::R<IF_SPEC>;
#[doc = "Field `TXC` reader - TX Complete Interrupt Flag"]
pub type TXC_R = crate::BitReader;
#[doc = "Field `TXBL` reader - TX Buffer Level Interrupt Flag"]
pub type TXBL_R = crate::BitReader;
#[doc = "Field `RXDATAV` reader - RX Data Valid Interrupt Flag"]
pub type RXDATAV_R = crate::BitReader;
#[doc = "Field `RXOF` reader - RX Overflow Interrupt Flag"]
pub type RXOF_R = crate::BitReader;
#[doc = "Field `RXUF` reader - RX Underflow Interrupt Flag"]
pub type RXUF_R = crate::BitReader;
#[doc = "Field `TXOF` reader - TX Overflow Interrupt Flag"]
pub type TXOF_R = crate::BitReader;
#[doc = "Field `PERR` reader - Parity Error Interrupt Flag"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `FERR` reader - Framing Error Interrupt Flag"]
pub type FERR_R = crate::BitReader;
#[doc = "Field `MPAF` reader - Multi-Processor Address Frame Interrupt Flag"]
pub type MPAF_R = crate::BitReader;
#[doc = "Field `STARTF` reader - Start Frame Interrupt Flag"]
pub type STARTF_R = crate::BitReader;
#[doc = "Field `SIGF` reader - Signal Frame Interrupt Flag"]
pub type SIGF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&self) -> RXOF_R {
        RXOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Multi-Processor Address Frame Interrupt Flag"]
    #[inline(always)]
    pub fn mpaf(&self) -> MPAF_R {
        MPAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start Frame Interrupt Flag"]
    #[inline(always)]
    pub fn startf(&self) -> STARTF_R {
        STARTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Signal Frame Interrupt Flag"]
    #[inline(always)]
    pub fn sigf(&self) -> SIGF_R {
        SIGF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("txc", &format_args!("{}", self.txc().bit()))
            .field("txbl", &format_args!("{}", self.txbl().bit()))
            .field("rxdatav", &format_args!("{}", self.rxdatav().bit()))
            .field("rxof", &format_args!("{}", self.rxof().bit()))
            .field("rxuf", &format_args!("{}", self.rxuf().bit()))
            .field("txof", &format_args!("{}", self.txof().bit()))
            .field("perr", &format_args!("{}", self.perr().bit()))
            .field("ferr", &format_args!("{}", self.ferr().bit()))
            .field("mpaf", &format_args!("{}", self.mpaf().bit()))
            .field("startf", &format_args!("{}", self.startf().bit()))
            .field("sigf", &format_args!("{}", self.sigf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<IF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IF_SPEC {}
#[doc = "`reset()` method sets IF to value 0x02"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
