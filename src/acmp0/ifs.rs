#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFSrs>;
#[doc = "Field `EDGE` writer - Set EDGE Interrupt Flag"]
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARMUP` writer - Set WARMUP Interrupt Flag"]
pub type WarmupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTCONFLICT` writer - Set APORTCONFLICT Interrupt Flag"]
pub type AportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set EDGE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EdgeW<IFSrs> {
        EdgeW::new(self, 0)
    }
    #[doc = "Bit 1 - Set WARMUP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn warmup(&mut self) -> WarmupW<IFSrs> {
        WarmupW::new(self, 1)
    }
    #[doc = "Bit 2 - Set APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aportconflict(&mut self) -> AportconflictW<IFSrs> {
        AportconflictW::new(self, 2)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFSrs;
impl crate::RegisterSpec for IFSrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFSrs {
    const RESET_VALUE: u32 = 0;
}
