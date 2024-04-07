#[doc = "Register `WAC` reader"]
pub type R = crate::R<WACrs>;
#[doc = "Register `WAC` writer"]
pub type W = crate::W<WACrs>;
#[doc = "Modular Operation Modulus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODULUS {
    #[doc = "0: Generic modulus. p = 2^256"]
    Bin256 = 0,
    #[doc = "1: Generic modulus. p = 2^128"]
    Bin128 = 1,
    #[doc = "2: Modulus for B-233 and K-233 ECC curves. p(t) = t^233 + t^74 + 1"]
    Eccbin233p = 2,
    #[doc = "3: Modulus for B-163 and K-163 ECC curves. p(t) = t^163 + t^7 + t^6 + t^3 + 1"]
    Eccbin163p = 3,
    #[doc = "4: Modulus for GCM. P(t) = t^128 + t^7 + t^2 + t + 1"]
    Gcmbin128 = 4,
    #[doc = "5: Modulus for P-256 ECC curve. p = 2^256 - 2^224 + 2^192 + 2^96 - 1"]
    Eccprime256p = 5,
    #[doc = "6: Modulus for P-224 ECC curve. p = 2^224 - 2^96 - 1"]
    Eccprime224p = 6,
    #[doc = "7: Modulus for P-192 ECC curve. p = 2^192 - 2^64 - 1"]
    Eccprime192p = 7,
    #[doc = "8: P modulus for B-233 ECC curve"]
    Eccbin233n = 8,
    #[doc = "9: P modulus for K-233 ECC curve"]
    Eccbin233kn = 9,
    #[doc = "10: P modulus for B-163 ECC curve"]
    Eccbin163n = 10,
    #[doc = "11: P modulus for K-163 ECC curve"]
    Eccbin163kn = 11,
    #[doc = "12: P modulus for P-256 ECC curve"]
    Eccprime256n = 12,
    #[doc = "13: P modulus for P-224 ECC curve"]
    Eccprime224n = 13,
    #[doc = "14: P modulus for P-192 ECC curve"]
    Eccprime192n = 14,
}
impl From<MODULUS> for u8 {
    #[inline(always)]
    fn from(variant: MODULUS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODULUS {
    type Ux = u8;
}
impl crate::IsEnum for MODULUS {}
#[doc = "Field `MODULUS` reader - Modular Operation Modulus"]
pub type ModulusR = crate::FieldReader<MODULUS>;
impl ModulusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODULUS> {
        match self.bits {
            0 => Some(MODULUS::Bin256),
            1 => Some(MODULUS::Bin128),
            2 => Some(MODULUS::Eccbin233p),
            3 => Some(MODULUS::Eccbin163p),
            4 => Some(MODULUS::Gcmbin128),
            5 => Some(MODULUS::Eccprime256p),
            6 => Some(MODULUS::Eccprime224p),
            7 => Some(MODULUS::Eccprime192p),
            8 => Some(MODULUS::Eccbin233n),
            9 => Some(MODULUS::Eccbin233kn),
            10 => Some(MODULUS::Eccbin163n),
            11 => Some(MODULUS::Eccbin163kn),
            12 => Some(MODULUS::Eccprime256n),
            13 => Some(MODULUS::Eccprime224n),
            14 => Some(MODULUS::Eccprime192n),
            _ => None,
        }
    }
    #[doc = "Generic modulus. p = 2^256"]
    #[inline(always)]
    pub fn is_bin256(&self) -> bool {
        *self == MODULUS::Bin256
    }
    #[doc = "Generic modulus. p = 2^128"]
    #[inline(always)]
    pub fn is_bin128(&self) -> bool {
        *self == MODULUS::Bin128
    }
    #[doc = "Modulus for B-233 and K-233 ECC curves. p(t) = t^233 + t^74 + 1"]
    #[inline(always)]
    pub fn is_eccbin233p(&self) -> bool {
        *self == MODULUS::Eccbin233p
    }
    #[doc = "Modulus for B-163 and K-163 ECC curves. p(t) = t^163 + t^7 + t^6 + t^3 + 1"]
    #[inline(always)]
    pub fn is_eccbin163p(&self) -> bool {
        *self == MODULUS::Eccbin163p
    }
    #[doc = "Modulus for GCM. P(t) = t^128 + t^7 + t^2 + t + 1"]
    #[inline(always)]
    pub fn is_gcmbin128(&self) -> bool {
        *self == MODULUS::Gcmbin128
    }
    #[doc = "Modulus for P-256 ECC curve. p = 2^256 - 2^224 + 2^192 + 2^96 - 1"]
    #[inline(always)]
    pub fn is_eccprime256p(&self) -> bool {
        *self == MODULUS::Eccprime256p
    }
    #[doc = "Modulus for P-224 ECC curve. p = 2^224 - 2^96 - 1"]
    #[inline(always)]
    pub fn is_eccprime224p(&self) -> bool {
        *self == MODULUS::Eccprime224p
    }
    #[doc = "Modulus for P-192 ECC curve. p = 2^192 - 2^64 - 1"]
    #[inline(always)]
    pub fn is_eccprime192p(&self) -> bool {
        *self == MODULUS::Eccprime192p
    }
    #[doc = "P modulus for B-233 ECC curve"]
    #[inline(always)]
    pub fn is_eccbin233n(&self) -> bool {
        *self == MODULUS::Eccbin233n
    }
    #[doc = "P modulus for K-233 ECC curve"]
    #[inline(always)]
    pub fn is_eccbin233kn(&self) -> bool {
        *self == MODULUS::Eccbin233kn
    }
    #[doc = "P modulus for B-163 ECC curve"]
    #[inline(always)]
    pub fn is_eccbin163n(&self) -> bool {
        *self == MODULUS::Eccbin163n
    }
    #[doc = "P modulus for K-163 ECC curve"]
    #[inline(always)]
    pub fn is_eccbin163kn(&self) -> bool {
        *self == MODULUS::Eccbin163kn
    }
    #[doc = "P modulus for P-256 ECC curve"]
    #[inline(always)]
    pub fn is_eccprime256n(&self) -> bool {
        *self == MODULUS::Eccprime256n
    }
    #[doc = "P modulus for P-224 ECC curve"]
    #[inline(always)]
    pub fn is_eccprime224n(&self) -> bool {
        *self == MODULUS::Eccprime224n
    }
    #[doc = "P modulus for P-192 ECC curve"]
    #[inline(always)]
    pub fn is_eccprime192n(&self) -> bool {
        *self == MODULUS::Eccprime192n
    }
}
#[doc = "Field `MODULUS` writer - Modular Operation Modulus"]
pub type ModulusW<'a, REG> = crate::FieldWriter<'a, REG, 4, MODULUS>;
impl<'a, REG> ModulusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generic modulus. p = 2^256"]
    #[inline(always)]
    pub fn bin256(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS::Bin256)
    }
    #[doc = "Generic modulus. p = 2^128"]
    #[inline(always)]
    pub fn bin128(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS::Bin128)
    }
    #[doc = "Modulus for B-233 and K-233 ECC curves. p(t) = t^233 + t^74 + 1"]
    #[inline(always)]
    pub fn eccbin233p(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS::Eccbin233p)
    }
    #[doc = "Modulus for B-163 and K-163 ECC curves. p(t) = t^163 + t^7 + t^6 + t^3 + 1"]
    #[inline(always)]
    pub fn eccbin163p(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS::Eccbin163p)
    }
    #[doc = "Modulus for GCM. P(t) = t^128 + t^7 + t^2 + t + 1"]
    #[inline(always)]
    pub fn gcmbin128(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS::Gcmbin128)
    }
    #[doc = "Modulus for P-256 ECC curve. p = 2^256 - 2^224 + 2^192 + 2^96 - 1"]
    #[inline(always)]
    pub fn eccprime256p(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS::Eccprime256p)
    }
    #[doc = "Modulus for P-224 ECC curve. p = 2^224 - 2^96 - 1"]
    #[inline(always)]
    pub fn eccprime224p(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS::Eccprime224p)
    }
    #[doc = "Modulus for P-192 ECC curve. p = 2^192 - 2^64 - 1"]
    #[inline(always)]
    pub fn eccprime192p(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS::Eccprime192p)
    }
    #[doc = "P modulus for B-233 ECC curve"]
    #[inline(always)]
    pub fn eccbin233n(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS::Eccbin233n)
    }
    #[doc = "P modulus for K-233 ECC curve"]
    #[inline(always)]
    pub fn eccbin233kn(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS::Eccbin233kn)
    }
    #[doc = "P modulus for B-163 ECC curve"]
    #[inline(always)]
    pub fn eccbin163n(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS::Eccbin163n)
    }
    #[doc = "P modulus for K-163 ECC curve"]
    #[inline(always)]
    pub fn eccbin163kn(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS::Eccbin163kn)
    }
    #[doc = "P modulus for P-256 ECC curve"]
    #[inline(always)]
    pub fn eccprime256n(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS::Eccprime256n)
    }
    #[doc = "P modulus for P-224 ECC curve"]
    #[inline(always)]
    pub fn eccprime224n(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS::Eccprime224n)
    }
    #[doc = "P modulus for P-192 ECC curve"]
    #[inline(always)]
    pub fn eccprime192n(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS::Eccprime192n)
    }
}
#[doc = "Field `MODOP` reader - Modular Operation Field Type"]
pub type ModopR = crate::BitReader;
#[doc = "Field `MODOP` writer - Modular Operation Field Type"]
pub type ModopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Multiply Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MULWIDTH {
    #[doc = "0: Multiply 256 bits"]
    Mul256 = 0,
    #[doc = "1: Multiply 128 bits"]
    Mul128 = 1,
    #[doc = "2: Same number of bits as specified by MODULUS"]
    Mulmod = 2,
}
impl From<MULWIDTH> for u8 {
    #[inline(always)]
    fn from(variant: MULWIDTH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MULWIDTH {
    type Ux = u8;
}
impl crate::IsEnum for MULWIDTH {}
#[doc = "Field `MULWIDTH` reader - Multiply Width"]
pub type MulwidthR = crate::FieldReader<MULWIDTH>;
impl MulwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MULWIDTH> {
        match self.bits {
            0 => Some(MULWIDTH::Mul256),
            1 => Some(MULWIDTH::Mul128),
            2 => Some(MULWIDTH::Mulmod),
            _ => None,
        }
    }
    #[doc = "Multiply 256 bits"]
    #[inline(always)]
    pub fn is_mul256(&self) -> bool {
        *self == MULWIDTH::Mul256
    }
    #[doc = "Multiply 128 bits"]
    #[inline(always)]
    pub fn is_mul128(&self) -> bool {
        *self == MULWIDTH::Mul128
    }
    #[doc = "Same number of bits as specified by MODULUS"]
    #[inline(always)]
    pub fn is_mulmod(&self) -> bool {
        *self == MULWIDTH::Mulmod
    }
}
#[doc = "Field `MULWIDTH` writer - Multiply Width"]
pub type MulwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, MULWIDTH>;
impl<'a, REG> MulwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Multiply 256 bits"]
    #[inline(always)]
    pub fn mul256(self) -> &'a mut crate::W<REG> {
        self.variant(MULWIDTH::Mul256)
    }
    #[doc = "Multiply 128 bits"]
    #[inline(always)]
    pub fn mul128(self) -> &'a mut crate::W<REG> {
        self.variant(MULWIDTH::Mul128)
    }
    #[doc = "Same number of bits as specified by MODULUS"]
    #[inline(always)]
    pub fn mulmod(self) -> &'a mut crate::W<REG> {
        self.variant(MULWIDTH::Mulmod)
    }
}
#[doc = "Result Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESULTWIDTH {
    #[doc = "0: Results have 256 bits"]
    _256bit = 0,
    #[doc = "1: Results have 128 bits"]
    _128bit = 1,
    #[doc = "2: Results have 260 bits. Upper bits of result can be read through DDATA0MSBS in CRYPTO_STATUS"]
    _260bit = 2,
}
impl From<RESULTWIDTH> for u8 {
    #[inline(always)]
    fn from(variant: RESULTWIDTH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RESULTWIDTH {
    type Ux = u8;
}
impl crate::IsEnum for RESULTWIDTH {}
#[doc = "Field `RESULTWIDTH` reader - Result Width"]
pub type ResultwidthR = crate::FieldReader<RESULTWIDTH>;
impl ResultwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RESULTWIDTH> {
        match self.bits {
            0 => Some(RESULTWIDTH::_256bit),
            1 => Some(RESULTWIDTH::_128bit),
            2 => Some(RESULTWIDTH::_260bit),
            _ => None,
        }
    }
    #[doc = "Results have 256 bits"]
    #[inline(always)]
    pub fn is_256bit(&self) -> bool {
        *self == RESULTWIDTH::_256bit
    }
    #[doc = "Results have 128 bits"]
    #[inline(always)]
    pub fn is_128bit(&self) -> bool {
        *self == RESULTWIDTH::_128bit
    }
    #[doc = "Results have 260 bits. Upper bits of result can be read through DDATA0MSBS in CRYPTO_STATUS"]
    #[inline(always)]
    pub fn is_260bit(&self) -> bool {
        *self == RESULTWIDTH::_260bit
    }
}
#[doc = "Field `RESULTWIDTH` writer - Result Width"]
pub type ResultwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, RESULTWIDTH>;
impl<'a, REG> ResultwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Results have 256 bits"]
    #[inline(always)]
    pub fn _256bit(self) -> &'a mut crate::W<REG> {
        self.variant(RESULTWIDTH::_256bit)
    }
    #[doc = "Results have 128 bits"]
    #[inline(always)]
    pub fn _128bit(self) -> &'a mut crate::W<REG> {
        self.variant(RESULTWIDTH::_128bit)
    }
    #[doc = "Results have 260 bits. Upper bits of result can be read through DDATA0MSBS in CRYPTO_STATUS"]
    #[inline(always)]
    pub fn _260bit(self) -> &'a mut crate::W<REG> {
        self.variant(RESULTWIDTH::_260bit)
    }
}
impl R {
    #[doc = "Bits 0:3 - Modular Operation Modulus"]
    #[inline(always)]
    pub fn modulus(&self) -> ModulusR {
        ModulusR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Modular Operation Field Type"]
    #[inline(always)]
    pub fn modop(&self) -> ModopR {
        ModopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Multiply Width"]
    #[inline(always)]
    pub fn mulwidth(&self) -> MulwidthR {
        MulwidthR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Result Width"]
    #[inline(always)]
    pub fn resultwidth(&self) -> ResultwidthR {
        ResultwidthR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Modular Operation Modulus"]
    #[inline(always)]
    #[must_use]
    pub fn modulus(&mut self) -> ModulusW<WACrs> {
        ModulusW::new(self, 0)
    }
    #[doc = "Bit 4 - Modular Operation Field Type"]
    #[inline(always)]
    #[must_use]
    pub fn modop(&mut self) -> ModopW<WACrs> {
        ModopW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Multiply Width"]
    #[inline(always)]
    #[must_use]
    pub fn mulwidth(&mut self) -> MulwidthW<WACrs> {
        MulwidthW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Result Width"]
    #[inline(always)]
    #[must_use]
    pub fn resultwidth(&mut self) -> ResultwidthW<WACrs> {
        ResultwidthW::new(self, 10)
    }
}
#[doc = "Wide Arithmetic Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WACrs;
impl crate::RegisterSpec for WACrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wac::R`](R) reader structure"]
impl crate::Readable for WACrs {}
#[doc = "`write(|w| ..)` method takes [`wac::W`](W) writer structure"]
impl crate::Writable for WACrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAC to value 0"]
impl crate::Resettable for WACrs {
    const RESET_VALUE: u32 = 0;
}
