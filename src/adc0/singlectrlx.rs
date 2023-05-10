#[doc = "Register `SINGLECTRLX` reader"]
pub struct R(crate::R<SINGLECTRLX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLECTRLX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLECTRLX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLECTRLX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SINGLECTRLX` writer"]
pub struct W(crate::W<SINGLECTRLX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGLECTRLX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SINGLECTRLX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGLECTRLX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREFSEL` reader - Single Channel Reference Selection"]
pub type VREFSEL_R = crate::FieldReader<u8, VREFSEL_A>;
#[doc = "Single Channel Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VREFSEL_A {
    #[doc = "0: Internal 0.83V Bandgap reference"]
    VBGR = 0,
    #[doc = "1: Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    VDDXWATT = 1,
    #[doc = "2: Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    VREFPWATT = 2,
    #[doc = "3: Raw single ended external Vref: ADCn_EXTP"]
    VREFP = 3,
    #[doc = "4: Special mode used to generate ENTROPY."]
    VENTROPY = 4,
    #[doc = "5: Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    VREFPNWATT = 5,
    #[doc = "6: Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    VREFPN = 6,
    #[doc = "7: Internal Bandgap reference at low setting 0.78V"]
    VBGRLOW = 7,
}
impl From<VREFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VREFSEL_A) -> Self {
        variant as _
    }
}
impl VREFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFSEL_A {
        match self.bits {
            0 => VREFSEL_A::VBGR,
            1 => VREFSEL_A::VDDXWATT,
            2 => VREFSEL_A::VREFPWATT,
            3 => VREFSEL_A::VREFP,
            4 => VREFSEL_A::VENTROPY,
            5 => VREFSEL_A::VREFPNWATT,
            6 => VREFSEL_A::VREFPN,
            7 => VREFSEL_A::VBGRLOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VBGR`"]
    #[inline(always)]
    pub fn is_vbgr(&self) -> bool {
        *self == VREFSEL_A::VBGR
    }
    #[doc = "Checks if the value of the field is `VDDXWATT`"]
    #[inline(always)]
    pub fn is_vddxwatt(&self) -> bool {
        *self == VREFSEL_A::VDDXWATT
    }
    #[doc = "Checks if the value of the field is `VREFPWATT`"]
    #[inline(always)]
    pub fn is_vrefpwatt(&self) -> bool {
        *self == VREFSEL_A::VREFPWATT
    }
    #[doc = "Checks if the value of the field is `VREFP`"]
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        *self == VREFSEL_A::VREFP
    }
    #[doc = "Checks if the value of the field is `VENTROPY`"]
    #[inline(always)]
    pub fn is_ventropy(&self) -> bool {
        *self == VREFSEL_A::VENTROPY
    }
    #[doc = "Checks if the value of the field is `VREFPNWATT`"]
    #[inline(always)]
    pub fn is_vrefpnwatt(&self) -> bool {
        *self == VREFSEL_A::VREFPNWATT
    }
    #[doc = "Checks if the value of the field is `VREFPN`"]
    #[inline(always)]
    pub fn is_vrefpn(&self) -> bool {
        *self == VREFSEL_A::VREFPN
    }
    #[doc = "Checks if the value of the field is `VBGRLOW`"]
    #[inline(always)]
    pub fn is_vbgrlow(&self) -> bool {
        *self == VREFSEL_A::VBGRLOW
    }
}
#[doc = "Field `VREFSEL` writer - Single Channel Reference Selection"]
pub type VREFSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SINGLECTRLX_SPEC, u8, VREFSEL_A, 3, O>;
impl<'a, const O: u8> VREFSEL_W<'a, O> {
    #[doc = "Internal 0.83V Bandgap reference"]
    #[inline(always)]
    pub fn vbgr(self) -> &'a mut W {
        self.variant(VREFSEL_A::VBGR)
    }
    #[doc = "Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vddxwatt(self) -> &'a mut W {
        self.variant(VREFSEL_A::VDDXWATT)
    }
    #[doc = "Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vrefpwatt(self) -> &'a mut W {
        self.variant(VREFSEL_A::VREFPWATT)
    }
    #[doc = "Raw single ended external Vref: ADCn_EXTP"]
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut W {
        self.variant(VREFSEL_A::VREFP)
    }
    #[doc = "Special mode used to generate ENTROPY."]
    #[inline(always)]
    pub fn ventropy(self) -> &'a mut W {
        self.variant(VREFSEL_A::VENTROPY)
    }
    #[doc = "Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vrefpnwatt(self) -> &'a mut W {
        self.variant(VREFSEL_A::VREFPNWATT)
    }
    #[doc = "Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    #[inline(always)]
    pub fn vrefpn(self) -> &'a mut W {
        self.variant(VREFSEL_A::VREFPN)
    }
    #[doc = "Internal Bandgap reference at low setting 0.78V"]
    #[inline(always)]
    pub fn vbgrlow(self) -> &'a mut W {
        self.variant(VREFSEL_A::VBGRLOW)
    }
}
#[doc = "Field `VREFATTFIX` reader - Enable Fixed Scaling on VREF"]
pub type VREFATTFIX_R = crate::BitReader<bool>;
#[doc = "Field `VREFATTFIX` writer - Enable Fixed Scaling on VREF"]
pub type VREFATTFIX_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINGLECTRLX_SPEC, bool, O>;
#[doc = "Field `VREFATT` reader - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
pub type VREFATT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFATT` writer - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
pub type VREFATT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SINGLECTRLX_SPEC, u8, u8, 4, O>;
#[doc = "Field `VINATT` reader - Code for VIN Attenuation Factor"]
pub type VINATT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VINATT` writer - Code for VIN Attenuation Factor"]
pub type VINATT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SINGLECTRLX_SPEC, u8, u8, 4, O>;
#[doc = "Field `DVL` reader - Single Channel DV Level Select"]
pub type DVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DVL` writer - Single Channel DV Level Select"]
pub type DVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SINGLECTRLX_SPEC, u8, u8, 2, O>;
#[doc = "Field `FIFOOFACT` reader - Single Channel FIFO Overflow Action"]
pub type FIFOOFACT_R = crate::BitReader<bool>;
#[doc = "Field `FIFOOFACT` writer - Single Channel FIFO Overflow Action"]
pub type FIFOOFACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINGLECTRLX_SPEC, bool, O>;
#[doc = "Field `PRSMODE` reader - Single Channel PRS Trigger Mode"]
pub type PRSMODE_R = crate::BitReader<bool>;
#[doc = "Field `PRSMODE` writer - Single Channel PRS Trigger Mode"]
pub type PRSMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINGLECTRLX_SPEC, bool, O>;
#[doc = "Field `PRSSEL` reader - Single Channel PRS Trigger Select"]
pub type PRSSEL_R = crate::FieldReader<u8, PRSSEL_A>;
#[doc = "Single Channel PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS ch 0 triggers single channel"]
    PRSCH0 = 0,
    #[doc = "1: PRS ch 1 triggers single channel"]
    PRSCH1 = 1,
    #[doc = "2: PRS ch 2 triggers single channel"]
    PRSCH2 = 2,
    #[doc = "3: PRS ch 3 triggers single channel"]
    PRSCH3 = 3,
    #[doc = "4: PRS ch 4 triggers single channel"]
    PRSCH4 = 4,
    #[doc = "5: PRS ch 5 triggers single channel"]
    PRSCH5 = 5,
    #[doc = "6: PRS ch 6 triggers single channel"]
    PRSCH6 = 6,
    #[doc = "7: PRS ch 7 triggers single channel"]
    PRSCH7 = 7,
    #[doc = "8: PRS ch 8 triggers single channel"]
    PRSCH8 = 8,
    #[doc = "9: PRS ch 9 triggers single channel"]
    PRSCH9 = 9,
    #[doc = "10: PRS ch 10 triggers single channel"]
    PRSCH10 = 10,
    #[doc = "11: PRS ch 11 triggers single channel"]
    PRSCH11 = 11,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
impl PRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRSSEL_A> {
        match self.bits {
            0 => Some(PRSSEL_A::PRSCH0),
            1 => Some(PRSSEL_A::PRSCH1),
            2 => Some(PRSSEL_A::PRSCH2),
            3 => Some(PRSSEL_A::PRSCH3),
            4 => Some(PRSSEL_A::PRSCH4),
            5 => Some(PRSSEL_A::PRSCH5),
            6 => Some(PRSSEL_A::PRSCH6),
            7 => Some(PRSSEL_A::PRSCH7),
            8 => Some(PRSSEL_A::PRSCH8),
            9 => Some(PRSSEL_A::PRSCH9),
            10 => Some(PRSSEL_A::PRSCH10),
            11 => Some(PRSSEL_A::PRSCH11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL_A::PRSCH11
    }
}
#[doc = "Field `PRSSEL` writer - Single Channel PRS Trigger Select"]
pub type PRSSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SINGLECTRLX_SPEC, u8, PRSSEL_A, 4, O>;
impl<'a, const O: u8> PRSSEL_W<'a, O> {
    #[doc = "PRS ch 0 triggers single channel"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers single channel"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers single channel"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers single channel"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers single channel"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers single channel"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS ch 6 triggers single channel"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS ch 7 triggers single channel"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS ch 8 triggers single channel"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS ch 9 triggers single channel"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS ch 10 triggers single channel"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS ch 11 triggers single channel"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH11)
    }
}
#[doc = "Field `CONVSTARTDELAY` reader - Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set"]
pub type CONVSTARTDELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONVSTARTDELAY` writer - Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set"]
pub type CONVSTARTDELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SINGLECTRLX_SPEC, u8, u8, 3, O>;
#[doc = "Field `CONVSTARTDELAYEN` reader - Enable Delaying Next Conversion Start"]
pub type CONVSTARTDELAYEN_R = crate::BitReader<bool>;
#[doc = "Field `CONVSTARTDELAYEN` writer - Enable Delaying Next Conversion Start"]
pub type CONVSTARTDELAYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINGLECTRLX_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Single Channel Reference Selection"]
    #[inline(always)]
    pub fn vrefsel(&self) -> VREFSEL_R {
        VREFSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enable Fixed Scaling on VREF"]
    #[inline(always)]
    pub fn vrefattfix(&self) -> VREFATTFIX_R {
        VREFATTFIX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
    #[inline(always)]
    pub fn vrefatt(&self) -> VREFATT_R {
        VREFATT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Code for VIN Attenuation Factor"]
    #[inline(always)]
    pub fn vinatt(&self) -> VINATT_R {
        VINATT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Single Channel DV Level Select"]
    #[inline(always)]
    pub fn dvl(&self) -> DVL_R {
        DVL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Single Channel FIFO Overflow Action"]
    #[inline(always)]
    pub fn fifoofact(&self) -> FIFOOFACT_R {
        FIFOOFACT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Single Channel PRS Trigger Mode"]
    #[inline(always)]
    pub fn prsmode(&self) -> PRSMODE_R {
        PRSMODE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - Single Channel PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set"]
    #[inline(always)]
    pub fn convstartdelay(&self) -> CONVSTARTDELAY_R {
        CONVSTARTDELAY_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Enable Delaying Next Conversion Start"]
    #[inline(always)]
    pub fn convstartdelayen(&self) -> CONVSTARTDELAYEN_R {
        CONVSTARTDELAYEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Single Channel Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vrefsel(&mut self) -> VREFSEL_W<0> {
        VREFSEL_W::new(self)
    }
    #[doc = "Bit 3 - Enable Fixed Scaling on VREF"]
    #[inline(always)]
    #[must_use]
    pub fn vrefattfix(&mut self) -> VREFATTFIX_W<3> {
        VREFATTFIX_W::new(self)
    }
    #[doc = "Bits 4:7 - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
    #[inline(always)]
    #[must_use]
    pub fn vrefatt(&mut self) -> VREFATT_W<4> {
        VREFATT_W::new(self)
    }
    #[doc = "Bits 8:11 - Code for VIN Attenuation Factor"]
    #[inline(always)]
    #[must_use]
    pub fn vinatt(&mut self) -> VINATT_W<8> {
        VINATT_W::new(self)
    }
    #[doc = "Bits 12:13 - Single Channel DV Level Select"]
    #[inline(always)]
    #[must_use]
    pub fn dvl(&mut self) -> DVL_W<12> {
        DVL_W::new(self)
    }
    #[doc = "Bit 14 - Single Channel FIFO Overflow Action"]
    #[inline(always)]
    #[must_use]
    pub fn fifoofact(&mut self) -> FIFOOFACT_W<14> {
        FIFOOFACT_W::new(self)
    }
    #[doc = "Bit 16 - Single Channel PRS Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsmode(&mut self) -> PRSMODE_W<16> {
        PRSMODE_W::new(self)
    }
    #[doc = "Bits 17:20 - Single Channel PRS Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<17> {
        PRSSEL_W::new(self)
    }
    #[doc = "Bits 24:26 - Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set"]
    #[inline(always)]
    #[must_use]
    pub fn convstartdelay(&mut self) -> CONVSTARTDELAY_W<24> {
        CONVSTARTDELAY_W::new(self)
    }
    #[doc = "Bit 27 - Enable Delaying Next Conversion Start"]
    #[inline(always)]
    #[must_use]
    pub fn convstartdelayen(&mut self) -> CONVSTARTDELAYEN_W<27> {
        CONVSTARTDELAYEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Single Channel Control Register Continued\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singlectrlx](index.html) module"]
pub struct SINGLECTRLX_SPEC;
impl crate::RegisterSpec for SINGLECTRLX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singlectrlx::R](R) reader structure"]
impl crate::Readable for SINGLECTRLX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [singlectrlx::W](W) writer structure"]
impl crate::Writable for SINGLECTRLX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SINGLECTRLX to value 0"]
impl crate::Resettable for SINGLECTRLX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
