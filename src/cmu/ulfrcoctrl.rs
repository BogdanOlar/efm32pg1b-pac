#[doc = "Register `ULFRCOCTRL` reader"]
pub type R = crate::R<ULFRCOCTRL_SPEC>;
#[doc = "Register `ULFRCOCTRL` writer"]
pub type W = crate::W<ULFRCOCTRL_SPEC>;
#[doc = "Field `TUNING` reader - ULFRCO TUNING Value"]
pub type TUNING_R = crate::FieldReader;
#[doc = "Field `TUNING` writer - ULFRCO TUNING Value"]
pub type TUNING_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `MODE` reader - ULFRCO Mode"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "ULFRCO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: ULFRCO = 1 kHz"]
    _1KHZ = 0,
    #[doc = "1: ULFRCO = 2 kHz"]
    _2KHZ = 1,
    #[doc = "2: ULFRCO = 4 kHz"]
    _4KHZ = 2,
    #[doc = "3: ULFRCO = 32 kHz"]
    _32KHZ = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::_1KHZ,
            1 => MODE_A::_2KHZ,
            2 => MODE_A::_4KHZ,
            3 => MODE_A::_32KHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "ULFRCO = 1 kHz"]
    #[inline(always)]
    pub fn is_1khz(&self) -> bool {
        *self == MODE_A::_1KHZ
    }
    #[doc = "ULFRCO = 2 kHz"]
    #[inline(always)]
    pub fn is_2khz(&self) -> bool {
        *self == MODE_A::_2KHZ
    }
    #[doc = "ULFRCO = 4 kHz"]
    #[inline(always)]
    pub fn is_4khz(&self) -> bool {
        *self == MODE_A::_4KHZ
    }
    #[doc = "ULFRCO = 32 kHz"]
    #[inline(always)]
    pub fn is_32khz(&self) -> bool {
        *self == MODE_A::_32KHZ
    }
}
#[doc = "Field `MODE` writer - ULFRCO Mode"]
pub type MODE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, MODE_A>;
impl<'a, REG, const O: u8> MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ULFRCO = 1 kHz"]
    #[inline(always)]
    pub fn _1khz(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_1KHZ)
    }
    #[doc = "ULFRCO = 2 kHz"]
    #[inline(always)]
    pub fn _2khz(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_2KHZ)
    }
    #[doc = "ULFRCO = 4 kHz"]
    #[inline(always)]
    pub fn _4khz(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_4KHZ)
    }
    #[doc = "ULFRCO = 32 kHz"]
    #[inline(always)]
    pub fn _32khz(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_32KHZ)
    }
}
#[doc = "Field `RESTRIM` reader - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)"]
pub type RESTRIM_R = crate::FieldReader;
#[doc = "Field `RESTRIM` writer - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)"]
pub type RESTRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:5 - ULFRCO TUNING Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 10:11 - ULFRCO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:17 - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)"]
    #[inline(always)]
    pub fn restrim(&self) -> RESTRIM_R {
        RESTRIM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ULFRCOCTRL")
            .field("tuning", &format_args!("{}", self.tuning().bits()))
            .field("mode", &format_args!("{}", self.mode().bits()))
            .field("restrim", &format_args!("{}", self.restrim().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<ULFRCOCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - ULFRCO TUNING Value"]
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TUNING_W<ULFRCOCTRL_SPEC, 0> {
        TUNING_W::new(self)
    }
    #[doc = "Bits 10:11 - ULFRCO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<ULFRCOCTRL_SPEC, 10> {
        MODE_W::new(self)
    }
    #[doc = "Bits 16:17 - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)"]
    #[inline(always)]
    #[must_use]
    pub fn restrim(&mut self) -> RESTRIM_W<ULFRCOCTRL_SPEC, 16> {
        RESTRIM_W::new(self)
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
#[doc = "ULFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ulfrcoctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulfrcoctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ULFRCOCTRL_SPEC;
impl crate::RegisterSpec for ULFRCOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ulfrcoctrl::R`](R) reader structure"]
impl crate::Readable for ULFRCOCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ulfrcoctrl::W`](W) writer structure"]
impl crate::Writable for ULFRCOCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ULFRCOCTRL to value 0x0002_0020"]
impl crate::Resettable for ULFRCOCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0020;
}
