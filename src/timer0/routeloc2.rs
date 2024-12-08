///Register `ROUTELOC2` reader
pub type R = crate::R<ROUTELOC2rs>;
///Register `ROUTELOC2` writer
pub type W = crate::W<ROUTELOC2rs>;
///I/O Location
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDTI0LOC {
    ///0: Location 0
    Loc0 = 0,
    ///1: Location 1
    Loc1 = 1,
    ///2: Location 2
    Loc2 = 2,
    ///3: Location 3
    Loc3 = 3,
    ///4: Location 4
    Loc4 = 4,
    ///5: Location 5
    Loc5 = 5,
    ///6: Location 6
    Loc6 = 6,
    ///7: Location 7
    Loc7 = 7,
    ///8: Location 8
    Loc8 = 8,
    ///9: Location 9
    Loc9 = 9,
    ///10: Location 10
    Loc10 = 10,
    ///11: Location 11
    Loc11 = 11,
    ///12: Location 12
    Loc12 = 12,
    ///13: Location 13
    Loc13 = 13,
    ///14: Location 14
    Loc14 = 14,
    ///15: Location 15
    Loc15 = 15,
    ///16: Location 16
    Loc16 = 16,
    ///17: Location 17
    Loc17 = 17,
    ///18: Location 18
    Loc18 = 18,
    ///19: Location 19
    Loc19 = 19,
    ///20: Location 20
    Loc20 = 20,
    ///21: Location 21
    Loc21 = 21,
    ///22: Location 22
    Loc22 = 22,
    ///23: Location 23
    Loc23 = 23,
    ///24: Location 24
    Loc24 = 24,
    ///25: Location 25
    Loc25 = 25,
    ///26: Location 26
    Loc26 = 26,
    ///27: Location 27
    Loc27 = 27,
    ///28: Location 28
    Loc28 = 28,
    ///29: Location 29
    Loc29 = 29,
    ///30: Location 30
    Loc30 = 30,
    ///31: Location 31
    Loc31 = 31,
}
impl From<CDTI0LOC> for u8 {
    #[inline(always)]
    fn from(variant: CDTI0LOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CDTI0LOC {
    type Ux = u8;
}
impl crate::IsEnum for CDTI0LOC {}
///Field `CDTI0LOC` reader - I/O Location
pub type Cdti0locR = crate::FieldReader<CDTI0LOC>;
impl Cdti0locR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CDTI0LOC> {
        match self.bits {
            0 => Some(CDTI0LOC::Loc0),
            1 => Some(CDTI0LOC::Loc1),
            2 => Some(CDTI0LOC::Loc2),
            3 => Some(CDTI0LOC::Loc3),
            4 => Some(CDTI0LOC::Loc4),
            5 => Some(CDTI0LOC::Loc5),
            6 => Some(CDTI0LOC::Loc6),
            7 => Some(CDTI0LOC::Loc7),
            8 => Some(CDTI0LOC::Loc8),
            9 => Some(CDTI0LOC::Loc9),
            10 => Some(CDTI0LOC::Loc10),
            11 => Some(CDTI0LOC::Loc11),
            12 => Some(CDTI0LOC::Loc12),
            13 => Some(CDTI0LOC::Loc13),
            14 => Some(CDTI0LOC::Loc14),
            15 => Some(CDTI0LOC::Loc15),
            16 => Some(CDTI0LOC::Loc16),
            17 => Some(CDTI0LOC::Loc17),
            18 => Some(CDTI0LOC::Loc18),
            19 => Some(CDTI0LOC::Loc19),
            20 => Some(CDTI0LOC::Loc20),
            21 => Some(CDTI0LOC::Loc21),
            22 => Some(CDTI0LOC::Loc22),
            23 => Some(CDTI0LOC::Loc23),
            24 => Some(CDTI0LOC::Loc24),
            25 => Some(CDTI0LOC::Loc25),
            26 => Some(CDTI0LOC::Loc26),
            27 => Some(CDTI0LOC::Loc27),
            28 => Some(CDTI0LOC::Loc28),
            29 => Some(CDTI0LOC::Loc29),
            30 => Some(CDTI0LOC::Loc30),
            31 => Some(CDTI0LOC::Loc31),
            _ => None,
        }
    }
    ///Location 0
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CDTI0LOC::Loc0
    }
    ///Location 1
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CDTI0LOC::Loc1
    }
    ///Location 2
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CDTI0LOC::Loc2
    }
    ///Location 3
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CDTI0LOC::Loc3
    }
    ///Location 4
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CDTI0LOC::Loc4
    }
    ///Location 5
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CDTI0LOC::Loc5
    }
    ///Location 6
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CDTI0LOC::Loc6
    }
    ///Location 7
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CDTI0LOC::Loc7
    }
    ///Location 8
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CDTI0LOC::Loc8
    }
    ///Location 9
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CDTI0LOC::Loc9
    }
    ///Location 10
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CDTI0LOC::Loc10
    }
    ///Location 11
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == CDTI0LOC::Loc11
    }
    ///Location 12
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == CDTI0LOC::Loc12
    }
    ///Location 13
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == CDTI0LOC::Loc13
    }
    ///Location 14
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == CDTI0LOC::Loc14
    }
    ///Location 15
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == CDTI0LOC::Loc15
    }
    ///Location 16
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == CDTI0LOC::Loc16
    }
    ///Location 17
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == CDTI0LOC::Loc17
    }
    ///Location 18
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == CDTI0LOC::Loc18
    }
    ///Location 19
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == CDTI0LOC::Loc19
    }
    ///Location 20
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == CDTI0LOC::Loc20
    }
    ///Location 21
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == CDTI0LOC::Loc21
    }
    ///Location 22
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == CDTI0LOC::Loc22
    }
    ///Location 23
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == CDTI0LOC::Loc23
    }
    ///Location 24
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == CDTI0LOC::Loc24
    }
    ///Location 25
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == CDTI0LOC::Loc25
    }
    ///Location 26
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == CDTI0LOC::Loc26
    }
    ///Location 27
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == CDTI0LOC::Loc27
    }
    ///Location 28
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == CDTI0LOC::Loc28
    }
    ///Location 29
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == CDTI0LOC::Loc29
    }
    ///Location 30
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == CDTI0LOC::Loc30
    }
    ///Location 31
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == CDTI0LOC::Loc31
    }
}
///Field `CDTI0LOC` writer - I/O Location
pub type Cdti0locW<'a, REG> = crate::FieldWriter<'a, REG, 6, CDTI0LOC>;
impl<'a, REG> Cdti0locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Location 0
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc0)
    }
    ///Location 1
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc1)
    }
    ///Location 2
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc2)
    }
    ///Location 3
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc3)
    }
    ///Location 4
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc4)
    }
    ///Location 5
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc5)
    }
    ///Location 6
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc6)
    }
    ///Location 7
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc7)
    }
    ///Location 8
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc8)
    }
    ///Location 9
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc9)
    }
    ///Location 10
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc10)
    }
    ///Location 11
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc11)
    }
    ///Location 12
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc12)
    }
    ///Location 13
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc13)
    }
    ///Location 14
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc14)
    }
    ///Location 15
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc15)
    }
    ///Location 16
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc16)
    }
    ///Location 17
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc17)
    }
    ///Location 18
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc18)
    }
    ///Location 19
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc19)
    }
    ///Location 20
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc20)
    }
    ///Location 21
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc21)
    }
    ///Location 22
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc22)
    }
    ///Location 23
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc23)
    }
    ///Location 24
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc24)
    }
    ///Location 25
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc25)
    }
    ///Location 26
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc26)
    }
    ///Location 27
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc27)
    }
    ///Location 28
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc28)
    }
    ///Location 29
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc29)
    }
    ///Location 30
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc30)
    }
    ///Location 31
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI0LOC::Loc31)
    }
}
///I/O Location
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDTI1LOC {
    ///0: Location 0
    Loc0 = 0,
    ///1: Location 1
    Loc1 = 1,
    ///2: Location 2
    Loc2 = 2,
    ///3: Location 3
    Loc3 = 3,
    ///4: Location 4
    Loc4 = 4,
    ///5: Location 5
    Loc5 = 5,
    ///6: Location 6
    Loc6 = 6,
    ///7: Location 7
    Loc7 = 7,
    ///8: Location 8
    Loc8 = 8,
    ///9: Location 9
    Loc9 = 9,
    ///10: Location 10
    Loc10 = 10,
    ///11: Location 11
    Loc11 = 11,
    ///12: Location 12
    Loc12 = 12,
    ///13: Location 13
    Loc13 = 13,
    ///14: Location 14
    Loc14 = 14,
    ///15: Location 15
    Loc15 = 15,
    ///16: Location 16
    Loc16 = 16,
    ///17: Location 17
    Loc17 = 17,
    ///18: Location 18
    Loc18 = 18,
    ///19: Location 19
    Loc19 = 19,
    ///20: Location 20
    Loc20 = 20,
    ///21: Location 21
    Loc21 = 21,
    ///22: Location 22
    Loc22 = 22,
    ///23: Location 23
    Loc23 = 23,
    ///24: Location 24
    Loc24 = 24,
    ///25: Location 25
    Loc25 = 25,
    ///26: Location 26
    Loc26 = 26,
    ///27: Location 27
    Loc27 = 27,
    ///28: Location 28
    Loc28 = 28,
    ///29: Location 29
    Loc29 = 29,
    ///30: Location 30
    Loc30 = 30,
    ///31: Location 31
    Loc31 = 31,
}
impl From<CDTI1LOC> for u8 {
    #[inline(always)]
    fn from(variant: CDTI1LOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CDTI1LOC {
    type Ux = u8;
}
impl crate::IsEnum for CDTI1LOC {}
///Field `CDTI1LOC` reader - I/O Location
pub type Cdti1locR = crate::FieldReader<CDTI1LOC>;
impl Cdti1locR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CDTI1LOC> {
        match self.bits {
            0 => Some(CDTI1LOC::Loc0),
            1 => Some(CDTI1LOC::Loc1),
            2 => Some(CDTI1LOC::Loc2),
            3 => Some(CDTI1LOC::Loc3),
            4 => Some(CDTI1LOC::Loc4),
            5 => Some(CDTI1LOC::Loc5),
            6 => Some(CDTI1LOC::Loc6),
            7 => Some(CDTI1LOC::Loc7),
            8 => Some(CDTI1LOC::Loc8),
            9 => Some(CDTI1LOC::Loc9),
            10 => Some(CDTI1LOC::Loc10),
            11 => Some(CDTI1LOC::Loc11),
            12 => Some(CDTI1LOC::Loc12),
            13 => Some(CDTI1LOC::Loc13),
            14 => Some(CDTI1LOC::Loc14),
            15 => Some(CDTI1LOC::Loc15),
            16 => Some(CDTI1LOC::Loc16),
            17 => Some(CDTI1LOC::Loc17),
            18 => Some(CDTI1LOC::Loc18),
            19 => Some(CDTI1LOC::Loc19),
            20 => Some(CDTI1LOC::Loc20),
            21 => Some(CDTI1LOC::Loc21),
            22 => Some(CDTI1LOC::Loc22),
            23 => Some(CDTI1LOC::Loc23),
            24 => Some(CDTI1LOC::Loc24),
            25 => Some(CDTI1LOC::Loc25),
            26 => Some(CDTI1LOC::Loc26),
            27 => Some(CDTI1LOC::Loc27),
            28 => Some(CDTI1LOC::Loc28),
            29 => Some(CDTI1LOC::Loc29),
            30 => Some(CDTI1LOC::Loc30),
            31 => Some(CDTI1LOC::Loc31),
            _ => None,
        }
    }
    ///Location 0
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CDTI1LOC::Loc0
    }
    ///Location 1
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CDTI1LOC::Loc1
    }
    ///Location 2
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CDTI1LOC::Loc2
    }
    ///Location 3
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CDTI1LOC::Loc3
    }
    ///Location 4
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CDTI1LOC::Loc4
    }
    ///Location 5
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CDTI1LOC::Loc5
    }
    ///Location 6
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CDTI1LOC::Loc6
    }
    ///Location 7
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CDTI1LOC::Loc7
    }
    ///Location 8
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CDTI1LOC::Loc8
    }
    ///Location 9
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CDTI1LOC::Loc9
    }
    ///Location 10
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CDTI1LOC::Loc10
    }
    ///Location 11
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == CDTI1LOC::Loc11
    }
    ///Location 12
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == CDTI1LOC::Loc12
    }
    ///Location 13
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == CDTI1LOC::Loc13
    }
    ///Location 14
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == CDTI1LOC::Loc14
    }
    ///Location 15
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == CDTI1LOC::Loc15
    }
    ///Location 16
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == CDTI1LOC::Loc16
    }
    ///Location 17
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == CDTI1LOC::Loc17
    }
    ///Location 18
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == CDTI1LOC::Loc18
    }
    ///Location 19
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == CDTI1LOC::Loc19
    }
    ///Location 20
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == CDTI1LOC::Loc20
    }
    ///Location 21
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == CDTI1LOC::Loc21
    }
    ///Location 22
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == CDTI1LOC::Loc22
    }
    ///Location 23
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == CDTI1LOC::Loc23
    }
    ///Location 24
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == CDTI1LOC::Loc24
    }
    ///Location 25
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == CDTI1LOC::Loc25
    }
    ///Location 26
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == CDTI1LOC::Loc26
    }
    ///Location 27
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == CDTI1LOC::Loc27
    }
    ///Location 28
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == CDTI1LOC::Loc28
    }
    ///Location 29
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == CDTI1LOC::Loc29
    }
    ///Location 30
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == CDTI1LOC::Loc30
    }
    ///Location 31
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == CDTI1LOC::Loc31
    }
}
///Field `CDTI1LOC` writer - I/O Location
pub type Cdti1locW<'a, REG> = crate::FieldWriter<'a, REG, 6, CDTI1LOC>;
impl<'a, REG> Cdti1locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Location 0
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc0)
    }
    ///Location 1
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc1)
    }
    ///Location 2
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc2)
    }
    ///Location 3
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc3)
    }
    ///Location 4
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc4)
    }
    ///Location 5
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc5)
    }
    ///Location 6
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc6)
    }
    ///Location 7
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc7)
    }
    ///Location 8
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc8)
    }
    ///Location 9
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc9)
    }
    ///Location 10
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc10)
    }
    ///Location 11
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc11)
    }
    ///Location 12
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc12)
    }
    ///Location 13
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc13)
    }
    ///Location 14
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc14)
    }
    ///Location 15
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc15)
    }
    ///Location 16
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc16)
    }
    ///Location 17
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc17)
    }
    ///Location 18
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc18)
    }
    ///Location 19
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc19)
    }
    ///Location 20
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc20)
    }
    ///Location 21
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc21)
    }
    ///Location 22
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc22)
    }
    ///Location 23
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc23)
    }
    ///Location 24
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc24)
    }
    ///Location 25
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc25)
    }
    ///Location 26
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc26)
    }
    ///Location 27
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc27)
    }
    ///Location 28
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc28)
    }
    ///Location 29
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc29)
    }
    ///Location 30
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc30)
    }
    ///Location 31
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI1LOC::Loc31)
    }
}
///I/O Location
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDTI2LOC {
    ///0: Location 0
    Loc0 = 0,
    ///1: Location 1
    Loc1 = 1,
    ///2: Location 2
    Loc2 = 2,
    ///3: Location 3
    Loc3 = 3,
    ///4: Location 4
    Loc4 = 4,
    ///5: Location 5
    Loc5 = 5,
    ///6: Location 6
    Loc6 = 6,
    ///7: Location 7
    Loc7 = 7,
    ///8: Location 8
    Loc8 = 8,
    ///9: Location 9
    Loc9 = 9,
    ///10: Location 10
    Loc10 = 10,
    ///11: Location 11
    Loc11 = 11,
    ///12: Location 12
    Loc12 = 12,
    ///13: Location 13
    Loc13 = 13,
    ///14: Location 14
    Loc14 = 14,
    ///15: Location 15
    Loc15 = 15,
    ///16: Location 16
    Loc16 = 16,
    ///17: Location 17
    Loc17 = 17,
    ///18: Location 18
    Loc18 = 18,
    ///19: Location 19
    Loc19 = 19,
    ///20: Location 20
    Loc20 = 20,
    ///21: Location 21
    Loc21 = 21,
    ///22: Location 22
    Loc22 = 22,
    ///23: Location 23
    Loc23 = 23,
    ///24: Location 24
    Loc24 = 24,
    ///25: Location 25
    Loc25 = 25,
    ///26: Location 26
    Loc26 = 26,
    ///27: Location 27
    Loc27 = 27,
    ///28: Location 28
    Loc28 = 28,
    ///29: Location 29
    Loc29 = 29,
    ///30: Location 30
    Loc30 = 30,
    ///31: Location 31
    Loc31 = 31,
}
impl From<CDTI2LOC> for u8 {
    #[inline(always)]
    fn from(variant: CDTI2LOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CDTI2LOC {
    type Ux = u8;
}
impl crate::IsEnum for CDTI2LOC {}
///Field `CDTI2LOC` reader - I/O Location
pub type Cdti2locR = crate::FieldReader<CDTI2LOC>;
impl Cdti2locR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CDTI2LOC> {
        match self.bits {
            0 => Some(CDTI2LOC::Loc0),
            1 => Some(CDTI2LOC::Loc1),
            2 => Some(CDTI2LOC::Loc2),
            3 => Some(CDTI2LOC::Loc3),
            4 => Some(CDTI2LOC::Loc4),
            5 => Some(CDTI2LOC::Loc5),
            6 => Some(CDTI2LOC::Loc6),
            7 => Some(CDTI2LOC::Loc7),
            8 => Some(CDTI2LOC::Loc8),
            9 => Some(CDTI2LOC::Loc9),
            10 => Some(CDTI2LOC::Loc10),
            11 => Some(CDTI2LOC::Loc11),
            12 => Some(CDTI2LOC::Loc12),
            13 => Some(CDTI2LOC::Loc13),
            14 => Some(CDTI2LOC::Loc14),
            15 => Some(CDTI2LOC::Loc15),
            16 => Some(CDTI2LOC::Loc16),
            17 => Some(CDTI2LOC::Loc17),
            18 => Some(CDTI2LOC::Loc18),
            19 => Some(CDTI2LOC::Loc19),
            20 => Some(CDTI2LOC::Loc20),
            21 => Some(CDTI2LOC::Loc21),
            22 => Some(CDTI2LOC::Loc22),
            23 => Some(CDTI2LOC::Loc23),
            24 => Some(CDTI2LOC::Loc24),
            25 => Some(CDTI2LOC::Loc25),
            26 => Some(CDTI2LOC::Loc26),
            27 => Some(CDTI2LOC::Loc27),
            28 => Some(CDTI2LOC::Loc28),
            29 => Some(CDTI2LOC::Loc29),
            30 => Some(CDTI2LOC::Loc30),
            31 => Some(CDTI2LOC::Loc31),
            _ => None,
        }
    }
    ///Location 0
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CDTI2LOC::Loc0
    }
    ///Location 1
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CDTI2LOC::Loc1
    }
    ///Location 2
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CDTI2LOC::Loc2
    }
    ///Location 3
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CDTI2LOC::Loc3
    }
    ///Location 4
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CDTI2LOC::Loc4
    }
    ///Location 5
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CDTI2LOC::Loc5
    }
    ///Location 6
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CDTI2LOC::Loc6
    }
    ///Location 7
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CDTI2LOC::Loc7
    }
    ///Location 8
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CDTI2LOC::Loc8
    }
    ///Location 9
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CDTI2LOC::Loc9
    }
    ///Location 10
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CDTI2LOC::Loc10
    }
    ///Location 11
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == CDTI2LOC::Loc11
    }
    ///Location 12
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == CDTI2LOC::Loc12
    }
    ///Location 13
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == CDTI2LOC::Loc13
    }
    ///Location 14
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == CDTI2LOC::Loc14
    }
    ///Location 15
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == CDTI2LOC::Loc15
    }
    ///Location 16
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == CDTI2LOC::Loc16
    }
    ///Location 17
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == CDTI2LOC::Loc17
    }
    ///Location 18
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == CDTI2LOC::Loc18
    }
    ///Location 19
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == CDTI2LOC::Loc19
    }
    ///Location 20
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == CDTI2LOC::Loc20
    }
    ///Location 21
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == CDTI2LOC::Loc21
    }
    ///Location 22
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == CDTI2LOC::Loc22
    }
    ///Location 23
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == CDTI2LOC::Loc23
    }
    ///Location 24
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == CDTI2LOC::Loc24
    }
    ///Location 25
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == CDTI2LOC::Loc25
    }
    ///Location 26
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == CDTI2LOC::Loc26
    }
    ///Location 27
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == CDTI2LOC::Loc27
    }
    ///Location 28
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == CDTI2LOC::Loc28
    }
    ///Location 29
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == CDTI2LOC::Loc29
    }
    ///Location 30
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == CDTI2LOC::Loc30
    }
    ///Location 31
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == CDTI2LOC::Loc31
    }
}
///Field `CDTI2LOC` writer - I/O Location
pub type Cdti2locW<'a, REG> = crate::FieldWriter<'a, REG, 6, CDTI2LOC>;
impl<'a, REG> Cdti2locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Location 0
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc0)
    }
    ///Location 1
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc1)
    }
    ///Location 2
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc2)
    }
    ///Location 3
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc3)
    }
    ///Location 4
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc4)
    }
    ///Location 5
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc5)
    }
    ///Location 6
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc6)
    }
    ///Location 7
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc7)
    }
    ///Location 8
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc8)
    }
    ///Location 9
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc9)
    }
    ///Location 10
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc10)
    }
    ///Location 11
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc11)
    }
    ///Location 12
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc12)
    }
    ///Location 13
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc13)
    }
    ///Location 14
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc14)
    }
    ///Location 15
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc15)
    }
    ///Location 16
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc16)
    }
    ///Location 17
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc17)
    }
    ///Location 18
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc18)
    }
    ///Location 19
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc19)
    }
    ///Location 20
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc20)
    }
    ///Location 21
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc21)
    }
    ///Location 22
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc22)
    }
    ///Location 23
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc23)
    }
    ///Location 24
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc24)
    }
    ///Location 25
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc25)
    }
    ///Location 26
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc26)
    }
    ///Location 27
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc27)
    }
    ///Location 28
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc28)
    }
    ///Location 29
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc29)
    }
    ///Location 30
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc30)
    }
    ///Location 31
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(CDTI2LOC::Loc31)
    }
}
impl R {
    ///Bits 0:5 - I/O Location
    #[inline(always)]
    pub fn cdti0loc(&self) -> Cdti0locR {
        Cdti0locR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - I/O Location
    #[inline(always)]
    pub fn cdti1loc(&self) -> Cdti1locR {
        Cdti1locR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:21 - I/O Location
    #[inline(always)]
    pub fn cdti2loc(&self) -> Cdti2locR {
        Cdti2locR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTELOC2")
            .field("cdti0loc", &self.cdti0loc())
            .field("cdti1loc", &self.cdti1loc())
            .field("cdti2loc", &self.cdti2loc())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - I/O Location
    #[inline(always)]
    #[must_use]
    pub fn cdti0loc(&mut self) -> Cdti0locW<ROUTELOC2rs> {
        Cdti0locW::new(self, 0)
    }
    ///Bits 8:13 - I/O Location
    #[inline(always)]
    #[must_use]
    pub fn cdti1loc(&mut self) -> Cdti1locW<ROUTELOC2rs> {
        Cdti1locW::new(self, 8)
    }
    ///Bits 16:21 - I/O Location
    #[inline(always)]
    #[must_use]
    pub fn cdti2loc(&mut self) -> Cdti2locW<ROUTELOC2rs> {
        Cdti2locW::new(self, 16)
    }
}
///I/O Routing Location Register
///
///You can [`read`](crate::Reg::read) this register and get [`routeloc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ROUTELOC2rs;
impl crate::RegisterSpec for ROUTELOC2rs {
    type Ux = u32;
}
///`read()` method returns [`routeloc2::R`](R) reader structure
impl crate::Readable for ROUTELOC2rs {}
///`write(|w| ..)` method takes [`routeloc2::W`](W) writer structure
impl crate::Writable for ROUTELOC2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ROUTELOC2 to value 0
impl crate::Resettable for ROUTELOC2rs {
    const RESET_VALUE: u32 = 0;
}
