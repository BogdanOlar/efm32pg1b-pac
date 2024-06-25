#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUSrs>;
#[doc = "Field `ACMPACT` reader - Analog Comparator Active"]
pub type AcmpactR = crate::BitReader;
#[doc = "Field `ACMPOUT` reader - Analog Comparator Output"]
pub type AcmpoutR = crate::BitReader;
#[doc = "Field `APORTCONFLICT` reader - APORT Conflict Output"]
pub type AportconflictR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Analog Comparator Active"]
    #[inline(always)]
    pub fn acmpact(&self) -> AcmpactR {
        AcmpactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator Output"]
    #[inline(always)]
    pub fn acmpout(&self) -> AcmpoutR {
        AcmpoutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - APORT Conflict Output"]
    #[inline(always)]
    pub fn aportconflict(&self) -> AportconflictR {
        AportconflictR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("acmpact", &self.acmpact())
            .field("acmpout", &self.acmpout())
            .field("aportconflict", &self.aportconflict())
            .finish()
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUSrs;
impl crate::RegisterSpec for STATUSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUSrs {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUSrs {
    const RESET_VALUE: u32 = 0;
}
