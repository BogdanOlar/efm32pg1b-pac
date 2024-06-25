#[doc = "Register `IF` reader"]
pub type R = crate::R<IFrs>;
#[doc = "Field `PERIOD` reader - Wakeup Event/Interrupt"]
pub type PeriodR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Wakeup Event/Interrupt"]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("period", &self.period())
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
