#[doc = "Register `LINKLOAD` writer"]
pub type W = crate::W<LINKLOADrs>;
#[doc = "Field `LINKLOAD` writer - DMA Link Loads"]
pub type LinkloadW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - DMA Link Loads"]
    #[inline(always)]
    #[must_use]
    pub fn linkload(&mut self) -> LinkloadW<LINKLOADrs> {
        LinkloadW::new(self, 0)
    }
}
#[doc = "DMA Channel Link Load Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linkload::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINKLOADrs;
impl crate::RegisterSpec for LINKLOADrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`linkload::W`](W) writer structure"]
impl crate::Writable for LINKLOADrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINKLOAD to value 0"]
impl crate::Resettable for LINKLOADrs {
    const RESET_VALUE: u32 = 0;
}
