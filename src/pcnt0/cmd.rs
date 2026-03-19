///Register `CMD` writer
pub type W = crate::W<CMDrs>;
///Field `LCNTIM` writer - Load CNT Immediately
pub type LcntimW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LTOPBIM` writer - Load TOPB Immediately
pub type LtopbimW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CMDrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Load CNT Immediately
    #[inline(always)]
    pub fn lcntim(&mut self) -> LcntimW<'_, CMDrs> {
        LcntimW::new(self, 0)
    }
    ///Bit 1 - Load TOPB Immediately
    #[inline(always)]
    pub fn ltopbim(&mut self) -> LtopbimW<'_, CMDrs> {
        LtopbimW::new(self, 1)
    }
}
///Command Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CMDrs;
impl crate::RegisterSpec for CMDrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cmd::W`](W) writer structure
impl crate::Writable for CMDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMD to value 0
impl crate::Resettable for CMDrs {}
