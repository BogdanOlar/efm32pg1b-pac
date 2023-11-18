#[doc = "Register `DATABYTEREV` reader"]
pub type R = crate::R<DATABYTEREV_SPEC>;
#[doc = "Field `DATABYTEREV` reader - Data Byte Reverse Value"]
pub type DATABYTEREV_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data Byte Reverse Value"]
    #[inline(always)]
    pub fn databyterev(&self) -> DATABYTEREV_R {
        DATABYTEREV_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATABYTEREV")
            .field(
                "databyterev",
                &format_args!("{}", self.databyterev().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DATABYTEREV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "CRC Data Byte Reverse Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`databyterev::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATABYTEREV_SPEC;
impl crate::RegisterSpec for DATABYTEREV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`databyterev::R`](R) reader structure"]
impl crate::Readable for DATABYTEREV_SPEC {}
#[doc = "`reset()` method sets DATABYTEREV to value 0"]
impl crate::Resettable for DATABYTEREV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
