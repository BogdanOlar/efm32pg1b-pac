#[doc = "Register `CC1_CTRL` reader"]
pub type R = crate::R<CC1_CTRLrs>;
#[doc = "Register `CC1_CTRL` writer"]
pub type W = crate::W<CC1_CTRLrs>;
#[doc = "CC Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    #[doc = "0: Compare/Capture channel turned off"]
    Off = 0,
    #[doc = "1: Input capture"]
    Inputcapture = 1,
    #[doc = "2: Output compare"]
    Outputcompare = 2,
    #[doc = "3: Pulse-Width Modulation"]
    Pwm = 3,
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
#[doc = "Field `MODE` reader - CC Channel Mode"]
pub type ModeR = crate::FieldReader<MODE>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE {
        match self.bits {
            0 => MODE::Off,
            1 => MODE::Inputcapture,
            2 => MODE::Outputcompare,
            3 => MODE::Pwm,
            _ => unreachable!(),
        }
    }
    #[doc = "Compare/Capture channel turned off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MODE::Off
    }
    #[doc = "Input capture"]
    #[inline(always)]
    pub fn is_inputcapture(&self) -> bool {
        *self == MODE::Inputcapture
    }
    #[doc = "Output compare"]
    #[inline(always)]
    pub fn is_outputcompare(&self) -> bool {
        *self == MODE::Outputcompare
    }
    #[doc = "Pulse-Width Modulation"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == MODE::Pwm
    }
}
#[doc = "Field `MODE` writer - CC Channel Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare/Capture channel turned off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Off)
    }
    #[doc = "Input capture"]
    #[inline(always)]
    pub fn inputcapture(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Inputcapture)
    }
    #[doc = "Output compare"]
    #[inline(always)]
    pub fn outputcompare(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Outputcompare)
    }
    #[doc = "Pulse-Width Modulation"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Pwm)
    }
}
#[doc = "Field `OUTINV` reader - Output Invert"]
pub type OutinvR = crate::BitReader;
#[doc = "Field `OUTINV` writer - Output Invert"]
pub type OutinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COIST` reader - Compare Output Initial State"]
pub type CoistR = crate::BitReader;
#[doc = "Field `COIST` writer - Compare Output Initial State"]
pub type CoistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Compare Match Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMOA {
    #[doc = "0: No action on compare match"]
    None = 0,
    #[doc = "1: Toggle output on compare match"]
    Toggle = 1,
    #[doc = "2: Clear output on compare match"]
    Clear = 2,
    #[doc = "3: Set output on compare match"]
    Set = 3,
}
impl From<CMOA> for u8 {
    #[inline(always)]
    fn from(variant: CMOA) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMOA {
    type Ux = u8;
}
impl crate::IsEnum for CMOA {}
#[doc = "Field `CMOA` reader - Compare Match Output Action"]
pub type CmoaR = crate::FieldReader<CMOA>;
impl CmoaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMOA {
        match self.bits {
            0 => CMOA::None,
            1 => CMOA::Toggle,
            2 => CMOA::Clear,
            3 => CMOA::Set,
            _ => unreachable!(),
        }
    }
    #[doc = "No action on compare match"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CMOA::None
    }
    #[doc = "Toggle output on compare match"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == CMOA::Toggle
    }
    #[doc = "Clear output on compare match"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CMOA::Clear
    }
    #[doc = "Set output on compare match"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMOA::Set
    }
}
#[doc = "Field `CMOA` writer - Compare Match Output Action"]
pub type CmoaW<'a, REG> = crate::FieldWriter<'a, REG, 2, CMOA, crate::Safe>;
impl<'a, REG> CmoaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action on compare match"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CMOA::None)
    }
    #[doc = "Toggle output on compare match"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(CMOA::Toggle)
    }
    #[doc = "Clear output on compare match"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CMOA::Clear)
    }
    #[doc = "Set output on compare match"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CMOA::Set)
    }
}
#[doc = "Counter Overflow Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COFOA {
    #[doc = "0: No action on counter overflow"]
    None = 0,
    #[doc = "1: Toggle output on counter overflow"]
    Toggle = 1,
    #[doc = "2: Clear output on counter overflow"]
    Clear = 2,
    #[doc = "3: Set output on counter overflow"]
    Set = 3,
}
impl From<COFOA> for u8 {
    #[inline(always)]
    fn from(variant: COFOA) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COFOA {
    type Ux = u8;
}
impl crate::IsEnum for COFOA {}
#[doc = "Field `COFOA` reader - Counter Overflow Output Action"]
pub type CofoaR = crate::FieldReader<COFOA>;
impl CofoaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COFOA {
        match self.bits {
            0 => COFOA::None,
            1 => COFOA::Toggle,
            2 => COFOA::Clear,
            3 => COFOA::Set,
            _ => unreachable!(),
        }
    }
    #[doc = "No action on counter overflow"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == COFOA::None
    }
    #[doc = "Toggle output on counter overflow"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == COFOA::Toggle
    }
    #[doc = "Clear output on counter overflow"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == COFOA::Clear
    }
    #[doc = "Set output on counter overflow"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == COFOA::Set
    }
}
#[doc = "Field `COFOA` writer - Counter Overflow Output Action"]
pub type CofoaW<'a, REG> = crate::FieldWriter<'a, REG, 2, COFOA, crate::Safe>;
impl<'a, REG> CofoaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action on counter overflow"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(COFOA::None)
    }
    #[doc = "Toggle output on counter overflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(COFOA::Toggle)
    }
    #[doc = "Clear output on counter overflow"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(COFOA::Clear)
    }
    #[doc = "Set output on counter overflow"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(COFOA::Set)
    }
}
#[doc = "Counter Underflow Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CUFOA {
    #[doc = "0: No action on counter underflow"]
    None = 0,
    #[doc = "1: Toggle output on counter underflow"]
    Toggle = 1,
    #[doc = "2: Clear output on counter underflow"]
    Clear = 2,
    #[doc = "3: Set output on counter underflow"]
    Set = 3,
}
impl From<CUFOA> for u8 {
    #[inline(always)]
    fn from(variant: CUFOA) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CUFOA {
    type Ux = u8;
}
impl crate::IsEnum for CUFOA {}
#[doc = "Field `CUFOA` reader - Counter Underflow Output Action"]
pub type CufoaR = crate::FieldReader<CUFOA>;
impl CufoaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CUFOA {
        match self.bits {
            0 => CUFOA::None,
            1 => CUFOA::Toggle,
            2 => CUFOA::Clear,
            3 => CUFOA::Set,
            _ => unreachable!(),
        }
    }
    #[doc = "No action on counter underflow"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CUFOA::None
    }
    #[doc = "Toggle output on counter underflow"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == CUFOA::Toggle
    }
    #[doc = "Clear output on counter underflow"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CUFOA::Clear
    }
    #[doc = "Set output on counter underflow"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CUFOA::Set
    }
}
#[doc = "Field `CUFOA` writer - Counter Underflow Output Action"]
pub type CufoaW<'a, REG> = crate::FieldWriter<'a, REG, 2, CUFOA, crate::Safe>;
impl<'a, REG> CufoaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action on counter underflow"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CUFOA::None)
    }
    #[doc = "Toggle output on counter underflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(CUFOA::Toggle)
    }
    #[doc = "Clear output on counter underflow"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CUFOA::Clear)
    }
    #[doc = "Set output on counter underflow"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CUFOA::Set)
    }
}
#[doc = "Compare/Capture Channel PRS Input Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL {
    #[doc = "0: PRS Channel 0 selected as input"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    Prsch11 = 11,
}
impl From<PRSSEL> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSEL {
    type Ux = u8;
}
impl crate::IsEnum for PRSSEL {}
#[doc = "Field `PRSSEL` reader - Compare/Capture Channel PRS Input Channel Selection"]
pub type PrsselR = crate::FieldReader<PRSSEL>;
impl PrsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSSEL> {
        match self.bits {
            0 => Some(PRSSEL::Prsch0),
            1 => Some(PRSSEL::Prsch1),
            2 => Some(PRSSEL::Prsch2),
            3 => Some(PRSSEL::Prsch3),
            4 => Some(PRSSEL::Prsch4),
            5 => Some(PRSSEL::Prsch5),
            6 => Some(PRSSEL::Prsch6),
            7 => Some(PRSSEL::Prsch7),
            8 => Some(PRSSEL::Prsch8),
            9 => Some(PRSSEL::Prsch9),
            10 => Some(PRSSEL::Prsch10),
            11 => Some(PRSSEL::Prsch11),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL::Prsch0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL::Prsch1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL::Prsch2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL::Prsch3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL::Prsch4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL::Prsch5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL::Prsch6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL::Prsch7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL::Prsch8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL::Prsch9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL::Prsch10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL::Prsch11
    }
}
#[doc = "Field `PRSSEL` writer - Compare/Capture Channel PRS Input Channel Selection"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, PRSSEL>;
impl<'a, REG> PrsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch11)
    }
}
#[doc = "Input Capture Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICEDGE {
    #[doc = "0: Rising edges detected"]
    Rising = 0,
    #[doc = "1: Falling edges detected"]
    Falling = 1,
    #[doc = "2: Both edges detected"]
    Both = 2,
    #[doc = "3: No edge detection, signal is left as it is"]
    None = 3,
}
impl From<ICEDGE> for u8 {
    #[inline(always)]
    fn from(variant: ICEDGE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ICEDGE {
    type Ux = u8;
}
impl crate::IsEnum for ICEDGE {}
#[doc = "Field `ICEDGE` reader - Input Capture Edge Select"]
pub type IcedgeR = crate::FieldReader<ICEDGE>;
impl IcedgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICEDGE {
        match self.bits {
            0 => ICEDGE::Rising,
            1 => ICEDGE::Falling,
            2 => ICEDGE::Both,
            3 => ICEDGE::None,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ICEDGE::Rising
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ICEDGE::Falling
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == ICEDGE::Both
    }
    #[doc = "No edge detection, signal is left as it is"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ICEDGE::None
    }
}
#[doc = "Field `ICEDGE` writer - Input Capture Edge Select"]
pub type IcedgeW<'a, REG> = crate::FieldWriter<'a, REG, 2, ICEDGE, crate::Safe>;
impl<'a, REG> IcedgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(ICEDGE::Rising)
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(ICEDGE::Falling)
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(ICEDGE::Both)
    }
    #[doc = "No edge detection, signal is left as it is"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(ICEDGE::None)
    }
}
#[doc = "Input Capture Event Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICEVCTRL {
    #[doc = "0: PRS output pulse and interrupt flag set on every capture"]
    Everyedge = 0,
    #[doc = "1: PRS output pulse and interrupt flag set on every second capture"]
    Everysecondedge = 1,
    #[doc = "2: PRS output pulse and interrupt flag set on rising edge only (if ICEDGE = BOTH)"]
    Rising = 2,
    #[doc = "3: PRS output pulse and interrupt flag set on falling edge only (if ICEDGE = BOTH)"]
    Falling = 3,
}
impl From<ICEVCTRL> for u8 {
    #[inline(always)]
    fn from(variant: ICEVCTRL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ICEVCTRL {
    type Ux = u8;
}
impl crate::IsEnum for ICEVCTRL {}
#[doc = "Field `ICEVCTRL` reader - Input Capture Event Control"]
pub type IcevctrlR = crate::FieldReader<ICEVCTRL>;
impl IcevctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICEVCTRL {
        match self.bits {
            0 => ICEVCTRL::Everyedge,
            1 => ICEVCTRL::Everysecondedge,
            2 => ICEVCTRL::Rising,
            3 => ICEVCTRL::Falling,
            _ => unreachable!(),
        }
    }
    #[doc = "PRS output pulse and interrupt flag set on every capture"]
    #[inline(always)]
    pub fn is_everyedge(&self) -> bool {
        *self == ICEVCTRL::Everyedge
    }
    #[doc = "PRS output pulse and interrupt flag set on every second capture"]
    #[inline(always)]
    pub fn is_everysecondedge(&self) -> bool {
        *self == ICEVCTRL::Everysecondedge
    }
    #[doc = "PRS output pulse and interrupt flag set on rising edge only (if ICEDGE = BOTH)"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ICEVCTRL::Rising
    }
    #[doc = "PRS output pulse and interrupt flag set on falling edge only (if ICEDGE = BOTH)"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ICEVCTRL::Falling
    }
}
#[doc = "Field `ICEVCTRL` writer - Input Capture Event Control"]
pub type IcevctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2, ICEVCTRL, crate::Safe>;
impl<'a, REG> IcevctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS output pulse and interrupt flag set on every capture"]
    #[inline(always)]
    pub fn everyedge(self) -> &'a mut crate::W<REG> {
        self.variant(ICEVCTRL::Everyedge)
    }
    #[doc = "PRS output pulse and interrupt flag set on every second capture"]
    #[inline(always)]
    pub fn everysecondedge(self) -> &'a mut crate::W<REG> {
        self.variant(ICEVCTRL::Everysecondedge)
    }
    #[doc = "PRS output pulse and interrupt flag set on rising edge only (if ICEDGE = BOTH)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(ICEVCTRL::Rising)
    }
    #[doc = "PRS output pulse and interrupt flag set on falling edge only (if ICEDGE = BOTH)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(ICEVCTRL::Falling)
    }
}
#[doc = "Field `PRSCONF` reader - PRS Configuration"]
pub type PrsconfR = crate::BitReader;
#[doc = "Field `PRSCONF` writer - PRS Configuration"]
pub type PrsconfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSEL` reader - Input Selection"]
pub type InselR = crate::BitReader;
#[doc = "Field `INSEL` writer - Input Selection"]
pub type InselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILT` reader - Digital Filter"]
pub type FiltR = crate::BitReader;
#[doc = "Field `FILT` writer - Digital Filter"]
pub type FiltW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output Invert"]
    #[inline(always)]
    pub fn outinv(&self) -> OutinvR {
        OutinvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare Output Initial State"]
    #[inline(always)]
    pub fn coist(&self) -> CoistR {
        CoistR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Compare Match Output Action"]
    #[inline(always)]
    pub fn cmoa(&self) -> CmoaR {
        CmoaR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Counter Overflow Output Action"]
    #[inline(always)]
    pub fn cofoa(&self) -> CofoaR {
        CofoaR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Counter Underflow Output Action"]
    #[inline(always)]
    pub fn cufoa(&self) -> CufoaR {
        CufoaR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Compare/Capture Channel PRS Input Channel Selection"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn icedge(&self) -> IcedgeR {
        IcedgeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Input Capture Event Control"]
    #[inline(always)]
    pub fn icevctrl(&self) -> IcevctrlR {
        IcevctrlR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - PRS Configuration"]
    #[inline(always)]
    pub fn prsconf(&self) -> PrsconfR {
        PrsconfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Input Selection"]
    #[inline(always)]
    pub fn insel(&self) -> InselR {
        InselR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Digital Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FiltR {
        FiltR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CC1_CTRLrs> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 2 - Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn outinv(&mut self) -> OutinvW<CC1_CTRLrs> {
        OutinvW::new(self, 2)
    }
    #[doc = "Bit 4 - Compare Output Initial State"]
    #[inline(always)]
    #[must_use]
    pub fn coist(&mut self) -> CoistW<CC1_CTRLrs> {
        CoistW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Compare Match Output Action"]
    #[inline(always)]
    #[must_use]
    pub fn cmoa(&mut self) -> CmoaW<CC1_CTRLrs> {
        CmoaW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Counter Overflow Output Action"]
    #[inline(always)]
    #[must_use]
    pub fn cofoa(&mut self) -> CofoaW<CC1_CTRLrs> {
        CofoaW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Counter Underflow Output Action"]
    #[inline(always)]
    #[must_use]
    pub fn cufoa(&mut self) -> CufoaW<CC1_CTRLrs> {
        CufoaW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Compare/Capture Channel PRS Input Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PrsselW<CC1_CTRLrs> {
        PrsselW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Input Capture Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn icedge(&mut self) -> IcedgeW<CC1_CTRLrs> {
        IcedgeW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Input Capture Event Control"]
    #[inline(always)]
    #[must_use]
    pub fn icevctrl(&mut self) -> IcevctrlW<CC1_CTRLrs> {
        IcevctrlW::new(self, 26)
    }
    #[doc = "Bit 28 - PRS Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prsconf(&mut self) -> PrsconfW<CC1_CTRLrs> {
        PrsconfW::new(self, 28)
    }
    #[doc = "Bit 29 - Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn insel(&mut self) -> InselW<CC1_CTRLrs> {
        InselW::new(self, 29)
    }
    #[doc = "Bit 30 - Digital Filter"]
    #[inline(always)]
    #[must_use]
    pub fn filt(&mut self) -> FiltW<CC1_CTRLrs> {
        FiltW::new(self, 30)
    }
}
#[doc = "CC Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc1_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc1_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC1_CTRLrs;
impl crate::RegisterSpec for CC1_CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc1_ctrl::R`](R) reader structure"]
impl crate::Readable for CC1_CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`cc1_ctrl::W`](W) writer structure"]
impl crate::Writable for CC1_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC1_CTRL to value 0"]
impl crate::Resettable for CC1_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
