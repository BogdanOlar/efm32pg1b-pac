///Register `IEN` reader
pub type R = crate::R<IENrs>;
///Register `IEN` writer
pub type W = crate::W<IENrs>;
///Field `TXC` reader - TXC Interrupt Enable
pub type TxcR = crate::BitReader;
///Field `TXC` writer - TXC Interrupt Enable
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXBL` reader - TXBL Interrupt Enable
pub type TxblR = crate::BitReader;
///Field `TXBL` writer - TXBL Interrupt Enable
pub type TxblW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXDATAV` reader - RXDATAV Interrupt Enable
pub type RxdatavR = crate::BitReader;
///Field `RXDATAV` writer - RXDATAV Interrupt Enable
pub type RxdatavW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFULL` reader - RXFULL Interrupt Enable
pub type RxfullR = crate::BitReader;
///Field `RXFULL` writer - RXFULL Interrupt Enable
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOF` reader - RXOF Interrupt Enable
pub type RxofR = crate::BitReader;
///Field `RXOF` writer - RXOF Interrupt Enable
pub type RxofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXUF` reader - RXUF Interrupt Enable
pub type RxufR = crate::BitReader;
///Field `RXUF` writer - RXUF Interrupt Enable
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXOF` reader - TXOF Interrupt Enable
pub type TxofR = crate::BitReader;
///Field `TXOF` writer - TXOF Interrupt Enable
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUF` reader - TXUF Interrupt Enable
pub type TxufR = crate::BitReader;
///Field `TXUF` writer - TXUF Interrupt Enable
pub type TxufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERR` reader - PERR Interrupt Enable
pub type PerrR = crate::BitReader;
///Field `PERR` writer - PERR Interrupt Enable
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FERR` reader - FERR Interrupt Enable
pub type FerrR = crate::BitReader;
///Field `FERR` writer - FERR Interrupt Enable
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPAF` reader - MPAF Interrupt Enable
pub type MpafR = crate::BitReader;
///Field `MPAF` writer - MPAF Interrupt Enable
pub type MpafW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSM` reader - SSM Interrupt Enable
pub type SsmR = crate::BitReader;
///Field `SSM` writer - SSM Interrupt Enable
pub type SsmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCF` reader - CCF Interrupt Enable
pub type CcfR = crate::BitReader;
///Field `CCF` writer - CCF Interrupt Enable
pub type CcfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXIDLE` reader - TXIDLE Interrupt Enable
pub type TxidleR = crate::BitReader;
///Field `TXIDLE` writer - TXIDLE Interrupt Enable
pub type TxidleW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCMP0` reader - TCMP0 Interrupt Enable
pub type Tcmp0R = crate::BitReader;
///Field `TCMP0` writer - TCMP0 Interrupt Enable
pub type Tcmp0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCMP1` reader - TCMP1 Interrupt Enable
pub type Tcmp1R = crate::BitReader;
///Field `TCMP1` writer - TCMP1 Interrupt Enable
pub type Tcmp1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCMP2` reader - TCMP2 Interrupt Enable
pub type Tcmp2R = crate::BitReader;
///Field `TCMP2` writer - TCMP2 Interrupt Enable
pub type Tcmp2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TXC Interrupt Enable
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXBL Interrupt Enable
    #[inline(always)]
    pub fn txbl(&self) -> TxblR {
        TxblR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RXDATAV Interrupt Enable
    #[inline(always)]
    pub fn rxdatav(&self) -> RxdatavR {
        RxdatavR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RXFULL Interrupt Enable
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RXOF Interrupt Enable
    #[inline(always)]
    pub fn rxof(&self) -> RxofR {
        RxofR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXUF Interrupt Enable
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TXOF Interrupt Enable
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXUF Interrupt Enable
    #[inline(always)]
    pub fn txuf(&self) -> TxufR {
        TxufR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PERR Interrupt Enable
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FERR Interrupt Enable
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - MPAF Interrupt Enable
    #[inline(always)]
    pub fn mpaf(&self) -> MpafR {
        MpafR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SSM Interrupt Enable
    #[inline(always)]
    pub fn ssm(&self) -> SsmR {
        SsmR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CCF Interrupt Enable
    #[inline(always)]
    pub fn ccf(&self) -> CcfR {
        CcfR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TXIDLE Interrupt Enable
    #[inline(always)]
    pub fn txidle(&self) -> TxidleR {
        TxidleR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TCMP0 Interrupt Enable
    #[inline(always)]
    pub fn tcmp0(&self) -> Tcmp0R {
        Tcmp0R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TCMP1 Interrupt Enable
    #[inline(always)]
    pub fn tcmp1(&self) -> Tcmp1R {
        Tcmp1R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TCMP2 Interrupt Enable
    #[inline(always)]
    pub fn tcmp2(&self) -> Tcmp2R {
        Tcmp2R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("txc", &self.txc())
            .field("txbl", &self.txbl())
            .field("rxdatav", &self.rxdatav())
            .field("rxfull", &self.rxfull())
            .field("rxof", &self.rxof())
            .field("rxuf", &self.rxuf())
            .field("txof", &self.txof())
            .field("txuf", &self.txuf())
            .field("perr", &self.perr())
            .field("ferr", &self.ferr())
            .field("mpaf", &self.mpaf())
            .field("ssm", &self.ssm())
            .field("ccf", &self.ccf())
            .field("txidle", &self.txidle())
            .field("tcmp0", &self.tcmp0())
            .field("tcmp1", &self.tcmp1())
            .field("tcmp2", &self.tcmp2())
            .finish()
    }
}
impl W {
    ///Bit 0 - TXC Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TxcW<IENrs> {
        TxcW::new(self, 0)
    }
    ///Bit 1 - TXBL Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn txbl(&mut self) -> TxblW<IENrs> {
        TxblW::new(self, 1)
    }
    ///Bit 2 - RXDATAV Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn rxdatav(&mut self) -> RxdatavW<IENrs> {
        RxdatavW::new(self, 2)
    }
    ///Bit 3 - RXFULL Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RxfullW<IENrs> {
        RxfullW::new(self, 3)
    }
    ///Bit 4 - RXOF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn rxof(&mut self) -> RxofW<IENrs> {
        RxofW::new(self, 4)
    }
    ///Bit 5 - RXUF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RxufW<IENrs> {
        RxufW::new(self, 5)
    }
    ///Bit 6 - TXOF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TxofW<IENrs> {
        TxofW::new(self, 6)
    }
    ///Bit 7 - TXUF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn txuf(&mut self) -> TxufW<IENrs> {
        TxufW::new(self, 7)
    }
    ///Bit 8 - PERR Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PerrW<IENrs> {
        PerrW::new(self, 8)
    }
    ///Bit 9 - FERR Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FerrW<IENrs> {
        FerrW::new(self, 9)
    }
    ///Bit 10 - MPAF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn mpaf(&mut self) -> MpafW<IENrs> {
        MpafW::new(self, 10)
    }
    ///Bit 11 - SSM Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SsmW<IENrs> {
        SsmW::new(self, 11)
    }
    ///Bit 12 - CCF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn ccf(&mut self) -> CcfW<IENrs> {
        CcfW::new(self, 12)
    }
    ///Bit 13 - TXIDLE Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn txidle(&mut self) -> TxidleW<IENrs> {
        TxidleW::new(self, 13)
    }
    ///Bit 14 - TCMP0 Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn tcmp0(&mut self) -> Tcmp0W<IENrs> {
        Tcmp0W::new(self, 14)
    }
    ///Bit 15 - TCMP1 Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn tcmp1(&mut self) -> Tcmp1W<IENrs> {
        Tcmp1W::new(self, 15)
    }
    ///Bit 16 - TCMP2 Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn tcmp2(&mut self) -> Tcmp2W<IENrs> {
        Tcmp2W::new(self, 16)
    }
}
///Interrupt Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IENrs;
impl crate::RegisterSpec for IENrs {
    type Ux = u32;
}
///`read()` method returns [`ien::R`](R) reader structure
impl crate::Readable for IENrs {}
///`write(|w| ..)` method takes [`ien::W`](W) writer structure
impl crate::Writable for IENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IEN to value 0
impl crate::Resettable for IENrs {
    const RESET_VALUE: u32 = 0;
}
