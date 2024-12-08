///Register `IFS` writer
pub type W = crate::W<IFSrs>;
///Field `OF` writer - Set OF Interrupt Flag
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC0` writer - Set CC0 Interrupt Flag
pub type Cc0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1` writer - Set CC1 Interrupt Flag
pub type Cc1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2` writer - Set CC2 Interrupt Flag
pub type Cc2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSCFAIL` writer - Set OSCFAIL Interrupt Flag
pub type OscfailW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTTICK` writer - Set CNTTICK Interrupt Flag
pub type CnttickW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MINTICK` writer - Set MINTICK Interrupt Flag
pub type MintickW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOURTICK` writer - Set HOURTICK Interrupt Flag
pub type HourtickW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAYTICK` writer - Set DAYTICK Interrupt Flag
pub type DaytickW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAYOWOF` writer - Set DAYOWOF Interrupt Flag
pub type DayowofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MONTHTICK` writer - Set MONTHTICK Interrupt Flag
pub type MonthtickW<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 1 - Set CC0 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cc0(&mut self) -> Cc0W<IFSrs> {
        Cc0W::new(self, 1)
    }
    ///Bit 2 - Set CC1 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> Cc1W<IFSrs> {
        Cc1W::new(self, 2)
    }
    ///Bit 3 - Set CC2 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> Cc2W<IFSrs> {
        Cc2W::new(self, 3)
    }
    ///Bit 4 - Set OSCFAIL Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn oscfail(&mut self) -> OscfailW<IFSrs> {
        OscfailW::new(self, 4)
    }
    ///Bit 5 - Set CNTTICK Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cnttick(&mut self) -> CnttickW<IFSrs> {
        CnttickW::new(self, 5)
    }
    ///Bit 6 - Set MINTICK Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn mintick(&mut self) -> MintickW<IFSrs> {
        MintickW::new(self, 6)
    }
    ///Bit 7 - Set HOURTICK Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn hourtick(&mut self) -> HourtickW<IFSrs> {
        HourtickW::new(self, 7)
    }
    ///Bit 8 - Set DAYTICK Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn daytick(&mut self) -> DaytickW<IFSrs> {
        DaytickW::new(self, 8)
    }
    ///Bit 9 - Set DAYOWOF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn dayowof(&mut self) -> DayowofW<IFSrs> {
        DayowofW::new(self, 9)
    }
    ///Bit 10 - Set MONTHTICK Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn monthtick(&mut self) -> MonthtickW<IFSrs> {
        MonthtickW::new(self, 10)
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
