use crate::square::Square;


#[derive(Copy, Clone)]
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


