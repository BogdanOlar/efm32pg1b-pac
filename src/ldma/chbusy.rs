///Register `CHBUSY` reader
pub type R = crate::R<CHBUSYrs>;
///Field `BUSY` reader - Channels Busy
pub type BusyR = crate::FieldReader;
impl R {
    ///Bits 0:7 - Channels Busy
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHBUSY")
            .field("busy", &self.busy())
            .finish()
    }
}
///DMA Channel Busy Register
///
///You can [`read`](crate::Reg::read) this register and get [`chbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CHBUSYrs;
impl crate::RegisterSpec for CHBUSYrs {
    type Ux = u32;
}
///`read()` method returns [`chbusy::R`](R) reader structure
impl crate::Readable for CHBUSYrs {}
///`reset()` method sets CHBUSY to value 0
impl crate::Resettable for CHBUSYrs {
    const RESET_VALUE: u32 = 0;
}
