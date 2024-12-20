///Register `RST` reader
pub type R = crate::R<RSTrs>;
///Register `RST` writer
pub type W = crate::W<RSTrs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
///Reset Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RSTrs;
impl crate::RegisterSpec for RSTrs {
    type Ux = u32;
}
///`read()` method returns [`rst::R`](R) reader structure
impl crate::Readable for RSTrs {}
///`write(|w| ..)` method takes [`rst::W`](W) writer structure
impl crate::Writable for RSTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RST to value 0
impl crate::Resettable for RSTrs {
    const RESET_VALUE: u32 = 0;
}
