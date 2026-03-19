///Register `IFS` writer
pub type W = crate::W<IFSrs>;
///Field `TXC` writer - Set TXC Interrupt Flag
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOF` writer - Set RXOF Interrupt Flag
pub type RxofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXUF` writer - Set RXUF Interrupt Flag
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXOF` writer - Set TXOF Interrupt Flag
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERR` writer - Set PERR Interrupt Flag
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FERR` writer - Set FERR Interrupt Flag
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPAF` writer - Set MPAF Interrupt Flag
pub type MpafW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STARTF` writer - Set STARTF Interrupt Flag
pub type StartfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SIGF` writer - Set SIGF Interrupt Flag
pub type SigfW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFSrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set TXC Interrupt Flag
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<'_, IFSrs> {
        TxcW::new(self, 0)
    }
    ///Bit 3 - Set RXOF Interrupt Flag
    #[inline(always)]
    pub fn rxof(&mut self) -> RxofW<'_, IFSrs> {
        RxofW::new(self, 3)
    }
    ///Bit 4 - Set RXUF Interrupt Flag
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<'_, IFSrs> {
        RxufW::new(self, 4)
    }
    ///Bit 5 - Set TXOF Interrupt Flag
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<'_, IFSrs> {
        TxofW::new(self, 5)
    }
    ///Bit 6 - Set PERR Interrupt Flag
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<'_, IFSrs> {
        PerrW::new(self, 6)
    }
    ///Bit 7 - Set FERR Interrupt Flag
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<'_, IFSrs> {
        FerrW::new(self, 7)
    }
    ///Bit 8 - Set MPAF Interrupt Flag
    #[inline(always)]
    pub fn mpaf(&mut self) -> MpafW<'_, IFSrs> {
        MpafW::new(self, 8)
    }
    ///Bit 9 - Set STARTF Interrupt Flag
    #[inline(always)]
    pub fn startf(&mut self) -> StartfW<'_, IFSrs> {
        StartfW::new(self, 9)
    }
    ///Bit 10 - Set SIGF Interrupt Flag
    #[inline(always)]
    pub fn sigf(&mut self) -> SigfW<'_, IFSrs> {
        SigfW::new(self, 10)
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
