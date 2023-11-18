#[doc = "Register `DCDCLPCTRL` reader"]
pub type R = crate::R<DCDCLPCTRL_SPEC>;
#[doc = "Register `DCDCLPCTRL` writer"]
pub type W = crate::W<DCDCLPCTRL_SPEC>;
#[doc = "Field `LPCMPHYSSEL` reader - LP Mode Hysteresis Selection"]
pub type LPCMPHYSSEL_R = crate::FieldReader;
#[doc = "Field `LPCMPHYSSEL` writer - LP Mode Hysteresis Selection"]
pub type LPCMPHYSSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `LPVREFDUTYEN` reader - LP Mode Duty Cycling Enable"]
pub type LPVREFDUTYEN_R = crate::BitReader;
#[doc = "Field `LPVREFDUTYEN` writer - LP Mode Duty Cycling Enable"]
pub type LPVREFDUTYEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPBLANK` reader - Reserved for internal use. Do not change."]
pub type LPBLANK_R = crate::FieldReader;
#[doc = "Field `LPBLANK` writer - Reserved for internal use. Do not change."]
pub type LPBLANK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection"]
    #[inline(always)]
    pub fn lpcmphyssel(&self) -> LPCMPHYSSEL_R {
        LPCMPHYSSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - LP Mode Duty Cycling Enable"]
    #[inline(always)]
    pub fn lpvrefdutyen(&self) -> LPVREFDUTYEN_R {
        LPVREFDUTYEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn lpblank(&self) -> LPBLANK_R {
        LPBLANK_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDCLPCTRL")
            .field(
                "lpcmphyssel",
                &format_args!("{}", self.lpcmphyssel().bits()),
            )
            .field(
                "lpvrefdutyen",
                &format_args!("{}", self.lpvrefdutyen().bit()),
            )
            .field("lpblank", &format_args!("{}", self.lpblank().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DCDCLPCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpcmphyssel(&mut self) -> LPCMPHYSSEL_W<DCDCLPCTRL_SPEC, 12> {
        LPCMPHYSSEL_W::new(self)
    }
    #[doc = "Bit 24 - LP Mode Duty Cycling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpvrefdutyen(&mut self) -> LPVREFDUTYEN_W<DCDCLPCTRL_SPEC, 24> {
        LPVREFDUTYEN_W::new(self)
    }
    #[doc = "Bits 25:26 - Reserved for internal use. Do not change."]
    #[inline(always)]
    #[must_use]
    pub fn lpblank(&mut self) -> LPBLANK_W<DCDCLPCTRL_SPEC, 25> {
        LPBLANK_W::new(self)
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
#[doc = "DCDC Low Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdclpctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdclpctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDCLPCTRL_SPEC;
impl crate::RegisterSpec for DCDCLPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdclpctrl::R`](R) reader structure"]
impl crate::Readable for DCDCLPCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdclpctrl::W`](W) writer structure"]
impl crate::Writable for DCDCLPCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDCLPCTRL to value 0x7000"]
impl crate::Resettable for DCDCLPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x7000;
}
