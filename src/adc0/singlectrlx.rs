///Register `SINGLECTRLX` reader
pub type R = crate::R<SINGLECTRLXrs>;
///Register `SINGLECTRLX` writer
pub type W = crate::W<SINGLECTRLXrs>;
///Single Channel Reference Selection
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VREFSEL {
    ///0: Internal 0.83V Bandgap reference
    Vbgr = 0,
    ///1: Scaled AVDD: AVDD*(the VREF attenuation factor)
    Vddxwatt = 1,
    ///2: Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)
    Vrefpwatt = 2,
    ///3: Raw single ended external Vref: ADCn_EXTP
    Vrefp = 3,
    ///4: Special mode used to generate ENTROPY.
    Ventropy = 4,
    ///5: Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)
    Vrefpnwatt = 5,
    ///6: Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)
    Vrefpn = 6,
    ///7: Internal Bandgap reference at low setting 0.78V
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
impl crate::IsEnum for VREFSEL {}
///Field `VREFSEL` reader - Single Channel Reference Selection
pub type VrefselR = crate::FieldReader<VREFSEL>;
impl VrefselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VREFSEL {
        match self.bits {
            0 => VREFSEL::Vbgr,
            1 => VREFSEL::Vddxwatt,
            2 => VREFSEL::Vrefpwatt,
            3 => VREFSEL::Vrefp,
            4 => VREFSEL::Ventropy,
            5 => VREFSEL::Vrefpnwatt,
            6 => VREFSEL::Vrefpn,
            7 => VREFSEL::Vbgrlow,
            _ => unreachable!(),
        }
    }
    ///Internal 0.83V Bandgap reference
    #[inline(always)]
    pub fn is_vbgr(&self) -> bool {
        *self == VREFSEL::Vbgr
    }
    ///Scaled AVDD: AVDD*(the VREF attenuation factor)
    #[inline(always)]
    pub fn is_vddxwatt(&self) -> bool {
        *self == VREFSEL::Vddxwatt
    }
    ///Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)
    #[inline(always)]
    pub fn is_vrefpwatt(&self) -> bool {
        *self == VREFSEL::Vrefpwatt
    }
    ///Raw single ended external Vref: ADCn_EXTP
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        *self == VREFSEL::Vrefp
    }
    ///Special mode used to generate ENTROPY.
    #[inline(always)]
    pub fn is_ventropy(&self) -> bool {
        *self == VREFSEL::Ventropy
    }
    ///Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)
    #[inline(always)]
    pub fn is_vrefpnwatt(&self) -> bool {
        *self == VREFSEL::Vrefpnwatt
    }
    ///Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)
    #[inline(always)]
    pub fn is_vrefpn(&self) -> bool {
        *self == VREFSEL::Vrefpn
    }
    ///Internal Bandgap reference at low setting 0.78V
    #[inline(always)]
    pub fn is_vbgrlow(&self) -> bool {
        *self == VREFSEL::Vbgrlow
    }
}
///Field `VREFSEL` writer - Single Channel Reference Selection
pub type VrefselW<'a, REG> = crate::FieldWriter<'a, REG, 3, VREFSEL, crate::Safe>;
impl<'a, REG> VrefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Internal 0.83V Bandgap reference
    #[inline(always)]
    pub fn vbgr(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL::Vbgr)
    }
    ///Scaled AVDD: AVDD*(the VREF attenuation factor)
    #[inline(always)]
    pub fn vddxwatt(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL::Vddxwatt)
    }
    ///Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)
    #[inline(always)]
    pub fn vrefpwatt(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL::Vrefpwatt)
    }
    ///Raw single ended external Vref: ADCn_EXTP
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL::Vrefp)
    }
    ///Special mode used to generate ENTROPY.
    #[inline(always)]
    pub fn ventropy(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL::Ventropy)
    }
    ///Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)
    #[inline(always)]
    pub fn vrefpnwatt(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL::Vrefpnwatt)
    }
    ///Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)
    #[inline(always)]
    pub fn vrefpn(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL::Vrefpn)
    }
    ///Internal Bandgap reference at low setting 0.78V
    #[inline(always)]
    pub fn vbgrlow(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL::Vbgrlow)
    }
}
///Field `VREFATTFIX` reader - Enable Fixed Scaling on VREF
pub type VrefattfixR = crate::BitReader;
///Field `VREFATTFIX` writer - Enable Fixed Scaling on VREF
pub type VrefattfixW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFATT` reader - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5
pub type VrefattR = crate::FieldReader;
///Field `VREFATT` writer - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5
pub type VrefattW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `VINATT` reader - Code for VIN Attenuation Factor
pub type VinattR = crate::FieldReader;
///Field `VINATT` writer - Code for VIN Attenuation Factor
pub type VinattW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DVL` reader - Single Channel DV Level Select
pub type DvlR = crate::FieldReader;
///Field `DVL` writer - Single Channel DV Level Select
pub type DvlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FIFOOFACT` reader - Single Channel FIFO Overflow Action
pub type FifoofactR = crate::BitReader;
///Field `FIFOOFACT` writer - Single Channel FIFO Overflow Action
pub type FifoofactW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRSMODE` reader - Single Channel PRS Trigger Mode
pub type PrsmodeR = crate::BitReader;
///Field `PRSMODE` writer - Single Channel PRS Trigger Mode
pub type PrsmodeW<'a, REG> = crate::BitWriter<'a, REG>;
///Single Channel PRS Trigger Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL {
    ///0: PRS ch 0 triggers single channel
    Prsch0 = 0,
    ///1: PRS ch 1 triggers single channel
    Prsch1 = 1,
    ///2: PRS ch 2 triggers single channel
    Prsch2 = 2,
    ///3: PRS ch 3 triggers single channel
    Prsch3 = 3,
    ///4: PRS ch 4 triggers single channel
    Prsch4 = 4,
    ///5: PRS ch 5 triggers single channel
    Prsch5 = 5,
    ///6: PRS ch 6 triggers single channel
    Prsch6 = 6,
    ///7: PRS ch 7 triggers single channel
    Prsch7 = 7,
    ///8: PRS ch 8 triggers single channel
    Prsch8 = 8,
    ///9: PRS ch 9 triggers single channel
    Prsch9 = 9,
    ///10: PRS ch 10 triggers single channel
    Prsch10 = 10,
    ///11: PRS ch 11 triggers single channel
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
impl crate::IsEnum for PRSSEL {}
///Field `PRSSEL` reader - Single Channel PRS Trigger Select
pub type PrsselR = crate::FieldReader<PRSSEL>;
impl PrsselR {
    ///Get enumerated values variant
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
    ///PRS ch 0 triggers single channel
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL::Prsch0
    }
    ///PRS ch 1 triggers single channel
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL::Prsch1
    }
    ///PRS ch 2 triggers single channel
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL::Prsch2
    }
    ///PRS ch 3 triggers single channel
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL::Prsch3
    }
    ///PRS ch 4 triggers single channel
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL::Prsch4
    }
    ///PRS ch 5 triggers single channel
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL::Prsch5
    }
    ///PRS ch 6 triggers single channel
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL::Prsch6
    }
    ///PRS ch 7 triggers single channel
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL::Prsch7
    }
    ///PRS ch 8 triggers single channel
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL::Prsch8
    }
    ///PRS ch 9 triggers single channel
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL::Prsch9
    }
    ///PRS ch 10 triggers single channel
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL::Prsch10
    }
    ///PRS ch 11 triggers single channel
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL::Prsch11
    }
}
///Field `PRSSEL` writer - Single Channel PRS Trigger Select
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, PRSSEL>;
impl<'a, REG> PrsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS ch 0 triggers single channel
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch0)
    }
    ///PRS ch 1 triggers single channel
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch1)
    }
    ///PRS ch 2 triggers single channel
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch2)
    }
    ///PRS ch 3 triggers single channel
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch3)
    }
    ///PRS ch 4 triggers single channel
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch4)
    }
    ///PRS ch 5 triggers single channel
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch5)
    }
    ///PRS ch 6 triggers single channel
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch6)
    }
    ///PRS ch 7 triggers single channel
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch7)
    }
    ///PRS ch 8 triggers single channel
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch8)
    }
    ///PRS ch 9 triggers single channel
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch9)
    }
    ///PRS ch 10 triggers single channel
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch10)
    }
    ///PRS ch 11 triggers single channel
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch11)
    }
}
///Field `CONVSTARTDELAY` reader - Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set
pub type ConvstartdelayR = crate::FieldReader;
///Field `CONVSTARTDELAY` writer - Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set
pub type ConvstartdelayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CONVSTARTDELAYEN` reader - Enable Delaying Next Conversion Start
pub type ConvstartdelayenR = crate::BitReader;
///Field `CONVSTARTDELAYEN` writer - Enable Delaying Next Conversion Start
pub type ConvstartdelayenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Single Channel Reference Selection
    #[inline(always)]
    pub fn vrefsel(&self) -> VrefselR {
        VrefselR::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Enable Fixed Scaling on VREF
    #[inline(always)]
    pub fn vrefattfix(&self) -> VrefattfixR {
        VrefattfixR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5
    #[inline(always)]
    pub fn vrefatt(&self) -> VrefattR {
        VrefattR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Code for VIN Attenuation Factor
    #[inline(always)]
    pub fn vinatt(&self) -> VinattR {
        VinattR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:13 - Single Channel DV Level Select
    #[inline(always)]
    pub fn dvl(&self) -> DvlR {
        DvlR::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - Single Channel FIFO Overflow Action
    #[inline(always)]
    pub fn fifoofact(&self) -> FifoofactR {
        FifoofactR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Single Channel PRS Trigger Mode
    #[inline(always)]
    pub fn prsmode(&self) -> PrsmodeR {
        PrsmodeR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:20 - Single Channel PRS Trigger Select
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    ///Bits 24:26 - Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set
    #[inline(always)]
    pub fn convstartdelay(&self) -> ConvstartdelayR {
        ConvstartdelayR::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 27 - Enable Delaying Next Conversion Start
    #[inline(always)]
    pub fn convstartdelayen(&self) -> ConvstartdelayenR {
        ConvstartdelayenR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SINGLECTRLX")
            .field("vrefsel", &self.vrefsel())
            .field("vrefattfix", &self.vrefattfix())
            .field("vrefatt", &self.vrefatt())
            .field("vinatt", &self.vinatt())
            .field("dvl", &self.dvl())
            .field("fifoofact", &self.fifoofact())
            .field("prsmode", &self.prsmode())
            .field("prssel", &self.prssel())
            .field("convstartdelay", &self.convstartdelay())
            .field("convstartdelayen", &self.convstartdelayen())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Single Channel Reference Selection
    #[inline(always)]
    #[must_use]
    pub fn vrefsel(&mut self) -> VrefselW<SINGLECTRLXrs> {
        VrefselW::new(self, 0)
    }
    ///Bit 3 - Enable Fixed Scaling on VREF
    #[inline(always)]
    #[must_use]
    pub fn vrefattfix(&mut self) -> VrefattfixW<SINGLECTRLXrs> {
        VrefattfixW::new(self, 3)
    }
    ///Bits 4:7 - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5
    #[inline(always)]
    #[must_use]
    pub fn vrefatt(&mut self) -> VrefattW<SINGLECTRLXrs> {
        VrefattW::new(self, 4)
    }
    ///Bits 8:11 - Code for VIN Attenuation Factor
    #[inline(always)]
    #[must_use]
    pub fn vinatt(&mut self) -> VinattW<SINGLECTRLXrs> {
        VinattW::new(self, 8)
    }
    ///Bits 12:13 - Single Channel DV Level Select
    #[inline(always)]
    #[must_use]
    pub fn dvl(&mut self) -> DvlW<SINGLECTRLXrs> {
        DvlW::new(self, 12)
    }
    ///Bit 14 - Single Channel FIFO Overflow Action
    #[inline(always)]
    #[must_use]
    pub fn fifoofact(&mut self) -> FifoofactW<SINGLECTRLXrs> {
        FifoofactW::new(self, 14)
    }
    ///Bit 16 - Single Channel PRS Trigger Mode
    #[inline(always)]
    #[must_use]
    pub fn prsmode(&mut self) -> PrsmodeW<SINGLECTRLXrs> {
        PrsmodeW::new(self, 16)
    }
    ///Bits 17:20 - Single Channel PRS Trigger Select
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PrsselW<SINGLECTRLXrs> {
        PrsselW::new(self, 17)
    }
    ///Bits 24:26 - Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set
    #[inline(always)]
    #[must_use]
    pub fn convstartdelay(&mut self) -> ConvstartdelayW<SINGLECTRLXrs> {
        ConvstartdelayW::new(self, 24)
    }
    ///Bit 27 - Enable Delaying Next Conversion Start
    #[inline(always)]
    #[must_use]
    pub fn convstartdelayen(&mut self) -> ConvstartdelayenW<SINGLECTRLXrs> {
        ConvstartdelayenW::new(self, 27)
    }
}
///Single Channel Control Register Continued
///
///You can [`read`](crate::Reg::read) this register and get [`singlectrlx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlectrlx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SINGLECTRLXrs;
impl crate::RegisterSpec for SINGLECTRLXrs {
    type Ux = u32;
}
///`read()` method returns [`singlectrlx::R`](R) reader structure
impl crate::Readable for SINGLECTRLXrs {}
///`write(|w| ..)` method takes [`singlectrlx::W`](W) writer structure
impl crate::Writable for SINGLECTRLXrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SINGLECTRLX to value 0
impl crate::Resettable for SINGLECTRLXrs {
    const RESET_VALUE: u32 = 0;
}
