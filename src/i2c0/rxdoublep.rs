#[doc = "Register `RXDOUBLEP` reader"]
pub type R = crate::R<RXDOUBLEP_SPEC>;
#[doc = "Field `RXDATAP0` reader - RX Data 0 Peek"]
pub type RXDATAP0_R = crate::FieldReader;
#[doc = "Field `RXDATAP1` reader - RX Data 1 Peek"]
pub type RXDATAP1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RX Data 0 Peek"]
    #[inline(always)]
    pub fn rxdatap0(&self) -> RXDATAP0_R {
        RXDATAP0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX Data 1 Peek"]
    #[inline(always)]
    pub fn rxdatap1(&self) -> RXDATAP1_R {
        RXDATAP1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDOUBLEP")
            .field("rxdatap0", &format_args!("{}", self.rxdatap0().bits()))
            .field("rxdatap1", &format_args!("{}", self.rxdatap1().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXDOUBLEP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Receive Buffer Double Data Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdoublep::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDOUBLEP_SPEC;
impl crate::RegisterSpec for RXDOUBLEP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdoublep::R`](R) reader structure"]
impl crate::Readable for RXDOUBLEP_SPEC {}
#[doc = "`reset()` method sets RXDOUBLEP to value 0"]
impl crate::Resettable for RXDOUBLEP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
