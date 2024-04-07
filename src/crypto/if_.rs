#[doc = "Register `IF` reader"]
pub type R = crate::R<IFrs>;
#[doc = "Field `INSTRDONE` reader - Instruction Done"]
pub type InstrdoneR = crate::BitReader;
#[doc = "Field `SEQDONE` reader - Sequence Done"]
pub type SeqdoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Instruction Done"]
    #[inline(always)]
    pub fn instrdone(&self) -> InstrdoneR {
        InstrdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sequence Done"]
    #[inline(always)]
    pub fn seqdone(&self) -> SeqdoneR {
        SeqdoneR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "AES Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
