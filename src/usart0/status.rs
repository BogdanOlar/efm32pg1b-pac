#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUSrs>;
#[doc = "Field `RXENS` reader - Receiver Enable Status"]
pub type RxensR = crate::BitReader;
#[doc = "Field `TXENS` reader - Transmitter Enable Status"]
pub type TxensR = crate::BitReader;
#[doc = "Field `MASTER` reader - SPI Master Mode"]
pub type MasterR = crate::BitReader;
#[doc = "Field `RXBLOCK` reader - Block Incoming Data"]
pub type RxblockR = crate::BitReader;
#[doc = "Field `TXTRI` reader - Transmitter Tristated"]
pub type TxtriR = crate::BitReader;
#[doc = "Field `TXC` reader - TX Complete"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXBL` reader - TX Buffer Level"]
pub type TxblR = crate::BitReader;
#[doc = "Field `RXDATAV` reader - RX Data Valid"]
pub type RxdatavR = crate::BitReader;
#[doc = "Field `RXFULL` reader - RX FIFO Full"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `TXBDRIGHT` reader - TX Buffer Expects Double Right Data"]
pub type TxbdrightR = crate::BitReader;
#[doc = "Field `TXBSRIGHT` reader - TX Buffer Expects Single Right Data"]
pub type TxbsrightR = crate::BitReader;
#[doc = "Field `RXDATAVRIGHT` reader - RX Data Right"]
pub type RxdatavrightR = crate::BitReader;
#[doc = "Field `RXFULLRIGHT` reader - RX Full of Right Data"]
pub type RxfullrightR = crate::BitReader;
#[doc = "Field `TXIDLE` reader - TX Idle"]
pub type TxidleR = crate::BitReader;
#[doc = "Field `TIMERRESTARTED` reader - The USART Timer Restarted Itself"]
pub type TimerrestartedR = crate::BitReader;
#[doc = "Field `TXBUFCNT` reader - TX Buffer Count"]
pub type TxbufcntR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Receiver Enable Status"]
    #[inline(always)]
    pub fn rxens(&self) -> RxensR {
        RxensR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Enable Status"]
    #[inline(always)]
    pub fn txens(&self) -> TxensR {
        TxensR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI Master Mode"]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Block Incoming Data"]
    #[inline(always)]
    pub fn rxblock(&self) -> RxblockR {
        RxblockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmitter Tristated"]
    #[inline(always)]
    pub fn txtri(&self) -> TxtriR {
        TxtriR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Buffer Level"]
    #[inline(always)]
    pub fn txbl(&self) -> TxblR {
        TxblR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Data Valid"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RxdatavR {
        RxdatavR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX FIFO Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TX Buffer Expects Double Right Data"]
    #[inline(always)]
    pub fn txbdright(&self) -> TxbdrightR {
        TxbdrightR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TX Buffer Expects Single Right Data"]
    #[inline(always)]
    pub fn txbsright(&self) -> TxbsrightR {
        TxbsrightR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RX Data Right"]
    #[inline(always)]
    pub fn rxdatavright(&self) -> RxdatavrightR {
        RxdatavrightR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RX Full of Right Data"]
    #[inline(always)]
    pub fn rxfullright(&self) -> RxfullrightR {
        RxfullrightR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX Idle"]
    #[inline(always)]
    pub fn txidle(&self) -> TxidleR {
        TxidleR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The USART Timer Restarted Itself"]
    #[inline(always)]
    pub fn timerrestarted(&self) -> TimerrestartedR {
        TimerrestartedR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - TX Buffer Count"]
    #[inline(always)]
    pub fn txbufcnt(&self) -> TxbufcntR {
        TxbufcntR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("rxens", &self.rxens())
            .field("txens", &self.txens())
            .field("master", &self.master())
            .field("rxblock", &self.rxblock())
            .field("txtri", &self.txtri())
            .field("txc", &self.txc())
            .field("txbl", &self.txbl())
            .field("rxdatav", &self.rxdatav())
            .field("rxfull", &self.rxfull())
            .field("txbdright", &self.txbdright())
            .field("txbsright", &self.txbsright())
            .field("rxdatavright", &self.rxdatavright())
            .field("rxfullright", &self.rxfullright())
            .field("txidle", &self.txidle())
            .field("timerrestarted", &self.timerrestarted())
            .field("txbufcnt", &self.txbufcnt())
            .finish()
    }
}
#[doc = "USART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUSrs;
impl crate::RegisterSpec for STATUSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUSrs {}
#[doc = "`reset()` method sets STATUS to value 0x2040"]
impl crate::Resettable for STATUSrs {
    const RESET_VALUE: u32 = 0x2040;
}
