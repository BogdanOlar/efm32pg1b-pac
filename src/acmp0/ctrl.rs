#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRLrs>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRLrs>;
#[doc = "Field `EN` reader - Analog Comparator Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Analog Comparator Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INACTVAL` reader - Inactive Value"]
pub type InactvalR = crate::BitReader;
#[doc = "Field `INACTVAL` writer - Inactive Value"]
pub type InactvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOINV` reader - Comparator GPIO Output Invert"]
pub type GpioinvR = crate::BitReader;
#[doc = "Field `GPIOINV` writer - Comparator GPIO Output Invert"]
pub type GpioinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTXMASTERDIS` reader - APORT Bus X Master Disable"]
pub type AportxmasterdisR = crate::BitReader;
#[doc = "Field `APORTXMASTERDIS` writer - APORT Bus X Master Disable"]
pub type AportxmasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTYMASTERDIS` reader - APORT Bus Y Master Disable"]
pub type AportymasterdisR = crate::BitReader;
#[doc = "Field `APORTYMASTERDIS` writer - APORT Bus Y Master Disable"]
pub type AportymasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTVMASTERDIS` reader - APORT Bus Master Disable for Bus Selected By VASEL"]
pub type AportvmasterdisR = crate::BitReader;
#[doc = "Field `APORTVMASTERDIS` writer - APORT Bus Master Disable for Bus Selected By VASEL"]
pub type AportvmasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Power Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWRSEL {
    #[doc = "0: AVDD supply"]
    Avdd = 0,
    #[doc = "1: DVDD supply"]
    Dvdd = 1,
    #[doc = "2: IOVDD/IOVDD0 supply"]
    Iovdd0 = 2,
    #[doc = "4: IOVDD1 supply (if part has two I/O voltages)"]
    Iovdd1 = 4,
}
impl From<PWRSEL> for u8 {
    #[inline(always)]
    fn from(variant: PWRSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWRSEL {
    type Ux = u8;
}
impl crate::IsEnum for PWRSEL {}
#[doc = "Field `PWRSEL` reader - Power Select"]
pub type PwrselR = crate::FieldReader<PWRSEL>;
impl PwrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWRSEL> {
        match self.bits {
            0 => Some(PWRSEL::Avdd),
            1 => Some(PWRSEL::Dvdd),
            2 => Some(PWRSEL::Iovdd0),
            4 => Some(PWRSEL::Iovdd1),
            _ => None,
        }
    }
    #[doc = "AVDD supply"]
    #[inline(always)]
    pub fn is_avdd(&self) -> bool {
        *self == PWRSEL::Avdd
    }
    #[doc = "DVDD supply"]
    #[inline(always)]
    pub fn is_dvdd(&self) -> bool {
        *self == PWRSEL::Dvdd
    }
    #[doc = "IOVDD/IOVDD0 supply"]
    #[inline(always)]
    pub fn is_iovdd0(&self) -> bool {
        *self == PWRSEL::Iovdd0
    }
    #[doc = "IOVDD1 supply (if part has two I/O voltages)"]
    #[inline(always)]
    pub fn is_iovdd1(&self) -> bool {
        *self == PWRSEL::Iovdd1
    }
}
#[doc = "Field `PWRSEL` writer - Power Select"]
pub type PwrselW<'a, REG> = crate::FieldWriter<'a, REG, 3, PWRSEL>;
impl<'a, REG> PwrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AVDD supply"]
    #[inline(always)]
    pub fn avdd(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSEL::Avdd)
    }
    #[doc = "DVDD supply"]
    #[inline(always)]
    pub fn dvdd(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSEL::Dvdd)
    }
    #[doc = "IOVDD/IOVDD0 supply"]
    #[inline(always)]
    pub fn iovdd0(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSEL::Iovdd0)
    }
    #[doc = "IOVDD1 supply (if part has two I/O voltages)"]
    #[inline(always)]
    pub fn iovdd1(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSEL::Iovdd1)
    }
}
#[doc = "Field `ACCURACY` reader - ACMP Accuracy Mode"]
pub type AccuracyR = crate::BitReader;
#[doc = "Field `ACCURACY` writer - ACMP Accuracy Mode"]
pub type AccuracyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUTRANGE {
    #[doc = "0: Setting when the input can be from 0 to ACMPVDD."]
    Full = 0,
    #[doc = "1: Setting when the input will always be greater than ACMPVDD/2."]
    Gtvdddiv2 = 1,
    #[doc = "2: Setting when the input will always be less than ACMPVDD/2."]
    Ltvdddiv2 = 2,
}
impl From<INPUTRANGE> for u8 {
    #[inline(always)]
    fn from(variant: INPUTRANGE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPUTRANGE {
    type Ux = u8;
}
impl crate::IsEnum for INPUTRANGE {}
#[doc = "Field `INPUTRANGE` reader - Input Range"]
pub type InputrangeR = crate::FieldReader<INPUTRANGE>;
impl InputrangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INPUTRANGE> {
        match self.bits {
            0 => Some(INPUTRANGE::Full),
            1 => Some(INPUTRANGE::Gtvdddiv2),
            2 => Some(INPUTRANGE::Ltvdddiv2),
            _ => None,
        }
    }
    #[doc = "Setting when the input can be from 0 to ACMPVDD."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == INPUTRANGE::Full
    }
    #[doc = "Setting when the input will always be greater than ACMPVDD/2."]
    #[inline(always)]
    pub fn is_gtvdddiv2(&self) -> bool {
        *self == INPUTRANGE::Gtvdddiv2
    }
    #[doc = "Setting when the input will always be less than ACMPVDD/2."]
    #[inline(always)]
    pub fn is_ltvdddiv2(&self) -> bool {
        *self == INPUTRANGE::Ltvdddiv2
    }
}
#[doc = "Field `INPUTRANGE` writer - Input Range"]
pub type InputrangeW<'a, REG> = crate::FieldWriter<'a, REG, 2, INPUTRANGE>;
impl<'a, REG> InputrangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Setting when the input can be from 0 to ACMPVDD."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTRANGE::Full)
    }
    #[doc = "Setting when the input will always be greater than ACMPVDD/2."]
    #[inline(always)]
    pub fn gtvdddiv2(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTRANGE::Gtvdddiv2)
    }
    #[doc = "Setting when the input will always be less than ACMPVDD/2."]
    #[inline(always)]
    pub fn ltvdddiv2(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTRANGE::Ltvdddiv2)
    }
}
#[doc = "Field `IRISE` reader - Rising Edge Interrupt Sense"]
pub type IriseR = crate::BitReader;
#[doc = "Field `IRISE` writer - Rising Edge Interrupt Sense"]
pub type IriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFALL` reader - Falling Edge Interrupt Sense"]
pub type IfallR = crate::BitReader;
#[doc = "Field `IFALL` writer - Falling Edge Interrupt Sense"]
pub type IfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIASPROG` reader - Bias Configuration"]
pub type BiasprogR = crate::FieldReader;
#[doc = "Field `BIASPROG` writer - Bias Configuration"]
pub type BiasprogW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FULLBIAS` reader - Full Bias Current"]
pub type FullbiasR = crate::BitReader;
#[doc = "Field `FULLBIAS` writer - Full Bias Current"]
pub type FullbiasW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    pub fn inactval(&self) -> InactvalR {
        InactvalR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator GPIO Output Invert"]
    #[inline(always)]
    pub fn gpioinv(&self) -> GpioinvR {
        GpioinvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - APORT Bus X Master Disable"]
    #[inline(always)]
    pub fn aportxmasterdis(&self) -> AportxmasterdisR {
        AportxmasterdisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - APORT Bus Y Master Disable"]
    #[inline(always)]
    pub fn aportymasterdis(&self) -> AportymasterdisR {
        AportymasterdisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - APORT Bus Master Disable for Bus Selected By VASEL"]
    #[inline(always)]
    pub fn aportvmasterdis(&self) -> AportvmasterdisR {
        AportvmasterdisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Power Select"]
    #[inline(always)]
    pub fn pwrsel(&self) -> PwrselR {
        PwrselR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - ACMP Accuracy Mode"]
    #[inline(always)]
    pub fn accuracy(&self) -> AccuracyR {
        AccuracyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Input Range"]
    #[inline(always)]
    pub fn inputrange(&self) -> InputrangeR {
        InputrangeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Rising Edge Interrupt Sense"]
    #[inline(always)]
    pub fn irise(&self) -> IriseR {
        IriseR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Falling Edge Interrupt Sense"]
    #[inline(always)]
    pub fn ifall(&self) -> IfallR {
        IfallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Bias Configuration"]
    #[inline(always)]
    pub fn biasprog(&self) -> BiasprogR {
        BiasprogR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Full Bias Current"]
    #[inline(always)]
    pub fn fullbias(&self) -> FullbiasR {
        FullbiasR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CTRLrs> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    #[must_use]
    pub fn inactval(&mut self) -> InactvalW<CTRLrs> {
        InactvalW::new(self, 2)
    }
    #[doc = "Bit 3 - Comparator GPIO Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn gpioinv(&mut self) -> GpioinvW<CTRLrs> {
        GpioinvW::new(self, 3)
    }
    #[doc = "Bit 8 - APORT Bus X Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aportxmasterdis(&mut self) -> AportxmasterdisW<CTRLrs> {
        AportxmasterdisW::new(self, 8)
    }
    #[doc = "Bit 9 - APORT Bus Y Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aportymasterdis(&mut self) -> AportymasterdisW<CTRLrs> {
        AportymasterdisW::new(self, 9)
    }
    #[doc = "Bit 10 - APORT Bus Master Disable for Bus Selected By VASEL"]
    #[inline(always)]
    #[must_use]
    pub fn aportvmasterdis(&mut self) -> AportvmasterdisW<CTRLrs> {
        AportvmasterdisW::new(self, 10)
    }
    #[doc = "Bits 12:14 - Power Select"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsel(&mut self) -> PwrselW<CTRLrs> {
        PwrselW::new(self, 12)
    }
    #[doc = "Bit 15 - ACMP Accuracy Mode"]
    #[inline(always)]
    #[must_use]
    pub fn accuracy(&mut self) -> AccuracyW<CTRLrs> {
        AccuracyW::new(self, 15)
    }
    #[doc = "Bits 18:19 - Input Range"]
    #[inline(always)]
    #[must_use]
    pub fn inputrange(&mut self) -> InputrangeW<CTRLrs> {
        InputrangeW::new(self, 18)
    }
    #[doc = "Bit 20 - Rising Edge Interrupt Sense"]
    #[inline(always)]
    #[must_use]
    pub fn irise(&mut self) -> IriseW<CTRLrs> {
        IriseW::new(self, 20)
    }
    #[doc = "Bit 21 - Falling Edge Interrupt Sense"]
    #[inline(always)]
    #[must_use]
    pub fn ifall(&mut self) -> IfallW<CTRLrs> {
        IfallW::new(self, 21)
    }
    #[doc = "Bits 24:29 - Bias Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn biasprog(&mut self) -> BiasprogW<CTRLrs> {
        BiasprogW::new(self, 24)
    }
    #[doc = "Bit 31 - Full Bias Current"]
    #[inline(always)]
    #[must_use]
    pub fn fullbias(&mut self) -> FullbiasW<CTRLrs> {
        FullbiasW::new(self, 31)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0700_0000"]
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0x0700_0000;
}
