#[doc = "Register `DCDCMISCCTRL` reader"]
pub type R = crate::R<DCDCMISCCTRL_SPEC>;
#[doc = "Register `DCDCMISCCTRL` writer"]
pub type W = crate::W<DCDCMISCCTRL_SPEC>;
#[doc = "Field `LNFORCECCM` reader - Force DCDC Into CCM Mode in Low Noise Operation"]
pub type LNFORCECCM_R = crate::BitReader;
#[doc = "Field `LNFORCECCM` writer - Force DCDC Into CCM Mode in Low Noise Operation"]
pub type LNFORCECCM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PFETCNT` reader - PFET Switch Number Selection"]
pub type PFETCNT_R = crate::FieldReader;
#[doc = "Field `PFETCNT` writer - PFET Switch Number Selection"]
pub type PFETCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `NFETCNT` reader - NFET Switch Number Selection"]
pub type NFETCNT_R = crate::FieldReader;
#[doc = "Field `NFETCNT` writer - NFET Switch Number Selection"]
pub type NFETCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `BYPLIMSEL` reader - Current Limit in Bypass Mode"]
pub type BYPLIMSEL_R = crate::FieldReader;
#[doc = "Field `BYPLIMSEL` writer - Current Limit in Bypass Mode"]
pub type BYPLIMSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `LPCLIMILIMSEL` reader - Current Limit Level Selection for Current Limiter in LP Mode"]
pub type LPCLIMILIMSEL_R = crate::FieldReader;
#[doc = "Field `LPCLIMILIMSEL` writer - Current Limit Level Selection for Current Limiter in LP Mode"]
pub type LPCLIMILIMSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `LNCLIMILIMSEL` reader - Current Limit Level Selection for Current Limiter in LN Mode"]
pub type LNCLIMILIMSEL_R = crate::FieldReader;
#[doc = "Field `LNCLIMILIMSEL` writer - Current Limit Level Selection for Current Limiter in LN Mode"]
pub type LNCLIMILIMSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `LPCMPBIAS` reader - LP Mode Comparator Bias Selection"]
pub type LPCMPBIAS_R = crate::FieldReader<LPCMPBIAS_A>;
#[doc = "LP Mode Comparator Bias Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPCMPBIAS_A {
    #[doc = "0: Maximum load current less than 75uA."]
    BIAS0 = 0,
    #[doc = "1: Maximum load current less than 500uA."]
    BIAS1 = 1,
    #[doc = "2: Maximum load current less than 2.5mA."]
    BIAS2 = 2,
    #[doc = "3: Maximum load current less than 10mA."]
    BIAS3 = 3,
}
impl From<LPCMPBIAS_A> for u8 {
    #[inline(always)]
    fn from(variant: LPCMPBIAS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPCMPBIAS_A {
    type Ux = u8;
}
impl LPCMPBIAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPCMPBIAS_A {
        match self.bits {
            0 => LPCMPBIAS_A::BIAS0,
            1 => LPCMPBIAS_A::BIAS1,
            2 => LPCMPBIAS_A::BIAS2,
            3 => LPCMPBIAS_A::BIAS3,
            _ => unreachable!(),
        }
    }
    #[doc = "Maximum load current less than 75uA."]
    #[inline(always)]
    pub fn is_bias0(&self) -> bool {
        *self == LPCMPBIAS_A::BIAS0
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline(always)]
    pub fn is_bias1(&self) -> bool {
        *self == LPCMPBIAS_A::BIAS1
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline(always)]
    pub fn is_bias2(&self) -> bool {
        *self == LPCMPBIAS_A::BIAS2
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline(always)]
    pub fn is_bias3(&self) -> bool {
        *self == LPCMPBIAS_A::BIAS3
    }
}
#[doc = "Field `LPCMPBIAS` writer - LP Mode Comparator Bias Selection"]
pub type LPCMPBIAS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, LPCMPBIAS_A>;
impl<'a, REG, const O: u8> LPCMPBIAS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum load current less than 75uA."]
    #[inline(always)]
    pub fn bias0(self) -> &'a mut crate::W<REG> {
        self.variant(LPCMPBIAS_A::BIAS0)
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline(always)]
    pub fn bias1(self) -> &'a mut crate::W<REG> {
        self.variant(LPCMPBIAS_A::BIAS1)
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline(always)]
    pub fn bias2(self) -> &'a mut crate::W<REG> {
        self.variant(LPCMPBIAS_A::BIAS2)
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline(always)]
    pub fn bias3(self) -> &'a mut crate::W<REG> {
        self.variant(LPCMPBIAS_A::BIAS3)
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDCMISCCTRL")
            .field("lnforceccm", &format_args!("{}", self.lnforceccm().bit()))
            .field("pfetcnt", &format_args!("{}", self.pfetcnt().bits()))
            .field("nfetcnt", &format_args!("{}", self.nfetcnt().bits()))
            .field("byplimsel", &format_args!("{}", self.byplimsel().bits()))
            .field(
                "lpclimilimsel",
                &format_args!("{}", self.lpclimilimsel().bits()),
            )
            .field(
                "lnclimilimsel",
                &format_args!("{}", self.lnclimilimsel().bits()),
            )
            .field("lpcmpbias", &format_args!("{}", self.lpcmpbias().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DCDCMISCCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Force DCDC Into CCM Mode in Low Noise Operation"]
    #[inline(always)]
    #[must_use]
    pub fn lnforceccm(&mut self) -> LNFORCECCM_W<DCDCMISCCTRL_SPEC, 0> {
        LNFORCECCM_W::new(self)
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pfetcnt(&mut self) -> PFETCNT_W<DCDCMISCCTRL_SPEC, 8> {
        PFETCNT_W::new(self)
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline(always)]
    #[must_use]
    pub fn nfetcnt(&mut self) -> NFETCNT_W<DCDCMISCCTRL_SPEC, 12> {
        NFETCNT_W::new(self)
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline(always)]
    #[must_use]
    pub fn byplimsel(&mut self) -> BYPLIMSEL_W<DCDCMISCCTRL_SPEC, 16> {
        BYPLIMSEL_W::new(self)
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpclimilimsel(&mut self) -> LPCLIMILIMSEL_W<DCDCMISCCTRL_SPEC, 20> {
        LPCLIMILIMSEL_W::new(self)
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lnclimilimsel(&mut self) -> LNCLIMILIMSEL_W<DCDCMISCCTRL_SPEC, 24> {
        LNCLIMILIMSEL_W::new(self)
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpcmpbias(&mut self) -> LPCMPBIAS_W<DCDCMISCCTRL_SPEC, 28> {
        LPCMPBIAS_W::new(self)
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
pub struct DCDCMISCCTRL_SPEC;
impl crate::RegisterSpec for DCDCMISCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdcmiscctrl::R`](R) reader structure"]
impl crate::Readable for DCDCMISCCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdcmiscctrl::W`](W) writer structure"]
impl crate::Writable for DCDCMISCCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDCMISCCTRL to value 0x3330_7700"]
impl crate::Resettable for DCDCMISCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3330_7700;
}
