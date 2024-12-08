///Register `IF` reader
pub type R = crate::R<IFrs>;
///Field `INSTRDONE` reader - Instruction Done
pub type InstrdoneR = crate::BitReader;
///Field `SEQDONE` reader - Sequence Done
pub type SeqdoneR = crate::BitReader;
impl R {
    ///Bit 0 - Instruction Done
    #[inline(always)]
    pub fn instrdone(&self) -> InstrdoneR {
        InstrdoneR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Sequence Done
    #[inline(always)]
    pub fn seqdone(&self) -> SeqdoneR {
        SeqdoneR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("instrdone", &self.instrdone())
            .field("seqdone", &self.seqdone())
            .finish()
    }
}
///AES Interrupt Flags
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
