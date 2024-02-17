#[doc = "Register `EXTIPINSELH` reader"]
pub type R = crate::R<EXTIPINSELHrs>;
#[doc = "Register `EXTIPINSELH` writer"]
pub type W = crate::W<EXTIPINSELHrs>;
#[doc = "Field `EXTIPINSEL8` reader - External Interrupt 8 Pin Select"]
pub type EXTIPINSEL8_R = crate::FieldReader<EXTIPINSEL8>;
#[doc = "External Interrupt 8 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL8 {
    #[doc = "0: Pin 8"]
    Pin8 = 0,
    #[doc = "1: Pin 9"]
    Pin9 = 1,
    #[doc = "2: Pin 10"]
    Pin10 = 2,
    #[doc = "3: Pin 11"]
    Pin11 = 3,
}
impl From<EXTIPINSEL8> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPINSEL8 {
    type Ux = u8;
}
impl EXTIPINSEL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTIPINSEL8 {
        match self.bits {
            0 => EXTIPINSEL8::Pin8,
            1 => EXTIPINSEL8::Pin9,
            2 => EXTIPINSEL8::Pin10,
            3 => EXTIPINSEL8::Pin11,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == EXTIPINSEL8::Pin8
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == EXTIPINSEL8::Pin9
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == EXTIPINSEL8::Pin10
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == EXTIPINSEL8::Pin11
    }
}
#[doc = "Field `EXTIPINSEL8` writer - External Interrupt 8 Pin Select"]
pub type EXTIPINSEL8_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EXTIPINSEL8>;
impl<'a, REG> EXTIPINSEL8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL8::Pin8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL8::Pin9)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL8::Pin10)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL8::Pin11)
    }
}
#[doc = "Field `EXTIPINSEL9` reader - External Interrupt 9 Pin Select"]
pub type EXTIPINSEL9_R = crate::FieldReader<EXTIPINSEL9>;
#[doc = "External Interrupt 9 Pin Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL9 {
    #[doc = "0: Pin 8"]
    Pin8 = 0,
    #[doc = "1: Pin 9"]
    Pin9 = 1,
    #[doc = "2: Pin 10"]
    Pin10 = 2,
    #[doc = "3: Pin 11"]
    Pin11 = 3,
}
impl From<EXTIPINSEL9> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPINSEL9 {
    type Ux = u8;
}
impl EXTIPINSEL9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTIPINSEL9 {
        match self.bits {
            0 => EXTIPINSEL9::Pin8,
            1 => EXTIPINSEL9::Pin9,
            2 => EXTIPINSEL9::Pin10,
            3 => EXTIPINSEL9::Pin11,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == EXTIPINSEL9::Pin8
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == EXTIPINSEL9::Pin9
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == EXTIPINSEL9::Pin10
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == EXTIPINSEL9::Pin11
    }
}
#[doc = "Field `EXTIPINSEL9` writer - External Interrupt 9 Pin Select"]
pub type EXTIPINSEL9_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EXTIPINSEL9>;
impl<'a, REG> EXTIPINSEL9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL9::Pin8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL9::Pin9)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL9::Pin10)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL9::Pin11)
    }
}
#[doc = "Field `EXTIPINSEL10` reader - External Interrupt 10 Pin Select"]
pub type EXTIPINSEL10_R = crate::FieldReader<EXTIPINSEL10>;
#[doc = "External Interrupt 10 Pin Select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL10 {
    #[doc = "0: Pin 8"]
    Pin8 = 0,
    #[doc = "1: Pin 9"]
    Pin9 = 1,
    #[doc = "2: Pin 10"]
    Pin10 = 2,
    #[doc = "3: Pin 11"]
    Pin11 = 3,
}
impl From<EXTIPINSEL10> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPINSEL10 {
    type Ux = u8;
}
impl EXTIPINSEL10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTIPINSEL10 {
        match self.bits {
            0 => EXTIPINSEL10::Pin8,
            1 => EXTIPINSEL10::Pin9,
            2 => EXTIPINSEL10::Pin10,
            3 => EXTIPINSEL10::Pin11,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == EXTIPINSEL10::Pin8
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == EXTIPINSEL10::Pin9
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == EXTIPINSEL10::Pin10
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == EXTIPINSEL10::Pin11
    }
}
#[doc = "Field `EXTIPINSEL10` writer - External Interrupt 10 Pin Select"]
pub type EXTIPINSEL10_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EXTIPINSEL10>;
impl<'a, REG> EXTIPINSEL10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL10::Pin8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL10::Pin9)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL10::Pin10)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL10::Pin11)
    }
}
#[doc = "Field `EXTIPINSEL11` reader - External Interrupt 11 Pin Select"]
pub type EXTIPINSEL11_R = crate::FieldReader<EXTIPINSEL11>;
#[doc = "External Interrupt 11 Pin Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL11 {
    #[doc = "0: Pin 8"]
    Pin8 = 0,
    #[doc = "1: Pin 9"]
    Pin9 = 1,
    #[doc = "2: Pin 10"]
    Pin10 = 2,
    #[doc = "3: Pin 11"]
    Pin11 = 3,
}
impl From<EXTIPINSEL11> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPINSEL11 {
    type Ux = u8;
}
impl EXTIPINSEL11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTIPINSEL11 {
        match self.bits {
            0 => EXTIPINSEL11::Pin8,
            1 => EXTIPINSEL11::Pin9,
            2 => EXTIPINSEL11::Pin10,
            3 => EXTIPINSEL11::Pin11,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == EXTIPINSEL11::Pin8
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == EXTIPINSEL11::Pin9
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == EXTIPINSEL11::Pin10
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == EXTIPINSEL11::Pin11
    }
}
#[doc = "Field `EXTIPINSEL11` writer - External Interrupt 11 Pin Select"]
pub type EXTIPINSEL11_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EXTIPINSEL11>;
impl<'a, REG> EXTIPINSEL11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL11::Pin8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL11::Pin9)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL11::Pin10)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL11::Pin11)
    }
}
#[doc = "Field `EXTIPINSEL12` reader - External Interrupt 12 Pin Select"]
pub type EXTIPINSEL12_R = crate::FieldReader<EXTIPINSEL12>;
#[doc = "External Interrupt 12 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL12 {
    #[doc = "0: Pin 12"]
    Pin12 = 0,
    #[doc = "1: Pin 13"]
    Pin13 = 1,
    #[doc = "2: Pin 14"]
    Pin14 = 2,
    #[doc = "3: Pin 15"]
    Pin15 = 3,
}
impl From<EXTIPINSEL12> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPINSEL12 {
    type Ux = u8;
}
impl EXTIPINSEL12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTIPINSEL12 {
        match self.bits {
            0 => EXTIPINSEL12::Pin12,
            1 => EXTIPINSEL12::Pin13,
            2 => EXTIPINSEL12::Pin14,
            3 => EXTIPINSEL12::Pin15,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == EXTIPINSEL12::Pin12
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == EXTIPINSEL12::Pin13
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == EXTIPINSEL12::Pin14
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == EXTIPINSEL12::Pin15
    }
}
#[doc = "Field `EXTIPINSEL12` writer - External Interrupt 12 Pin Select"]
pub type EXTIPINSEL12_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EXTIPINSEL12>;
impl<'a, REG> EXTIPINSEL12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL12::Pin12)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL12::Pin13)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL12::Pin14)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL12::Pin15)
    }
}
#[doc = "Field `EXTIPINSEL13` reader - External Interrupt 13 Pin Select"]
pub type EXTIPINSEL13_R = crate::FieldReader<EXTIPINSEL13>;
#[doc = "External Interrupt 13 Pin Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL13 {
    #[doc = "0: Pin 12"]
    Pin12 = 0,
    #[doc = "1: Pin 13"]
    Pin13 = 1,
    #[doc = "2: Pin 14"]
    Pin14 = 2,
    #[doc = "3: Pin 15"]
    Pin15 = 3,
}
impl From<EXTIPINSEL13> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPINSEL13 {
    type Ux = u8;
}
impl EXTIPINSEL13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTIPINSEL13 {
        match self.bits {
            0 => EXTIPINSEL13::Pin12,
            1 => EXTIPINSEL13::Pin13,
            2 => EXTIPINSEL13::Pin14,
            3 => EXTIPINSEL13::Pin15,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == EXTIPINSEL13::Pin12
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == EXTIPINSEL13::Pin13
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == EXTIPINSEL13::Pin14
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == EXTIPINSEL13::Pin15
    }
}
#[doc = "Field `EXTIPINSEL13` writer - External Interrupt 13 Pin Select"]
pub type EXTIPINSEL13_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EXTIPINSEL13>;
impl<'a, REG> EXTIPINSEL13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL13::Pin12)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL13::Pin13)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL13::Pin14)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL13::Pin15)
    }
}
#[doc = "Field `EXTIPINSEL14` reader - External Interrupt 14 Pin Select"]
pub type EXTIPINSEL14_R = crate::FieldReader<EXTIPINSEL14>;
#[doc = "External Interrupt 14 Pin Select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL14 {
    #[doc = "0: Pin 12"]
    Pin12 = 0,
    #[doc = "1: Pin 13"]
    Pin13 = 1,
    #[doc = "2: Pin 14"]
    Pin14 = 2,
    #[doc = "3: Pin 15"]
    Pin15 = 3,
}
impl From<EXTIPINSEL14> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPINSEL14 {
    type Ux = u8;
}
impl EXTIPINSEL14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTIPINSEL14 {
        match self.bits {
            0 => EXTIPINSEL14::Pin12,
            1 => EXTIPINSEL14::Pin13,
            2 => EXTIPINSEL14::Pin14,
            3 => EXTIPINSEL14::Pin15,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == EXTIPINSEL14::Pin12
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == EXTIPINSEL14::Pin13
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == EXTIPINSEL14::Pin14
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == EXTIPINSEL14::Pin15
    }
}
#[doc = "Field `EXTIPINSEL14` writer - External Interrupt 14 Pin Select"]
pub type EXTIPINSEL14_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EXTIPINSEL14>;
impl<'a, REG> EXTIPINSEL14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL14::Pin12)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL14::Pin13)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL14::Pin14)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL14::Pin15)
    }
}
#[doc = "Field `EXTIPINSEL15` reader - External Interrupt 15 Pin Select"]
pub type EXTIPINSEL15_R = crate::FieldReader<EXTIPINSEL15>;
#[doc = "External Interrupt 15 Pin Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL15 {
    #[doc = "0: Pin 12"]
    Pin12 = 0,
    #[doc = "1: Pin 13"]
    Pin13 = 1,
    #[doc = "2: Pin 14"]
    Pin14 = 2,
    #[doc = "3: Pin 15"]
    Pin15 = 3,
}
impl From<EXTIPINSEL15> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPINSEL15 {
    type Ux = u8;
}
impl EXTIPINSEL15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTIPINSEL15 {
        match self.bits {
            0 => EXTIPINSEL15::Pin12,
            1 => EXTIPINSEL15::Pin13,
            2 => EXTIPINSEL15::Pin14,
            3 => EXTIPINSEL15::Pin15,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == EXTIPINSEL15::Pin12
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == EXTIPINSEL15::Pin13
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == EXTIPINSEL15::Pin14
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == EXTIPINSEL15::Pin15
    }
}
#[doc = "Field `EXTIPINSEL15` writer - External Interrupt 15 Pin Select"]
pub type EXTIPINSEL15_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EXTIPINSEL15>;
impl<'a, REG> EXTIPINSEL15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL15::Pin12)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL15::Pin13)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL15::Pin14)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL15::Pin15)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Interrupt 8 Pin Select"]
    #[inline(always)]
    pub fn extipinsel8(&self) -> EXTIPINSEL8_R {
        EXTIPINSEL8_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Interrupt 9 Pin Select"]
    #[inline(always)]
    pub fn extipinsel9(&self) -> EXTIPINSEL9_R {
        EXTIPINSEL9_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Interrupt 10 Pin Select"]
    #[inline(always)]
    pub fn extipinsel10(&self) -> EXTIPINSEL10_R {
        EXTIPINSEL10_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External Interrupt 11 Pin Select"]
    #[inline(always)]
    pub fn extipinsel11(&self) -> EXTIPINSEL11_R {
        EXTIPINSEL11_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - External Interrupt 12 Pin Select"]
    #[inline(always)]
    pub fn extipinsel12(&self) -> EXTIPINSEL12_R {
        EXTIPINSEL12_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - External Interrupt 13 Pin Select"]
    #[inline(always)]
    pub fn extipinsel13(&self) -> EXTIPINSEL13_R {
        EXTIPINSEL13_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - External Interrupt 14 Pin Select"]
    #[inline(always)]
    pub fn extipinsel14(&self) -> EXTIPINSEL14_R {
        EXTIPINSEL14_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - External Interrupt 15 Pin Select"]
    #[inline(always)]
    pub fn extipinsel15(&self) -> EXTIPINSEL15_R {
        EXTIPINSEL15_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt 8 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel8(&mut self) -> EXTIPINSEL8_W<EXTIPINSELHrs> {
        EXTIPINSEL8_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - External Interrupt 9 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel9(&mut self) -> EXTIPINSEL9_W<EXTIPINSELHrs> {
        EXTIPINSEL9_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - External Interrupt 10 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel10(&mut self) -> EXTIPINSEL10_W<EXTIPINSELHrs> {
        EXTIPINSEL10_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - External Interrupt 11 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel11(&mut self) -> EXTIPINSEL11_W<EXTIPINSELHrs> {
        EXTIPINSEL11_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - External Interrupt 12 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel12(&mut self) -> EXTIPINSEL12_W<EXTIPINSELHrs> {
        EXTIPINSEL12_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - External Interrupt 13 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel13(&mut self) -> EXTIPINSEL13_W<EXTIPINSELHrs> {
        EXTIPINSEL13_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - External Interrupt 14 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel14(&mut self) -> EXTIPINSEL14_W<EXTIPINSELHrs> {
        EXTIPINSEL14_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - External Interrupt 15 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel15(&mut self) -> EXTIPINSEL15_W<EXTIPINSELHrs> {
        EXTIPINSEL15_W::new(self, 28)
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
#[doc = "External Interrupt Pin Select High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extipinselh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extipinselh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIPINSELHrs;
impl crate::RegisterSpec for EXTIPINSELHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extipinselh::R`](R) reader structure"]
impl crate::Readable for EXTIPINSELHrs {}
#[doc = "`write(|w| ..)` method takes [`extipinselh::W`](W) writer structure"]
impl crate::Writable for EXTIPINSELHrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTIPINSELH to value 0x3210_3210"]
impl crate::Resettable for EXTIPINSELHrs {
    const RESET_VALUE: u32 = 0x3210_3210;
}
