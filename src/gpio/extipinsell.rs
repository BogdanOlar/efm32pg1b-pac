#[doc = "Register `EXTIPINSELL` reader"]
pub type R = crate::R<EXTIPINSELLrs>;
#[doc = "Register `EXTIPINSELL` writer"]
pub type W = crate::W<EXTIPINSELLrs>;
#[doc = "External Interrupt 0 Pin Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL0 {
    #[doc = "0: Pin 0"]
    Pin0 = 0,
    #[doc = "1: Pin 1"]
    Pin1 = 1,
    #[doc = "2: Pin 2"]
    Pin2 = 2,
    #[doc = "3: Pin 3"]
    Pin3 = 3,
}
impl From<EXTIPINSEL0> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPINSEL0 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPINSEL0 {}
#[doc = "Field `EXTIPINSEL0` reader - External Interrupt 0 Pin Select"]
pub type Extipinsel0R = crate::FieldReader<EXTIPINSEL0>;
impl Extipinsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTIPINSEL0 {
        match self.bits {
            0 => EXTIPINSEL0::Pin0,
            1 => EXTIPINSEL0::Pin1,
            2 => EXTIPINSEL0::Pin2,
            3 => EXTIPINSEL0::Pin3,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL0::Pin0
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL0::Pin1
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL0::Pin2
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL0::Pin3
    }
}
#[doc = "Field `EXTIPINSEL0` writer - External Interrupt 0 Pin Select"]
pub type Extipinsel0W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTIPINSEL0, crate::Safe>;
impl<'a, REG> Extipinsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL0::Pin0)
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL0::Pin1)
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL0::Pin2)
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL0::Pin3)
    }
}
#[doc = "External Interrupt 1 Pin Select\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL1 {
    #[doc = "0: Pin 0"]
    Pin0 = 0,
    #[doc = "1: Pin 1"]
    Pin1 = 1,
    #[doc = "2: Pin 2"]
    Pin2 = 2,
    #[doc = "3: Pin 3"]
    Pin3 = 3,
}
impl From<EXTIPINSEL1> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPINSEL1 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPINSEL1 {}
#[doc = "Field `EXTIPINSEL1` reader - External Interrupt 1 Pin Select"]
pub type Extipinsel1R = crate::FieldReader<EXTIPINSEL1>;
impl Extipinsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTIPINSEL1 {
        match self.bits {
            0 => EXTIPINSEL1::Pin0,
            1 => EXTIPINSEL1::Pin1,
            2 => EXTIPINSEL1::Pin2,
            3 => EXTIPINSEL1::Pin3,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL1::Pin0
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL1::Pin1
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL1::Pin2
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL1::Pin3
    }
}
#[doc = "Field `EXTIPINSEL1` writer - External Interrupt 1 Pin Select"]
pub type Extipinsel1W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTIPINSEL1, crate::Safe>;
impl<'a, REG> Extipinsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL1::Pin0)
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL1::Pin1)
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL1::Pin2)
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL1::Pin3)
    }
}
#[doc = "External Interrupt 2 Pin Select\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL2 {
    #[doc = "0: Pin 0"]
    Pin0 = 0,
    #[doc = "1: Pin 1"]
    Pin1 = 1,
    #[doc = "2: Pin 2"]
    Pin2 = 2,
    #[doc = "3: Pin 3"]
    Pin3 = 3,
}
impl From<EXTIPINSEL2> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPINSEL2 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPINSEL2 {}
#[doc = "Field `EXTIPINSEL2` reader - External Interrupt 2 Pin Select"]
pub type Extipinsel2R = crate::FieldReader<EXTIPINSEL2>;
impl Extipinsel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTIPINSEL2 {
        match self.bits {
            0 => EXTIPINSEL2::Pin0,
            1 => EXTIPINSEL2::Pin1,
            2 => EXTIPINSEL2::Pin2,
            3 => EXTIPINSEL2::Pin3,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL2::Pin0
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL2::Pin1
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL2::Pin2
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL2::Pin3
    }
}
#[doc = "Field `EXTIPINSEL2` writer - External Interrupt 2 Pin Select"]
pub type Extipinsel2W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTIPINSEL2, crate::Safe>;
impl<'a, REG> Extipinsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL2::Pin0)
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL2::Pin1)
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL2::Pin2)
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL2::Pin3)
    }
}
#[doc = "External Interrupt 3 Pin Select\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL3 {
    #[doc = "0: Pin 0"]
    Pin0 = 0,
    #[doc = "1: Pin 1"]
    Pin1 = 1,
    #[doc = "2: Pin 2"]
    Pin2 = 2,
    #[doc = "3: Pin 3"]
    Pin3 = 3,
}
impl From<EXTIPINSEL3> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPINSEL3 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPINSEL3 {}
#[doc = "Field `EXTIPINSEL3` reader - External Interrupt 3 Pin Select"]
pub type Extipinsel3R = crate::FieldReader<EXTIPINSEL3>;
impl Extipinsel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTIPINSEL3 {
        match self.bits {
            0 => EXTIPINSEL3::Pin0,
            1 => EXTIPINSEL3::Pin1,
            2 => EXTIPINSEL3::Pin2,
            3 => EXTIPINSEL3::Pin3,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL3::Pin0
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL3::Pin1
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL3::Pin2
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL3::Pin3
    }
}
#[doc = "Field `EXTIPINSEL3` writer - External Interrupt 3 Pin Select"]
pub type Extipinsel3W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTIPINSEL3, crate::Safe>;
impl<'a, REG> Extipinsel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL3::Pin0)
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL3::Pin1)
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL3::Pin2)
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL3::Pin3)
    }
}
#[doc = "External Interrupt 4 Pin Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL4 {
    #[doc = "0: Pin 4"]
    Pin4 = 0,
    #[doc = "1: Pin 5"]
    Pin5 = 1,
    #[doc = "2: Pin 6"]
    Pin6 = 2,
    #[doc = "3: Pin 7"]
    Pin7 = 3,
}
impl From<EXTIPINSEL4> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPINSEL4 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPINSEL4 {}
#[doc = "Field `EXTIPINSEL4` reader - External Interrupt 4 Pin Select"]
pub type Extipinsel4R = crate::FieldReader<EXTIPINSEL4>;
impl Extipinsel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTIPINSEL4 {
        match self.bits {
            0 => EXTIPINSEL4::Pin4,
            1 => EXTIPINSEL4::Pin5,
            2 => EXTIPINSEL4::Pin6,
            3 => EXTIPINSEL4::Pin7,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == EXTIPINSEL4::Pin4
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == EXTIPINSEL4::Pin5
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == EXTIPINSEL4::Pin6
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == EXTIPINSEL4::Pin7
    }
}
#[doc = "Field `EXTIPINSEL4` writer - External Interrupt 4 Pin Select"]
pub type Extipinsel4W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTIPINSEL4, crate::Safe>;
impl<'a, REG> Extipinsel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL4::Pin4)
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL4::Pin5)
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL4::Pin6)
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL4::Pin7)
    }
}
#[doc = "External Interrupt 5 Pin Select\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL5 {
    #[doc = "0: Pin 4"]
    Pin4 = 0,
    #[doc = "1: Pin 5"]
    Pin5 = 1,
    #[doc = "2: Pin 6"]
    Pin6 = 2,
    #[doc = "3: Pin 7"]
    Pin7 = 3,
}
impl From<EXTIPINSEL5> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPINSEL5 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPINSEL5 {}
#[doc = "Field `EXTIPINSEL5` reader - External Interrupt 5 Pin Select"]
pub type Extipinsel5R = crate::FieldReader<EXTIPINSEL5>;
impl Extipinsel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTIPINSEL5 {
        match self.bits {
            0 => EXTIPINSEL5::Pin4,
            1 => EXTIPINSEL5::Pin5,
            2 => EXTIPINSEL5::Pin6,
            3 => EXTIPINSEL5::Pin7,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == EXTIPINSEL5::Pin4
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == EXTIPINSEL5::Pin5
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == EXTIPINSEL5::Pin6
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == EXTIPINSEL5::Pin7
    }
}
#[doc = "Field `EXTIPINSEL5` writer - External Interrupt 5 Pin Select"]
pub type Extipinsel5W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTIPINSEL5, crate::Safe>;
impl<'a, REG> Extipinsel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL5::Pin4)
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL5::Pin5)
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL5::Pin6)
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL5::Pin7)
    }
}
#[doc = "External Interrupt 6 Pin Select\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL6 {
    #[doc = "0: Pin 4"]
    Pin4 = 0,
    #[doc = "1: Pin 5"]
    Pin5 = 1,
    #[doc = "2: Pin 6"]
    Pin6 = 2,
    #[doc = "3: Pin 7"]
    Pin7 = 3,
}
impl From<EXTIPINSEL6> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPINSEL6 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPINSEL6 {}
#[doc = "Field `EXTIPINSEL6` reader - External Interrupt 6 Pin Select"]
pub type Extipinsel6R = crate::FieldReader<EXTIPINSEL6>;
impl Extipinsel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTIPINSEL6 {
        match self.bits {
            0 => EXTIPINSEL6::Pin4,
            1 => EXTIPINSEL6::Pin5,
            2 => EXTIPINSEL6::Pin6,
            3 => EXTIPINSEL6::Pin7,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == EXTIPINSEL6::Pin4
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == EXTIPINSEL6::Pin5
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == EXTIPINSEL6::Pin6
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == EXTIPINSEL6::Pin7
    }
}
#[doc = "Field `EXTIPINSEL6` writer - External Interrupt 6 Pin Select"]
pub type Extipinsel6W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTIPINSEL6, crate::Safe>;
impl<'a, REG> Extipinsel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL6::Pin4)
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL6::Pin5)
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL6::Pin6)
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL6::Pin7)
    }
}
#[doc = "External Interrupt 7 Pin Select\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL7 {
    #[doc = "0: Pin 4"]
    Pin4 = 0,
    #[doc = "1: Pin 5"]
    Pin5 = 1,
    #[doc = "2: Pin 6"]
    Pin6 = 2,
    #[doc = "3: Pin 7"]
    Pin7 = 3,
}
impl From<EXTIPINSEL7> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPINSEL7 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPINSEL7 {}
#[doc = "Field `EXTIPINSEL7` reader - External Interrupt 7 Pin Select"]
pub type Extipinsel7R = crate::FieldReader<EXTIPINSEL7>;
impl Extipinsel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTIPINSEL7 {
        match self.bits {
            0 => EXTIPINSEL7::Pin4,
            1 => EXTIPINSEL7::Pin5,
            2 => EXTIPINSEL7::Pin6,
            3 => EXTIPINSEL7::Pin7,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == EXTIPINSEL7::Pin4
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == EXTIPINSEL7::Pin5
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == EXTIPINSEL7::Pin6
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == EXTIPINSEL7::Pin7
    }
}
#[doc = "Field `EXTIPINSEL7` writer - External Interrupt 7 Pin Select"]
pub type Extipinsel7W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTIPINSEL7, crate::Safe>;
impl<'a, REG> Extipinsel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL7::Pin4)
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL7::Pin5)
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL7::Pin6)
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPINSEL7::Pin7)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Interrupt 0 Pin Select"]
    #[inline(always)]
    pub fn extipinsel0(&self) -> Extipinsel0R {
        Extipinsel0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Interrupt 1 Pin Select"]
    #[inline(always)]
    pub fn extipinsel1(&self) -> Extipinsel1R {
        Extipinsel1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Interrupt 2 Pin Select"]
    #[inline(always)]
    pub fn extipinsel2(&self) -> Extipinsel2R {
        Extipinsel2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External Interrupt 3 Pin Select"]
    #[inline(always)]
    pub fn extipinsel3(&self) -> Extipinsel3R {
        Extipinsel3R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - External Interrupt 4 Pin Select"]
    #[inline(always)]
    pub fn extipinsel4(&self) -> Extipinsel4R {
        Extipinsel4R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - External Interrupt 5 Pin Select"]
    #[inline(always)]
    pub fn extipinsel5(&self) -> Extipinsel5R {
        Extipinsel5R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - External Interrupt 6 Pin Select"]
    #[inline(always)]
    pub fn extipinsel6(&self) -> Extipinsel6R {
        Extipinsel6R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - External Interrupt 7 Pin Select"]
    #[inline(always)]
    pub fn extipinsel7(&self) -> Extipinsel7R {
        Extipinsel7R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTIPINSELL")
            .field("extipinsel0", &self.extipinsel0())
            .field("extipinsel1", &self.extipinsel1())
            .field("extipinsel2", &self.extipinsel2())
            .field("extipinsel3", &self.extipinsel3())
            .field("extipinsel4", &self.extipinsel4())
            .field("extipinsel5", &self.extipinsel5())
            .field("extipinsel6", &self.extipinsel6())
            .field("extipinsel7", &self.extipinsel7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt 0 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel0(&mut self) -> Extipinsel0W<EXTIPINSELLrs> {
        Extipinsel0W::new(self, 0)
    }
    #[doc = "Bits 4:5 - External Interrupt 1 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel1(&mut self) -> Extipinsel1W<EXTIPINSELLrs> {
        Extipinsel1W::new(self, 4)
    }
    #[doc = "Bits 8:9 - External Interrupt 2 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel2(&mut self) -> Extipinsel2W<EXTIPINSELLrs> {
        Extipinsel2W::new(self, 8)
    }
    #[doc = "Bits 12:13 - External Interrupt 3 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel3(&mut self) -> Extipinsel3W<EXTIPINSELLrs> {
        Extipinsel3W::new(self, 12)
    }
    #[doc = "Bits 16:17 - External Interrupt 4 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel4(&mut self) -> Extipinsel4W<EXTIPINSELLrs> {
        Extipinsel4W::new(self, 16)
    }
    #[doc = "Bits 20:21 - External Interrupt 5 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel5(&mut self) -> Extipinsel5W<EXTIPINSELLrs> {
        Extipinsel5W::new(self, 20)
    }
    #[doc = "Bits 24:25 - External Interrupt 6 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel6(&mut self) -> Extipinsel6W<EXTIPINSELLrs> {
        Extipinsel6W::new(self, 24)
    }
    #[doc = "Bits 28:29 - External Interrupt 7 Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel7(&mut self) -> Extipinsel7W<EXTIPINSELLrs> {
        Extipinsel7W::new(self, 28)
    }
}
#[doc = "External Interrupt Pin Select Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extipinsell::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipinsell::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIPINSELLrs;
impl crate::RegisterSpec for EXTIPINSELLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extipinsell::R`](R) reader structure"]
impl crate::Readable for EXTIPINSELLrs {}
#[doc = "`write(|w| ..)` method takes [`extipinsell::W`](W) writer structure"]
impl crate::Writable for EXTIPINSELLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTIPINSELL to value 0x3210_3210"]
impl crate::Resettable for EXTIPINSELLrs {
    const RESET_VALUE: u32 = 0x3210_3210;
}
