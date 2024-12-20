///Register `IF` reader
pub type R = crate::R<IFrs>;
///Field `EXT` reader - External Pin Interrupt Flag
pub type ExtR = crate::FieldReader<u16>;
///Field `EM4WU` reader - EM4 Wake Up Pin Interrupt Flag
pub type Em4wuR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - External Pin Interrupt Flag
    #[inline(always)]
    pub fn ext(&self) -> ExtR {
        ExtR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - EM4 Wake Up Pin Interrupt Flag
    #[inline(always)]
    pub fn em4wu(&self) -> Em4wuR {
        Em4wuR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("ext", &self.ext())
            .field("em4wu", &self.em4wu())
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
