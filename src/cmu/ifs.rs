#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFSrs>;
#[doc = "Field `HFRCORDY` writer - Set HFRCORDY Interrupt Flag"]
pub type HfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXORDY` writer - Set HFXORDY Interrupt Flag"]
pub type HfxordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCORDY` writer - Set LFRCORDY Interrupt Flag"]
pub type LfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXORDY` writer - Set LFXORDY Interrupt Flag"]
pub type LfxordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCORDY` writer - Set AUXHFRCORDY Interrupt Flag"]
pub type AuxhfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRDY` writer - Set CALRDY Interrupt Flag"]
pub type CalrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOF` writer - Set CALOF Interrupt Flag"]
pub type CalofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXODISERR` writer - Set HFXODISERR Interrupt Flag"]
pub type HfxodiserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOAUTOSW` writer - Set HFXOAUTOSW Interrupt Flag"]
pub type HfxoautoswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOPEAKDETERR` writer - Set HFXOPEAKDETERR Interrupt Flag"]
pub type HfxopeakdeterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOPEAKDETRDY` writer - Set HFXOPEAKDETRDY Interrupt Flag"]
pub type HfxopeakdetrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOSHUNTOPTRDY` writer - Set HFXOSHUNTOPTRDY Interrupt Flag"]
pub type HfxoshuntoptrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRCODIS` writer - Set HFRCODIS Interrupt Flag"]
pub type HfrcodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFTIMEOUTERR` writer - Set LFTIMEOUTERR Interrupt Flag"]
pub type LftimeouterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMUERR` writer - Set CMUERR Interrupt Flag"]
pub type CmuerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set HFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcordy(&mut self) -> HfrcordyW<IFSrs> {
        HfrcordyW::new(self, 0)
    }
    #[doc = "Bit 1 - Set HFXORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxordy(&mut self) -> HfxordyW<IFSrs> {
        HfxordyW::new(self, 1)
    }
    #[doc = "Bit 2 - Set LFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcordy(&mut self) -> LfrcordyW<IFSrs> {
        LfrcordyW::new(self, 2)
    }
    #[doc = "Bit 3 - Set LFXORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfxordy(&mut self) -> LfxordyW<IFSrs> {
        LfxordyW::new(self, 3)
    }
    #[doc = "Bit 4 - Set AUXHFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcordy(&mut self) -> AuxhfrcordyW<IFSrs> {
        AuxhfrcordyW::new(self, 4)
    }
    #[doc = "Bit 5 - Set CALRDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn calrdy(&mut self) -> CalrdyW<IFSrs> {
        CalrdyW::new(self, 5)
    }
    #[doc = "Bit 6 - Set CALOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn calof(&mut self) -> CalofW<IFSrs> {
        CalofW::new(self, 6)
    }
    #[doc = "Bit 8 - Set HFXODISERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxodiserr(&mut self) -> HfxodiserrW<IFSrs> {
        HfxodiserrW::new(self, 8)
    }
    #[doc = "Bit 9 - Set HFXOAUTOSW Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoautosw(&mut self) -> HfxoautoswW<IFSrs> {
        HfxoautoswW::new(self, 9)
    }
    #[doc = "Bit 10 - Set HFXOPEAKDETERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdeterr(&mut self) -> HfxopeakdeterrW<IFSrs> {
        HfxopeakdeterrW::new(self, 10)
    }
    #[doc = "Bit 11 - Set HFXOPEAKDETRDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdetrdy(&mut self) -> HfxopeakdetrdyW<IFSrs> {
        HfxopeakdetrdyW::new(self, 11)
    }
    #[doc = "Bit 12 - Set HFXOSHUNTOPTRDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoshuntoptrdy(&mut self) -> HfxoshuntoptrdyW<IFSrs> {
        HfxoshuntoptrdyW::new(self, 12)
    }
    #[doc = "Bit 13 - Set HFRCODIS Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcodis(&mut self) -> HfrcodisW<IFSrs> {
        HfrcodisW::new(self, 13)
    }
    #[doc = "Bit 14 - Set LFTIMEOUTERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lftimeouterr(&mut self) -> LftimeouterrW<IFSrs> {
        LftimeouterrW::new(self, 14)
    }
    #[doc = "Bit 31 - Set CMUERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmuerr(&mut self) -> CmuerrW<IFSrs> {
        CmuerrW::new(self, 31)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFSrs;
impl crate::RegisterSpec for IFSrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFSrs {
    const RESET_VALUE: u32 = 0;
}
