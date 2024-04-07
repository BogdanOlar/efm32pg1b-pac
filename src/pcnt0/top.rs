#[doc = "Register `TOP` reader"]
pub type R = crate::R<TOPrs>;
#[doc = "Field `TOP` reader - Counter Top Value"]
pub type TopR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Top Value"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Top Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`top::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOPrs;
impl crate::RegisterSpec for TOPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`top::R`](R) reader structure"]
impl crate::Readable for TOPrs {}
#[doc = "`reset()` method sets TOP to value 0xff"]
impl crate::Resettable for TOPrs {
    const RESET_VALUE: u32 = 0xff;
}
