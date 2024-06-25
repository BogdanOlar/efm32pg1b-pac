#[doc = "Register `CH6_DST` reader"]
pub type R = crate::R<CH6_DSTrs>;
#[doc = "Register `CH6_DST` writer"]
pub type W = crate::W<CH6_DSTrs>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH6_DST")
            .field("dstaddr", &self.dstaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Data Address"]
    #[inline(always)]
    #[must_use]
    pub fn dstaddr(&mut self) -> DstaddrW<CH6_DSTrs> {
        DstaddrW::new(self, 0)
    }
}
#[doc = "Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_dst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_dst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH6_DSTrs;
impl crate::RegisterSpec for CH6_DSTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6_dst::R`](R) reader structure"]
impl crate::Readable for CH6_DSTrs {}
#[doc = "`write(|w| ..)` method takes [`ch6_dst::W`](W) writer structure"]
impl crate::Writable for CH6_DSTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH6_DST to value 0"]
impl crate::Resettable for CH6_DSTrs {
    const RESET_VALUE: u32 = 0;
}
