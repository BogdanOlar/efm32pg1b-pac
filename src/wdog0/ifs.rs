///Register `IFS` writer
pub type W = crate::W<IFSrs>;
///Field `TOUT` writer - Set TOUT Interrupt Flag
pub type ToutW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WARN` writer - Set WARN Interrupt Flag
pub type WarnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WIN` writer - Set WIN Interrupt Flag
pub type WinW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEM0` writer - Set PEM0 Interrupt Flag
pub type Pem0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEM1` writer - Set PEM1 Interrupt Flag
pub type Pem1W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFSrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set TOUT Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> ToutW<IFSrs> {
        ToutW::new(self, 0)
    }
    ///Bit 1 - Set WARN Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn warn(&mut self) -> WarnW<IFSrs> {
        WarnW::new(self, 1)
    }
    ///Bit 2 - Set WIN Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WinW<IFSrs> {
        WinW::new(self, 2)
    }
    ///Bit 3 - Set PEM0 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn pem0(&mut self) -> Pem0W<IFSrs> {
        Pem0W::new(self, 3)
    }
    ///Bit 4 - Set PEM1 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn pem1(&mut self) -> Pem1W<IFSrs> {
        Pem1W::new(self, 4)
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
