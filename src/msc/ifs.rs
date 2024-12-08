///Register `IFS` writer
pub type W = crate::W<IFSrs>;
///Field `ERASE` writer - Set ERASE Interrupt Flag
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRITE` writer - Set WRITE Interrupt Flag
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHOF` writer - Set CHOF Interrupt Flag
pub type ChofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMOF` writer - Set CMOF Interrupt Flag
pub type CmofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRUPF` writer - Set PWRUPF Interrupt Flag
pub type PwrupfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHERR` writer - Set ICACHERR Interrupt Flag
pub type IcacherrW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFSrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set ERASE Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> EraseW<IFSrs> {
        EraseW::new(self, 0)
    }
    ///Bit 1 - Set WRITE Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WriteW<IFSrs> {
        WriteW::new(self, 1)
    }
    ///Bit 2 - Set CHOF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn chof(&mut self) -> ChofW<IFSrs> {
        ChofW::new(self, 2)
    }
    ///Bit 3 - Set CMOF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn cmof(&mut self) -> CmofW<IFSrs> {
        CmofW::new(self, 3)
    }
    ///Bit 4 - Set PWRUPF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn pwrupf(&mut self) -> PwrupfW<IFSrs> {
        PwrupfW::new(self, 4)
    }
    ///Bit 5 - Set ICACHERR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn icacherr(&mut self) -> IcacherrW<IFSrs> {
        IcacherrW::new(self, 5)
    }
}
///Interrupt Flag Set Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFSrs;
impl crate::RegisterSpec for IFSrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ifs::W`](W) writer structure
impl crate::Writable for IFSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IFS to value 0
impl crate::Resettable for IFSrs {
    const RESET_VALUE: u32 = 0;
}
