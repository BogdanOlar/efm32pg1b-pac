#[doc = "Register `CC1_TIME` reader"]
pub type R = crate::R<CC1_TIMErs>;
#[doc = "Register `CC1_TIME` writer"]
pub type W = crate::W<CC1_TIMErs>;
#[doc = "Field `SECU` reader - Seconds, Units"]
pub type SecuR = crate::FieldReader;
#[doc = "Field `SECU` writer - Seconds, Units"]
pub type SecuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SECT` reader - Seconds, Tens"]
pub type SectR = crate::FieldReader;
#[doc = "Field `SECT` writer - Seconds, Tens"]
pub type SectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MINU` reader - Minutes, Units"]
pub type MinuR = crate::FieldReader;
#[doc = "Field `MINU` writer - Minutes, Units"]
pub type MinuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MINT` reader - Minutes, Tens"]
pub type MintR = crate::FieldReader;
#[doc = "Field `MINT` writer - Minutes, Tens"]
pub type MintW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HOURU` reader - Hours, Units"]
pub type HouruR = crate::FieldReader;
#[doc = "Field `HOURU` writer - Hours, Units"]
pub type HouruW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOURT` reader - Hours, Tens"]
pub type HourtR = crate::FieldReader;
#[doc = "Field `HOURT` writer - Hours, Tens"]
pub type HourtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Seconds, Units"]
    #[inline(always)]
    pub fn secu(&self) -> SecuR {
        SecuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Seconds, Tens"]
    #[inline(always)]
    pub fn sect(&self) -> SectR {
        SectR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minutes, Units"]
    #[inline(always)]
    pub fn minu(&self) -> MinuR {
        MinuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minutes, Tens"]
    #[inline(always)]
    pub fn mint(&self) -> MintR {
        MintR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Hours, Units"]
    #[inline(always)]
    pub fn houru(&self) -> HouruR {
        HouruR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hours, Tens"]
    #[inline(always)]
    pub fn hourt(&self) -> HourtR {
        HourtR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Seconds, Units"]
    #[inline(always)]
    #[must_use]
    pub fn secu(&mut self) -> SecuW<CC1_TIMErs> {
        SecuW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Seconds, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn sect(&mut self) -> SectW<CC1_TIMErs> {
        SectW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Minutes, Units"]
    #[inline(always)]
    #[must_use]
    pub fn minu(&mut self) -> MinuW<CC1_TIMErs> {
        MinuW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minutes, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn mint(&mut self) -> MintW<CC1_TIMErs> {
        MintW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Hours, Units"]
    #[inline(always)]
    #[must_use]
    pub fn houru(&mut self) -> HouruW<CC1_TIMErs> {
        HouruW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hours, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn hourt(&mut self) -> HourtW<CC1_TIMErs> {
        HourtW::new(self, 20)
    }
}
#[doc = "Capture/Compare Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc1_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc1_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC1_TIMErs;
impl crate::RegisterSpec for CC1_TIMErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc1_time::R`](R) reader structure"]
impl crate::Readable for CC1_TIMErs {}
#[doc = "`write(|w| ..)` method takes [`cc1_time::W`](W) writer structure"]
impl crate::Writable for CC1_TIMErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC1_TIME to value 0"]
impl crate::Resettable for CC1_TIMErs {
    const RESET_VALUE: u32 = 0;
}
