#[doc = "Register `HFXOCTRL` reader"]
pub type R = crate::R<HFXOCTRL_SPEC>;
#[doc = "Register `HFXOCTRL` writer"]
pub type W = crate::W<HFXOCTRL_SPEC>;
#[doc = "Field `MODE` reader - HFXO Mode"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - HFXO Mode"]
pub type MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEAKDETSHUNTOPTMODE` reader - HFXO Automatic Peak Detection and Shunt Current Optimization Mode"]
pub type PEAKDETSHUNTOPTMODE_R = crate::FieldReader<PEAKDETSHUNTOPTMODE_A>;
#[doc = "HFXO Automatic Peak Detection and Shunt Current Optimization Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PEAKDETSHUNTOPTMODE_A {
    #[doc = "0: Automatic control of HFXO peak detection and shunt optimization sequences. CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can also be used."]
    AUTOCMD = 0,
    #[doc = "1: CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can be used to trigger peak detection and shunt optimization sequences."]
    CMD = 1,
    #[doc = "2: CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE, REGISH, REGSELILOW, and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    MANUAL = 2,
}
impl From<PEAKDETSHUNTOPTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PEAKDETSHUNTOPTMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PEAKDETSHUNTOPTMODE_A {
    type Ux = u8;
}
impl PEAKDETSHUNTOPTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PEAKDETSHUNTOPTMODE_A> {
        match self.bits {
            0 => Some(PEAKDETSHUNTOPTMODE_A::AUTOCMD),
            1 => Some(PEAKDETSHUNTOPTMODE_A::CMD),
            2 => Some(PEAKDETSHUNTOPTMODE_A::MANUAL),
            _ => None,
        }
    }
    #[doc = "Automatic control of HFXO peak detection and shunt optimization sequences. CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can also be used."]
    #[inline(always)]
    pub fn is_autocmd(&self) -> bool {
        *self == PEAKDETSHUNTOPTMODE_A::AUTOCMD
    }
    #[doc = "CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can be used to trigger peak detection and shunt optimization sequences."]
    #[inline(always)]
    pub fn is_cmd(&self) -> bool {
        *self == PEAKDETSHUNTOPTMODE_A::CMD
    }
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE, REGISH, REGSELILOW, and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == PEAKDETSHUNTOPTMODE_A::MANUAL
    }
}
#[doc = "Field `PEAKDETSHUNTOPTMODE` writer - HFXO Automatic Peak Detection and Shunt Current Optimization Mode"]
pub type PEAKDETSHUNTOPTMODE_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, PEAKDETSHUNTOPTMODE_A>;
impl<'a, REG, const O: u8> PEAKDETSHUNTOPTMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Automatic control of HFXO peak detection and shunt optimization sequences. CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can also be used."]
    #[inline(always)]
    pub fn autocmd(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETSHUNTOPTMODE_A::AUTOCMD)
    }
    #[doc = "CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can be used to trigger peak detection and shunt optimization sequences."]
    #[inline(always)]
    pub fn cmd(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETSHUNTOPTMODE_A::CMD)
    }
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE, REGISH, REGSELILOW, and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETSHUNTOPTMODE_A::MANUAL)
    }
}
#[doc = "Field `LOWPOWER` reader - Low Power Mode Control"]
pub type LOWPOWER_R = crate::BitReader;
#[doc = "Field `LOWPOWER` writer - Low Power Mode Control"]
pub type LOWPOWER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XTI2GND` reader - Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off"]
pub type XTI2GND_R = crate::BitReader;
#[doc = "Field `XTI2GND` writer - Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off"]
pub type XTI2GND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XTO2GND` reader - Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off"]
pub type XTO2GND_R = crate::BitReader;
#[doc = "Field `XTO2GND` writer - Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off"]
pub type XTO2GND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LFTIMEOUT` reader - HFXO Low Frequency Timeout"]
pub type LFTIMEOUT_R = crate::FieldReader<LFTIMEOUT_A>;
#[doc = "HFXO Low Frequency Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFTIMEOUT_A {
    #[doc = "0: Timeout period of 0 cycles (disabled)"]
    _0CYCLES = 0,
    #[doc = "1: Timeout period of 2 cycles"]
    _2CYCLES = 1,
    #[doc = "2: Timeout period of 4 cycles"]
    _4CYCLES = 2,
    #[doc = "3: Timeout period of 16 cycles"]
    _16CYCLES = 3,
    #[doc = "4: Timeout period of 32 cycles"]
    _32CYCLES = 4,
    #[doc = "5: Timeout period of 64 cycles"]
    _64CYCLES = 5,
    #[doc = "6: Timeout period of 1024 cycles"]
    _1KCYCLES = 6,
    #[doc = "7: Timeout period of 4096 cycles"]
    _4KCYCLES = 7,
}
impl From<LFTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: LFTIMEOUT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFTIMEOUT_A {
    type Ux = u8;
}
impl LFTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LFTIMEOUT_A {
        match self.bits {
            0 => LFTIMEOUT_A::_0CYCLES,
            1 => LFTIMEOUT_A::_2CYCLES,
            2 => LFTIMEOUT_A::_4CYCLES,
            3 => LFTIMEOUT_A::_16CYCLES,
            4 => LFTIMEOUT_A::_32CYCLES,
            5 => LFTIMEOUT_A::_64CYCLES,
            6 => LFTIMEOUT_A::_1KCYCLES,
            7 => LFTIMEOUT_A::_4KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Timeout period of 0 cycles (disabled)"]
    #[inline(always)]
    pub fn is_0cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_0CYCLES
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_2CYCLES
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_4CYCLES
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_16CYCLES
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_32CYCLES
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_64CYCLES
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == LFTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == LFTIMEOUT_A::_4KCYCLES
    }
}
#[doc = "Field `LFTIMEOUT` writer - HFXO Low Frequency Timeout"]
pub type LFTIMEOUT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, LFTIMEOUT_A>;
impl<'a, REG, const O: u8> LFTIMEOUT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout period of 0 cycles (disabled)"]
    #[inline(always)]
    pub fn _0cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT_A::_0CYCLES)
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT_A::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT_A::_32CYCLES)
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT_A::_64CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT_A::_4KCYCLES)
    }
}
#[doc = "Field `AUTOSTARTEM0EM1` reader - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type AUTOSTARTEM0EM1_R = crate::BitReader;
#[doc = "Field `AUTOSTARTEM0EM1` writer - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type AUTOSTARTEM0EM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUTOSTARTSELEM0EM1` reader - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type AUTOSTARTSELEM0EM1_R = crate::BitReader;
#[doc = "Field `AUTOSTARTSELEM0EM1` writer - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type AUTOSTARTSELEM0EM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - HFXO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection and Shunt Current Optimization Mode"]
    #[inline(always)]
    pub fn peakdetshuntoptmode(&self) -> PEAKDETSHUNTOPTMODE_R {
        PEAKDETSHUNTOPTMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Low Power Mode Control"]
    #[inline(always)]
    pub fn lowpower(&self) -> LOWPOWER_R {
        LOWPOWER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off"]
    #[inline(always)]
    pub fn xti2gnd(&self) -> XTI2GND_R {
        XTI2GND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off"]
    #[inline(always)]
    pub fn xto2gnd(&self) -> XTO2GND_R {
        XTO2GND_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline(always)]
    pub fn lftimeout(&self) -> LFTIMEOUT_R {
        LFTIMEOUT_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartem0em1(&self) -> AUTOSTARTEM0EM1_R {
        AUTOSTARTEM0EM1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartselem0em1(&self) -> AUTOSTARTSELEM0EM1_R {
        AUTOSTARTSELEM0EM1_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFXOCTRL")
            .field("mode", &format_args!("{}", self.mode().bit()))
            .field(
                "peakdetshuntoptmode",
                &format_args!("{}", self.peakdetshuntoptmode().bits()),
            )
            .field("lowpower", &format_args!("{}", self.lowpower().bit()))
            .field("xti2gnd", &format_args!("{}", self.xti2gnd().bit()))
            .field("xto2gnd", &format_args!("{}", self.xto2gnd().bit()))
            .field("lftimeout", &format_args!("{}", self.lftimeout().bits()))
            .field(
                "autostartem0em1",
                &format_args!("{}", self.autostartem0em1().bit()),
            )
            .field(
                "autostartselem0em1",
                &format_args!("{}", self.autostartselem0em1().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HFXOCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - HFXO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<HFXOCTRL_SPEC, 0> {
        MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection and Shunt Current Optimization Mode"]
    #[inline(always)]
    #[must_use]
    pub fn peakdetshuntoptmode(&mut self) -> PEAKDETSHUNTOPTMODE_W<HFXOCTRL_SPEC, 4> {
        PEAKDETSHUNTOPTMODE_W::new(self)
    }
    #[doc = "Bit 8 - Low Power Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn lowpower(&mut self) -> LOWPOWER_W<HFXOCTRL_SPEC, 8> {
        LOWPOWER_W::new(self)
    }
    #[doc = "Bit 9 - Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off"]
    #[inline(always)]
    #[must_use]
    pub fn xti2gnd(&mut self) -> XTI2GND_W<HFXOCTRL_SPEC, 9> {
        XTI2GND_W::new(self)
    }
    #[doc = "Bit 10 - Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off"]
    #[inline(always)]
    #[must_use]
    pub fn xto2gnd(&mut self) -> XTO2GND_W<HFXOCTRL_SPEC, 10> {
        XTO2GND_W::new(self)
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn lftimeout(&mut self) -> LFTIMEOUT_W<HFXOCTRL_SPEC, 24> {
        LFTIMEOUT_W::new(self)
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    #[must_use]
    pub fn autostartem0em1(&mut self) -> AUTOSTARTEM0EM1_W<HFXOCTRL_SPEC, 28> {
        AUTOSTARTEM0EM1_W::new(self)
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    #[must_use]
    pub fn autostartselem0em1(&mut self) -> AUTOSTARTSELEM0EM1_W<HFXOCTRL_SPEC, 29> {
        AUTOSTARTSELEM0EM1_W::new(self)
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
#[doc = "HFXO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxoctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxoctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFXOCTRL_SPEC;
impl crate::RegisterSpec for HFXOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxoctrl::R`](R) reader structure"]
impl crate::Readable for HFXOCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfxoctrl::W`](W) writer structure"]
impl crate::Writable for HFXOCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFXOCTRL to value 0"]
impl crate::Resettable for HFXOCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
