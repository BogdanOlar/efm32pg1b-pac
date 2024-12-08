///Register `AUXCNT` reader
pub type R = crate::R<AUXCNTrs>;
///Field `AUXCNT` reader - Auxiliary Counter Value
pub type AuxcntR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Auxiliary Counter Value
    #[inline(always)]
    pub fn auxcnt(&self) -> AuxcntR {
        AuxcntR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUXCNT")
            .field("auxcnt", &self.auxcnt())
            .finish()
    }
}
///Auxiliary Counter Value Register
///
///You can [`read`](crate::Reg::read) this register and get [`auxcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AUXCNTrs;
impl crate::RegisterSpec for AUXCNTrs {
    type Ux = u32;
}
///`read()` method returns [`auxcnt::R`](R) reader structure
impl crate::Readable for AUXCNTrs {}
///`reset()` method sets AUXCNT to value 0
impl crate::Resettable for AUXCNTrs {
    const RESET_VALUE: u32 = 0;
}
