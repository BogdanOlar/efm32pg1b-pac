#[doc = "Register `CC2_DATE` reader"]
pub type R = crate::R<CC2_DATErs>;
#[doc = "Register `CC2_DATE` writer"]
pub type W = crate::W<CC2_DATErs>;
#[doc = "Field `DAYU` reader - Day of Month/week, Units"]
pub type DAYU_R = crate::FieldReader;
#[doc = "Field `DAYU` writer - Day of Month/week, Units"]
pub type DAYU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAYT` reader - Day of Month/week, Tens"]
pub type DAYT_R = crate::FieldReader;
#[doc = "Field `DAYT` writer - Day of Month/week, Tens"]
pub type DAYT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MONTHU` reader - Month, Units"]
pub type MONTHU_R = crate::FieldReader;
#[doc = "Field `MONTHU` writer - Month, Units"]
pub type MONTHU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MONTHT` reader - Month, Tens"]
pub type MONTHT_R = crate::BitReader;
#[doc = "Field `MONTHT` writer - Month, Tens"]
pub type MONTHT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Day of Month/week, Units"]
    #[inline(always)]
    pub fn dayu(&self) -> DAYU_R {
        DAYU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Day of Month/week, Tens"]
    #[inline(always)]
    pub fn dayt(&self) -> DAYT_R {
        DAYT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month, Units"]
    #[inline(always)]
    pub fn monthu(&self) -> MONTHU_R {
        MONTHU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month, Tens"]
    #[inline(always)]
    pub fn montht(&self) -> MONTHT_R {
        MONTHT_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Day of Month/week, Units"]
    #[inline(always)]
    #[must_use]
    pub fn dayu(&mut self) -> DAYU_W<CC2_DATErs> {
        DAYU_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Day of Month/week, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn dayt(&mut self) -> DAYT_W<CC2_DATErs> {
        DAYT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Month, Units"]
    #[inline(always)]
    #[must_use]
    pub fn monthu(&mut self) -> MONTHU_W<CC2_DATErs> {
        MONTHU_W::new(self, 8)
    }
    #[doc = "Bit 12 - Month, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn montht(&mut self) -> MONTHT_W<CC2_DATErs> {
        MONTHT_W::new(self, 12)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Capture/Compare Date Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc2_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc2_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC2_DATErs;
impl crate::RegisterSpec for CC2_DATErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2_date::R`](R) reader structure"]
impl crate::Readable for CC2_DATErs {}
#[doc = "`write(|w| ..)` method takes [`cc2_date::W`](W) writer structure"]
impl crate::Writable for CC2_DATErs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC2_DATE to value 0"]
impl crate::Resettable for CC2_DATErs {
    const RESET_VALUE: u32 = 0;
}
