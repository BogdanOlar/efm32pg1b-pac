#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRLrs>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRLrs>;
#[doc = "Field `EN` reader - Enable CRYOTIMER"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable CRYOTIMER"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DEBUGRUN_R = crate::BitReader;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DEBUGRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCSEL` reader - Select Low Frequency Oscillator"]
pub type OSCSEL_R = crate::FieldReader<OSCSEL>;
#[doc = "Select Low Frequency Oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSCSEL {
    #[doc = "0: Select Low Frequency RC Oscillator"]
    Lfrco = 0,
    #[doc = "1: Select Low Frequency Crystal Oscillator"]
    Lfxo = 1,
    #[doc = "2: Select Ultra Low Frequency RC Oscillator"]
    Ulfrco = 2,
}
impl From<OSCSEL> for u8 {
    #[inline(always)]
    fn from(variant: OSCSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSCSEL {
    type Ux = u8;
}
impl OSCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OSCSEL> {
        match self.bits {
            0 => Some(OSCSEL::Lfrco),
            1 => Some(OSCSEL::Lfxo),
            2 => Some(OSCSEL::Ulfrco),
            _ => None,
        }
    }
    #[doc = "Select Low Frequency RC Oscillator"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == OSCSEL::Lfrco
    }
    #[doc = "Select Low Frequency Crystal Oscillator"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == OSCSEL::Lfxo
    }
    #[doc = "Select Ultra Low Frequency RC Oscillator"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == OSCSEL::Ulfrco
    }
}
#[doc = "Field `OSCSEL` writer - Select Low Frequency Oscillator"]
pub type OSCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSCSEL>;
impl<'a, REG> OSCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select Low Frequency RC Oscillator"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSEL::Lfrco)
    }
    #[doc = "Select Low Frequency Crystal Oscillator"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSEL::Lfxo)
    }
    #[doc = "Select Ultra Low Frequency RC Oscillator"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSEL::Ulfrco)
    }
}
#[doc = "Field `PRESC` reader - Prescaler Setting"]
pub type PRESC_R = crate::FieldReader<PRESC>;
#[doc = "Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC {
    #[doc = "0: LF Oscillator frequency undivided"]
    Div1 = 0,
    #[doc = "1: LF Oscillator frequency divided by 2"]
    Div2 = 1,
    #[doc = "2: LF Oscillator frequency divided by 4"]
    Div4 = 2,
    #[doc = "3: LF Oscillator frequency divided by 8"]
    Div8 = 3,
    #[doc = "4: LF Oscillator frequency divided by 16"]
    Div16 = 4,
    #[doc = "5: LF Oscillator frequency divided by 32"]
    Div32 = 5,
    #[doc = "6: LF Oscillator frequency divided by 64"]
    Div64 = 6,
    #[doc = "7: LF Oscillator frequency divided by 128"]
    Div128 = 7,
}
impl From<PRESC> for u8 {
    #[inline(always)]
    fn from(variant: PRESC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC {
    type Ux = u8;
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRESC {
        match self.bits {
            0 => PRESC::Div1,
            1 => PRESC::Div2,
            2 => PRESC::Div4,
            3 => PRESC::Div8,
            4 => PRESC::Div16,
            5 => PRESC::Div32,
            6 => PRESC::Div64,
            7 => PRESC::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "LF Oscillator frequency undivided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC::Div1
    }
    #[doc = "LF Oscillator frequency divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC::Div2
    }
    #[doc = "LF Oscillator frequency divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC::Div4
    }
    #[doc = "LF Oscillator frequency divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC::Div8
    }
    #[doc = "LF Oscillator frequency divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC::Div16
    }
    #[doc = "LF Oscillator frequency divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC::Div32
    }
    #[doc = "LF Oscillator frequency divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC::Div64
    }
    #[doc = "LF Oscillator frequency divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC::Div128
    }
}
#[doc = "Field `PRESC` writer - Prescaler Setting"]
pub type PRESC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PRESC>;
impl<'a, REG> PRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LF Oscillator frequency undivided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div1)
    }
    #[doc = "LF Oscillator frequency divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div2)
    }
    #[doc = "LF Oscillator frequency divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div4)
    }
    #[doc = "LF Oscillator frequency divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div8)
    }
    #[doc = "LF Oscillator frequency divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div16)
    }
    #[doc = "LF Oscillator frequency divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div32)
    }
    #[doc = "LF Oscillator frequency divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div64)
    }
    #[doc = "LF Oscillator frequency divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div128)
    }
}
impl R {
    #[doc = "Bit 0 - Enable CRYOTIMER"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Select Low Frequency Oscillator"]
    #[inline(always)]
    pub fn oscsel(&self) -> OSCSEL_R {
        OSCSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable CRYOTIMER"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRLrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debugrun(&mut self) -> DEBUGRUN_W<CTRLrs> {
        DEBUGRUN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Select Low Frequency Oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn oscsel(&mut self) -> OSCSEL_W<CTRLrs> {
        OSCSEL_W::new(self, 2)
    }
    #[doc = "Bits 5:7 - Prescaler Setting"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<CTRLrs> {
        PRESC_W::new(self, 5)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
