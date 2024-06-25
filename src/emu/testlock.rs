#[doc = "Register `TESTLOCK` reader"]
pub type R = crate::R<TESTLOCKrs>;
#[doc = "Register `TESTLOCK` writer"]
pub type W = crate::W<TESTLOCKrs>;
#[doc = "Configuration Lock Key\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum LOCKKEY {
    #[doc = "0: `0`"]
    Unlocked = 0,
    #[doc = "1: `1`"]
    Locked = 1,
}
impl From<LOCKKEY> for u16 {
    #[inline(always)]
    fn from(variant: LOCKKEY) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LOCKKEY {
    type Ux = u16;
}
impl crate::IsEnum for LOCKKEY {}
#[doc = "Field `LOCKKEY` reader - Configuration Lock Key"]
pub type LockkeyR = crate::FieldReader<LOCKKEY>;
impl LockkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LOCKKEY> {
        match self.bits {
            0 => Some(LOCKKEY::Unlocked),
            1 => Some(LOCKKEY::Locked),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKKEY::Unlocked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKKEY::Locked
    }
}
#[doc = "Field `LOCKKEY` writer - Configuration Lock Key"]
pub type LockkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, LOCKKEY>;
impl<'a, REG> LockkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKKEY::Unlocked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKKEY::Locked)
    }
}
impl R {
    #[doc = "Bits 0:15 - Configuration Lock Key"]
    #[inline(always)]
    pub fn lockkey(&self) -> LockkeyR {
        LockkeyR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TESTLOCK")
            .field("lockkey", &self.lockkey())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Configuration Lock Key"]
    #[inline(always)]
    #[must_use]
    pub fn lockkey(&mut self) -> LockkeyW<TESTLOCKrs> {
        LockkeyW::new(self, 0)
    }
}
#[doc = "Test Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`testlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TESTLOCKrs;
impl crate::RegisterSpec for TESTLOCKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`testlock::R`](R) reader structure"]
impl crate::Readable for TESTLOCKrs {}
#[doc = "`write(|w| ..)` method takes [`testlock::W`](W) writer structure"]
impl crate::Writable for TESTLOCKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TESTLOCK to value 0"]
impl crate::Resettable for TESTLOCKrs {
    const RESET_VALUE: u32 = 0;
}
