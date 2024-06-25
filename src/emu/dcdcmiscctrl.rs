#[doc = "Register `DCDCMISCCTRL` reader"]
pub type R = crate::R<DCDCMISCCTRLrs>;
#[doc = "Register `DCDCMISCCTRL` writer"]
pub type W = crate::W<DCDCMISCCTRLrs>;
#[doc = "Field `LNFORCECCM` reader - Force DCDC Into CCM Mode in Low Noise Operation"]
pub type LnforceccmR = crate::BitReader;
#[doc = "Field `LNFORCECCM` writer - Force DCDC Into CCM Mode in Low Noise Operation"]
pub type LnforceccmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFETCNT` reader - PFET Switch Number Selection"]
pub type PfetcntR = crate::FieldReader;
#[doc = "Field `PFETCNT` writer - PFET Switch Number Selection"]
pub type PfetcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NFETCNT` reader - NFET Switch Number Selection"]
pub type NfetcntR = crate::FieldReader;
#[doc = "Field `NFETCNT` writer - NFET Switch Number Selection"]
pub type NfetcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BYPLIMSEL` reader - Current Limit in Bypass Mode"]
pub type ByplimselR = crate::FieldReader;
#[doc = "Field `BYPLIMSEL` writer - Current Limit in Bypass Mode"]
pub type ByplimselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPCLIMILIMSEL` reader - Current Limit Level Selection for Current Limiter in LP Mode"]
pub type LpclimilimselR = crate::FieldReader;
#[doc = "Field `LPCLIMILIMSEL` writer - Current Limit Level Selection for Current Limiter in LP Mode"]
pub type LpclimilimselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LNCLIMILIMSEL` reader - Current Limit Level Selection for Current Limiter in LN Mode"]
pub type LnclimilimselR = crate::FieldReader;
#[doc = "Field `LNCLIMILIMSEL` writer - Current Limit Level Selection for Current Limiter in LN Mode"]
pub type LnclimilimselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "LP Mode Comparator Bias Selection\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
impl crate::IsEnum for LPCMPBIAS {}
#[doc = "Field `LPCMPBIAS` reader - LP Mode Comparator Bias Selection"]
pub type LpcmpbiasR = crate::FieldReader<LPCMPBIAS>;
impl LpcmpbiasR {
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
pub type LpcmpbiasW<'a, REG> = crate::FieldWriter<'a, REG, 2, LPCMPBIAS, crate::Safe>;
impl<'a, REG> LpcmpbiasW<'a, REG>
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
    pub fn lnforceccm(&self) -> LnforceccmR {
        LnforceccmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline(always)]
    pub fn pfetcnt(&self) -> PfetcntR {
        PfetcntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline(always)]
    pub fn nfetcnt(&self) -> NfetcntR {
        NfetcntR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline(always)]
    pub fn byplimsel(&self) -> ByplimselR {
        ByplimselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline(always)]
    pub fn lpclimilimsel(&self) -> LpclimilimselR {
        LpclimilimselR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline(always)]
    pub fn lnclimilimsel(&self) -> LnclimilimselR {
        LnclimilimselR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection"]
    #[inline(always)]
    pub fn lpcmpbias(&self) -> LpcmpbiasR {
        LpcmpbiasR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDCMISCCTRL")
            .field("lnforceccm", &self.lnforceccm())
            .field("pfetcnt", &self.pfetcnt())
            .field("nfetcnt", &self.nfetcnt())
            .field("byplimsel", &self.byplimsel())
            .field("lpclimilimsel", &self.lpclimilimsel())
            .field("lnclimilimsel", &self.lnclimilimsel())
            .field("lpcmpbias", &self.lpcmpbias())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Force DCDC Into CCM Mode in Low Noise Operation"]
    #[inline(always)]
    #[must_use]
    pub fn lnforceccm(&mut self) -> LnforceccmW<DCDCMISCCTRLrs> {
        LnforceccmW::new(self, 0)
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pfetcnt(&mut self) -> PfetcntW<DCDCMISCCTRLrs> {
        PfetcntW::new(self, 8)
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline(always)]
    #[must_use]
    pub fn nfetcnt(&mut self) -> NfetcntW<DCDCMISCCTRLrs> {
        NfetcntW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline(always)]
    #[must_use]
    pub fn byplimsel(&mut self) -> ByplimselW<DCDCMISCCTRLrs> {
        ByplimselW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpclimilimsel(&mut self) -> LpclimilimselW<DCDCMISCCTRLrs> {
        LpclimilimselW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lnclimilimsel(&mut self) -> LnclimilimselW<DCDCMISCCTRLrs> {
        LnclimilimselW::new(self, 24)
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpcmpbias(&mut self) -> LpcmpbiasW<DCDCMISCCTRLrs> {
        LpcmpbiasW::new(self, 28)
    }
}
#[doc = "DCDC Miscellaneous Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcmiscctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcmiscctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDCMISCCTRLrs;
impl crate::RegisterSpec for DCDCMISCCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdcmiscctrl::R`](R) reader structure"]
impl crate::Readable for DCDCMISCCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`dcdcmiscctrl::W`](W) writer structure"]
impl crate::Writable for DCDCMISCCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDCMISCCTRL to value 0x3330_7700"]
impl crate::Resettable for DCDCMISCCTRLrs {
    const RESET_VALUE: u32 = 0x3330_7700;
}
