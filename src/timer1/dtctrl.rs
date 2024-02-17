#[doc = "Register `DTCTRL` reader"]
pub type R = crate::R<DTCTRLrs>;
#[doc = "Register `DTCTRL` writer"]
pub type W = crate::W<DTCTRLrs>;
#[doc = "Field `DTEN` reader - DTI Enable"]
pub type DTEN_R = crate::BitReader;
#[doc = "Field `DTEN` writer - DTI Enable"]
pub type DTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTDAS` reader - DTI Automatic Start-up Functionality"]
pub type DTDAS_R = crate::BitReader;
#[doc = "Field `DTDAS` writer - DTI Automatic Start-up Functionality"]
pub type DTDAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIPOL` reader - DTI Inactive Polarity"]
pub type DTIPOL_R = crate::BitReader;
#[doc = "Field `DTIPOL` writer - DTI Inactive Polarity"]
pub type DTIPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCINV` reader - DTI Complementary Output Invert"]
pub type DTCINV_R = crate::BitReader;
#[doc = "Field `DTCINV` writer - DTI Complementary Output Invert"]
pub type DTCINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTPRSSEL` reader - DTI PRS Source Channel Select"]
pub type DTPRSSEL_R = crate::FieldReader<DTPRSSEL>;
#[doc = "DTI PRS Source Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTPRSSEL {
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
impl From<DTPRSSEL> for u8 {
    #[inline(always)]
    fn from(variant: DTPRSSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTPRSSEL {
    type Ux = u8;
}
impl DTPRSSEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRSSEL::Prsch0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRSSEL::Prsch1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRSSEL::Prsch2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRSSEL::Prsch3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRSSEL::Prsch4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRSSEL::Prsch5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRSSEL::Prsch6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRSSEL::Prsch7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == DTPRSSEL::Prsch8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == DTPRSSEL::Prsch9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == DTPRSSEL::Prsch10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == DTPRSSEL::Prsch11
    }
}
#[doc = "Field `DTPRSSEL` writer - DTI PRS Source Channel Select"]
pub type DTPRSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DTPRSSEL>;
impl<'a, REG> DTPRSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRSSEL::Prsch11)
    }
}
#[doc = "Field `DTAR` reader - DTI Always Run"]
pub type DTAR_R = crate::BitReader;
#[doc = "Field `DTAR` writer - DTI Always Run"]
pub type DTAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFATS` reader - DTI Fault Action on Timer Stop"]
pub type DTFATS_R = crate::BitReader;
#[doc = "Field `DTFATS` writer - DTI Fault Action on Timer Stop"]
pub type DTFATS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTPRSEN` reader - DTI PRS Source Enable"]
pub type DTPRSEN_R = crate::BitReader;
#[doc = "Field `DTPRSEN` writer - DTI PRS Source Enable"]
pub type DTPRSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    pub fn dtdas(&self) -> DTDAS_R {
        DTDAS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DTI Inactive Polarity"]
    #[inline(always)]
    pub fn dtipol(&self) -> DTIPOL_R {
        DTIPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTI Complementary Output Invert"]
    #[inline(always)]
    pub fn dtcinv(&self) -> DTCINV_R {
        DTCINV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - DTI PRS Source Channel Select"]
    #[inline(always)]
    pub fn dtprssel(&self) -> DTPRSSEL_R {
        DTPRSSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - DTI Always Run"]
    #[inline(always)]
    pub fn dtar(&self) -> DTAR_R {
        DTAR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DTI Fault Action on Timer Stop"]
    #[inline(always)]
    pub fn dtfats(&self) -> DTFATS_R {
        DTFATS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 24 - DTI PRS Source Enable"]
    #[inline(always)]
    pub fn dtprsen(&self) -> DTPRSEN_R {
        DTPRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<DTCTRLrs> {
        DTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    #[must_use]
    pub fn dtdas(&mut self) -> DTDAS_W<DTCTRLrs> {
        DTDAS_W::new(self, 1)
    }
    #[doc = "Bit 2 - DTI Inactive Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dtipol(&mut self) -> DTIPOL_W<DTCTRLrs> {
        DTIPOL_W::new(self, 2)
    }
    #[doc = "Bit 3 - DTI Complementary Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn dtcinv(&mut self) -> DTCINV_W<DTCTRLrs> {
        DTCINV_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - DTI PRS Source Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn dtprssel(&mut self) -> DTPRSSEL_W<DTCTRLrs> {
        DTPRSSEL_W::new(self, 4)
    }
    #[doc = "Bit 9 - DTI Always Run"]
    #[inline(always)]
    #[must_use]
    pub fn dtar(&mut self) -> DTAR_W<DTCTRLrs> {
        DTAR_W::new(self, 9)
    }
    #[doc = "Bit 10 - DTI Fault Action on Timer Stop"]
    #[inline(always)]
    #[must_use]
    pub fn dtfats(&mut self) -> DTFATS_W<DTCTRLrs> {
        DTFATS_W::new(self, 10)
    }
    #[doc = "Bit 24 - DTI PRS Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtprsen(&mut self) -> DTPRSEN_W<DTCTRLrs> {
        DTPRSEN_W::new(self, 24)
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
#[doc = "DTI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTCTRLrs;
impl crate::RegisterSpec for DTCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtctrl::R`](R) reader structure"]
impl crate::Readable for DTCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`dtctrl::W`](W) writer structure"]
impl crate::Writable for DTCTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTCTRL to value 0"]
impl crate::Resettable for DTCTRLrs {
    const RESET_VALUE: u32 = 0;
}
