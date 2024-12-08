///Register `IF` reader
pub type R = crate::R<IFrs>;
///Field `FPIOC` reader - FPU invalid operation
pub type FpiocR = crate::BitReader;
///Field `FPDZC` reader - FPU divide-by-zero exception
pub type FpdzcR = crate::BitReader;
///Field `FPUFC` reader - FPU underflow exception
pub type FpufcR = crate::BitReader;
///Field `FPOFC` reader - FPU overflow exception
pub type FpofcR = crate::BitReader;
///Field `FPIDC` reader - FPU input denormal exception
pub type FpidcR = crate::BitReader;
///Field `FPIXC` reader - FPU inexact exception
pub type FpixcR = crate::BitReader;
impl R {
    ///Bit 0 - FPU invalid operation
    #[inline(always)]
    pub fn fpioc(&self) -> FpiocR {
        FpiocR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FPU divide-by-zero exception
    #[inline(always)]
    pub fn fpdzc(&self) -> FpdzcR {
        FpdzcR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FPU underflow exception
    #[inline(always)]
    pub fn fpufc(&self) -> FpufcR {
        FpufcR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FPU overflow exception
    #[inline(always)]
    pub fn fpofc(&self) -> FpofcR {
        FpofcR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FPU input denormal exception
    #[inline(always)]
    pub fn fpidc(&self) -> FpidcR {
        FpidcR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - FPU inexact exception
    #[inline(always)]
    pub fn fpixc(&self) -> FpixcR {
        FpixcR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("fpioc", &self.fpioc())
            .field("fpdzc", &self.fpdzc())
            .field("fpufc", &self.fpufc())
            .field("fpofc", &self.fpofc())
            .field("fpidc", &self.fpidc())
            .field("fpixc", &self.fpixc())
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
