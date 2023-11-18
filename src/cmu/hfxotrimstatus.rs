#[doc = "Register `HFXOTRIMSTATUS` reader"]
pub type R = crate::R<HFXOTRIMSTATUS_SPEC>;
#[doc = "Field `IBTRIMXOCORE` reader - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm"]
pub type IBTRIMXOCORE_R = crate::FieldReader;
#[doc = "Field `REGISH` reader - Value of REGISH Found By Automatic HFXO Shunt Current Optimization Algorithm"]
pub type REGISH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm"]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORE_R {
        IBTRIMXOCORE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:10 - Value of REGISH Found By Automatic HFXO Shunt Current Optimization Algorithm"]
    #[inline(always)]
    pub fn regish(&self) -> REGISH_R {
        REGISH_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFXOTRIMSTATUS")
            .field(
                "ibtrimxocore",
                &format_args!("{}", self.ibtrimxocore().bits()),
            )
            .field("regish", &format_args!("{}", self.regish().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HFXOTRIMSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "HFXO Trim Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxotrimstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFXOTRIMSTATUS_SPEC;
impl crate::RegisterSpec for HFXOTRIMSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxotrimstatus::R`](R) reader structure"]
impl crate::Readable for HFXOTRIMSTATUS_SPEC {}
#[doc = "`reset()` method sets HFXOTRIMSTATUS to value 0x0500"]
impl crate::Resettable for HFXOTRIMSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0500;
}
