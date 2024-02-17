#[doc = "Register `IF` reader"]
pub type R = crate::R<IFrs>;
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
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IFrs {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0;
}
