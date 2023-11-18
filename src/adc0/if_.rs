#[doc = "Register `IF` reader"]
pub type R = crate::R<IF_SPEC>;
#[doc = "Field `SINGLE` reader - Single Conversion Complete Interrupt Flag"]
pub type SINGLE_R = crate::BitReader;
#[doc = "Field `SCAN` reader - Scan Conversion Complete Interrupt Flag"]
pub type SCAN_R = crate::BitReader;
#[doc = "Field `SINGLEOF` reader - Single FIFO Overflow Interrupt Flag"]
pub type SINGLEOF_R = crate::BitReader;
#[doc = "Field `SCANOF` reader - Scan FIFO Overflow Interrupt Flag"]
pub type SCANOF_R = crate::BitReader;
#[doc = "Field `SINGLEUF` reader - Single FIFO Underflow Interrupt Flag"]
pub type SINGLEUF_R = crate::BitReader;
#[doc = "Field `SCANUF` reader - Scan FIFO Underflow Interrupt Flag"]
pub type SCANUF_R = crate::BitReader;
#[doc = "Field `SINGLECMP` reader - Single Result Compare Match Interrupt Flag"]
pub type SINGLECMP_R = crate::BitReader;
#[doc = "Field `SCANCMP` reader - Scan Result Compare Match Interrupt Flag"]
pub type SCANCMP_R = crate::BitReader;
#[doc = "Field `VREFOV` reader - VREF Over Voltage Interrupt Flag"]
pub type VREFOV_R = crate::BitReader;
#[doc = "Field `PROGERR` reader - Programming Error Interrupt Flag"]
pub type PROGERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Single FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn singleof(&self) -> SINGLEOF_R {
        SINGLEOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Scan FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn scanof(&self) -> SCANOF_R {
        SCANOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Single FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn singleuf(&self) -> SINGLEUF_R {
        SINGLEUF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Scan FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn scanuf(&self) -> SCANUF_R {
        SCANUF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Single Result Compare Match Interrupt Flag"]
    #[inline(always)]
    pub fn singlecmp(&self) -> SINGLECMP_R {
        SINGLECMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Scan Result Compare Match Interrupt Flag"]
    #[inline(always)]
    pub fn scancmp(&self) -> SCANCMP_R {
        SCANCMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - VREF Over Voltage Interrupt Flag"]
    #[inline(always)]
    pub fn vrefov(&self) -> VREFOV_R {
        VREFOV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Programming Error Interrupt Flag"]
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("single", &format_args!("{}", self.single().bit()))
            .field("scan", &format_args!("{}", self.scan().bit()))
            .field("singleof", &format_args!("{}", self.singleof().bit()))
            .field("scanof", &format_args!("{}", self.scanof().bit()))
            .field("singleuf", &format_args!("{}", self.singleuf().bit()))
            .field("scanuf", &format_args!("{}", self.scanuf().bit()))
            .field("singlecmp", &format_args!("{}", self.singlecmp().bit()))
            .field("scancmp", &format_args!("{}", self.scancmp().bit()))
            .field("vrefov", &format_args!("{}", self.vrefov().bit()))
            .field("progerr", &format_args!("{}", self.progerr().bit()))
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
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
