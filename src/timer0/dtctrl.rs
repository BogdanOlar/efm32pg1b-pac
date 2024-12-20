///Register `DTCTRL` reader
pub type R = crate::R<DTCTRLrs>;
///Register `DTCTRL` writer
pub type W = crate::W<DTCTRLrs>;
///Field `DTEN` reader - DTI Enable
pub type DtenR = crate::BitReader;
///Field `DTEN` writer - DTI Enable
pub type DtenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTDAS` reader - DTI Automatic Start-up Functionality
pub type DtdasR = crate::BitReader;
///Field `DTDAS` writer - DTI Automatic Start-up Functionality
pub type DtdasW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTIPOL` reader - DTI Inactive Polarity
pub type DtipolR = crate::BitReader;
///Field `DTIPOL` writer - DTI Inactive Polarity
pub type DtipolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTCINV` reader - DTI Complementary Output Invert
pub type DtcinvR = crate::BitReader;
///Field `DTCINV` writer - DTI Complementary Output Invert
pub type DtcinvW<'a, REG> = crate::BitWriter<'a, REG>;
///DTI PRS Source Channel Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTPRSSEL {
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
impl From<DTPRSSEL> for u8 {
    #[inline(always)]
    fn from(variant: DTPRSSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTPRSSEL {
    type Ux = u8;
}
impl crate::IsEnum for DTPRSSEL {}
///Field `DTPRSSEL` reader - DTI PRS Source Channel Select
pub type DtprsselR = crate::FieldReader<DTPRSSEL>;
impl DtprsselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DTPRSSEL> {
        match self.bits {
            0 => Some(DTPRSSEL::Prsch0),
            1 => Some(DTPRSSEL::Prsch1),
            2 => Some(DTPRSSEL::Prsch2),
            3 => Some(DTPRSSEL::Prsch3),
            4 => Some(DTPRSSEL::Prsch4),
            5 => Some(DTPRSSEL::Prsch5),
            6 => Some(DTPRSSEL::Prsch6),
            7 => Some(DTPRSSEL::Prsch7),
            8 => Some(DTPRSSEL::Prsch8),
            9 => Some(DTPRSSEL::Prsch9),
            10 => Some(DTPRSSEL::Prsch10),
            11 => Some(DTPRSSEL::Prsch11),
            _ => None,
        }
    }
    ///PRS Channel 0 selected as input
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRSSEL::Prsch0
    }
    ///PRS Channel 1 selected as input
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRSSEL::Prsch1
    }
    ///PRS Channel 2 selected as input
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRSSEL::Prsch2
    }
    ///PRS Channel 3 selected as input
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRSSEL::Prsch3
    }
    ///PRS Channel 4 selected as input
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRSSEL::Prsch4
    }
    ///PRS Channel 5 selected as input
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRSSEL::Prsch5
    }
    ///PRS Channel 6 selected as input
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRSSEL::Prsch6
    }
    ///PRS Channel 7 selected as input
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRSSEL::Prsch7
    }
    ///PRS Channel 8 selected as input
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == DTPRSSEL::Prsch8
    }
    ///PRS Channel 9 selected as input
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == DTPRSSEL::Prsch9
    }
    ///PRS Channel 10 selected as input
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == DTPRSSEL::Prsch10
    }
    ///PRS Channel 11 selected as input
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == DTPRSSEL::Prsch11
    }
}
///Field `DTPRSSEL` writer - DTI PRS Source Channel Select
pub type DtprsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, DTPRSSEL>;
impl<'a, REG> DtprsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected as input
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch0)
    }
    ///PRS Channel 1 selected as input
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch1)
    }
    ///PRS Channel 2 selected as input
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch2)
    }
    ///PRS Channel 3 selected as input
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch3)
    }
    ///PRS Channel 4 selected as input
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch4)
    }
    ///PRS Channel 5 selected as input
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch5)
    }
    ///PRS Channel 6 selected as input
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch6)
    }
    ///PRS Channel 7 selected as input
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch7)
    }
    ///PRS Channel 8 selected as input
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch8)
    }
    ///PRS Channel 9 selected as input
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch9)
    }
    ///PRS Channel 10 selected as input
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch10)
    }
    ///PRS Channel 11 selected as input
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch11)
    }
}
///Field `DTAR` reader - DTI Always Run
pub type DtarR = crate::BitReader;
///Field `DTAR` writer - DTI Always Run
pub type DtarW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTFATS` reader - DTI Fault Action on Timer Stop
pub type DtfatsR = crate::BitReader;
///Field `DTFATS` writer - DTI Fault Action on Timer Stop
pub type DtfatsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTPRSEN` reader - DTI PRS Source Enable
pub type DtprsenR = crate::BitReader;
///Field `DTPRSEN` writer - DTI PRS Source Enable
pub type DtprsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DTI Enable
    #[inline(always)]
    pub fn dten(&self) -> DtenR {
        DtenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DTI Automatic Start-up Functionality
    #[inline(always)]
    pub fn dtdas(&self) -> DtdasR {
        DtdasR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DTI Inactive Polarity
    #[inline(always)]
    pub fn dtipol(&self) -> DtipolR {
        DtipolR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DTI Complementary Output Invert
    #[inline(always)]
    pub fn dtcinv(&self) -> DtcinvR {
        DtcinvR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - DTI PRS Source Channel Select
    #[inline(always)]
    pub fn dtprssel(&self) -> DtprsselR {
        DtprsselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 9 - DTI Always Run
    #[inline(always)]
    pub fn dtar(&self) -> DtarR {
        DtarR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DTI Fault Action on Timer Stop
    #[inline(always)]
    pub fn dtfats(&self) -> DtfatsR {
        DtfatsR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 24 - DTI PRS Source Enable
    #[inline(always)]
    pub fn dtprsen(&self) -> DtprsenR {
        DtprsenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTCTRL")
            .field("dten", &self.dten())
            .field("dtdas", &self.dtdas())
            .field("dtipol", &self.dtipol())
            .field("dtcinv", &self.dtcinv())
            .field("dtprssel", &self.dtprssel())
            .field("dtar", &self.dtar())
            .field("dtfats", &self.dtfats())
            .field("dtprsen", &self.dtprsen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DTI Enable
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DtenW<DTCTRLrs> {
        DtenW::new(self, 0)
    }
    ///Bit 1 - DTI Automatic Start-up Functionality
    #[inline(always)]
    #[must_use]
    pub fn dtdas(&mut self) -> DtdasW<DTCTRLrs> {
        DtdasW::new(self, 1)
    }
    ///Bit 2 - DTI Inactive Polarity
    #[inline(always)]
    #[must_use]
    pub fn dtipol(&mut self) -> DtipolW<DTCTRLrs> {
        DtipolW::new(self, 2)
    }
    ///Bit 3 - DTI Complementary Output Invert
    #[inline(always)]
    #[must_use]
    pub fn dtcinv(&mut self) -> DtcinvW<DTCTRLrs> {
        DtcinvW::new(self, 3)
    }
    ///Bits 4:7 - DTI PRS Source Channel Select
    #[inline(always)]
    #[must_use]
    pub fn dtprssel(&mut self) -> DtprsselW<DTCTRLrs> {
        DtprsselW::new(self, 4)
    }
    ///Bit 9 - DTI Always Run
    #[inline(always)]
    #[must_use]
    pub fn dtar(&mut self) -> DtarW<DTCTRLrs> {
        DtarW::new(self, 9)
    }
    ///Bit 10 - DTI Fault Action on Timer Stop
    #[inline(always)]
    #[must_use]
    pub fn dtfats(&mut self) -> DtfatsW<DTCTRLrs> {
        DtfatsW::new(self, 10)
    }
    ///Bit 24 - DTI PRS Source Enable
    #[inline(always)]
    #[must_use]
    pub fn dtprsen(&mut self) -> DtprsenW<DTCTRLrs> {
        DtprsenW::new(self, 24)
    }
}
///DTI Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dtctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DTCTRLrs;
impl crate::RegisterSpec for DTCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`dtctrl::R`](R) reader structure
impl crate::Readable for DTCTRLrs {}
///`write(|w| ..)` method takes [`dtctrl::W`](W) writer structure
impl crate::Writable for DTCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DTCTRL to value 0
impl crate::Resettable for DTCTRLrs {
    const RESET_VALUE: u32 = 0;
}
