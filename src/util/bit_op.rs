pub trait SingleByte {
    fn value(&self) -> u8;

    fn from_bools(bs: &[bool]) -> u8 {
        assert!(bs.len() <= 8);
        bs.iter()
            .enumerate()
            .map(|(i, b)| if *b { 1 << i } else { 0 })
            .sum()
    }

    fn get(&self, i: usize) -> bool {
        self.value() & (1 << i) != 0
    }

    fn set(&self, i: usize, b: bool) -> u8 {
        let v = if b { 1 } else { 0 };
        self.value() | (v << i)
    }

    /// mask の分だけのビット数を offset の位置から取り出す。
    /// mask は 0 ビットから始まる値を与え、戻り値も 0 ビットから始まるようにシフトされる。
    ///
    /// # Examples
    /// ```
    /// use hexcect::util::SingleByte;
    ///
    /// let a = 0b_1110_0110_u8;
    /// let b = a.get_with_mask(0b111, 3);
    /// assert_eq!(b, 0b100);
    /// ```
    fn get_with_mask(&self, mask: u8, offset: usize) -> u8 {
        self.value() >> offset & mask
    }

    /// mask は反転してシフトされ、v もシフトされてセットされる。
    ///
    /// # Examples
    /// ```
    /// use hexcect::util::SingleByte;
    ///
    /// let a = 0b_1110_0110_u8;
    /// let b = a.set_with_mask(0b111, 3, 0b010);
    /// assert_eq!(b, 0b_1101_0110);
    /// ```
    fn set_with_mask(&self, mask: u8, offset: usize, v: u8) -> u8 {
        (self.value() & !(mask << offset)) | (v << offset)
    }
}

impl SingleByte for u8 {
    fn value(&self) -> u8 {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singlebyte_get_with_mask() {
        assert_eq!(0b_1010_0110.get_with_mask(0b111, 0), 0b110);
        assert_eq!(0b_1010_0110.get_with_mask(0b111, 1), 0b011);
        assert_eq!(0b_1010_0110.get_with_mask(0b111, 2), 0b001);
        assert_eq!(0b_1010_0110.get_with_mask(0b111, 3), 0b100);
        assert_eq!(0b_1010_0110.get_with_mask(0b111, 4), 0b010);
        assert_eq!(0b_1010_0110.get_with_mask(0b111, 5), 0b101);
        assert_eq!(0b_1010_0110.get_with_mask(0b111, 6), 0b010);
        assert_eq!(0b_1010_0110.get_with_mask(0b111, 7), 0b001);

        assert_eq!(0b_1010_0110.get_with_mask(0b11, 0), 0b10);
        assert_eq!(0b_1010_0110.get_with_mask(0b11, 1), 0b11);
        assert_eq!(0b_1010_0110.get_with_mask(0b11, 2), 0b01);
        assert_eq!(0b_1010_0110.get_with_mask(0b11, 3), 0b00);
        assert_eq!(0b_1010_0110.get_with_mask(0b11, 4), 0b10);
        assert_eq!(0b_1010_0110.get_with_mask(0b11, 5), 0b01);
        assert_eq!(0b_1010_0110.get_with_mask(0b11, 6), 0b10);
        assert_eq!(0b_1010_0110.get_with_mask(0b11, 7), 0b01);
    }

    #[test]
    fn singlebyte_set_with_mask() {
        assert_eq!(0b_1110_0101.set_with_mask(0b111, 0, 0b000), 0b_1110_0000);

        assert_eq!(0b_1110_0101.set_with_mask(0b111, 0, 0b010), 0b_1110_0010);
        assert_eq!(0b_1110_0101.set_with_mask(0b111, 1, 0b010), 0b_1110_0101);
        assert_eq!(0b_1110_0101.set_with_mask(0b111, 2, 0b010), 0b_1110_1001);
        assert_eq!(0b_1110_0101.set_with_mask(0b111, 3, 0b010), 0b_1101_0101);
        assert_eq!(0b_1110_0101.set_with_mask(0b111, 4, 0b010), 0b_1010_0101);
        assert_eq!(0b_1110_0101.set_with_mask(0b111, 5, 0b010), 0b_0100_0101);
        assert_eq!(0b_1110_0101.set_with_mask(0b111, 6, 0b010), 0b_1010_0101);
        assert_eq!(0b_1110_0101.set_with_mask(0b111, 7, 0b010), 0b_0110_0101);

        assert_eq!(0b_1110_0101.set_with_mask(0b111, 1, 0b111), 0b_1110_1111);
        assert_eq!(0b_1110_0101.set_with_mask(0b111, 2, 0b110), 0b_1111_1001);
        assert_eq!(0b_1110_0101.set_with_mask(0b111, 3, 0b110), 0b_1111_0101);
    }
}
