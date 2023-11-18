#[doc = "Register `CC0_CCVP` reader"]
pub type R = crate::R<CC0_CCVP_SPEC>;
#[doc = "Field `CCVP` reader - CC Channel Value Peek"]
pub type CCVP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - CC Channel Value Peek"]
    #[inline(always)]
    pub fn ccvp(&self) -> CCVP_R {
        CCVP_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CC0_CCVP")
            .field("ccvp", &format_args!("{}", self.ccvp().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CC0_CCVP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "CC Channel Value Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc0_ccvp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC0_CCVP_SPEC;
impl crate::RegisterSpec for CC0_CCVP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc0_ccvp::R`](R) reader structure"]
impl crate::Readable for CC0_CCVP_SPEC {}
#[doc = "`reset()` method sets CC0_CCVP to value 0"]
impl crate::Resettable for CC0_CCVP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
