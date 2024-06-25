#[doc = "Register `IEN` reader"]
pub type R = crate::R<IENrs>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IENrs>;
#[doc = "Field `OF` reader - OF Interrupt Enable"]
pub type OfR = crate::BitReader;
#[doc = "Field `OF` writer - OF Interrupt Enable"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0` reader - CC0 Interrupt Enable"]
pub type Cc0R = crate::BitReader;
#[doc = "Field `CC0` writer - CC0 Interrupt Enable"]
pub type Cc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1` reader - CC1 Interrupt Enable"]
pub type Cc1R = crate::BitReader;
#[doc = "Field `CC1` writer - CC1 Interrupt Enable"]
pub type Cc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2` reader - CC2 Interrupt Enable"]
pub type Cc2R = crate::BitReader;
#[doc = "Field `CC2` writer - CC2 Interrupt Enable"]
pub type Cc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCFAIL` reader - OSCFAIL Interrupt Enable"]
pub type OscfailR = crate::BitReader;
#[doc = "Field `OSCFAIL` writer - OSCFAIL Interrupt Enable"]
pub type OscfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTTICK` reader - CNTTICK Interrupt Enable"]
pub type CnttickR = crate::BitReader;
#[doc = "Field `CNTTICK` writer - CNTTICK Interrupt Enable"]
pub type CnttickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINTICK` reader - MINTICK Interrupt Enable"]
pub type MintickR = crate::BitReader;
#[doc = "Field `MINTICK` writer - MINTICK Interrupt Enable"]
pub type MintickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOURTICK` reader - HOURTICK Interrupt Enable"]
pub type HourtickR = crate::BitReader;
#[doc = "Field `HOURTICK` writer - HOURTICK Interrupt Enable"]
pub type HourtickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAYTICK` reader - DAYTICK Interrupt Enable"]
pub type DaytickR = crate::BitReader;
#[doc = "Field `DAYTICK` writer - DAYTICK Interrupt Enable"]
pub type DaytickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAYOWOF` reader - DAYOWOF Interrupt Enable"]
pub type DayowofR = crate::BitReader;
#[doc = "Field `DAYOWOF` writer - DAYOWOF Interrupt Enable"]
pub type DayowofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONTHTICK` reader - MONTHTICK Interrupt Enable"]
pub type MonthtickR = crate::BitReader;
#[doc = "Field `MONTHTICK` writer - MONTHTICK Interrupt Enable"]
pub type MonthtickW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC0 Interrupt Enable"]
    #[inline(always)]
    pub fn cc0(&self) -> Cc0R {
        Cc0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC1 Interrupt Enable"]
    #[inline(always)]
    pub fn cc1(&self) -> Cc1R {
        Cc1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC2 Interrupt Enable"]
    #[inline(always)]
    pub fn cc2(&self) -> Cc2R {
        Cc2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OSCFAIL Interrupt Enable"]
    #[inline(always)]
    pub fn oscfail(&self) -> OscfailR {
        OscfailR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CNTTICK Interrupt Enable"]
    #[inline(always)]
    pub fn cnttick(&self) -> CnttickR {
        CnttickR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MINTICK Interrupt Enable"]
    #[inline(always)]
    pub fn mintick(&self) -> MintickR {
        MintickR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HOURTICK Interrupt Enable"]
    #[inline(always)]
    pub fn hourtick(&self) -> HourtickR {
        HourtickR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DAYTICK Interrupt Enable"]
    #[inline(always)]
    pub fn daytick(&self) -> DaytickR {
        DaytickR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DAYOWOF Interrupt Enable"]
    #[inline(always)]
    pub fn dayowof(&self) -> DayowofR {
        DayowofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MONTHTICK Interrupt Enable"]
    #[inline(always)]
    pub fn monthtick(&self) -> MonthtickR {
        MonthtickR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("of", &self.of())
            .field("cc0", &self.cc0())
            .field("cc1", &self.cc1())
            .field("cc2", &self.cc2())
            .field("oscfail", &self.oscfail())
            .field("cnttick", &self.cnttick())
            .field("mintick", &self.mintick())
            .field("hourtick", &self.hourtick())
            .field("daytick", &self.daytick())
            .field("dayowof", &self.dayowof())
            .field("monthtick", &self.monthtick())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OfW<IENrs> {
        OfW::new(self, 0)
    }
    #[doc = "Bit 1 - CC0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc0(&mut self) -> Cc0W<IENrs> {
        Cc0W::new(self, 1)
    }
    #[doc = "Bit 2 - CC1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> Cc1W<IENrs> {
        Cc1W::new(self, 2)
    }
    #[doc = "Bit 3 - CC2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> Cc2W<IENrs> {
        Cc2W::new(self, 3)
    }
    #[doc = "Bit 4 - OSCFAIL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oscfail(&mut self) -> OscfailW<IENrs> {
        OscfailW::new(self, 4)
    }
    #[doc = "Bit 5 - CNTTICK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cnttick(&mut self) -> CnttickW<IENrs> {
        CnttickW::new(self, 5)
    }
    #[doc = "Bit 6 - MINTICK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mintick(&mut self) -> MintickW<IENrs> {
        MintickW::new(self, 6)
    }
    #[doc = "Bit 7 - HOURTICK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hourtick(&mut self) -> HourtickW<IENrs> {
        HourtickW::new(self, 7)
    }
    #[doc = "Bit 8 - DAYTICK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn daytick(&mut self) -> DaytickW<IENrs> {
        DaytickW::new(self, 8)
    }
    #[doc = "Bit 9 - DAYOWOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dayowof(&mut self) -> DayowofW<IENrs> {
        DayowofW::new(self, 9)
    }
    #[doc = "Bit 10 - MONTHTICK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn monthtick(&mut self) -> MonthtickW<IENrs> {
        MonthtickW::new(self, 10)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IENrs;
impl crate::RegisterSpec for IENrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IENrs {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IENrs {
    const RESET_VALUE: u32 = 0;
}
