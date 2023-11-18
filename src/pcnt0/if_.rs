#[doc = "Register `IF` reader"]
pub type R = crate::R<IF_SPEC>;
#[doc = "Field `UF` reader - Underflow Interrupt Read Flag"]
pub type UF_R = crate::BitReader;
#[doc = "Field `OF` reader - Overflow Interrupt Read Flag"]
pub type OF_R = crate::BitReader;
#[doc = "Field `DIRCNG` reader - Direction Change Detect Interrupt Flag"]
pub type DIRCNG_R = crate::BitReader;
#[doc = "Field `AUXOF` reader - Auxiliary Overflow Interrupt Read Flag"]
pub type AUXOF_R = crate::BitReader;
#[doc = "Field `TCC` reader - Triggered Compare Interrupt Read Flag"]
pub type TCC_R = crate::BitReader;
#[doc = "Field `OQSTERR` reader - Oversampling Quadrature State Error Interrupt"]
pub type OQSTERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Underflow Interrupt Read Flag"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt Read Flag"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dircng(&self) -> DIRCNG_R {
        DIRCNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auxiliary Overflow Interrupt Read Flag"]
    #[inline(always)]
    pub fn auxof(&self) -> AUXOF_R {
        AUXOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Triggered Compare Interrupt Read Flag"]
    #[inline(always)]
    pub fn tcc(&self) -> TCC_R {
        TCC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Oversampling Quadrature State Error Interrupt"]
    #[inline(always)]
    pub fn oqsterr(&self) -> OQSTERR_R {
        OQSTERR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("uf", &format_args!("{}", self.uf().bit()))
            .field("of", &format_args!("{}", self.of().bit()))
            .field("dircng", &format_args!("{}", self.dircng().bit()))
            .field("auxof", &format_args!("{}", self.auxof().bit()))
            .field("tcc", &format_args!("{}", self.tcc().bit()))
            .field("oqsterr", &format_args!("{}", self.oqsterr().bit()))
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
