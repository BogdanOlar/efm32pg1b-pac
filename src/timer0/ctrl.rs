#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRLrs>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRLrs>;
#[doc = "Field `MODE` reader - Timer Mode"]
pub type MODE_R = crate::FieldReader<MODE>;
#[doc = "Timer Mode\n\nValue on reset: 0"]
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
impl MODE_R {
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
pub type MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE>;
impl<'a, REG> MODE_W<'a, REG>
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
pub type SYNC_R = crate::BitReader;
#[doc = "Field `SYNC` writer - Timer Start/Stop/Reload Synchronization"]
pub type SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSMEN` reader - One-shot Mode Enable"]
pub type OSMEN_R = crate::BitReader;
#[doc = "Field `OSMEN` writer - One-shot Mode Enable"]
pub type OSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QDM` reader - Quadrature Decoder Mode Selection"]
pub type QDM_R = crate::BitReader;
#[doc = "Field `QDM` writer - Quadrature Decoder Mode Selection"]
pub type QDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DEBUGRUN_R = crate::BitReader;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DEBUGRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACLRACT` reader - DMA Request Clear on Active"]
pub type DMACLRACT_R = crate::BitReader;
#[doc = "Field `DMACLRACT` writer - DMA Request Clear on Active"]
pub type DMACLRACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RISEA` reader - Timer Rising Input Edge Action"]
pub type RISEA_R = crate::FieldReader<RISEA>;
#[doc = "Timer Rising Input Edge Action\n\nValue on reset: 0"]
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
impl RISEA_R {
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
pub type RISEA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RISEA>;
impl<'a, REG> RISEA_W<'a, REG>
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
#[doc = "Field `FALLA` reader - Timer Falling Input Edge Action"]
pub type FALLA_R = crate::FieldReader<FALLA>;
#[doc = "Timer Falling Input Edge Action\n\nValue on reset: 0"]
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
impl FALLA_R {
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
pub type FALLA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FALLA>;
impl<'a, REG> FALLA_W<'a, REG>
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
pub type X2CNT_R = crate::BitReader;
#[doc = "Field `X2CNT` writer - 2x Count Mode"]
pub type X2CNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSEL` reader - Clock Source Select"]
pub type CLKSEL_R = crate::FieldReader<CLKSEL>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
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
impl CLKSEL_R {
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
pub type CLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLKSEL>;
impl<'a, REG> CLKSEL_W<'a, REG>
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
#[doc = "Field `PRESC` reader - Prescaler Setting"]
pub type PRESC_R = crate::FieldReader<PRESC>;
#[doc = "Prescaler Setting\n\nValue on reset: 0"]
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
impl PRESC_R {
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
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRESC>;
impl<'a, REG> PRESC_W<'a, REG>
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
pub type ATI_R = crate::BitReader;
#[doc = "Field `ATI` writer - Always Track Inputs"]
pub type ATI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSSCOIST` reader - Reload-Start Sets Compare Output Initial State"]
pub type RSSCOIST_R = crate::BitReader;
#[doc = "Field `RSSCOIST` writer - Reload-Start Sets Compare Output Initial State"]
pub type RSSCOIST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Timer Start/Stop/Reload Synchronization"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - One-shot Mode Enable"]
    #[inline(always)]
    pub fn osmen(&self) -> OSMEN_R {
        OSMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Quadrature Decoder Mode Selection"]
    #[inline(always)]
    pub fn qdm(&self) -> QDM_R {
        QDM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Request Clear on Active"]
    #[inline(always)]
    pub fn dmaclract(&self) -> DMACLRACT_R {
        DMACLRACT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Timer Rising Input Edge Action"]
    #[inline(always)]
    pub fn risea(&self) -> RISEA_R {
        RISEA_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Timer Falling Input Edge Action"]
    #[inline(always)]
    pub fn falla(&self) -> FALLA_R {
        FALLA_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 13 - 2x Count Mode"]
    #[inline(always)]
    pub fn x2cnt(&self) -> X2CNT_R {
        X2CNT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Clock Source Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Always Track Inputs"]
    #[inline(always)]
    pub fn ati(&self) -> ATI_R {
        ATI_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reload-Start Sets Compare Output Initial State"]
    #[inline(always)]
    pub fn rsscoist(&self) -> RSSCOIST_R {
        RSSCOIST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTRLrs> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bit 3 - Timer Start/Stop/Reload Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<CTRLrs> {
        SYNC_W::new(self, 3)
    }
    #[doc = "Bit 4 - One-shot Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn osmen(&mut self) -> OSMEN_W<CTRLrs> {
        OSMEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Quadrature Decoder Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn qdm(&mut self) -> QDM_W<CTRLrs> {
        QDM_W::new(self, 5)
    }
    #[doc = "Bit 6 - Debug Mode Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debugrun(&mut self) -> DEBUGRUN_W<CTRLrs> {
        DEBUGRUN_W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA Request Clear on Active"]
    #[inline(always)]
    #[must_use]
    pub fn dmaclract(&mut self) -> DMACLRACT_W<CTRLrs> {
        DMACLRACT_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Timer Rising Input Edge Action"]
    #[inline(always)]
    #[must_use]
    pub fn risea(&mut self) -> RISEA_W<CTRLrs> {
        RISEA_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Timer Falling Input Edge Action"]
    #[inline(always)]
    #[must_use]
    pub fn falla(&mut self) -> FALLA_W<CTRLrs> {
        FALLA_W::new(self, 10)
    }
    #[doc = "Bit 13 - 2x Count Mode"]
    #[inline(always)]
    #[must_use]
    pub fn x2cnt(&mut self) -> X2CNT_W<CTRLrs> {
        X2CNT_W::new(self, 13)
    }
    #[doc = "Bits 16:17 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<CTRLrs> {
        CLKSEL_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Prescaler Setting"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<CTRLrs> {
        PRESC_W::new(self, 24)
    }
    #[doc = "Bit 28 - Always Track Inputs"]
    #[inline(always)]
    #[must_use]
    pub fn ati(&mut self) -> ATI_W<CTRLrs> {
        ATI_W::new(self, 28)
    }
    #[doc = "Bit 29 - Reload-Start Sets Compare Output Initial State"]
    #[inline(always)]
    #[must_use]
    pub fn rsscoist(&mut self) -> RSSCOIST_W<CTRLrs> {
        RSSCOIST_W::new(self, 29)
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
