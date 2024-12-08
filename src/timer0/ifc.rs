///Register `IFC` writer
pub type W = crate::W<IFCrs>;
///Field `OF` writer - Clear OF Interrupt Flag
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UF` writer - Clear UF Interrupt Flag
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIRCHG` writer - Clear DIRCHG Interrupt Flag
pub type DirchgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC0` writer - Clear CC0 Interrupt Flag
pub type Cc0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1` writer - Clear CC1 Interrupt Flag
pub type Cc1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2` writer - Clear CC2 Interrupt Flag
pub type Cc2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3` writer - Clear CC3 Interrupt Flag
pub type Cc3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICBOF0` writer - Clear ICBOF0 Interrupt Flag
pub type Icbof0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICBOF1` writer - Clear ICBOF1 Interrupt Flag
pub type Icbof1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICBOF2` writer - Clear ICBOF2 Interrupt Flag
pub type Icbof2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICBOF3` writer - Clear ICBOF3 Interrupt Flag
pub type Icbof3W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear OF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OfW<IFCrs> {
        OfW::new(self, 0)
    }
    ///Bit 1 - Clear UF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UfW<IFCrs> {
        UfW::new(self, 1)
    }
    ///Bit 2 - Clear DIRCHG Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn dirchg(&mut self) -> DirchgW<IFCrs> {
        DirchgW::new(self, 2)
    }
    ///Bit 4 - Clear CC0 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cc0(&mut self) -> Cc0W<IFCrs> {
        Cc0W::new(self, 4)
    }
    ///Bit 5 - Clear CC1 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> Cc1W<IFCrs> {
        Cc1W::new(self, 5)
    }
    ///Bit 6 - Clear CC2 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> Cc2W<IFCrs> {
        Cc2W::new(self, 6)
    }
    ///Bit 7 - Clear CC3 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cc3(&mut self) -> Cc3W<IFCrs> {
        Cc3W::new(self, 7)
    }
    ///Bit 8 - Clear ICBOF0 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn icbof0(&mut self) -> Icbof0W<IFCrs> {
        Icbof0W::new(self, 8)
    }
    ///Bit 9 - Clear ICBOF1 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn icbof1(&mut self) -> Icbof1W<IFCrs> {
        Icbof1W::new(self, 9)
    }
    ///Bit 10 - Clear ICBOF2 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn icbof2(&mut self) -> Icbof2W<IFCrs> {
        Icbof2W::new(self, 10)
    }
    ///Bit 11 - Clear ICBOF3 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn icbof3(&mut self) -> Icbof3W<IFCrs> {
        Icbof3W::new(self, 11)
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
