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
///Field `STARTF` reader - STARTF Interrupt Enable
pub type StartfR = crate::BitReader;
///Field `STARTF` writer - STARTF Interrupt Enable
pub type StartfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SIGF` reader - SIGF Interrupt Enable
pub type SigfR = crate::BitReader;
///Field `SIGF` writer - SIGF Interrupt Enable
pub type SigfW<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 3 - RXOF Interrupt Enable
    #[inline(always)]
    pub fn rxof(&self) -> RxofR {
        RxofR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RXUF Interrupt Enable
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TXOF Interrupt Enable
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PERR Interrupt Enable
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - FERR Interrupt Enable
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - MPAF Interrupt Enable
    #[inline(always)]
    pub fn mpaf(&self) -> MpafR {
        MpafR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - STARTF Interrupt Enable
    #[inline(always)]
    pub fn startf(&self) -> StartfR {
        StartfR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SIGF Interrupt Enable
    #[inline(always)]
    pub fn sigf(&self) -> SigfR {
        SigfR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("txc", &self.txc())
            .field("txbl", &self.txbl())
            .field("rxdatav", &self.rxdatav())
            .field("rxof", &self.rxof())
            .field("rxuf", &self.rxuf())
            .field("txof", &self.txof())
            .field("perr", &self.perr())
            .field("ferr", &self.ferr())
            .field("mpaf", &self.mpaf())
            .field("startf", &self.startf())
            .field("sigf", &self.sigf())
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
    ///Bit 3 - RXOF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn rxof(&mut self) -> RxofW<IENrs> {
        RxofW::new(self, 3)
    }
    ///Bit 4 - RXUF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RxufW<IENrs> {
        RxufW::new(self, 4)
    }
    ///Bit 5 - TXOF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TxofW<IENrs> {
        TxofW::new(self, 5)
    }
    ///Bit 6 - PERR Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PerrW<IENrs> {
        PerrW::new(self, 6)
    }
    ///Bit 7 - FERR Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FerrW<IENrs> {
        FerrW::new(self, 7)
    }
    ///Bit 8 - MPAF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn mpaf(&mut self) -> MpafW<IENrs> {
        MpafW::new(self, 8)
    }
    ///Bit 9 - STARTF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn startf(&mut self) -> StartfW<IENrs> {
        StartfW::new(self, 9)
    }
    ///Bit 10 - SIGF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn sigf(&mut self) -> SigfW<IENrs> {
        SigfW::new(self, 10)
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
