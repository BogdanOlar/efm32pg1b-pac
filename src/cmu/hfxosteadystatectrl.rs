#[doc = "Register `HFXOSTEADYSTATECTRL` reader"]
pub type R = crate::R<HFXOSTEADYSTATECTRLrs>;
#[doc = "Register `HFXOSTEADYSTATECTRL` writer"]
pub type W = crate::W<HFXOSTEADYSTATECTRLrs>;
#[doc = "Field `IBTRIMXOCORE` reader - Sets the Steady State Oscillator Core Bias Current."]
pub type IBTRIMXOCORE_R = crate::FieldReader;
#[doc = "Field `IBTRIMXOCORE` writer - Sets the Steady State Oscillator Core Bias Current."]
pub type IBTRIMXOCORE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `REGISH` reader - Sets the Steady State Regulator Output Current Level (shunt Regulator)"]
pub type REGISH_R = crate::FieldReader;
#[doc = "Field `REGISH` writer - Sets the Steady State Regulator Output Current Level (shunt Regulator)"]
pub type REGISH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CTUNE` reader - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_R = crate::FieldReader<u16>;
#[doc = "Field `CTUNE` writer - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `REGSELILOW` reader - Controls Regulator Minimum Shunt Current Detection Relative to Nominal"]
pub type REGSELILOW_R = crate::FieldReader;
#[doc = "Field `REGSELILOW` writer - Controls Regulator Minimum Shunt Current Detection Relative to Nominal"]
pub type REGSELILOW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PEAKDETEN` reader - Enables Oscillator Peak Detectors"]
pub type PEAKDETEN_R = crate::BitReader;
#[doc = "Field `PEAKDETEN` writer - Enables Oscillator Peak Detectors"]
pub type PEAKDETEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGISHUPPER` reader - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA"]
pub type REGISHUPPER_R = crate::FieldReader;
#[doc = "Field `REGISHUPPER` writer - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA"]
pub type REGISHUPPER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
impl W {
    #[doc = "Bits 0:6 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    #[must_use]
    pub fn ibtrimxocore(&mut self) -> IBTRIMXOCORE_W<HFXOSTEADYSTATECTRLrs> {
        IBTRIMXOCORE_W::new(self, 0)
    }
    #[doc = "Bits 7:10 - Sets the Steady State Regulator Output Current Level (shunt Regulator)"]
    #[inline(always)]
    #[must_use]
    pub fn regish(&mut self) -> REGISH_W<HFXOSTEADYSTATECTRLrs> {
        REGISH_W::new(self, 7)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    #[must_use]
    pub fn ctune(&mut self) -> CTUNE_W<HFXOSTEADYSTATECTRLrs> {
        CTUNE_W::new(self, 11)
    }
    #[doc = "Bits 24:25 - Controls Regulator Minimum Shunt Current Detection Relative to Nominal"]
    #[inline(always)]
    #[must_use]
    pub fn regselilow(&mut self) -> REGSELILOW_W<HFXOSTEADYSTATECTRLrs> {
        REGSELILOW_W::new(self, 24)
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline(always)]
    #[must_use]
    pub fn peakdeten(&mut self) -> PEAKDETEN_W<HFXOSTEADYSTATECTRLrs> {
        PEAKDETEN_W::new(self, 26)
    }
    #[doc = "Bits 28:31 - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA"]
    #[inline(always)]
    #[must_use]
    pub fn regishupper(&mut self) -> REGISHUPPER_W<HFXOSTEADYSTATECTRLrs> {
        REGISHUPPER_W::new(self, 28)
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
pub struct HFXOSTEADYSTATECTRLrs;
impl crate::RegisterSpec for HFXOSTEADYSTATECTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxosteadystatectrl::R`](R) reader structure"]
impl crate::Readable for HFXOSTEADYSTATECTRLrs {}
#[doc = "`write(|w| ..)` method takes [`hfxosteadystatectrl::W`](W) writer structure"]
impl crate::Writable for HFXOSTEADYSTATECTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFXOSTEADYSTATECTRL to value 0xa30a_ad09"]
impl crate::Resettable for HFXOSTEADYSTATECTRLrs {
    const RESET_VALUE: u32 = 0xa30a_ad09;
}
