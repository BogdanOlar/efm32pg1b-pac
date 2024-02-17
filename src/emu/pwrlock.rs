#[doc = "Register `PWRLOCK` reader"]
pub type R = crate::R<PWRLOCKrs>;
#[doc = "Register `PWRLOCK` writer"]
pub type W = crate::W<PWRLOCKrs>;
#[doc = "Field `LOCKKEY` reader - Regulator and Supply Configuration Lock Key"]
pub type LOCKKEY_R = crate::FieldReader<LOCKKEY>;
#[doc = "Regulator and Supply Configuration Lock Key\n\nValue on reset: 0"]
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
impl LOCKKEY_R {
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
#[doc = "Field `LOCKKEY` writer - Regulator and Supply Configuration Lock Key"]
pub type LOCKKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, LOCKKEY>;
impl<'a, REG> LOCKKEY_W<'a, REG>
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
    #[doc = "Bits 0:15 - Regulator and Supply Configuration Lock Key"]
    #[inline(always)]
    pub fn lockkey(&self) -> LOCKKEY_R {
        LOCKKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Regulator and Supply Configuration Lock Key"]
    #[inline(always)]
    #[must_use]
    pub fn lockkey(&mut self) -> LOCKKEY_W<PWRLOCKrs> {
        LOCKKEY_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Regulator and Supply Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrlock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrlock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRLOCKrs;
impl crate::RegisterSpec for PWRLOCKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrlock::R`](R) reader structure"]
impl crate::Readable for PWRLOCKrs {}
#[doc = "`write(|w| ..)` method takes [`pwrlock::W`](W) writer structure"]
impl crate::Writable for PWRLOCKrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRLOCK to value 0"]
impl crate::Resettable for PWRLOCKrs {
    const RESET_VALUE: u32 = 0;
}
