#[doc = "Register `HFXOSTARTUPCTRL` reader"]
pub struct R(crate::R<HFXOSTARTUPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFXOSTARTUPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFXOSTARTUPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFXOSTARTUPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFXOSTARTUPCTRL` writer"]
pub struct W(crate::W<HFXOSTARTUPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFXOSTARTUPCTRL_SPEC>;
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
impl From<crate::W<HFXOSTARTUPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFXOSTARTUPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBTRIMXOCORE` reader - Sets the Startup Oscillator Core Bias Current"]
pub type IBTRIMXOCORE_R = crate::FieldReader;
#[doc = "Field `IBTRIMXOCORE` writer - Sets the Startup Oscillator Core Bias Current"]
pub type IBTRIMXOCORE_W<'a, const O: u8> = crate::FieldWriter<'a, HFXOSTARTUPCTRL_SPEC, 7, O>;
#[doc = "Field `CTUNE` reader - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_R = crate::FieldReader<u16>;
#[doc = "Field `CTUNE` writer - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_W<'a, const O: u8> = crate::FieldWriter<'a, HFXOSTARTUPCTRL_SPEC, 9, O, u16>;
#[doc = "Field `RESERVED0` reader - This Field is Reserved. It Should Be Set to 0x9"]
pub type RESERVED0_R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - This Field is Reserved. It Should Be Set to 0x9"]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, HFXOSTARTUPCTRL_SPEC, 7, O>;
#[doc = "Field `RESERVED1` reader - Sets the Regulator Output Current Level (shunt Regulator)"]
pub type RESERVED1_R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - Sets the Regulator Output Current Level (shunt Regulator)"]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, HFXOSTARTUPCTRL_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:6 - Sets the Startup Oscillator Core Bias Current"]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORE_R {
        IBTRIMXOCORE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&self) -> CTUNE_R {
        CTUNE_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bits 21:27 - This Field is Reserved. It Should Be Set to 0x9"]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 21) & 0x7f) as u8)
    }
    #[doc = "Bits 28:31 - Sets the Regulator Output Current Level (shunt Regulator)"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Sets the Startup Oscillator Core Bias Current"]
    #[inline(always)]
    #[must_use]
    pub fn ibtrimxocore(&mut self) -> IBTRIMXOCORE_W<0> {
        IBTRIMXOCORE_W::new(self)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    #[must_use]
    pub fn ctune(&mut self) -> CTUNE_W<11> {
        CTUNE_W::new(self)
    }
    #[doc = "Bits 21:27 - This Field is Reserved. It Should Be Set to 0x9"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<21> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 28:31 - Sets the Regulator Output Current Level (shunt Regulator)"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<28> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFXO Startup Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxostartupctrl](index.html) module"]
pub struct HFXOSTARTUPCTRL_SPEC;
impl crate::RegisterSpec for HFXOSTARTUPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfxostartupctrl::R](R) reader structure"]
impl crate::Readable for HFXOSTARTUPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfxostartupctrl::W](W) writer structure"]
impl crate::Writable for HFXOSTARTUPCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFXOSTARTUPCTRL to value 0xa125_0060"]
impl crate::Resettable for HFXOSTARTUPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xa125_0060;
}
