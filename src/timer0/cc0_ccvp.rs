///Register `CC0_CCVP` reader
pub type R = crate::R<CC0_CCVPrs>;
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
        f.debug_struct("CC0_CCVP")
            .field("ccvp", &self.ccvp())
            .finish()
    }
}
///CC Channel Value Peek Register
///
///You can [`read`](crate::Reg::read) this register and get [`cc0_ccvp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CC0_CCVPrs;
impl crate::RegisterSpec for CC0_CCVPrs {
    type Ux = u32;
}
///`read()` method returns [`cc0_ccvp::R`](R) reader structure
impl crate::Readable for CC0_CCVPrs {}
///`reset()` method sets CC0_CCVP to value 0
impl crate::Resettable for CC0_CCVPrs {
    const RESET_VALUE: u32 = 0;
}
