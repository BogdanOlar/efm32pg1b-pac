#[doc = "Register `HFXOSTEADYSTATECTRL` reader"]
pub type R = crate::R<HFXOSTEADYSTATECTRL_SPEC>;
#[doc = "Register `HFXOSTEADYSTATECTRL` writer"]
pub type W = crate::W<HFXOSTEADYSTATECTRL_SPEC>;
#[doc = "Field `IBTRIMXOCORE` reader - Sets the Steady State Oscillator Core Bias Current."]
pub type IBTRIMXOCORE_R = crate::FieldReader;
#[doc = "Field `IBTRIMXOCORE` writer - Sets the Steady State Oscillator Core Bias Current."]
pub type IBTRIMXOCORE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `REGISH` reader - Sets the Steady State Regulator Output Current Level (shunt Regulator)"]
pub type REGISH_R = crate::FieldReader;
#[doc = "Field `REGISH` writer - Sets the Steady State Regulator Output Current Level (shunt Regulator)"]
pub type REGISH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CTUNE` reader - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_R = crate::FieldReader<u16>;
#[doc = "Field `CTUNE` writer - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `REGSELILOW` reader - Controls Regulator Minimum Shunt Current Detection Relative to Nominal"]
pub type REGSELILOW_R = crate::FieldReader;
#[doc = "Field `REGSELILOW` writer - Controls Regulator Minimum Shunt Current Detection Relative to Nominal"]
pub type REGSELILOW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PEAKDETEN` reader - Enables Oscillator Peak Detectors"]
pub type PEAKDETEN_R = crate::BitReader;
#[doc = "Field `PEAKDETEN` writer - Enables Oscillator Peak Detectors"]
pub type PEAKDETEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REGISHUPPER` reader - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA"]
pub type REGISHUPPER_R = crate::FieldReader;
#[doc = "Field `REGISHUPPER` writer - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA"]
pub type REGISHUPPER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:6 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORE_R {
        IBTRIMXOCORE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:10 - Sets the Steady State Regulator Output Current Level (shunt Regulator)"]
    #[inline(always)]
    pub fn regish(&self) -> REGISH_R {
        REGISH_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&self) -> CTUNE_R {
        CTUNE_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:25 - Controls Regulator Minimum Shunt Current Detection Relative to Nominal"]
    #[inline(always)]
    pub fn regselilow(&self) -> REGSELILOW_R {
        REGSELILOW_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline(always)]
    pub fn peakdeten(&self) -> PEAKDETEN_R {
        PEAKDETEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA"]
    #[inline(always)]
    pub fn regishupper(&self) -> REGISHUPPER_R {
        REGISHUPPER_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFXOSTEADYSTATECTRL")
            .field(
                "ibtrimxocore",
                &format_args!("{}", self.ibtrimxocore().bits()),
            )
            .field("regish", &format_args!("{}", self.regish().bits()))
            .field("ctune", &format_args!("{}", self.ctune().bits()))
            .field("regselilow", &format_args!("{}", self.regselilow().bits()))
            .field("peakdeten", &format_args!("{}", self.peakdeten().bit()))
            .field(
                "regishupper",
                &format_args!("{}", self.regishupper().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HFXOSTEADYSTATECTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    #[must_use]
    pub fn ibtrimxocore(&mut self) -> IBTRIMXOCORE_W<HFXOSTEADYSTATECTRL_SPEC, 0> {
        IBTRIMXOCORE_W::new(self)
    }
    #[doc = "Bits 7:10 - Sets the Steady State Regulator Output Current Level (shunt Regulator)"]
    #[inline(always)]
    #[must_use]
    pub fn regish(&mut self) -> REGISH_W<HFXOSTEADYSTATECTRL_SPEC, 7> {
        REGISH_W::new(self)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    #[must_use]
    pub fn ctune(&mut self) -> CTUNE_W<HFXOSTEADYSTATECTRL_SPEC, 11> {
        CTUNE_W::new(self)
    }
    #[doc = "Bits 24:25 - Controls Regulator Minimum Shunt Current Detection Relative to Nominal"]
    #[inline(always)]
    #[must_use]
    pub fn regselilow(&mut self) -> REGSELILOW_W<HFXOSTEADYSTATECTRL_SPEC, 24> {
        REGSELILOW_W::new(self)
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline(always)]
    #[must_use]
    pub fn peakdeten(&mut self) -> PEAKDETEN_W<HFXOSTEADYSTATECTRL_SPEC, 26> {
        PEAKDETEN_W::new(self)
    }
    #[doc = "Bits 28:31 - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA"]
    #[inline(always)]
    #[must_use]
    pub fn regishupper(&mut self) -> REGISHUPPER_W<HFXOSTEADYSTATECTRL_SPEC, 28> {
        REGISHUPPER_W::new(self)
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
#[doc = "HFXO Steady State Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxosteadystatectrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxosteadystatectrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFXOSTEADYSTATECTRL_SPEC;
impl crate::RegisterSpec for HFXOSTEADYSTATECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxosteadystatectrl::R`](R) reader structure"]
impl crate::Readable for HFXOSTEADYSTATECTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfxosteadystatectrl::W`](W) writer structure"]
impl crate::Writable for HFXOSTEADYSTATECTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFXOSTEADYSTATECTRL to value 0xa30a_ad09"]
impl crate::Resettable for HFXOSTEADYSTATECTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xa30a_ad09;
}
