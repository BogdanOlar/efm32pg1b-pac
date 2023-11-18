#[doc = "Register `RXDATAP` reader"]
pub type R = crate::R<RXDATAP_SPEC>;
#[doc = "Field `RXDATAP` reader - RX Data Peek"]
pub type RXDATAP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RX Data Peek"]
    #[inline(always)]
    pub fn rxdatap(&self) -> RXDATAP_R {
        RXDATAP_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDATAP")
            .field("rxdatap", &format_args!("{}", self.rxdatap().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXDATAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Receive Buffer Data Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdatap::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDATAP_SPEC;
impl crate::RegisterSpec for RXDATAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdatap::R`](R) reader structure"]
impl crate::Readable for RXDATAP_SPEC {}
#[doc = "`reset()` method sets RXDATAP to value 0"]
impl crate::Resettable for RXDATAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
