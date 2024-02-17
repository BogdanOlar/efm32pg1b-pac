#[doc = "Register `EXTIPSELL` reader"]
pub type R = crate::R<EXTIPSELLrs>;
#[doc = "Register `EXTIPSELL` writer"]
pub type W = crate::W<EXTIPSELLrs>;
#[doc = "Field `EXTIPSEL0` reader - External Interrupt 0 Port Select"]
pub type EXTIPSEL0_R = crate::FieldReader<EXTIPSEL0>;
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
impl EXTIPSEL0_R {
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
pub type EXTIPSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL0>;
impl<'a, REG> EXTIPSEL0_W<'a, REG>
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
#[doc = "Field `EXTIPSEL1` reader - External Interrupt 1 Port Select"]
pub type EXTIPSEL1_R = crate::FieldReader<EXTIPSEL1>;
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
impl EXTIPSEL1_R {
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
pub type EXTIPSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL1>;
impl<'a, REG> EXTIPSEL1_W<'a, REG>
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
#[doc = "Field `EXTIPSEL2` reader - External Interrupt 2 Port Select"]
pub type EXTIPSEL2_R = crate::FieldReader<EXTIPSEL2>;
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
impl EXTIPSEL2_R {
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
pub type EXTIPSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL2>;
impl<'a, REG> EXTIPSEL2_W<'a, REG>
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
#[doc = "Field `EXTIPSEL3` reader - External Interrupt 3 Port Select"]
pub type EXTIPSEL3_R = crate::FieldReader<EXTIPSEL3>;
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
impl EXTIPSEL3_R {
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
pub type EXTIPSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL3>;
impl<'a, REG> EXTIPSEL3_W<'a, REG>
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
#[doc = "Field `EXTIPSEL4` reader - External Interrupt 4 Port Select"]
pub type EXTIPSEL4_R = crate::FieldReader<EXTIPSEL4>;
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
impl EXTIPSEL4_R {
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
pub type EXTIPSEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL4>;
impl<'a, REG> EXTIPSEL4_W<'a, REG>
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
#[doc = "Field `EXTIPSEL5` reader - External Interrupt 5 Port Select"]
pub type EXTIPSEL5_R = crate::FieldReader<EXTIPSEL5>;
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
impl EXTIPSEL5_R {
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
pub type EXTIPSEL5_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL5>;
impl<'a, REG> EXTIPSEL5_W<'a, REG>
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
#[doc = "Field `EXTIPSEL6` reader - External Interrupt 6 Port Select"]
pub type EXTIPSEL6_R = crate::FieldReader<EXTIPSEL6>;
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
impl EXTIPSEL6_R {
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
pub type EXTIPSEL6_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL6>;
impl<'a, REG> EXTIPSEL6_W<'a, REG>
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
#[doc = "Field `EXTIPSEL7` reader - External Interrupt 7 Port Select"]
pub type EXTIPSEL7_R = crate::FieldReader<EXTIPSEL7>;
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
impl EXTIPSEL7_R {
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
pub type EXTIPSEL7_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTIPSEL7>;
impl<'a, REG> EXTIPSEL7_W<'a, REG>
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
    pub fn extipsel0(&self) -> EXTIPSEL0_R {
        EXTIPSEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External Interrupt 1 Port Select"]
    #[inline(always)]
    pub fn extipsel1(&self) -> EXTIPSEL1_R {
        EXTIPSEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External Interrupt 2 Port Select"]
    #[inline(always)]
    pub fn extipsel2(&self) -> EXTIPSEL2_R {
        EXTIPSEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External Interrupt 3 Port Select"]
    #[inline(always)]
    pub fn extipsel3(&self) -> EXTIPSEL3_R {
        EXTIPSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - External Interrupt 4 Port Select"]
    #[inline(always)]
    pub fn extipsel4(&self) -> EXTIPSEL4_R {
        EXTIPSEL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - External Interrupt 5 Port Select"]
    #[inline(always)]
    pub fn extipsel5(&self) -> EXTIPSEL5_R {
        EXTIPSEL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - External Interrupt 6 Port Select"]
    #[inline(always)]
    pub fn extipsel6(&self) -> EXTIPSEL6_R {
        EXTIPSEL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - External Interrupt 7 Port Select"]
    #[inline(always)]
    pub fn extipsel7(&self) -> EXTIPSEL7_R {
        EXTIPSEL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - External Interrupt 0 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel0(&mut self) -> EXTIPSEL0_W<EXTIPSELLrs> {
        EXTIPSEL0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - External Interrupt 1 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel1(&mut self) -> EXTIPSEL1_W<EXTIPSELLrs> {
        EXTIPSEL1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - External Interrupt 2 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel2(&mut self) -> EXTIPSEL2_W<EXTIPSELLrs> {
        EXTIPSEL2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - External Interrupt 3 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel3(&mut self) -> EXTIPSEL3_W<EXTIPSELLrs> {
        EXTIPSEL3_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - External Interrupt 4 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel4(&mut self) -> EXTIPSEL4_W<EXTIPSELLrs> {
        EXTIPSEL4_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - External Interrupt 5 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel5(&mut self) -> EXTIPSEL5_W<EXTIPSELLrs> {
        EXTIPSEL5_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - External Interrupt 6 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel6(&mut self) -> EXTIPSEL6_W<EXTIPSELLrs> {
        EXTIPSEL6_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - External Interrupt 7 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel7(&mut self) -> EXTIPSEL7_W<EXTIPSELLrs> {
        EXTIPSEL7_W::new(self, 28)
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
#[doc = "External Interrupt Port Select Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extipsell::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extipsell::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIPSELLrs;
impl crate::RegisterSpec for EXTIPSELLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extipsell::R`](R) reader structure"]
impl crate::Readable for EXTIPSELLrs {}
#[doc = "`write(|w| ..)` method takes [`extipsell::W`](W) writer structure"]
impl crate::Writable for EXTIPSELLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTIPSELL to value 0"]
impl crate::Resettable for EXTIPSELLrs {
    const RESET_VALUE: u32 = 0;
}
