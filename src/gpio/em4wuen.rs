#[doc = "Register `EM4WUEN` reader"]
pub type R = crate::R<EM4WUEN_SPEC>;
#[doc = "Register `EM4WUEN` writer"]
pub type W = crate::W<EM4WUEN_SPEC>;
#[doc = "Field `EM4WUEN` reader - EM4 Wake Up Enable"]
pub type EM4WUEN_R = crate::FieldReader<u16>;
#[doc = "Field `EM4WUEN` writer - EM4 Wake Up Enable"]
pub type EM4WUEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 16:31 - EM4 Wake Up Enable"]
    #[inline(always)]
    pub fn em4wuen(&self) -> EM4WUEN_R {
        EM4WUEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EM4WUEN")
            .field("em4wuen", &format_args!("{}", self.em4wuen().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<EM4WUEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 16:31 - EM4 Wake Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em4wuen(&mut self) -> EM4WUEN_W<EM4WUEN_SPEC, 16> {
        EM4WUEN_W::new(self)
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
#[doc = "EM4 Wake Up Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`em4wuen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em4wuen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EM4WUEN_SPEC;
impl crate::RegisterSpec for EM4WUEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em4wuen::R`](R) reader structure"]
impl crate::Readable for EM4WUEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`em4wuen::W`](W) writer structure"]
impl crate::Writable for EM4WUEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EM4WUEN to value 0"]
impl crate::Resettable for EM4WUEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
