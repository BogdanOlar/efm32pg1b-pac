#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `ERASE`"]
pub type ERASE_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRITE`"]
pub type WRITE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHOF`"]
pub type CHOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMOF`"]
pub type CMOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWRUPF`"]
pub type PWRUPF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ICACHERR`"]
pub type ICACHERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Erase Done Interrupt Read Flag"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write Done Interrupt Read Flag"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Cache Hits Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn chof(&self) -> CHOF_R {
        CHOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Cache Misses Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn cmof(&self) -> CMOF_R {
        CMOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash Power Up Sequence Complete Flag"]
    #[inline(always)]
    pub fn pwrupf(&self) -> PWRUPF_R {
        PWRUPF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ICache RAM Parity Error Flag"]
    #[inline(always)]
    pub fn icacherr(&self) -> ICACHERR_R {
        ICACHERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
