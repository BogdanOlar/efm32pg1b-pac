#[doc = "Register `EXTIPSELH` reader"]
pub type R = crate::R<EXTIPSELHrs>;
#[doc = "Register `EXTIPSELH` writer"]
pub type W = crate::W<EXTIPSELHrs>;
#[doc = "External Interrupt 8 Port Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL8 {
    #[doc = "0: Port A group selected for external interrupt 8"]
    Porta = 0,
    #[doc = "1: Port B group selected for external interrupt 8"]
    Portb = 1,
    #[doc = "2: Port C group selected for external interrupt 8"]
    Portc = 2,
    #[doc = "3: Port D group selected for external interrupt 8"]
    Portd = 3,
    #[doc = "5: Port F group selected for external interrupt 8"]
    Portf = 5,
}
impl From<EXTIPSEL8> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL8 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPSEL8 {}
#[doc = "Field `EXTIPSEL8` reader - External Interrupt 8 Port Select"]
pub type Extipsel8R = crate::FieldReader<EXTIPSEL8>;
impl Extipsel8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL8> {
        match self.bits {
            0 => Some(EXTIPSEL8::Porta),
            1 => Some(EXTIPSEL8::Portb),
            2 => Some(EXTIPSEL8::Portc),
            3 => Some(EXTIPSEL8::Portd),
            5 => Some(EXTIPSEL8::Portf),
            _ => None,
        }
    }
    #[doc = "Port A group selected for external interrupt 8"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL8::Porta
    }
    #[doc = "Port B group selected for external interrupt 8"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL8::Portb
    }
    #[doc = "Port C group selected for external interrupt 8"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL8::Portc
    }
    #[doc = "Port D group selected for external interrupt 8"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL8::Portd
    }
    #[doc = "Port F group selected for external interrupt 8"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL8::Portf
    }
}
#[doc = "Field `EXTIPSEL8` writer - External Interrupt 8 Port Select"]
pub type Extipsel8W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL8>;
impl<'a, REG> Extipsel8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected for external interrupt 8"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL8::Porta)
    }
    #[doc = "Port B group selected for external interrupt 8"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL8::Portb)
    }
    #[doc = "Port C group selected for external interrupt 8"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL8::Portc)
    }
    #[doc = "Port D group selected for external interrupt 8"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL8::Portd)
    }
    #[doc = "Port F group selected for external interrupt 8"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL8::Portf)
    }
}
#[doc = "External Interrupt 9 Port Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL9 {
    #[doc = "0: Port A group selected for external interrupt 9"]
    Porta = 0,
    #[doc = "1: Port B group selected for external interrupt 9"]
    Portb = 1,
    #[doc = "2: Port C group selected for external interrupt 9"]
    Portc = 2,
    #[doc = "3: Port D group selected for external interrupt 9"]
    Portd = 3,
    #[doc = "5: Port F group selected for external interrupt 9"]
    Portf = 5,
}
impl From<EXTIPSEL9> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL9 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPSEL9 {}
#[doc = "Field `EXTIPSEL9` reader - External Interrupt 9 Port Select"]
pub type Extipsel9R = crate::FieldReader<EXTIPSEL9>;
impl Extipsel9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL9> {
        match self.bits {
            0 => Some(EXTIPSEL9::Porta),
            1 => Some(EXTIPSEL9::Portb),
            2 => Some(EXTIPSEL9::Portc),
            3 => Some(EXTIPSEL9::Portd),
            5 => Some(EXTIPSEL9::Portf),
            _ => None,
        }
    }
    #[doc = "Port A group selected for external interrupt 9"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL9::Porta
    }
    #[doc = "Port B group selected for external interrupt 9"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL9::Portb
    }
    #[doc = "Port C group selected for external interrupt 9"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL9::Portc
    }
    #[doc = "Port D group selected for external interrupt 9"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL9::Portd
    }
    #[doc = "Port F group selected for external interrupt 9"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL9::Portf
    }
}
#[doc = "Field `EXTIPSEL9` writer - External Interrupt 9 Port Select"]
pub type Extipsel9W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL9>;
impl<'a, REG> Extipsel9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected for external interrupt 9"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL9::Porta)
    }
    #[doc = "Port B group selected for external interrupt 9"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL9::Portb)
    }
    #[doc = "Port C group selected for external interrupt 9"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL9::Portc)
    }
    #[doc = "Port D group selected for external interrupt 9"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL9::Portd)
    }
    #[doc = "Port F group selected for external interrupt 9"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL9::Portf)
    }
}
#[doc = "External Interrupt 10 Port Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL10 {
    #[doc = "0: Port A group selected for external interrupt 10"]
    Porta = 0,
    #[doc = "1: Port B group selected for external interrupt 10"]
    Portb = 1,
    #[doc = "2: Port C group selected for external interrupt 10"]
    Portc = 2,
    #[doc = "3: Port D group selected for external interrupt 10"]
    Portd = 3,
    #[doc = "5: Port F group selected for external interrupt 10"]
    Portf = 5,
}
impl From<EXTIPSEL10> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL10 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPSEL10 {}
#[doc = "Field `EXTIPSEL10` reader - External Interrupt 10 Port Select"]
pub type Extipsel10R = crate::FieldReader<EXTIPSEL10>;
impl Extipsel10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL10> {
        match self.bits {
            0 => Some(EXTIPSEL10::Porta),
            1 => Some(EXTIPSEL10::Portb),
            2 => Some(EXTIPSEL10::Portc),
            3 => Some(EXTIPSEL10::Portd),
            5 => Some(EXTIPSEL10::Portf),
            _ => None,
        }
    }
    #[doc = "Port A group selected for external interrupt 10"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL10::Porta
    }
    #[doc = "Port B group selected for external interrupt 10"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL10::Portb
    }
    #[doc = "Port C group selected for external interrupt 10"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL10::Portc
    }
    #[doc = "Port D group selected for external interrupt 10"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL10::Portd
    }
    #[doc = "Port F group selected for external interrupt 10"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL10::Portf
    }
}
#[doc = "Field `EXTIPSEL10` writer - External Interrupt 10 Port Select"]
pub type Extipsel10W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL10>;
impl<'a, REG> Extipsel10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected for external interrupt 10"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL10::Porta)
    }
    #[doc = "Port B group selected for external interrupt 10"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL10::Portb)
    }
    #[doc = "Port C group selected for external interrupt 10"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL10::Portc)
    }
    #[doc = "Port D group selected for external interrupt 10"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL10::Portd)
    }
    #[doc = "Port F group selected for external interrupt 10"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL10::Portf)
    }
}
#[doc = "External Interrupt 11 Port Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL11 {
    #[doc = "0: Port A group selected for external interrupt 11"]
    Porta = 0,
    #[doc = "1: Port B group selected for external interrupt 11"]
    Portb = 1,
    #[doc = "2: Port C group selected for external interrupt 11"]
    Portc = 2,
    #[doc = "3: Port D group selected for external interrupt 11"]
    Portd = 3,
    #[doc = "5: Port F group selected for external interrupt 11"]
    Portf = 5,
}
impl From<EXTIPSEL11> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL11 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPSEL11 {}
#[doc = "Field `EXTIPSEL11` reader - External Interrupt 11 Port Select"]
pub type Extipsel11R = crate::FieldReader<EXTIPSEL11>;
impl Extipsel11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL11> {
        match self.bits {
            0 => Some(EXTIPSEL11::Porta),
            1 => Some(EXTIPSEL11::Portb),
            2 => Some(EXTIPSEL11::Portc),
            3 => Some(EXTIPSEL11::Portd),
            5 => Some(EXTIPSEL11::Portf),
            _ => None,
        }
    }
    #[doc = "Port A group selected for external interrupt 11"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL11::Porta
    }
    #[doc = "Port B group selected for external interrupt 11"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL11::Portb
    }
    #[doc = "Port C group selected for external interrupt 11"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL11::Portc
    }
    #[doc = "Port D group selected for external interrupt 11"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL11::Portd
    }
    #[doc = "Port F group selected for external interrupt 11"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL11::Portf
    }
}
#[doc = "Field `EXTIPSEL11` writer - External Interrupt 11 Port Select"]
pub type Extipsel11W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL11>;
impl<'a, REG> Extipsel11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected for external interrupt 11"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL11::Porta)
    }
    #[doc = "Port B group selected for external interrupt 11"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL11::Portb)
    }
    #[doc = "Port C group selected for external interrupt 11"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL11::Portc)
    }
    #[doc = "Port D group selected for external interrupt 11"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL11::Portd)
    }
    #[doc = "Port F group selected for external interrupt 11"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL11::Portf)
    }
}
#[doc = "External Interrupt 12 Port Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL12 {
    #[doc = "0: Port A group selected for external interrupt 12"]
    Porta = 0,
    #[doc = "1: Port B group selected for external interrupt 12"]
    Portb = 1,
    #[doc = "2: Port C group selected for external interrupt 12"]
    Portc = 2,
    #[doc = "3: Port D group selected for external interrupt 12"]
    Portd = 3,
    #[doc = "5: Port F group selected for external interrupt 12"]
    Portf = 5,
}
impl From<EXTIPSEL12> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL12 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPSEL12 {}
#[doc = "Field `EXTIPSEL12` reader - External Interrupt 12 Port Select"]
pub type Extipsel12R = crate::FieldReader<EXTIPSEL12>;
impl Extipsel12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL12> {
        match self.bits {
            0 => Some(EXTIPSEL12::Porta),
            1 => Some(EXTIPSEL12::Portb),
            2 => Some(EXTIPSEL12::Portc),
            3 => Some(EXTIPSEL12::Portd),
            5 => Some(EXTIPSEL12::Portf),
            _ => None,
        }
    }
    #[doc = "Port A group selected for external interrupt 12"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL12::Porta
    }
    #[doc = "Port B group selected for external interrupt 12"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL12::Portb
    }
    #[doc = "Port C group selected for external interrupt 12"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL12::Portc
    }
    #[doc = "Port D group selected for external interrupt 12"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL12::Portd
    }
    #[doc = "Port F group selected for external interrupt 12"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL12::Portf
    }
}
#[doc = "Field `EXTIPSEL12` writer - External Interrupt 12 Port Select"]
pub type Extipsel12W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL12>;
impl<'a, REG> Extipsel12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected for external interrupt 12"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL12::Porta)
    }
    #[doc = "Port B group selected for external interrupt 12"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL12::Portb)
    }
    #[doc = "Port C group selected for external interrupt 12"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL12::Portc)
    }
    #[doc = "Port D group selected for external interrupt 12"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL12::Portd)
    }
    #[doc = "Port F group selected for external interrupt 12"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL12::Portf)
    }
}
#[doc = "External Interrupt 13 Port Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL13 {
    #[doc = "0: Port A group selected for external interrupt 13"]
    Porta = 0,
    #[doc = "1: Port B group selected for external interrupt 13"]
    Portb = 1,
    #[doc = "2: Port C group selected for external interrupt 13"]
    Portc = 2,
    #[doc = "3: Port D group selected for external interrupt 13"]
    Portd = 3,
    #[doc = "5: Port F group selected for external interrupt 13"]
    Portf = 5,
}
impl From<EXTIPSEL13> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL13 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPSEL13 {}
#[doc = "Field `EXTIPSEL13` reader - External Interrupt 13 Port Select"]
pub type Extipsel13R = crate::FieldReader<EXTIPSEL13>;
impl Extipsel13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL13> {
        match self.bits {
            0 => Some(EXTIPSEL13::Porta),
            1 => Some(EXTIPSEL13::Portb),
            2 => Some(EXTIPSEL13::Portc),
            3 => Some(EXTIPSEL13::Portd),
            5 => Some(EXTIPSEL13::Portf),
            _ => None,
        }
    }
    #[doc = "Port A group selected for external interrupt 13"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL13::Porta
    }
    #[doc = "Port B group selected for external interrupt 13"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL13::Portb
    }
    #[doc = "Port C group selected for external interrupt 13"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL13::Portc
    }
    #[doc = "Port D group selected for external interrupt 13"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL13::Portd
    }
    #[doc = "Port F group selected for external interrupt 13"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL13::Portf
    }
}
#[doc = "Field `EXTIPSEL13` writer - External Interrupt 13 Port Select"]
pub type Extipsel13W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL13>;
impl<'a, REG> Extipsel13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected for external interrupt 13"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL13::Porta)
    }
    #[doc = "Port B group selected for external interrupt 13"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL13::Portb)
    }
    #[doc = "Port C group selected for external interrupt 13"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL13::Portc)
    }
    #[doc = "Port D group selected for external interrupt 13"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL13::Portd)
    }
    #[doc = "Port F group selected for external interrupt 13"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL13::Portf)
    }
}
#[doc = "External Interrupt 14 Port Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL14 {
    #[doc = "0: Port A group selected for external interrupt 14"]
    Porta = 0,
    #[doc = "1: Port B group selected for external interrupt 14"]
    Portb = 1,
    #[doc = "2: Port C group selected for external interrupt 14"]
    Portc = 2,
    #[doc = "3: Port D group selected for external interrupt 14"]
    Portd = 3,
    #[doc = "5: Port F group selected for external interrupt 14"]
    Portf = 5,
}
impl From<EXTIPSEL14> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL14 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPSEL14 {}
#[doc = "Field `EXTIPSEL14` reader - External Interrupt 14 Port Select"]
pub type Extipsel14R = crate::FieldReader<EXTIPSEL14>;
impl Extipsel14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL14> {
        match self.bits {
            0 => Some(EXTIPSEL14::Porta),
            1 => Some(EXTIPSEL14::Portb),
            2 => Some(EXTIPSEL14::Portc),
            3 => Some(EXTIPSEL14::Portd),
            5 => Some(EXTIPSEL14::Portf),
            _ => None,
        }
    }
    #[doc = "Port A group selected for external interrupt 14"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL14::Porta
    }
    #[doc = "Port B group selected for external interrupt 14"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL14::Portb
    }
    #[doc = "Port C group selected for external interrupt 14"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL14::Portc
    }
    #[doc = "Port D group selected for external interrupt 14"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL14::Portd
    }
    #[doc = "Port F group selected for external interrupt 14"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL14::Portf
    }
}
#[doc = "Field `EXTIPSEL14` writer - External Interrupt 14 Port Select"]
pub type Extipsel14W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL14>;
impl<'a, REG> Extipsel14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected for external interrupt 14"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL14::Porta)
    }
    #[doc = "Port B group selected for external interrupt 14"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL14::Portb)
    }
    #[doc = "Port C group selected for external interrupt 14"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL14::Portc)
    }
    #[doc = "Port D group selected for external interrupt 14"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL14::Portd)
    }
    #[doc = "Port F group selected for external interrupt 14"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL14::Portf)
    }
}
#[doc = "External Interrupt 15 Port Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL15 {
    #[doc = "0: Port A group selected for external interrupt 15"]
    Porta = 0,
    #[doc = "1: Port B group selected for external interrupt 15"]
    Portb = 1,
    #[doc = "2: Port C group selected for external interrupt 15"]
    Portc = 2,
    #[doc = "3: Port D group selected for external interrupt 15"]
    Portd = 3,
    #[doc = "5: Port F group selected for external interrupt 15"]
    Portf = 5,
}
impl From<EXTIPSEL15> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL15 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPSEL15 {}
#[doc = "Field `EXTIPSEL15` reader - External Interrupt 15 Port Select"]
pub type Extipsel15R = crate::FieldReader<EXTIPSEL15>;
impl Extipsel15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL15> {
        match self.bits {
            0 => Some(EXTIPSEL15::Porta),
            1 => Some(EXTIPSEL15::Portb),
            2 => Some(EXTIPSEL15::Portc),
            3 => Some(EXTIPSEL15::Portd),
            5 => Some(EXTIPSEL15::Portf),
            _ => None,
        }
    }
    #[doc = "Port A group selected for external interrupt 15"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL15::Porta
    }
    #[doc = "Port B group selected for external interrupt 15"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL15::Portb
    }
    #[doc = "Port C group selected for external interrupt 15"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL15::Portc
    }
    #[doc = "Port D group selected for external interrupt 15"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL15::Portd
    }
    #[doc = "Port F group selected for external interrupt 15"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL15::Portf
    }
}
#[doc = "Field `EXTIPSEL15` writer - External Interrupt 15 Port Select"]
pub type Extipsel15W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL15>;
impl<'a, REG> Extipsel15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected for external interrupt 15"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL15::Porta)
    }
    #[doc = "Port B group selected for external interrupt 15"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL15::Portb)
    }
    #[doc = "Port C group selected for external interrupt 15"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL15::Portc)
    }
    #[doc = "Port D group selected for external interrupt 15"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL15::Portd)
    }
    #[doc = "Port F group selected for external interrupt 15"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL15::Portf)
    }
}
impl R {
    #[doc = "Bits 0:3 - External Interrupt 8 Port Select"]
    #[inline(always)]
    pub fn extipsel8(&self) -> Extipsel8R {
        Extipsel8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External Interrupt 9 Port Select"]
    #[inline(always)]
    pub fn extipsel9(&self) -> Extipsel9R {
        Extipsel9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External Interrupt 10 Port Select"]
    #[inline(always)]
    pub fn extipsel10(&self) -> Extipsel10R {
        Extipsel10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External Interrupt 11 Port Select"]
    #[inline(always)]
    pub fn extipsel11(&self) -> Extipsel11R {
        Extipsel11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - External Interrupt 12 Port Select"]
    #[inline(always)]
    pub fn extipsel12(&self) -> Extipsel12R {
        Extipsel12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - External Interrupt 13 Port Select"]
    #[inline(always)]
    pub fn extipsel13(&self) -> Extipsel13R {
        Extipsel13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - External Interrupt 14 Port Select"]
    #[inline(always)]
    pub fn extipsel14(&self) -> Extipsel14R {
        Extipsel14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - External Interrupt 15 Port Select"]
    #[inline(always)]
    pub fn extipsel15(&self) -> Extipsel15R {
        Extipsel15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTIPSELH")
            .field("extipsel8", &self.extipsel8())
            .field("extipsel9", &self.extipsel9())
            .field("extipsel10", &self.extipsel10())
            .field("extipsel11", &self.extipsel11())
            .field("extipsel12", &self.extipsel12())
            .field("extipsel13", &self.extipsel13())
            .field("extipsel14", &self.extipsel14())
            .field("extipsel15", &self.extipsel15())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - External Interrupt 8 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel8(&mut self) -> Extipsel8W<EXTIPSELHrs> {
        Extipsel8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - External Interrupt 9 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel9(&mut self) -> Extipsel9W<EXTIPSELHrs> {
        Extipsel9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - External Interrupt 10 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel10(&mut self) -> Extipsel10W<EXTIPSELHrs> {
        Extipsel10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - External Interrupt 11 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel11(&mut self) -> Extipsel11W<EXTIPSELHrs> {
        Extipsel11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - External Interrupt 12 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel12(&mut self) -> Extipsel12W<EXTIPSELHrs> {
        Extipsel12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - External Interrupt 13 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel13(&mut self) -> Extipsel13W<EXTIPSELHrs> {
        Extipsel13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - External Interrupt 14 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel14(&mut self) -> Extipsel14W<EXTIPSELHrs> {
        Extipsel14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - External Interrupt 15 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel15(&mut self) -> Extipsel15W<EXTIPSELHrs> {
        Extipsel15W::new(self, 28)
    }
}
#[doc = "External Interrupt Port Select High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extipselh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipselh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIPSELHrs;
impl crate::RegisterSpec for EXTIPSELHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extipselh::R`](R) reader structure"]
impl crate::Readable for EXTIPSELHrs {}
#[doc = "`write(|w| ..)` method takes [`extipselh::W`](W) writer structure"]
impl crate::Writable for EXTIPSELHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTIPSELH to value 0"]
impl crate::Resettable for EXTIPSELHrs {
    const RESET_VALUE: u32 = 0;
}
