#[doc = "Register `ULFRCOCTRL` reader"]
pub type R = crate::R<ULFRCOCTRLrs>;
#[doc = "Register `ULFRCOCTRL` writer"]
pub type W = crate::W<ULFRCOCTRLrs>;
#[doc = "Field `TUNING` reader - ULFRCO TUNING Value"]
pub type TuningR = crate::FieldReader;
#[doc = "Field `TUNING` writer - ULFRCO TUNING Value"]
pub type TuningW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "ULFRCO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    #[doc = "0: ULFRCO = 1 kHz"]
    _1khz = 0,
    #[doc = "1: ULFRCO = 2 kHz"]
    _2khz = 1,
    #[doc = "2: ULFRCO = 4 kHz"]
    _4khz = 2,
    #[doc = "3: ULFRCO = 32 kHz"]
    _32khz = 3,
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
#[doc = "Field `MODE` reader - ULFRCO Mode"]
pub type ModeR = crate::FieldReader<MODE>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE {
        match self.bits {
            0 => MODE::_1khz,
            1 => MODE::_2khz,
            2 => MODE::_4khz,
            3 => MODE::_32khz,
            _ => unreachable!(),
        }
    }
    #[doc = "ULFRCO = 1 kHz"]
    #[inline(always)]
    pub fn is_1khz(&self) -> bool {
        *self == MODE::_1khz
    }
    #[doc = "ULFRCO = 2 kHz"]
    #[inline(always)]
    pub fn is_2khz(&self) -> bool {
        *self == MODE::_2khz
    }
    #[doc = "ULFRCO = 4 kHz"]
    #[inline(always)]
    pub fn is_4khz(&self) -> bool {
        *self == MODE::_4khz
    }
    #[doc = "ULFRCO = 32 kHz"]
    #[inline(always)]
    pub fn is_32khz(&self) -> bool {
        *self == MODE::_32khz
    }
}
#[doc = "Field `MODE` writer - ULFRCO Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ULFRCO = 1 kHz"]
    #[inline(always)]
    pub fn _1khz(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::_1khz)
    }
    #[doc = "ULFRCO = 2 kHz"]
    #[inline(always)]
    pub fn _2khz(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::_2khz)
    }
    #[doc = "ULFRCO = 4 kHz"]
    #[inline(always)]
    pub fn _4khz(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::_4khz)
    }
    #[doc = "ULFRCO = 32 kHz"]
    #[inline(always)]
    pub fn _32khz(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::_32khz)
    }
}
#[doc = "Field `RESTRIM` reader - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)"]
pub type RestrimR = crate::FieldReader;
#[doc = "Field `RESTRIM` writer - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)"]
pub type RestrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - ULFRCO TUNING Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 10:11 - ULFRCO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:17 - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)"]
    #[inline(always)]
    pub fn restrim(&self) -> RestrimR {
        RestrimR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ULFRCO TUNING Value"]
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TuningW<ULFRCOCTRLrs> {
        TuningW::new(self, 0)
    }
    #[doc = "Bits 10:11 - ULFRCO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<ULFRCOCTRLrs> {
        ModeW::new(self, 10)
    }
    #[doc = "Bits 16:17 - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)"]
    #[inline(always)]
    #[must_use]
    pub fn restrim(&mut self) -> RestrimW<ULFRCOCTRLrs> {
        RestrimW::new(self, 16)
    }
}
#[doc = "ULFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ulfrcoctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulfrcoctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ULFRCOCTRLrs;
impl crate::RegisterSpec for ULFRCOCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ulfrcoctrl::R`](R) reader structure"]
impl crate::Readable for ULFRCOCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`ulfrcoctrl::W`](W) writer structure"]
impl crate::Writable for ULFRCOCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ULFRCOCTRL to value 0x0002_0020"]
impl crate::Resettable for ULFRCOCTRLrs {
    const RESET_VALUE: u32 = 0x0002_0020;
}
