///Register `IF` reader
pub type R = crate::R<IFrs>;
///Field `UF` reader - Underflow Interrupt Read Flag
pub type UfR = crate::BitReader;
///Field `OF` reader - Overflow Interrupt Read Flag
pub type OfR = crate::BitReader;
///Field `DIRCNG` reader - Direction Change Detect Interrupt Flag
pub type DircngR = crate::BitReader;
///Field `AUXOF` reader - Auxiliary Overflow Interrupt Read Flag
pub type AuxofR = crate::BitReader;
///Field `TCC` reader - Triggered Compare Interrupt Read Flag
pub type TccR = crate::BitReader;
///Field `OQSTERR` reader - Oversampling Quadrature State Error Interrupt
pub type OqsterrR = crate::BitReader;
impl R {
    ///Bit 0 - Underflow Interrupt Read Flag
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Overflow Interrupt Read Flag
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Direction Change Detect Interrupt Flag
    #[inline(always)]
    pub fn dircng(&self) -> DircngR {
        DircngR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Auxiliary Overflow Interrupt Read Flag
    #[inline(always)]
    pub fn auxof(&self) -> AuxofR {
        AuxofR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Triggered Compare Interrupt Read Flag
    #[inline(always)]
    pub fn tcc(&self) -> TccR {
        TccR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Oversampling Quadrature State Error Interrupt
    #[inline(always)]
    pub fn oqsterr(&self) -> OqsterrR {
        OqsterrR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("uf", &self.uf())
            .field("of", &self.of())
            .field("dircng", &self.dircng())
            .field("auxof", &self.auxof())
            .field("tcc", &self.tcc())
            .field("oqsterr", &self.oqsterr())
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
