///Register `IFC` writer
pub type W = crate::W<IFCrs>;
///Field `TXC` writer - Clear TXC Interrupt Flag
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFULL` writer - Clear RXFULL Interrupt Flag
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOF` writer - Clear RXOF Interrupt Flag
pub type RxofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXUF` writer - Clear RXUF Interrupt Flag
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXOF` writer - Clear TXOF Interrupt Flag
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUF` writer - Clear TXUF Interrupt Flag
pub type TxufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERR` writer - Clear PERR Interrupt Flag
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FERR` writer - Clear FERR Interrupt Flag
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPAF` writer - Clear MPAF Interrupt Flag
pub type MpafW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSM` writer - Clear SSM Interrupt Flag
pub type SsmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCF` writer - Clear CCF Interrupt Flag
pub type CcfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXIDLE` writer - Clear TXIDLE Interrupt Flag
pub type TxidleW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCMP0` writer - Clear TCMP0 Interrupt Flag
pub type Tcmp0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCMP1` writer - Clear TCMP1 Interrupt Flag
pub type Tcmp1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCMP2` writer - Clear TCMP2 Interrupt Flag
pub type Tcmp2W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear TXC Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TxcW<IFCrs> {
        TxcW::new(self, 0)
    }
    ///Bit 3 - Clear RXFULL Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RxfullW<IFCrs> {
        RxfullW::new(self, 3)
    }
    ///Bit 4 - Clear RXOF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn rxof(&mut self) -> RxofW<IFCrs> {
        RxofW::new(self, 4)
    }
    ///Bit 5 - Clear RXUF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RxufW<IFCrs> {
        RxufW::new(self, 5)
    }
    ///Bit 6 - Clear TXOF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TxofW<IFCrs> {
        TxofW::new(self, 6)
    }
    ///Bit 7 - Clear TXUF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn txuf(&mut self) -> TxufW<IFCrs> {
        TxufW::new(self, 7)
    }
    ///Bit 8 - Clear PERR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PerrW<IFCrs> {
        PerrW::new(self, 8)
    }
    ///Bit 9 - Clear FERR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FerrW<IFCrs> {
        FerrW::new(self, 9)
    }
    ///Bit 10 - Clear MPAF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn mpaf(&mut self) -> MpafW<IFCrs> {
        MpafW::new(self, 10)
    }
    ///Bit 11 - Clear SSM Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SsmW<IFCrs> {
        SsmW::new(self, 11)
    }
    ///Bit 12 - Clear CCF Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn ccf(&mut self) -> CcfW<IFCrs> {
        CcfW::new(self, 12)
    }
    ///Bit 13 - Clear TXIDLE Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn txidle(&mut self) -> TxidleW<IFCrs> {
        TxidleW::new(self, 13)
    }
    ///Bit 14 - Clear TCMP0 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn tcmp0(&mut self) -> Tcmp0W<IFCrs> {
        Tcmp0W::new(self, 14)
    }
    ///Bit 15 - Clear TCMP1 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn tcmp1(&mut self) -> Tcmp1W<IFCrs> {
        Tcmp1W::new(self, 15)
    }
    ///Bit 16 - Clear TCMP2 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn tcmp2(&mut self) -> Tcmp2W<IFCrs> {
        Tcmp2W::new(self, 16)
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
