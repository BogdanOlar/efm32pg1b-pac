#[doc = "Register `IF` reader"]
pub type R = crate::R<IFrs>;
#[doc = "Field `COMP0` reader - Compare Match 0 Interrupt Flag"]
pub type COMP0_R = crate::BitReader;
#[doc = "Field `COMP1` reader - Compare Match 1 Interrupt Flag"]
pub type COMP1_R = crate::BitReader;
#[doc = "Field `UF` reader - Underflow Interrupt Flag"]
pub type UF_R = crate::BitReader;
#[doc = "Field `REP0` reader - Repeat Counter 0 Interrupt Flag"]
pub type REP0_R = crate::BitReader;
#[doc = "Field `REP1` reader - Repeat Counter 1 Interrupt Flag"]
pub type REP1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Compare Match 0 Interrupt Flag"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Match 1 Interrupt Flag"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Repeat Counter 0 Interrupt Flag"]
    #[inline(always)]
    pub fn rep0(&self) -> REP0_R {
        REP0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Repeat Counter 1 Interrupt Flag"]
    #[inline(always)]
    pub fn rep1(&self) -> REP1_R {
        REP1_R::new(((self.bits >> 4) & 1) != 0)
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
