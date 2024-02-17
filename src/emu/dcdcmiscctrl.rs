#[doc = "Register `DCDCMISCCTRL` reader"]
pub type R = crate::R<DCDCMISCCTRLrs>;
#[doc = "Register `DCDCMISCCTRL` writer"]
pub type W = crate::W<DCDCMISCCTRLrs>;
#[doc = "Field `LNFORCECCM` reader - Force DCDC Into CCM Mode in Low Noise Operation"]
pub type LNFORCECCM_R = crate::BitReader;
#[doc = "Field `LNFORCECCM` writer - Force DCDC Into CCM Mode in Low Noise Operation"]
pub type LNFORCECCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFETCNT` reader - PFET Switch Number Selection"]
pub type PFETCNT_R = crate::FieldReader;
#[doc = "Field `PFETCNT` writer - PFET Switch Number Selection"]
pub type PFETCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NFETCNT` reader - NFET Switch Number Selection"]
pub type NFETCNT_R = crate::FieldReader;
#[doc = "Field `NFETCNT` writer - NFET Switch Number Selection"]
pub type NFETCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BYPLIMSEL` reader - Current Limit in Bypass Mode"]
pub type BYPLIMSEL_R = crate::FieldReader;
#[doc = "Field `BYPLIMSEL` writer - Current Limit in Bypass Mode"]
pub type BYPLIMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPCLIMILIMSEL` reader - Current Limit Level Selection for Current Limiter in LP Mode"]
pub type LPCLIMILIMSEL_R = crate::FieldReader;
#[doc = "Field `LPCLIMILIMSEL` writer - Current Limit Level Selection for Current Limiter in LP Mode"]
pub type LPCLIMILIMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LNCLIMILIMSEL` reader - Current Limit Level Selection for Current Limiter in LN Mode"]
pub type LNCLIMILIMSEL_R = crate::FieldReader;
#[doc = "Field `LNCLIMILIMSEL` writer - Current Limit Level Selection for Current Limiter in LN Mode"]
pub type LNCLIMILIMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LPCMPBIAS` reader - LP Mode Comparator Bias Selection"]
pub type LPCMPBIAS_R = crate::FieldReader<LPCMPBIAS>;
#[doc = "LP Mode Comparator Bias Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPCMPBIAS {
    #[doc = "0: Maximum load current less than 75uA."]
    Bias0 = 0,
    #[doc = "1: Maximum load current less than 500uA."]
    Bias1 = 1,
    #[doc = "2: Maximum load current less than 2.5mA."]
    Bias2 = 2,
    #[doc = "3: Maximum load current less than 10mA."]
    Bias3 = 3,
}
impl From<LPCMPBIAS> for u8 {
    #[inline(always)]
    fn from(variant: LPCMPBIAS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPCMPBIAS {
    type Ux = u8;
}
impl LPCMPBIAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPCMPBIAS {
        match self.bits {
            0 => LPCMPBIAS::Bias0,
            1 => LPCMPBIAS::Bias1,
            2 => LPCMPBIAS::Bias2,
            3 => LPCMPBIAS::Bias3,
            _ => unreachable!(),
        }
    }
    #[doc = "Maximum load current less than 75uA."]
    #[inline(always)]
    pub fn is_bias0(&self) -> bool {
        *self == LPCMPBIAS::Bias0
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline(always)]
    pub fn is_bias1(&self) -> bool {
        *self == LPCMPBIAS::Bias1
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline(always)]
    pub fn is_bias2(&self) -> bool {
        *self == LPCMPBIAS::Bias2
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline(always)]
    pub fn is_bias3(&self) -> bool {
        *self == LPCMPBIAS::Bias3
    }
}
#[doc = "Field `LPCMPBIAS` writer - LP Mode Comparator Bias Selection"]
pub type LPCMPBIAS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LPCMPBIAS>;
impl<'a, REG> LPCMPBIAS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum load current less than 75uA."]
    #[inline(always)]
    pub fn bias0(self) -> &'a mut crate::W<REG> {
        self.variant(LPCMPBIAS::Bias0)
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline(always)]
    pub fn bias1(self) -> &'a mut crate::W<REG> {
        self.variant(LPCMPBIAS::Bias1)
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline(always)]
    pub fn bias2(self) -> &'a mut crate::W<REG> {
        self.variant(LPCMPBIAS::Bias2)
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline(always)]
    pub fn bias3(self) -> &'a mut crate::W<REG> {
        self.variant(LPCMPBIAS::Bias3)
    }
}
impl R {
    #[doc = "Bit 0 - Force DCDC Into CCM Mode in Low Noise Operation"]
    #[inline(always)]
    pub fn lnforceccm(&self) -> LNFORCECCM_R {
        LNFORCECCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline(always)]
    pub fn pfetcnt(&self) -> PFETCNT_R {
        PFETCNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline(always)]
    pub fn nfetcnt(&self) -> NFETCNT_R {
        NFETCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline(always)]
    pub fn byplimsel(&self) -> BYPLIMSEL_R {
        BYPLIMSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline(always)]
    pub fn lpclimilimsel(&self) -> LPCLIMILIMSEL_R {
        LPCLIMILIMSEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline(always)]
    pub fn lnclimilimsel(&self) -> LNCLIMILIMSEL_R {
        LNCLIMILIMSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection"]
    #[inline(always)]
    pub fn lpcmpbias(&self) -> LPCMPBIAS_R {
        LPCMPBIAS_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Force DCDC Into CCM Mode in Low Noise Operation"]
    #[inline(always)]
    #[must_use]
    pub fn lnforceccm(&mut self) -> LNFORCECCM_W<DCDCMISCCTRLrs> {
        LNFORCECCM_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pfetcnt(&mut self) -> PFETCNT_W<DCDCMISCCTRLrs> {
        PFETCNT_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline(always)]
    #[must_use]
    pub fn nfetcnt(&mut self) -> NFETCNT_W<DCDCMISCCTRLrs> {
        NFETCNT_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline(always)]
    #[must_use]
    pub fn byplimsel(&mut self) -> BYPLIMSEL_W<DCDCMISCCTRLrs> {
        BYPLIMSEL_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpclimilimsel(&mut self) -> LPCLIMILIMSEL_W<DCDCMISCCTRLrs> {
        LPCLIMILIMSEL_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lnclimilimsel(&mut self) -> LNCLIMILIMSEL_W<DCDCMISCCTRLrs> {
        LNCLIMILIMSEL_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpcmpbias(&mut self) -> LPCMPBIAS_W<DCDCMISCCTRLrs> {
        LPCMPBIAS_W::new(self, 28)
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
#[doc = "DCDC Miscellaneous Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdcmiscctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdcmiscctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDCMISCCTRLrs;
impl crate::RegisterSpec for DCDCMISCCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdcmiscctrl::R`](R) reader structure"]
impl crate::Readable for DCDCMISCCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`dcdcmiscctrl::W`](W) writer structure"]
impl crate::Writable for DCDCMISCCTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDCMISCCTRL to value 0x3330_7700"]
impl crate::Resettable for DCDCMISCCTRLrs {
    const RESET_VALUE: u32 = 0x3330_7700;
}
