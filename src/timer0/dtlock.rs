///Register `DTLOCK` reader
pub type R = crate::R<DTLOCKrs>;
///Register `DTLOCK` writer
pub type W = crate::W<DTLOCKrs>;
///DTI Lock Key
///
///Value on reset: 0
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
///Field `LOCKKEY` reader - DTI Lock Key
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
///Field `LOCKKEY` writer - DTI Lock Key
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
    ///Bits 0:15 - DTI Lock Key
    #[inline(always)]
    pub fn lockkey(&self) -> LockkeyR {
        LockkeyR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTLOCK")
            .field("lockkey", &self.lockkey())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - DTI Lock Key
    #[inline(always)]
    #[must_use]
    pub fn lockkey(&mut self) -> LockkeyW<DTLOCKrs> {
        LockkeyW::new(self, 0)
    }
}
///DTI Configuration Lock Register
///
///You can [`read`](crate::Reg::read) this register and get [`dtlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DTLOCKrs;
impl crate::RegisterSpec for DTLOCKrs {
    type Ux = u32;
}
///`read()` method returns [`dtlock::R`](R) reader structure
impl crate::Readable for DTLOCKrs {}
///`write(|w| ..)` method takes [`dtlock::W`](W) writer structure
impl crate::Writable for DTLOCKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DTLOCK to value 0
impl crate::Resettable for DTLOCKrs {
    const RESET_VALUE: u32 = 0;
}
