#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFSrs>;
#[doc = "Field `ERASE` writer - Set ERASE Interrupt Flag"]
pub type ERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` writer - Set WRITE Interrupt Flag"]
pub type WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOF` writer - Set CHOF Interrupt Flag"]
pub type CHOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMOF` writer - Set CMOF Interrupt Flag"]
pub type CMOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPF` writer - Set PWRUPF Interrupt Flag"]
pub type PWRUPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHERR` writer - Set ICACHERR Interrupt Flag"]
pub type ICACHERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set ERASE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> ERASE_W<IFSrs> {
        ERASE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set WRITE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<IFSrs> {
        WRITE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set CHOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn chof(&mut self) -> CHOF_W<IFSrs> {
        CHOF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set CMOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmof(&mut self) -> CMOF_W<IFSrs> {
        CMOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set PWRUPF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pwrupf(&mut self) -> PWRUPF_W<IFSrs> {
        PWRUPF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set ICACHERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icacherr(&mut self) -> ICACHERR_W<IFSrs> {
        ICACHERR_W::new(self, 5)
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
