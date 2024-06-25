#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRLrs>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRLrs>;
#[doc = "Timer Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    #[doc = "0: Up-count mode"]
    Up = 0,
    #[doc = "1: Down-count mode"]
    Down = 1,
    #[doc = "2: Up/down-count mode"]
    Updown = 2,
    #[doc = "3: Quadrature decoder mode"]
    Qdec = 3,
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
#[doc = "Field `MODE` reader - Timer Mode"]
pub type ModeR = crate::FieldReader<MODE>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE {
        match self.bits {
            0 => MODE::Up,
            1 => MODE::Down,
            2 => MODE::Updown,
            3 => MODE::Qdec,
            _ => unreachable!(),
        }
    }
    #[doc = "Up-count mode"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == MODE::Up
    }
    #[doc = "Down-count mode"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == MODE::Down
    }
    #[doc = "Up/down-count mode"]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == MODE::Updown
    }
    #[doc = "Quadrature decoder mode"]
    #[inline(always)]
    pub fn is_qdec(&self) -> bool {
        *self == MODE::Qdec
    }
}
#[doc = "Field `MODE` writer - Timer Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Up-count mode"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Up)
    }
    #[doc = "Down-count mode"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Down)
    }
    #[doc = "Up/down-count mode"]
    #[inline(always)]
    pub fn updown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Updown)
    }
    #[doc = "Quadrature decoder mode"]
    #[inline(always)]
    pub fn qdec(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Qdec)
    }
}
#[doc = "Field `SYNC` reader - Timer Start/Stop/Reload Synchronization"]
pub type SyncR = crate::BitReader;
#[doc = "Field `SYNC` writer - Timer Start/Stop/Reload Synchronization"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSMEN` reader - One-shot Mode Enable"]
pub type OsmenR = crate::BitReader;
#[doc = "Field `OSMEN` writer - One-shot Mode Enable"]
pub type OsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QDM` reader - Quadrature Decoder Mode Selection"]
pub type QdmR = crate::BitReader;
#[doc = "Field `QDM` writer - Quadrature Decoder Mode Selection"]
pub type QdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DebugrunR = crate::BitReader;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DebugrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACLRACT` reader - DMA Request Clear on Active"]
pub type DmaclractR = crate::BitReader;
#[doc = "Field `DMACLRACT` writer - DMA Request Clear on Active"]
pub type DmaclractW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer Rising Input Edge Action\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RISEA {
    #[doc = "0: No action"]
    None = 0,
    #[doc = "1: Start counter without reload"]
    Start = 1,
    #[doc = "2: Stop counter without reload"]
    Stop = 2,
    #[doc = "3: Reload and start counter"]
    Reloadstart = 3,
}
impl From<RISEA> for u8 {
    #[inline(always)]
    fn from(variant: RISEA) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RISEA {
    type Ux = u8;
}
impl crate::IsEnum for RISEA {}
#[doc = "Field `RISEA` reader - Timer Rising Input Edge Action"]
pub type RiseaR = crate::FieldReader<RISEA>;
impl RiseaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RISEA {
        match self.bits {
            0 => RISEA::None,
            1 => RISEA::Start,
            2 => RISEA::Stop,
            3 => RISEA::Reloadstart,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RISEA::None
    }
    #[doc = "Start counter without reload"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == RISEA::Start
    }
    #[doc = "Stop counter without reload"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RISEA::Stop
    }
    #[doc = "Reload and start counter"]
    #[inline(always)]
    pub fn is_reloadstart(&self) -> bool {
        *self == RISEA::Reloadstart
    }
}
#[doc = "Field `RISEA` writer - Timer Rising Input Edge Action"]
pub type RiseaW<'a, REG> = crate::FieldWriter<'a, REG, 2, RISEA, crate::Safe>;
impl<'a, REG> RiseaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(RISEA::None)
    }
    #[doc = "Start counter without reload"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(RISEA::Start)
    }
    #[doc = "Stop counter without reload"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(RISEA::Stop)
    }
    #[doc = "Reload and start counter"]
    #[inline(always)]
    pub fn reloadstart(self) -> &'a mut crate::W<REG> {
        self.variant(RISEA::Reloadstart)
    }
}
#[doc = "Timer Falling Input Edge Action\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FALLA {
    #[doc = "0: No action"]
    None = 0,
    #[doc = "1: Start counter without reload"]
    Start = 1,
    #[doc = "2: Stop counter without reload"]
    Stop = 2,
    #[doc = "3: Reload and start counter"]
    Reloadstart = 3,
}
impl From<FALLA> for u8 {
    #[inline(always)]
    fn from(variant: FALLA) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FALLA {
    type Ux = u8;
}
impl crate::IsEnum for FALLA {}
#[doc = "Field `FALLA` reader - Timer Falling Input Edge Action"]
pub type FallaR = crate::FieldReader<FALLA>;
impl FallaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FALLA {
        match self.bits {
            0 => FALLA::None,
            1 => FALLA::Start,
            2 => FALLA::Stop,
            3 => FALLA::Reloadstart,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == FALLA::None
    }
    #[doc = "Start counter without reload"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == FALLA::Start
    }
    #[doc = "Stop counter without reload"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == FALLA::Stop
    }
    #[doc = "Reload and start counter"]
    #[inline(always)]
    pub fn is_reloadstart(&self) -> bool {
        *self == FALLA::Reloadstart
    }
}
#[doc = "Field `FALLA` writer - Timer Falling Input Edge Action"]
pub type FallaW<'a, REG> = crate::FieldWriter<'a, REG, 2, FALLA, crate::Safe>;
impl<'a, REG> FallaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(FALLA::None)
    }
    #[doc = "Start counter without reload"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(FALLA::Start)
    }
    #[doc = "Stop counter without reload"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(FALLA::Stop)
    }
    #[doc = "Reload and start counter"]
    #[inline(always)]
    pub fn reloadstart(self) -> &'a mut crate::W<REG> {
        self.variant(FALLA::Reloadstart)
    }
}
#[doc = "Field `X2CNT` reader - 2x Count Mode"]
pub type X2cntR = crate::BitReader;
#[doc = "Field `X2CNT` writer - 2x Count Mode"]
pub type X2cntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL {
    #[doc = "0: Prescaled HFPERCLK"]
    Preschfperclk = 0,
    #[doc = "1: Compare/Capture Channel 1 Input"]
    Cc1 = 1,
    #[doc = "2: Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer"]
    Timerouf = 2,
}
impl From<CLKSEL> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKSEL {
    type Ux = u8;
}
impl crate::IsEnum for CLKSEL {}
#[doc = "Field `CLKSEL` reader - Clock Source Select"]
pub type ClkselR = crate::FieldReader<CLKSEL>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKSEL> {
        match self.bits {
            0 => Some(CLKSEL::Preschfperclk),
            1 => Some(CLKSEL::Cc1),
            2 => Some(CLKSEL::Timerouf),
            _ => None,
        }
    }
    #[doc = "Prescaled HFPERCLK"]
    #[inline(always)]
    pub fn is_preschfperclk(&self) -> bool {
        *self == CLKSEL::Preschfperclk
    }
    #[doc = "Compare/Capture Channel 1 Input"]
    #[inline(always)]
    pub fn is_cc1(&self) -> bool {
        *self == CLKSEL::Cc1
    }
    #[doc = "Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer"]
    #[inline(always)]
    pub fn is_timerouf(&self) -> bool {
        *self == CLKSEL::Timerouf
    }
}
#[doc = "Field `CLKSEL` writer - Clock Source Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, CLKSEL>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaled HFPERCLK"]
    #[inline(always)]
    pub fn preschfperclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL::Preschfperclk)
    }
    #[doc = "Compare/Capture Channel 1 Input"]
    #[inline(always)]
    pub fn cc1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL::Cc1)
    }
    #[doc = "Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer"]
    #[inline(always)]
    pub fn timerouf(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL::Timerouf)
    }
}
#[doc = "Prescaler Setting\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC {
    #[doc = "0: The HFPERCLK is undivided"]
    Div1 = 0,
    #[doc = "1: The HFPERCLK is divided by 2"]
    Div2 = 1,
    #[doc = "2: The HFPERCLK is divided by 4"]
    Div4 = 2,
    #[doc = "3: The HFPERCLK is divided by 8"]
    Div8 = 3,
    #[doc = "4: The HFPERCLK is divided by 16"]
    Div16 = 4,
    #[doc = "5: The HFPERCLK is divided by 32"]
    Div32 = 5,
    #[doc = "6: The HFPERCLK is divided by 64"]
    Div64 = 6,
    #[doc = "7: The HFPERCLK is divided by 128"]
    Div128 = 7,
    #[doc = "8: The HFPERCLK is divided by 256"]
    Div256 = 8,
    #[doc = "9: The HFPERCLK is divided by 512"]
    Div512 = 9,
    #[doc = "10: The HFPERCLK is divided by 1024"]
    Div1024 = 10,
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
impl crate::IsEnum for PRESC {}
#[doc = "Field `PRESC` reader - Prescaler Setting"]
pub type PrescR = crate::FieldReader<PRESC>;
impl PrescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESC> {
        match self.bits {
            0 => Some(PRESC::Div1),
            1 => Some(PRESC::Div2),
            2 => Some(PRESC::Div4),
            3 => Some(PRESC::Div8),
            4 => Some(PRESC::Div16),
            5 => Some(PRESC::Div32),
            6 => Some(PRESC::Div64),
            7 => Some(PRESC::Div128),
            8 => Some(PRESC::Div256),
            9 => Some(PRESC::Div512),
            10 => Some(PRESC::Div1024),
            _ => None,
        }
    }
    #[doc = "The HFPERCLK is undivided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC::Div1
    }
    #[doc = "The HFPERCLK is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC::Div2
    }
    #[doc = "The HFPERCLK is divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC::Div4
    }
    #[doc = "The HFPERCLK is divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC::Div8
    }
    #[doc = "The HFPERCLK is divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC::Div16
    }
    #[doc = "The HFPERCLK is divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC::Div32
    }
    #[doc = "The HFPERCLK is divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC::Div64
    }
    #[doc = "The HFPERCLK is divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC::Div128
    }
    #[doc = "The HFPERCLK is divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESC::Div256
    }
    #[doc = "The HFPERCLK is divided by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == PRESC::Div512
    }
    #[doc = "The HFPERCLK is divided by 1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == PRESC::Div1024
    }
}
#[doc = "Field `PRESC` writer - Prescaler Setting"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 4, PRESC>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The HFPERCLK is undivided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div1)
    }
    #[doc = "The HFPERCLK is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div2)
    }
    #[doc = "The HFPERCLK is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div4)
    }
    #[doc = "The HFPERCLK is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div8)
    }
    #[doc = "The HFPERCLK is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div16)
    }
    #[doc = "The HFPERCLK is divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div32)
    }
    #[doc = "The HFPERCLK is divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div64)
    }
    #[doc = "The HFPERCLK is divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div128)
    }
    #[doc = "The HFPERCLK is divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div256)
    }
    #[doc = "The HFPERCLK is divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div512)
    }
    #[doc = "The HFPERCLK is divided by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div1024)
    }
}
#[doc = "Field `ATI` reader - Always Track Inputs"]
pub type AtiR = crate::BitReader;
#[doc = "Field `ATI` writer - Always Track Inputs"]
pub type AtiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSSCOIST` reader - Reload-Start Sets Compare Output Initial State"]
pub type RsscoistR = crate::BitReader;
#[doc = "Field `RSSCOIST` writer - Reload-Start Sets Compare Output Initial State"]
pub type RsscoistW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Timer Start/Stop/Reload Synchronization"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - One-shot Mode Enable"]
    #[inline(always)]
    pub fn osmen(&self) -> OsmenR {
        OsmenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Quadrature Decoder Mode Selection"]
    #[inline(always)]
    pub fn qdm(&self) -> QdmR {
        QdmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DebugrunR {
        DebugrunR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Request Clear on Active"]
    #[inline(always)]
    pub fn dmaclract(&self) -> DmaclractR {
        DmaclractR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Timer Rising Input Edge Action"]
    #[inline(always)]
    pub fn risea(&self) -> RiseaR {
        RiseaR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Timer Falling Input Edge Action"]
    #[inline(always)]
    pub fn falla(&self) -> FallaR {
        FallaR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 13 - 2x Count Mode"]
    #[inline(always)]
    pub fn x2cnt(&self) -> X2cntR {
        X2cntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Clock Source Select"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Always Track Inputs"]
    #[inline(always)]
    pub fn ati(&self) -> AtiR {
        AtiR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reload-Start Sets Compare Output Initial State"]
    #[inline(always)]
    pub fn rsscoist(&self) -> RsscoistR {
        RsscoistR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("mode", &self.mode())
            .field("sync", &self.sync())
            .field("osmen", &self.osmen())
            .field("qdm", &self.qdm())
            .field("debugrun", &self.debugrun())
            .field("dmaclract", &self.dmaclract())
            .field("risea", &self.risea())
            .field("falla", &self.falla())
            .field("x2cnt", &self.x2cnt())
            .field("clksel", &self.clksel())
            .field("presc", &self.presc())
            .field("ati", &self.ati())
            .field("rsscoist", &self.rsscoist())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CTRLrs> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 3 - Timer Start/Stop/Reload Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SyncW<CTRLrs> {
        SyncW::new(self, 3)
    }
    #[doc = "Bit 4 - One-shot Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn osmen(&mut self) -> OsmenW<CTRLrs> {
        OsmenW::new(self, 4)
    }
    #[doc = "Bit 5 - Quadrature Decoder Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn qdm(&mut self) -> QdmW<CTRLrs> {
        QdmW::new(self, 5)
    }
    #[doc = "Bit 6 - Debug Mode Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debugrun(&mut self) -> DebugrunW<CTRLrs> {
        DebugrunW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA Request Clear on Active"]
    #[inline(always)]
    #[must_use]
    pub fn dmaclract(&mut self) -> DmaclractW<CTRLrs> {
        DmaclractW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Timer Rising Input Edge Action"]
    #[inline(always)]
    #[must_use]
    pub fn risea(&mut self) -> RiseaW<CTRLrs> {
        RiseaW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Timer Falling Input Edge Action"]
    #[inline(always)]
    #[must_use]
    pub fn falla(&mut self) -> FallaW<CTRLrs> {
        FallaW::new(self, 10)
    }
    #[doc = "Bit 13 - 2x Count Mode"]
    #[inline(always)]
    #[must_use]
    pub fn x2cnt(&mut self) -> X2cntW<CTRLrs> {
        X2cntW::new(self, 13)
    }
    #[doc = "Bits 16:17 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> ClkselW<CTRLrs> {
        ClkselW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Prescaler Setting"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PrescW<CTRLrs> {
        PrescW::new(self, 24)
    }
    #[doc = "Bit 28 - Always Track Inputs"]
    #[inline(always)]
    #[must_use]
    pub fn ati(&mut self) -> AtiW<CTRLrs> {
        AtiW::new(self, 28)
    }
    #[doc = "Bit 29 - Reload-Start Sets Compare Output Initial State"]
    #[inline(always)]
    #[must_use]
    pub fn rsscoist(&mut self) -> RsscoistW<CTRLrs> {
        RsscoistW::new(self, 29)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
