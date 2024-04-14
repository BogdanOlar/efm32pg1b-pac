#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRLrs>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRLrs>;
#[doc = "Field `DRIVE_STRENGTH` reader - Drive Strength for Port"]
pub type DriveStrengthR = crate::BitReader;
#[doc = "Field `DRIVE_STRENGTH` writer - Drive Strength for Port"]
pub type DriveStrengthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEW_RATE` reader - Slewrate Limit for Port"]
pub type SlewRateR = crate::FieldReader;
#[doc = "Field `SLEW_RATE` writer - Slewrate Limit for Port"]
pub type SlewRateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIN_DIS` reader - Data in Disable"]
pub type DinDisR = crate::BitReader;
#[doc = "Field `DIN_DIS` writer - Data in Disable"]
pub type DinDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVE_STRENGTH_ALT` reader - Alternate Drive Strength for Port"]
pub type DriveStrengthAltR = crate::BitReader;
#[doc = "Field `DRIVE_STRENGTH_ALT` writer - Alternate Drive Strength for Port"]
pub type DriveStrengthAltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEW_RATE_ALT` reader - Alternate Slewrate Limit for Port"]
pub type SlewRateAltR = crate::FieldReader;
#[doc = "Field `SLEW_RATE_ALT` writer - Alternate Slewrate Limit for Port"]
pub type SlewRateAltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIN_DIS_ALT` reader - Alternate Data in Disable"]
pub type DinDisAltR = crate::BitReader;
#[doc = "Field `DIN_DIS_ALT` writer - Alternate Data in Disable"]
pub type DinDisAltW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Drive Strength for Port"]
    #[inline(always)]
    pub fn drive_strength(&self) -> DriveStrengthR {
        DriveStrengthR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slew_rate(&self) -> SlewRateR {
        SlewRateR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 12 - Data in Disable"]
    #[inline(always)]
    pub fn din_dis(&self) -> DinDisR {
        DinDisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Alternate Drive Strength for Port"]
    #[inline(always)]
    pub fn drive_strength_alt(&self) -> DriveStrengthAltR {
        DriveStrengthAltR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Alternate Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slew_rate_alt(&self) -> SlewRateAltR {
        SlewRateAltR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 28 - Alternate Data in Disable"]
    #[inline(always)]
    pub fn din_dis_alt(&self) -> DinDisAltR {
        DinDisAltR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drive Strength for Port"]
    #[inline(always)]
    #[must_use]
    pub fn drive_strength(&mut self) -> DriveStrengthW<CTRLrs> {
        DriveStrengthW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Slewrate Limit for Port"]
    #[inline(always)]
    #[must_use]
    pub fn slew_rate(&mut self) -> SlewRateW<CTRLrs> {
        SlewRateW::new(self, 4)
    }
    #[doc = "Bit 12 - Data in Disable"]
    #[inline(always)]
    #[must_use]
    pub fn din_dis(&mut self) -> DinDisW<CTRLrs> {
        DinDisW::new(self, 12)
    }
    #[doc = "Bit 16 - Alternate Drive Strength for Port"]
    #[inline(always)]
    #[must_use]
    pub fn drive_strength_alt(&mut self) -> DriveStrengthAltW<CTRLrs> {
        DriveStrengthAltW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Alternate Slewrate Limit for Port"]
    #[inline(always)]
    #[must_use]
    pub fn slew_rate_alt(&mut self) -> SlewRateAltW<CTRLrs> {
        SlewRateAltW::new(self, 20)
    }
    #[doc = "Bit 28 - Alternate Data in Disable"]
    #[inline(always)]
    #[must_use]
    pub fn din_dis_alt(&mut self) -> DinDisAltW<CTRLrs> {
        DinDisAltW::new(self, 28)
    }
}
#[doc = "Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0x0050_0050"]
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0x0050_0050;
}
