///Register `LFRCOCTRL` reader
pub type R = crate::R<LFRCOCTRLrs>;
///Register `LFRCOCTRL` writer
pub type W = crate::W<LFRCOCTRLrs>;
///Field `TUNING` reader - LFRCO Tuning Value
pub type TuningR = crate::FieldReader<u16>;
///Field `TUNING` writer - LFRCO Tuning Value
pub type TuningW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `ENVREF` reader - Enable Duty Cycling of Vref
pub type EnvrefR = crate::BitReader;
///Field `ENVREF` writer - Enable Duty Cycling of Vref
pub type EnvrefW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENCHOP` reader - Enable Comparator Chopping
pub type EnchopR = crate::BitReader;
///Field `ENCHOP` writer - Enable Comparator Chopping
pub type EnchopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENDEM` reader - Enable Dynamic Element Matching
pub type EndemR = crate::BitReader;
///Field `ENDEM` writer - Enable Dynamic Element Matching
pub type EndemW<'a, REG> = crate::BitWriter<'a, REG>;
///LFRCO Timeout
///
///Value on reset: 1
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEOUT {
    ///0: Timeout period of 2 cycles
    _2cycles = 0,
    ///1: Timeout period of 16 cycles
    _16cycles = 1,
    ///2: Timeout period of 32 cycles
    _32cycles = 2,
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
///Field `TIMEOUT` reader - LFRCO Timeout
pub type TimeoutR = crate::FieldReader<TIMEOUT>;
impl TimeoutR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIMEOUT> {
        match self.bits {
            0 => Some(TIMEOUT::_2cycles),
            1 => Some(TIMEOUT::_16cycles),
            2 => Some(TIMEOUT::_32cycles),
            _ => None,
        }
    }
    ///Timeout period of 2 cycles
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == TIMEOUT::_2cycles
    }
    ///Timeout period of 16 cycles
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == TIMEOUT::_16cycles
    }
    ///Timeout period of 32 cycles
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == TIMEOUT::_32cycles
    }
}
///Field `TIMEOUT` writer - LFRCO Timeout
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 2, TIMEOUT>;
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
    ///Timeout period of 16 cycles
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_16cycles)
    }
    ///Timeout period of 32 cycles
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT::_32cycles)
    }
}
///Field `GMCCURTUNE` reader - Tuning of Gmc Current
pub type GmccurtuneR = crate::FieldReader;
///Field `GMCCURTUNE` writer - Tuning of Gmc Current
pub type GmccurtuneW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:8 - LFRCO Tuning Value
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 16 - Enable Duty Cycling of Vref
    #[inline(always)]
    pub fn envref(&self) -> EnvrefR {
        EnvrefR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Enable Comparator Chopping
    #[inline(always)]
    pub fn enchop(&self) -> EnchopR {
        EnchopR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Enable Dynamic Element Matching
    #[inline(always)]
    pub fn endem(&self) -> EndemR {
        EndemR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 24:25 - LFRCO Timeout
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 28:31 - Tuning of Gmc Current
    #[inline(always)]
    pub fn gmccurtune(&self) -> GmccurtuneR {
        GmccurtuneR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFRCOCTRL")
            .field("tuning", &self.tuning())
            .field("envref", &self.envref())
            .field("enchop", &self.enchop())
            .field("endem", &self.endem())
            .field("timeout", &self.timeout())
            .field("gmccurtune", &self.gmccurtune())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - LFRCO Tuning Value
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TuningW<LFRCOCTRLrs> {
        TuningW::new(self, 0)
    }
    ///Bit 16 - Enable Duty Cycling of Vref
    #[inline(always)]
    #[must_use]
    pub fn envref(&mut self) -> EnvrefW<LFRCOCTRLrs> {
        EnvrefW::new(self, 16)
    }
    ///Bit 17 - Enable Comparator Chopping
    #[inline(always)]
    #[must_use]
    pub fn enchop(&mut self) -> EnchopW<LFRCOCTRLrs> {
        EnchopW::new(self, 17)
    }
    ///Bit 18 - Enable Dynamic Element Matching
    #[inline(always)]
    #[must_use]
    pub fn endem(&mut self) -> EndemW<LFRCOCTRLrs> {
        EndemW::new(self, 18)
    }
    ///Bits 24:25 - LFRCO Timeout
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<LFRCOCTRLrs> {
        TimeoutW::new(self, 24)
    }
    ///Bits 28:31 - Tuning of Gmc Current
    #[inline(always)]
    #[must_use]
    pub fn gmccurtune(&mut self) -> GmccurtuneW<LFRCOCTRLrs> {
        GmccurtuneW::new(self, 28)
    }
}
///LFRCO Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`lfrcoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfrcoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LFRCOCTRLrs;
impl crate::RegisterSpec for LFRCOCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`lfrcoctrl::R`](R) reader structure
impl crate::Readable for LFRCOCTRLrs {}
///`write(|w| ..)` method takes [`lfrcoctrl::W`](W) writer structure
impl crate::Writable for LFRCOCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LFRCOCTRL to value 0x8106_0100
impl crate::Resettable for LFRCOCTRLrs {
    const RESET_VALUE: u32 = 0x8106_0100;
}
