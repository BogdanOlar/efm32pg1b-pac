///Register `IFC` writer
pub type W = crate::W<IFCrs>;
///Field `SINGLEOF` writer - Clear SINGLEOF Interrupt Flag
pub type SingleofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCANOF` writer - Clear SCANOF Interrupt Flag
pub type ScanofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SINGLEUF` writer - Clear SINGLEUF Interrupt Flag
pub type SingleufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCANUF` writer - Clear SCANUF Interrupt Flag
pub type ScanufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SINGLECMP` writer - Clear SINGLECMP Interrupt Flag
pub type SinglecmpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCANCMP` writer - Clear SCANCMP Interrupt Flag
pub type ScancmpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFOV` writer - Clear VREFOV Interrupt Flag
pub type VrefovW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PROGERR` writer - Clear PROGERR Interrupt Flag
pub type ProgerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 8 - Clear SINGLEOF Interrupt Flag
    #[inline(always)]
    pub fn singleof(&mut self) -> SingleofW<'_, IFCrs> {
        SingleofW::new(self, 8)
    }
    ///Bit 9 - Clear SCANOF Interrupt Flag
    #[inline(always)]
    pub fn scanof(&mut self) -> ScanofW<'_, IFCrs> {
        ScanofW::new(self, 9)
    }
    ///Bit 10 - Clear SINGLEUF Interrupt Flag
    #[inline(always)]
    pub fn singleuf(&mut self) -> SingleufW<'_, IFCrs> {
        SingleufW::new(self, 10)
    }
    ///Bit 11 - Clear SCANUF Interrupt Flag
    #[inline(always)]
    pub fn scanuf(&mut self) -> ScanufW<'_, IFCrs> {
        ScanufW::new(self, 11)
    }
    ///Bit 16 - Clear SINGLECMP Interrupt Flag
    #[inline(always)]
    pub fn singlecmp(&mut self) -> SinglecmpW<'_, IFCrs> {
        SinglecmpW::new(self, 16)
    }
    ///Bit 17 - Clear SCANCMP Interrupt Flag
    #[inline(always)]
    pub fn scancmp(&mut self) -> ScancmpW<'_, IFCrs> {
        ScancmpW::new(self, 17)
    }
    ///Bit 24 - Clear VREFOV Interrupt Flag
    #[inline(always)]
    pub fn vrefov(&mut self) -> VrefovW<'_, IFCrs> {
        VrefovW::new(self, 24)
    }
    ///Bit 25 - Clear PROGERR Interrupt Flag
    #[inline(always)]
    pub fn progerr(&mut self) -> ProgerrW<'_, IFCrs> {
        ProgerrW::new(self, 25)
    }
}
///Interrupt Flag Clear Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFCrs;
impl crate::RegisterSpec for IFCrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ifc::W`](W) writer structure
impl crate::Writable for IFCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IFC to value 0
impl crate::Resettable for IFCrs {}
