///Register `IEN` reader
pub type R = crate::R<IENrs>;
///Register `IEN` writer
pub type W = crate::W<IENrs>;
///Field `HFRCORDY` reader - HFRCORDY Interrupt Enable
pub type HfrcordyR = crate::BitReader;
///Field `HFRCORDY` writer - HFRCORDY Interrupt Enable
pub type HfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFXORDY` reader - HFXORDY Interrupt Enable
pub type HfxordyR = crate::BitReader;
///Field `HFXORDY` writer - HFXORDY Interrupt Enable
pub type HfxordyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFRCORDY` reader - LFRCORDY Interrupt Enable
pub type LfrcordyR = crate::BitReader;
///Field `LFRCORDY` writer - LFRCORDY Interrupt Enable
pub type LfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFXORDY` reader - LFXORDY Interrupt Enable
pub type LfxordyR = crate::BitReader;
///Field `LFXORDY` writer - LFXORDY Interrupt Enable
pub type LfxordyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUXHFRCORDY` reader - AUXHFRCORDY Interrupt Enable
pub type AuxhfrcordyR = crate::BitReader;
///Field `AUXHFRCORDY` writer - AUXHFRCORDY Interrupt Enable
pub type AuxhfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALRDY` reader - CALRDY Interrupt Enable
pub type CalrdyR = crate::BitReader;
///Field `CALRDY` writer - CALRDY Interrupt Enable
pub type CalrdyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALOF` reader - CALOF Interrupt Enable
pub type CalofR = crate::BitReader;
///Field `CALOF` writer - CALOF Interrupt Enable
pub type CalofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFXODISERR` reader - HFXODISERR Interrupt Enable
pub type HfxodiserrR = crate::BitReader;
///Field `HFXODISERR` writer - HFXODISERR Interrupt Enable
pub type HfxodiserrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFXOAUTOSW` reader - HFXOAUTOSW Interrupt Enable
pub type HfxoautoswR = crate::BitReader;
///Field `HFXOAUTOSW` writer - HFXOAUTOSW Interrupt Enable
pub type HfxoautoswW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFXOPEAKDETERR` reader - HFXOPEAKDETERR Interrupt Enable
pub type HfxopeakdeterrR = crate::BitReader;
///Field `HFXOPEAKDETERR` writer - HFXOPEAKDETERR Interrupt Enable
pub type HfxopeakdeterrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFXOPEAKDETRDY` reader - HFXOPEAKDETRDY Interrupt Enable
pub type HfxopeakdetrdyR = crate::BitReader;
///Field `HFXOPEAKDETRDY` writer - HFXOPEAKDETRDY Interrupt Enable
pub type HfxopeakdetrdyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFXOSHUNTOPTRDY` reader - HFXOSHUNTOPTRDY Interrupt Enable
pub type HfxoshuntoptrdyR = crate::BitReader;
///Field `HFXOSHUNTOPTRDY` writer - HFXOSHUNTOPTRDY Interrupt Enable
pub type HfxoshuntoptrdyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFRCODIS` reader - HFRCODIS Interrupt Enable
pub type HfrcodisR = crate::BitReader;
///Field `HFRCODIS` writer - HFRCODIS Interrupt Enable
pub type HfrcodisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFTIMEOUTERR` reader - LFTIMEOUTERR Interrupt Enable
pub type LftimeouterrR = crate::BitReader;
///Field `LFTIMEOUTERR` writer - LFTIMEOUTERR Interrupt Enable
pub type LftimeouterrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMUERR` reader - CMUERR Interrupt Enable
pub type CmuerrR = crate::BitReader;
///Field `CMUERR` writer - CMUERR Interrupt Enable
pub type CmuerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - HFRCORDY Interrupt Enable
    #[inline(always)]
    pub fn hfrcordy(&self) -> HfrcordyR {
        HfrcordyR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HFXORDY Interrupt Enable
    #[inline(always)]
    pub fn hfxordy(&self) -> HfxordyR {
        HfxordyR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LFRCORDY Interrupt Enable
    #[inline(always)]
    pub fn lfrcordy(&self) -> LfrcordyR {
        LfrcordyR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LFXORDY Interrupt Enable
    #[inline(always)]
    pub fn lfxordy(&self) -> LfxordyR {
        LfxordyR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AUXHFRCORDY Interrupt Enable
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AuxhfrcordyR {
        AuxhfrcordyR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CALRDY Interrupt Enable
    #[inline(always)]
    pub fn calrdy(&self) -> CalrdyR {
        CalrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CALOF Interrupt Enable
    #[inline(always)]
    pub fn calof(&self) -> CalofR {
        CalofR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - HFXODISERR Interrupt Enable
    #[inline(always)]
    pub fn hfxodiserr(&self) -> HfxodiserrR {
        HfxodiserrR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HFXOAUTOSW Interrupt Enable
    #[inline(always)]
    pub fn hfxoautosw(&self) -> HfxoautoswR {
        HfxoautoswR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HFXOPEAKDETERR Interrupt Enable
    #[inline(always)]
    pub fn hfxopeakdeterr(&self) -> HfxopeakdeterrR {
        HfxopeakdeterrR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HFXOPEAKDETRDY Interrupt Enable
    #[inline(always)]
    pub fn hfxopeakdetrdy(&self) -> HfxopeakdetrdyR {
        HfxopeakdetrdyR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - HFXOSHUNTOPTRDY Interrupt Enable
    #[inline(always)]
    pub fn hfxoshuntoptrdy(&self) -> HfxoshuntoptrdyR {
        HfxoshuntoptrdyR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - HFRCODIS Interrupt Enable
    #[inline(always)]
    pub fn hfrcodis(&self) -> HfrcodisR {
        HfrcodisR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - LFTIMEOUTERR Interrupt Enable
    #[inline(always)]
    pub fn lftimeouterr(&self) -> LftimeouterrR {
        LftimeouterrR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 31 - CMUERR Interrupt Enable
    #[inline(always)]
    pub fn cmuerr(&self) -> CmuerrR {
        CmuerrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("hfrcordy", &self.hfrcordy())
            .field("hfxordy", &self.hfxordy())
            .field("lfrcordy", &self.lfrcordy())
            .field("lfxordy", &self.lfxordy())
            .field("auxhfrcordy", &self.auxhfrcordy())
            .field("calrdy", &self.calrdy())
            .field("calof", &self.calof())
            .field("hfxodiserr", &self.hfxodiserr())
            .field("hfxoautosw", &self.hfxoautosw())
            .field("hfxopeakdeterr", &self.hfxopeakdeterr())
            .field("hfxopeakdetrdy", &self.hfxopeakdetrdy())
            .field("hfxoshuntoptrdy", &self.hfxoshuntoptrdy())
            .field("hfrcodis", &self.hfrcodis())
            .field("lftimeouterr", &self.lftimeouterr())
            .field("cmuerr", &self.cmuerr())
            .finish()
    }
}
impl W {
    ///Bit 0 - HFRCORDY Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn hfrcordy(&mut self) -> HfrcordyW<IENrs> {
        HfrcordyW::new(self, 0)
    }
    ///Bit 1 - HFXORDY Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn hfxordy(&mut self) -> HfxordyW<IENrs> {
        HfxordyW::new(self, 1)
    }
    ///Bit 2 - LFRCORDY Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn lfrcordy(&mut self) -> LfrcordyW<IENrs> {
        LfrcordyW::new(self, 2)
    }
    ///Bit 3 - LFXORDY Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn lfxordy(&mut self) -> LfxordyW<IENrs> {
        LfxordyW::new(self, 3)
    }
    ///Bit 4 - AUXHFRCORDY Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcordy(&mut self) -> AuxhfrcordyW<IENrs> {
        AuxhfrcordyW::new(self, 4)
    }
    ///Bit 5 - CALRDY Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn calrdy(&mut self) -> CalrdyW<IENrs> {
        CalrdyW::new(self, 5)
    }
    ///Bit 6 - CALOF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn calof(&mut self) -> CalofW<IENrs> {
        CalofW::new(self, 6)
    }
    ///Bit 8 - HFXODISERR Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn hfxodiserr(&mut self) -> HfxodiserrW<IENrs> {
        HfxodiserrW::new(self, 8)
    }
    ///Bit 9 - HFXOAUTOSW Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn hfxoautosw(&mut self) -> HfxoautoswW<IENrs> {
        HfxoautoswW::new(self, 9)
    }
    ///Bit 10 - HFXOPEAKDETERR Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdeterr(&mut self) -> HfxopeakdeterrW<IENrs> {
        HfxopeakdeterrW::new(self, 10)
    }
    ///Bit 11 - HFXOPEAKDETRDY Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdetrdy(&mut self) -> HfxopeakdetrdyW<IENrs> {
        HfxopeakdetrdyW::new(self, 11)
    }
    ///Bit 12 - HFXOSHUNTOPTRDY Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn hfxoshuntoptrdy(&mut self) -> HfxoshuntoptrdyW<IENrs> {
        HfxoshuntoptrdyW::new(self, 12)
    }
    ///Bit 13 - HFRCODIS Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn hfrcodis(&mut self) -> HfrcodisW<IENrs> {
        HfrcodisW::new(self, 13)
    }
    ///Bit 14 - LFTIMEOUTERR Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn lftimeouterr(&mut self) -> LftimeouterrW<IENrs> {
        LftimeouterrW::new(self, 14)
    }
    ///Bit 31 - CMUERR Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn cmuerr(&mut self) -> CmuerrW<IENrs> {
        CmuerrW::new(self, 31)
    }
}
///Interrupt Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IENrs;
impl crate::RegisterSpec for IENrs {
    type Ux = u32;
}
///`read()` method returns [`ien::R`](R) reader structure
impl crate::Readable for IENrs {}
///`write(|w| ..)` method takes [`ien::W`](W) writer structure
impl crate::Writable for IENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IEN to value 0
impl crate::Resettable for IENrs {
    const RESET_VALUE: u32 = 0;
}
