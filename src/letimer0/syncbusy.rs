///Register `SYNCBUSY` reader
pub type R = crate::R<SYNCBUSYrs>;
///Field `CMD` reader - CMD Register Busy
pub type CmdR = crate::BitReader;
impl R {
    ///Bit 1 - CMD Register Busy
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNCBUSY")
            .field("cmd", &self.cmd())
            .finish()
    }
}
///Synchronization Busy Register
///
///You can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SYNCBUSYrs;
impl crate::RegisterSpec for SYNCBUSYrs {
    type Ux = u32;
}
///`read()` method returns [`syncbusy::R`](R) reader structure
impl crate::Readable for SYNCBUSYrs {}
///`reset()` method sets SYNCBUSY to value 0
impl crate::Resettable for SYNCBUSYrs {
    const RESET_VALUE: u32 = 0;
}
