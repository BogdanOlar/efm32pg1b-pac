///Register `IFC` writer
pub type W = crate::W<IFCrs>;
///Field `EDGE` writer - Clear EDGE Interrupt Flag
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WARMUP` writer - Clear WARMUP Interrupt Flag
pub type WarmupW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APORTCONFLICT` writer - Clear APORTCONFLICT Interrupt Flag
pub type AportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear EDGE Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EdgeW<IFCrs> {
        EdgeW::new(self, 0)
    }
    ///Bit 1 - Clear WARMUP Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn warmup(&mut self) -> WarmupW<IFCrs> {
        WarmupW::new(self, 1)
    }
    ///Bit 2 - Clear APORTCONFLICT Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn aportconflict(&mut self) -> AportconflictW<IFCrs> {
        AportconflictW::new(self, 2)
    }
}
///Interrupt Flag Clear Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFCrs;
impl crate::RegisterSpec for IFCrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ifc::W`](W) writer structure
impl crate::Writable for IFCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IFC to value 0
impl crate::Resettable for IFCrs {
    const RESET_VALUE: u32 = 0;
}
