///Register `IFC` writer
pub type W = crate::W<IFCrs>;
///Field `INSTRDONE` writer - Clear INSTRDONE Interrupt Flag
pub type InstrdoneW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEQDONE` writer - Clear SEQDONE Interrupt Flag
pub type SeqdoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear INSTRDONE Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn instrdone(&mut self) -> InstrdoneW<IFCrs> {
        InstrdoneW::new(self, 0)
    }
    ///Bit 1 - Clear SEQDONE Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn seqdone(&mut self) -> SeqdoneW<IFCrs> {
        SeqdoneW::new(self, 1)
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
