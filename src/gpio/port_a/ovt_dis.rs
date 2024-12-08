///Register `OVT_DIS` reader
pub type R = crate::R<OVT_DISrs>;
///Register `OVT_DIS` writer
pub type W = crate::W<OVT_DISrs>;
///Field `PINS_OVT_DIS` reader - Disable Over Voltage Capability for pins 0:15
pub type PinsOvtDisR = crate::FieldReader<u16>;
///Field `PINS_OVT_DIS` writer - Disable Over Voltage Capability for pins 0:15
pub type PinsOvtDisW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Disable Over Voltage Capability for pins 0:15
    #[inline(always)]
    pub fn pins_ovt_dis(&self) -> PinsOvtDisR {
        PinsOvtDisR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OVT_DIS")
            .field("pins_ovt_dis", &self.pins_ovt_dis())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Disable Over Voltage Capability for pins 0:15
    #[inline(always)]
    #[must_use]
    pub fn pins_ovt_dis(&mut self) -> PinsOvtDisW<OVT_DISrs> {
        PinsOvtDisW::new(self, 0)
    }
}
///Over Voltage Disable for All Modes
///
///You can [`read`](crate::Reg::read) this register and get [`ovt_dis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ovt_dis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct OVT_DISrs;
impl crate::RegisterSpec for OVT_DISrs {
    type Ux = u32;
}
///`read()` method returns [`ovt_dis::R`](R) reader structure
impl crate::Readable for OVT_DISrs {}
///`write(|w| ..)` method takes [`ovt_dis::W`](W) writer structure
impl crate::Writable for OVT_DISrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OVT_DIS to value 0
impl crate::Resettable for OVT_DISrs {
    const RESET_VALUE: u32 = 0;
}
