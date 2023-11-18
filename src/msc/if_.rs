#[doc = "Register `IF` reader"]
pub type R = crate::R<IF_SPEC>;
#[doc = "Field `ERASE` reader - Erase Done Interrupt Read Flag"]
pub type ERASE_R = crate::BitReader;
#[doc = "Field `WRITE` reader - Write Done Interrupt Read Flag"]
pub type WRITE_R = crate::BitReader;
#[doc = "Field `CHOF` reader - Cache Hits Overflow Interrupt Flag"]
pub type CHOF_R = crate::BitReader;
#[doc = "Field `CMOF` reader - Cache Misses Overflow Interrupt Flag"]
pub type CMOF_R = crate::BitReader;
#[doc = "Field `PWRUPF` reader - Flash Power Up Sequence Complete Flag"]
pub type PWRUPF_R = crate::BitReader;
#[doc = "Field `ICACHERR` reader - ICache RAM Parity Error Flag"]
pub type ICACHERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Erase Done Interrupt Read Flag"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Done Interrupt Read Flag"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cache Hits Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn chof(&self) -> CHOF_R {
        CHOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cache Misses Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn cmof(&self) -> CMOF_R {
        CMOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash Power Up Sequence Complete Flag"]
    #[inline(always)]
    pub fn pwrupf(&self) -> PWRUPF_R {
        PWRUPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ICache RAM Parity Error Flag"]
    #[inline(always)]
    pub fn icacherr(&self) -> ICACHERR_R {
        ICACHERR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("erase", &format_args!("{}", self.erase().bit()))
            .field("write", &format_args!("{}", self.write().bit()))
            .field("chof", &format_args!("{}", self.chof().bit()))
            .field("cmof", &format_args!("{}", self.cmof().bit()))
            .field("pwrupf", &format_args!("{}", self.pwrupf().bit()))
            .field("icacherr", &format_args!("{}", self.icacherr().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<IF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IF_SPEC {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
