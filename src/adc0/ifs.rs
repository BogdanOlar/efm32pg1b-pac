#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFSrs>;
#[doc = "Field `SINGLEOF` writer - Set SINGLEOF Interrupt Flag"]
pub type SINGLEOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANOF` writer - Set SCANOF Interrupt Flag"]
pub type SCANOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEUF` writer - Set SINGLEUF Interrupt Flag"]
pub type SINGLEUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANUF` writer - Set SCANUF Interrupt Flag"]
pub type SCANUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLECMP` writer - Set SINGLECMP Interrupt Flag"]
pub type SINGLECMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANCMP` writer - Set SCANCMP Interrupt Flag"]
pub type SCANCMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFOV` writer - Set VREFOV Interrupt Flag"]
pub type VREFOV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGERR` writer - Set PROGERR Interrupt Flag"]
pub type PROGERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 8 - Set SINGLEOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn singleof(&mut self) -> SINGLEOF_W<IFSrs> {
        SINGLEOF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set SCANOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scanof(&mut self) -> SCANOF_W<IFSrs> {
        SCANOF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set SINGLEUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn singleuf(&mut self) -> SINGLEUF_W<IFSrs> {
        SINGLEUF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set SCANUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scanuf(&mut self) -> SCANUF_W<IFSrs> {
        SCANUF_W::new(self, 11)
    }
    #[doc = "Bit 16 - Set SINGLECMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn singlecmp(&mut self) -> SINGLECMP_W<IFSrs> {
        SINGLECMP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set SCANCMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scancmp(&mut self) -> SCANCMP_W<IFSrs> {
        SCANCMP_W::new(self, 17)
    }
    #[doc = "Bit 24 - Set VREFOV Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vrefov(&mut self) -> VREFOV_W<IFSrs> {
        VREFOV_W::new(self, 24)
    }
    #[doc = "Bit 25 - Set PROGERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn progerr(&mut self) -> PROGERR_W<IFSrs> {
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
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFSrs;
impl crate::RegisterSpec for IFSrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFSrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFSrs {
    const RESET_VALUE: u32 = 0;
}
