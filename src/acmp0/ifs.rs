///Register `IFS` writer
pub type W = crate::W<IFSrs>;
///Field `EDGE` writer - Set EDGE Interrupt Flag
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WARMUP` writer - Set WARMUP Interrupt Flag
pub type WarmupW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APORTCONFLICT` writer - Set APORTCONFLICT Interrupt Flag
pub type AportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFSrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set EDGE Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EdgeW<IFSrs> {
        EdgeW::new(self, 0)
    }
    ///Bit 1 - Set WARMUP Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn warmup(&mut self) -> WarmupW<IFSrs> {
        WarmupW::new(self, 1)
    }
    ///Bit 2 - Set APORTCONFLICT Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn aportconflict(&mut self) -> AportconflictW<IFSrs> {
        AportconflictW::new(self, 2)
    }
}
///Interrupt Flag Set Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFSrs;
impl crate::RegisterSpec for IFSrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ifs::W`](W) writer structure
impl crate::Writable for IFSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IFS to value 0
impl crate::Resettable for IFSrs {
    const RESET_VALUE: u32 = 0;
}
