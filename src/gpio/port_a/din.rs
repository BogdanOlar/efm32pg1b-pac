///Register `DIN` reader
pub type R = crate::R<DINrs>;
///Field `PINS_DIN` reader - Data in for pins 0:15
pub type PinsDinR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Data in for pins 0:15
    #[inline(always)]
    pub fn pins_din(&self) -> PinsDinR {
        PinsDinR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN")
            .field("pins_din", &self.pins_din())
            .finish()
    }
}
///Port Data in Register
///
///You can [`read`](crate::Reg::read) this register and get [`din::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DINrs;
impl crate::RegisterSpec for DINrs {
    type Ux = u32;
}
///`read()` method returns [`din::R`](R) reader structure
impl crate::Readable for DINrs {}
///`reset()` method sets DIN to value 0
impl crate::Resettable for DINrs {
    const RESET_VALUE: u32 = 0;
}
