#[doc = "Register `CC0_DATE` reader"]
pub type R = crate::R<CC0_DATErs>;
#[doc = "Register `CC0_DATE` writer"]
pub type W = crate::W<CC0_DATErs>;
#[doc = "Field `DAYU` reader - Day of Month/week, Units"]
pub type DayuR = crate::FieldReader;
#[doc = "Field `DAYU` writer - Day of Month/week, Units"]
pub type DayuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAYT` reader - Day of Month/week, Tens"]
pub type DaytR = crate::FieldReader;
#[doc = "Field `DAYT` writer - Day of Month/week, Tens"]
pub type DaytW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MONTHU` reader - Month, Units"]
pub type MonthuR = crate::FieldReader;
#[doc = "Field `MONTHU` writer - Month, Units"]
pub type MonthuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MONTHT` reader - Month, Tens"]
pub type MonthtR = crate::BitReader;
#[doc = "Field `MONTHT` writer - Month, Tens"]
pub type MonthtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Day of Month/week, Units"]
    #[inline(always)]
    pub fn dayu(&self) -> DayuR {
        DayuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Day of Month/week, Tens"]
    #[inline(always)]
    pub fn dayt(&self) -> DaytR {
        DaytR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month, Units"]
    #[inline(always)]
    pub fn monthu(&self) -> MonthuR {
        MonthuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month, Tens"]
    #[inline(always)]
    pub fn montht(&self) -> MonthtR {
        MonthtR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Day of Month/week, Units"]
    #[inline(always)]
    #[must_use]
    pub fn dayu(&mut self) -> DayuW<CC0_DATErs> {
        DayuW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Day of Month/week, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn dayt(&mut self) -> DaytW<CC0_DATErs> {
        DaytW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Month, Units"]
    #[inline(always)]
    #[must_use]
    pub fn monthu(&mut self) -> MonthuW<CC0_DATErs> {
        MonthuW::new(self, 8)
    }
    #[doc = "Bit 12 - Month, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn montht(&mut self) -> MonthtW<CC0_DATErs> {
        MonthtW::new(self, 12)
    }
}
#[doc = "Capture/Compare Date Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc0_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc0_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC0_DATErs;
impl crate::RegisterSpec for CC0_DATErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc0_date::R`](R) reader structure"]
impl crate::Readable for CC0_DATErs {}
#[doc = "`write(|w| ..)` method takes [`cc0_date::W`](W) writer structure"]
impl crate::Writable for CC0_DATErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC0_DATE to value 0"]
impl crate::Resettable for CC0_DATErs {
    const RESET_VALUE: u32 = 0;
}
