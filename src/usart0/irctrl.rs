///Register `IRCTRL` reader
pub type R = crate::R<IRCTRLrs>;
///Register `IRCTRL` writer
pub type W = crate::W<IRCTRLrs>;
///Field `IREN` reader - Enable IrDA Module
pub type IrenR = crate::BitReader;
///Field `IREN` writer - Enable IrDA Module
pub type IrenW<'a, REG> = crate::BitWriter<'a, REG>;
///IrDA TX Pulse Width
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IRPW {
    ///0: IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1
    One = 0,
    ///1: IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1
    Two = 1,
    ///2: IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1
    Three = 2,
    ///3: IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1
    Four = 3,
}
impl From<IRPW> for u8 {
    #[inline(always)]
    fn from(variant: IRPW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IRPW {
    type Ux = u8;
}
impl crate::IsEnum for IRPW {}
///Field `IRPW` reader - IrDA TX Pulse Width
pub type IrpwR = crate::FieldReader<IRPW>;
impl IrpwR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRPW {
        match self.bits {
            0 => IRPW::One,
            1 => IRPW::Two,
            2 => IRPW::Three,
            3 => IRPW::Four,
            _ => unreachable!(),
        }
    }
    ///IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == IRPW::One
    }
    ///IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == IRPW::Two
    }
    ///IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == IRPW::Three
    }
    ///IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == IRPW::Four
    }
}
///Field `IRPW` writer - IrDA TX Pulse Width
pub type IrpwW<'a, REG> = crate::FieldWriter<'a, REG, 2, IRPW, crate::Safe>;
impl<'a, REG> IrpwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(IRPW::One)
    }
    ///IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(IRPW::Two)
    }
    ///IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(IRPW::Three)
    }
    ///IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(IRPW::Four)
    }
}
///Field `IRFILT` reader - IrDA RX Filter
pub type IrfiltR = crate::BitReader;
///Field `IRFILT` writer - IrDA RX Filter
pub type IrfiltW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRPRSEN` reader - IrDA PRS Channel Enable
pub type IrprsenR = crate::BitReader;
///Field `IRPRSEN` writer - IrDA PRS Channel Enable
pub type IrprsenW<'a, REG> = crate::BitWriter<'a, REG>;
///IrDA PRS Channel Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IRPRSSEL {
    ///0: PRS Channel 0 selected
    Prsch0 = 0,
    ///1: PRS Channel 1 selected
    Prsch1 = 1,
    ///2: PRS Channel 2 selected
    Prsch2 = 2,
    ///3: PRS Channel 3 selected
    Prsch3 = 3,
    ///4: PRS Channel 4 selected
    Prsch4 = 4,
    ///5: PRS Channel 5 selected
    Prsch5 = 5,
    ///6: PRS Channel 6 selected
    Prsch6 = 6,
    ///7: PRS Channel 7 selected
    Prsch7 = 7,
    ///8: PRS Channel 8 selected
    Prsch8 = 8,
    ///9: PRS Channel 9 selected
    Prsch9 = 9,
    ///10: PRS Channel 10 selected
    Prsch10 = 10,
    ///11: PRS Channel 11 selected
    Prsch11 = 11,
}
impl From<IRPRSSEL> for u8 {
    #[inline(always)]
    fn from(variant: IRPRSSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IRPRSSEL {
    type Ux = u8;
}
impl crate::IsEnum for IRPRSSEL {}
///Field `IRPRSSEL` reader - IrDA PRS Channel Select
pub type IrprsselR = crate::FieldReader<IRPRSSEL>;
impl IrprsselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<IRPRSSEL> {
        match self.bits {
            0 => Some(IRPRSSEL::Prsch0),
            1 => Some(IRPRSSEL::Prsch1),
            2 => Some(IRPRSSEL::Prsch2),
            3 => Some(IRPRSSEL::Prsch3),
            4 => Some(IRPRSSEL::Prsch4),
            5 => Some(IRPRSSEL::Prsch5),
            6 => Some(IRPRSSEL::Prsch6),
            7 => Some(IRPRSSEL::Prsch7),
            8 => Some(IRPRSSEL::Prsch8),
            9 => Some(IRPRSSEL::Prsch9),
            10 => Some(IRPRSSEL::Prsch10),
            11 => Some(IRPRSSEL::Prsch11),
            _ => None,
        }
    }
    ///PRS Channel 0 selected
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == IRPRSSEL::Prsch0
    }
    ///PRS Channel 1 selected
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == IRPRSSEL::Prsch1
    }
    ///PRS Channel 2 selected
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == IRPRSSEL::Prsch2
    }
    ///PRS Channel 3 selected
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == IRPRSSEL::Prsch3
    }
    ///PRS Channel 4 selected
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == IRPRSSEL::Prsch4
    }
    ///PRS Channel 5 selected
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == IRPRSSEL::Prsch5
    }
    ///PRS Channel 6 selected
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == IRPRSSEL::Prsch6
    }
    ///PRS Channel 7 selected
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == IRPRSSEL::Prsch7
    }
    ///PRS Channel 8 selected
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == IRPRSSEL::Prsch8
    }
    ///PRS Channel 9 selected
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == IRPRSSEL::Prsch9
    }
    ///PRS Channel 10 selected
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == IRPRSSEL::Prsch10
    }
    ///PRS Channel 11 selected
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == IRPRSSEL::Prsch11
    }
}
///Field `IRPRSSEL` writer - IrDA PRS Channel Select
pub type IrprsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, IRPRSSEL>;
impl<'a, REG> IrprsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL::Prsch0)
    }
    ///PRS Channel 1 selected
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL::Prsch1)
    }
    ///PRS Channel 2 selected
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL::Prsch2)
    }
    ///PRS Channel 3 selected
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL::Prsch3)
    }
    ///PRS Channel 4 selected
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL::Prsch4)
    }
    ///PRS Channel 5 selected
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL::Prsch5)
    }
    ///PRS Channel 6 selected
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL::Prsch6)
    }
    ///PRS Channel 7 selected
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL::Prsch7)
    }
    ///PRS Channel 8 selected
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL::Prsch8)
    }
    ///PRS Channel 9 selected
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL::Prsch9)
    }
    ///PRS Channel 10 selected
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL::Prsch10)
    }
    ///PRS Channel 11 selected
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(IRPRSSEL::Prsch11)
    }
}
impl R {
    ///Bit 0 - Enable IrDA Module
    #[inline(always)]
    pub fn iren(&self) -> IrenR {
        IrenR::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - IrDA TX Pulse Width
    #[inline(always)]
    pub fn irpw(&self) -> IrpwR {
        IrpwR::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - IrDA RX Filter
    #[inline(always)]
    pub fn irfilt(&self) -> IrfiltR {
        IrfiltR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - IrDA PRS Channel Enable
    #[inline(always)]
    pub fn irprsen(&self) -> IrprsenR {
        IrprsenR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - IrDA PRS Channel Select
    #[inline(always)]
    pub fn irprssel(&self) -> IrprsselR {
        IrprsselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRCTRL")
            .field("iren", &self.iren())
            .field("irpw", &self.irpw())
            .field("irfilt", &self.irfilt())
            .field("irprsen", &self.irprsen())
            .field("irprssel", &self.irprssel())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable IrDA Module
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IrenW<IRCTRLrs> {
        IrenW::new(self, 0)
    }
    ///Bits 1:2 - IrDA TX Pulse Width
    #[inline(always)]
    #[must_use]
    pub fn irpw(&mut self) -> IrpwW<IRCTRLrs> {
        IrpwW::new(self, 1)
    }
    ///Bit 3 - IrDA RX Filter
    #[inline(always)]
    #[must_use]
    pub fn irfilt(&mut self) -> IrfiltW<IRCTRLrs> {
        IrfiltW::new(self, 3)
    }
    ///Bit 7 - IrDA PRS Channel Enable
    #[inline(always)]
    #[must_use]
    pub fn irprsen(&mut self) -> IrprsenW<IRCTRLrs> {
        IrprsenW::new(self, 7)
    }
    ///Bits 8:11 - IrDA PRS Channel Select
    #[inline(always)]
    #[must_use]
    pub fn irprssel(&mut self) -> IrprsselW<IRCTRLrs> {
        IrprsselW::new(self, 8)
    }
}
///IrDA Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`irctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IRCTRLrs;
impl crate::RegisterSpec for IRCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`irctrl::R`](R) reader structure
impl crate::Readable for IRCTRLrs {}
///`write(|w| ..)` method takes [`irctrl::W`](W) writer structure
impl crate::Writable for IRCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IRCTRL to value 0
impl crate::Resettable for IRCTRLrs {
    const RESET_VALUE: u32 = 0;
}
