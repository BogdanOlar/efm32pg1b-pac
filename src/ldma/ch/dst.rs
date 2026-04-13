///Register `DST` reader
pub type R = crate::R<DSTrs>;
///Register `DST` writer
pub type W = crate::W<DSTrs>;
///Field `DSTADDR` reader - Destination Data Address
pub type DstaddrR = crate::FieldReader<u32>;
///Field `DSTADDR` writer - Destination Data Address
pub type DstaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Destination Data Address
    #[inline(always)]
    pub fn dstaddr(&self) -> DstaddrR {
        DstaddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DST")
            .field("dstaddr", &self.dstaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Destination Data Address
    #[inline(always)]
    pub fn dstaddr(&mut self) -> DstaddrW<'_, DSTrs> {
        DstaddrW::new(self, 0)
    }
}
///Channel Descriptor Destination Data Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`dst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DSTrs;
impl crate::RegisterSpec for DSTrs {
    type Ux = u32;
}
///`read()` method returns [`dst::R`](R) reader structure
impl crate::Readable for DSTrs {}
///`write(|w| ..)` method takes [`dst::W`](W) writer structure
impl crate::Writable for DSTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DST to value 0
impl crate::Resettable for DSTrs {}
