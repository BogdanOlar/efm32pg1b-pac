///Register `STATUS` reader
pub type R = crate::R<STATUSrs>;
///Field `RXENS` reader - Receiver Enable Status
pub type RxensR = crate::BitReader;
///Field `TXENS` reader - Transmitter Enable Status
pub type TxensR = crate::BitReader;
///Field `RXBLOCK` reader - Block Incoming Data
pub type RxblockR = crate::BitReader;
///Field `TXC` reader - TX Complete
pub type TxcR = crate::BitReader;
///Field `TXBL` reader - TX Buffer Level
pub type TxblR = crate::BitReader;
///Field `RXDATAV` reader - RX Data Valid
pub type RxdatavR = crate::BitReader;
///Field `TXIDLE` reader - TX Idle
pub type TxidleR = crate::BitReader;
impl R {
    ///Bit 0 - Receiver Enable Status
    #[inline(always)]
    pub fn rxens(&self) -> RxensR {
        RxensR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmitter Enable Status
    #[inline(always)]
    pub fn txens(&self) -> TxensR {
        TxensR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Block Incoming Data
    #[inline(always)]
    pub fn rxblock(&self) -> RxblockR {
        RxblockR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TX Complete
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TX Buffer Level
    #[inline(always)]
    pub fn txbl(&self) -> TxblR {
        TxblR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RX Data Valid
    #[inline(always)]
    pub fn rxdatav(&self) -> RxdatavR {
        RxdatavR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TX Idle
    #[inline(always)]
    pub fn txidle(&self) -> TxidleR {
        TxidleR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("rxens", &self.rxens())
            .field("txens", &self.txens())
            .field("rxblock", &self.rxblock())
            .field("txc", &self.txc())
            .field("txbl", &self.txbl())
            .field("rxdatav", &self.rxdatav())
            .field("txidle", &self.txidle())
            .finish()
    }
}
///Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct STATUSrs;
impl crate::RegisterSpec for STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUSrs {}
///`reset()` method sets STATUS to value 0x50
impl crate::Resettable for STATUSrs {
    const RESET_VALUE: u32 = 0x50;
}
