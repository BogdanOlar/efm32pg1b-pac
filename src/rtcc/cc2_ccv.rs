#[doc = "Register `CC2_CCV` reader"]
pub type R = crate::R<CC2_CCVrs>;
#[doc = "Register `CC2_CCV` writer"]
pub type W = crate::W<CC2_CCVrs>;
#[doc = "Field `CCV` reader - Capture/Compare Value"]
pub type CcvR = crate::FieldReader<u32>;
#[doc = "Field `CCV` writer - Capture/Compare Value"]
pub type CcvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Capture/Compare Value"]
    #[inline(always)]
    pub fn ccv(&self) -> CcvR {
        CcvR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CC2_CCV").field("ccv", &self.ccv()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Capture/Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn ccv(&mut self) -> CcvW<CC2_CCVrs> {
        CcvW::new(self, 0)
    }
}
#[doc = "Capture/Compare Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_ccv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_ccv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC2_CCVrs;
impl crate::RegisterSpec for CC2_CCVrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2_ccv::R`](R) reader structure"]
impl crate::Readable for CC2_CCVrs {}
#[doc = "`write(|w| ..)` method takes [`cc2_ccv::W`](W) writer structure"]
impl crate::Writable for CC2_CCVrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC2_CCV to value 0"]
impl crate::Resettable for CC2_CCVrs {
    const RESET_VALUE: u32 = 0;
}
