///Register `CC0_TIME` reader
pub type R = crate::R<CC0_TIMErs>;
///Register `CC0_TIME` writer
pub type W = crate::W<CC0_TIMErs>;
///Field `SECU` reader - Seconds, Units
pub type SecuR = crate::FieldReader;
///Field `SECU` writer - Seconds, Units
pub type SecuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SECT` reader - Seconds, Tens
pub type SectR = crate::FieldReader;
///Field `SECT` writer - Seconds, Tens
pub type SectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MINU` reader - Minutes, Units
pub type MinuR = crate::FieldReader;
///Field `MINU` writer - Minutes, Units
pub type MinuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MINT` reader - Minutes, Tens
pub type MintR = crate::FieldReader;
///Field `MINT` writer - Minutes, Tens
pub type MintW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HOURU` reader - Hours, Units
pub type HouruR = crate::FieldReader;
///Field `HOURU` writer - Hours, Units
pub type HouruW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HOURT` reader - Hours, Tens
pub type HourtR = crate::FieldReader;
///Field `HOURT` writer - Hours, Tens
pub type HourtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - Seconds, Units
    #[inline(always)]
    pub fn secu(&self) -> SecuR {
        SecuR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - Seconds, Tens
    #[inline(always)]
    pub fn sect(&self) -> SectR {
        SectR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:11 - Minutes, Units
    #[inline(always)]
    pub fn minu(&self) -> MinuR {
        MinuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:14 - Minutes, Tens
    #[inline(always)]
    pub fn mint(&self) -> MintR {
        MintR::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:19 - Hours, Units
    #[inline(always)]
    pub fn houru(&self) -> HouruR {
        HouruR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:21 - Hours, Tens
    #[inline(always)]
    pub fn hourt(&self) -> HourtR {
        HourtR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CC0_TIME")
            .field("secu", &self.secu())
            .field("sect", &self.sect())
            .field("minu", &self.minu())
            .field("mint", &self.mint())
            .field("houru", &self.houru())
            .field("hourt", &self.hourt())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Seconds, Units
    #[inline(always)]
    #[must_use]
    pub fn secu(&mut self) -> SecuW<CC0_TIMErs> {
        SecuW::new(self, 0)
    }
    ///Bits 4:6 - Seconds, Tens
    #[inline(always)]
    #[must_use]
    pub fn sect(&mut self) -> SectW<CC0_TIMErs> {
        SectW::new(self, 4)
    }
    ///Bits 8:11 - Minutes, Units
    #[inline(always)]
    #[must_use]
    pub fn minu(&mut self) -> MinuW<CC0_TIMErs> {
        MinuW::new(self, 8)
    }
    ///Bits 12:14 - Minutes, Tens
    #[inline(always)]
    #[must_use]
    pub fn mint(&mut self) -> MintW<CC0_TIMErs> {
        MintW::new(self, 12)
    }
    ///Bits 16:19 - Hours, Units
    #[inline(always)]
    #[must_use]
    pub fn houru(&mut self) -> HouruW<CC0_TIMErs> {
        HouruW::new(self, 16)
    }
    ///Bits 20:21 - Hours, Tens
    #[inline(always)]
    #[must_use]
    pub fn hourt(&mut self) -> HourtW<CC0_TIMErs> {
        HourtW::new(self, 20)
    }
}
///Capture/Compare Time Register
///
///You can [`read`](crate::Reg::read) this register and get [`cc0_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CC0_TIMErs;
impl crate::RegisterSpec for CC0_TIMErs {
    type Ux = u32;
}
///`read()` method returns [`cc0_time::R`](R) reader structure
impl crate::Readable for CC0_TIMErs {}
///`write(|w| ..)` method takes [`cc0_time::W`](W) writer structure
impl crate::Writable for CC0_TIMErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CC0_TIME to value 0
impl crate::Resettable for CC0_TIMErs {
    const RESET_VALUE: u32 = 0;
}
