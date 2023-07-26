#[doc = "Register `CC0_CCV` reader"]
pub struct R(crate::R<CC0_CCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC0_CCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC0_CCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC0_CCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC0_CCV` writer"]
pub struct W(crate::W<CC0_CCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC0_CCV_SPEC>;
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
impl From<crate::W<CC0_CCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC0_CCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCV` reader - CC Channel Value"]
pub type CCV_R = crate::FieldReader<u16>;
#[doc = "Field `CCV` writer - CC Channel Value"]
pub type CCV_W<'a, const O: u8> = crate::FieldWriter<'a, CC0_CCV_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CC Channel Value"]
    #[inline(always)]
    pub fn ccv(&self) -> CCV_R {
        CCV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CC Channel Value"]
    #[inline(always)]
    #[must_use]
    pub fn ccv(&mut self) -> CCV_W<0> {
        CCV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CC Channel Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc0_ccv](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct CC0_CCV_SPEC;
impl crate::RegisterSpec for CC0_CCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc0_ccv::R](R) reader structure"]
impl crate::Readable for CC0_CCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc0_ccv::W](W) writer structure"]
impl crate::Writable for CC0_CCV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC0_CCV to value 0"]
impl crate::Resettable for CC0_CCV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
