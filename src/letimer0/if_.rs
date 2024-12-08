///Register `IF` reader
pub type R = crate::R<IFrs>;
///Field `COMP0` reader - Compare Match 0 Interrupt Flag
pub type Comp0R = crate::BitReader;
///Field `COMP1` reader - Compare Match 1 Interrupt Flag
pub type Comp1R = crate::BitReader;
///Field `UF` reader - Underflow Interrupt Flag
pub type UfR = crate::BitReader;
///Field `REP0` reader - Repeat Counter 0 Interrupt Flag
pub type Rep0R = crate::BitReader;
///Field `REP1` reader - Repeat Counter 1 Interrupt Flag
pub type Rep1R = crate::BitReader;
impl R {
    ///Bit 0 - Compare Match 0 Interrupt Flag
    #[inline(always)]
    pub fn comp0(&self) -> Comp0R {
        Comp0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare Match 1 Interrupt Flag
    #[inline(always)]
    pub fn comp1(&self) -> Comp1R {
        Comp1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Underflow Interrupt Flag
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Repeat Counter 0 Interrupt Flag
    #[inline(always)]
    pub fn rep0(&self) -> Rep0R {
        Rep0R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Repeat Counter 1 Interrupt Flag
    #[inline(always)]
    pub fn rep1(&self) -> Rep1R {
        Rep1R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("comp0", &self.comp0())
            .field("comp1", &self.comp1())
            .field("uf", &self.uf())
            .field("rep0", &self.rep0())
            .field("rep1", &self.rep1())
            .finish()
    }
}
///Interrupt Flag Register
///
///You can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
///`read()` method returns [`if_::R`](R) reader structure
impl crate::Readable for IFrs {}
///`reset()` method sets IF to value 0
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0;
}
