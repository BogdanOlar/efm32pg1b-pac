///Register `IFS` writer
pub type W = crate::W<IFSrs>;
///Field `TXC` writer - Set TXC Interrupt Flag
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFULL` writer - Set RXFULL Interrupt Flag
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOF` writer - Set RXOF Interrupt Flag
pub type RxofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXUF` writer - Set RXUF Interrupt Flag
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXOF` writer - Set TXOF Interrupt Flag
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUF` writer - Set TXUF Interrupt Flag
pub type TxufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERR` writer - Set PERR Interrupt Flag
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FERR` writer - Set FERR Interrupt Flag
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPAF` writer - Set MPAF Interrupt Flag
pub type MpafW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSM` writer - Set SSM Interrupt Flag
pub type SsmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCF` writer - Set CCF Interrupt Flag
pub type CcfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXIDLE` writer - Set TXIDLE Interrupt Flag
pub type TxidleW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCMP0` writer - Set TCMP0 Interrupt Flag
pub type Tcmp0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCMP1` writer - Set TCMP1 Interrupt Flag
pub type Tcmp1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCMP2` writer - Set TCMP2 Interrupt Flag
pub type Tcmp2W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFSrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set TXC Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TxcW<IFSrs> {
        TxcW::new(self, 0)
    }
    ///Bit 3 - Set RXFULL Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RxfullW<IFSrs> {
        RxfullW::new(self, 3)
    }
    ///Bit 4 - Set RXOF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn rxof(&mut self) -> RxofW<IFSrs> {
        RxofW::new(self, 4)
    }
    ///Bit 5 - Set RXUF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RxufW<IFSrs> {
        RxufW::new(self, 5)
    }
    ///Bit 6 - Set TXOF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TxofW<IFSrs> {
        TxofW::new(self, 6)
    }
    ///Bit 7 - Set TXUF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn txuf(&mut self) -> TxufW<IFSrs> {
        TxufW::new(self, 7)
    }
    ///Bit 8 - Set PERR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PerrW<IFSrs> {
        PerrW::new(self, 8)
    }
    ///Bit 9 - Set FERR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FerrW<IFSrs> {
        FerrW::new(self, 9)
    }
    ///Bit 10 - Set MPAF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn mpaf(&mut self) -> MpafW<IFSrs> {
        MpafW::new(self, 10)
    }
    ///Bit 11 - Set SSM Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SsmW<IFSrs> {
        SsmW::new(self, 11)
    }
    ///Bit 12 - Set CCF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn ccf(&mut self) -> CcfW<IFSrs> {
        CcfW::new(self, 12)
    }
    ///Bit 13 - Set TXIDLE Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn txidle(&mut self) -> TxidleW<IFSrs> {
        TxidleW::new(self, 13)
    }
    ///Bit 14 - Set TCMP0 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn tcmp0(&mut self) -> Tcmp0W<IFSrs> {
        Tcmp0W::new(self, 14)
    }
    ///Bit 15 - Set TCMP1 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn tcmp1(&mut self) -> Tcmp1W<IFSrs> {
        Tcmp1W::new(self, 15)
    }
    ///Bit 16 - Set TCMP2 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn tcmp2(&mut self) -> Tcmp2W<IFSrs> {
        Tcmp2W::new(self, 16)
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
