///Register `IFC` writer
pub type W = crate::W<IFCrs>;
///Field `HFRCORDY` writer - Clear HFRCORDY Interrupt Flag
pub type HfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFXORDY` writer - Clear HFXORDY Interrupt Flag
pub type HfxordyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFRCORDY` writer - Clear LFRCORDY Interrupt Flag
pub type LfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFXORDY` writer - Clear LFXORDY Interrupt Flag
pub type LfxordyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUXHFRCORDY` writer - Clear AUXHFRCORDY Interrupt Flag
pub type AuxhfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALRDY` writer - Clear CALRDY Interrupt Flag
pub type CalrdyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALOF` writer - Clear CALOF Interrupt Flag
pub type CalofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFXODISERR` writer - Clear HFXODISERR Interrupt Flag
pub type HfxodiserrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFXOAUTOSW` writer - Clear HFXOAUTOSW Interrupt Flag
pub type HfxoautoswW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFXOPEAKDETERR` writer - Clear HFXOPEAKDETERR Interrupt Flag
pub type HfxopeakdeterrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFXOPEAKDETRDY` writer - Clear HFXOPEAKDETRDY Interrupt Flag
pub type HfxopeakdetrdyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFXOSHUNTOPTRDY` writer - Clear HFXOSHUNTOPTRDY Interrupt Flag
pub type HfxoshuntoptrdyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFRCODIS` writer - Clear HFRCODIS Interrupt Flag
pub type HfrcodisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFTIMEOUTERR` writer - Clear LFTIMEOUTERR Interrupt Flag
pub type LftimeouterrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMUERR` writer - Clear CMUERR Interrupt Flag
pub type CmuerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear HFRCORDY Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn hfrcordy(&mut self) -> HfrcordyW<IFCrs> {
        HfrcordyW::new(self, 0)
    }
    ///Bit 1 - Clear HFXORDY Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn hfxordy(&mut self) -> HfxordyW<IFCrs> {
        HfxordyW::new(self, 1)
    }
    ///Bit 2 - Clear LFRCORDY Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn lfrcordy(&mut self) -> LfrcordyW<IFCrs> {
        LfrcordyW::new(self, 2)
    }
    ///Bit 3 - Clear LFXORDY Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn lfxordy(&mut self) -> LfxordyW<IFCrs> {
        LfxordyW::new(self, 3)
    }
    ///Bit 4 - Clear AUXHFRCORDY Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcordy(&mut self) -> AuxhfrcordyW<IFCrs> {
        AuxhfrcordyW::new(self, 4)
    }
    ///Bit 5 - Clear CALRDY Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn calrdy(&mut self) -> CalrdyW<IFCrs> {
        CalrdyW::new(self, 5)
    }
    ///Bit 6 - Clear CALOF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn calof(&mut self) -> CalofW<IFCrs> {
        CalofW::new(self, 6)
    }
    ///Bit 8 - Clear HFXODISERR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn hfxodiserr(&mut self) -> HfxodiserrW<IFCrs> {
        HfxodiserrW::new(self, 8)
    }
    ///Bit 9 - Clear HFXOAUTOSW Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn hfxoautosw(&mut self) -> HfxoautoswW<IFCrs> {
        HfxoautoswW::new(self, 9)
    }
    ///Bit 10 - Clear HFXOPEAKDETERR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdeterr(&mut self) -> HfxopeakdeterrW<IFCrs> {
        HfxopeakdeterrW::new(self, 10)
    }
    ///Bit 11 - Clear HFXOPEAKDETRDY Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdetrdy(&mut self) -> HfxopeakdetrdyW<IFCrs> {
        HfxopeakdetrdyW::new(self, 11)
    }
    ///Bit 12 - Clear HFXOSHUNTOPTRDY Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn hfxoshuntoptrdy(&mut self) -> HfxoshuntoptrdyW<IFCrs> {
        HfxoshuntoptrdyW::new(self, 12)
    }
    ///Bit 13 - Clear HFRCODIS Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn hfrcodis(&mut self) -> HfrcodisW<IFCrs> {
        HfrcodisW::new(self, 13)
    }
    ///Bit 14 - Clear LFTIMEOUTERR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn lftimeouterr(&mut self) -> LftimeouterrW<IFCrs> {
        LftimeouterrW::new(self, 14)
    }
    ///Bit 31 - Clear CMUERR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cmuerr(&mut self) -> CmuerrW<IFCrs> {
        CmuerrW::new(self, 31)
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
