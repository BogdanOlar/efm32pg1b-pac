///Register `IFS` writer
pub type W = crate::W<IFSrs>;
///Field `UF` writer - Set UF Interrupt Flag
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OF` writer - Set OF Interrupt Flag
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIRCNG` writer - Set DIRCNG Interrupt Flag
pub type DircngW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUXOF` writer - Set AUXOF Interrupt Flag
pub type AuxofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCC` writer - Set TCC Interrupt Flag
pub type TccW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OQSTERR` writer - Set OQSTERR Interrupt Flag
pub type OqsterrW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFSrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set UF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UfW<IFSrs> {
        UfW::new(self, 0)
    }
    ///Bit 1 - Set OF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OfW<IFSrs> {
        OfW::new(self, 1)
    }
    ///Bit 2 - Set DIRCNG Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn dircng(&mut self) -> DircngW<IFSrs> {
        DircngW::new(self, 2)
    }
    ///Bit 3 - Set AUXOF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn auxof(&mut self) -> AuxofW<IFSrs> {
        AuxofW::new(self, 3)
    }
    ///Bit 4 - Set TCC Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TccW<IFSrs> {
        TccW::new(self, 4)
    }
    ///Bit 5 - Set OQSTERR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn oqsterr(&mut self) -> OqsterrW<IFSrs> {
        OqsterrW::new(self, 5)
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
