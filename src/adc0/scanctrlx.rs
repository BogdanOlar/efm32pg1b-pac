#[doc = "Register `SCANCTRLX` reader"]
pub type R = crate::R<SCANCTRLX_SPEC>;
#[doc = "Register `SCANCTRLX` writer"]
pub type W = crate::W<SCANCTRLX_SPEC>;
#[doc = "Field `VREFSEL` reader - Scan Channel Reference Selection"]
pub type VREFSEL_R = crate::FieldReader<VREFSEL_A>;
#[doc = "Scan Channel Reference Selection\n\nValue on reset: 0"]
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
impl crate::FieldSpec for VREFSEL_A {
    type Ux = u8;
}
impl VREFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VREFSEL_A> {
        match self.bits {
            0 => Some(VREFSEL_A::VBGR),
            1 => Some(VREFSEL_A::VDDXWATT),
            2 => Some(VREFSEL_A::VREFPWATT),
            3 => Some(VREFSEL_A::VREFP),
            5 => Some(VREFSEL_A::VREFPNWATT),
            6 => Some(VREFSEL_A::VREFPN),
            7 => Some(VREFSEL_A::VBGRLOW),
            _ => None,
        }
    }
    #[doc = "Internal 0.83V Bandgap reference"]
    #[inline(always)]
    pub fn is_vbgr(&self) -> bool {
        *self == VREFSEL_A::VBGR
    }
    #[doc = "Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn is_vddxwatt(&self) -> bool {
        *self == VREFSEL_A::VDDXWATT
    }
    #[doc = "Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn is_vrefpwatt(&self) -> bool {
        *self == VREFSEL_A::VREFPWATT
    }
    #[doc = "Raw single ended external Vref: ADCn_EXTP"]
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        *self == VREFSEL_A::VREFP
    }
    #[doc = "Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn is_vrefpnwatt(&self) -> bool {
        *self == VREFSEL_A::VREFPNWATT
    }
    #[doc = "Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    #[inline(always)]
    pub fn is_vrefpn(&self) -> bool {
        *self == VREFSEL_A::VREFPN
    }
    #[doc = "Internal Bandgap reference at low setting 0.78V"]
    #[inline(always)]
    pub fn is_vbgrlow(&self) -> bool {
        *self == VREFSEL_A::VBGRLOW
    }
}
#[doc = "Field `VREFSEL` writer - Scan Channel Reference Selection"]
pub type VREFSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, VREFSEL_A>;
impl<'a, REG, const O: u8> VREFSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal 0.83V Bandgap reference"]
    #[inline(always)]
    pub fn vbgr(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL_A::VBGR)
    }
    #[doc = "Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vddxwatt(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL_A::VDDXWATT)
    }
    #[doc = "Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vrefpwatt(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL_A::VREFPWATT)
    }
    #[doc = "Raw single ended external Vref: ADCn_EXTP"]
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL_A::VREFP)
    }
    #[doc = "Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vrefpnwatt(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL_A::VREFPNWATT)
    }
    #[doc = "Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    #[inline(always)]
    pub fn vrefpn(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL_A::VREFPN)
    }
    #[doc = "Internal Bandgap reference at low setting 0.78V"]
    #[inline(always)]
    pub fn vbgrlow(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL_A::VBGRLOW)
    }
}
#[doc = "Field `VREFATTFIX` reader - Enable Fixed Scaling on VREF"]
pub type VREFATTFIX_R = crate::BitReader;
#[doc = "Field `VREFATTFIX` writer - Enable Fixed Scaling on VREF"]
pub type VREFATTFIX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VREFATT` reader - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
pub type VREFATT_R = crate::FieldReader;
#[doc = "Field `VREFATT` writer - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
pub type VREFATT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `VINATT` reader - Code for VIN Attenuation Factor"]
pub type VINATT_R = crate::FieldReader;
#[doc = "Field `VINATT` writer - Code for VIN Attenuation Factor"]
pub type VINATT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DVL` reader - Scan DV Level Select"]
pub type DVL_R = crate::FieldReader;
#[doc = "Field `DVL` writer - Scan DV Level Select"]
pub type DVL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FIFOOFACT` reader - Scan FIFO Overflow Action"]
pub type FIFOOFACT_R = crate::BitReader;
#[doc = "Field `FIFOOFACT` writer - Scan FIFO Overflow Action"]
pub type FIFOOFACT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRSMODE` reader - Scan PRS Trigger Mode"]
pub type PRSMODE_R = crate::BitReader;
#[doc = "Field `PRSMODE` writer - Scan PRS Trigger Mode"]
pub type PRSMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRSSEL` reader - Scan Sequence PRS Trigger Select"]
pub type PRSSEL_R = crate::FieldReader<PRSSEL_A>;
#[doc = "Scan Sequence PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS ch 0 triggers scan sequence"]
    PRSCH0 = 0,
    #[doc = "1: PRS ch 1 triggers scan sequence"]
    PRSCH1 = 1,
    #[doc = "2: PRS ch 2 triggers scan sequence"]
    PRSCH2 = 2,
    #[doc = "3: PRS ch 3 triggers scan sequence"]
    PRSCH3 = 3,
    #[doc = "4: PRS ch 4 triggers scan sequence"]
    PRSCH4 = 4,
    #[doc = "5: PRS ch 5 triggers scan sequence"]
    PRSCH5 = 5,
    #[doc = "6: PRS ch 6 triggers scan sequence"]
    PRSCH6 = 6,
    #[doc = "7: PRS ch 7 triggers scan sequence"]
    PRSCH7 = 7,
    #[doc = "8: PRS ch 8 triggers scan sequence"]
    PRSCH8 = 8,
    #[doc = "9: PRS ch 9 triggers scan sequence"]
    PRSCH9 = 9,
    #[doc = "10: PRS ch 10 triggers scan sequence"]
    PRSCH10 = 10,
    #[doc = "11: PRS ch 11 triggers scan sequence"]
    PRSCH11 = 11,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSEL_A {
    type Ux = u8;
}
impl PRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSSEL_A> {
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
    #[doc = "PRS ch 0 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "PRS ch 1 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "PRS ch 2 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "PRS ch 3 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "PRS ch 4 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "PRS ch 5 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
    #[doc = "PRS ch 6 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL_A::PRSCH6
    }
    #[doc = "PRS ch 7 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL_A::PRSCH7
    }
    #[doc = "PRS ch 8 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL_A::PRSCH8
    }
    #[doc = "PRS ch 9 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL_A::PRSCH9
    }
    #[doc = "PRS ch 10 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL_A::PRSCH10
    }
    #[doc = "PRS ch 11 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL_A::PRSCH11
    }
}
#[doc = "Field `PRSSEL` writer - Scan Sequence PRS Trigger Select"]
pub type PRSSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, PRSSEL_A>;
impl<'a, REG, const O: u8> PRSSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS ch 0 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS ch 6 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS ch 7 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS ch 8 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS ch 9 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS ch 10 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS ch 11 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH11)
    }
}
#[doc = "Field `CONVSTARTDELAY` reader - Delay Next Conversion Start If CONVSTARTDELAYEN is Set"]
pub type CONVSTARTDELAY_R = crate::FieldReader;
#[doc = "Field `CONVSTARTDELAY` writer - Delay Next Conversion Start If CONVSTARTDELAYEN is Set"]
pub type CONVSTARTDELAY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CONVSTARTDELAYEN` reader - Enable Delaying Next Conversion Start"]
pub type CONVSTARTDELAYEN_R = crate::BitReader;
#[doc = "Field `CONVSTARTDELAYEN` writer - Enable Delaying Next Conversion Start"]
pub type CONVSTARTDELAYEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Scan Channel Reference Selection"]
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
    #[doc = "Bits 12:13 - Scan DV Level Select"]
    #[inline(always)]
    pub fn dvl(&self) -> DVL_R {
        DVL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Scan FIFO Overflow Action"]
    #[inline(always)]
    pub fn fifoofact(&self) -> FIFOOFACT_R {
        FIFOOFACT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Scan PRS Trigger Mode"]
    #[inline(always)]
    pub fn prsmode(&self) -> PRSMODE_R {
        PRSMODE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - Scan Sequence PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Delay Next Conversion Start If CONVSTARTDELAYEN is Set"]
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCANCTRLX")
            .field("vrefsel", &format_args!("{}", self.vrefsel().bits()))
            .field("vrefattfix", &format_args!("{}", self.vrefattfix().bit()))
            .field("vrefatt", &format_args!("{}", self.vrefatt().bits()))
            .field("vinatt", &format_args!("{}", self.vinatt().bits()))
            .field("dvl", &format_args!("{}", self.dvl().bits()))
            .field("fifoofact", &format_args!("{}", self.fifoofact().bit()))
            .field("prsmode", &format_args!("{}", self.prsmode().bit()))
            .field("prssel", &format_args!("{}", self.prssel().bits()))
            .field(
                "convstartdelay",
                &format_args!("{}", self.convstartdelay().bits()),
            )
            .field(
                "convstartdelayen",
                &format_args!("{}", self.convstartdelayen().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SCANCTRLX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Scan Channel Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vrefsel(&mut self) -> VREFSEL_W<SCANCTRLX_SPEC, 0> {
        VREFSEL_W::new(self)
    }
    #[doc = "Bit 3 - Enable Fixed Scaling on VREF"]
    #[inline(always)]
    #[must_use]
    pub fn vrefattfix(&mut self) -> VREFATTFIX_W<SCANCTRLX_SPEC, 3> {
        VREFATTFIX_W::new(self)
    }
    #[doc = "Bits 4:7 - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
    #[inline(always)]
    #[must_use]
    pub fn vrefatt(&mut self) -> VREFATT_W<SCANCTRLX_SPEC, 4> {
        VREFATT_W::new(self)
    }
    #[doc = "Bits 8:11 - Code for VIN Attenuation Factor"]
    #[inline(always)]
    #[must_use]
    pub fn vinatt(&mut self) -> VINATT_W<SCANCTRLX_SPEC, 8> {
        VINATT_W::new(self)
    }
    #[doc = "Bits 12:13 - Scan DV Level Select"]
    #[inline(always)]
    #[must_use]
    pub fn dvl(&mut self) -> DVL_W<SCANCTRLX_SPEC, 12> {
        DVL_W::new(self)
    }
    #[doc = "Bit 14 - Scan FIFO Overflow Action"]
    #[inline(always)]
    #[must_use]
    pub fn fifoofact(&mut self) -> FIFOOFACT_W<SCANCTRLX_SPEC, 14> {
        FIFOOFACT_W::new(self)
    }
    #[doc = "Bit 16 - Scan PRS Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsmode(&mut self) -> PRSMODE_W<SCANCTRLX_SPEC, 16> {
        PRSMODE_W::new(self)
    }
    #[doc = "Bits 17:20 - Scan Sequence PRS Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<SCANCTRLX_SPEC, 17> {
        PRSSEL_W::new(self)
    }
    #[doc = "Bits 24:26 - Delay Next Conversion Start If CONVSTARTDELAYEN is Set"]
    #[inline(always)]
    #[must_use]
    pub fn convstartdelay(&mut self) -> CONVSTARTDELAY_W<SCANCTRLX_SPEC, 24> {
        CONVSTARTDELAY_W::new(self)
    }
    #[doc = "Bit 27 - Enable Delaying Next Conversion Start"]
    #[inline(always)]
    #[must_use]
    pub fn convstartdelayen(&mut self) -> CONVSTARTDELAYEN_W<SCANCTRLX_SPEC, 27> {
        CONVSTARTDELAYEN_W::new(self)
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
#[doc = "Scan Control Register Continued\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scanctrlx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scanctrlx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANCTRLX_SPEC;
impl crate::RegisterSpec for SCANCTRLX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanctrlx::R`](R) reader structure"]
impl crate::Readable for SCANCTRLX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scanctrlx::W`](W) writer structure"]
impl crate::Writable for SCANCTRLX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCANCTRLX to value 0"]
impl crate::Resettable for SCANCTRLX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
