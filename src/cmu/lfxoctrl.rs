#[doc = "Register `LFXOCTRL` reader"]
pub type R = crate::R<LFXOCTRLrs>;
#[doc = "Register `LFXOCTRL` writer"]
pub type W = crate::W<LFXOCTRLrs>;
#[doc = "Field `TUNING` reader - LFXO Internal Capacitor Array Tuning Value"]
pub type TUNING_R = crate::FieldReader;
#[doc = "Field `TUNING` writer - LFXO Internal Capacitor Array Tuning Value"]
pub type TUNING_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MODE` reader - LFXO Mode"]
pub type MODE_R = crate::FieldReader<MODE>;
#[doc = "LFXO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    #[doc = "0: 32768 Hz crystal oscillator"]
    Xtal = 0,
    #[doc = "1: An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz)."]
    Bufextclk = 1,
    #[doc = "2: Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
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
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE> {
        match self.bits {
            0 => Some(MODE::Xtal),
            1 => Some(MODE::Bufextclk),
            2 => Some(MODE::Digextclk),
            _ => None,
        }
    }
    #[doc = "32768 Hz crystal oscillator"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == MODE::Xtal
    }
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz)."]
    #[inline(always)]
    pub fn is_bufextclk(&self) -> bool {
        *self == MODE::Bufextclk
    }
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline(always)]
    pub fn is_digextclk(&self) -> bool {
        *self == MODE::Digextclk
    }
}
#[doc = "Field `MODE` writer - LFXO Mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32768 Hz crystal oscillator"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Xtal)
    }
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz)."]
    #[inline(always)]
    pub fn bufextclk(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Bufextclk)
    }
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Digextclk)
    }
}
#[doc = "Field `GAIN` reader - LFXO Startup Gain"]
pub type GAIN_R = crate::FieldReader;
#[doc = "Field `GAIN` writer - LFXO Startup Gain"]
pub type GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HIGHAMPL` reader - LFXO High XTAL Oscillation Amplitude Enable"]
pub type HIGHAMPL_R = crate::BitReader;
#[doc = "Field `HIGHAMPL` writer - LFXO High XTAL Oscillation Amplitude Enable"]
pub type HIGHAMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AGC` reader - LFXO AGC Enable"]
pub type AGC_R = crate::BitReader;
#[doc = "Field `AGC` writer - LFXO AGC Enable"]
pub type AGC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUR` reader - LFXO Current Trim"]
pub type CUR_R = crate::FieldReader;
#[doc = "Field `CUR` writer - LFXO Current Trim"]
pub type CUR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BUFCUR` reader - LFXO Buffer Bias Current"]
pub type BUFCUR_R = crate::BitReader;
#[doc = "Field `BUFCUR` writer - LFXO Buffer Bias Current"]
pub type BUFCUR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - LFXO Timeout"]
pub type TIMEOUT_R = crate::FieldReader<TIMEOUT>;
#[doc = "LFXO Timeout\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEOUT {
    #[doc = "0: Timeout period of 2 cycles"]
    _2cycles = 0,
    #[doc = "1: Timeout period of 256 cycles"]
    _256cycles = 1,
    #[doc = "2: Timeout period of 1024 cycles"]
    _1kcycles = 2,
    #[doc = "3: Timeout period of 2048 cycles"]
    _2kcycles = 3,
    #[doc = "4: Timeout period of 4096 cycles"]
    _4kcycles = 4,
    #[doc = "5: Timeout period of 8192 cycles"]
    _8kcycles = 5,
    #[doc = "6: Timeout period of 16384 cycles"]
    _16kcycles = 6,
    #[doc = "7: Timeout period of 32768 cycles"]
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
impl TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == TIMEOUT::_2cycles
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == TIMEOUT::_256cycles
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == TIMEOUT::_1kcycles
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == TIMEOUT::_2kcycles
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == TIMEOUT::_4kcycles
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == TIMEOUT::_8kcycles
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == TIMEOUT::_16kcycles
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == TIMEOUT::_32kcycles
    }
}
#[doc = "Field `TIMEOUT` writer - LFXO Timeout"]
pub type TIMEOUT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, TIMEOUT>;
impl<'a, REG> TIMEOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_2cycles)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_256cycles)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_1kcycles)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_2kcycles)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_4kcycles)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_8kcycles)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_16kcycles)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_32kcycles)
    }
}
impl R {
    #[doc = "Bits 0:6 - LFXO Internal Capacitor Array Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - LFXO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 11:12 - LFXO Startup Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 14 - LFXO High XTAL Oscillation Amplitude Enable"]
    #[inline(always)]
    pub fn highampl(&self) -> HIGHAMPL_R {
        HIGHAMPL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LFXO AGC Enable"]
    #[inline(always)]
    pub fn agc(&self) -> AGC_R {
        AGC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - LFXO Current Trim"]
    #[inline(always)]
    pub fn cur(&self) -> CUR_R {
        CUR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - LFXO Buffer Bias Current"]
    #[inline(always)]
    pub fn bufcur(&self) -> BUFCUR_R {
        BUFCUR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - LFXO Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - LFXO Internal Capacitor Array Tuning Value"]
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TUNING_W<LFXOCTRLrs> {
        TUNING_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - LFXO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<LFXOCTRLrs> {
        MODE_W::new(self, 8)
    }
    #[doc = "Bits 11:12 - LFXO Startup Gain"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GAIN_W<LFXOCTRLrs> {
        GAIN_W::new(self, 11)
    }
    #[doc = "Bit 14 - LFXO High XTAL Oscillation Amplitude Enable"]
    #[inline(always)]
    #[must_use]
    pub fn highampl(&mut self) -> HIGHAMPL_W<LFXOCTRLrs> {
        HIGHAMPL_W::new(self, 14)
    }
    #[doc = "Bit 15 - LFXO AGC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn agc(&mut self) -> AGC_W<LFXOCTRLrs> {
        AGC_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - LFXO Current Trim"]
    #[inline(always)]
    #[must_use]
    pub fn cur(&mut self) -> CUR_W<LFXOCTRLrs> {
        CUR_W::new(self, 16)
    }
    #[doc = "Bit 20 - LFXO Buffer Bias Current"]
    #[inline(always)]
    #[must_use]
    pub fn bufcur(&mut self) -> BUFCUR_W<LFXOCTRLrs> {
        BUFCUR_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - LFXO Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<LFXOCTRLrs> {
        TIMEOUT_W::new(self, 24)
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
#[doc = "LFXO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfxoctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfxoctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFXOCTRLrs;
impl crate::RegisterSpec for LFXOCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfxoctrl::R`](R) reader structure"]
impl crate::Readable for LFXOCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`lfxoctrl::W`](W) writer structure"]
impl crate::Writable for LFXOCTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFXOCTRL to value 0x0700_9000"]
impl crate::Resettable for LFXOCTRLrs {
    const RESET_VALUE: u32 = 0x0700_9000;
}
