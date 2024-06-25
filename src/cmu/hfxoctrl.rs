#[doc = "Register `HFXOCTRL` reader"]
pub type R = crate::R<HFXOCTRLrs>;
#[doc = "Register `HFXOCTRL` writer"]
pub type W = crate::W<HFXOCTRLrs>;
#[doc = "Field `MODE` reader - HFXO Mode"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - HFXO Mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "HFXO Automatic Peak Detection and Shunt Current Optimization Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PEAKDETSHUNTOPTMODE {
    #[doc = "0: Automatic control of HFXO peak detection and shunt optimization sequences. CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can also be used."]
    Autocmd = 0,
    #[doc = "1: CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can be used to trigger peak detection and shunt optimization sequences."]
    Cmd = 1,
    #[doc = "2: CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE, REGISH, REGSELILOW, and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    Manual = 2,
}
impl From<PEAKDETSHUNTOPTMODE> for u8 {
    #[inline(always)]
    fn from(variant: PEAKDETSHUNTOPTMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PEAKDETSHUNTOPTMODE {
    type Ux = u8;
}
impl crate::IsEnum for PEAKDETSHUNTOPTMODE {}
#[doc = "Field `PEAKDETSHUNTOPTMODE` reader - HFXO Automatic Peak Detection and Shunt Current Optimization Mode"]
pub type PeakdetshuntoptmodeR = crate::FieldReader<PEAKDETSHUNTOPTMODE>;
impl PeakdetshuntoptmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PEAKDETSHUNTOPTMODE> {
        match self.bits {
            0 => Some(PEAKDETSHUNTOPTMODE::Autocmd),
            1 => Some(PEAKDETSHUNTOPTMODE::Cmd),
            2 => Some(PEAKDETSHUNTOPTMODE::Manual),
            _ => None,
        }
    }
    #[doc = "Automatic control of HFXO peak detection and shunt optimization sequences. CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can also be used."]
    #[inline(always)]
    pub fn is_autocmd(&self) -> bool {
        *self == PEAKDETSHUNTOPTMODE::Autocmd
    }
    #[doc = "CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can be used to trigger peak detection and shunt optimization sequences."]
    #[inline(always)]
    pub fn is_cmd(&self) -> bool {
        *self == PEAKDETSHUNTOPTMODE::Cmd
    }
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE, REGISH, REGSELILOW, and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == PEAKDETSHUNTOPTMODE::Manual
    }
}
#[doc = "Field `PEAKDETSHUNTOPTMODE` writer - HFXO Automatic Peak Detection and Shunt Current Optimization Mode"]
pub type PeakdetshuntoptmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, PEAKDETSHUNTOPTMODE>;
impl<'a, REG> PeakdetshuntoptmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Automatic control of HFXO peak detection and shunt optimization sequences. CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can also be used."]
    #[inline(always)]
    pub fn autocmd(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETSHUNTOPTMODE::Autocmd)
    }
    #[doc = "CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can be used to trigger peak detection and shunt optimization sequences."]
    #[inline(always)]
    pub fn cmd(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETSHUNTOPTMODE::Cmd)
    }
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE, REGISH, REGSELILOW, and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETSHUNTOPTMODE::Manual)
    }
}
#[doc = "Field `LOWPOWER` reader - Low Power Mode Control"]
pub type LowpowerR = crate::BitReader;
#[doc = "Field `LOWPOWER` writer - Low Power Mode Control"]
pub type LowpowerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTI2GND` reader - Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off"]
pub type Xti2gndR = crate::BitReader;
#[doc = "Field `XTI2GND` writer - Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off"]
pub type Xti2gndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTO2GND` reader - Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off"]
pub type Xto2gndR = crate::BitReader;
#[doc = "Field `XTO2GND` writer - Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off"]
pub type Xto2gndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "HFXO Low Frequency Timeout\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFTIMEOUT {
    #[doc = "0: Timeout period of 0 cycles (disabled)"]
    _0cycles = 0,
    #[doc = "1: Timeout period of 2 cycles"]
    _2cycles = 1,
    #[doc = "2: Timeout period of 4 cycles"]
    _4cycles = 2,
    #[doc = "3: Timeout period of 16 cycles"]
    _16cycles = 3,
    #[doc = "4: Timeout period of 32 cycles"]
    _32cycles = 4,
    #[doc = "5: Timeout period of 64 cycles"]
    _64cycles = 5,
    #[doc = "6: Timeout period of 1024 cycles"]
    _1kcycles = 6,
    #[doc = "7: Timeout period of 4096 cycles"]
    _4kcycles = 7,
}
impl From<LFTIMEOUT> for u8 {
    #[inline(always)]
    fn from(variant: LFTIMEOUT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFTIMEOUT {
    type Ux = u8;
}
impl crate::IsEnum for LFTIMEOUT {}
#[doc = "Field `LFTIMEOUT` reader - HFXO Low Frequency Timeout"]
pub type LftimeoutR = crate::FieldReader<LFTIMEOUT>;
impl LftimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LFTIMEOUT {
        match self.bits {
            0 => LFTIMEOUT::_0cycles,
            1 => LFTIMEOUT::_2cycles,
            2 => LFTIMEOUT::_4cycles,
            3 => LFTIMEOUT::_16cycles,
            4 => LFTIMEOUT::_32cycles,
            5 => LFTIMEOUT::_64cycles,
            6 => LFTIMEOUT::_1kcycles,
            7 => LFTIMEOUT::_4kcycles,
            _ => unreachable!(),
        }
    }
    #[doc = "Timeout period of 0 cycles (disabled)"]
    #[inline(always)]
    pub fn is_0cycles(&self) -> bool {
        *self == LFTIMEOUT::_0cycles
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == LFTIMEOUT::_2cycles
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == LFTIMEOUT::_4cycles
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == LFTIMEOUT::_16cycles
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == LFTIMEOUT::_32cycles
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == LFTIMEOUT::_64cycles
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == LFTIMEOUT::_1kcycles
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == LFTIMEOUT::_4kcycles
    }
}
#[doc = "Field `LFTIMEOUT` writer - HFXO Low Frequency Timeout"]
pub type LftimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 3, LFTIMEOUT, crate::Safe>;
impl<'a, REG> LftimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout period of 0 cycles (disabled)"]
    #[inline(always)]
    pub fn _0cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT::_0cycles)
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT::_2cycles)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT::_4cycles)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT::_16cycles)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT::_32cycles)
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT::_64cycles)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT::_1kcycles)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT::_4kcycles)
    }
}
#[doc = "Field `AUTOSTARTEM0EM1` reader - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type Autostartem0em1R = crate::BitReader;
#[doc = "Field `AUTOSTARTEM0EM1` writer - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type Autostartem0em1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOSTARTSELEM0EM1` reader - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type Autostartselem0em1R = crate::BitReader;
#[doc = "Field `AUTOSTARTSELEM0EM1` writer - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type Autostartselem0em1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HFXO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection and Shunt Current Optimization Mode"]
    #[inline(always)]
    pub fn peakdetshuntoptmode(&self) -> PeakdetshuntoptmodeR {
        PeakdetshuntoptmodeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Low Power Mode Control"]
    #[inline(always)]
    pub fn lowpower(&self) -> LowpowerR {
        LowpowerR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off"]
    #[inline(always)]
    pub fn xti2gnd(&self) -> Xti2gndR {
        Xti2gndR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off"]
    #[inline(always)]
    pub fn xto2gnd(&self) -> Xto2gndR {
        Xto2gndR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline(always)]
    pub fn lftimeout(&self) -> LftimeoutR {
        LftimeoutR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartem0em1(&self) -> Autostartem0em1R {
        Autostartem0em1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartselem0em1(&self) -> Autostartselem0em1R {
        Autostartselem0em1R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFXOCTRL")
            .field("mode", &self.mode())
            .field("peakdetshuntoptmode", &self.peakdetshuntoptmode())
            .field("lowpower", &self.lowpower())
            .field("xti2gnd", &self.xti2gnd())
            .field("xto2gnd", &self.xto2gnd())
            .field("lftimeout", &self.lftimeout())
            .field("autostartem0em1", &self.autostartem0em1())
            .field("autostartselem0em1", &self.autostartselem0em1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - HFXO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<HFXOCTRLrs> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection and Shunt Current Optimization Mode"]
    #[inline(always)]
    #[must_use]
    pub fn peakdetshuntoptmode(&mut self) -> PeakdetshuntoptmodeW<HFXOCTRLrs> {
        PeakdetshuntoptmodeW::new(self, 4)
    }
    #[doc = "Bit 8 - Low Power Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn lowpower(&mut self) -> LowpowerW<HFXOCTRLrs> {
        LowpowerW::new(self, 8)
    }
    #[doc = "Bit 9 - Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off"]
    #[inline(always)]
    #[must_use]
    pub fn xti2gnd(&mut self) -> Xti2gndW<HFXOCTRLrs> {
        Xti2gndW::new(self, 9)
    }
    #[doc = "Bit 10 - Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off"]
    #[inline(always)]
    #[must_use]
    pub fn xto2gnd(&mut self) -> Xto2gndW<HFXOCTRLrs> {
        Xto2gndW::new(self, 10)
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn lftimeout(&mut self) -> LftimeoutW<HFXOCTRLrs> {
        LftimeoutW::new(self, 24)
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    #[must_use]
    pub fn autostartem0em1(&mut self) -> Autostartem0em1W<HFXOCTRLrs> {
        Autostartem0em1W::new(self, 28)
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    #[must_use]
    pub fn autostartselem0em1(&mut self) -> Autostartselem0em1W<HFXOCTRLrs> {
        Autostartselem0em1W::new(self, 29)
    }
}
#[doc = "HFXO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfxoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfxoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFXOCTRLrs;
impl crate::RegisterSpec for HFXOCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxoctrl::R`](R) reader structure"]
impl crate::Readable for HFXOCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`hfxoctrl::W`](W) writer structure"]
impl crate::Writable for HFXOCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFXOCTRL to value 0"]
impl crate::Resettable for HFXOCTRLrs {
    const RESET_VALUE: u32 = 0;
}
