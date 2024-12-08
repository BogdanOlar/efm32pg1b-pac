///Register `IEN` reader
pub type R = crate::R<IENrs>;
///Register `IEN` writer
pub type W = crate::W<IENrs>;
///Field `START` reader - START Interrupt Enable
pub type StartR = crate::BitReader;
///Field `START` writer - START Interrupt Enable
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTART` reader - RSTART Interrupt Enable
pub type RstartR = crate::BitReader;
///Field `RSTART` writer - RSTART Interrupt Enable
pub type RstartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDR` reader - ADDR Interrupt Enable
pub type AddrR = crate::BitReader;
///Field `ADDR` writer - ADDR Interrupt Enable
pub type AddrW<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `ACK` reader - ACK Interrupt Enable
pub type AckR = crate::BitReader;
///Field `ACK` writer - ACK Interrupt Enable
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACK` reader - NACK Interrupt Enable
pub type NackR = crate::BitReader;
///Field `NACK` writer - NACK Interrupt Enable
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTOP` reader - MSTOP Interrupt Enable
pub type MstopR = crate::BitReader;
///Field `MSTOP` writer - MSTOP Interrupt Enable
pub type MstopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARBLOST` reader - ARBLOST Interrupt Enable
pub type ArblostR = crate::BitReader;
///Field `ARBLOST` writer - ARBLOST Interrupt Enable
pub type ArblostW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSERR` reader - BUSERR Interrupt Enable
pub type BuserrR = crate::BitReader;
///Field `BUSERR` writer - BUSERR Interrupt Enable
pub type BuserrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSHOLD` reader - BUSHOLD Interrupt Enable
pub type BusholdR = crate::BitReader;
///Field `BUSHOLD` writer - BUSHOLD Interrupt Enable
pub type BusholdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXOF` reader - TXOF Interrupt Enable
pub type TxofR = crate::BitReader;
///Field `TXOF` writer - TXOF Interrupt Enable
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXUF` reader - RXUF Interrupt Enable
pub type RxufR = crate::BitReader;
///Field `RXUF` writer - RXUF Interrupt Enable
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BITO` reader - BITO Interrupt Enable
pub type BitoR = crate::BitReader;
///Field `BITO` writer - BITO Interrupt Enable
pub type BitoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLTO` reader - CLTO Interrupt Enable
pub type CltoR = crate::BitReader;
///Field `CLTO` writer - CLTO Interrupt Enable
pub type CltoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSTOP` reader - SSTOP Interrupt Enable
pub type SstopR = crate::BitReader;
///Field `SSTOP` writer - SSTOP Interrupt Enable
pub type SstopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFULL` reader - RXFULL Interrupt Enable
pub type RxfullR = crate::BitReader;
///Field `RXFULL` writer - RXFULL Interrupt Enable
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLERR` reader - CLERR Interrupt Enable
pub type ClerrR = crate::BitReader;
///Field `CLERR` writer - CLERR Interrupt Enable
pub type ClerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - START Interrupt Enable
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RSTART Interrupt Enable
    #[inline(always)]
    pub fn rstart(&self) -> RstartR {
        RstartR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADDR Interrupt Enable
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TXC Interrupt Enable
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TXBL Interrupt Enable
    #[inline(always)]
    pub fn txbl(&self) -> TxblR {
        TxblR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXDATAV Interrupt Enable
    #[inline(always)]
    pub fn rxdatav(&self) -> RxdatavR {
        RxdatavR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ACK Interrupt Enable
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - NACK Interrupt Enable
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - MSTOP Interrupt Enable
    #[inline(always)]
    pub fn mstop(&self) -> MstopR {
        MstopR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ARBLOST Interrupt Enable
    #[inline(always)]
    pub fn arblost(&self) -> ArblostR {
        ArblostR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - BUSERR Interrupt Enable
    #[inline(always)]
    pub fn buserr(&self) -> BuserrR {
        BuserrR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - BUSHOLD Interrupt Enable
    #[inline(always)]
    pub fn bushold(&self) -> BusholdR {
        BusholdR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TXOF Interrupt Enable
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RXUF Interrupt Enable
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - BITO Interrupt Enable
    #[inline(always)]
    pub fn bito(&self) -> BitoR {
        BitoR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CLTO Interrupt Enable
    #[inline(always)]
    pub fn clto(&self) -> CltoR {
        CltoR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SSTOP Interrupt Enable
    #[inline(always)]
    pub fn sstop(&self) -> SstopR {
        SstopR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - RXFULL Interrupt Enable
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CLERR Interrupt Enable
    #[inline(always)]
    pub fn clerr(&self) -> ClerrR {
        ClerrR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("start", &self.start())
            .field("rstart", &self.rstart())
            .field("addr", &self.addr())
            .field("txc", &self.txc())
            .field("txbl", &self.txbl())
            .field("rxdatav", &self.rxdatav())
            .field("ack", &self.ack())
            .field("nack", &self.nack())
            .field("mstop", &self.mstop())
            .field("arblost", &self.arblost())
            .field("buserr", &self.buserr())
            .field("bushold", &self.bushold())
            .field("txof", &self.txof())
            .field("rxuf", &self.rxuf())
            .field("bito", &self.bito())
            .field("clto", &self.clto())
            .field("sstop", &self.sstop())
            .field("rxfull", &self.rxfull())
            .field("clerr", &self.clerr())
            .finish()
    }
}
impl W {
    ///Bit 0 - START Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<IENrs> {
        StartW::new(self, 0)
    }
    ///Bit 1 - RSTART Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn rstart(&mut self) -> RstartW<IENrs> {
        RstartW::new(self, 1)
    }
    ///Bit 2 - ADDR Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<IENrs> {
        AddrW::new(self, 2)
    }
    ///Bit 3 - TXC Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TxcW<IENrs> {
        TxcW::new(self, 3)
    }
    ///Bit 4 - TXBL Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn txbl(&mut self) -> TxblW<IENrs> {
        TxblW::new(self, 4)
    }
    ///Bit 5 - RXDATAV Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn rxdatav(&mut self) -> RxdatavW<IENrs> {
        RxdatavW::new(self, 5)
    }
    ///Bit 6 - ACK Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<IENrs> {
        AckW::new(self, 6)
    }
    ///Bit 7 - NACK Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<IENrs> {
        NackW::new(self, 7)
    }
    ///Bit 8 - MSTOP Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn mstop(&mut self) -> MstopW<IENrs> {
        MstopW::new(self, 8)
    }
    ///Bit 9 - ARBLOST Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn arblost(&mut self) -> ArblostW<IENrs> {
        ArblostW::new(self, 9)
    }
    ///Bit 10 - BUSERR Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BuserrW<IENrs> {
        BuserrW::new(self, 10)
    }
    ///Bit 11 - BUSHOLD Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn bushold(&mut self) -> BusholdW<IENrs> {
        BusholdW::new(self, 11)
    }
    ///Bit 12 - TXOF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TxofW<IENrs> {
        TxofW::new(self, 12)
    }
    ///Bit 13 - RXUF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RxufW<IENrs> {
        RxufW::new(self, 13)
    }
    ///Bit 14 - BITO Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn bito(&mut self) -> BitoW<IENrs> {
        BitoW::new(self, 14)
    }
    ///Bit 15 - CLTO Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn clto(&mut self) -> CltoW<IENrs> {
        CltoW::new(self, 15)
    }
    ///Bit 16 - SSTOP Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn sstop(&mut self) -> SstopW<IENrs> {
        SstopW::new(self, 16)
    }
    ///Bit 17 - RXFULL Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RxfullW<IENrs> {
        RxfullW::new(self, 17)
    }
    ///Bit 18 - CLERR Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn clerr(&mut self) -> ClerrW<IENrs> {
        ClerrW::new(self, 18)
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
