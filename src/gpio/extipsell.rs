#[doc = "Register `EXTIPSELL` reader"]
pub type R = crate::R<EXTIPSELLrs>;
#[doc = "Register `EXTIPSELL` writer"]
pub type W = crate::W<EXTIPSELLrs>;
#[doc = "External Interrupt 0 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL0 {
    #[doc = "0: Port A group selected for external interrupt 0"]
    Porta = 0,
    #[doc = "1: Port B group selected for external interrupt 0"]
    Portb = 1,
    #[doc = "2: Port C group selected for external interrupt 0"]
    Portc = 2,
    #[doc = "3: Port D group selected for external interrupt 0"]
    Portd = 3,
    #[doc = "5: Port F group selected for external interrupt 0"]
    Portf = 5,
}
impl From<EXTIPSEL0> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL0 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPSEL0 {}
#[doc = "Field `EXTIPSEL0` reader - External Interrupt 0 Port Select"]
pub type Extipsel0R = crate::FieldReader<EXTIPSEL0>;
impl Extipsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL0> {
        match self.bits {
            0 => Some(EXTIPSEL0::Porta),
            1 => Some(EXTIPSEL0::Portb),
            2 => Some(EXTIPSEL0::Portc),
            3 => Some(EXTIPSEL0::Portd),
            5 => Some(EXTIPSEL0::Portf),
            _ => None,
        }
    }
    #[doc = "Port A group selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL0::Porta
    }
    #[doc = "Port B group selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL0::Portb
    }
    #[doc = "Port C group selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL0::Portc
    }
    #[doc = "Port D group selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL0::Portd
    }
    #[doc = "Port F group selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL0::Portf
    }
}
#[doc = "Field `EXTIPSEL0` writer - External Interrupt 0 Port Select"]
pub type Extipsel0W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL0>;
impl<'a, REG> Extipsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected for external interrupt 0"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL0::Porta)
    }
    #[doc = "Port B group selected for external interrupt 0"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL0::Portb)
    }
    #[doc = "Port C group selected for external interrupt 0"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL0::Portc)
    }
    #[doc = "Port D group selected for external interrupt 0"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL0::Portd)
    }
    #[doc = "Port F group selected for external interrupt 0"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL0::Portf)
    }
}
#[doc = "External Interrupt 1 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL1 {
    #[doc = "0: Port A group selected for external interrupt 1"]
    Porta = 0,
    #[doc = "1: Port B group selected for external interrupt 1"]
    Portb = 1,
    #[doc = "2: Port C group selected for external interrupt 1"]
    Portc = 2,
    #[doc = "3: Port D group selected for external interrupt 1"]
    Portd = 3,
    #[doc = "5: Port F group selected for external interrupt 1"]
    Portf = 5,
}
impl From<EXTIPSEL1> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL1 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPSEL1 {}
#[doc = "Field `EXTIPSEL1` reader - External Interrupt 1 Port Select"]
pub type Extipsel1R = crate::FieldReader<EXTIPSEL1>;
impl Extipsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL1> {
        match self.bits {
            0 => Some(EXTIPSEL1::Porta),
            1 => Some(EXTIPSEL1::Portb),
            2 => Some(EXTIPSEL1::Portc),
            3 => Some(EXTIPSEL1::Portd),
            5 => Some(EXTIPSEL1::Portf),
            _ => None,
        }
    }
    #[doc = "Port A group selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL1::Porta
    }
    #[doc = "Port B group selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL1::Portb
    }
    #[doc = "Port C group selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL1::Portc
    }
    #[doc = "Port D group selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL1::Portd
    }
    #[doc = "Port F group selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL1::Portf
    }
}
#[doc = "Field `EXTIPSEL1` writer - External Interrupt 1 Port Select"]
pub type Extipsel1W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL1>;
impl<'a, REG> Extipsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected for external interrupt 1"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL1::Porta)
    }
    #[doc = "Port B group selected for external interrupt 1"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL1::Portb)
    }
    #[doc = "Port C group selected for external interrupt 1"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL1::Portc)
    }
    #[doc = "Port D group selected for external interrupt 1"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL1::Portd)
    }
    #[doc = "Port F group selected for external interrupt 1"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL1::Portf)
    }
}
#[doc = "External Interrupt 2 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL2 {
    #[doc = "0: Port A group selected for external interrupt 2"]
    Porta = 0,
    #[doc = "1: Port B group selected for external interrupt 2"]
    Portb = 1,
    #[doc = "2: Port C group selected for external interrupt 2"]
    Portc = 2,
    #[doc = "3: Port D group selected for external interrupt 2"]
    Portd = 3,
    #[doc = "5: Port F group selected for external interrupt 2"]
    Portf = 5,
}
impl From<EXTIPSEL2> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL2 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPSEL2 {}
#[doc = "Field `EXTIPSEL2` reader - External Interrupt 2 Port Select"]
pub type Extipsel2R = crate::FieldReader<EXTIPSEL2>;
impl Extipsel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL2> {
        match self.bits {
            0 => Some(EXTIPSEL2::Porta),
            1 => Some(EXTIPSEL2::Portb),
            2 => Some(EXTIPSEL2::Portc),
            3 => Some(EXTIPSEL2::Portd),
            5 => Some(EXTIPSEL2::Portf),
            _ => None,
        }
    }
    #[doc = "Port A group selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL2::Porta
    }
    #[doc = "Port B group selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL2::Portb
    }
    #[doc = "Port C group selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL2::Portc
    }
    #[doc = "Port D group selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL2::Portd
    }
    #[doc = "Port F group selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL2::Portf
    }
}
#[doc = "Field `EXTIPSEL2` writer - External Interrupt 2 Port Select"]
pub type Extipsel2W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL2>;
impl<'a, REG> Extipsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected for external interrupt 2"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL2::Porta)
    }
    #[doc = "Port B group selected for external interrupt 2"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL2::Portb)
    }
    #[doc = "Port C group selected for external interrupt 2"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL2::Portc)
    }
    #[doc = "Port D group selected for external interrupt 2"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL2::Portd)
    }
    #[doc = "Port F group selected for external interrupt 2"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL2::Portf)
    }
}
#[doc = "External Interrupt 3 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL3 {
    #[doc = "0: Port A group selected for external interrupt 3"]
    Porta = 0,
    #[doc = "1: Port B group selected for external interrupt 3"]
    Portb = 1,
    #[doc = "2: Port C group selected for external interrupt 3"]
    Portc = 2,
    #[doc = "3: Port D group selected for external interrupt 3"]
    Portd = 3,
    #[doc = "5: Port F group selected for external interrupt 3"]
    Portf = 5,
}
impl From<EXTIPSEL3> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL3 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPSEL3 {}
#[doc = "Field `EXTIPSEL3` reader - External Interrupt 3 Port Select"]
pub type Extipsel3R = crate::FieldReader<EXTIPSEL3>;
impl Extipsel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL3> {
        match self.bits {
            0 => Some(EXTIPSEL3::Porta),
            1 => Some(EXTIPSEL3::Portb),
            2 => Some(EXTIPSEL3::Portc),
            3 => Some(EXTIPSEL3::Portd),
            5 => Some(EXTIPSEL3::Portf),
            _ => None,
        }
    }
    #[doc = "Port A group selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL3::Porta
    }
    #[doc = "Port B group selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL3::Portb
    }
    #[doc = "Port C group selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL3::Portc
    }
    #[doc = "Port D group selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL3::Portd
    }
    #[doc = "Port F group selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL3::Portf
    }
}
#[doc = "Field `EXTIPSEL3` writer - External Interrupt 3 Port Select"]
pub type Extipsel3W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL3>;
impl<'a, REG> Extipsel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected for external interrupt 3"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL3::Porta)
    }
    #[doc = "Port B group selected for external interrupt 3"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL3::Portb)
    }
    #[doc = "Port C group selected for external interrupt 3"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL3::Portc)
    }
    #[doc = "Port D group selected for external interrupt 3"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL3::Portd)
    }
    #[doc = "Port F group selected for external interrupt 3"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL3::Portf)
    }
}
#[doc = "External Interrupt 4 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL4 {
    #[doc = "0: Port A group selected for external interrupt 4"]
    Porta = 0,
    #[doc = "1: Port B group selected for external interrupt 4"]
    Portb = 1,
    #[doc = "2: Port C group selected for external interrupt 4"]
    Portc = 2,
    #[doc = "3: Port D group selected for external interrupt 4"]
    Portd = 3,
    #[doc = "5: Port F group selected for external interrupt 4"]
    Portf = 5,
}
impl From<EXTIPSEL4> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL4 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPSEL4 {}
#[doc = "Field `EXTIPSEL4` reader - External Interrupt 4 Port Select"]
pub type Extipsel4R = crate::FieldReader<EXTIPSEL4>;
impl Extipsel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL4> {
        match self.bits {
            0 => Some(EXTIPSEL4::Porta),
            1 => Some(EXTIPSEL4::Portb),
            2 => Some(EXTIPSEL4::Portc),
            3 => Some(EXTIPSEL4::Portd),
            5 => Some(EXTIPSEL4::Portf),
            _ => None,
        }
    }
    #[doc = "Port A group selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL4::Porta
    }
    #[doc = "Port B group selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL4::Portb
    }
    #[doc = "Port C group selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL4::Portc
    }
    #[doc = "Port D group selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL4::Portd
    }
    #[doc = "Port F group selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL4::Portf
    }
}
#[doc = "Field `EXTIPSEL4` writer - External Interrupt 4 Port Select"]
pub type Extipsel4W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL4>;
impl<'a, REG> Extipsel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected for external interrupt 4"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL4::Porta)
    }
    #[doc = "Port B group selected for external interrupt 4"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL4::Portb)
    }
    #[doc = "Port C group selected for external interrupt 4"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL4::Portc)
    }
    #[doc = "Port D group selected for external interrupt 4"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL4::Portd)
    }
    #[doc = "Port F group selected for external interrupt 4"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL4::Portf)
    }
}
#[doc = "External Interrupt 5 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL5 {
    #[doc = "0: Port A group selected for external interrupt 5"]
    Porta = 0,
    #[doc = "1: Port B group selected for external interrupt 5"]
    Portb = 1,
    #[doc = "2: Port C group selected for external interrupt 5"]
    Portc = 2,
    #[doc = "3: Port D group selected for external interrupt 5"]
    Portd = 3,
    #[doc = "5: Port F group selected for external interrupt 5"]
    Portf = 5,
}
impl From<EXTIPSEL5> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL5 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPSEL5 {}
#[doc = "Field `EXTIPSEL5` reader - External Interrupt 5 Port Select"]
pub type Extipsel5R = crate::FieldReader<EXTIPSEL5>;
impl Extipsel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL5> {
        match self.bits {
            0 => Some(EXTIPSEL5::Porta),
            1 => Some(EXTIPSEL5::Portb),
            2 => Some(EXTIPSEL5::Portc),
            3 => Some(EXTIPSEL5::Portd),
            5 => Some(EXTIPSEL5::Portf),
            _ => None,
        }
    }
    #[doc = "Port A group selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL5::Porta
    }
    #[doc = "Port B group selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL5::Portb
    }
    #[doc = "Port C group selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL5::Portc
    }
    #[doc = "Port D group selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL5::Portd
    }
    #[doc = "Port F group selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL5::Portf
    }
}
#[doc = "Field `EXTIPSEL5` writer - External Interrupt 5 Port Select"]
pub type Extipsel5W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL5>;
impl<'a, REG> Extipsel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected for external interrupt 5"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL5::Porta)
    }
    #[doc = "Port B group selected for external interrupt 5"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL5::Portb)
    }
    #[doc = "Port C group selected for external interrupt 5"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL5::Portc)
    }
    #[doc = "Port D group selected for external interrupt 5"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL5::Portd)
    }
    #[doc = "Port F group selected for external interrupt 5"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL5::Portf)
    }
}
#[doc = "External Interrupt 6 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL6 {
    #[doc = "0: Port A group selected for external interrupt 6"]
    Porta = 0,
    #[doc = "1: Port B group selected for external interrupt 6"]
    Portb = 1,
    #[doc = "2: Port C group selected for external interrupt 6"]
    Portc = 2,
    #[doc = "3: Port D group selected for external interrupt 6"]
    Portd = 3,
    #[doc = "5: Port F group selected for external interrupt 6"]
    Portf = 5,
}
impl From<EXTIPSEL6> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL6 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPSEL6 {}
#[doc = "Field `EXTIPSEL6` reader - External Interrupt 6 Port Select"]
pub type Extipsel6R = crate::FieldReader<EXTIPSEL6>;
impl Extipsel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL6> {
        match self.bits {
            0 => Some(EXTIPSEL6::Porta),
            1 => Some(EXTIPSEL6::Portb),
            2 => Some(EXTIPSEL6::Portc),
            3 => Some(EXTIPSEL6::Portd),
            5 => Some(EXTIPSEL6::Portf),
            _ => None,
        }
    }
    #[doc = "Port A group selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL6::Porta
    }
    #[doc = "Port B group selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL6::Portb
    }
    #[doc = "Port C group selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL6::Portc
    }
    #[doc = "Port D group selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL6::Portd
    }
    #[doc = "Port F group selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL6::Portf
    }
}
#[doc = "Field `EXTIPSEL6` writer - External Interrupt 6 Port Select"]
pub type Extipsel6W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL6>;
impl<'a, REG> Extipsel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected for external interrupt 6"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL6::Porta)
    }
    #[doc = "Port B group selected for external interrupt 6"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL6::Portb)
    }
    #[doc = "Port C group selected for external interrupt 6"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL6::Portc)
    }
    #[doc = "Port D group selected for external interrupt 6"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL6::Portd)
    }
    #[doc = "Port F group selected for external interrupt 6"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL6::Portf)
    }
}
#[doc = "External Interrupt 7 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL7 {
    #[doc = "0: Port A group selected for external interrupt 7"]
    Porta = 0,
    #[doc = "1: Port B group selected for external interrupt 7"]
    Portb = 1,
    #[doc = "2: Port C group selected for external interrupt 7"]
    Portc = 2,
    #[doc = "3: Port D group selected for external interrupt 7"]
    Portd = 3,
    #[doc = "5: Port F group selected for external interrupt 7"]
    Portf = 5,
}
impl From<EXTIPSEL7> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL7 {
    type Ux = u8;
}
impl crate::IsEnum for EXTIPSEL7 {}
#[doc = "Field `EXTIPSEL7` reader - External Interrupt 7 Port Select"]
pub type Extipsel7R = crate::FieldReader<EXTIPSEL7>;
impl Extipsel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL7> {
        match self.bits {
            0 => Some(EXTIPSEL7::Porta),
            1 => Some(EXTIPSEL7::Portb),
            2 => Some(EXTIPSEL7::Portc),
            3 => Some(EXTIPSEL7::Portd),
            5 => Some(EXTIPSEL7::Portf),
            _ => None,
        }
    }
    #[doc = "Port A group selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL7::Porta
    }
    #[doc = "Port B group selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL7::Portb
    }
    #[doc = "Port C group selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL7::Portc
    }
    #[doc = "Port D group selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL7::Portd
    }
    #[doc = "Port F group selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL7::Portf
    }
}
#[doc = "Field `EXTIPSEL7` writer - External Interrupt 7 Port Select"]
pub type Extipsel7W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL7>;
impl<'a, REG> Extipsel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected for external interrupt 7"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL7::Porta)
    }
    #[doc = "Port B group selected for external interrupt 7"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL7::Portb)
    }
    #[doc = "Port C group selected for external interrupt 7"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL7::Portc)
    }
    #[doc = "Port D group selected for external interrupt 7"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL7::Portd)
    }
    #[doc = "Port F group selected for external interrupt 7"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL7::Portf)
    }
}
impl R {
    #[doc = "Bits 0:3 - External Interrupt 0 Port Select"]
    #[inline(always)]
    pub fn extipsel0(&self) -> Extipsel0R {
        Extipsel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External Interrupt 1 Port Select"]
    #[inline(always)]
    pub fn extipsel1(&self) -> Extipsel1R {
        Extipsel1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External Interrupt 2 Port Select"]
    #[inline(always)]
    pub fn extipsel2(&self) -> Extipsel2R {
        Extipsel2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External Interrupt 3 Port Select"]
    #[inline(always)]
    pub fn extipsel3(&self) -> Extipsel3R {
        Extipsel3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - External Interrupt 4 Port Select"]
    #[inline(always)]
    pub fn extipsel4(&self) -> Extipsel4R {
        Extipsel4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - External Interrupt 5 Port Select"]
    #[inline(always)]
    pub fn extipsel5(&self) -> Extipsel5R {
        Extipsel5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - External Interrupt 6 Port Select"]
    #[inline(always)]
    pub fn extipsel6(&self) -> Extipsel6R {
        Extipsel6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - External Interrupt 7 Port Select"]
    #[inline(always)]
    pub fn extipsel7(&self) -> Extipsel7R {
        Extipsel7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - External Interrupt 0 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel0(&mut self) -> Extipsel0W<EXTIPSELLrs> {
        Extipsel0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - External Interrupt 1 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel1(&mut self) -> Extipsel1W<EXTIPSELLrs> {
        Extipsel1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - External Interrupt 2 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel2(&mut self) -> Extipsel2W<EXTIPSELLrs> {
        Extipsel2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - External Interrupt 3 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel3(&mut self) -> Extipsel3W<EXTIPSELLrs> {
        Extipsel3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - External Interrupt 4 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel4(&mut self) -> Extipsel4W<EXTIPSELLrs> {
        Extipsel4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - External Interrupt 5 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel5(&mut self) -> Extipsel5W<EXTIPSELLrs> {
        Extipsel5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - External Interrupt 6 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel6(&mut self) -> Extipsel6W<EXTIPSELLrs> {
        Extipsel6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - External Interrupt 7 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel7(&mut self) -> Extipsel7W<EXTIPSELLrs> {
        Extipsel7W::new(self, 28)
    }
}
#[doc = "External Interrupt Port Select Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extipsell::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extipsell::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIPSELLrs;
impl crate::RegisterSpec for EXTIPSELLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extipsell::R`](R) reader structure"]
impl crate::Readable for EXTIPSELLrs {}
#[doc = "`write(|w| ..)` method takes [`extipsell::W`](W) writer structure"]
impl crate::Writable for EXTIPSELLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTIPSELL to value 0"]
impl crate::Resettable for EXTIPSELLrs {
    const RESET_VALUE: u32 = 0;
}
