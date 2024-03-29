#[doc = "Register `IF` reader"]
pub type R = crate::R<IFrs>;
#[doc = "Field `EXT` reader - External Pin Interrupt Flag"]
pub type EXT_R = crate::FieldReader<u16>;
#[doc = "Field `EM4WU` reader - EM4 Wake Up Pin Interrupt Flag"]
pub type EM4WU_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - External Pin Interrupt Flag"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - EM4 Wake Up Pin Interrupt Flag"]
    #[inline(always)]
    pub fn em4wu(&self) -> EM4WU_R {
        EM4WU_R::new(((self.bits >> 16) & 0xffff) as u16)
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
