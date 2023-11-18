#[doc = "Register `BIASCONF` reader"]
pub type R = crate::R<BIASCONF_SPEC>;
#[doc = "Register `BIASCONF` writer"]
pub type W = crate::W<BIASCONF_SPEC>;
#[doc = "Field `NADUTYEM01` reader - NA DUTY in EM01"]
pub type NADUTYEM01_R = crate::BitReader;
#[doc = "Field `NADUTYEM01` writer - NA DUTY in EM01"]
pub type NADUTYEM01_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPEM01` reader - LP in EM01"]
pub type LPEM01_R = crate::BitReader;
#[doc = "Field `LPEM01` writer - LP in EM01"]
pub type LPEM01_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GMCEM23` reader - GMC in EM234"]
pub type GMCEM23_R = crate::BitReader;
#[doc = "Field `GMCEM23` writer - GMC in EM234"]
pub type GMCEM23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UADUTYEM23` reader - UADUTY in EM234"]
pub type UADUTYEM23_R = crate::BitReader;
#[doc = "Field `UADUTYEM23` writer - UADUTY in EM234"]
pub type UADUTYEM23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NADUTYEM23` reader - NA DUTY in EM234"]
pub type NADUTYEM23_R = crate::BitReader;
#[doc = "Field `NADUTYEM23` writer - NA DUTY in EM234"]
pub type NADUTYEM23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPEM23` reader - LP in EM234"]
pub type LPEM23_R = crate::BitReader;
#[doc = "Field `LPEM23` writer - LP in EM234"]
pub type LPEM23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - NA DUTY in EM01"]
    #[inline(always)]
    pub fn nadutyem01(&self) -> NADUTYEM01_R {
        NADUTYEM01_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LP in EM01"]
    #[inline(always)]
    pub fn lpem01(&self) -> LPEM01_R {
        LPEM01_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GMC in EM234"]
    #[inline(always)]
    pub fn gmcem23(&self) -> GMCEM23_R {
        GMCEM23_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UADUTY in EM234"]
    #[inline(always)]
    pub fn uadutyem23(&self) -> UADUTYEM23_R {
        UADUTYEM23_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA DUTY in EM234"]
    #[inline(always)]
    pub fn nadutyem23(&self) -> NADUTYEM23_R {
        NADUTYEM23_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LP in EM234"]
    #[inline(always)]
    pub fn lpem23(&self) -> LPEM23_R {
        LPEM23_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BIASCONF")
            .field("nadutyem01", &format_args!("{}", self.nadutyem01().bit()))
            .field("lpem01", &format_args!("{}", self.lpem01().bit()))
            .field("gmcem23", &format_args!("{}", self.gmcem23().bit()))
            .field("uadutyem23", &format_args!("{}", self.uadutyem23().bit()))
            .field("nadutyem23", &format_args!("{}", self.nadutyem23().bit()))
            .field("lpem23", &format_args!("{}", self.lpem23().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<BIASCONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2 - NA DUTY in EM01"]
    #[inline(always)]
    #[must_use]
    pub fn nadutyem01(&mut self) -> NADUTYEM01_W<BIASCONF_SPEC, 2> {
        NADUTYEM01_W::new(self)
    }
    #[doc = "Bit 3 - LP in EM01"]
    #[inline(always)]
    #[must_use]
    pub fn lpem01(&mut self) -> LPEM01_W<BIASCONF_SPEC, 3> {
        LPEM01_W::new(self)
    }
    #[doc = "Bit 4 - GMC in EM234"]
    #[inline(always)]
    #[must_use]
    pub fn gmcem23(&mut self) -> GMCEM23_W<BIASCONF_SPEC, 4> {
        GMCEM23_W::new(self)
    }
    #[doc = "Bit 5 - UADUTY in EM234"]
    #[inline(always)]
    #[must_use]
    pub fn uadutyem23(&mut self) -> UADUTYEM23_W<BIASCONF_SPEC, 5> {
        UADUTYEM23_W::new(self)
    }
    #[doc = "Bit 6 - NA DUTY in EM234"]
    #[inline(always)]
    #[must_use]
    pub fn nadutyem23(&mut self) -> NADUTYEM23_W<BIASCONF_SPEC, 6> {
        NADUTYEM23_W::new(self)
    }
    #[doc = "Bit 7 - LP in EM234"]
    #[inline(always)]
    #[must_use]
    pub fn lpem23(&mut self) -> LPEM23_W<BIASCONF_SPEC, 7> {
        LPEM23_W::new(self)
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
#[doc = "Configurations Related to the Bias\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`biasconf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`biasconf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIASCONF_SPEC;
impl crate::RegisterSpec for BIASCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`biasconf::R`](R) reader structure"]
impl crate::Readable for BIASCONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`biasconf::W`](W) writer structure"]
impl crate::Writable for BIASCONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BIASCONF to value 0xf8"]
impl crate::Resettable for BIASCONF_SPEC {
    const RESET_VALUE: Self::Ux = 0xf8;
}
