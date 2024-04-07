#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFSrs>;
#[doc = "Field `ERASE` writer - Set ERASE Interrupt Flag"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` writer - Set WRITE Interrupt Flag"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOF` writer - Set CHOF Interrupt Flag"]
pub type ChofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMOF` writer - Set CMOF Interrupt Flag"]
pub type CmofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPF` writer - Set PWRUPF Interrupt Flag"]
pub type PwrupfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHERR` writer - Set ICACHERR Interrupt Flag"]
pub type IcacherrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set ERASE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> EraseW<IFSrs> {
        EraseW::new(self, 0)
    }
    #[doc = "Bit 1 - Set WRITE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WriteW<IFSrs> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - Set CHOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn chof(&mut self) -> ChofW<IFSrs> {
        ChofW::new(self, 2)
    }
    #[doc = "Bit 3 - Set CMOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmof(&mut self) -> CmofW<IFSrs> {
        CmofW::new(self, 3)
    }
    #[doc = "Bit 4 - Set PWRUPF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pwrupf(&mut self) -> PwrupfW<IFSrs> {
        PwrupfW::new(self, 4)
    }
    #[doc = "Bit 5 - Set ICACHERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icacherr(&mut self) -> IcacherrW<IFSrs> {
        IcacherrW::new(self, 5)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFSrs;
impl crate::RegisterSpec for IFSrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFSrs {
    const RESET_VALUE: u32 = 0;
}
