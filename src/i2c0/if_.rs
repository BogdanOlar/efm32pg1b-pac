///Register `IF` reader
pub type R = crate::R<IFrs>;
///Field `START` reader - START Condition Interrupt Flag
pub type StartR = crate::BitReader;
///Field `RSTART` reader - Repeated START Condition Interrupt Flag
pub type RstartR = crate::BitReader;
///Field `ADDR` reader - Address Interrupt Flag
pub type AddrR = crate::BitReader;
///Field `TXC` reader - Transfer Completed Interrupt Flag
pub type TxcR = crate::BitReader;
///Field `TXBL` reader - Transmit Buffer Level Interrupt Flag
pub type TxblR = crate::BitReader;
///Field `RXDATAV` reader - Receive Data Valid Interrupt Flag
pub type RxdatavR = crate::BitReader;
///Field `ACK` reader - Acknowledge Received Interrupt Flag
pub type AckR = crate::BitReader;
///Field `NACK` reader - Not Acknowledge Received Interrupt Flag
pub type NackR = crate::BitReader;
///Field `MSTOP` reader - Master STOP Condition Interrupt Flag
pub type MstopR = crate::BitReader;
///Field `ARBLOST` reader - Arbitration Lost Interrupt Flag
pub type ArblostR = crate::BitReader;
///Field `BUSERR` reader - Bus Error Interrupt Flag
pub type BuserrR = crate::BitReader;
///Field `BUSHOLD` reader - Bus Held Interrupt Flag
pub type BusholdR = crate::BitReader;
///Field `TXOF` reader - Transmit Buffer Overflow Interrupt Flag
pub type TxofR = crate::BitReader;
///Field `RXUF` reader - Receive Buffer Underflow Interrupt Flag
pub type RxufR = crate::BitReader;
///Field `BITO` reader - Bus Idle Timeout Interrupt Flag
pub type BitoR = crate::BitReader;
///Field `CLTO` reader - Clock Low Timeout Interrupt Flag
pub type CltoR = crate::BitReader;
///Field `SSTOP` reader - Slave STOP Condition Interrupt Flag
pub type SstopR = crate::BitReader;
///Field `RXFULL` reader - Receive Buffer Full Interrupt Flag
pub type RxfullR = crate::BitReader;
///Field `CLERR` reader - Clock Low Error Interrupt Flag
pub type ClerrR = crate::BitReader;
impl R {
    ///Bit 0 - START Condition Interrupt Flag
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Repeated START Condition Interrupt Flag
    #[inline(always)]
    pub fn rstart(&self) -> RstartR {
        RstartR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Address Interrupt Flag
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transfer Completed Interrupt Flag
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transmit Buffer Level Interrupt Flag
    #[inline(always)]
    pub fn txbl(&self) -> TxblR {
        TxblR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Receive Data Valid Interrupt Flag
    #[inline(always)]
    pub fn rxdatav(&self) -> RxdatavR {
        RxdatavR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Acknowledge Received Interrupt Flag
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Not Acknowledge Received Interrupt Flag
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Master STOP Condition Interrupt Flag
    #[inline(always)]
    pub fn mstop(&self) -> MstopR {
        MstopR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Arbitration Lost Interrupt Flag
    #[inline(always)]
    pub fn arblost(&self) -> ArblostR {
        ArblostR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Bus Error Interrupt Flag
    #[inline(always)]
    pub fn buserr(&self) -> BuserrR {
        BuserrR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Bus Held Interrupt Flag
    #[inline(always)]
    pub fn bushold(&self) -> BusholdR {
        BusholdR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Transmit Buffer Overflow Interrupt Flag
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Receive Buffer Underflow Interrupt Flag
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Bus Idle Timeout Interrupt Flag
    #[inline(always)]
    pub fn bito(&self) -> BitoR {
        BitoR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Clock Low Timeout Interrupt Flag
    #[inline(always)]
    pub fn clto(&self) -> CltoR {
        CltoR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Slave STOP Condition Interrupt Flag
    #[inline(always)]
    pub fn sstop(&self) -> SstopR {
        SstopR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Receive Buffer Full Interrupt Flag
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Clock Low Error Interrupt Flag
    #[inline(always)]
    pub fn clerr(&self) -> ClerrR {
        ClerrR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
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
///Interrupt Flag Register
///
///You can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
///`read()` method returns [`if_::R`](R) reader structure
impl crate::Readable for IFrs {}
///`reset()` method sets IF to value 0x10
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0x10;
}
