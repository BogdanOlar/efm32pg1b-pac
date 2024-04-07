#[doc = "Register `IEN` reader"]
pub type R = crate::R<IENrs>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IENrs>;
#[doc = "Field `ERASE` reader - ERASE Interrupt Enable"]
pub type EraseR = crate::BitReader;
#[doc = "Field `ERASE` writer - ERASE Interrupt Enable"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` reader - WRITE Interrupt Enable"]
pub type WriteR = crate::BitReader;
#[doc = "Field `WRITE` writer - WRITE Interrupt Enable"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOF` reader - CHOF Interrupt Enable"]
pub type ChofR = crate::BitReader;
#[doc = "Field `CHOF` writer - CHOF Interrupt Enable"]
pub type ChofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMOF` reader - CMOF Interrupt Enable"]
pub type CmofR = crate::BitReader;
#[doc = "Field `CMOF` writer - CMOF Interrupt Enable"]
pub type CmofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPF` reader - PWRUPF Interrupt Enable"]
pub type PwrupfR = crate::BitReader;
#[doc = "Field `PWRUPF` writer - PWRUPF Interrupt Enable"]
pub type PwrupfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHERR` reader - ICACHERR Interrupt Enable"]
pub type IcacherrR = crate::BitReader;
#[doc = "Field `ICACHERR` writer - ICACHERR Interrupt Enable"]
pub type IcacherrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ERASE Interrupt Enable"]
    #[inline(always)]
    pub fn erase(&self) -> EraseR {
        EraseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WRITE Interrupt Enable"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CHOF Interrupt Enable"]
    #[inline(always)]
    pub fn chof(&self) -> ChofR {
        ChofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMOF Interrupt Enable"]
    #[inline(always)]
    pub fn cmof(&self) -> CmofR {
        CmofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWRUPF Interrupt Enable"]
    #[inline(always)]
    pub fn pwrupf(&self) -> PwrupfR {
        PwrupfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ICACHERR Interrupt Enable"]
    #[inline(always)]
    pub fn icacherr(&self) -> IcacherrR {
        IcacherrR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ERASE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> EraseW<IENrs> {
        EraseW::new(self, 0)
    }
    #[doc = "Bit 1 - WRITE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WriteW<IENrs> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - CHOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chof(&mut self) -> ChofW<IENrs> {
        ChofW::new(self, 2)
    }
    #[doc = "Bit 3 - CMOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmof(&mut self) -> CmofW<IENrs> {
        CmofW::new(self, 3)
    }
    #[doc = "Bit 4 - PWRUPF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwrupf(&mut self) -> PwrupfW<IENrs> {
        PwrupfW::new(self, 4)
    }
    #[doc = "Bit 5 - ICACHERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icacherr(&mut self) -> IcacherrW<IENrs> {
        IcacherrW::new(self, 5)
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
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IENrs {
    const RESET_VALUE: u32 = 0;
}
