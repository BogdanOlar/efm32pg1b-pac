///Register `IFS` writer
pub type W = crate::W<IFSrs>;
///Field `FPIOC` writer - Set FPIOC Interrupt Flag
pub type FpiocW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPDZC` writer - Set FPDZC Interrupt Flag
pub type FpdzcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPUFC` writer - Set FPUFC Interrupt Flag
pub type FpufcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPOFC` writer - Set FPOFC Interrupt Flag
pub type FpofcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIDC` writer - Set FPIDC Interrupt Flag
pub type FpidcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIXC` writer - Set FPIXC Interrupt Flag
pub type FpixcW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFSrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set FPIOC Interrupt Flag
    #[inline(always)]
    pub fn fpioc(&mut self) -> FpiocW<'_, IFSrs> {
        FpiocW::new(self, 0)
    }
    ///Bit 1 - Set FPDZC Interrupt Flag
    #[inline(always)]
    pub fn fpdzc(&mut self) -> FpdzcW<'_, IFSrs> {
        FpdzcW::new(self, 1)
    }
    ///Bit 2 - Set FPUFC Interrupt Flag
    #[inline(always)]
    pub fn fpufc(&mut self) -> FpufcW<'_, IFSrs> {
        FpufcW::new(self, 2)
    }
    ///Bit 3 - Set FPOFC Interrupt Flag
    #[inline(always)]
    pub fn fpofc(&mut self) -> FpofcW<'_, IFSrs> {
        FpofcW::new(self, 3)
    }
    ///Bit 4 - Set FPIDC Interrupt Flag
    #[inline(always)]
    pub fn fpidc(&mut self) -> FpidcW<'_, IFSrs> {
        FpidcW::new(self, 4)
    }
    ///Bit 5 - Set FPIXC Interrupt Flag
    #[inline(always)]
    pub fn fpixc(&mut self) -> FpixcW<'_, IFSrs> {
        FpixcW::new(self, 5)
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
}
///`reset()` method sets IFS to value 0
impl crate::Resettable for IFSrs {}
