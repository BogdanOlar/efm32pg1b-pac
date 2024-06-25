#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFCrs>;
#[doc = "Field `ERASE` writer - Clear ERASE Interrupt Flag"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` writer - Clear WRITE Interrupt Flag"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOF` writer - Clear CHOF Interrupt Flag"]
pub type ChofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMOF` writer - Clear CMOF Interrupt Flag"]
pub type CmofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPF` writer - Clear PWRUPF Interrupt Flag"]
pub type PwrupfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHERR` writer - Clear ICACHERR Interrupt Flag"]
pub type IcacherrW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear ERASE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> EraseW<IFCrs> {
        EraseW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear WRITE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WriteW<IFCrs> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear CHOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn chof(&mut self) -> ChofW<IFCrs> {
        ChofW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear CMOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmof(&mut self) -> CmofW<IFCrs> {
        CmofW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear PWRUPF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pwrupf(&mut self) -> PwrupfW<IFCrs> {
        PwrupfW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear ICACHERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icacherr(&mut self) -> IcacherrW<IFCrs> {
        IcacherrW::new(self, 5)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCrs;
impl crate::RegisterSpec for IFCrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFCrs {
    const RESET_VALUE: u32 = 0;
}
