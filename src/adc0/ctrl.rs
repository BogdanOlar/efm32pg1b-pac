#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRLrs>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRLrs>;
#[doc = "Field `WARMUPMODE` reader - Warm-up Mode"]
pub type WARMUPMODE_R = crate::FieldReader<WARMUPMODE>;
#[doc = "Warm-up Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WARMUPMODE {
    #[doc = "0: ADC is shut down after each conversion. 5us warmup time is used before each conversion."]
    Normal = 0,
    #[doc = "1: ADC is kept in standby mode between conversions. 1us warmup time is used before each conversion."]
    Keepinstandby = 1,
    #[doc = "2: ADC is kept in slow acquisition mode between conversions. 1us warmup time is used before each conversion."]
    Keepinslowacc = 2,
    #[doc = "3: ADC is kept on after conversions, allowing for continuous conversion."]
    Keepadcwarm = 3,
}
impl From<WARMUPMODE> for u8 {
    #[inline(always)]
    fn from(variant: WARMUPMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WARMUPMODE {
    type Ux = u8;
}
impl WARMUPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WARMUPMODE {
        match self.bits {
            0 => WARMUPMODE::Normal,
            1 => WARMUPMODE::Keepinstandby,
            2 => WARMUPMODE::Keepinslowacc,
            3 => WARMUPMODE::Keepadcwarm,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC is shut down after each conversion. 5us warmup time is used before each conversion."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == WARMUPMODE::Normal
    }
    #[doc = "ADC is kept in standby mode between conversions. 1us warmup time is used before each conversion."]
    #[inline(always)]
    pub fn is_keepinstandby(&self) -> bool {
        *self == WARMUPMODE::Keepinstandby
    }
    #[doc = "ADC is kept in slow acquisition mode between conversions. 1us warmup time is used before each conversion."]
    #[inline(always)]
    pub fn is_keepinslowacc(&self) -> bool {
        *self == WARMUPMODE::Keepinslowacc
    }
    #[doc = "ADC is kept on after conversions, allowing for continuous conversion."]
    #[inline(always)]
    pub fn is_keepadcwarm(&self) -> bool {
        *self == WARMUPMODE::Keepadcwarm
    }
}
#[doc = "Field `WARMUPMODE` writer - Warm-up Mode"]
pub type WARMUPMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WARMUPMODE>;
impl<'a, REG> WARMUPMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC is shut down after each conversion. 5us warmup time is used before each conversion."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(WARMUPMODE::Normal)
    }
    #[doc = "ADC is kept in standby mode between conversions. 1us warmup time is used before each conversion."]
    #[inline(always)]
    pub fn keepinstandby(self) -> &'a mut crate::W<REG> {
        self.variant(WARMUPMODE::Keepinstandby)
    }
    #[doc = "ADC is kept in slow acquisition mode between conversions. 1us warmup time is used before each conversion."]
    #[inline(always)]
    pub fn keepinslowacc(self) -> &'a mut crate::W<REG> {
        self.variant(WARMUPMODE::Keepinslowacc)
    }
    #[doc = "ADC is kept on after conversions, allowing for continuous conversion."]
    #[inline(always)]
    pub fn keepadcwarm(self) -> &'a mut crate::W<REG> {
        self.variant(WARMUPMODE::Keepadcwarm)
    }
}
#[doc = "Field `SINGLEDMAWU` reader - SINGLEFIFO DMA Wakeup"]
pub type SINGLEDMAWU_R = crate::BitReader;
#[doc = "Field `SINGLEDMAWU` writer - SINGLEFIFO DMA Wakeup"]
pub type SINGLEDMAWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANDMAWU` reader - SCANFIFO DMA Wakeup"]
pub type SCANDMAWU_R = crate::BitReader;
#[doc = "Field `SCANDMAWU` writer - SCANFIFO DMA Wakeup"]
pub type SCANDMAWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAILGATE` reader - Conversion Tailgating"]
pub type TAILGATE_R = crate::BitReader;
#[doc = "Field `TAILGATE` writer - Conversion Tailgating"]
pub type TAILGATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCCLKEN` reader - Selects ASYNC CLK Enable Mode When ADCCLKMODE=1"]
pub type ASYNCCLKEN_R = crate::BitReader;
#[doc = "Field `ASYNCCLKEN` writer - Selects ASYNC CLK Enable Mode When ADCCLKMODE=1"]
pub type ASYNCCLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCCLKMODE` reader - ADC Clock Mode"]
pub type ADCCLKMODE_R = crate::BitReader;
#[doc = "Field `ADCCLKMODE` writer - ADC Clock Mode"]
pub type ADCCLKMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESC` reader - Prescalar Setting for ADC Sample and Conversion Clock"]
pub type PRESC_R = crate::FieldReader<PRESC>;
#[doc = "Prescalar Setting for ADC Sample and Conversion Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC {
    #[doc = "0: `0`"]
    Nodivision = 0,
}
impl From<PRESC> for u8 {
    #[inline(always)]
    fn from(variant: PRESC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC {
    type Ux = u8;
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESC> {
        match self.bits {
            0 => Some(PRESC::Nodivision),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC::Nodivision
    }
}
#[doc = "Field `PRESC` writer - Prescalar Setting for ADC Sample and Conversion Clock"]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 7, PRESC>;
impl<'a, REG> PRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Nodivision)
    }
}
#[doc = "Field `TIMEBASE` reader - 1us Time Base"]
pub type TIMEBASE_R = crate::FieldReader;
#[doc = "Field `TIMEBASE` writer - 1us Time Base"]
pub type TIMEBASE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `OVSRSEL` reader - Oversample Rate Select"]
pub type OVSRSEL_R = crate::FieldReader<OVSRSEL>;
#[doc = "Oversample Rate Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSRSEL {
    #[doc = "0: 2 samples for each conversion result"]
    X2 = 0,
    #[doc = "1: 4 samples for each conversion result"]
    X4 = 1,
    #[doc = "2: 8 samples for each conversion result"]
    X8 = 2,
    #[doc = "3: 16 samples for each conversion result"]
    X16 = 3,
    #[doc = "4: 32 samples for each conversion result"]
    X32 = 4,
    #[doc = "5: 64 samples for each conversion result"]
    X64 = 5,
    #[doc = "6: 128 samples for each conversion result"]
    X128 = 6,
    #[doc = "7: 256 samples for each conversion result"]
    X256 = 7,
    #[doc = "8: 512 samples for each conversion result"]
    X512 = 8,
    #[doc = "9: 1024 samples for each conversion result"]
    X1024 = 9,
    #[doc = "10: 2048 samples for each conversion result"]
    X2048 = 10,
    #[doc = "11: 4096 samples for each conversion result"]
    X4096 = 11,
}
impl From<OVSRSEL> for u8 {
    #[inline(always)]
    fn from(variant: OVSRSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OVSRSEL {
    type Ux = u8;
}
impl OVSRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OVSRSEL> {
        match self.bits {
            0 => Some(OVSRSEL::X2),
            1 => Some(OVSRSEL::X4),
            2 => Some(OVSRSEL::X8),
            3 => Some(OVSRSEL::X16),
            4 => Some(OVSRSEL::X32),
            5 => Some(OVSRSEL::X64),
            6 => Some(OVSRSEL::X128),
            7 => Some(OVSRSEL::X256),
            8 => Some(OVSRSEL::X512),
            9 => Some(OVSRSEL::X1024),
            10 => Some(OVSRSEL::X2048),
            11 => Some(OVSRSEL::X4096),
            _ => None,
        }
    }
    #[doc = "2 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == OVSRSEL::X2
    }
    #[doc = "4 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == OVSRSEL::X4
    }
    #[doc = "8 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == OVSRSEL::X8
    }
    #[doc = "16 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == OVSRSEL::X16
    }
    #[doc = "32 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x32(&self) -> bool {
        *self == OVSRSEL::X32
    }
    #[doc = "64 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x64(&self) -> bool {
        *self == OVSRSEL::X64
    }
    #[doc = "128 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x128(&self) -> bool {
        *self == OVSRSEL::X128
    }
    #[doc = "256 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x256(&self) -> bool {
        *self == OVSRSEL::X256
    }
    #[doc = "512 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x512(&self) -> bool {
        *self == OVSRSEL::X512
    }
    #[doc = "1024 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x1024(&self) -> bool {
        *self == OVSRSEL::X1024
    }
    #[doc = "2048 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x2048(&self) -> bool {
        *self == OVSRSEL::X2048
    }
    #[doc = "4096 samples for each conversion result"]
    #[inline(always)]
    pub fn is_x4096(&self) -> bool {
        *self == OVSRSEL::X4096
    }
}
#[doc = "Field `OVSRSEL` writer - Oversample Rate Select"]
pub type OVSRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, OVSRSEL>;
impl<'a, REG> OVSRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2 samples for each conversion result"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X2)
    }
    #[doc = "4 samples for each conversion result"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X4)
    }
    #[doc = "8 samples for each conversion result"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X8)
    }
    #[doc = "16 samples for each conversion result"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X16)
    }
    #[doc = "32 samples for each conversion result"]
    #[inline(always)]
    pub fn x32(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X32)
    }
    #[doc = "64 samples for each conversion result"]
    #[inline(always)]
    pub fn x64(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X64)
    }
    #[doc = "128 samples for each conversion result"]
    #[inline(always)]
    pub fn x128(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X128)
    }
    #[doc = "256 samples for each conversion result"]
    #[inline(always)]
    pub fn x256(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X256)
    }
    #[doc = "512 samples for each conversion result"]
    #[inline(always)]
    pub fn x512(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X512)
    }
    #[doc = "1024 samples for each conversion result"]
    #[inline(always)]
    pub fn x1024(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X1024)
    }
    #[doc = "2048 samples for each conversion result"]
    #[inline(always)]
    pub fn x2048(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X2048)
    }
    #[doc = "4096 samples for each conversion result"]
    #[inline(always)]
    pub fn x4096(self) -> &'a mut crate::W<REG> {
        self.variant(OVSRSEL::X4096)
    }
}
#[doc = "Field `CHCONMODE` reader - Channel Connect"]
pub type CHCONMODE_R = crate::BitReader;
#[doc = "Field `CHCONMODE` writer - Channel Connect"]
pub type CHCONMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Warm-up Mode"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WARMUPMODE_R {
        WARMUPMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - SINGLEFIFO DMA Wakeup"]
    #[inline(always)]
    pub fn singledmawu(&self) -> SINGLEDMAWU_R {
        SINGLEDMAWU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCANFIFO DMA Wakeup"]
    #[inline(always)]
    pub fn scandmawu(&self) -> SCANDMAWU_R {
        SCANDMAWU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Conversion Tailgating"]
    #[inline(always)]
    pub fn tailgate(&self) -> TAILGATE_R {
        TAILGATE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects ASYNC CLK Enable Mode When ADCCLKMODE=1"]
    #[inline(always)]
    pub fn asyncclken(&self) -> ASYNCCLKEN_R {
        ASYNCCLKEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC Clock Mode"]
    #[inline(always)]
    pub fn adcclkmode(&self) -> ADCCLKMODE_R {
        ADCCLKMODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Prescalar Setting for ADC Sample and Conversion Clock"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - 1us Time Base"]
    #[inline(always)]
    pub fn timebase(&self) -> TIMEBASE_R {
        TIMEBASE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - Oversample Rate Select"]
    #[inline(always)]
    pub fn ovsrsel(&self) -> OVSRSEL_R {
        OVSRSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Channel Connect"]
    #[inline(always)]
    pub fn chconmode(&self) -> CHCONMODE_R {
        CHCONMODE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Warm-up Mode"]
    #[inline(always)]
    #[must_use]
    pub fn warmupmode(&mut self) -> WARMUPMODE_W<CTRLrs> {
        WARMUPMODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - SINGLEFIFO DMA Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn singledmawu(&mut self) -> SINGLEDMAWU_W<CTRLrs> {
        SINGLEDMAWU_W::new(self, 2)
    }
    #[doc = "Bit 3 - SCANFIFO DMA Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn scandmawu(&mut self) -> SCANDMAWU_W<CTRLrs> {
        SCANDMAWU_W::new(self, 3)
    }
    #[doc = "Bit 4 - Conversion Tailgating"]
    #[inline(always)]
    #[must_use]
    pub fn tailgate(&mut self) -> TAILGATE_W<CTRLrs> {
        TAILGATE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Selects ASYNC CLK Enable Mode When ADCCLKMODE=1"]
    #[inline(always)]
    #[must_use]
    pub fn asyncclken(&mut self) -> ASYNCCLKEN_W<CTRLrs> {
        ASYNCCLKEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - ADC Clock Mode"]
    #[inline(always)]
    #[must_use]
    pub fn adcclkmode(&mut self) -> ADCCLKMODE_W<CTRLrs> {
        ADCCLKMODE_W::new(self, 7)
    }
    #[doc = "Bits 8:14 - Prescalar Setting for ADC Sample and Conversion Clock"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<CTRLrs> {
        PRESC_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - 1us Time Base"]
    #[inline(always)]
    #[must_use]
    pub fn timebase(&mut self) -> TIMEBASE_W<CTRLrs> {
        TIMEBASE_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Oversample Rate Select"]
    #[inline(always)]
    #[must_use]
    pub fn ovsrsel(&mut self) -> OVSRSEL_W<CTRLrs> {
        OVSRSEL_W::new(self, 24)
    }
    #[doc = "Bit 29 - Channel Connect"]
    #[inline(always)]
    #[must_use]
    pub fn chconmode(&mut self) -> CHCONMODE_W<CTRLrs> {
        CHCONMODE_W::new(self, 29)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x001f_0000"]
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0x001f_0000;
}
