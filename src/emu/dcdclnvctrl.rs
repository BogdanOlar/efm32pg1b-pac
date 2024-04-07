#[doc = "Register `DCDCLNVCTRL` reader"]
pub type R = crate::R<DCDCLNVCTRLrs>;
#[doc = "Register `DCDCLNVCTRL` writer"]
pub type W = crate::W<DCDCLNVCTRLrs>;
#[doc = "Field `LNATT` reader - Low Noise Mode Feedback Attenuation"]
pub type LnattR = crate::BitReader;
#[doc = "Field `LNATT` writer - Low Noise Mode Feedback Attenuation"]
pub type LnattW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LNVREF` reader - Low Noise Mode VREF Trim"]
pub type LnvrefR = crate::FieldReader;
#[doc = "Field `LNVREF` writer - Low Noise Mode VREF Trim"]
pub type LnvrefW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 1 - Low Noise Mode Feedback Attenuation"]
    #[inline(always)]
    pub fn lnatt(&self) -> LnattR {
        LnattR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Low Noise Mode VREF Trim"]
    #[inline(always)]
    pub fn lnvref(&self) -> LnvrefR {
        LnvrefR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Low Noise Mode Feedback Attenuation"]
    #[inline(always)]
    #[must_use]
    pub fn lnatt(&mut self) -> LnattW<DCDCLNVCTRLrs> {
        LnattW::new(self, 1)
    }
    #[doc = "Bits 8:14 - Low Noise Mode VREF Trim"]
    #[inline(always)]
    #[must_use]
    pub fn lnvref(&mut self) -> LnvrefW<DCDCLNVCTRLrs> {
        LnvrefW::new(self, 8)
    }
}
#[doc = "DCDC Low Noise Voltage Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdclnvctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdclnvctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDCLNVCTRLrs;
impl crate::RegisterSpec for DCDCLNVCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdclnvctrl::R`](R) reader structure"]
impl crate::Readable for DCDCLNVCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`dcdclnvctrl::W`](W) writer structure"]
impl crate::Writable for DCDCLNVCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDCLNVCTRL to value 0x7100"]
impl crate::Resettable for DCDCLNVCTRLrs {
    const RESET_VALUE: u32 = 0x7100;
}
