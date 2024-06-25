#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<ROUTEPENrs>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<ROUTEPENrs>;
#[doc = "Field `OUTPEN` reader - ACMP Output Pin Enable"]
pub type OutpenR = crate::BitReader;
#[doc = "Field `OUTPEN` writer - ACMP Output Pin Enable"]
pub type OutpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ACMP Output Pin Enable"]
    #[inline(always)]
    pub fn outpen(&self) -> OutpenR {
        OutpenR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTEPEN")
            .field("outpen", &self.outpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ACMP Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn outpen(&mut self) -> OutpenW<ROUTEPENrs> {
        OutpenW::new(self, 0)
    }
}
#[doc = "I/O Routing Pine Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTEPENrs;
impl crate::RegisterSpec for ROUTEPENrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routepen::R`](R) reader structure"]
impl crate::Readable for ROUTEPENrs {}
#[doc = "`write(|w| ..)` method takes [`routepen::W`](W) writer structure"]
impl crate::Writable for ROUTEPENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPENrs {
    const RESET_VALUE: u32 = 0;
}
