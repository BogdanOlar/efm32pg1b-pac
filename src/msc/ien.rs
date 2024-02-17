#[doc = "Register `IEN` reader"]
pub type R = crate::R<IENrs>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IENrs>;
#[doc = "Field `ERASE` reader - ERASE Interrupt Enable"]
pub type ERASE_R = crate::BitReader;
#[doc = "Field `ERASE` writer - ERASE Interrupt Enable"]
pub type ERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` reader - WRITE Interrupt Enable"]
pub type WRITE_R = crate::BitReader;
#[doc = "Field `WRITE` writer - WRITE Interrupt Enable"]
pub type WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOF` reader - CHOF Interrupt Enable"]
pub type CHOF_R = crate::BitReader;
#[doc = "Field `CHOF` writer - CHOF Interrupt Enable"]
pub type CHOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMOF` reader - CMOF Interrupt Enable"]
pub type CMOF_R = crate::BitReader;
#[doc = "Field `CMOF` writer - CMOF Interrupt Enable"]
pub type CMOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPF` reader - PWRUPF Interrupt Enable"]
pub type PWRUPF_R = crate::BitReader;
#[doc = "Field `PWRUPF` writer - PWRUPF Interrupt Enable"]
pub type PWRUPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHERR` reader - ICACHERR Interrupt Enable"]
pub type ICACHERR_R = crate::BitReader;
#[doc = "Field `ICACHERR` writer - ICACHERR Interrupt Enable"]
pub type ICACHERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ERASE Interrupt Enable"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WRITE Interrupt Enable"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CHOF Interrupt Enable"]
    #[inline(always)]
    pub fn chof(&self) -> CHOF_R {
        CHOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMOF Interrupt Enable"]
    #[inline(always)]
    pub fn cmof(&self) -> CMOF_R {
        CMOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWRUPF Interrupt Enable"]
    #[inline(always)]
    pub fn pwrupf(&self) -> PWRUPF_R {
        PWRUPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ICACHERR Interrupt Enable"]
    #[inline(always)]
    pub fn icacherr(&self) -> ICACHERR_R {
        ICACHERR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ERASE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> ERASE_W<IENrs> {
        ERASE_W::new(self, 0)
    }
    #[doc = "Bit 1 - WRITE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<IENrs> {
        WRITE_W::new(self, 1)
    }
    #[doc = "Bit 2 - CHOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chof(&mut self) -> CHOF_W<IENrs> {
        CHOF_W::new(self, 2)
    }
    #[doc = "Bit 3 - CMOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmof(&mut self) -> CMOF_W<IENrs> {
        CMOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - PWRUPF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwrupf(&mut self) -> PWRUPF_W<IENrs> {
        PWRUPF_W::new(self, 4)
    }
    #[doc = "Bit 5 - ICACHERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icacherr(&mut self) -> ICACHERR_W<IENrs> {
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IENrs;
impl crate::RegisterSpec for IENrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IENrs {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IENrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IENrs {
    const RESET_VALUE: u32 = 0;
}
