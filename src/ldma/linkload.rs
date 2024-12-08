///Register `LINKLOAD` writer
pub type W = crate::W<LINKLOADrs>;
///Field `LINKLOAD` writer - DMA Link Loads
pub type LinkloadW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<LINKLOADrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - DMA Link Loads
    #[inline(always)]
    #[must_use]
    pub fn linkload(&mut self) -> LinkloadW<LINKLOADrs> {
        LinkloadW::new(self, 0)
    }
}
///DMA Channel Link Load Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linkload::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LINKLOADrs;
impl crate::RegisterSpec for LINKLOADrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`linkload::W`](W) writer structure
impl crate::Writable for LINKLOADrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LINKLOAD to value 0
impl crate::Resettable for LINKLOADrs {
    const RESET_VALUE: u32 = 0;
}
