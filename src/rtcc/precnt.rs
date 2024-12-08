///Register `PRECNT` reader
pub type R = crate::R<PRECNTrs>;
///Register `PRECNT` writer
pub type W = crate::W<PRECNTrs>;
///Field `PRECNT` reader - Pre-Counter Value
pub type PrecntR = crate::FieldReader<u16>;
///Field `PRECNT` writer - Pre-Counter Value
pub type PrecntW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:14 - Pre-Counter Value
    #[inline(always)]
    pub fn precnt(&self) -> PrecntR {
        PrecntR::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRECNT")
            .field("precnt", &self.precnt())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - Pre-Counter Value
    #[inline(always)]
    #[must_use]
    pub fn precnt(&mut self) -> PrecntW<PRECNTrs> {
        PrecntW::new(self, 0)
    }
}
///Pre-Counter Value Register
///
///You can [`read`](crate::Reg::read) this register and get [`precnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`precnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PRECNTrs;
impl crate::RegisterSpec for PRECNTrs {
    type Ux = u32;
}
///`read()` method returns [`precnt::R`](R) reader structure
impl crate::Readable for PRECNTrs {}
///`write(|w| ..)` method takes [`precnt::W`](W) writer structure
impl crate::Writable for PRECNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRECNT to value 0
impl crate::Resettable for PRECNTrs {
    const RESET_VALUE: u32 = 0;
}
