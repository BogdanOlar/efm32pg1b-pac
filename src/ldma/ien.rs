///Register `IEN` reader
pub type R = crate::R<IENrs>;
///Register `IEN` writer
pub type W = crate::W<IENrs>;
///Field `DONE` reader - DONE Interrupt Enable
pub type DoneR = crate::FieldReader;
///Field `DONE` writer - DONE Interrupt Enable
pub type DoneW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ERROR` reader - ERROR Interrupt Enable
pub type ErrorR = crate::BitReader;
///Field `ERROR` writer - ERROR Interrupt Enable
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - DONE Interrupt Enable
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 0xff) as u8)
    }
    ///Bit 31 - ERROR Interrupt Enable
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("done", &self.done())
            .field("error", &self.error())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DONE Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<IENrs> {
        DoneW::new(self, 0)
    }
    ///Bit 31 - ERROR Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IENrs> {
        ErrorW::new(self, 31)
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
