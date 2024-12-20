///Register `CH3_DST` reader
pub type R = crate::R<CH3_DSTrs>;
///Register `CH3_DST` writer
pub type W = crate::W<CH3_DSTrs>;
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
        f.debug_struct("CH3_DST")
            .field("dstaddr", &self.dstaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Destination Data Address
    #[inline(always)]
    #[must_use]
    pub fn dstaddr(&mut self) -> DstaddrW<CH3_DSTrs> {
        DstaddrW::new(self, 0)
    }
}
///Channel Descriptor Destination Data Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch3_dst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_dst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CH3_DSTrs;
impl crate::RegisterSpec for CH3_DSTrs {
    type Ux = u32;
}
///`read()` method returns [`ch3_dst::R`](R) reader structure
impl crate::Readable for CH3_DSTrs {}
///`write(|w| ..)` method takes [`ch3_dst::W`](W) writer structure
impl crate::Writable for CH3_DSTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH3_DST to value 0
impl crate::Resettable for CH3_DSTrs {
    const RESET_VALUE: u32 = 0;
}
