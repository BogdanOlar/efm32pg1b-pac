#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SYNCBUSYrs>;
#[doc = "Field `CMD` reader - CMD Register Busy"]
pub type CmdR = crate::BitReader;
impl R {
    #[doc = "Bit 5 - CMD Register Busy"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Synchronization Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCBUSYrs;
impl crate::RegisterSpec for SYNCBUSYrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SYNCBUSYrs {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSYrs {
    const RESET_VALUE: u32 = 0;
}
