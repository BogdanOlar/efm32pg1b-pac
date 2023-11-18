#[doc = "Register `PWRCTRL` reader"]
pub type R = crate::R<PWRCTRL_SPEC>;
#[doc = "Register `PWRCTRL` writer"]
pub type W = crate::W<PWRCTRL_SPEC>;
#[doc = "Field `ANASW` reader - Analog Switch Selection"]
pub type ANASW_R = crate::BitReader;
#[doc = "Field `ANASW` writer - Analog Switch Selection"]
pub type ANASW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 5 - Analog Switch Selection"]
    #[inline(always)]
    pub fn anasw(&self) -> ANASW_R {
        ANASW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRCTRL")
            .field("anasw", &format_args!("{}", self.anasw().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PWRCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 5 - Analog Switch Selection"]
    #[inline(always)]
    #[must_use]
    pub fn anasw(&mut self) -> ANASW_W<PWRCTRL_SPEC, 5> {
        ANASW_W::new(self)
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
#[doc = "Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRCTRL_SPEC;
impl crate::RegisterSpec for PWRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrctrl::R`](R) reader structure"]
impl crate::Readable for PWRCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwrctrl::W`](W) writer structure"]
impl crate::Writable for PWRCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCTRL to value 0"]
impl crate::Resettable for PWRCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
