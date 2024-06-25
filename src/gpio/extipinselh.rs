#[doc = "Register `EXTIPINSELH` reader"]
pub type R = crate::R<EXTIPINSELHrs>;
#[doc = "Register `EXTIPINSELH` writer"]
pub type W = crate::W<EXTIPINSELHrs>;
#[doc = "External Interrupt 8 Pin Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
impl crate::IsEnum for EXTIPINSEL8 {}
#[doc = "Field `EXTIPINSEL8` reader - External Interrupt 8 Pin Select"]
pub type Extipinsel8R = crate::FieldReader<EXTIPINSEL8>;
impl Extipinsel8R {
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
pub type Extipinsel8W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTIPINSEL8, crate::Safe>;
impl<'a, REG> Extipinsel8W<'a, REG>
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
#[doc = "External Interrupt 9 Pin Select\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
impl crate::IsEnum for EXTIPINSEL9 {}
#[doc = "Field `EXTIPINSEL9` reader - External Interrupt 9 Pin Select"]
pub type Extipinsel9R = crate::FieldReader<EXTIPINSEL9>;
impl Extipinsel9R {
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
pub type Extipinsel9W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTIPINSEL9, crate::Safe>;
impl<'a, REG> Extipinsel9W<'a, REG>
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
#[doc = "External Interrupt 10 Pin Select\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
impl crate::IsEnum for EXTIPINSEL10 {}
#[doc = "Field `EXTIPINSEL10` reader - External Interrupt 10 Pin Select"]
pub type Extipinsel10R = crate::FieldReader<EXTIPINSEL10>;
impl Extipinsel10R {
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
pub type Extipinsel10W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTIPINSEL10, crate::Safe>;
impl<'a, REG> Extipinsel10W<'a, REG>
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
#[doc = "External Interrupt 11 Pin Select\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
impl crate::IsEnum for EXTIPINSEL11 {}
#[doc = "Field `EXTIPINSEL11` reader - External Interrupt 11 Pin Select"]
pub type Extipinsel11R = crate::FieldReader<EXTIPINSEL11>;
impl Extipinsel11R {
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
pub type Extipinsel11W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTIPINSEL11, crate::Safe>;
impl<'a, REG> Extipinsel11W<'a, REG>
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
#[doc = "External Interrupt 12 Pin Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
impl crate::IsEnum for EXTIPINSEL12 {}
#[doc = "Field `EXTIPINSEL12` reader - External Interrupt 12 Pin Select"]
pub type Extipinsel12R = crate::FieldReader<EXTIPINSEL12>;
impl Extipinsel12R {
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
pub type Extipinsel12W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTIPINSEL12, crate::Safe>;
impl<'a, REG> Extipinsel12W<'a, REG>
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
#[doc = "External Interrupt 13 Pin Select\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
impl crate::IsEnum for EXTIPINSEL13 {}
#[doc = "Field `EXTIPINSEL13` reader - External Interrupt 13 Pin Select"]
pub type Extipinsel13R = crate::FieldReader<EXTIPINSEL13>;
impl Extipinsel13R {
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
pub type Extipinsel13W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTIPINSEL13, crate::Safe>;
impl<'a, REG> Extipinsel13W<'a, REG>
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
#[doc = "External Interrupt 14 Pin Select\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
impl crate::IsEnum for EXTIPINSEL14 {}
#[doc = "Field `EXTIPINSEL14` reader - External Interrupt 14 Pin Select"]
pub type Extipinsel14R = crate::FieldReader<EXTIPINSEL14>;
impl Extipinsel14R {
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
pub type Extipinsel14W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTIPINSEL14, crate::Safe>;
impl<'a, REG> Extipinsel14W<'a, REG>
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
#[doc = "External Interrupt 15 Pin Select\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
impl crate::IsEnum for EXTIPINSEL15 {}
#[doc = "Field `EXTIPINSEL15` reader - External Interrupt 15 Pin Select"]
pub type Extipinsel15R = crate::FieldReader<EXTIPINSEL15>;
impl Extipinsel15R {
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
pub type Extipinsel15W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTIPINSEL15, crate::Safe>;
impl<'a, REG> Extipinsel15W<'a, REG>
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
    pub fn extipinsel8(&self) -> Extipinsel8R {
        Extipinsel8R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Interrupt 9 Pin Select"]
    #[inline(always)]
    pub fn extipinsel9(&self) -> Extipinsel9R {
        Extipinsel9R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Interrupt 10 Pin Select"]
    #[inline(always)]
    pub fn extipinsel10(&self) -> Extipinsel10R {
        Extipinsel10R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External Interrupt 11 Pin Select"]
    #[inline(always)]
    pub fn extipinsel11(&self) -> Extipinsel11R {
        Extipinsel11R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - External Interrupt 12 Pin Select"]
    #[inline(always)]
    pub fn extipinsel12(&self) -> Extipinsel12R {
        Extipinsel12R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - External Interrupt 13 Pin Select"]
    #[inline(always)]
    pub fn extipinsel13(&self) -> Extipinsel13R {
        Extipinsel13R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - External Interrupt 14 Pin Select"]
    #[inline(always)]
    pub fn extipinsel14(&self) -> Extipinsel14R {
        Extipinsel14R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - External Interrupt 15 Pin Select"]
    #[inline(always)]
    pub fn extipinsel15(&self) -> Extipinsel15R {
        Extipinsel15R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTIPINSELH")
            .field("extipinsel8", &self.extipinsel8())
            .field("extipinsel9", &self.extipinsel9())
            .field("extipinsel10", &self.extipinsel10())
            .field("extipinsel11", &self.extipinsel11())
            .field("extipinsel12", &self.extipinsel12())
            .field("extipinsel13", &self.extipinsel13())
            .field("extipinsel14", &self.extipinsel14())
            .field("extipinsel15", &self.extipinsel15())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt 8 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel8(&mut self) -> Extipinsel8W<EXTIPINSELHrs> {
        Extipinsel8W::new(self, 0)
    }
    #[doc = "Bits 4:5 - External Interrupt 9 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel9(&mut self) -> Extipinsel9W<EXTIPINSELHrs> {
        Extipinsel9W::new(self, 4)
    }
    #[doc = "Bits 8:9 - External Interrupt 10 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel10(&mut self) -> Extipinsel10W<EXTIPINSELHrs> {
        Extipinsel10W::new(self, 8)
    }
    #[doc = "Bits 12:13 - External Interrupt 11 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel11(&mut self) -> Extipinsel11W<EXTIPINSELHrs> {
        Extipinsel11W::new(self, 12)
    }
    #[doc = "Bits 16:17 - External Interrupt 12 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel12(&mut self) -> Extipinsel12W<EXTIPINSELHrs> {
        Extipinsel12W::new(self, 16)
    }
    #[doc = "Bits 20:21 - External Interrupt 13 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel13(&mut self) -> Extipinsel13W<EXTIPINSELHrs> {
        Extipinsel13W::new(self, 20)
    }
    #[doc = "Bits 24:25 - External Interrupt 14 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel14(&mut self) -> Extipinsel14W<EXTIPINSELHrs> {
        Extipinsel14W::new(self, 24)
    }
    #[doc = "Bits 28:29 - External Interrupt 15 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel15(&mut self) -> Extipinsel15W<EXTIPINSELHrs> {
        Extipinsel15W::new(self, 28)
    }
}
#[doc = "External Interrupt Pin Select High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extipinselh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipinselh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIPINSELHrs;
impl crate::RegisterSpec for EXTIPINSELHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extipinselh::R`](R) reader structure"]
impl crate::Readable for EXTIPINSELHrs {}
#[doc = "`write(|w| ..)` method takes [`extipinselh::W`](W) writer structure"]
impl crate::Writable for EXTIPINSELHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTIPINSELH to value 0x3210_3210"]
impl crate::Resettable for EXTIPINSELHrs {
    const RESET_VALUE: u32 = 0x3210_3210;
}
