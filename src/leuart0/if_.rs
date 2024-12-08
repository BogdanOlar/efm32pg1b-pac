///Register `IF` reader
pub type R = crate::R<IFrs>;
///Field `TXC` reader - TX Complete Interrupt Flag
pub type TxcR = crate::BitReader;
///Field `TXBL` reader - TX Buffer Level Interrupt Flag
pub type TxblR = crate::BitReader;
///Field `RXDATAV` reader - RX Data Valid Interrupt Flag
pub type RxdatavR = crate::BitReader;
///Field `RXOF` reader - RX Overflow Interrupt Flag
pub type RxofR = crate::BitReader;
///Field `RXUF` reader - RX Underflow Interrupt Flag
pub type RxufR = crate::BitReader;
///Field `TXOF` reader - TX Overflow Interrupt Flag
pub type TxofR = crate::BitReader;
///Field `PERR` reader - Parity Error Interrupt Flag
pub type PerrR = crate::BitReader;
///Field `FERR` reader - Framing Error Interrupt Flag
pub type FerrR = crate::BitReader;
///Field `MPAF` reader - Multi-Processor Address Frame Interrupt Flag
pub type MpafR = crate::BitReader;
///Field `STARTF` reader - Start Frame Interrupt Flag
pub type StartfR = crate::BitReader;
///Field `SIGF` reader - Signal Frame Interrupt Flag
pub type SigfR = crate::BitReader;
impl R {
    ///Bit 0 - TX Complete Interrupt Flag
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX Buffer Level Interrupt Flag
    #[inline(always)]
    pub fn txbl(&self) -> TxblR {
        TxblR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RX Data Valid Interrupt Flag
    #[inline(always)]
    pub fn rxdatav(&self) -> RxdatavR {
        RxdatavR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RX Overflow Interrupt Flag
    #[inline(always)]
    pub fn rxof(&self) -> RxofR {
        RxofR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RX Underflow Interrupt Flag
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TX Overflow Interrupt Flag
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Parity Error Interrupt Flag
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Framing Error Interrupt Flag
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Multi-Processor Address Frame Interrupt Flag
    #[inline(always)]
    pub fn mpaf(&self) -> MpafR {
        MpafR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Start Frame Interrupt Flag
    #[inline(always)]
    pub fn startf(&self) -> StartfR {
        StartfR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Signal Frame Interrupt Flag
    #[inline(always)]
    pub fn sigf(&self) -> SigfR {
        SigfR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
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
///Interrupt Flag Register
///
///You can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
///`read()` method returns [`if_::R`](R) reader structure
impl crate::Readable for IFrs {}
///`reset()` method sets IF to value 0x02
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0x02;
}
