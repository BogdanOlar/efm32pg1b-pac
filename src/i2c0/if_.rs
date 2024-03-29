#[doc = "Register `IF` reader"]
pub type R = crate::R<IFrs>;
#[doc = "Field `START` reader - START Condition Interrupt Flag"]
pub type START_R = crate::BitReader;
#[doc = "Field `RSTART` reader - Repeated START Condition Interrupt Flag"]
pub type RSTART_R = crate::BitReader;
#[doc = "Field `ADDR` reader - Address Interrupt Flag"]
pub type ADDR_R = crate::BitReader;
#[doc = "Field `TXC` reader - Transfer Completed Interrupt Flag"]
pub type TXC_R = crate::BitReader;
#[doc = "Field `TXBL` reader - Transmit Buffer Level Interrupt Flag"]
pub type TXBL_R = crate::BitReader;
#[doc = "Field `RXDATAV` reader - Receive Data Valid Interrupt Flag"]
pub type RXDATAV_R = crate::BitReader;
#[doc = "Field `ACK` reader - Acknowledge Received Interrupt Flag"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `NACK` reader - Not Acknowledge Received Interrupt Flag"]
pub type NACK_R = crate::BitReader;
#[doc = "Field `MSTOP` reader - Master STOP Condition Interrupt Flag"]
pub type MSTOP_R = crate::BitReader;
#[doc = "Field `ARBLOST` reader - Arbitration Lost Interrupt Flag"]
pub type ARBLOST_R = crate::BitReader;
#[doc = "Field `BUSERR` reader - Bus Error Interrupt Flag"]
pub type BUSERR_R = crate::BitReader;
#[doc = "Field `BUSHOLD` reader - Bus Held Interrupt Flag"]
pub type BUSHOLD_R = crate::BitReader;
#[doc = "Field `TXOF` reader - Transmit Buffer Overflow Interrupt Flag"]
pub type TXOF_R = crate::BitReader;
#[doc = "Field `RXUF` reader - Receive Buffer Underflow Interrupt Flag"]
pub type RXUF_R = crate::BitReader;
#[doc = "Field `BITO` reader - Bus Idle Timeout Interrupt Flag"]
pub type BITO_R = crate::BitReader;
#[doc = "Field `CLTO` reader - Clock Low Timeout Interrupt Flag"]
pub type CLTO_R = crate::BitReader;
#[doc = "Field `SSTOP` reader - Slave STOP Condition Interrupt Flag"]
pub type SSTOP_R = crate::BitReader;
#[doc = "Field `RXFULL` reader - Receive Buffer Full Interrupt Flag"]
pub type RXFULL_R = crate::BitReader;
#[doc = "Field `CLERR` reader - Clock Low Error Interrupt Flag"]
pub type CLERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - START Condition Interrupt Flag"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repeated START Condition Interrupt Flag"]
    #[inline(always)]
    pub fn rstart(&self) -> RSTART_R {
        RSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Address Interrupt Flag"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer Completed Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Not Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master STOP Condition Interrupt Flag"]
    #[inline(always)]
    pub fn mstop(&self) -> MSTOP_R {
        MSTOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Flag"]
    #[inline(always)]
    pub fn arblost(&self) -> ARBLOST_R {
        ARBLOST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus Held Interrupt Flag"]
    #[inline(always)]
    pub fn bushold(&self) -> BUSHOLD_R {
        BUSHOLD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive Buffer Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bus Idle Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn bito(&self) -> BITO_R {
        BITO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock Low Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn clto(&self) -> CLTO_R {
        CLTO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave STOP Condition Interrupt Flag"]
    #[inline(always)]
    pub fn sstop(&self) -> SSTOP_R {
        SSTOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive Buffer Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clock Low Error Interrupt Flag"]
    #[inline(always)]
    pub fn clerr(&self) -> CLERR_R {
        CLERR_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IFrs {}
#[doc = "`reset()` method sets IF to value 0x10"]
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0x10;
}
