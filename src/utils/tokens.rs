
pub type Token = TokenKind;

/** All toke as the last two arguments (line, col) of start of token */
#[derive(Debug,Clone,PartialEq,Copy)]
pub enum TokenKind {
    /** End of token stream */
    EOF(usize,usize),

    /** The multiplication Operator */
    Multiplier(usize,usize),

    /** The addition Operator */
    Adder(usize,usize),

    /** The equal operator, reserved for asignation */
    Equal(usize,usize),

    /** Variables identifiers (start_offset,end_offset) */
    Identifier(usize,usize,usize,usize),

    /** Real number */
    Real(f64,usize,usize),

    /** Integer number */
    Integer(i64,usize,usize),

    /** ( delimiting the opening of a parenthesis group */
    OpenParenthesis(usize,usize),

    /** { delimiting the opening of a curly group */
    OpenCurly(usize,usize),

    /** } delimiting the end of a square group */
    CloseCurly(usize,usize),

    /** ) delimiting the end of a parenthesis group */
    CloseParenthesis(usize,usize),

    /** Semicolon token */
    Semicolon(usize,usize),

    /** Unknown token in place of error (start_offset,end_offset,line,column)*/
    UnknownToken(usize,usize,usize,usize),

    /** inv keywored */
    Inv(usize,usize),

    /** afficher keyword */
    Afficher(usize,usize),

    /** aff_ral keyword */
    AffRal(usize,usize),

    /** racine keyword */
    Sqrt(usize,usize)
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match *self {
            TokenKind::Afficher(..) => write!(f,"Afficher"),
            TokenKind::EOF(..) => write!(f,"EOF"),
            TokenKind::AffRal(..) => write!(f,"Aff_ral"),
            TokenKind::CloseParenthesis(..) => write!(f,"CloseParenthesis"),
            TokenKind::CloseCurly(..) => write!(f,"CloseCurly"),
            TokenKind::Equal(..) => write!(f,"Equal"),
            TokenKind::Inv(..) => write!(f,"Inv"),
            TokenKind::Sqrt(..) => write!(f,"Sqrt"),
            TokenKind::OpenParenthesis(..) => write!(f,"OpenParenthesis"),
            TokenKind::OpenCurly(..) => write!(f,"OpenCurly"),
            TokenKind::Semicolon(..) => write!(f,"Semicolon"),
            TokenKind::Identifier(s, e,..) => write!(f,"Id({},{})",s,e),
            TokenKind::Integer(v,..) => write!(f,"Integer({})",v),
            TokenKind::Real(v,..) => write!(f,"Real({})",v),
            TokenKind::UnknownToken(s, e,..) => write!(f,"Unknown({},{})",s,e),
            TokenKind::Adder(..) => write!(f,"Adder"),
            TokenKind::Multiplier(..) => write!(f,"Multiplier")
        }
    }
}

impl Token {
    pub fn pos(&self) -> (usize,usize){
        match *self {
            TokenKind::Afficher(l,c) => return (l,c),
            TokenKind::EOF(l,c) => return (l,c),
            TokenKind::AffRal(l,c) => return (l,c),
            TokenKind::CloseParenthesis(l,c) => return (l,c),
            TokenKind::CloseCurly(l,c) => return (l,c),
            TokenKind::Equal(l,c) => return (l,c),
            TokenKind::Inv(l,c) => return (l,c),
            TokenKind::Sqrt(l,c) => return (l,c),
            TokenKind::OpenParenthesis(l,c) => return (l,c),
            TokenKind::OpenCurly(l,c) => return (l,c),
            TokenKind::Semicolon(l,c) => return (l,c),
            TokenKind::Identifier(_s, _e,l,c) => return (l,c),
            TokenKind::Integer(_v,l,c) => return (l,c),
            TokenKind::Real(_v,l,c) => return (l,c),
            TokenKind::UnknownToken(_s, _e,l,c) => return (l,c),
            TokenKind::Adder(l,c) => return (l,c),
            TokenKind::Multiplier(l,c) => return (l,c),
        }
    }
}