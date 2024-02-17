#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFCrs>;
#[doc = "Field `SINGLEOF` writer - Clear SINGLEOF Interrupt Flag"]
pub type SINGLEOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANOF` writer - Clear SCANOF Interrupt Flag"]
pub type SCANOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEUF` writer - Clear SINGLEUF Interrupt Flag"]
pub type SINGLEUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANUF` writer - Clear SCANUF Interrupt Flag"]
pub type SCANUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLECMP` writer - Clear SINGLECMP Interrupt Flag"]
pub type SINGLECMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANCMP` writer - Clear SCANCMP Interrupt Flag"]
pub type SCANCMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFOV` writer - Clear VREFOV Interrupt Flag"]
pub type VREFOV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGERR` writer - Clear PROGERR Interrupt Flag"]
pub type PROGERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 8 - Clear SINGLEOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn singleof(&mut self) -> SINGLEOF_W<IFCrs> {
        SINGLEOF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear SCANOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scanof(&mut self) -> SCANOF_W<IFCrs> {
        SCANOF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear SINGLEUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn singleuf(&mut self) -> SINGLEUF_W<IFCrs> {
        SINGLEUF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear SCANUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scanuf(&mut self) -> SCANUF_W<IFCrs> {
        SCANUF_W::new(self, 11)
    }
    #[doc = "Bit 16 - Clear SINGLECMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn singlecmp(&mut self) -> SINGLECMP_W<IFCrs> {
        SINGLECMP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear SCANCMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scancmp(&mut self) -> SCANCMP_W<IFCrs> {
        SCANCMP_W::new(self, 17)
    }
    #[doc = "Bit 24 - Clear VREFOV Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vrefov(&mut self) -> VREFOV_W<IFCrs> {
        VREFOV_W::new(self, 24)
    }
    #[doc = "Bit 25 - Clear PROGERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn progerr(&mut self) -> PROGERR_W<IFCrs> {
        PROGERR_W::new(self, 25)
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
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCrs;
impl crate::RegisterSpec for IFCrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFCrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFCrs {
    const RESET_VALUE: u32 = 0;
}
