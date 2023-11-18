#[doc = "Register `PA_DOUT` reader"]
pub type R = crate::R<PA_DOUT_SPEC>;
#[doc = "Register `PA_DOUT` writer"]
pub type W = crate::W<PA_DOUT_SPEC>;
#[doc = "Field `DOUT` reader - Data Out"]
pub type DOUT_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT` writer - Data Out"]
pub type DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Out"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PA_DOUT")
            .field("dout", &format_args!("{}", self.dout().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PA_DOUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout(&mut self) -> DOUT_W<PA_DOUT_SPEC, 0> {
        DOUT_W::new(self)
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
#[doc = "Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_dout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_dout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PA_DOUT_SPEC;
impl crate::RegisterSpec for PA_DOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_dout::R`](R) reader structure"]
impl crate::Readable for PA_DOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pa_dout::W`](W) writer structure"]
impl crate::Writable for PA_DOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PA_DOUT to value 0"]
impl crate::Resettable for PA_DOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
