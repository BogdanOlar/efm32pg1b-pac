#[doc = "Register `EXTIFALL` reader"]
pub type R = crate::R<EXTIFALLrs>;
#[doc = "Register `EXTIFALL` writer"]
pub type W = crate::W<EXTIFALLrs>;
#[doc = "Field `EXTIFALL` reader - External Interrupt N Falling Edge Trigger Enable"]
pub type ExtifallR = crate::FieldReader<u16>;
#[doc = "Field `EXTIFALL` writer - External Interrupt N Falling Edge Trigger Enable"]
pub type ExtifallW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - External Interrupt N Falling Edge Trigger Enable"]
    #[inline(always)]
    pub fn extifall(&self) -> ExtifallR {
        ExtifallR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - External Interrupt N Falling Edge Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extifall(&mut self) -> ExtifallW<EXTIFALLrs> {
        ExtifallW::new(self, 0)
    }
}
#[doc = "External Interrupt Falling Edge Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extifall::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extifall::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIFALLrs;
impl crate::RegisterSpec for EXTIFALLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extifall::R`](R) reader structure"]
impl crate::Readable for EXTIFALLrs {}
#[doc = "`write(|w| ..)` method takes [`extifall::W`](W) writer structure"]
impl crate::Writable for EXTIFALLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTIFALL to value 0"]
impl crate::Resettable for EXTIFALLrs {
    const RESET_VALUE: u32 = 0;
}
