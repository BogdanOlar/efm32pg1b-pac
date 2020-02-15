#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCKED`"]
pub type LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `INVADDR`"]
pub type INVADDR_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDATAREADY`"]
pub type WDATAREADY_R = crate::R<bool, bool>;
#[doc = "Reader of field `WORDTIMEOUT`"]
pub type WORDTIMEOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERASEABORTED`"]
pub type ERASEABORTED_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCRUNNING`"]
pub type PCRUNNING_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Erase/Write Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Access Locked"]
    #[inline(always)]
    pub fn locked(&self) -> LOCKED_R {
        LOCKED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Invalid Write Address or Erase Page"]
    #[inline(always)]
    pub fn invaddr(&self) -> INVADDR_R {
        INVADDR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WDATA Write Ready"]
    #[inline(always)]
    pub fn wdataready(&self) -> WDATAREADY_R {
        WDATAREADY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash Write Word Timeout"]
    #[inline(always)]
    pub fn wordtimeout(&self) -> WORDTIMEOUT_R {
        WORDTIMEOUT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The Current Flash Erase Operation Aborted"]
    #[inline(always)]
    pub fn eraseaborted(&self) -> ERASEABORTED_R {
        ERASEABORTED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Performance Counters Running"]
    #[inline(always)]
    pub fn pcrunning(&self) -> PCRUNNING_R {
        PCRUNNING_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
