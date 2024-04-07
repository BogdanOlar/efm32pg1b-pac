#[doc = "Register `CH7_DST` reader"]
pub type R = crate::R<CH7_DSTrs>;
#[doc = "Register `CH7_DST` writer"]
pub type W = crate::W<CH7_DSTrs>;
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
    pub fn dstaddr(&mut self) -> DstaddrW<CH7_DSTrs> {
        DstaddrW::new(self, 0)
    }
}
#[doc = "Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_dst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_dst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH7_DSTrs;
impl crate::RegisterSpec for CH7_DSTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7_dst::R`](R) reader structure"]
impl crate::Readable for CH7_DSTrs {}
#[doc = "`write(|w| ..)` method takes [`ch7_dst::W`](W) writer structure"]
impl crate::Writable for CH7_DSTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH7_DST to value 0"]
impl crate::Resettable for CH7_DSTrs {
    const RESET_VALUE: u32 = 0;
}
