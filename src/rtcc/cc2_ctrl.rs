///Register `CC2_CTRL` reader
pub type R = crate::R<CC2_CTRLrs>;
///Register `CC2_CTRL` writer
pub type W = crate::W<CC2_CTRLrs>;
///CC Channel Mode
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    ///0: Compare/Capture channel turned off
    Off = 0,
    ///1: Input capture
    Inputcapture = 1,
    ///2: Output compare
    Outputcompare = 2,
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
///Field `MODE` reader - CC Channel Mode
pub type ModeR = crate::FieldReader<MODE>;
impl ModeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE> {
        match self.bits {
            0 => Some(MODE::Off),
            1 => Some(MODE::Inputcapture),
            2 => Some(MODE::Outputcompare),
            _ => None,
        }
    }
    ///Compare/Capture channel turned off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MODE::Off
    }
    ///Input capture
    #[inline(always)]
    pub fn is_inputcapture(&self) -> bool {
        *self == MODE::Inputcapture
    }
    ///Output compare
    #[inline(always)]
    pub fn is_outputcompare(&self) -> bool {
        *self == MODE::Outputcompare
    }
}
///Field `MODE` writer - CC Channel Mode
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Compare/Capture channel turned off
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Off)
    }
    ///Input capture
    #[inline(always)]
    pub fn inputcapture(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Inputcapture)
    }
    ///Output compare
    #[inline(always)]
    pub fn outputcompare(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Outputcompare)
    }
}
///Compare Match Output Action
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMOA {
    ///0: A single clock cycle pulse is generated on output
    Pulse = 0,
    ///1: Toggle output on compare match
    Toggle = 1,
    ///2: Clear output on compare match
    Clear = 2,
    ///3: Set output on compare match
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
///Field `CMOA` reader - Compare Match Output Action
pub type CmoaR = crate::FieldReader<CMOA>;
impl CmoaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMOA {
        match self.bits {
            0 => CMOA::Pulse,
            1 => CMOA::Toggle,
            2 => CMOA::Clear,
            3 => CMOA::Set,
            _ => unreachable!(),
        }
    }
    ///A single clock cycle pulse is generated on output
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == CMOA::Pulse
    }
    ///Toggle output on compare match
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == CMOA::Toggle
    }
    ///Clear output on compare match
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CMOA::Clear
    }
    ///Set output on compare match
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMOA::Set
    }
}
///Field `CMOA` writer - Compare Match Output Action
pub type CmoaW<'a, REG> = crate::FieldWriter<'a, REG, 2, CMOA, crate::Safe>;
impl<'a, REG> CmoaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///A single clock cycle pulse is generated on output
    #[inline(always)]
    pub fn pulse(self) -> &'a mut crate::W<REG> {
        self.variant(CMOA::Pulse)
    }
    ///Toggle output on compare match
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(CMOA::Toggle)
    }
    ///Clear output on compare match
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CMOA::Clear)
    }
    ///Set output on compare match
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CMOA::Set)
    }
}
///Input Capture Edge Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICEDGE {
    ///0: Rising edges detected
    Rising = 0,
    ///1: Falling edges detected
    Falling = 1,
    ///2: Both edges detected
    Both = 2,
    ///3: No edge detection, signal is left as it is
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
///Field `ICEDGE` reader - Input Capture Edge Select
pub type IcedgeR = crate::FieldReader<ICEDGE>;
impl IcedgeR {
    ///Get enumerated values variant
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
    ///Rising edges detected
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ICEDGE::Rising
    }
    ///Falling edges detected
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ICEDGE::Falling
    }
    ///Both edges detected
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == ICEDGE::Both
    }
    ///No edge detection, signal is left as it is
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ICEDGE::None
    }
}
///Field `ICEDGE` writer - Input Capture Edge Select
pub type IcedgeW<'a, REG> = crate::FieldWriter<'a, REG, 2, ICEDGE, crate::Safe>;
impl<'a, REG> IcedgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Rising edges detected
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(ICEDGE::Rising)
    }
    ///Falling edges detected
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(ICEDGE::Falling)
    }
    ///Both edges detected
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(ICEDGE::Both)
    }
    ///No edge detection, signal is left as it is
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(ICEDGE::None)
    }
}
///Compare/Capture Channel PRS Input Channel Selection
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL {
    ///0: PRS Channel 0 selected as input
    Prsch0 = 0,
    ///1: PRS Channel 1 selected as input
    Prsch1 = 1,
    ///2: PRS Channel 2 selected as input
    Prsch2 = 2,
    ///3: PRS Channel 3 selected as input
    Prsch3 = 3,
    ///4: PRS Channel 4 selected as input
    Prsch4 = 4,
    ///5: PRS Channel 5 selected as input
    Prsch5 = 5,
    ///6: PRS Channel 6 selected as input
    Prsch6 = 6,
    ///7: PRS Channel 7 selected as input
    Prsch7 = 7,
    ///8: PRS Channel 8 selected as input
    Prsch8 = 8,
    ///9: PRS Channel 9 selected as input
    Prsch9 = 9,
    ///10: PRS Channel 10 selected as input
    Prsch10 = 10,
    ///11: PRS Channel 11 selected as input
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
///Field `PRSSEL` reader - Compare/Capture Channel PRS Input Channel Selection
pub type PrsselR = crate::FieldReader<PRSSEL>;
impl PrsselR {
    ///Get enumerated values variant
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
    ///PRS Channel 0 selected as input
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL::Prsch0
    }
    ///PRS Channel 1 selected as input
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL::Prsch1
    }
    ///PRS Channel 2 selected as input
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL::Prsch2
    }
    ///PRS Channel 3 selected as input
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL::Prsch3
    }
    ///PRS Channel 4 selected as input
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL::Prsch4
    }
    ///PRS Channel 5 selected as input
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL::Prsch5
    }
    ///PRS Channel 6 selected as input
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL::Prsch6
    }
    ///PRS Channel 7 selected as input
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL::Prsch7
    }
    ///PRS Channel 8 selected as input
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL::Prsch8
    }
    ///PRS Channel 9 selected as input
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL::Prsch9
    }
    ///PRS Channel 10 selected as input
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL::Prsch10
    }
    ///PRS Channel 11 selected as input
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL::Prsch11
    }
}
///Field `PRSSEL` writer - Compare/Capture Channel PRS Input Channel Selection
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, PRSSEL>;
impl<'a, REG> PrsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected as input
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch0)
    }
    ///PRS Channel 1 selected as input
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch1)
    }
    ///PRS Channel 2 selected as input
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch2)
    }
    ///PRS Channel 3 selected as input
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch3)
    }
    ///PRS Channel 4 selected as input
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch4)
    }
    ///PRS Channel 5 selected as input
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch5)
    }
    ///PRS Channel 6 selected as input
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch6)
    }
    ///PRS Channel 7 selected as input
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch7)
    }
    ///PRS Channel 8 selected as input
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch8)
    }
    ///PRS Channel 9 selected as input
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch9)
    }
    ///PRS Channel 10 selected as input
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch10)
    }
    ///PRS Channel 11 selected as input
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch11)
    }
}
///Field `COMPBASE` reader - Capture Compare Channel Comparison Base
pub type CompbaseR = crate::BitReader;
///Field `COMPBASE` writer - Capture Compare Channel Comparison Base
pub type CompbaseW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMPMASK` reader - Capture Compare Channel Comparison Mask
pub type CompmaskR = crate::FieldReader;
///Field `COMPMASK` writer - Capture Compare Channel Comparison Mask
pub type CompmaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DAYCC` reader - Day Capture/Compare Selection
pub type DayccR = crate::BitReader;
///Field `DAYCC` writer - Day Capture/Compare Selection
pub type DayccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - CC Channel Mode
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Compare Match Output Action
    #[inline(always)]
    pub fn cmoa(&self) -> CmoaR {
        CmoaR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Input Capture Edge Select
    #[inline(always)]
    pub fn icedge(&self) -> IcedgeR {
        IcedgeR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:9 - Compare/Capture Channel PRS Input Channel Selection
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    ///Bit 11 - Capture Compare Channel Comparison Base
    #[inline(always)]
    pub fn compbase(&self) -> CompbaseR {
        CompbaseR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:16 - Capture Compare Channel Comparison Mask
    #[inline(always)]
    pub fn compmask(&self) -> CompmaskR {
        CompmaskR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    ///Bit 17 - Day Capture/Compare Selection
    #[inline(always)]
    pub fn daycc(&self) -> DayccR {
        DayccR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CC2_CTRL")
            .field("mode", &self.mode())
            .field("cmoa", &self.cmoa())
            .field("icedge", &self.icedge())
            .field("prssel", &self.prssel())
            .field("compbase", &self.compbase())
            .field("compmask", &self.compmask())
            .field("daycc", &self.daycc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - CC Channel Mode
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CC2_CTRLrs> {
        ModeW::new(self, 0)
    }
    ///Bits 2:3 - Compare Match Output Action
    #[inline(always)]
    #[must_use]
    pub fn cmoa(&mut self) -> CmoaW<CC2_CTRLrs> {
        CmoaW::new(self, 2)
    }
    ///Bits 4:5 - Input Capture Edge Select
    #[inline(always)]
    #[must_use]
    pub fn icedge(&mut self) -> IcedgeW<CC2_CTRLrs> {
        IcedgeW::new(self, 4)
    }
    ///Bits 6:9 - Compare/Capture Channel PRS Input Channel Selection
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PrsselW<CC2_CTRLrs> {
        PrsselW::new(self, 6)
    }
    ///Bit 11 - Capture Compare Channel Comparison Base
    #[inline(always)]
    #[must_use]
    pub fn compbase(&mut self) -> CompbaseW<CC2_CTRLrs> {
        CompbaseW::new(self, 11)
    }
    ///Bits 12:16 - Capture Compare Channel Comparison Mask
    #[inline(always)]
    #[must_use]
    pub fn compmask(&mut self) -> CompmaskW<CC2_CTRLrs> {
        CompmaskW::new(self, 12)
    }
    ///Bit 17 - Day Capture/Compare Selection
    #[inline(always)]
    #[must_use]
    pub fn daycc(&mut self) -> DayccW<CC2_CTRLrs> {
        DayccW::new(self, 17)
    }
}
///CC Channel Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`cc2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CC2_CTRLrs;
impl crate::RegisterSpec for CC2_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`cc2_ctrl::R`](R) reader structure
impl crate::Readable for CC2_CTRLrs {}
///`write(|w| ..)` method takes [`cc2_ctrl::W`](W) writer structure
impl crate::Writable for CC2_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CC2_CTRL to value 0
impl crate::Resettable for CC2_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
