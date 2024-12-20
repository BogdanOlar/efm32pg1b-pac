///Register `IFC` writer
pub type W = crate::W<IFCrs>;
///Field `OF` writer - Clear OF Interrupt Flag
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC0` writer - Clear CC0 Interrupt Flag
pub type Cc0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1` writer - Clear CC1 Interrupt Flag
pub type Cc1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2` writer - Clear CC2 Interrupt Flag
pub type Cc2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSCFAIL` writer - Clear OSCFAIL Interrupt Flag
pub type OscfailW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTTICK` writer - Clear CNTTICK Interrupt Flag
pub type CnttickW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MINTICK` writer - Clear MINTICK Interrupt Flag
pub type MintickW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOURTICK` writer - Clear HOURTICK Interrupt Flag
pub type HourtickW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAYTICK` writer - Clear DAYTICK Interrupt Flag
pub type DaytickW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAYOWOF` writer - Clear DAYOWOF Interrupt Flag
pub type DayowofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MONTHTICK` writer - Clear MONTHTICK Interrupt Flag
pub type MonthtickW<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 1 - Clear CC0 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cc0(&mut self) -> Cc0W<IFCrs> {
        Cc0W::new(self, 1)
    }
    ///Bit 2 - Clear CC1 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> Cc1W<IFCrs> {
        Cc1W::new(self, 2)
    }
    ///Bit 3 - Clear CC2 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> Cc2W<IFCrs> {
        Cc2W::new(self, 3)
    }
    ///Bit 4 - Clear OSCFAIL Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn oscfail(&mut self) -> OscfailW<IFCrs> {
        OscfailW::new(self, 4)
    }
    ///Bit 5 - Clear CNTTICK Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cnttick(&mut self) -> CnttickW<IFCrs> {
        CnttickW::new(self, 5)
    }
    ///Bit 6 - Clear MINTICK Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn mintick(&mut self) -> MintickW<IFCrs> {
        MintickW::new(self, 6)
    }
    ///Bit 7 - Clear HOURTICK Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn hourtick(&mut self) -> HourtickW<IFCrs> {
        HourtickW::new(self, 7)
    }
    ///Bit 8 - Clear DAYTICK Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn daytick(&mut self) -> DaytickW<IFCrs> {
        DaytickW::new(self, 8)
    }
    ///Bit 9 - Clear DAYOWOF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn dayowof(&mut self) -> DayowofW<IFCrs> {
        DayowofW::new(self, 9)
    }
    ///Bit 10 - Clear MONTHTICK Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn monthtick(&mut self) -> MonthtickW<IFCrs> {
        MonthtickW::new(self, 10)
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
