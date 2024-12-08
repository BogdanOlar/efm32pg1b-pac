///Register `LFECLKEN0` reader
pub type R = crate::R<LFECLKEN0rs>;
///Register `LFECLKEN0` writer
pub type W = crate::W<LFECLKEN0rs>;
///Field `RTCC` reader - Real-Time Counter and Calendar Clock Enable
pub type RtccR = crate::BitReader;
///Field `RTCC` writer - Real-Time Counter and Calendar Clock Enable
pub type RtccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Real-Time Counter and Calendar Clock Enable
    #[inline(always)]
    pub fn rtcc(&self) -> RtccR {
        RtccR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFECLKEN0")
            .field("rtcc", &self.rtcc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Real-Time Counter and Calendar Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn rtcc(&mut self) -> RtccW<LFECLKEN0rs> {
        RtccW::new(self, 0)
    }
}
///Low Frequency E Clock Enable Register 0 (Async Reg)
///
///You can [`read`](crate::Reg::read) this register and get [`lfeclken0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfeclken0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LFECLKEN0rs;
impl crate::RegisterSpec for LFECLKEN0rs {
    type Ux = u32;
}
///`read()` method returns [`lfeclken0::R`](R) reader structure
impl crate::Readable for LFECLKEN0rs {}
///`write(|w| ..)` method takes [`lfeclken0::W`](W) writer structure
impl crate::Writable for LFECLKEN0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LFECLKEN0 to value 0
impl crate::Resettable for LFECLKEN0rs {
    const RESET_VALUE: u32 = 0;
}
