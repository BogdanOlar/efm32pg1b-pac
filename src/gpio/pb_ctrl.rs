#[doc = "Register `PB_CTRL` reader"]
pub type R = crate::R<PB_CTRLrs>;
#[doc = "Register `PB_CTRL` writer"]
pub type W = crate::W<PB_CTRLrs>;
#[doc = "Field `DRIVESTRENGTH` reader - Drive Strength for Port"]
pub type DrivestrengthR = crate::BitReader;
#[doc = "Field `DRIVESTRENGTH` writer - Drive Strength for Port"]
pub type DrivestrengthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEWRATE` reader - Slewrate Limit for Port"]
pub type SlewrateR = crate::FieldReader;
#[doc = "Field `SLEWRATE` writer - Slewrate Limit for Port"]
pub type SlewrateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DINDIS` reader - Data in Disable"]
pub type DindisR = crate::BitReader;
#[doc = "Field `DINDIS` writer - Data in Disable"]
pub type DindisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVESTRENGTHALT` reader - Alternate Drive Strength for Port"]
pub type DrivestrengthaltR = crate::BitReader;
#[doc = "Field `DRIVESTRENGTHALT` writer - Alternate Drive Strength for Port"]
pub type DrivestrengthaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEWRATEALT` reader - Alternate Slewrate Limit for Port"]
pub type SlewratealtR = crate::FieldReader;
#[doc = "Field `SLEWRATEALT` writer - Alternate Slewrate Limit for Port"]
pub type SlewratealtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DINDISALT` reader - Alternate Data in Disable"]
pub type DindisaltR = crate::BitReader;
#[doc = "Field `DINDISALT` writer - Alternate Data in Disable"]
pub type DindisaltW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Drive Strength for Port"]
    #[inline(always)]
    pub fn drivestrength(&self) -> DrivestrengthR {
        DrivestrengthR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slewrate(&self) -> SlewrateR {
        SlewrateR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 12 - Data in Disable"]
    #[inline(always)]
    pub fn dindis(&self) -> DindisR {
        DindisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Alternate Drive Strength for Port"]
    #[inline(always)]
    pub fn drivestrengthalt(&self) -> DrivestrengthaltR {
        DrivestrengthaltR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Alternate Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slewratealt(&self) -> SlewratealtR {
        SlewratealtR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 28 - Alternate Data in Disable"]
    #[inline(always)]
    pub fn dindisalt(&self) -> DindisaltR {
        DindisaltR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drive Strength for Port"]
    #[inline(always)]
    #[must_use]
    pub fn drivestrength(&mut self) -> DrivestrengthW<PB_CTRLrs> {
        DrivestrengthW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Slewrate Limit for Port"]
    #[inline(always)]
    #[must_use]
    pub fn slewrate(&mut self) -> SlewrateW<PB_CTRLrs> {
        SlewrateW::new(self, 4)
    }
    #[doc = "Bit 12 - Data in Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dindis(&mut self) -> DindisW<PB_CTRLrs> {
        DindisW::new(self, 12)
    }
    #[doc = "Bit 16 - Alternate Drive Strength for Port"]
    #[inline(always)]
    #[must_use]
    pub fn drivestrengthalt(&mut self) -> DrivestrengthaltW<PB_CTRLrs> {
        DrivestrengthaltW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Alternate Slewrate Limit for Port"]
    #[inline(always)]
    #[must_use]
    pub fn slewratealt(&mut self) -> SlewratealtW<PB_CTRLrs> {
        SlewratealtW::new(self, 20)
    }
    #[doc = "Bit 28 - Alternate Data in Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dindisalt(&mut self) -> DindisaltW<PB_CTRLrs> {
        DindisaltW::new(self, 28)
    }
}
#[doc = "Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PB_CTRLrs;
impl crate::RegisterSpec for PB_CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_ctrl::R`](R) reader structure"]
impl crate::Readable for PB_CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`pb_ctrl::W`](W) writer structure"]
impl crate::Writable for PB_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB_CTRL to value 0x0050_0050"]
impl crate::Resettable for PB_CTRLrs {
    const RESET_VALUE: u32 = 0x0050_0050;
}
