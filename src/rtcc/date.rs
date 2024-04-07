#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATErs>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DATErs>;
#[doc = "Field `DAYOMU` reader - Day of Month, Units"]
pub type DayomuR = crate::FieldReader;
#[doc = "Field `DAYOMU` writer - Day of Month, Units"]
pub type DayomuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAYOMT` reader - Day of Month, Tens"]
pub type DayomtR = crate::FieldReader;
#[doc = "Field `DAYOMT` writer - Day of Month, Tens"]
pub type DayomtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MONTHU` reader - Month, Units"]
pub type MonthuR = crate::FieldReader;
#[doc = "Field `MONTHU` writer - Month, Units"]
pub type MonthuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MONTHT` reader - Month, Tens"]
pub type MonthtR = crate::BitReader;
#[doc = "Field `MONTHT` writer - Month, Tens"]
pub type MonthtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YEARU` reader - Year, Units"]
pub type YearuR = crate::FieldReader;
#[doc = "Field `YEARU` writer - Year, Units"]
pub type YearuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YEART` reader - Year, Tens"]
pub type YeartR = crate::FieldReader;
#[doc = "Field `YEART` writer - Year, Tens"]
pub type YeartW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAYOW` reader - Day of Week"]
pub type DayowR = crate::FieldReader;
#[doc = "Field `DAYOW` writer - Day of Week"]
pub type DayowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Day of Month, Units"]
    #[inline(always)]
    pub fn dayomu(&self) -> DayomuR {
        DayomuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Day of Month, Tens"]
    #[inline(always)]
    pub fn dayomt(&self) -> DayomtR {
        DayomtR::new(((self.bits >> 4) & 3) as u8)
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
    #[doc = "Bits 16:19 - Year, Units"]
    #[inline(always)]
    pub fn yearu(&self) -> YearuR {
        YearuR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Year, Tens"]
    #[inline(always)]
    pub fn yeart(&self) -> YeartR {
        YeartR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    pub fn dayow(&self) -> DayowR {
        DayowR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Day of Month, Units"]
    #[inline(always)]
    #[must_use]
    pub fn dayomu(&mut self) -> DayomuW<DATErs> {
        DayomuW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Day of Month, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn dayomt(&mut self) -> DayomtW<DATErs> {
        DayomtW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Month, Units"]
    #[inline(always)]
    #[must_use]
    pub fn monthu(&mut self) -> MonthuW<DATErs> {
        MonthuW::new(self, 8)
    }
    #[doc = "Bit 12 - Month, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn montht(&mut self) -> MonthtW<DATErs> {
        MonthtW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Year, Units"]
    #[inline(always)]
    #[must_use]
    pub fn yearu(&mut self) -> YearuW<DATErs> {
        YearuW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Year, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn yeart(&mut self) -> YeartW<DATErs> {
        YeartW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    #[must_use]
    pub fn dayow(&mut self) -> DayowW<DATErs> {
        DayowW::new(self, 24)
    }
}
#[doc = "Date Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATErs;
impl crate::RegisterSpec for DATErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DATErs {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DATErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATE to value 0"]
impl crate::Resettable for DATErs {
    const RESET_VALUE: u32 = 0;
}
