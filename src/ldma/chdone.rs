#[doc = "Register `CHDONE` reader"]
pub type R = crate::R<CHDONErs>;
#[doc = "Register `CHDONE` writer"]
pub type W = crate::W<CHDONErs>;
#[doc = "Field `CHDONE` reader - DMA Channel Linking or Done"]
pub type ChdoneR = crate::FieldReader;
#[doc = "Field `CHDONE` writer - DMA Channel Linking or Done"]
pub type ChdoneW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DMA Channel Linking or Done"]
    #[inline(always)]
    pub fn chdone(&self) -> ChdoneR {
        ChdoneR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Channel Linking or Done"]
    #[inline(always)]
    #[must_use]
    pub fn chdone(&mut self) -> ChdoneW<CHDONErs> {
        ChdoneW::new(self, 0)
    }
}
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chdone::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdone::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHDONErs;
impl crate::RegisterSpec for CHDONErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chdone::R`](R) reader structure"]
impl crate::Readable for CHDONErs {}
#[doc = "`write(|w| ..)` method takes [`chdone::W`](W) writer structure"]
impl crate::Writable for CHDONErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHDONE to value 0"]
impl crate::Resettable for CHDONErs {
    const RESET_VALUE: u32 = 0;
}
