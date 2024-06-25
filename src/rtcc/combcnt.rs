#[doc = "Register `COMBCNT` reader"]
pub type R = crate::R<COMBCNTrs>;
#[doc = "Field `PRECNT` reader - Pre-Counter Value"]
pub type PrecntR = crate::FieldReader<u16>;
#[doc = "Field `CNTLSB` reader - Counter Value"]
pub type CntlsbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:14 - Pre-Counter Value"]
    #[inline(always)]
    pub fn precnt(&self) -> PrecntR {
        PrecntR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:31 - Counter Value"]
    #[inline(always)]
    pub fn cntlsb(&self) -> CntlsbR {
        CntlsbR::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMBCNT")
            .field("precnt", &self.precnt())
            .field("cntlsb", &self.cntlsb())
            .finish()
    }
}
#[doc = "Combined Pre-Counter and Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`combcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMBCNTrs;
impl crate::RegisterSpec for COMBCNTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`combcnt::R`](R) reader structure"]
impl crate::Readable for COMBCNTrs {}
#[doc = "`reset()` method sets COMBCNT to value 0"]
impl crate::Resettable for COMBCNTrs {
    const RESET_VALUE: u32 = 0;
}
