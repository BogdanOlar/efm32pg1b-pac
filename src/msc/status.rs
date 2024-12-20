///Register `STATUS` reader
pub type R = crate::R<STATUSrs>;
///Field `BUSY` reader - Erase/Write Busy
pub type BusyR = crate::BitReader;
///Field `LOCKED` reader - Access Locked
pub type LockedR = crate::BitReader;
///Field `INVADDR` reader - Invalid Write Address or Erase Page
pub type InvaddrR = crate::BitReader;
///Field `WDATAREADY` reader - WDATA Write Ready
pub type WdatareadyR = crate::BitReader;
///Field `WORDTIMEOUT` reader - Flash Write Word Timeout
pub type WordtimeoutR = crate::BitReader;
///Field `ERASEABORTED` reader - The Current Flash Erase Operation Aborted
pub type EraseabortedR = crate::BitReader;
///Field `PCRUNNING` reader - Performance Counters Running
pub type PcrunningR = crate::BitReader;
impl R {
    ///Bit 0 - Erase/Write Busy
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Access Locked
    #[inline(always)]
    pub fn locked(&self) -> LockedR {
        LockedR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Invalid Write Address or Erase Page
    #[inline(always)]
    pub fn invaddr(&self) -> InvaddrR {
        InvaddrR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WDATA Write Ready
    #[inline(always)]
    pub fn wdataready(&self) -> WdatareadyR {
        WdatareadyR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Flash Write Word Timeout
    #[inline(always)]
    pub fn wordtimeout(&self) -> WordtimeoutR {
        WordtimeoutR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - The Current Flash Erase Operation Aborted
    #[inline(always)]
    pub fn eraseaborted(&self) -> EraseabortedR {
        EraseabortedR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Performance Counters Running
    #[inline(always)]
    pub fn pcrunning(&self) -> PcrunningR {
        PcrunningR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("busy", &self.busy())
            .field("locked", &self.locked())
            .field("invaddr", &self.invaddr())
            .field("wdataready", &self.wdataready())
            .field("wordtimeout", &self.wordtimeout())
            .field("eraseaborted", &self.eraseaborted())
            .field("pcrunning", &self.pcrunning())
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
///`reset()` method sets STATUS to value 0x08
impl crate::Resettable for STATUSrs {
    const RESET_VALUE: u32 = 0x08;
}
