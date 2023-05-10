#[doc = "Register `DCDCTIMING` reader"]
pub struct R(crate::R<DCDCTIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCTIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCTIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCTIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCTIMING` writer"]
pub struct W(crate::W<DCDCTIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCTIMING_SPEC>;
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
impl From<crate::W<DCDCTIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCTIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPINITWAIT` reader - Low Power Initialization Wait Time"]
pub type LPINITWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPINITWAIT` writer - Low Power Initialization Wait Time"]
pub type LPINITWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDCTIMING_SPEC, u8, u8, 8, O>;
#[doc = "Field `COMPENPRCHGEN` reader - LN Mode Precharge Enable"]
pub type COMPENPRCHGEN_R = crate::BitReader<bool>;
#[doc = "Field `COMPENPRCHGEN` writer - LN Mode Precharge Enable"]
pub type COMPENPRCHGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDCTIMING_SPEC, bool, O>;
#[doc = "Field `LNWAIT` reader - Low Noise Controller Initialization Wait Time"]
pub type LNWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LNWAIT` writer - Low Noise Controller Initialization Wait Time"]
pub type LNWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDCTIMING_SPEC, u8, u8, 5, O>;
#[doc = "Field `BYPWAIT` reader - Bypass Mode Transition From Low Power or Low Noise Modes Wait Wait"]
pub type BYPWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYPWAIT` writer - Bypass Mode Transition From Low Power or Low Noise Modes Wait Wait"]
pub type BYPWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDCTIMING_SPEC, u8, u8, 8, O>;
#[doc = "Field `DUTYSCALE` reader - Select Bias Duty Cycle Clock"]
pub type DUTYSCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DUTYSCALE` writer - Select Bias Duty Cycle Clock"]
pub type DUTYSCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDCTIMING_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:7 - Low Power Initialization Wait Time"]
    #[inline(always)]
    pub fn lpinitwait(&self) -> LPINITWAIT_R {
        LPINITWAIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 11 - LN Mode Precharge Enable"]
    #[inline(always)]
    pub fn compenprchgen(&self) -> COMPENPRCHGEN_R {
        COMPENPRCHGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:16 - Low Noise Controller Initialization Wait Time"]
    #[inline(always)]
    pub fn lnwait(&self) -> LNWAIT_R {
        LNWAIT_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 20:27 - Bypass Mode Transition From Low Power or Low Noise Modes Wait Wait"]
    #[inline(always)]
    pub fn bypwait(&self) -> BYPWAIT_R {
        BYPWAIT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 29:30 - Select Bias Duty Cycle Clock"]
    #[inline(always)]
    pub fn dutyscale(&self) -> DUTYSCALE_R {
        DUTYSCALE_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low Power Initialization Wait Time"]
    #[inline(always)]
    #[must_use]
    pub fn lpinitwait(&mut self) -> LPINITWAIT_W<0> {
        LPINITWAIT_W::new(self)
    }
    #[doc = "Bit 11 - LN Mode Precharge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compenprchgen(&mut self) -> COMPENPRCHGEN_W<11> {
        COMPENPRCHGEN_W::new(self)
    }
    #[doc = "Bits 12:16 - Low Noise Controller Initialization Wait Time"]
    #[inline(always)]
    #[must_use]
    pub fn lnwait(&mut self) -> LNWAIT_W<12> {
        LNWAIT_W::new(self)
    }
    #[doc = "Bits 20:27 - Bypass Mode Transition From Low Power or Low Noise Modes Wait Wait"]
    #[inline(always)]
    #[must_use]
    pub fn bypwait(&mut self) -> BYPWAIT_W<20> {
        BYPWAIT_W::new(self)
    }
    #[doc = "Bits 29:30 - Select Bias Duty Cycle Clock"]
    #[inline(always)]
    #[must_use]
    pub fn dutyscale(&mut self) -> DUTYSCALE_W<29> {
        DUTYSCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC Controller Timing Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdctiming](index.html) module"]
pub struct DCDCTIMING_SPEC;
impl crate::RegisterSpec for DCDCTIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdctiming::R](R) reader structure"]
impl crate::Readable for DCDCTIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdctiming::W](W) writer structure"]
impl crate::Writable for DCDCTIMING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDCTIMING to value 0x0ff1_f8ff"]
impl crate::Resettable for DCDCTIMING_SPEC {
    const RESET_VALUE: Self::Ux = 0x0ff1_f8ff;
}
