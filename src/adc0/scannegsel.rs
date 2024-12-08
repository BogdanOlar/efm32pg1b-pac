///Register `SCANNEGSEL` reader
pub type R = crate::R<SCANNEGSELrs>;
///Register `SCANNEGSEL` writer
pub type W = crate::W<SCANNEGSELrs>;
///Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUT0NEGSEL {
    ///0: Selects ADCn_INPUT1 as negative channel input
    Input1 = 0,
    ///1: Selects ADCn_INPUT3 as negative channel input
    Input3 = 1,
    ///2: Selects ADCn_INPUT5 as negative channel input
    Input5 = 2,
    ///3: Selects ADCn_INPUT7 as negative channel input
    Input7 = 3,
}
impl From<INPUT0NEGSEL> for u8 {
    #[inline(always)]
    fn from(variant: INPUT0NEGSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPUT0NEGSEL {
    type Ux = u8;
}
impl crate::IsEnum for INPUT0NEGSEL {}
///Field `INPUT0NEGSEL` reader - Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode
pub type Input0negselR = crate::FieldReader<INPUT0NEGSEL>;
impl Input0negselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INPUT0NEGSEL {
        match self.bits {
            0 => INPUT0NEGSEL::Input1,
            1 => INPUT0NEGSEL::Input3,
            2 => INPUT0NEGSEL::Input5,
            3 => INPUT0NEGSEL::Input7,
            _ => unreachable!(),
        }
    }
    ///Selects ADCn_INPUT1 as negative channel input
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == INPUT0NEGSEL::Input1
    }
    ///Selects ADCn_INPUT3 as negative channel input
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == INPUT0NEGSEL::Input3
    }
    ///Selects ADCn_INPUT5 as negative channel input
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == INPUT0NEGSEL::Input5
    }
    ///Selects ADCn_INPUT7 as negative channel input
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == INPUT0NEGSEL::Input7
    }
}
///Field `INPUT0NEGSEL` writer - Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode
pub type Input0negselW<'a, REG> = crate::FieldWriter<'a, REG, 2, INPUT0NEGSEL, crate::Safe>;
impl<'a, REG> Input0negselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Selects ADCn_INPUT1 as negative channel input
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0NEGSEL::Input1)
    }
    ///Selects ADCn_INPUT3 as negative channel input
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0NEGSEL::Input3)
    }
    ///Selects ADCn_INPUT5 as negative channel input
    #[inline(always)]
    pub fn input5(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0NEGSEL::Input5)
    }
    ///Selects ADCn_INPUT7 as negative channel input
    #[inline(always)]
    pub fn input7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0NEGSEL::Input7)
    }
}
///Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode
///
///Value on reset: 1
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUT2NEGSEL {
    ///0: Selects ADCn_INPUT1 as negative channel input
    Input1 = 0,
    ///1: Selects ADCn_INPUT3 as negative channel input
    Input3 = 1,
    ///2: Selects ADCn_INPUT5 as negative channel input
    Input5 = 2,
    ///3: Selects ADCn_INPUT7 as negative channel input
    Input7 = 3,
}
impl From<INPUT2NEGSEL> for u8 {
    #[inline(always)]
    fn from(variant: INPUT2NEGSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPUT2NEGSEL {
    type Ux = u8;
}
impl crate::IsEnum for INPUT2NEGSEL {}
///Field `INPUT2NEGSEL` reader - Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode
pub type Input2negselR = crate::FieldReader<INPUT2NEGSEL>;
impl Input2negselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INPUT2NEGSEL {
        match self.bits {
            0 => INPUT2NEGSEL::Input1,
            1 => INPUT2NEGSEL::Input3,
            2 => INPUT2NEGSEL::Input5,
            3 => INPUT2NEGSEL::Input7,
            _ => unreachable!(),
        }
    }
    ///Selects ADCn_INPUT1 as negative channel input
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == INPUT2NEGSEL::Input1
    }
    ///Selects ADCn_INPUT3 as negative channel input
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == INPUT2NEGSEL::Input3
    }
    ///Selects ADCn_INPUT5 as negative channel input
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == INPUT2NEGSEL::Input5
    }
    ///Selects ADCn_INPUT7 as negative channel input
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == INPUT2NEGSEL::Input7
    }
}
///Field `INPUT2NEGSEL` writer - Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode
pub type Input2negselW<'a, REG> = crate::FieldWriter<'a, REG, 2, INPUT2NEGSEL, crate::Safe>;
impl<'a, REG> Input2negselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Selects ADCn_INPUT1 as negative channel input
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT2NEGSEL::Input1)
    }
    ///Selects ADCn_INPUT3 as negative channel input
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT2NEGSEL::Input3)
    }
    ///Selects ADCn_INPUT5 as negative channel input
    #[inline(always)]
    pub fn input5(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT2NEGSEL::Input5)
    }
    ///Selects ADCn_INPUT7 as negative channel input
    #[inline(always)]
    pub fn input7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT2NEGSEL::Input7)
    }
}
///Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode
///
///Value on reset: 2
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUT4NEGSEL {
    ///0: Selects ADCn_INPUT1 as negative channel input
    Input1 = 0,
    ///1: Selects ADCn_INPUT3 as negative channel input
    Input3 = 1,
    ///2: Selects ADCn_INPUT5 as negative channel input
    Input5 = 2,
    ///3: Selects ADCn_INPUT7 as negative channel input
    Input7 = 3,
}
impl From<INPUT4NEGSEL> for u8 {
    #[inline(always)]
    fn from(variant: INPUT4NEGSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPUT4NEGSEL {
    type Ux = u8;
}
impl crate::IsEnum for INPUT4NEGSEL {}
///Field `INPUT4NEGSEL` reader - Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode
pub type Input4negselR = crate::FieldReader<INPUT4NEGSEL>;
impl Input4negselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INPUT4NEGSEL {
        match self.bits {
            0 => INPUT4NEGSEL::Input1,
            1 => INPUT4NEGSEL::Input3,
            2 => INPUT4NEGSEL::Input5,
            3 => INPUT4NEGSEL::Input7,
            _ => unreachable!(),
        }
    }
    ///Selects ADCn_INPUT1 as negative channel input
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == INPUT4NEGSEL::Input1
    }
    ///Selects ADCn_INPUT3 as negative channel input
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == INPUT4NEGSEL::Input3
    }
    ///Selects ADCn_INPUT5 as negative channel input
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == INPUT4NEGSEL::Input5
    }
    ///Selects ADCn_INPUT7 as negative channel input
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == INPUT4NEGSEL::Input7
    }
}
///Field `INPUT4NEGSEL` writer - Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode
pub type Input4negselW<'a, REG> = crate::FieldWriter<'a, REG, 2, INPUT4NEGSEL, crate::Safe>;
impl<'a, REG> Input4negselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Selects ADCn_INPUT1 as negative channel input
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT4NEGSEL::Input1)
    }
    ///Selects ADCn_INPUT3 as negative channel input
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT4NEGSEL::Input3)
    }
    ///Selects ADCn_INPUT5 as negative channel input
    #[inline(always)]
    pub fn input5(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT4NEGSEL::Input5)
    }
    ///Selects ADCn_INPUT7 as negative channel input
    #[inline(always)]
    pub fn input7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT4NEGSEL::Input7)
    }
}
///Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode
///
///Value on reset: 3
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUT6NEGSEL {
    ///0: Selects ADCn_INPUT1 as negative channel input
    Input1 = 0,
    ///1: Selects ADCn_INPUT3 as negative channel input
    Input3 = 1,
    ///2: Selects ADCn_INPUT5 as negative channel input
    Input5 = 2,
    ///3: Selects ADCn_INPUT7 as negative channel input
    Input7 = 3,
}
impl From<INPUT6NEGSEL> for u8 {
    #[inline(always)]
    fn from(variant: INPUT6NEGSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPUT6NEGSEL {
    type Ux = u8;
}
impl crate::IsEnum for INPUT6NEGSEL {}
///Field `INPUT6NEGSEL` reader - Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode
pub type Input6negselR = crate::FieldReader<INPUT6NEGSEL>;
impl Input6negselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INPUT6NEGSEL {
        match self.bits {
            0 => INPUT6NEGSEL::Input1,
            1 => INPUT6NEGSEL::Input3,
            2 => INPUT6NEGSEL::Input5,
            3 => INPUT6NEGSEL::Input7,
            _ => unreachable!(),
        }
    }
    ///Selects ADCn_INPUT1 as negative channel input
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == INPUT6NEGSEL::Input1
    }
    ///Selects ADCn_INPUT3 as negative channel input
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == INPUT6NEGSEL::Input3
    }
    ///Selects ADCn_INPUT5 as negative channel input
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == INPUT6NEGSEL::Input5
    }
    ///Selects ADCn_INPUT7 as negative channel input
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == INPUT6NEGSEL::Input7
    }
}
///Field `INPUT6NEGSEL` writer - Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode
pub type Input6negselW<'a, REG> = crate::FieldWriter<'a, REG, 2, INPUT6NEGSEL, crate::Safe>;
impl<'a, REG> Input6negselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Selects ADCn_INPUT1 as negative channel input
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT6NEGSEL::Input1)
    }
    ///Selects ADCn_INPUT3 as negative channel input
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT6NEGSEL::Input3)
    }
    ///Selects ADCn_INPUT5 as negative channel input
    #[inline(always)]
    pub fn input5(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT6NEGSEL::Input5)
    }
    ///Selects ADCn_INPUT7 as negative channel input
    #[inline(always)]
    pub fn input7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT6NEGSEL::Input7)
    }
}
///Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode
///
///Value on reset: 1
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUT9NEGSEL {
    ///0: Selects ADCn_INPUT8 as negative channel input
    Input8 = 0,
    ///1: Selects ADCn_INPUT10 as negative channel input
    Input10 = 1,
    ///2: Selects ADCn_INPUT12 as negative channel input
    Input12 = 2,
    ///3: Selects ADCn_INPUT14 as negative channel input
    Input14 = 3,
}
impl From<INPUT9NEGSEL> for u8 {
    #[inline(always)]
    fn from(variant: INPUT9NEGSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPUT9NEGSEL {
    type Ux = u8;
}
impl crate::IsEnum for INPUT9NEGSEL {}
///Field `INPUT9NEGSEL` reader - Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode
pub type Input9negselR = crate::FieldReader<INPUT9NEGSEL>;
impl Input9negselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INPUT9NEGSEL {
        match self.bits {
            0 => INPUT9NEGSEL::Input8,
            1 => INPUT9NEGSEL::Input10,
            2 => INPUT9NEGSEL::Input12,
            3 => INPUT9NEGSEL::Input14,
            _ => unreachable!(),
        }
    }
    ///Selects ADCn_INPUT8 as negative channel input
    #[inline(always)]
    pub fn is_input8(&self) -> bool {
        *self == INPUT9NEGSEL::Input8
    }
    ///Selects ADCn_INPUT10 as negative channel input
    #[inline(always)]
    pub fn is_input10(&self) -> bool {
        *self == INPUT9NEGSEL::Input10
    }
    ///Selects ADCn_INPUT12 as negative channel input
    #[inline(always)]
    pub fn is_input12(&self) -> bool {
        *self == INPUT9NEGSEL::Input12
    }
    ///Selects ADCn_INPUT14 as negative channel input
    #[inline(always)]
    pub fn is_input14(&self) -> bool {
        *self == INPUT9NEGSEL::Input14
    }
}
///Field `INPUT9NEGSEL` writer - Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode
pub type Input9negselW<'a, REG> = crate::FieldWriter<'a, REG, 2, INPUT9NEGSEL, crate::Safe>;
impl<'a, REG> Input9negselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Selects ADCn_INPUT8 as negative channel input
    #[inline(always)]
    pub fn input8(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT9NEGSEL::Input8)
    }
    ///Selects ADCn_INPUT10 as negative channel input
    #[inline(always)]
    pub fn input10(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT9NEGSEL::Input10)
    }
    ///Selects ADCn_INPUT12 as negative channel input
    #[inline(always)]
    pub fn input12(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT9NEGSEL::Input12)
    }
    ///Selects ADCn_INPUT14 as negative channel input
    #[inline(always)]
    pub fn input14(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT9NEGSEL::Input14)
    }
}
///Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode
///
///Value on reset: 2
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUT11NEGSEL {
    ///0: Selects ADCn_INPUT8 as negative channel input
    Input8 = 0,
    ///1: Selects ADCn_INPUT10 as negative channel input
    Input10 = 1,
    ///2: Selects ADCn_INPUT12 as negative channel input
    Input12 = 2,
    ///3: Selects ADCn_INPUT14 as negative channel input
    Input14 = 3,
}
impl From<INPUT11NEGSEL> for u8 {
    #[inline(always)]
    fn from(variant: INPUT11NEGSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPUT11NEGSEL {
    type Ux = u8;
}
impl crate::IsEnum for INPUT11NEGSEL {}
///Field `INPUT11NEGSEL` reader - Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode
pub type Input11negselR = crate::FieldReader<INPUT11NEGSEL>;
impl Input11negselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INPUT11NEGSEL {
        match self.bits {
            0 => INPUT11NEGSEL::Input8,
            1 => INPUT11NEGSEL::Input10,
            2 => INPUT11NEGSEL::Input12,
            3 => INPUT11NEGSEL::Input14,
            _ => unreachable!(),
        }
    }
    ///Selects ADCn_INPUT8 as negative channel input
    #[inline(always)]
    pub fn is_input8(&self) -> bool {
        *self == INPUT11NEGSEL::Input8
    }
    ///Selects ADCn_INPUT10 as negative channel input
    #[inline(always)]
    pub fn is_input10(&self) -> bool {
        *self == INPUT11NEGSEL::Input10
    }
    ///Selects ADCn_INPUT12 as negative channel input
    #[inline(always)]
    pub fn is_input12(&self) -> bool {
        *self == INPUT11NEGSEL::Input12
    }
    ///Selects ADCn_INPUT14 as negative channel input
    #[inline(always)]
    pub fn is_input14(&self) -> bool {
        *self == INPUT11NEGSEL::Input14
    }
}
///Field `INPUT11NEGSEL` writer - Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode
pub type Input11negselW<'a, REG> = crate::FieldWriter<'a, REG, 2, INPUT11NEGSEL, crate::Safe>;
impl<'a, REG> Input11negselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Selects ADCn_INPUT8 as negative channel input
    #[inline(always)]
    pub fn input8(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT11NEGSEL::Input8)
    }
    ///Selects ADCn_INPUT10 as negative channel input
    #[inline(always)]
    pub fn input10(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT11NEGSEL::Input10)
    }
    ///Selects ADCn_INPUT12 as negative channel input
    #[inline(always)]
    pub fn input12(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT11NEGSEL::Input12)
    }
    ///Selects ADCn_INPUT14 as negative channel input
    #[inline(always)]
    pub fn input14(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT11NEGSEL::Input14)
    }
}
///Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode
///
///Value on reset: 3
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUT13NEGSEL {
    ///0: Selects ADCn_INPUT8 as negative channel input
    Input8 = 0,
    ///1: Selects ADCn_INPUT10 as negative channel input
    Input10 = 1,
    ///2: Selects ADCn_INPUT12 as negative channel input
    Input12 = 2,
    ///3: Selects ADCn_INPUT14 as negative channel input
    Input14 = 3,
}
impl From<INPUT13NEGSEL> for u8 {
    #[inline(always)]
    fn from(variant: INPUT13NEGSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPUT13NEGSEL {
    type Ux = u8;
}
impl crate::IsEnum for INPUT13NEGSEL {}
///Field `INPUT13NEGSEL` reader - Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode
pub type Input13negselR = crate::FieldReader<INPUT13NEGSEL>;
impl Input13negselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INPUT13NEGSEL {
        match self.bits {
            0 => INPUT13NEGSEL::Input8,
            1 => INPUT13NEGSEL::Input10,
            2 => INPUT13NEGSEL::Input12,
            3 => INPUT13NEGSEL::Input14,
            _ => unreachable!(),
        }
    }
    ///Selects ADCn_INPUT8 as negative channel input
    #[inline(always)]
    pub fn is_input8(&self) -> bool {
        *self == INPUT13NEGSEL::Input8
    }
    ///Selects ADCn_INPUT10 as negative channel input
    #[inline(always)]
    pub fn is_input10(&self) -> bool {
        *self == INPUT13NEGSEL::Input10
    }
    ///Selects ADCn_INPUT12 as negative channel input
    #[inline(always)]
    pub fn is_input12(&self) -> bool {
        *self == INPUT13NEGSEL::Input12
    }
    ///Selects ADCn_INPUT14 as negative channel input
    #[inline(always)]
    pub fn is_input14(&self) -> bool {
        *self == INPUT13NEGSEL::Input14
    }
}
///Field `INPUT13NEGSEL` writer - Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode
pub type Input13negselW<'a, REG> = crate::FieldWriter<'a, REG, 2, INPUT13NEGSEL, crate::Safe>;
impl<'a, REG> Input13negselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Selects ADCn_INPUT8 as negative channel input
    #[inline(always)]
    pub fn input8(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT13NEGSEL::Input8)
    }
    ///Selects ADCn_INPUT10 as negative channel input
    #[inline(always)]
    pub fn input10(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT13NEGSEL::Input10)
    }
    ///Selects ADCn_INPUT12 as negative channel input
    #[inline(always)]
    pub fn input12(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT13NEGSEL::Input12)
    }
    ///Selects ADCn_INPUT14 as negative channel input
    #[inline(always)]
    pub fn input14(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT13NEGSEL::Input14)
    }
}
///Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUT15NEGSEL {
    ///0: Selects ADCn_INPUT8 as negative channel input
    Input8 = 0,
    ///1: Selects ADCn_INPUT10 as negative channel input
    Input10 = 1,
    ///2: Selects ADCn_INPUT12 as negative channel input
    Input12 = 2,
    ///3: Selects ADCn_INPUT14 as negative channel input
    Input14 = 3,
}
impl From<INPUT15NEGSEL> for u8 {
    #[inline(always)]
    fn from(variant: INPUT15NEGSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPUT15NEGSEL {
    type Ux = u8;
}
impl crate::IsEnum for INPUT15NEGSEL {}
///Field `INPUT15NEGSEL` reader - Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode
pub type Input15negselR = crate::FieldReader<INPUT15NEGSEL>;
impl Input15negselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INPUT15NEGSEL {
        match self.bits {
            0 => INPUT15NEGSEL::Input8,
            1 => INPUT15NEGSEL::Input10,
            2 => INPUT15NEGSEL::Input12,
            3 => INPUT15NEGSEL::Input14,
            _ => unreachable!(),
        }
    }
    ///Selects ADCn_INPUT8 as negative channel input
    #[inline(always)]
    pub fn is_input8(&self) -> bool {
        *self == INPUT15NEGSEL::Input8
    }
    ///Selects ADCn_INPUT10 as negative channel input
    #[inline(always)]
    pub fn is_input10(&self) -> bool {
        *self == INPUT15NEGSEL::Input10
    }
    ///Selects ADCn_INPUT12 as negative channel input
    #[inline(always)]
    pub fn is_input12(&self) -> bool {
        *self == INPUT15NEGSEL::Input12
    }
    ///Selects ADCn_INPUT14 as negative channel input
    #[inline(always)]
    pub fn is_input14(&self) -> bool {
        *self == INPUT15NEGSEL::Input14
    }
}
///Field `INPUT15NEGSEL` writer - Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode
pub type Input15negselW<'a, REG> = crate::FieldWriter<'a, REG, 2, INPUT15NEGSEL, crate::Safe>;
impl<'a, REG> Input15negselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Selects ADCn_INPUT8 as negative channel input
    #[inline(always)]
    pub fn input8(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT15NEGSEL::Input8)
    }
    ///Selects ADCn_INPUT10 as negative channel input
    #[inline(always)]
    pub fn input10(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT15NEGSEL::Input10)
    }
    ///Selects ADCn_INPUT12 as negative channel input
    #[inline(always)]
    pub fn input12(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT15NEGSEL::Input12)
    }
    ///Selects ADCn_INPUT14 as negative channel input
    #[inline(always)]
    pub fn input14(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT15NEGSEL::Input14)
    }
}
impl R {
    ///Bits 0:1 - Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode
    #[inline(always)]
    pub fn input0negsel(&self) -> Input0negselR {
        Input0negselR::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode
    #[inline(always)]
    pub fn input2negsel(&self) -> Input2negselR {
        Input2negselR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode
    #[inline(always)]
    pub fn input4negsel(&self) -> Input4negselR {
        Input4negselR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode
    #[inline(always)]
    pub fn input6negsel(&self) -> Input6negselR {
        Input6negselR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode
    #[inline(always)]
    pub fn input9negsel(&self) -> Input9negselR {
        Input9negselR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode
    #[inline(always)]
    pub fn input11negsel(&self) -> Input11negselR {
        Input11negselR::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode
    #[inline(always)]
    pub fn input13negsel(&self) -> Input13negselR {
        Input13negselR::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode
    #[inline(always)]
    pub fn input15negsel(&self) -> Input15negselR {
        Input15negselR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCANNEGSEL")
            .field("input0negsel", &self.input0negsel())
            .field("input2negsel", &self.input2negsel())
            .field("input4negsel", &self.input4negsel())
            .field("input6negsel", &self.input6negsel())
            .field("input9negsel", &self.input9negsel())
            .field("input11negsel", &self.input11negsel())
            .field("input13negsel", &self.input13negsel())
            .field("input15negsel", &self.input15negsel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode
    #[inline(always)]
    #[must_use]
    pub fn input0negsel(&mut self) -> Input0negselW<SCANNEGSELrs> {
        Input0negselW::new(self, 0)
    }
    ///Bits 2:3 - Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode
    #[inline(always)]
    #[must_use]
    pub fn input2negsel(&mut self) -> Input2negselW<SCANNEGSELrs> {
        Input2negselW::new(self, 2)
    }
    ///Bits 4:5 - Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode
    #[inline(always)]
    #[must_use]
    pub fn input4negsel(&mut self) -> Input4negselW<SCANNEGSELrs> {
        Input4negselW::new(self, 4)
    }
    ///Bits 6:7 - Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode
    #[inline(always)]
    #[must_use]
    pub fn input6negsel(&mut self) -> Input6negselW<SCANNEGSELrs> {
        Input6negselW::new(self, 6)
    }
    ///Bits 8:9 - Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode
    #[inline(always)]
    #[must_use]
    pub fn input9negsel(&mut self) -> Input9negselW<SCANNEGSELrs> {
        Input9negselW::new(self, 8)
    }
    ///Bits 10:11 - Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode
    #[inline(always)]
    #[must_use]
    pub fn input11negsel(&mut self) -> Input11negselW<SCANNEGSELrs> {
        Input11negselW::new(self, 10)
    }
    ///Bits 12:13 - Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode
    #[inline(always)]
    #[must_use]
    pub fn input13negsel(&mut self) -> Input13negselW<SCANNEGSELrs> {
        Input13negselW::new(self, 12)
    }
    ///Bits 14:15 - Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode
    #[inline(always)]
    #[must_use]
    pub fn input15negsel(&mut self) -> Input15negselW<SCANNEGSELrs> {
        Input15negselW::new(self, 14)
    }
}
///Negative Input Select Register for Scan
///
///You can [`read`](crate::Reg::read) this register and get [`scannegsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scannegsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SCANNEGSELrs;
impl crate::RegisterSpec for SCANNEGSELrs {
    type Ux = u32;
}
///`read()` method returns [`scannegsel::R`](R) reader structure
impl crate::Readable for SCANNEGSELrs {}
///`write(|w| ..)` method takes [`scannegsel::W`](W) writer structure
impl crate::Writable for SCANNEGSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SCANNEGSEL to value 0x39e4
impl crate::Resettable for SCANNEGSELrs {
    const RESET_VALUE: u32 = 0x39e4;
}
