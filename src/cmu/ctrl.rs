#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRLrs>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRLrs>;
#[doc = "Field `CLKOUTSEL0` reader - Clock Output Select 0"]
pub type CLKOUTSEL0_R = crate::FieldReader<CLKOUTSEL0>;
#[doc = "Clock Output Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUTSEL0 {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: ULFRCO (directly from oscillator)"]
    Ulfrco = 1,
    #[doc = "2: LFRCO (directly from oscillator)"]
    Lfrco = 2,
    #[doc = "3: LFXO (directly from oscillator)"]
    Lfxo = 3,
    #[doc = "6: HFXO (directly from oscillator)"]
    Hfxo = 6,
    #[doc = "7: HFEXPCLK"]
    Hfexpclk = 7,
    #[doc = "9: ULFRCO (qualified)"]
    Ulfrcoq = 9,
    #[doc = "10: LFRCO (qualified)"]
    Lfrcoq = 10,
    #[doc = "11: LFXO (qualified)"]
    Lfxoq = 11,
    #[doc = "12: HFRCO (qualified)"]
    Hfrcoq = 12,
    #[doc = "13: AUXHFRCO (qualified)"]
    Auxhfrcoq = 13,
    #[doc = "14: HFXO (qualified)"]
    Hfxoq = 14,
    #[doc = "15: HFSRCCLK"]
    Hfsrcclk = 15,
}
impl From<CLKOUTSEL0> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKOUTSEL0 {
    type Ux = u8;
}
impl CLKOUTSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKOUTSEL0> {
        match self.bits {
            0 => Some(CLKOUTSEL0::Disabled),
            1 => Some(CLKOUTSEL0::Ulfrco),
            2 => Some(CLKOUTSEL0::Lfrco),
            3 => Some(CLKOUTSEL0::Lfxo),
            6 => Some(CLKOUTSEL0::Hfxo),
            7 => Some(CLKOUTSEL0::Hfexpclk),
            9 => Some(CLKOUTSEL0::Ulfrcoq),
            10 => Some(CLKOUTSEL0::Lfrcoq),
            11 => Some(CLKOUTSEL0::Lfxoq),
            12 => Some(CLKOUTSEL0::Hfrcoq),
            13 => Some(CLKOUTSEL0::Auxhfrcoq),
            14 => Some(CLKOUTSEL0::Hfxoq),
            15 => Some(CLKOUTSEL0::Hfsrcclk),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOUTSEL0::Disabled
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL0::Ulfrco
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL0::Lfrco
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL0::Lfxo
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL0::Hfxo
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == CLKOUTSEL0::Hfexpclk
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn is_ulfrcoq(&self) -> bool {
        *self == CLKOUTSEL0::Ulfrcoq
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn is_lfrcoq(&self) -> bool {
        *self == CLKOUTSEL0::Lfrcoq
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn is_lfxoq(&self) -> bool {
        *self == CLKOUTSEL0::Lfxoq
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn is_hfrcoq(&self) -> bool {
        *self == CLKOUTSEL0::Hfrcoq
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == CLKOUTSEL0::Auxhfrcoq
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn is_hfxoq(&self) -> bool {
        *self == CLKOUTSEL0::Hfxoq
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == CLKOUTSEL0::Hfsrcclk
    }
}
#[doc = "Field `CLKOUTSEL0` writer - Clock Output Select 0"]
pub type CLKOUTSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CLKOUTSEL0>;
impl<'a, REG> CLKOUTSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0::Disabled)
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0::Ulfrco)
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0::Lfrco)
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0::Lfxo)
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0::Hfxo)
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0::Hfexpclk)
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn ulfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0::Ulfrcoq)
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn lfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0::Lfrcoq)
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn lfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0::Lfxoq)
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn hfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0::Hfrcoq)
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn auxhfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0::Auxhfrcoq)
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn hfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0::Hfxoq)
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0::Hfsrcclk)
    }
}
#[doc = "Field `CLKOUTSEL1` reader - Clock Output Select 1"]
pub type CLKOUTSEL1_R = crate::FieldReader<CLKOUTSEL1>;
#[doc = "Clock Output Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUTSEL1 {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: ULFRCO (directly from oscillator)"]
    Ulfrco = 1,
    #[doc = "2: LFRCO (directly from oscillator)"]
    Lfrco = 2,
    #[doc = "3: LFXO (directly from oscillator)"]
    Lfxo = 3,
    #[doc = "6: HFXO (directly from oscillator)"]
    Hfxo = 6,
    #[doc = "7: HFEXPCLK"]
    Hfexpclk = 7,
    #[doc = "9: ULFRCO (qualified)"]
    Ulfrcoq = 9,
    #[doc = "10: LFRCO (qualified)"]
    Lfrcoq = 10,
    #[doc = "11: LFXO (qualified)"]
    Lfxoq = 11,
    #[doc = "12: HFRCO (qualified)"]
    Hfrcoq = 12,
    #[doc = "13: AUXHFRCO (qualified)"]
    Auxhfrcoq = 13,
    #[doc = "14: HFXO (qualified)"]
    Hfxoq = 14,
    #[doc = "15: HFSRCCLK"]
    Hfsrcclk = 15,
}
impl From<CLKOUTSEL1> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKOUTSEL1 {
    type Ux = u8;
}
impl CLKOUTSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKOUTSEL1> {
        match self.bits {
            0 => Some(CLKOUTSEL1::Disabled),
            1 => Some(CLKOUTSEL1::Ulfrco),
            2 => Some(CLKOUTSEL1::Lfrco),
            3 => Some(CLKOUTSEL1::Lfxo),
            6 => Some(CLKOUTSEL1::Hfxo),
            7 => Some(CLKOUTSEL1::Hfexpclk),
            9 => Some(CLKOUTSEL1::Ulfrcoq),
            10 => Some(CLKOUTSEL1::Lfrcoq),
            11 => Some(CLKOUTSEL1::Lfxoq),
            12 => Some(CLKOUTSEL1::Hfrcoq),
            13 => Some(CLKOUTSEL1::Auxhfrcoq),
            14 => Some(CLKOUTSEL1::Hfxoq),
            15 => Some(CLKOUTSEL1::Hfsrcclk),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOUTSEL1::Disabled
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL1::Ulfrco
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL1::Lfrco
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL1::Lfxo
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL1::Hfxo
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == CLKOUTSEL1::Hfexpclk
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn is_ulfrcoq(&self) -> bool {
        *self == CLKOUTSEL1::Ulfrcoq
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn is_lfrcoq(&self) -> bool {
        *self == CLKOUTSEL1::Lfrcoq
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn is_lfxoq(&self) -> bool {
        *self == CLKOUTSEL1::Lfxoq
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn is_hfrcoq(&self) -> bool {
        *self == CLKOUTSEL1::Hfrcoq
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == CLKOUTSEL1::Auxhfrcoq
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn is_hfxoq(&self) -> bool {
        *self == CLKOUTSEL1::Hfxoq
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == CLKOUTSEL1::Hfsrcclk
    }
}
#[doc = "Field `CLKOUTSEL1` writer - Clock Output Select 1"]
pub type CLKOUTSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CLKOUTSEL1>;
impl<'a, REG> CLKOUTSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1::Disabled)
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1::Ulfrco)
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1::Lfrco)
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1::Lfxo)
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1::Hfxo)
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1::Hfexpclk)
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn ulfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1::Ulfrcoq)
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn lfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1::Lfrcoq)
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn lfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1::Lfxoq)
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn hfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1::Hfrcoq)
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn auxhfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1::Auxhfrcoq)
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn hfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1::Hfxoq)
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1::Hfsrcclk)
    }
}
#[doc = "Field `WSHFLE` reader - Wait State for High-Frequency LE Interface"]
pub type WSHFLE_R = crate::BitReader;
#[doc = "Field `WSHFLE` writer - Wait State for High-Frequency LE Interface"]
pub type WSHFLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFPERCLKEN` reader - HFPERCLK Enable"]
pub type HFPERCLKEN_R = crate::BitReader;
#[doc = "Field `HFPERCLKEN` writer - HFPERCLK Enable"]
pub type HFPERCLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Clock Output Select 0"]
    #[inline(always)]
    pub fn clkoutsel0(&self) -> CLKOUTSEL0_R {
        CLKOUTSEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - Clock Output Select 1"]
    #[inline(always)]
    pub fn clkoutsel1(&self) -> CLKOUTSEL1_R {
        CLKOUTSEL1_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Wait State for High-Frequency LE Interface"]
    #[inline(always)]
    pub fn wshfle(&self) -> WSHFLE_R {
        WSHFLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - HFPERCLK Enable"]
    #[inline(always)]
    pub fn hfperclken(&self) -> HFPERCLKEN_R {
        HFPERCLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Output Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutsel0(&mut self) -> CLKOUTSEL0_W<CTRLrs> {
        CLKOUTSEL0_W::new(self, 0)
    }
    #[doc = "Bits 5:8 - Clock Output Select 1"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutsel1(&mut self) -> CLKOUTSEL1_W<CTRLrs> {
        CLKOUTSEL1_W::new(self, 5)
    }
    #[doc = "Bit 16 - Wait State for High-Frequency LE Interface"]
    #[inline(always)]
    #[must_use]
    pub fn wshfle(&mut self) -> WSHFLE_W<CTRLrs> {
        WSHFLE_W::new(self, 16)
    }
    #[doc = "Bit 20 - HFPERCLK Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfperclken(&mut self) -> HFPERCLKEN_W<CTRLrs> {
        HFPERCLKEN_W::new(self, 20)
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
#[doc = "CMU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0x0010_0000"]
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0x0010_0000;
}
