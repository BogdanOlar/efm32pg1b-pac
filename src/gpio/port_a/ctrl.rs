///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///Field `DRIVE_STRENGTH` reader - Drive strength setting for port pins not using alternate modes. Value `0` is STRONG (10mA), value `1` is WEAK (1mA)
pub type DriveStrengthR = crate::BitReader;
///Field `DRIVE_STRENGTH` writer - Drive strength setting for port pins not using alternate modes. Value `0` is STRONG (10mA), value `1` is WEAK (1mA)
pub type DriveStrengthW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLEW_RATE` reader - Slewrate limit for port pins not using alternate modes. Higher values represent faster slewrates
pub type SlewRateR = crate::FieldReader;
///Field `SLEW_RATE` writer - Slewrate limit for port pins not using alternate modes. Higher values represent faster slewrates
pub type SlewRateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DIN_DIS` reader - Data in Disable
pub type DinDisR = crate::BitReader;
///Field `DIN_DIS` writer - Data in Disable
pub type DinDisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DRIVE_STRENGTH_ALT` reader - Drive strength setting for port pins using alternate modes. Value `0` is STRONG (10mA), value `1` is WEAK (1mA)
pub type DriveStrengthAltR = crate::BitReader;
///Field `DRIVE_STRENGTH_ALT` writer - Drive strength setting for port pins using alternate modes. Value `0` is STRONG (10mA), value `1` is WEAK (1mA)
pub type DriveStrengthAltW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLEW_RATE_ALT` reader - Slewrate limit for port pins using alternate modes. Higher values represent faster slewrates
pub type SlewRateAltR = crate::FieldReader;
///Field `SLEW_RATE_ALT` writer - Slewrate limit for port pins using alternate modes. Higher values represent faster slewrates
pub type SlewRateAltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DIN_DIS_ALT` reader - Alternate Data in Disable
pub type DinDisAltR = crate::BitReader;
///Field `DIN_DIS_ALT` writer - Alternate Data in Disable
pub type DinDisAltW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Drive strength setting for port pins not using alternate modes. Value `0` is STRONG (10mA), value `1` is WEAK (1mA)
    #[inline(always)]
    pub fn drive_strength(&self) -> DriveStrengthR {
        DriveStrengthR::new((self.bits & 1) != 0)
    }
    ///Bits 4:6 - Slewrate limit for port pins not using alternate modes. Higher values represent faster slewrates
    #[inline(always)]
    pub fn slew_rate(&self) -> SlewRateR {
        SlewRateR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 12 - Data in Disable
    #[inline(always)]
    pub fn din_dis(&self) -> DinDisR {
        DinDisR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Drive strength setting for port pins using alternate modes. Value `0` is STRONG (10mA), value `1` is WEAK (1mA)
    #[inline(always)]
    pub fn drive_strength_alt(&self) -> DriveStrengthAltR {
        DriveStrengthAltR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:22 - Slewrate limit for port pins using alternate modes. Higher values represent faster slewrates
    #[inline(always)]
    pub fn slew_rate_alt(&self) -> SlewRateAltR {
        SlewRateAltR::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bit 28 - Alternate Data in Disable
    #[inline(always)]
    pub fn din_dis_alt(&self) -> DinDisAltR {
        DinDisAltR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("drive_strength", &self.drive_strength())
            .field("slew_rate", &self.slew_rate())
            .field("din_dis", &self.din_dis())
            .field("drive_strength_alt", &self.drive_strength_alt())
            .field("slew_rate_alt", &self.slew_rate_alt())
            .field("din_dis_alt", &self.din_dis_alt())
            .finish()
    }
}
impl W {
    ///Bit 0 - Drive strength setting for port pins not using alternate modes. Value `0` is STRONG (10mA), value `1` is WEAK (1mA)
    #[inline(always)]
    #[must_use]
    pub fn drive_strength(&mut self) -> DriveStrengthW<CTRLrs> {
        DriveStrengthW::new(self, 0)
    }
    ///Bits 4:6 - Slewrate limit for port pins not using alternate modes. Higher values represent faster slewrates
    #[inline(always)]
    #[must_use]
    pub fn slew_rate(&mut self) -> SlewRateW<CTRLrs> {
        SlewRateW::new(self, 4)
    }
    ///Bit 12 - Data in Disable
    #[inline(always)]
    #[must_use]
    pub fn din_dis(&mut self) -> DinDisW<CTRLrs> {
        DinDisW::new(self, 12)
    }
    ///Bit 16 - Drive strength setting for port pins using alternate modes. Value `0` is STRONG (10mA), value `1` is WEAK (1mA)
    #[inline(always)]
    #[must_use]
    pub fn drive_strength_alt(&mut self) -> DriveStrengthAltW<CTRLrs> {
        DriveStrengthAltW::new(self, 16)
    }
    ///Bits 20:22 - Slewrate limit for port pins using alternate modes. Higher values represent faster slewrates
    #[inline(always)]
    #[must_use]
    pub fn slew_rate_alt(&mut self) -> SlewRateAltW<CTRLrs> {
        SlewRateAltW::new(self, 20)
    }
    ///Bit 28 - Alternate Data in Disable
    #[inline(always)]
    #[must_use]
    pub fn din_dis_alt(&mut self) -> DinDisAltW<CTRLrs> {
        DinDisAltW::new(self, 28)
    }
}
///Port Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ctrl::R`](R) reader structure
impl crate::Readable for CTRLrs {}
///`write(|w| ..)` method takes [`ctrl::W`](W) writer structure
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTRL to value 0x0050_0050
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0x0050_0050;
}
