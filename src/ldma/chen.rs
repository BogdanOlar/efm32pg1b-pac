///Register `CHEN` reader
pub type R = crate::R<CHENrs>;
///Register `CHEN` writer
pub type W = crate::W<CHENrs>;
///Field `CHEN` reader - Channel Enables
pub type ChenR = crate::FieldReader;
///Field `CHEN` writer - Channel Enables
pub type ChenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Channel Enables
    #[inline(always)]
    pub fn chen(&self) -> ChenR {
        ChenR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHEN").field("chen", &self.chen()).finish()
    }
}
impl W {
    ///Bits 0:7 - Channel Enables
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> ChenW<CHENrs> {
        ChenW::new(self, 0)
    }
}
///DMA Channel Enable Register (Single-Cycle RMW)
///
///You can [`read`](crate::Reg::read) this register and get [`chen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CHENrs;
impl crate::RegisterSpec for CHENrs {
    type Ux = u32;
}
///`read()` method returns [`chen::R`](R) reader structure
impl crate::Readable for CHENrs {}
///`write(|w| ..)` method takes [`chen::W`](W) writer structure
impl crate::Writable for CHENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CHEN to value 0
impl crate::Resettable for CHENrs {
    const RESET_VALUE: u32 = 0;
}
