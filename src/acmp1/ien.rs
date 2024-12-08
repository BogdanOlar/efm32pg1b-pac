///Register `IEN` reader
pub type R = crate::R<IENrs>;
///Register `IEN` writer
pub type W = crate::W<IENrs>;
///Field `EDGE` reader - EDGE Interrupt Enable
pub type EdgeR = crate::BitReader;
///Field `EDGE` writer - EDGE Interrupt Enable
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WARMUP` reader - WARMUP Interrupt Enable
pub type WarmupR = crate::BitReader;
///Field `WARMUP` writer - WARMUP Interrupt Enable
pub type WarmupW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APORTCONFLICT` reader - APORTCONFLICT Interrupt Enable
pub type AportconflictR = crate::BitReader;
///Field `APORTCONFLICT` writer - APORTCONFLICT Interrupt Enable
pub type AportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - EDGE Interrupt Enable
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WARMUP Interrupt Enable
    #[inline(always)]
    pub fn warmup(&self) -> WarmupR {
        WarmupR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - APORTCONFLICT Interrupt Enable
    #[inline(always)]
    pub fn aportconflict(&self) -> AportconflictR {
        AportconflictR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("edge", &self.edge())
            .field("warmup", &self.warmup())
            .field("aportconflict", &self.aportconflict())
            .finish()
    }
}
impl W {
    ///Bit 0 - EDGE Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EdgeW<IENrs> {
        EdgeW::new(self, 0)
    }
    ///Bit 1 - WARMUP Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn warmup(&mut self) -> WarmupW<IENrs> {
        WarmupW::new(self, 1)
    }
    ///Bit 2 - APORTCONFLICT Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn aportconflict(&mut self) -> AportconflictW<IENrs> {
        AportconflictW::new(self, 2)
    }
}
///Interrupt Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IENrs;
impl crate::RegisterSpec for IENrs {
    type Ux = u32;
}
///`read()` method returns [`ien::R`](R) reader structure
impl crate::Readable for IENrs {}
///`write(|w| ..)` method takes [`ien::W`](W) writer structure
impl crate::Writable for IENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IEN to value 0
impl crate::Resettable for IENrs {
    const RESET_VALUE: u32 = 0;
}
