///Register `LFXOCTRL` reader
pub type R = crate::R<LFXOCTRLrs>;
///Register `LFXOCTRL` writer
pub type W = crate::W<LFXOCTRLrs>;
///Field `TUNING` reader - LFXO Internal Capacitor Array Tuning Value
pub type TuningR = crate::FieldReader;
///Field `TUNING` writer - LFXO Internal Capacitor Array Tuning Value
pub type TuningW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///LFXO Mode
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    ///0: 32768 Hz crystal oscillator
    Xtal = 0,
    ///1: An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz).
    Bufextclk = 1,
    ///2: Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed.
    Digextclk = 2,
}
impl From<MODE> for u8 {
    #[inline(always)]
    fn from(variant: MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE {
    type Ux = u8;
}
impl crate::IsEnum for MODE {}
///Field `MODE` reader - LFXO Mode
pub type ModeR = crate::FieldReader<MODE>;
impl ModeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE> {
        match self.bits {
            0 => Some(MODE::Xtal),
            1 => Some(MODE::Bufextclk),
            2 => Some(MODE::Digextclk),
            _ => None,
        }
    }
    ///32768 Hz crystal oscillator
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == MODE::Xtal
    }
    ///An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz).
    #[inline(always)]
    pub fn is_bufextclk(&self) -> bool {
        *self == MODE::Bufextclk
    }
    ///Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed.
    #[inline(always)]
    pub fn is_digextclk(&self) -> bool {
        *self == MODE::Digextclk
    }
}
///Field `MODE` writer - LFXO Mode
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///32768 Hz crystal oscillator
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Xtal)
    }
    ///An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz).
    #[inline(always)]
    pub fn bufextclk(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Bufextclk)
    }
    ///Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed.
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Digextclk)
    }
}
///Field `GAIN` reader - LFXO Startup Gain
pub type GainR = crate::FieldReader;
///Field `GAIN` writer - LFXO Startup Gain
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HIGHAMPL` reader - LFXO High XTAL Oscillation Amplitude Enable
pub type HighamplR = crate::BitReader;
///Field `HIGHAMPL` writer - LFXO High XTAL Oscillation Amplitude Enable
pub type HighamplW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AGC` reader - LFXO AGC Enable
pub type AgcR = crate::BitReader;
///Field `AGC` writer - LFXO AGC Enable
pub type AgcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUR` reader - LFXO Current Trim
pub type CurR = crate::FieldReader;
///Field `CUR` writer - LFXO Current Trim
pub type CurW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BUFCUR` reader - LFXO Buffer Bias Current
pub type BufcurR = crate::BitReader;
///Field `BUFCUR` writer - LFXO Buffer Bias Current
pub type BufcurW<'a, REG> = crate::BitWriter<'a, REG>;
///LFXO Timeout
///
///Value on reset: 7
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEOUT {
    ///0: Timeout period of 2 cycles
    _2cycles = 0,
    ///1: Timeout period of 256 cycles
    _256cycles = 1,
    ///2: Timeout period of 1024 cycles
    _1kcycles = 2,
    ///3: Timeout period of 2048 cycles
    _2kcycles = 3,
    ///4: Timeout period of 4096 cycles
    _4kcycles = 4,
    ///5: Timeout period of 8192 cycles
    _8kcycles = 5,
    ///6: Timeout period of 16384 cycles
    _16kcycles = 6,
    ///7: Timeout period of 32768 cycles
    _32kcycles = 7,
}
impl From<TIMEOUT> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMEOUT {
    type Ux = u8;
}
impl crate::IsEnum for TIMEOUT {}
///Field `TIMEOUT` reader - LFXO Timeout
pub type TimeoutR = crate::FieldReader<TIMEOUT>;
impl TimeoutR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIMEOUT {
        match self.bits {
            0 => TIMEOUT::_2cycles,
            1 => TIMEOUT::_256cycles,
            2 => TIMEOUT::_1kcycles,
            3 => TIMEOUT::_2kcycles,
            4 => TIMEOUT::_4kcycles,
            5 => TIMEOUT::_8kcycles,
            6 => TIMEOUT::_16kcycles,
            7 => TIMEOUT::_32kcycles,
            _ => unreachable!(),
        }
    }
    ///Timeout period of 2 cycles
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == TIMEOUT::_2cycles
    }
    ///Timeout period of 256 cycles
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == TIMEOUT::_256cycles
    }
    ///Timeout period of 1024 cycles
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == TIMEOUT::_1kcycles
    }
    ///Timeout period of 2048 cycles
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == TIMEOUT::_2kcycles
    }
    ///Timeout period of 4096 cycles
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == TIMEOUT::_4kcycles
    }
    ///Timeout period of 8192 cycles
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == TIMEOUT::_8kcycles
    }
    ///Timeout period of 16384 cycles
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == TIMEOUT::_16kcycles
    }
    ///Timeout period of 32768 cycles
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == TIMEOUT::_32kcycles
    }
}
///Field `TIMEOUT` writer - LFXO Timeout
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 3, TIMEOUT, crate::Safe>;
impl<'a, REG> TimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timeout period of 2 cycles
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_2cycles)
    }
    ///Timeout period of 256 cycles
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_256cycles)
    }
    ///Timeout period of 1024 cycles
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_1kcycles)
    }
    ///Timeout period of 2048 cycles
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_2kcycles)
    }
    ///Timeout period of 4096 cycles
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_4kcycles)
    }
    ///Timeout period of 8192 cycles
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_8kcycles)
    }
    ///Timeout period of 16384 cycles
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_16kcycles)
    }
    ///Timeout period of 32768 cycles
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_32kcycles)
    }
}
impl R {
    ///Bits 0:6 - LFXO Internal Capacitor Array Tuning Value
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:9 - LFXO Mode
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 11:12 - LFXO Startup Gain
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 14 - LFXO High XTAL Oscillation Amplitude Enable
    #[inline(always)]
    pub fn highampl(&self) -> HighamplR {
        HighamplR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - LFXO AGC Enable
    #[inline(always)]
    pub fn agc(&self) -> AgcR {
        AgcR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - LFXO Current Trim
    #[inline(always)]
    pub fn cur(&self) -> CurR {
        CurR::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 20 - LFXO Buffer Bias Current
    #[inline(always)]
    pub fn bufcur(&self) -> BufcurR {
        BufcurR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 24:26 - LFXO Timeout
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFXOCTRL")
            .field("tuning", &self.tuning())
            .field("mode", &self.mode())
            .field("gain", &self.gain())
            .field("highampl", &self.highampl())
            .field("agc", &self.agc())
            .field("cur", &self.cur())
            .field("bufcur", &self.bufcur())
            .field("timeout", &self.timeout())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - LFXO Internal Capacitor Array Tuning Value
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TuningW<LFXOCTRLrs> {
        TuningW::new(self, 0)
    }
    ///Bits 8:9 - LFXO Mode
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<LFXOCTRLrs> {
        ModeW::new(self, 8)
    }
    ///Bits 11:12 - LFXO Startup Gain
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GainW<LFXOCTRLrs> {
        GainW::new(self, 11)
    }
    ///Bit 14 - LFXO High XTAL Oscillation Amplitude Enable
    #[inline(always)]
    #[must_use]
    pub fn highampl(&mut self) -> HighamplW<LFXOCTRLrs> {
        HighamplW::new(self, 14)
    }
    ///Bit 15 - LFXO AGC Enable
    #[inline(always)]
    #[must_use]
    pub fn agc(&mut self) -> AgcW<LFXOCTRLrs> {
        AgcW::new(self, 15)
    }
    ///Bits 16:17 - LFXO Current Trim
    #[inline(always)]
    #[must_use]
    pub fn cur(&mut self) -> CurW<LFXOCTRLrs> {
        CurW::new(self, 16)
    }
    ///Bit 20 - LFXO Buffer Bias Current
    #[inline(always)]
    #[must_use]
    pub fn bufcur(&mut self) -> BufcurW<LFXOCTRLrs> {
        BufcurW::new(self, 20)
    }
    ///Bits 24:26 - LFXO Timeout
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<LFXOCTRLrs> {
        TimeoutW::new(self, 24)
    }
}
///LFXO Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`lfxoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfxoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LFXOCTRLrs;
impl crate::RegisterSpec for LFXOCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`lfxoctrl::R`](R) reader structure
impl crate::Readable for LFXOCTRLrs {}
///`write(|w| ..)` method takes [`lfxoctrl::W`](W) writer structure
impl crate::Writable for LFXOCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LFXOCTRL to value 0x0700_9000
impl crate::Resettable for LFXOCTRLrs {
    const RESET_VALUE: u32 = 0x0700_9000;
}
