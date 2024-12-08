///Register `SINGLEFIFOCOUNT` reader
pub type R = crate::R<SINGLEFIFOCOUNTrs>;
///Field `SINGLEDC` reader - Single Data Count
pub type SingledcR = crate::FieldReader;
impl R {
    ///Bits 0:2 - Single Data Count
    #[inline(always)]
    pub fn singledc(&self) -> SingledcR {
        SingledcR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SINGLEFIFOCOUNT")
            .field("singledc", &self.singledc())
            .finish()
    }
}
///Single FIFO Count Register
///
///You can [`read`](crate::Reg::read) this register and get [`singlefifocount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SINGLEFIFOCOUNTrs;
impl crate::RegisterSpec for SINGLEFIFOCOUNTrs {
    type Ux = u32;
}
///`read()` method returns [`singlefifocount::R`](R) reader structure
impl crate::Readable for SINGLEFIFOCOUNTrs {}
///`reset()` method sets SINGLEFIFOCOUNT to value 0
impl crate::Resettable for SINGLEFIFOCOUNTrs {
    const RESET_VALUE: u32 = 0;
}
