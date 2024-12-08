///Register `IFC` writer
pub type W = crate::W<IFCrs>;
///Field `FPIOC` writer - Clear FPIOC Interrupt Flag
pub type FpiocW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPDZC` writer - Clear FPDZC Interrupt Flag
pub type FpdzcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPUFC` writer - Clear FPUFC Interrupt Flag
pub type FpufcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPOFC` writer - Clear FPOFC Interrupt Flag
pub type FpofcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIDC` writer - Clear FPIDC Interrupt Flag
pub type FpidcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIXC` writer - Clear FPIXC Interrupt Flag
pub type FpixcW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear FPIOC Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn fpioc(&mut self) -> FpiocW<IFCrs> {
        FpiocW::new(self, 0)
    }
    ///Bit 1 - Clear FPDZC Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn fpdzc(&mut self) -> FpdzcW<IFCrs> {
        FpdzcW::new(self, 1)
    }
    ///Bit 2 - Clear FPUFC Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn fpufc(&mut self) -> FpufcW<IFCrs> {
        FpufcW::new(self, 2)
    }
    ///Bit 3 - Clear FPOFC Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn fpofc(&mut self) -> FpofcW<IFCrs> {
        FpofcW::new(self, 3)
    }
    ///Bit 4 - Clear FPIDC Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn fpidc(&mut self) -> FpidcW<IFCrs> {
        FpidcW::new(self, 4)
    }
    ///Bit 5 - Clear FPIXC Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn fpixc(&mut self) -> FpixcW<IFCrs> {
        FpixcW::new(self, 5)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IFC to value 0
impl crate::Resettable for IFCrs {
    const RESET_VALUE: u32 = 0;
}
