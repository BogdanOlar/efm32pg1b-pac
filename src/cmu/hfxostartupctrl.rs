#[doc = "Register `HFXOSTARTUPCTRL` reader"]
pub type R = crate::R<HFXOSTARTUPCTRL_SPEC>;
#[doc = "Register `HFXOSTARTUPCTRL` writer"]
pub type W = crate::W<HFXOSTARTUPCTRL_SPEC>;
#[doc = "Field `IBTRIMXOCORE` reader - Sets the Startup Oscillator Core Bias Current"]
pub type IBTRIMXOCORE_R = crate::FieldReader;
#[doc = "Field `IBTRIMXOCORE` writer - Sets the Startup Oscillator Core Bias Current"]
pub type IBTRIMXOCORE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `CTUNE` reader - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_R = crate::FieldReader<u16>;
#[doc = "Field `CTUNE` writer - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `RESERVED0` reader - This Field is Reserved. It Should Be Set to 0x9"]
pub type RESERVED0_R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - This Field is Reserved. It Should Be Set to 0x9"]
pub type RESERVED0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `RESERVED1` reader - Sets the Regulator Output Current Level (shunt Regulator)"]
pub type RESERVED1_R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - Sets the Regulator Output Current Level (shunt Regulator)"]
pub type RESERVED1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFXOSTARTUPCTRL")
            .field(
                "ibtrimxocore",
                &format_args!("{}", self.ibtrimxocore().bits()),
            )
            .field("ctune", &format_args!("{}", self.ctune().bits()))
            .field("reserved0", &format_args!("{}", self.reserved0().bits()))
            .field("reserved1", &format_args!("{}", self.reserved1().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HFXOSTARTUPCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Sets the Startup Oscillator Core Bias Current"]
    #[inline(always)]
    #[must_use]
    pub fn ibtrimxocore(&mut self) -> IBTRIMXOCORE_W<HFXOSTARTUPCTRL_SPEC, 0> {
        IBTRIMXOCORE_W::new(self)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    #[must_use]
    pub fn ctune(&mut self) -> CTUNE_W<HFXOSTARTUPCTRL_SPEC, 11> {
        CTUNE_W::new(self)
    }
    #[doc = "Bits 21:27 - This Field is Reserved. It Should Be Set to 0x9"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<HFXOSTARTUPCTRL_SPEC, 21> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 28:31 - Sets the Regulator Output Current Level (shunt Regulator)"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<HFXOSTARTUPCTRL_SPEC, 28> {
        RESERVED1_W::new(self)
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
pub struct HFXOSTARTUPCTRL_SPEC;
impl crate::RegisterSpec for HFXOSTARTUPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxostartupctrl::R`](R) reader structure"]
impl crate::Readable for HFXOSTARTUPCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfxostartupctrl::W`](W) writer structure"]
impl crate::Writable for HFXOSTARTUPCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFXOSTARTUPCTRL to value 0xa125_0060"]
impl crate::Resettable for HFXOSTARTUPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xa125_0060;
}
