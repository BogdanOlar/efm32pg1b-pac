#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMDrs>;
#[doc = "Field `START` writer - Start LETIMER"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` writer - Stop LETIMER"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR` writer - Clear LETIMER"]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTO0` writer - Clear Toggle Output 0"]
pub type Cto0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTO1` writer - Clear Toggle Output 1"]
pub type Cto1W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CMDrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Start LETIMER"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CMDrs> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Stop LETIMER"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<CMDrs> {
        StopW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear LETIMER"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> ClearW<CMDrs> {
        ClearW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Toggle Output 0"]
    #[inline(always)]
    #[must_use]
    pub fn cto0(&mut self) -> Cto0W<CMDrs> {
        Cto0W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Toggle Output 1"]
    #[inline(always)]
    #[must_use]
    pub fn cto1(&mut self) -> Cto1W<CMDrs> {
        Cto1W::new(self, 4)
    }
}
#[doc = "Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDrs;
impl crate::RegisterSpec for CMDrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMDrs {
    const RESET_VALUE: u32 = 0;
}
