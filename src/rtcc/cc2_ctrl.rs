#[doc = "Register `CC2_CTRL` reader"]
pub type R = crate::R<CC2_CTRLrs>;
#[doc = "Register `CC2_CTRL` writer"]
pub type W = crate::W<CC2_CTRLrs>;
#[doc = "Field `MODE` reader - CC Channel Mode"]
pub type MODE_R = crate::FieldReader<MODE>;
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
    pub const fn variant(&self) -> Option<MODE> {
        match self.bits {
            0 => Some(MODE::Off),
            1 => Some(MODE::Inputcapture),
            2 => Some(MODE::Outputcompare),
            _ => None,
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
}
#[doc = "Field `MODE` writer - CC Channel Mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE>;
impl<'a, REG> MODE_W<'a, REG>
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
}
#[doc = "Field `CMOA` reader - Compare Match Output Action"]
pub type CMOA_R = crate::FieldReader<CMOA>;
#[doc = "Compare Match Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMOA {
    #[doc = "0: A single clock cycle pulse is generated on output"]
    Pulse = 0,
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
impl CMOA_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "A single clock cycle pulse is generated on output"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == CMOA::Pulse
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
pub type CMOA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CMOA>;
impl<'a, REG> CMOA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A single clock cycle pulse is generated on output"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut crate::W<REG> {
        self.variant(CMOA::Pulse)
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
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(CMOA::Set)
    }
}
#[doc = "Field `ICEDGE` reader - Input Capture Edge Select"]
pub type ICEDGE_R = crate::FieldReader<ICEDGE>;
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
impl ICEDGE_R {
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
pub type ICEDGE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ICEDGE>;
impl<'a, REG> ICEDGE_W<'a, REG>
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
#[doc = "Field `PRSSEL` reader - Compare/Capture Channel PRS Input Channel Selection"]
pub type PRSSEL_R = crate::FieldReader<PRSSEL>;
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
impl PRSSEL_R {
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
pub type PRSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRSSEL>;
impl<'a, REG> PRSSEL_W<'a, REG>
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
#[doc = "Field `COMPBASE` reader - Capture Compare Channel Comparison Base"]
pub type COMPBASE_R = crate::BitReader;
#[doc = "Field `COMPBASE` writer - Capture Compare Channel Comparison Base"]
pub type COMPBASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPMASK` reader - Capture Compare Channel Comparison Mask"]
pub type COMPMASK_R = crate::FieldReader;
#[doc = "Field `COMPMASK` writer - Capture Compare Channel Comparison Mask"]
pub type COMPMASK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DAYCC` reader - Day Capture/Compare Selection"]
pub type DAYCC_R = crate::BitReader;
#[doc = "Field `DAYCC` writer - Day Capture/Compare Selection"]
pub type DAYCC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Compare Match Output Action"]
    #[inline(always)]
    pub fn cmoa(&self) -> CMOA_R {
        CMOA_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn icedge(&self) -> ICEDGE_R {
        ICEDGE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:9 - Compare/Capture Channel PRS Input Channel Selection"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - Capture Compare Channel Comparison Base"]
    #[inline(always)]
    pub fn compbase(&self) -> COMPBASE_R {
        COMPBASE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:16 - Capture Compare Channel Comparison Mask"]
    #[inline(always)]
    pub fn compmask(&self) -> COMPMASK_R {
        COMPMASK_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - Day Capture/Compare Selection"]
    #[inline(always)]
    pub fn daycc(&self) -> DAYCC_R {
        DAYCC_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CC2_CTRLrs> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Compare Match Output Action"]
    #[inline(always)]
    #[must_use]
    pub fn cmoa(&mut self) -> CMOA_W<CC2_CTRLrs> {
        CMOA_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Input Capture Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn icedge(&mut self) -> ICEDGE_W<CC2_CTRLrs> {
        ICEDGE_W::new(self, 4)
    }
    #[doc = "Bits 6:9 - Compare/Capture Channel PRS Input Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<CC2_CTRLrs> {
        PRSSEL_W::new(self, 6)
    }
    #[doc = "Bit 11 - Capture Compare Channel Comparison Base"]
    #[inline(always)]
    #[must_use]
    pub fn compbase(&mut self) -> COMPBASE_W<CC2_CTRLrs> {
        COMPBASE_W::new(self, 11)
    }
    #[doc = "Bits 12:16 - Capture Compare Channel Comparison Mask"]
    #[inline(always)]
    #[must_use]
    pub fn compmask(&mut self) -> COMPMASK_W<CC2_CTRLrs> {
        COMPMASK_W::new(self, 12)
    }
    #[doc = "Bit 17 - Day Capture/Compare Selection"]
    #[inline(always)]
    #[must_use]
    pub fn daycc(&mut self) -> DAYCC_W<CC2_CTRLrs> {
        DAYCC_W::new(self, 17)
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
#[doc = "CC Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc2_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc2_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC2_CTRLrs;
impl crate::RegisterSpec for CC2_CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2_ctrl::R`](R) reader structure"]
impl crate::Readable for CC2_CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`cc2_ctrl::W`](W) writer structure"]
impl crate::Writable for CC2_CTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC2_CTRL to value 0"]
impl crate::Resettable for CC2_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
