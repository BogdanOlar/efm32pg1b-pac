///Register `CC2_CCVP` reader
pub type R = crate::R<CC2_CCVPrs>;
///Field `CCVP` reader - CC Channel Value Peek
pub type CcvpR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - CC Channel Value Peek
    #[inline(always)]
    pub fn ccvp(&self) -> CcvpR {
        CcvpR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CC2_CCVP")
            .field("ccvp", &self.ccvp())
            .finish()
    }
}
///CC Channel Value Peek Register
///
///You can [`read`](crate::Reg::read) this register and get [`cc2_ccvp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CC2_CCVPrs;
impl crate::RegisterSpec for CC2_CCVPrs {
    type Ux = u32;
}
///`read()` method returns [`cc2_ccvp::R`](R) reader structure
impl crate::Readable for CC2_CCVPrs {}
///`reset()` method sets CC2_CCVP to value 0
impl crate::Resettable for CC2_CCVPrs {
    const RESET_VALUE: u32 = 0;
}
