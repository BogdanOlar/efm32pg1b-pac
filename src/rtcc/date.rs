///Register `DATE` reader
pub type R = crate::R<DATErs>;
///Register `DATE` writer
pub type W = crate::W<DATErs>;
///Field `DAYOMU` reader - Day of Month, Units
pub type DayomuR = crate::FieldReader;
///Field `DAYOMU` writer - Day of Month, Units
pub type DayomuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DAYOMT` reader - Day of Month, Tens
pub type DayomtR = crate::FieldReader;
///Field `DAYOMT` writer - Day of Month, Tens
pub type DayomtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MONTHU` reader - Month, Units
pub type MonthuR = crate::FieldReader;
///Field `MONTHU` writer - Month, Units
pub type MonthuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MONTHT` reader - Month, Tens
pub type MonthtR = crate::BitReader;
///Field `MONTHT` writer - Month, Tens
pub type MonthtW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `YEARU` reader - Year, Units
pub type YearuR = crate::FieldReader;
///Field `YEARU` writer - Year, Units
pub type YearuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `YEART` reader - Year, Tens
pub type YeartR = crate::FieldReader;
///Field `YEART` writer - Year, Tens
pub type YeartW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DAYOW` reader - Day of Week
pub type DayowR = crate::FieldReader;
///Field `DAYOW` writer - Day of Week
pub type DayowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:3 - Day of Month, Units
    #[inline(always)]
    pub fn dayomu(&self) -> DayomuR {
        DayomuR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - Day of Month, Tens
    #[inline(always)]
    pub fn dayomt(&self) -> DayomtR {
        DayomtR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:11 - Month, Units
    #[inline(always)]
    pub fn monthu(&self) -> MonthuR {
        MonthuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Month, Tens
    #[inline(always)]
    pub fn montht(&self) -> MonthtR {
        MonthtR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 16:19 - Year, Units
    #[inline(always)]
    pub fn yearu(&self) -> YearuR {
        YearuR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Year, Tens
    #[inline(always)]
    pub fn yeart(&self) -> YeartR {
        YeartR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:26 - Day of Week
    #[inline(always)]
    pub fn dayow(&self) -> DayowR {
        DayowR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field("dayomu", &self.dayomu())
            .field("dayomt", &self.dayomt())
            .field("monthu", &self.monthu())
            .field("montht", &self.montht())
            .field("yearu", &self.yearu())
            .field("yeart", &self.yeart())
            .field("dayow", &self.dayow())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Day of Month, Units
    #[inline(always)]
    #[must_use]
    pub fn dayomu(&mut self) -> DayomuW<DATErs> {
        DayomuW::new(self, 0)
    }
    ///Bits 4:5 - Day of Month, Tens
    #[inline(always)]
    #[must_use]
    pub fn dayomt(&mut self) -> DayomtW<DATErs> {
        DayomtW::new(self, 4)
    }
    ///Bits 8:11 - Month, Units
    #[inline(always)]
    #[must_use]
    pub fn monthu(&mut self) -> MonthuW<DATErs> {
        MonthuW::new(self, 8)
    }
    ///Bit 12 - Month, Tens
    #[inline(always)]
    #[must_use]
    pub fn montht(&mut self) -> MonthtW<DATErs> {
        MonthtW::new(self, 12)
    }
    ///Bits 16:19 - Year, Units
    #[inline(always)]
    #[must_use]
    pub fn yearu(&mut self) -> YearuW<DATErs> {
        YearuW::new(self, 16)
    }
    ///Bits 20:23 - Year, Tens
    #[inline(always)]
    #[must_use]
    pub fn yeart(&mut self) -> YeartW<DATErs> {
        YeartW::new(self, 20)
    }
    ///Bits 24:26 - Day of Week
    #[inline(always)]
    #[must_use]
    pub fn dayow(&mut self) -> DayowW<DATErs> {
        DayowW::new(self, 24)
    }
}
///Date Register
///
///You can [`read`](crate::Reg::read) this register and get [`date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DATErs;
impl crate::RegisterSpec for DATErs {
    type Ux = u32;
}
///`read()` method returns [`date::R`](R) reader structure
impl crate::Readable for DATErs {}
///`write(|w| ..)` method takes [`date::W`](W) writer structure
impl crate::Writable for DATErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATE to value 0
impl crate::Resettable for DATErs {
    const RESET_VALUE: u32 = 0;
}
