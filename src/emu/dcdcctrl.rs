#[doc = "Register `DCDCCTRL` reader"]
pub type R = crate::R<DCDCCTRLrs>;
#[doc = "Register `DCDCCTRL` writer"]
pub type W = crate::W<DCDCCTRLrs>;
#[doc = "Regulator Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCDCMODE {
    #[doc = "0: DCDC regulator is operating in bypass mode. Prior to configuring DCDCMODE=BYPASS, software must set EMU_DCDCCLIMCTRL.BYPLIMEN=1 to prevent excessive current between VREGVDD and DVDD supplies."]
    Bypass = 0,
    #[doc = "1: DCDC regulator is operating in low noise mode."]
    Lownoise = 1,
    #[doc = "2: DCDC regulator is operating in low power mode."]
    Lowpower = 2,
    #[doc = "3: DCDC regulator is off and the bypass switch is off. Note: DVDD must be supplied externally"]
    Off = 3,
}
impl From<DCDCMODE> for u8 {
    #[inline(always)]
    fn from(variant: DCDCMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DCDCMODE {
    type Ux = u8;
}
impl crate::IsEnum for DCDCMODE {}
#[doc = "Field `DCDCMODE` reader - Regulator Mode"]
pub type DcdcmodeR = crate::FieldReader<DCDCMODE>;
impl DcdcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCDCMODE {
        match self.bits {
            0 => DCDCMODE::Bypass,
            1 => DCDCMODE::Lownoise,
            2 => DCDCMODE::Lowpower,
            3 => DCDCMODE::Off,
            _ => unreachable!(),
        }
    }
    #[doc = "DCDC regulator is operating in bypass mode. Prior to configuring DCDCMODE=BYPASS, software must set EMU_DCDCCLIMCTRL.BYPLIMEN=1 to prevent excessive current between VREGVDD and DVDD supplies."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == DCDCMODE::Bypass
    }
    #[doc = "DCDC regulator is operating in low noise mode."]
    #[inline(always)]
    pub fn is_lownoise(&self) -> bool {
        *self == DCDCMODE::Lownoise
    }
    #[doc = "DCDC regulator is operating in low power mode."]
    #[inline(always)]
    pub fn is_lowpower(&self) -> bool {
        *self == DCDCMODE::Lowpower
    }
    #[doc = "DCDC regulator is off and the bypass switch is off. Note: DVDD must be supplied externally"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DCDCMODE::Off
    }
}
#[doc = "Field `DCDCMODE` writer - Regulator Mode"]
pub type DcdcmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, DCDCMODE, crate::Safe>;
impl<'a, REG> DcdcmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DCDC regulator is operating in bypass mode. Prior to configuring DCDCMODE=BYPASS, software must set EMU_DCDCCLIMCTRL.BYPLIMEN=1 to prevent excessive current between VREGVDD and DVDD supplies."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(DCDCMODE::Bypass)
    }
    #[doc = "DCDC regulator is operating in low noise mode."]
    #[inline(always)]
    pub fn lownoise(self) -> &'a mut crate::W<REG> {
        self.variant(DCDCMODE::Lownoise)
    }
    #[doc = "DCDC regulator is operating in low power mode."]
    #[inline(always)]
    pub fn lowpower(self) -> &'a mut crate::W<REG> {
        self.variant(DCDCMODE::Lowpower)
    }
    #[doc = "DCDC regulator is off and the bypass switch is off. Note: DVDD must be supplied externally"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(DCDCMODE::Off)
    }
}
#[doc = "Field `DCDCMODEEM23` reader - DCDC Mode EM23"]
pub type Dcdcmodeem23R = crate::BitReader;
#[doc = "Field `DCDCMODEEM23` writer - DCDC Mode EM23"]
pub type Dcdcmodeem23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCMODEEM4` reader - DCDC Mode EM4H"]
pub type Dcdcmodeem4R = crate::BitReader;
#[doc = "Field `DCDCMODEEM4` writer - DCDC Mode EM4H"]
pub type Dcdcmodeem4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Regulator Mode"]
    #[inline(always)]
    pub fn dcdcmode(&self) -> DcdcmodeR {
        DcdcmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - DCDC Mode EM23"]
    #[inline(always)]
    pub fn dcdcmodeem23(&self) -> Dcdcmodeem23R {
        Dcdcmodeem23R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DCDC Mode EM4H"]
    #[inline(always)]
    pub fn dcdcmodeem4(&self) -> Dcdcmodeem4R {
        Dcdcmodeem4R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDCCTRL")
            .field("dcdcmode", &self.dcdcmode())
            .field("dcdcmodeem23", &self.dcdcmodeem23())
            .field("dcdcmodeem4", &self.dcdcmodeem4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Regulator Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcmode(&mut self) -> DcdcmodeW<DCDCCTRLrs> {
        DcdcmodeW::new(self, 0)
    }
    #[doc = "Bit 4 - DCDC Mode EM23"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcmodeem23(&mut self) -> Dcdcmodeem23W<DCDCCTRLrs> {
        Dcdcmodeem23W::new(self, 4)
    }
    #[doc = "Bit 5 - DCDC Mode EM4H"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcmodeem4(&mut self) -> Dcdcmodeem4W<DCDCCTRLrs> {
        Dcdcmodeem4W::new(self, 5)
    }
}
#[doc = "DCDC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDCCTRLrs;
impl crate::RegisterSpec for DCDCCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdcctrl::R`](R) reader structure"]
impl crate::Readable for DCDCCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`dcdcctrl::W`](W) writer structure"]
impl crate::Writable for DCDCCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDCCTRL to value 0x30"]
impl crate::Resettable for DCDCCTRLrs {
    const RESET_VALUE: u32 = 0x30;
}
