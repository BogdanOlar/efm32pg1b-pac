///Register `IF` reader
pub type R = crate::R<IFrs>;
///Field `DONE` reader - DMA Structure Operation Done Interrupt Flag
pub type DoneR = crate::FieldReader;
///Field `ERROR` reader - Transfer Error Interrupt Flag
pub type ErrorR = crate::BitReader;
impl R {
    ///Bits 0:7 - DMA Structure Operation Done Interrupt Flag
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 0xff) as u8)
    }
    ///Bit 31 - Transfer Error Interrupt Flag
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("done", &self.done())
            .field("error", &self.error())
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
