#[doc = "Register `OVT_DIS` reader"]
pub type R = crate::R<OVT_DISrs>;
#[doc = "Register `OVT_DIS` writer"]
pub type W = crate::W<OVT_DISrs>;
#[doc = "Field `PINS_OVT_DIS` reader - Disable Over Voltage Capability for pins 0:15"]
pub type PinsOvtDisR = crate::FieldReader<u16>;
#[doc = "Field `PINS_OVT_DIS` writer - Disable Over Voltage Capability for pins 0:15"]
pub type PinsOvtDisW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Disable Over Voltage Capability for pins 0:15"]
    #[inline(always)]
    pub fn pins_ovt_dis(&self) -> PinsOvtDisR {
        PinsOvtDisR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Disable Over Voltage Capability for pins 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn pins_ovt_dis(&mut self) -> PinsOvtDisW<OVT_DISrs> {
        PinsOvtDisW::new(self, 0)
    }
}
#[doc = "Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ovt_dis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ovt_dis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OVT_DISrs;
impl crate::RegisterSpec for OVT_DISrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ovt_dis::R`](R) reader structure"]
impl crate::Readable for OVT_DISrs {}
#[doc = "`write(|w| ..)` method takes [`ovt_dis::W`](W) writer structure"]
impl crate::Writable for OVT_DISrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OVT_DIS to value 0"]
impl crate::Resettable for OVT_DISrs {
    const RESET_VALUE: u32 = 0;
}
