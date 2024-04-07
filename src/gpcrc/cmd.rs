#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMDrs>;
#[doc = "Field `INIT` writer - Initialization Enable"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Initialization Enable"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> InitW<CMDrs> {
        InitW::new(self, 0)
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
