///Register `ULFRCOCTRL` reader
pub type R = crate::R<ULFRCOCTRLrs>;
///Register `ULFRCOCTRL` writer
pub type W = crate::W<ULFRCOCTRLrs>;
///Field `TUNING` reader - ULFRCO TUNING Value
pub type TuningR = crate::FieldReader;
///Field `TUNING` writer - ULFRCO TUNING Value
pub type TuningW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///ULFRCO Mode
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    ///0: ULFRCO = 1 kHz
    _1khz = 0,
    ///1: ULFRCO = 2 kHz
    _2khz = 1,
    ///2: ULFRCO = 4 kHz
    _4khz = 2,
    ///3: ULFRCO = 32 kHz
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
///Field `MODE` reader - ULFRCO Mode
pub type ModeR = crate::FieldReader<MODE>;
impl ModeR {
    ///Get enumerated values variant
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
    ///ULFRCO = 1 kHz
    #[inline(always)]
    pub fn is_1khz(&self) -> bool {
        *self == MODE::_1khz
    }
    ///ULFRCO = 2 kHz
    #[inline(always)]
    pub fn is_2khz(&self) -> bool {
        *self == MODE::_2khz
    }
    ///ULFRCO = 4 kHz
    #[inline(always)]
    pub fn is_4khz(&self) -> bool {
        *self == MODE::_4khz
    }
    ///ULFRCO = 32 kHz
    #[inline(always)]
    pub fn is_32khz(&self) -> bool {
        *self == MODE::_32khz
    }
}
///Field `MODE` writer - ULFRCO Mode
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ULFRCO = 1 kHz
    #[inline(always)]
    pub fn _1khz(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::_1khz)
    }
    ///ULFRCO = 2 kHz
    #[inline(always)]
    pub fn _2khz(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::_2khz)
    }
    ///ULFRCO = 4 kHz
    #[inline(always)]
    pub fn _4khz(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::_4khz)
    }
    ///ULFRCO = 32 kHz
    #[inline(always)]
    pub fn _32khz(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::_32khz)
    }
}
///Field `RESTRIM` reader - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)
pub type RestrimR = crate::FieldReader;
///Field `RESTRIM` writer - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)
pub type RestrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:5 - ULFRCO TUNING Value
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 10:11 - ULFRCO Mode
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 16:17 - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)
    #[inline(always)]
    pub fn restrim(&self) -> RestrimR {
        RestrimR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ULFRCOCTRL")
            .field("tuning", &self.tuning())
            .field("mode", &self.mode())
            .field("restrim", &self.restrim())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - ULFRCO TUNING Value
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TuningW<ULFRCOCTRLrs> {
        TuningW::new(self, 0)
    }
    ///Bits 10:11 - ULFRCO Mode
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<ULFRCOCTRLrs> {
        ModeW::new(self, 10)
    }
    ///Bits 16:17 - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)
    #[inline(always)]
    #[must_use]
    pub fn restrim(&mut self) -> RestrimW<ULFRCOCTRLrs> {
        RestrimW::new(self, 16)
    }
}
///ULFRCO Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ulfrcoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulfrcoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ULFRCOCTRLrs;
impl crate::RegisterSpec for ULFRCOCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ulfrcoctrl::R`](R) reader structure
impl crate::Readable for ULFRCOCTRLrs {}
///`write(|w| ..)` method takes [`ulfrcoctrl::W`](W) writer structure
impl crate::Writable for ULFRCOCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ULFRCOCTRL to value 0x0002_0020
impl crate::Resettable for ULFRCOCTRLrs {
    const RESET_VALUE: u32 = 0x0002_0020;
}
