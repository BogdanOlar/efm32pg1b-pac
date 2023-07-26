#[doc = "Register `DDATA4` reader"]
pub struct R(crate::R<DDATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDATA4` writer"]
pub struct W(crate::W<DDATA4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DDATA4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDATA4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDATA4` reader - Double Data 0 Access"]
pub type DDATA4_R = crate::FieldReader<u32>;
#[doc = "Field `DDATA4` writer - Double Data 0 Access"]
pub type DDATA4_W<'a, const O: u8> = crate::FieldWriter<'a, DDATA4_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata4(&self) -> DDATA4_R {
        DDATA4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    #[must_use]
    pub fn ddata4(&mut self) -> DDATA4_W<0> {
        DDATA4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDATA4 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddata4](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct DDATA4_SPEC;
impl crate::RegisterSpec for DDATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddata4::R](R) reader structure"]
impl crate::Readable for DDATA4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddata4::W](W) writer structure"]
impl crate::Writable for DDATA4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDATA4 to value 0"]
impl crate::Resettable for DDATA4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
