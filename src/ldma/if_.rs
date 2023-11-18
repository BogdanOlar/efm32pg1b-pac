#[doc = "Register `IF` reader"]
pub type R = crate::R<IF_SPEC>;
#[doc = "Field `DONE` reader - DMA Structure Operation Done Interrupt Flag"]
pub type DONE_R = crate::FieldReader;
#[doc = "Field `ERROR` reader - Transfer Error Interrupt Flag"]
pub type ERROR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - DMA Structure Operation Done Interrupt Flag"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Transfer Error Interrupt Flag"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("done", &format_args!("{}", self.done().bits()))
            .field("error", &format_args!("{}", self.error().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<IF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IF_SPEC {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
