#[doc = "Register `IF` reader"]
pub type R = crate::R<IFrs>;
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
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IFrs {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0;
}
