///Register `SCANMASK` reader
pub type R = crate::R<SCANMASKrs>;
///Register `SCANMASK` writer
pub type W = crate::W<SCANMASKrs>;
///Field `SCANINPUTEN` reader - Scan Sequence Input Mask
pub type ScaninputenR = crate::FieldReader<u32>;
///Field `SCANINPUTEN` writer - Scan Sequence Input Mask
pub type ScaninputenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Scan Sequence Input Mask
    #[inline(always)]
    pub fn scaninputen(&self) -> ScaninputenR {
        ScaninputenR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCANMASK")
            .field("scaninputen", &self.scaninputen())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Scan Sequence Input Mask
    #[inline(always)]
    #[must_use]
    pub fn scaninputen(&mut self) -> ScaninputenW<SCANMASKrs> {
        ScaninputenW::new(self, 0)
    }
}
///Scan Sequence Input Mask Register
///
///You can [`read`](crate::Reg::read) this register and get [`scanmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SCANMASKrs;
impl crate::RegisterSpec for SCANMASKrs {
    type Ux = u32;
}
///`read()` method returns [`scanmask::R`](R) reader structure
impl crate::Readable for SCANMASKrs {}
///`write(|w| ..)` method takes [`scanmask::W`](W) writer structure
impl crate::Writable for SCANMASKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SCANMASK to value 0
impl crate::Resettable for SCANMASKrs {
    const RESET_VALUE: u32 = 0;
}
