///Register `ROUTEPEN` reader
pub type R = crate::R<ROUTEPENrs>;
///Register `ROUTEPEN` writer
pub type W = crate::W<ROUTEPENrs>;
///Field `OUT0PEN` reader - Output 0 Pin Enable
pub type Out0penR = crate::BitReader;
///Field `OUT0PEN` writer - Output 0 Pin Enable
pub type Out0penW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT1PEN` reader - Output 1 Pin Enable
pub type Out1penR = crate::BitReader;
///Field `OUT1PEN` writer - Output 1 Pin Enable
pub type Out1penW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Output 0 Pin Enable
    #[inline(always)]
    pub fn out0pen(&self) -> Out0penR {
        Out0penR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Output 1 Pin Enable
    #[inline(always)]
    pub fn out1pen(&self) -> Out1penR {
        Out1penR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTEPEN")
            .field("out0pen", &self.out0pen())
            .field("out1pen", &self.out1pen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Output 0 Pin Enable
    #[inline(always)]
    #[must_use]
    pub fn out0pen(&mut self) -> Out0penW<ROUTEPENrs> {
        Out0penW::new(self, 0)
    }
    ///Bit 1 - Output 1 Pin Enable
    #[inline(always)]
    #[must_use]
    pub fn out1pen(&mut self) -> Out1penW<ROUTEPENrs> {
        Out1penW::new(self, 1)
    }
}
///I/O Routing Pin Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`routepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ROUTEPENrs;
impl crate::RegisterSpec for ROUTEPENrs {
    type Ux = u32;
}
///`read()` method returns [`routepen::R`](R) reader structure
impl crate::Readable for ROUTEPENrs {}
///`write(|w| ..)` method takes [`routepen::W`](W) writer structure
impl crate::Writable for ROUTEPENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ROUTEPEN to value 0
impl crate::Resettable for ROUTEPENrs {
    const RESET_VALUE: u32 = 0;
}
