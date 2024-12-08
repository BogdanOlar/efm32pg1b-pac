///Register `ROUTELOC0` reader
pub type R = crate::R<ROUTELOC0rs>;
///Register `ROUTELOC0` writer
pub type W = crate::W<ROUTELOC0rs>;
///I/O Location
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXLOC {
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
impl From<RXLOC> for u8 {
    #[inline(always)]
    fn from(variant: RXLOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXLOC {
    type Ux = u8;
}
impl crate::IsEnum for RXLOC {}
///Field `RXLOC` reader - I/O Location
pub type RxlocR = crate::FieldReader<RXLOC>;
impl RxlocR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RXLOC> {
        match self.bits {
            0 => Some(RXLOC::Loc0),
            1 => Some(RXLOC::Loc1),
            2 => Some(RXLOC::Loc2),
            3 => Some(RXLOC::Loc3),
            4 => Some(RXLOC::Loc4),
            5 => Some(RXLOC::Loc5),
            6 => Some(RXLOC::Loc6),
            7 => Some(RXLOC::Loc7),
            8 => Some(RXLOC::Loc8),
            9 => Some(RXLOC::Loc9),
            10 => Some(RXLOC::Loc10),
            11 => Some(RXLOC::Loc11),
            12 => Some(RXLOC::Loc12),
            13 => Some(RXLOC::Loc13),
            14 => Some(RXLOC::Loc14),
            15 => Some(RXLOC::Loc15),
            16 => Some(RXLOC::Loc16),
            17 => Some(RXLOC::Loc17),
            18 => Some(RXLOC::Loc18),
            19 => Some(RXLOC::Loc19),
            20 => Some(RXLOC::Loc20),
            21 => Some(RXLOC::Loc21),
            22 => Some(RXLOC::Loc22),
            23 => Some(RXLOC::Loc23),
            24 => Some(RXLOC::Loc24),
            25 => Some(RXLOC::Loc25),
            26 => Some(RXLOC::Loc26),
            27 => Some(RXLOC::Loc27),
            28 => Some(RXLOC::Loc28),
            29 => Some(RXLOC::Loc29),
            30 => Some(RXLOC::Loc30),
            31 => Some(RXLOC::Loc31),
            _ => None,
        }
    }
    ///Location 0
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == RXLOC::Loc0
    }
    ///Location 1
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == RXLOC::Loc1
    }
    ///Location 2
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == RXLOC::Loc2
    }
    ///Location 3
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == RXLOC::Loc3
    }
    ///Location 4
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == RXLOC::Loc4
    }
    ///Location 5
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == RXLOC::Loc5
    }
    ///Location 6
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == RXLOC::Loc6
    }
    ///Location 7
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == RXLOC::Loc7
    }
    ///Location 8
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == RXLOC::Loc8
    }
    ///Location 9
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == RXLOC::Loc9
    }
    ///Location 10
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == RXLOC::Loc10
    }
    ///Location 11
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == RXLOC::Loc11
    }
    ///Location 12
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == RXLOC::Loc12
    }
    ///Location 13
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == RXLOC::Loc13
    }
    ///Location 14
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == RXLOC::Loc14
    }
    ///Location 15
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == RXLOC::Loc15
    }
    ///Location 16
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == RXLOC::Loc16
    }
    ///Location 17
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == RXLOC::Loc17
    }
    ///Location 18
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == RXLOC::Loc18
    }
    ///Location 19
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == RXLOC::Loc19
    }
    ///Location 20
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == RXLOC::Loc20
    }
    ///Location 21
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == RXLOC::Loc21
    }
    ///Location 22
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == RXLOC::Loc22
    }
    ///Location 23
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == RXLOC::Loc23
    }
    ///Location 24
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == RXLOC::Loc24
    }
    ///Location 25
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == RXLOC::Loc25
    }
    ///Location 26
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == RXLOC::Loc26
    }
    ///Location 27
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == RXLOC::Loc27
    }
    ///Location 28
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == RXLOC::Loc28
    }
    ///Location 29
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == RXLOC::Loc29
    }
    ///Location 30
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == RXLOC::Loc30
    }
    ///Location 31
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == RXLOC::Loc31
    }
}
///Field `RXLOC` writer - I/O Location
pub type RxlocW<'a, REG> = crate::FieldWriter<'a, REG, 6, RXLOC>;
impl<'a, REG> RxlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Location 0
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc0)
    }
    ///Location 1
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc1)
    }
    ///Location 2
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc2)
    }
    ///Location 3
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc3)
    }
    ///Location 4
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc4)
    }
    ///Location 5
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc5)
    }
    ///Location 6
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc6)
    }
    ///Location 7
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc7)
    }
    ///Location 8
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc8)
    }
    ///Location 9
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc9)
    }
    ///Location 10
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc10)
    }
    ///Location 11
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc11)
    }
    ///Location 12
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc12)
    }
    ///Location 13
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc13)
    }
    ///Location 14
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc14)
    }
    ///Location 15
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc15)
    }
    ///Location 16
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc16)
    }
    ///Location 17
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc17)
    }
    ///Location 18
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc18)
    }
    ///Location 19
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc19)
    }
    ///Location 20
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc20)
    }
    ///Location 21
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc21)
    }
    ///Location 22
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc22)
    }
    ///Location 23
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc23)
    }
    ///Location 24
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc24)
    }
    ///Location 25
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc25)
    }
    ///Location 26
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc26)
    }
    ///Location 27
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc27)
    }
    ///Location 28
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc28)
    }
    ///Location 29
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc29)
    }
    ///Location 30
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc30)
    }
    ///Location 31
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc31)
    }
}
///I/O Location
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXLOC {
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
impl From<TXLOC> for u8 {
    #[inline(always)]
    fn from(variant: TXLOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXLOC {
    type Ux = u8;
}
impl crate::IsEnum for TXLOC {}
///Field `TXLOC` reader - I/O Location
pub type TxlocR = crate::FieldReader<TXLOC>;
impl TxlocR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TXLOC> {
        match self.bits {
            0 => Some(TXLOC::Loc0),
            1 => Some(TXLOC::Loc1),
            2 => Some(TXLOC::Loc2),
            3 => Some(TXLOC::Loc3),
            4 => Some(TXLOC::Loc4),
            5 => Some(TXLOC::Loc5),
            6 => Some(TXLOC::Loc6),
            7 => Some(TXLOC::Loc7),
            8 => Some(TXLOC::Loc8),
            9 => Some(TXLOC::Loc9),
            10 => Some(TXLOC::Loc10),
            11 => Some(TXLOC::Loc11),
            12 => Some(TXLOC::Loc12),
            13 => Some(TXLOC::Loc13),
            14 => Some(TXLOC::Loc14),
            15 => Some(TXLOC::Loc15),
            16 => Some(TXLOC::Loc16),
            17 => Some(TXLOC::Loc17),
            18 => Some(TXLOC::Loc18),
            19 => Some(TXLOC::Loc19),
            20 => Some(TXLOC::Loc20),
            21 => Some(TXLOC::Loc21),
            22 => Some(TXLOC::Loc22),
            23 => Some(TXLOC::Loc23),
            24 => Some(TXLOC::Loc24),
            25 => Some(TXLOC::Loc25),
            26 => Some(TXLOC::Loc26),
            27 => Some(TXLOC::Loc27),
            28 => Some(TXLOC::Loc28),
            29 => Some(TXLOC::Loc29),
            30 => Some(TXLOC::Loc30),
            31 => Some(TXLOC::Loc31),
            _ => None,
        }
    }
    ///Location 0
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == TXLOC::Loc0
    }
    ///Location 1
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == TXLOC::Loc1
    }
    ///Location 2
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == TXLOC::Loc2
    }
    ///Location 3
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == TXLOC::Loc3
    }
    ///Location 4
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == TXLOC::Loc4
    }
    ///Location 5
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == TXLOC::Loc5
    }
    ///Location 6
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == TXLOC::Loc6
    }
    ///Location 7
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == TXLOC::Loc7
    }
    ///Location 8
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == TXLOC::Loc8
    }
    ///Location 9
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == TXLOC::Loc9
    }
    ///Location 10
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == TXLOC::Loc10
    }
    ///Location 11
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == TXLOC::Loc11
    }
    ///Location 12
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == TXLOC::Loc12
    }
    ///Location 13
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == TXLOC::Loc13
    }
    ///Location 14
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == TXLOC::Loc14
    }
    ///Location 15
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == TXLOC::Loc15
    }
    ///Location 16
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == TXLOC::Loc16
    }
    ///Location 17
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == TXLOC::Loc17
    }
    ///Location 18
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == TXLOC::Loc18
    }
    ///Location 19
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == TXLOC::Loc19
    }
    ///Location 20
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == TXLOC::Loc20
    }
    ///Location 21
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == TXLOC::Loc21
    }
    ///Location 22
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == TXLOC::Loc22
    }
    ///Location 23
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == TXLOC::Loc23
    }
    ///Location 24
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == TXLOC::Loc24
    }
    ///Location 25
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == TXLOC::Loc25
    }
    ///Location 26
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == TXLOC::Loc26
    }
    ///Location 27
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == TXLOC::Loc27
    }
    ///Location 28
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == TXLOC::Loc28
    }
    ///Location 29
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == TXLOC::Loc29
    }
    ///Location 30
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == TXLOC::Loc30
    }
    ///Location 31
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == TXLOC::Loc31
    }
}
///Field `TXLOC` writer - I/O Location
pub type TxlocW<'a, REG> = crate::FieldWriter<'a, REG, 6, TXLOC>;
impl<'a, REG> TxlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Location 0
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc0)
    }
    ///Location 1
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc1)
    }
    ///Location 2
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc2)
    }
    ///Location 3
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc3)
    }
    ///Location 4
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc4)
    }
    ///Location 5
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc5)
    }
    ///Location 6
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc6)
    }
    ///Location 7
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc7)
    }
    ///Location 8
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc8)
    }
    ///Location 9
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc9)
    }
    ///Location 10
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc10)
    }
    ///Location 11
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc11)
    }
    ///Location 12
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc12)
    }
    ///Location 13
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc13)
    }
    ///Location 14
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc14)
    }
    ///Location 15
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc15)
    }
    ///Location 16
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc16)
    }
    ///Location 17
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc17)
    }
    ///Location 18
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc18)
    }
    ///Location 19
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc19)
    }
    ///Location 20
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc20)
    }
    ///Location 21
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc21)
    }
    ///Location 22
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc22)
    }
    ///Location 23
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc23)
    }
    ///Location 24
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc24)
    }
    ///Location 25
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc25)
    }
    ///Location 26
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc26)
    }
    ///Location 27
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc27)
    }
    ///Location 28
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc28)
    }
    ///Location 29
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc29)
    }
    ///Location 30
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc30)
    }
    ///Location 31
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc31)
    }
}
impl R {
    ///Bits 0:5 - I/O Location
    #[inline(always)]
    pub fn rxloc(&self) -> RxlocR {
        RxlocR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - I/O Location
    #[inline(always)]
    pub fn txloc(&self) -> TxlocR {
        TxlocR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTELOC0")
            .field("rxloc", &self.rxloc())
            .field("txloc", &self.txloc())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - I/O Location
    #[inline(always)]
    #[must_use]
    pub fn rxloc(&mut self) -> RxlocW<ROUTELOC0rs> {
        RxlocW::new(self, 0)
    }
    ///Bits 8:13 - I/O Location
    #[inline(always)]
    #[must_use]
    pub fn txloc(&mut self) -> TxlocW<ROUTELOC0rs> {
        TxlocW::new(self, 8)
    }
}
///I/O Routing Location Register
///
///You can [`read`](crate::Reg::read) this register and get [`routeloc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ROUTELOC0rs;
impl crate::RegisterSpec for ROUTELOC0rs {
    type Ux = u32;
}
///`read()` method returns [`routeloc0::R`](R) reader structure
impl crate::Readable for ROUTELOC0rs {}
///`write(|w| ..)` method takes [`routeloc0::W`](W) writer structure
impl crate::Writable for ROUTELOC0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ROUTELOC0 to value 0
impl crate::Resettable for ROUTELOC0rs {
    const RESET_VALUE: u32 = 0;
}
