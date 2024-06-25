#[doc = "Register `IF` reader"]
pub type R = crate::R<IFrs>;
#[doc = "Field `SINGLE` reader - Single Conversion Complete Interrupt Flag"]
pub type SingleR = crate::BitReader;
#[doc = "Field `SCAN` reader - Scan Conversion Complete Interrupt Flag"]
pub type ScanR = crate::BitReader;
#[doc = "Field `SINGLEOF` reader - Single FIFO Overflow Interrupt Flag"]
pub type SingleofR = crate::BitReader;
#[doc = "Field `SCANOF` reader - Scan FIFO Overflow Interrupt Flag"]
pub type ScanofR = crate::BitReader;
#[doc = "Field `SINGLEUF` reader - Single FIFO Underflow Interrupt Flag"]
pub type SingleufR = crate::BitReader;
#[doc = "Field `SCANUF` reader - Scan FIFO Underflow Interrupt Flag"]
pub type ScanufR = crate::BitReader;
#[doc = "Field `SINGLECMP` reader - Single Result Compare Match Interrupt Flag"]
pub type SinglecmpR = crate::BitReader;
#[doc = "Field `SCANCMP` reader - Scan Result Compare Match Interrupt Flag"]
pub type ScancmpR = crate::BitReader;
#[doc = "Field `VREFOV` reader - VREF Over Voltage Interrupt Flag"]
pub type VrefovR = crate::BitReader;
#[doc = "Field `PROGERR` reader - Programming Error Interrupt Flag"]
pub type ProgerrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn single(&self) -> SingleR {
        SingleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn scan(&self) -> ScanR {
        ScanR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Single FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn singleof(&self) -> SingleofR {
        SingleofR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Scan FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn scanof(&self) -> ScanofR {
        ScanofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Single FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn singleuf(&self) -> SingleufR {
        SingleufR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Scan FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn scanuf(&self) -> ScanufR {
        ScanufR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Single Result Compare Match Interrupt Flag"]
    #[inline(always)]
    pub fn singlecmp(&self) -> SinglecmpR {
        SinglecmpR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Scan Result Compare Match Interrupt Flag"]
    #[inline(always)]
    pub fn scancmp(&self) -> ScancmpR {
        ScancmpR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - VREF Over Voltage Interrupt Flag"]
    #[inline(always)]
    pub fn vrefov(&self) -> VrefovR {
        VrefovR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Programming Error Interrupt Flag"]
    #[inline(always)]
    pub fn progerr(&self) -> ProgerrR {
        ProgerrR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("single", &self.single())
            .field("scan", &self.scan())
            .field("singleof", &self.singleof())
            .field("scanof", &self.scanof())
            .field("singleuf", &self.singleuf())
            .field("scanuf", &self.scanuf())
            .field("singlecmp", &self.singlecmp())
            .field("scancmp", &self.scancmp())
            .field("vrefov", &self.vrefov())
            .field("progerr", &self.progerr())
            .finish()
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IFrs {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0;
}
