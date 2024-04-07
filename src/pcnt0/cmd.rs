#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMDrs>;
#[doc = "Field `LCNTIM` writer - Load CNT Immediately"]
pub type LcntimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTOPBIM` writer - Load TOPB Immediately"]
pub type LtopbimW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Load CNT Immediately"]
    #[inline(always)]
    #[must_use]
    pub fn lcntim(&mut self) -> LcntimW<CMDrs> {
        LcntimW::new(self, 0)
    }
    #[doc = "Bit 1 - Load TOPB Immediately"]
    #[inline(always)]
    #[must_use]
    pub fn ltopbim(&mut self) -> LtopbimW<CMDrs> {
        LtopbimW::new(self, 1)
    }
}
#[doc = "Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDrs;
impl crate::RegisterSpec for CMDrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMDrs {
    const RESET_VALUE: u32 = 0;
}
