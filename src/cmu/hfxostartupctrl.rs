#[doc = "Register `HFXOSTARTUPCTRL` reader"]
pub type R = crate::R<HFXOSTARTUPCTRLrs>;
#[doc = "Register `HFXOSTARTUPCTRL` writer"]
pub type W = crate::W<HFXOSTARTUPCTRLrs>;
#[doc = "Field `IBTRIMXOCORE` reader - Sets the Startup Oscillator Core Bias Current"]
pub type IBTRIMXOCORE_R = crate::FieldReader;
#[doc = "Field `IBTRIMXOCORE` writer - Sets the Startup Oscillator Core Bias Current"]
pub type IBTRIMXOCORE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CTUNE` reader - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_R = crate::FieldReader<u16>;
#[doc = "Field `CTUNE` writer - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RESERVED0` reader - This Field is Reserved. It Should Be Set to 0x9"]
pub type RESERVED0_R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - This Field is Reserved. It Should Be Set to 0x9"]
pub type RESERVED0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RESERVED1` reader - Sets the Regulator Output Current Level (shunt Regulator)"]
pub type RESERVED1_R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - Sets the Regulator Output Current Level (shunt Regulator)"]
pub type RESERVED1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:6 - Sets the Startup Oscillator Core Bias Current"]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORE_R {
        IBTRIMXOCORE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&self) -> CTUNE_R {
        CTUNE_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bits 21:27 - This Field is Reserved. It Should Be Set to 0x9"]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 21) & 0x7f) as u8)
    }
    #[doc = "Bits 28:31 - Sets the Regulator Output Current Level (shunt Regulator)"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Sets the Startup Oscillator Core Bias Current"]
    #[inline(always)]
    #[must_use]
    pub fn ibtrimxocore(&mut self) -> IBTRIMXOCORE_W<HFXOSTARTUPCTRLrs> {
        IBTRIMXOCORE_W::new(self, 0)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    #[must_use]
    pub fn ctune(&mut self) -> CTUNE_W<HFXOSTARTUPCTRLrs> {
        CTUNE_W::new(self, 11)
    }
    #[doc = "Bits 21:27 - This Field is Reserved. It Should Be Set to 0x9"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<HFXOSTARTUPCTRLrs> {
        RESERVED0_W::new(self, 21)
    }
    #[doc = "Bits 28:31 - Sets the Regulator Output Current Level (shunt Regulator)"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<HFXOSTARTUPCTRLrs> {
        RESERVED1_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HFXO Startup Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxostartupctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxostartupctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFXOSTARTUPCTRLrs;
impl crate::RegisterSpec for HFXOSTARTUPCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxostartupctrl::R`](R) reader structure"]
impl crate::Readable for HFXOSTARTUPCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`hfxostartupctrl::W`](W) writer structure"]
impl crate::Writable for HFXOSTARTUPCTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFXOSTARTUPCTRL to value 0xa125_0060"]
impl crate::Resettable for HFXOSTARTUPCTRLrs {
    const RESET_VALUE: u32 = 0xa125_0060;
}
