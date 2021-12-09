use crate::square::Square;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Move {
    start: Square,
    end: Square
}

impl Move {
    /// Creates new move object from squares
    pub fn new(start: Square, end: Square) -> Move {
        Move {
            start, end
        }
    }


    /// Converts uci string into move object
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_api::square::Square;
    /// # use chess_api::movement::Move;
    /// #
    /// let m = Move::from_uci("e3c4").unwrap();
    /// assert_eq!(m, Move::new(Square::new(4, 2), Square::new(2, 3)));
    ///
    /// Move::from_uci("a1a1a").unwrap_err();
    /// Move::from_uci("a0a0").unwrap_err();
    /// ```
    pub fn from_uci(uci: &str) -> Result<Move, ()> {
        let mut chars = uci.chars();
        let result = Self::from_uci_iter(&mut chars);

        if chars.next().is_none() { result } else { Err(()) }
    }

    pub fn from_uci_iter<I>(chars: &mut I) -> Result<Move, ()>
        where I: Iterator<Item=char>
    {
        let start = Square::from_uci_iter(chars)?;
        let end = Square::from_uci_iter(chars)?;

        Ok(Move::new(start, end))
    }

    /// Converts move into uci string
    ///
    /// ```
    /// # use chess_api::square::Square;
    /// # use chess_api::movement::Move;
    /// #
    /// let m = Move::new(Square::new(0, 0), Square::new(7, 7));
    /// assert_eq!(m.to_uci(), "a1h8");
    /// ```
    pub fn to_uci(&self) -> String {
        let mut uci = self.start.to_uci();
        uci.push_str(self.end.to_uci().as_str());
        uci
    }
}


