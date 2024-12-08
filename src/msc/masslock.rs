///Register `MASSLOCK` reader
pub type R = crate::R<MASSLOCKrs>;
///Register `MASSLOCK` writer
pub type W = crate::W<MASSLOCKrs>;
///Mass Erase Lock
///
///Value on reset: 1
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum LOCKKEY {
    ///0: `0`
    Unlocked = 0,
    ///1: `1`
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
///Field `LOCKKEY` reader - Mass Erase Lock
pub type LockkeyR = crate::FieldReader<LOCKKEY>;
impl LockkeyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LOCKKEY> {
        match self.bits {
            0 => Some(LOCKKEY::Unlocked),
            1 => Some(LOCKKEY::Locked),
            _ => None,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKKEY::Unlocked
    }
    ///`1`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKKEY::Locked
    }
}
///Field `LOCKKEY` writer - Mass Erase Lock
pub type LockkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, LOCKKEY>;
impl<'a, REG> LockkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///`0`
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKKEY::Unlocked)
    }
    ///`1`
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKKEY::Locked)
    }
}
impl R {
    ///Bits 0:15 - Mass Erase Lock
    #[inline(always)]
    pub fn lockkey(&self) -> LockkeyR {
        LockkeyR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASSLOCK")
            .field("lockkey", &self.lockkey())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Mass Erase Lock
    #[inline(always)]
    #[must_use]
    pub fn lockkey(&mut self) -> LockkeyW<MASSLOCKrs> {
        LockkeyW::new(self, 0)
    }
}
///Mass Erase Lock Register
///
///You can [`read`](crate::Reg::read) this register and get [`masslock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masslock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct MASSLOCKrs;
impl crate::RegisterSpec for MASSLOCKrs {
    type Ux = u32;
}
///`read()` method returns [`masslock::R`](R) reader structure
impl crate::Readable for MASSLOCKrs {}
///`write(|w| ..)` method takes [`masslock::W`](W) writer structure
impl crate::Writable for MASSLOCKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MASSLOCK to value 0x01
impl crate::Resettable for MASSLOCKrs {
    const RESET_VALUE: u32 = 0x01;
}
