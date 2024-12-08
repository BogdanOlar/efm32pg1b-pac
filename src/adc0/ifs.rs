///Register `IFS` writer
pub type W = crate::W<IFSrs>;
///Field `SINGLEOF` writer - Set SINGLEOF Interrupt Flag
pub type SingleofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCANOF` writer - Set SCANOF Interrupt Flag
pub type ScanofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SINGLEUF` writer - Set SINGLEUF Interrupt Flag
pub type SingleufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCANUF` writer - Set SCANUF Interrupt Flag
pub type ScanufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SINGLECMP` writer - Set SINGLECMP Interrupt Flag
pub type SinglecmpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCANCMP` writer - Set SCANCMP Interrupt Flag
pub type ScancmpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFOV` writer - Set VREFOV Interrupt Flag
pub type VrefovW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PROGERR` writer - Set PROGERR Interrupt Flag
pub type ProgerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFSrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 8 - Set SINGLEOF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn singleof(&mut self) -> SingleofW<IFSrs> {
        SingleofW::new(self, 8)
    }
    ///Bit 9 - Set SCANOF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn scanof(&mut self) -> ScanofW<IFSrs> {
        ScanofW::new(self, 9)
    }
    ///Bit 10 - Set SINGLEUF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn singleuf(&mut self) -> SingleufW<IFSrs> {
        SingleufW::new(self, 10)
    }
    ///Bit 11 - Set SCANUF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn scanuf(&mut self) -> ScanufW<IFSrs> {
        ScanufW::new(self, 11)
    }
    ///Bit 16 - Set SINGLECMP Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn singlecmp(&mut self) -> SinglecmpW<IFSrs> {
        SinglecmpW::new(self, 16)
    }
    ///Bit 17 - Set SCANCMP Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn scancmp(&mut self) -> ScancmpW<IFSrs> {
        ScancmpW::new(self, 17)
    }
    ///Bit 24 - Set VREFOV Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn vrefov(&mut self) -> VrefovW<IFSrs> {
        VrefovW::new(self, 24)
    }
    ///Bit 25 - Set PROGERR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn progerr(&mut self) -> ProgerrW<IFSrs> {
        ProgerrW::new(self, 25)
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
