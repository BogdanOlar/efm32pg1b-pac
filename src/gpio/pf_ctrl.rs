#[doc = "Register `PF_CTRL` reader"]
pub type R = crate::R<PF_CTRL_SPEC>;
#[doc = "Register `PF_CTRL` writer"]
pub type W = crate::W<PF_CTRL_SPEC>;
#[doc = "Field `DRIVESTRENGTH` reader - Drive Strength for Port"]
pub type DRIVESTRENGTH_R = crate::BitReader;
#[doc = "Field `DRIVESTRENGTH` writer - Drive Strength for Port"]
pub type DRIVESTRENGTH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLEWRATE` reader - Slewrate Limit for Port"]
pub type SLEWRATE_R = crate::FieldReader;
#[doc = "Field `SLEWRATE` writer - Slewrate Limit for Port"]
pub type SLEWRATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DINDIS` reader - Data in Disable"]
pub type DINDIS_R = crate::BitReader;
#[doc = "Field `DINDIS` writer - Data in Disable"]
pub type DINDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRIVESTRENGTHALT` reader - Alternate Drive Strength for Port"]
pub type DRIVESTRENGTHALT_R = crate::BitReader;
#[doc = "Field `DRIVESTRENGTHALT` writer - Alternate Drive Strength for Port"]
pub type DRIVESTRENGTHALT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLEWRATEALT` reader - Alternate Slewrate Limit for Port"]
pub type SLEWRATEALT_R = crate::FieldReader;
#[doc = "Field `SLEWRATEALT` writer - Alternate Slewrate Limit for Port"]
pub type SLEWRATEALT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DINDISALT` reader - Alternate Data in Disable"]
pub type DINDISALT_R = crate::BitReader;
#[doc = "Field `DINDISALT` writer - Alternate Data in Disable"]
pub type DINDISALT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Drive Strength for Port"]
    #[inline(always)]
    pub fn drivestrength(&self) -> DRIVESTRENGTH_R {
        DRIVESTRENGTH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slewrate(&self) -> SLEWRATE_R {
        SLEWRATE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 12 - Data in Disable"]
    #[inline(always)]
    pub fn dindis(&self) -> DINDIS_R {
        DINDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Alternate Drive Strength for Port"]
    #[inline(always)]
    pub fn drivestrengthalt(&self) -> DRIVESTRENGTHALT_R {
        DRIVESTRENGTHALT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Alternate Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slewratealt(&self) -> SLEWRATEALT_R {
        SLEWRATEALT_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 28 - Alternate Data in Disable"]
    #[inline(always)]
    pub fn dindisalt(&self) -> DINDISALT_R {
        DINDISALT_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PF_CTRL")
            .field(
                "drivestrength",
                &format_args!("{}", self.drivestrength().bit()),
            )
            .field("slewrate", &format_args!("{}", self.slewrate().bits()))
            .field("dindis", &format_args!("{}", self.dindis().bit()))
            .field(
                "drivestrengthalt",
                &format_args!("{}", self.drivestrengthalt().bit()),
            )
            .field(
                "slewratealt",
                &format_args!("{}", self.slewratealt().bits()),
            )
            .field("dindisalt", &format_args!("{}", self.dindisalt().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PF_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Drive Strength for Port"]
    #[inline(always)]
    #[must_use]
    pub fn drivestrength(&mut self) -> DRIVESTRENGTH_W<PF_CTRL_SPEC, 0> {
        DRIVESTRENGTH_W::new(self)
    }
    #[doc = "Bits 4:6 - Slewrate Limit for Port"]
    #[inline(always)]
    #[must_use]
    pub fn slewrate(&mut self) -> SLEWRATE_W<PF_CTRL_SPEC, 4> {
        SLEWRATE_W::new(self)
    }
    #[doc = "Bit 12 - Data in Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dindis(&mut self) -> DINDIS_W<PF_CTRL_SPEC, 12> {
        DINDIS_W::new(self)
    }
    #[doc = "Bit 16 - Alternate Drive Strength for Port"]
    #[inline(always)]
    #[must_use]
    pub fn drivestrengthalt(&mut self) -> DRIVESTRENGTHALT_W<PF_CTRL_SPEC, 16> {
        DRIVESTRENGTHALT_W::new(self)
    }
    #[doc = "Bits 20:22 - Alternate Slewrate Limit for Port"]
    #[inline(always)]
    #[must_use]
    pub fn slewratealt(&mut self) -> SLEWRATEALT_W<PF_CTRL_SPEC, 20> {
        SLEWRATEALT_W::new(self)
    }
    #[doc = "Bit 28 - Alternate Data in Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dindisalt(&mut self) -> DINDISALT_W<PF_CTRL_SPEC, 28> {
        DINDISALT_W::new(self)
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
#[doc = "Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PF_CTRL_SPEC;
impl crate::RegisterSpec for PF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_ctrl::R`](R) reader structure"]
impl crate::Readable for PF_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pf_ctrl::W`](W) writer structure"]
impl crate::Writable for PF_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PF_CTRL to value 0x0050_0050"]
impl crate::Resettable for PF_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0050_0050;
}
