///Register `HFXOSTEADYSTATECTRL` reader
pub type R = crate::R<HFXOSTEADYSTATECTRLrs>;
///Register `HFXOSTEADYSTATECTRL` writer
pub type W = crate::W<HFXOSTEADYSTATECTRLrs>;
///Field `IBTRIMXOCORE` reader - Sets the Steady State Oscillator Core Bias Current.
pub type IbtrimxocoreR = crate::FieldReader;
///Field `IBTRIMXOCORE` writer - Sets the Steady State Oscillator Core Bias Current.
pub type IbtrimxocoreW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `REGISH` reader - Sets the Steady State Regulator Output Current Level (shunt Regulator)
pub type RegishR = crate::FieldReader;
///Field `REGISH` writer - Sets the Steady State Regulator Output Current Level (shunt Regulator)
pub type RegishW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CTUNE` reader - Sets Oscillator Tuning Capacitance
pub type CtuneR = crate::FieldReader<u16>;
///Field `CTUNE` writer - Sets Oscillator Tuning Capacitance
pub type CtuneW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `REGSELILOW` reader - Controls Regulator Minimum Shunt Current Detection Relative to Nominal
pub type RegselilowR = crate::FieldReader;
///Field `REGSELILOW` writer - Controls Regulator Minimum Shunt Current Detection Relative to Nominal
pub type RegselilowW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PEAKDETEN` reader - Enables Oscillator Peak Detectors
pub type PeakdetenR = crate::BitReader;
///Field `PEAKDETEN` writer - Enables Oscillator Peak Detectors
pub type PeakdetenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REGISHUPPER` reader - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA
pub type RegishupperR = crate::FieldReader;
///Field `REGISHUPPER` writer - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA
pub type RegishupperW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:6 - Sets the Steady State Oscillator Core Bias Current.
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IbtrimxocoreR {
        IbtrimxocoreR::new((self.bits & 0x7f) as u8)
    }
    ///Bits 7:10 - Sets the Steady State Regulator Output Current Level (shunt Regulator)
    #[inline(always)]
    pub fn regish(&self) -> RegishR {
        RegishR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    ///Bits 11:19 - Sets Oscillator Tuning Capacitance
    #[inline(always)]
    pub fn ctune(&self) -> CtuneR {
        CtuneR::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    ///Bits 24:25 - Controls Regulator Minimum Shunt Current Detection Relative to Nominal
    #[inline(always)]
    pub fn regselilow(&self) -> RegselilowR {
        RegselilowR::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - Enables Oscillator Peak Detectors
    #[inline(always)]
    pub fn peakdeten(&self) -> PeakdetenR {
        PeakdetenR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 28:31 - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA
    #[inline(always)]
    pub fn regishupper(&self) -> RegishupperR {
        RegishupperR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFXOSTEADYSTATECTRL")
            .field("ibtrimxocore", &self.ibtrimxocore())
            .field("regish", &self.regish())
            .field("ctune", &self.ctune())
            .field("regselilow", &self.regselilow())
            .field("peakdeten", &self.peakdeten())
            .field("regishupper", &self.regishupper())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Sets the Steady State Oscillator Core Bias Current.
    #[inline(always)]
    #[must_use]
    pub fn ibtrimxocore(&mut self) -> IbtrimxocoreW<HFXOSTEADYSTATECTRLrs> {
        IbtrimxocoreW::new(self, 0)
    }
    ///Bits 7:10 - Sets the Steady State Regulator Output Current Level (shunt Regulator)
    #[inline(always)]
    #[must_use]
    pub fn regish(&mut self) -> RegishW<HFXOSTEADYSTATECTRLrs> {
        RegishW::new(self, 7)
    }
    ///Bits 11:19 - Sets Oscillator Tuning Capacitance
    #[inline(always)]
    #[must_use]
    pub fn ctune(&mut self) -> CtuneW<HFXOSTEADYSTATECTRLrs> {
        CtuneW::new(self, 11)
    }
    ///Bits 24:25 - Controls Regulator Minimum Shunt Current Detection Relative to Nominal
    #[inline(always)]
    #[must_use]
    pub fn regselilow(&mut self) -> RegselilowW<HFXOSTEADYSTATECTRLrs> {
        RegselilowW::new(self, 24)
    }
    ///Bit 26 - Enables Oscillator Peak Detectors
    #[inline(always)]
    #[must_use]
    pub fn peakdeten(&mut self) -> PeakdetenW<HFXOSTEADYSTATECTRLrs> {
        PeakdetenW::new(self, 26)
    }
    ///Bits 28:31 - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA
    #[inline(always)]
    #[must_use]
    pub fn regishupper(&mut self) -> RegishupperW<HFXOSTEADYSTATECTRLrs> {
        RegishupperW::new(self, 28)
    }
}
///HFXO Steady State Control
///
///You can [`read`](crate::Reg::read) this register and get [`hfxosteadystatectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfxosteadystatectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HFXOSTEADYSTATECTRLrs;
impl crate::RegisterSpec for HFXOSTEADYSTATECTRLrs {
    type Ux = u32;
}
///`read()` method returns [`hfxosteadystatectrl::R`](R) reader structure
impl crate::Readable for HFXOSTEADYSTATECTRLrs {}
///`write(|w| ..)` method takes [`hfxosteadystatectrl::W`](W) writer structure
impl crate::Writable for HFXOSTEADYSTATECTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HFXOSTEADYSTATECTRL to value 0xa30a_ad09
impl crate::Resettable for HFXOSTEADYSTATECTRLrs {
    const RESET_VALUE: u32 = 0xa30a_ad09;
}
