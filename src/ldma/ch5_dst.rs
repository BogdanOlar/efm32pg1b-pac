#[doc = "Register `CH5_DST` reader"]
pub type R = crate::R<CH5_DSTrs>;
#[doc = "Register `CH5_DST` writer"]
pub type W = crate::W<CH5_DSTrs>;
#[doc = "Field `DSTADDR` reader - Destination Data Address"]
pub type DstaddrR = crate::FieldReader<u32>;
#[doc = "Field `DSTADDR` writer - Destination Data Address"]
pub type DstaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination Data Address"]
    #[inline(always)]
    pub fn dstaddr(&self) -> DstaddrR {
        DstaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Data Address"]
    #[inline(always)]
    #[must_use]
    pub fn dstaddr(&mut self) -> DstaddrW<CH5_DSTrs> {
        DstaddrW::new(self, 0)
    }
}
#[doc = "Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_dst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_dst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH5_DSTrs;
impl crate::RegisterSpec for CH5_DSTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5_dst::R`](R) reader structure"]
impl crate::Readable for CH5_DSTrs {}
#[doc = "`write(|w| ..)` method takes [`ch5_dst::W`](W) writer structure"]
impl crate::Writable for CH5_DSTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH5_DST to value 0"]
impl crate::Resettable for CH5_DSTrs {
    const RESET_VALUE: u32 = 0;
}
