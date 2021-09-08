pub trait SingleByte {
    fn value(&self) -> u8;

    fn from_bools(bs: &[bool]) -> u8 {
        assert!(bs.len() <= 8);
        bs.iter()
            .enumerate()
            .map(|(i, b)| if *b { 1 << i } else { 0 })
            .sum()
    }

    fn at(&self, i: u8) -> bool {
        self.value() & (1 << i) != 0
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
