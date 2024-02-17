#[doc = "Register `SCANCTRLX` reader"]
pub type R = crate::R<SCANCTRLXrs>;
#[doc = "Register `SCANCTRLX` writer"]
pub type W = crate::W<SCANCTRLXrs>;
#[doc = "Field `VREFSEL` reader - Scan Channel Reference Selection"]
pub type VREFSEL_R = crate::FieldReader<VREFSEL>;
#[doc = "Scan Channel Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VREFSEL {
    #[doc = "0: Internal 0.83V Bandgap reference"]
    Vbgr = 0,
    #[doc = "1: Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    Vddxwatt = 1,
    #[doc = "2: Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    Vrefpwatt = 2,
    #[doc = "3: Raw single ended external Vref: ADCn_EXTP"]
    Vrefp = 3,
    #[doc = "5: Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    Vrefpnwatt = 5,
    #[doc = "6: Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    Vrefpn = 6,
    #[doc = "7: Internal Bandgap reference at low setting 0.78V"]
    Vbgrlow = 7,
}
impl From<VREFSEL> for u8 {
    #[inline(always)]
    fn from(variant: VREFSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VREFSEL {
    type Ux = u8;
}
impl VREFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VREFSEL> {
        match self.bits {
            0 => Some(VREFSEL::Vbgr),
            1 => Some(VREFSEL::Vddxwatt),
            2 => Some(VREFSEL::Vrefpwatt),
            3 => Some(VREFSEL::Vrefp),
            5 => Some(VREFSEL::Vrefpnwatt),
            6 => Some(VREFSEL::Vrefpn),
            7 => Some(VREFSEL::Vbgrlow),
            _ => None,
        }
    }
    #[doc = "Internal 0.83V Bandgap reference"]
    #[inline(always)]
    pub fn is_vbgr(&self) -> bool {
        *self == VREFSEL::Vbgr
    }
    #[doc = "Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn is_vddxwatt(&self) -> bool {
        *self == VREFSEL::Vddxwatt
    }
    #[doc = "Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn is_vrefpwatt(&self) -> bool {
        *self == VREFSEL::Vrefpwatt
    }
    #[doc = "Raw single ended external Vref: ADCn_EXTP"]
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        *self == VREFSEL::Vrefp
    }
    #[doc = "Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn is_vrefpnwatt(&self) -> bool {
        *self == VREFSEL::Vrefpnwatt
    }
    #[doc = "Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    #[inline(always)]
    pub fn is_vrefpn(&self) -> bool {
        *self == VREFSEL::Vrefpn
    }
    #[doc = "Internal Bandgap reference at low setting 0.78V"]
    #[inline(always)]
    pub fn is_vbgrlow(&self) -> bool {
        *self == VREFSEL::Vbgrlow
    }
}
#[doc = "Field `VREFSEL` writer - Scan Channel Reference Selection"]
pub type VREFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, VREFSEL>;
impl<'a, REG> VREFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal 0.83V Bandgap reference"]
    #[inline(always)]
    pub fn vbgr(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL::Vbgr)
    }
    #[doc = "Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vddxwatt(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL::Vddxwatt)
    }
    #[doc = "Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vrefpwatt(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL::Vrefpwatt)
    }
    #[doc = "Raw single ended external Vref: ADCn_EXTP"]
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL::Vrefp)
    }
    #[doc = "Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vrefpnwatt(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL::Vrefpnwatt)
    }
    #[doc = "Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    #[inline(always)]
    pub fn vrefpn(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL::Vrefpn)
    }
    #[doc = "Internal Bandgap reference at low setting 0.78V"]
    #[inline(always)]
    pub fn vbgrlow(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL::Vbgrlow)
    }
}
#[doc = "Field `VREFATTFIX` reader - Enable Fixed Scaling on VREF"]
pub type VREFATTFIX_R = crate::BitReader;
#[doc = "Field `VREFATTFIX` writer - Enable Fixed Scaling on VREF"]
pub type VREFATTFIX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFATT` reader - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
pub type VREFATT_R = crate::FieldReader;
#[doc = "Field `VREFATT` writer - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
pub type VREFATT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VINATT` reader - Code for VIN Attenuation Factor"]
pub type VINATT_R = crate::FieldReader;
#[doc = "Field `VINATT` writer - Code for VIN Attenuation Factor"]
pub type VINATT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DVL` reader - Scan DV Level Select"]
pub type DVL_R = crate::FieldReader;
#[doc = "Field `DVL` writer - Scan DV Level Select"]
pub type DVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIFOOFACT` reader - Scan FIFO Overflow Action"]
pub type FIFOOFACT_R = crate::BitReader;
#[doc = "Field `FIFOOFACT` writer - Scan FIFO Overflow Action"]
pub type FIFOOFACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSMODE` reader - Scan PRS Trigger Mode"]
pub type PRSMODE_R = crate::BitReader;
#[doc = "Field `PRSMODE` writer - Scan PRS Trigger Mode"]
pub type PRSMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSSEL` reader - Scan Sequence PRS Trigger Select"]
pub type PRSSEL_R = crate::FieldReader<PRSSEL>;
#[doc = "Scan Sequence PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL {
    #[doc = "0: PRS ch 0 triggers scan sequence"]
    Prsch0 = 0,
    #[doc = "1: PRS ch 1 triggers scan sequence"]
    Prsch1 = 1,
    #[doc = "2: PRS ch 2 triggers scan sequence"]
    Prsch2 = 2,
    #[doc = "3: PRS ch 3 triggers scan sequence"]
    Prsch3 = 3,
    #[doc = "4: PRS ch 4 triggers scan sequence"]
    Prsch4 = 4,
    #[doc = "5: PRS ch 5 triggers scan sequence"]
    Prsch5 = 5,
    #[doc = "6: PRS ch 6 triggers scan sequence"]
    Prsch6 = 6,
    #[doc = "7: PRS ch 7 triggers scan sequence"]
    Prsch7 = 7,
    #[doc = "8: PRS ch 8 triggers scan sequence"]
    Prsch8 = 8,
    #[doc = "9: PRS ch 9 triggers scan sequence"]
    Prsch9 = 9,
    #[doc = "10: PRS ch 10 triggers scan sequence"]
    Prsch10 = 10,
    #[doc = "11: PRS ch 11 triggers scan sequence"]
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
    #[doc = "PRS ch 0 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL::Prsch0
    }
    #[doc = "PRS ch 1 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL::Prsch1
    }
    #[doc = "PRS ch 2 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL::Prsch2
    }
    #[doc = "PRS ch 3 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL::Prsch3
    }
    #[doc = "PRS ch 4 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL::Prsch4
    }
    #[doc = "PRS ch 5 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL::Prsch5
    }
    #[doc = "PRS ch 6 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL::Prsch6
    }
    #[doc = "PRS ch 7 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL::Prsch7
    }
    #[doc = "PRS ch 8 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL::Prsch8
    }
    #[doc = "PRS ch 9 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL::Prsch9
    }
    #[doc = "PRS ch 10 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL::Prsch10
    }
    #[doc = "PRS ch 11 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL::Prsch11
    }
}
#[doc = "Field `PRSSEL` writer - Scan Sequence PRS Trigger Select"]
pub type PRSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRSSEL>;
impl<'a, REG> PRSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS ch 0 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch0)
    }
    #[doc = "PRS ch 1 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch1)
    }
    #[doc = "PRS ch 2 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch2)
    }
    #[doc = "PRS ch 3 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch3)
    }
    #[doc = "PRS ch 4 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch4)
    }
    #[doc = "PRS ch 5 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch5)
    }
    #[doc = "PRS ch 6 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch6)
    }
    #[doc = "PRS ch 7 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch7)
    }
    #[doc = "PRS ch 8 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch8)
    }
    #[doc = "PRS ch 9 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch9)
    }
    #[doc = "PRS ch 10 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch10)
    }
    #[doc = "PRS ch 11 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch11)
    }
}
#[doc = "Field `CONVSTARTDELAY` reader - Delay Next Conversion Start If CONVSTARTDELAYEN is Set"]
pub type CONVSTARTDELAY_R = crate::FieldReader;
#[doc = "Field `CONVSTARTDELAY` writer - Delay Next Conversion Start If CONVSTARTDELAYEN is Set"]
pub type CONVSTARTDELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CONVSTARTDELAYEN` reader - Enable Delaying Next Conversion Start"]
pub type CONVSTARTDELAYEN_R = crate::BitReader;
#[doc = "Field `CONVSTARTDELAYEN` writer - Enable Delaying Next Conversion Start"]
pub type CONVSTARTDELAYEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bits 0:2 - Scan Channel Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vrefsel(&mut self) -> VREFSEL_W<SCANCTRLXrs> {
        VREFSEL_W::new(self, 0)
    }
    #[doc = "Bit 3 - Enable Fixed Scaling on VREF"]
    #[inline(always)]
    #[must_use]
    pub fn vrefattfix(&mut self) -> VREFATTFIX_W<SCANCTRLXrs> {
        VREFATTFIX_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
    #[inline(always)]
    #[must_use]
    pub fn vrefatt(&mut self) -> VREFATT_W<SCANCTRLXrs> {
        VREFATT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Code for VIN Attenuation Factor"]
    #[inline(always)]
    #[must_use]
    pub fn vinatt(&mut self) -> VINATT_W<SCANCTRLXrs> {
        VINATT_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Scan DV Level Select"]
    #[inline(always)]
    #[must_use]
    pub fn dvl(&mut self) -> DVL_W<SCANCTRLXrs> {
        DVL_W::new(self, 12)
    }
    #[doc = "Bit 14 - Scan FIFO Overflow Action"]
    #[inline(always)]
    #[must_use]
    pub fn fifoofact(&mut self) -> FIFOOFACT_W<SCANCTRLXrs> {
        FIFOOFACT_W::new(self, 14)
    }
    #[doc = "Bit 16 - Scan PRS Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsmode(&mut self) -> PRSMODE_W<SCANCTRLXrs> {
        PRSMODE_W::new(self, 16)
    }
    #[doc = "Bits 17:20 - Scan Sequence PRS Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<SCANCTRLXrs> {
        PRSSEL_W::new(self, 17)
    }
    #[doc = "Bits 24:26 - Delay Next Conversion Start If CONVSTARTDELAYEN is Set"]
    #[inline(always)]
    #[must_use]
    pub fn convstartdelay(&mut self) -> CONVSTARTDELAY_W<SCANCTRLXrs> {
        CONVSTARTDELAY_W::new(self, 24)
    }
    #[doc = "Bit 27 - Enable Delaying Next Conversion Start"]
    #[inline(always)]
    #[must_use]
    pub fn convstartdelayen(&mut self) -> CONVSTARTDELAYEN_W<SCANCTRLXrs> {
        CONVSTARTDELAYEN_W::new(self, 27)
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
pub struct SCANCTRLXrs;
impl crate::RegisterSpec for SCANCTRLXrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanctrlx::R`](R) reader structure"]
impl crate::Readable for SCANCTRLXrs {}
#[doc = "`write(|w| ..)` method takes [`scanctrlx::W`](W) writer structure"]
impl crate::Writable for SCANCTRLXrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCANCTRLX to value 0"]
impl crate::Resettable for SCANCTRLXrs {
    const RESET_VALUE: u32 = 0;
}
