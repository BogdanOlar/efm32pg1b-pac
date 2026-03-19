///Register `IFC` writer
pub type W = crate::W<IFCrs>;
///Field `TOUT` writer - Clear TOUT Interrupt Flag
pub type ToutW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WARN` writer - Clear WARN Interrupt Flag
pub type WarnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WIN` writer - Clear WIN Interrupt Flag
pub type WinW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEM0` writer - Clear PEM0 Interrupt Flag
pub type Pem0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEM1` writer - Clear PEM1 Interrupt Flag
pub type Pem1W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear TOUT Interrupt Flag
    #[inline(always)]
    pub fn tout(&mut self) -> ToutW<'_, IFCrs> {
        ToutW::new(self, 0)
    }
    ///Bit 1 - Clear WARN Interrupt Flag
    #[inline(always)]
    pub fn warn(&mut self) -> WarnW<'_, IFCrs> {
        WarnW::new(self, 1)
    }
    ///Bit 2 - Clear WIN Interrupt Flag
    #[inline(always)]
    pub fn win(&mut self) -> WinW<'_, IFCrs> {
        WinW::new(self, 2)
    }
    ///Bit 3 - Clear PEM0 Interrupt Flag
    #[inline(always)]
    pub fn pem0(&mut self) -> Pem0W<'_, IFCrs> {
        Pem0W::new(self, 3)
    }
    ///Bit 4 - Clear PEM1 Interrupt Flag
    #[inline(always)]
    pub fn pem1(&mut self) -> Pem1W<'_, IFCrs> {
        Pem1W::new(self, 4)
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
