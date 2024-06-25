#[doc = "Register `CACHECMD` writer"]
pub type W = crate::W<CACHECMDrs>;
#[doc = "Field `INVCACHE` writer - Invalidate Instruction Cache"]
pub type InvcacheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTPC` writer - Start Performance Counters"]
pub type StartpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPPC` writer - Stop Performance Counters"]
pub type StoppcW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CACHECMDrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Invalidate Instruction Cache"]
    #[inline(always)]
    #[must_use]
    pub fn invcache(&mut self) -> InvcacheW<CACHECMDrs> {
        InvcacheW::new(self, 0)
    }
    #[doc = "Bit 1 - Start Performance Counters"]
    #[inline(always)]
    #[must_use]
    pub fn startpc(&mut self) -> StartpcW<CACHECMDrs> {
        StartpcW::new(self, 1)
    }
    #[doc = "Bit 2 - Stop Performance Counters"]
    #[inline(always)]
    #[must_use]
    pub fn stoppc(&mut self) -> StoppcW<CACHECMDrs> {
        StoppcW::new(self, 2)
    }
}
#[doc = "Flash Cache Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cachecmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHECMDrs;
impl crate::RegisterSpec for CACHECMDrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cachecmd::W`](W) writer structure"]
impl crate::Writable for CACHECMDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHECMD to value 0"]
impl crate::Resettable for CACHECMDrs {
    const RESET_VALUE: u32 = 0;
}
