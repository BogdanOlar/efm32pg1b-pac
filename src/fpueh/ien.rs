///Register `IEN` reader
pub type R = crate::R<IENrs>;
///Register `IEN` writer
pub type W = crate::W<IENrs>;
///Field `FPIOC` reader - FPIOC Interrupt Enable
pub type FpiocR = crate::BitReader;
///Field `FPIOC` writer - FPIOC Interrupt Enable
pub type FpiocW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPDZC` reader - FPDZC Interrupt Enable
pub type FpdzcR = crate::BitReader;
///Field `FPDZC` writer - FPDZC Interrupt Enable
pub type FpdzcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPUFC` reader - FPUFC Interrupt Enable
pub type FpufcR = crate::BitReader;
///Field `FPUFC` writer - FPUFC Interrupt Enable
pub type FpufcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPOFC` reader - FPOFC Interrupt Enable
pub type FpofcR = crate::BitReader;
///Field `FPOFC` writer - FPOFC Interrupt Enable
pub type FpofcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIDC` reader - FPIDC Interrupt Enable
pub type FpidcR = crate::BitReader;
///Field `FPIDC` writer - FPIDC Interrupt Enable
pub type FpidcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIXC` reader - FPIXC Interrupt Enable
pub type FpixcR = crate::BitReader;
///Field `FPIXC` writer - FPIXC Interrupt Enable
pub type FpixcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - FPIOC Interrupt Enable
    #[inline(always)]
    pub fn fpioc(&self) -> FpiocR {
        FpiocR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FPDZC Interrupt Enable
    #[inline(always)]
    pub fn fpdzc(&self) -> FpdzcR {
        FpdzcR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FPUFC Interrupt Enable
    #[inline(always)]
    pub fn fpufc(&self) -> FpufcR {
        FpufcR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FPOFC Interrupt Enable
    #[inline(always)]
    pub fn fpofc(&self) -> FpofcR {
        FpofcR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FPIDC Interrupt Enable
    #[inline(always)]
    pub fn fpidc(&self) -> FpidcR {
        FpidcR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - FPIXC Interrupt Enable
    #[inline(always)]
    pub fn fpixc(&self) -> FpixcR {
        FpixcR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("fpioc", &self.fpioc())
            .field("fpdzc", &self.fpdzc())
            .field("fpufc", &self.fpufc())
            .field("fpofc", &self.fpofc())
            .field("fpidc", &self.fpidc())
            .field("fpixc", &self.fpixc())
            .finish()
    }
}
impl W {
    ///Bit 0 - FPIOC Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn fpioc(&mut self) -> FpiocW<IENrs> {
        FpiocW::new(self, 0)
    }
    ///Bit 1 - FPDZC Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn fpdzc(&mut self) -> FpdzcW<IENrs> {
        FpdzcW::new(self, 1)
    }
    ///Bit 2 - FPUFC Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn fpufc(&mut self) -> FpufcW<IENrs> {
        FpufcW::new(self, 2)
    }
    ///Bit 3 - FPOFC Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn fpofc(&mut self) -> FpofcW<IENrs> {
        FpofcW::new(self, 3)
    }
    ///Bit 4 - FPIDC Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn fpidc(&mut self) -> FpidcW<IENrs> {
        FpidcW::new(self, 4)
    }
    ///Bit 5 - FPIXC Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn fpixc(&mut self) -> FpixcW<IENrs> {
        FpixcW::new(self, 5)
    }
}
///Interrupt Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IENrs;
impl crate::RegisterSpec for IENrs {
    type Ux = u32;
}
///`read()` method returns [`ien::R`](R) reader structure
impl crate::Readable for IENrs {}
///`write(|w| ..)` method takes [`ien::W`](W) writer structure
impl crate::Writable for IENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IEN to value 0
impl crate::Resettable for IENrs {
    const RESET_VALUE: u32 = 0;
}
