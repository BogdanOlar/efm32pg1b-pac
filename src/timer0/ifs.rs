///Register `IFS` writer
pub type W = crate::W<IFSrs>;
///Field `OF` writer - Set OF Interrupt Flag
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UF` writer - Set UF Interrupt Flag
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIRCHG` writer - Set DIRCHG Interrupt Flag
pub type DirchgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC0` writer - Set CC0 Interrupt Flag
pub type Cc0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1` writer - Set CC1 Interrupt Flag
pub type Cc1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2` writer - Set CC2 Interrupt Flag
pub type Cc2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3` writer - Set CC3 Interrupt Flag
pub type Cc3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICBOF0` writer - Set ICBOF0 Interrupt Flag
pub type Icbof0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICBOF1` writer - Set ICBOF1 Interrupt Flag
pub type Icbof1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICBOF2` writer - Set ICBOF2 Interrupt Flag
pub type Icbof2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICBOF3` writer - Set ICBOF3 Interrupt Flag
pub type Icbof3W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFSrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set OF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OfW<IFSrs> {
        OfW::new(self, 0)
    }
    ///Bit 1 - Set UF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UfW<IFSrs> {
        UfW::new(self, 1)
    }
    ///Bit 2 - Set DIRCHG Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn dirchg(&mut self) -> DirchgW<IFSrs> {
        DirchgW::new(self, 2)
    }
    ///Bit 4 - Set CC0 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cc0(&mut self) -> Cc0W<IFSrs> {
        Cc0W::new(self, 4)
    }
    ///Bit 5 - Set CC1 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> Cc1W<IFSrs> {
        Cc1W::new(self, 5)
    }
    ///Bit 6 - Set CC2 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> Cc2W<IFSrs> {
        Cc2W::new(self, 6)
    }
    ///Bit 7 - Set CC3 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cc3(&mut self) -> Cc3W<IFSrs> {
        Cc3W::new(self, 7)
    }
    ///Bit 8 - Set ICBOF0 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn icbof0(&mut self) -> Icbof0W<IFSrs> {
        Icbof0W::new(self, 8)
    }
    ///Bit 9 - Set ICBOF1 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn icbof1(&mut self) -> Icbof1W<IFSrs> {
        Icbof1W::new(self, 9)
    }
    ///Bit 10 - Set ICBOF2 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn icbof2(&mut self) -> Icbof2W<IFSrs> {
        Icbof2W::new(self, 10)
    }
    ///Bit 11 - Set ICBOF3 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn icbof3(&mut self) -> Icbof3W<IFSrs> {
        Icbof3W::new(self, 11)
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
