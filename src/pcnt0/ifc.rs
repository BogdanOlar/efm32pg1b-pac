///Register `IFC` writer
pub type W = crate::W<IFCrs>;
///Field `UF` writer - Clear UF Interrupt Flag
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OF` writer - Clear OF Interrupt Flag
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIRCNG` writer - Clear DIRCNG Interrupt Flag
pub type DircngW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUXOF` writer - Clear AUXOF Interrupt Flag
pub type AuxofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCC` writer - Clear TCC Interrupt Flag
pub type TccW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OQSTERR` writer - Clear OQSTERR Interrupt Flag
pub type OqsterrW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear UF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UfW<IFCrs> {
        UfW::new(self, 0)
    }
    ///Bit 1 - Clear OF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OfW<IFCrs> {
        OfW::new(self, 1)
    }
    ///Bit 2 - Clear DIRCNG Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn dircng(&mut self) -> DircngW<IFCrs> {
        DircngW::new(self, 2)
    }
    ///Bit 3 - Clear AUXOF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn auxof(&mut self) -> AuxofW<IFCrs> {
        AuxofW::new(self, 3)
    }
    ///Bit 4 - Clear TCC Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TccW<IFCrs> {
        TccW::new(self, 4)
    }
    ///Bit 5 - Clear OQSTERR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn oqsterr(&mut self) -> OqsterrW<IFCrs> {
        OqsterrW::new(self, 5)
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
