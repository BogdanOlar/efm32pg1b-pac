#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LOCKrs>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LOCKrs>;
#[doc = "Timer Lock Key\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TIMERLOCKKEY {
    #[doc = "0: `0`"]
    Unlocked = 0,
    #[doc = "1: `1`"]
    Locked = 1,
}
impl From<TIMERLOCKKEY> for u16 {
    #[inline(always)]
    fn from(variant: TIMERLOCKKEY) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMERLOCKKEY {
    type Ux = u16;
}
impl crate::IsEnum for TIMERLOCKKEY {}
#[doc = "Field `TIMERLOCKKEY` reader - Timer Lock Key"]
pub type TimerlockkeyR = crate::FieldReader<TIMERLOCKKEY>;
impl TimerlockkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIMERLOCKKEY> {
        match self.bits {
            0 => Some(TIMERLOCKKEY::Unlocked),
            1 => Some(TIMERLOCKKEY::Locked),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == TIMERLOCKKEY::Unlocked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == TIMERLOCKKEY::Locked
    }
}
#[doc = "Field `TIMERLOCKKEY` writer - Timer Lock Key"]
pub type TimerlockkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, TIMERLOCKKEY>;
impl<'a, REG> TimerlockkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(TIMERLOCKKEY::Unlocked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(TIMERLOCKKEY::Locked)
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer Lock Key"]
    #[inline(always)]
    pub fn timerlockkey(&self) -> TimerlockkeyR {
        TimerlockkeyR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCK")
            .field("timerlockkey", &self.timerlockkey())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer Lock Key"]
    #[inline(always)]
    #[must_use]
    pub fn timerlockkey(&mut self) -> TimerlockkeyW<LOCKrs> {
        TimerlockkeyW::new(self, 0)
    }
}
#[doc = "TIMER Configuration Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCKrs;
impl crate::RegisterSpec for LOCKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LOCKrs {}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LOCKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LOCKrs {
    const RESET_VALUE: u32 = 0;
}
