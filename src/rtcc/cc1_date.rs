///Register `CC1_DATE` reader
pub type R = crate::R<CC1_DATErs>;
///Register `CC1_DATE` writer
pub type W = crate::W<CC1_DATErs>;
///Field `DAYU` reader - Day of Month/week, Units
pub type DayuR = crate::FieldReader;
///Field `DAYU` writer - Day of Month/week, Units
pub type DayuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DAYT` reader - Day of Month/week, Tens
pub type DaytR = crate::FieldReader;
///Field `DAYT` writer - Day of Month/week, Tens
pub type DaytW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MONTHU` reader - Month, Units
pub type MonthuR = crate::FieldReader;
///Field `MONTHU` writer - Month, Units
pub type MonthuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MONTHT` reader - Month, Tens
pub type MonthtR = crate::BitReader;
///Field `MONTHT` writer - Month, Tens
pub type MonthtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Day of Month/week, Units
    #[inline(always)]
    pub fn dayu(&self) -> DayuR {
        DayuR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - Day of Month/week, Tens
    #[inline(always)]
    pub fn dayt(&self) -> DaytR {
        DaytR::new(((self.bits >> 4) & 3) as u8)
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CC1_DATE")
            .field("dayu", &self.dayu())
            .field("dayt", &self.dayt())
            .field("monthu", &self.monthu())
            .field("montht", &self.montht())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Day of Month/week, Units
    #[inline(always)]
    #[must_use]
    pub fn dayu(&mut self) -> DayuW<CC1_DATErs> {
        DayuW::new(self, 0)
    }
    ///Bits 4:5 - Day of Month/week, Tens
    #[inline(always)]
    #[must_use]
    pub fn dayt(&mut self) -> DaytW<CC1_DATErs> {
        DaytW::new(self, 4)
    }
    ///Bits 8:11 - Month, Units
    #[inline(always)]
    #[must_use]
    pub fn monthu(&mut self) -> MonthuW<CC1_DATErs> {
        MonthuW::new(self, 8)
    }
    ///Bit 12 - Month, Tens
    #[inline(always)]
    #[must_use]
    pub fn montht(&mut self) -> MonthtW<CC1_DATErs> {
        MonthtW::new(self, 12)
    }
}
///Capture/Compare Date Register
///
///You can [`read`](crate::Reg::read) this register and get [`cc1_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CC1_DATErs;
impl crate::RegisterSpec for CC1_DATErs {
    type Ux = u32;
}
///`read()` method returns [`cc1_date::R`](R) reader structure
impl crate::Readable for CC1_DATErs {}
///`write(|w| ..)` method takes [`cc1_date::W`](W) writer structure
impl crate::Writable for CC1_DATErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CC1_DATE to value 0
impl crate::Resettable for CC1_DATErs {
    const RESET_VALUE: u32 = 0;
}
