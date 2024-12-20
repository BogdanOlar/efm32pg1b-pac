///Register `WDATA` reader
pub type R = crate::R<WDATArs>;
///Register `WDATA` writer
pub type W = crate::W<WDATArs>;
///Field `WDATA` reader - Write Data
pub type WdataR = crate::FieldReader<u32>;
///Field `WDATA` writer - Write Data
pub type WdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Write Data
    #[inline(always)]
    pub fn wdata(&self) -> WdataR {
        WdataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDATA")
            .field("wdata", &self.wdata())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Write Data
    #[inline(always)]
    #[must_use]
    pub fn wdata(&mut self) -> WdataW<WDATArs> {
        WdataW::new(self, 0)
    }
}
///Write Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WDATArs;
impl crate::RegisterSpec for WDATArs {
    type Ux = u32;
}
///`read()` method returns [`wdata::R`](R) reader structure
impl crate::Readable for WDATArs {}
///`write(|w| ..)` method takes [`wdata::W`](W) writer structure
impl crate::Writable for WDATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WDATA to value 0
impl crate::Resettable for WDATArs {
    const RESET_VALUE: u32 = 0;
}
