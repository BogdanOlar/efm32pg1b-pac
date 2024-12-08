///Register `SYNC` reader
pub type R = crate::R<SYNCrs>;
///Register `SYNC` writer
pub type W = crate::W<SYNCrs>;
///Field `SYNCTRIG` reader - Synchronization Trigger
pub type SynctrigR = crate::FieldReader;
///Field `SYNCTRIG` writer - Synchronization Trigger
pub type SynctrigW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Synchronization Trigger
    #[inline(always)]
    pub fn synctrig(&self) -> SynctrigR {
        SynctrigR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNC")
            .field("synctrig", &self.synctrig())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Synchronization Trigger
    #[inline(always)]
    #[must_use]
    pub fn synctrig(&mut self) -> SynctrigW<SYNCrs> {
        SynctrigW::new(self, 0)
    }
}
///DMA Synchronization Trigger Register (Single-Cycle RMW)
///
///You can [`read`](crate::Reg::read) this register and get [`sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SYNCrs;
impl crate::RegisterSpec for SYNCrs {
    type Ux = u32;
}
///`read()` method returns [`sync::R`](R) reader structure
impl crate::Readable for SYNCrs {}
///`write(|w| ..)` method takes [`sync::W`](W) writer structure
impl crate::Writable for SYNCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SYNC to value 0
impl crate::Resettable for SYNCrs {
    const RESET_VALUE: u32 = 0;
}
