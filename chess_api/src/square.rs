

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Square {
    x: u8,
    y: u8,
}

impl Square {
    /// Creates new square object
    pub fn new(x: u8, y: u8) -> Square {
        assert!(x < 8 && y < 8);
        
        Square {
            x, y
        }
    }

    /// Creates square object from uci string
    ///
    /// # Examples
    /// ```
    /// # use chess_api::square::Square;
    /// #
    /// assert_eq!(Square::new(0, 0), Square::from_uci("a1").unwrap());
    /// assert_eq!(Square::new(0, 1), Square::from_uci("a2").unwrap());
    /// assert_eq!(Square::new(7, 7), Square::from_uci("h8").unwrap());
    ///
    /// Square::from_uci("a0").unwrap_err();
    /// Square::from_uci("a9").unwrap_err();
    /// Square::from_uci("i1").unwrap_err();
    /// ```
    pub fn from_uci(uci: &str) -> Result<Square, ()> {
        let mut chars = uci.chars();
        let file = chars.next();
        let rank = chars.next();
  
        if chars.next().is_some() {
            return Err(())
        }

        if let (Some(file), Some(rank)) = (file, rank) {
            "abcdefgh".chars().enumerate()
                .find_map(|(i, ch)| if ch == file { Some(i as u8) } else { None })
                .zip(rank.to_digit(10)
                    .filter(|&y| y > 0 && y < 9)
                    .map(|y| (y - 1) as u8))
                .map(|(x, y)| Square::new(x, y))
                .ok_or(())
        } else { Err(()) }
    }

    /// Converts square to uci string
    ///
    /// # Examples
    /// ```
    /// # use chess_api::square::Square;
    /// #
    /// let a1 = Square::new(0, 0);
    /// assert_eq!(a1.to_uci(), "a1");
    ///
    /// let c4 = Square::new(2, 3);
    /// assert_eq!(c4.to_uci(), "c4");
    /// ```
    pub fn to_uci(&self) -> String {
        format!("{}{}", "abcdefgh".chars().nth(self.x.into()).unwrap(), self.y + 1)
    }
}
