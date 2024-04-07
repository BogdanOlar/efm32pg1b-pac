#[doc = "Register `IEN` reader"]
pub type R = crate::R<IENrs>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IENrs>;
#[doc = "Field `EDGE` reader - EDGE Interrupt Enable"]
pub type EdgeR = crate::BitReader;
#[doc = "Field `EDGE` writer - EDGE Interrupt Enable"]
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARMUP` reader - WARMUP Interrupt Enable"]
pub type WarmupR = crate::BitReader;
#[doc = "Field `WARMUP` writer - WARMUP Interrupt Enable"]
pub type WarmupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTCONFLICT` reader - APORTCONFLICT Interrupt Enable"]
pub type AportconflictR = crate::BitReader;
#[doc = "Field `APORTCONFLICT` writer - APORTCONFLICT Interrupt Enable"]
pub type AportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EDGE Interrupt Enable"]
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WARMUP Interrupt Enable"]
    #[inline(always)]
    pub fn warmup(&self) -> WarmupR {
        WarmupR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn aportconflict(&self) -> AportconflictR {
        AportconflictR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EDGE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EdgeW<IENrs> {
        EdgeW::new(self, 0)
    }
    #[doc = "Bit 1 - WARMUP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn warmup(&mut self) -> WarmupW<IENrs> {
        WarmupW::new(self, 1)
    }
    #[doc = "Bit 2 - APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aportconflict(&mut self) -> AportconflictW<IENrs> {
        AportconflictW::new(self, 2)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IENrs;
impl crate::RegisterSpec for IENrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IENrs {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IENrs {
    const RESET_VALUE: u32 = 0;
}
