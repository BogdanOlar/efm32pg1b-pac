#[doc = "Register `DTFAULT` reader"]
pub type R = crate::R<DTFAULT_SPEC>;
#[doc = "Field `DTPRS0F` reader - DTI PRS 0 Fault"]
pub type DTPRS0F_R = crate::BitReader;
#[doc = "Field `DTPRS1F` reader - DTI PRS 1 Fault"]
pub type DTPRS1F_R = crate::BitReader;
#[doc = "Field `DTDBGF` reader - DTI Debugger Fault"]
pub type DTDBGF_R = crate::BitReader;
#[doc = "Field `DTLOCKUPF` reader - DTI Lockup Fault"]
pub type DTLOCKUPF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DTI PRS 0 Fault"]
    #[inline(always)]
    pub fn dtprs0f(&self) -> DTPRS0F_R {
        DTPRS0F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTI PRS 1 Fault"]
    #[inline(always)]
    pub fn dtprs1f(&self) -> DTPRS1F_R {
        DTPRS1F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DTI Debugger Fault"]
    #[inline(always)]
    pub fn dtdbgf(&self) -> DTDBGF_R {
        DTDBGF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTI Lockup Fault"]
    #[inline(always)]
    pub fn dtlockupf(&self) -> DTLOCKUPF_R {
        DTLOCKUPF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTFAULT")
            .field("dtprs0f", &format_args!("{}", self.dtprs0f().bit()))
            .field("dtprs1f", &format_args!("{}", self.dtprs1f().bit()))
            .field("dtdbgf", &format_args!("{}", self.dtdbgf().bit()))
            .field("dtlockupf", &format_args!("{}", self.dtlockupf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DTFAULT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DTI Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtfault::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTFAULT_SPEC;
impl crate::RegisterSpec for DTFAULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtfault::R`](R) reader structure"]
impl crate::Readable for DTFAULT_SPEC {}
#[doc = "`reset()` method sets DTFAULT to value 0"]
impl crate::Resettable for DTFAULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
