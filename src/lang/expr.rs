use std::str::FromStr;

#[derive(Debug)]
pub enum NumType {
    I8,
    I16,
    I32,
    I64,
    I128,

    U8,
    U16,
    U32,
    U64,
    U128,

    F32,
    F64,

    None,
}

impl NumType {
    pub fn name(&self) -> &'static str {
        match self {
            NumType::I8 => "i8",
            NumType::I16 => "i16",
            NumType::I32 => "i32",
            NumType::I64 => "i64",
            NumType::I128 => "i128",
            NumType::U8 => "u8",
            NumType::U16 => "u16",
            NumType::U32 => "u32",
            NumType::U64 => "u64",
            NumType::U128 => "u128",
            NumType::F32 => "f32",
            NumType::F64 => "f64",
            NumType::None => "",
        }
    }
}

impl TryFrom<&str> for NumType {
    type Error = String;

    fn try_from(ty: &str) -> Result<Self, Self::Error> {

        match ty {
            "i8" => Ok(Self::I8),
            "i16" => Ok(Self::I16),
            "i32" => Ok(Self::I32),
            "i64" => Ok(Self::I64),
            "i128" => Ok(Self::I128),

            "u8" => Ok(Self::U8),
            "u16" => Ok(Self::U16),
            "u32" => Ok(Self::U32),
            "u64" => Ok(Self::U64),
            "u128" => Ok(Self::U128),

            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),

            "" => Ok(Self::None),

            v => Err(format!("'{}' is not a valid number type", v)),
        }
    }
}

#[derive(Debug)]
struct EnumMember {
    name: String,
    args: Vec<Type>,
}

#[derive(Debug)]
pub struct Enum {
    name: String,
    m: Vec<EnumMember>
}

#[derive(Debug)]
struct StructMember {
    name: String,
    ty: Type,
}

#[derive(Debug)]
pub struct Struct {
    name: String,
    m: Vec<StructMember>,
}

#[derive(Debug)]
pub struct Function {
    header: FnHeader,
    body: Block,
}

#[derive(Debug)]
struct FnArg {
    name: String,
    ty: Type,
    md: bool,
}

#[derive(Debug)]
pub struct FnHeader {
    name: String,
    args: Vec<FnArg>,
    ret_val: Option<Type>,
}

#[derive(Debug)]
pub struct Impl {
    /// implemented trait
    tr: Option<Trail>,
    /// implemented type
    ty: Type,
    /// implemented functions
    fns: Vec<Function>,
}

#[derive(Debug)]
pub struct Trait {
    /// name of the trait
    name: String,
    /// declared function headers
    fns: Vec<FnHeader>,
}

#[derive(Debug)]
pub struct Block {
    content: Vec<Stat>,
    return_value: Option<Expr>,
}

#[derive(Debug)]
pub struct Return {
    val: Expr,
}

#[derive(Debug)]
pub enum If {
    If(Expr, Block, Option<Box<If>>),
    Else(Block),
}

#[derive(Debug)]
pub struct Trail {
    head: String,
    trail: Vec<String>
}

#[derive(Debug)]
pub struct Match<T> {
    expr: Expr,
    cases: Vec<MatchBranch<T>>,
}

#[derive(Debug)]
struct MatchBranch<T> {
    case: MatchCase,
    block: T,
}

#[derive(Debug)]
enum MatchCase {
    /// Matches against literals / constants
    Literal(Expr),
    /// Match branch with multiple cases
    Multi(Box<MatchCase>, Vec<MatchCase>),
    /// Match against range
    Range(Expr, Expr),
    /// Match against data type with parameter list
    Data(Type, Vec<MatchCase>),
    Tuple(Vec<MatchCase>),
    /// Named parameter type
    Param(String),
}

#[derive(Debug)]
pub enum Expr {
    Literal(String),
    NumLit(String, NumType),
    FloatLit(String, NumType),
    CharLit(char),
    BoolLit(bool),

    Path(Trail),
    DotOp(Box<Expr>, String),
    Identifier(String),

    Assign(Box<Expr>, Box<Expr>),
    AssignAdd(Box<Expr>, Box<Expr>),
    AssignSub(Box<Expr>, Box<Expr>),
    AssignMul(Box<Expr>, Box<Expr>),
    AssignDiv(Box<Expr>, Box<Expr>),
    AssignMod(Box<Expr>, Box<Expr>),

    AssignAnd(Box<Expr>, Box<Expr>),
    AssignOr(Box<Expr>, Box<Expr>),
    AssignXor(Box<Expr>, Box<Expr>),
    AssignLShift(Box<Expr>, Box<Expr>),
    AssignRShift(Box<Expr>, Box<Expr>),

    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Ge(Box<Expr>, Box<Expr>),

    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Negate(Box<Expr>),

    LShift(Box<Expr>, Box<Expr>),
    RShift(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Inv(Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),

    LAnd(Box<Expr>, Box<Expr>),
    LOr(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),

    Unwrap(Box<Expr>),

    Call(Box<Expr>, Vec<Expr>),
    Block(Box<Block>),
    If(Box<If>),
    Match(Box<Match<Expr>>),
    Range(Box<Expr>, Box<Expr>),
    Index(Box<Expr>, Box<Expr>),
    ArrayInit(Box<Expr>, Box<Expr>),
    ArrayExplicit(Vec<Expr>),

    Ref(Box<Expr>),
    RefMut(Box<Expr>),
    Deref(Box<Expr>),
    Cast(Box<Expr>, Type),

    Tuple(Vec<Expr>),
    StructInit(Type, Vec<(String, Expr)>),
}

#[derive(Debug)]
pub enum Stat {
    Define(String, Box<Expr>, bool),
    ExprStat(Expr),
    Return(Return),
    Break(Option<Box<Expr>>),
    Continue,
    If(Box<If>),
    Match(Box<Match<Stat>>),
    While(Box<Expr>, Block),
    Loop(Block),
    For(Vec<String>, Box<Expr>, Block),
    Block(Box<Block>),
}

#[derive(Debug)]
pub enum Type {
    None,
    Plain(Trail),
    Ref(Box<Type>),
    MutRef(Box<Type>),
    Ptr(Box<Type>),
    MutPtr(Box<Type>),
    Tuple(Vec<Type>),
}


peg::parser!(pub grammar parser() for str {

    pub rule enum_def() -> Enum
        = _ "enum" _ name:identifier() _ "{" _ m:((_ mem:enum_member() _ {mem}) ** ",") _ "," _ "}" _ { Enum { name, m } }
        / _ "enum" _ name:identifier() _ "{" _ m:((_ mem:enum_member() _ {mem}) ** ",") _ "}" _ { Enum { name, m } }

    rule enum_member() -> EnumMember
        = name:identifier() _ "(" _ args:((_ t:ty() _ {t}) ** ",") _ ")" { EnumMember{ name, args } }
        / name:identifier() { EnumMember{ name, args: Vec::new() } }

    pub rule struct_def() -> Struct
        = _ "struct" _ name:identifier() _ "{" _ m:((_ field:identifier() _ ":" _ ty:ty() _ { StructMember { name: field, ty } }) ** ",") _ "," _ "}" _ { Struct { name, m } }
        / _ "struct" _ name:identifier() _ "{" _ m:((_ field:identifier() _ ":" _ ty:ty() _ { StructMember { name: field, ty } }) ** ",") _ "}" _ { Struct { name, m } }

    pub rule tr() -> Trait
        = _ "trait" _ name:identifier() _ "{" _ fns:((_ header:fn_header() _ ";" _ {header}) ** _) "}" _ { Trait { name, fns } }

    pub rule function() -> Function
        = _ header:fn_header() body:fn_block() { Function { header, body } }

    rule fn_header() -> FnHeader
        = "fn" _ name:identifier() _ "(" args:((_ arg:fn_arg() _ { arg }) ** ",") ")" _ "->" _ ret_val:ty() { FnHeader{ name, args, ret_val:Some(ret_val) } }
        / "fn" _ name:identifier() _ "(" args:((_ arg:fn_arg() _ { arg }) ** ",") ")" { FnHeader{ name, args, ret_val: None } }

    rule fn_arg() -> FnArg
        = "mut" _ name:identifier() _ ":" _ t:ty() { FnArg { name, ty: t, md: true } }
        / name:identifier() _ ":" _ t:ty() { FnArg { name, ty: t, md: false } }

    rule expression() -> Expr = precedence!{
        a:@ _ "&=" _ b:(@) { Expr::AssignAnd(Box::new(a), Box::new(b)) }
        a:@ _ "|=" _ b:(@) { Expr::AssignOr(Box::new(a), Box::new(b)) }
        a:@ _ "^=" _ b:(@) { Expr::AssignXor(Box::new(a), Box::new(b)) }
        a:@ _ "<<=" _ b:(@) { Expr::AssignLShift(Box::new(a), Box::new(b)) }
        a:@ _ ">>=" _ b:(@) { Expr::AssignRShift(Box::new(a), Box::new(b)) }
        --
        a:@ _ "=" _ b:(@) { Expr::Assign(Box::new(a), Box::new(b)) }
        a:@ _ "+=" _ b:(@) { Expr::AssignAdd(Box::new(a), Box::new(b)) }
        a:@ _ "-=" _ b:(@) { Expr::AssignSub(Box::new(a), Box::new(b)) }
        a:@ _ "*=" _ b:(@) { Expr::AssignMul(Box::new(a), Box::new(b)) }
        a:@ _ "/=" _ b:(@) { Expr::AssignDiv(Box::new(a), Box::new(b)) }
        a:@ _ "%=" _ b:(@) { Expr::AssignMod(Box::new(a), Box::new(b)) }
        --
        a:@ _ "||" _ b:(@) { Expr::LOr(Box::new(a), Box::new(b)) }
        --
        a:@ _ "&&" _ b:(@) { Expr::LAnd(Box::new(a), Box::new(b)) }
        --
        a:@ _ "==" _ b:(@) { Expr::Eq(Box::new(a), Box::new(b)) }
        a:@ _ "!=" _ b:(@) { Expr::Ne(Box::new(a), Box::new(b)) }
        a:@ _ "<"  _ b:(@) { Expr::Lt(Box::new(a), Box::new(b)) }
        a:@ _ ">"  _ b:(@) { Expr::Gt(Box::new(a), Box::new(b)) }
        a:@ _ "<=" _ b:(@) { Expr::Le(Box::new(a), Box::new(b)) }
        a:@ _ ">=" _ b:(@) { Expr::Ge(Box::new(a), Box::new(b)) }
        --
        a:@ _ "|" _ b:(@) { Expr::Or(Box::new(a), Box::new(b)) }
        --
        a:@ _ "^" _ b:(@) { Expr::Xor(Box::new(a), Box::new(b)) }
        --
        a:@ _ "&" _ b:(@) { Expr::And(Box::new(a), Box::new(b)) }
        --
        a:@ _ "<<" _ b:(@) { Expr::LShift(Box::new(a), Box::new(b)) }
        a:@ _ ">>" _ b:(@) { Expr::RShift(Box::new(a), Box::new(b)) }
        --
        a:@ _ "+" _ b:(@) { Expr::Add(Box::new(a), Box::new(b)) }
        a:@ _ "-" _ b:(@) { Expr::Sub(Box::new(a), Box::new(b)) }
        --
        a:@ _ "*" _ b:(@) { Expr::Mul(Box::new(a), Box::new(b)) }
        a:@ _ "/" _ b:(@) { Expr::Div(Box::new(a), Box::new(b)) }
        a:@ _ "%" _ b:(@) { Expr::Mod(Box::new(a), Box::new(b)) }
        --
        a:@ _ "as" _ ty:ty() { Expr::Cast(Box::new(a), ty) }
        --
        "-" _ a:@ { Expr::Negate(Box::new(a)) }
        "*" _ a:@ { Expr::Deref(Box::new(a)) }
        "!" _ a:@ { Expr::Not(Box::new(a)) }
        "~" _ a:@ { Expr::Inv(Box::new(a)) }
        "&" _ a:@ { Expr::Ref(Box::new(a)) }
        "&" _ "mut" _ a:@ { Expr::RefMut(Box::new(a)) }
        --
        a:@ _ "?" { Expr::Unwrap(Box::new(a)) }
        --
        n:num_lit() { n }
        --
        a:@ _ ".." _ b:(@) { Expr::Range(Box::new(a), Box::new(b)) }
        a:@ _ "[" _ index:expression() "]" { Expr::Index(Box::new(a), Box::new(index)) }
        --
        a:@ _ "." _ name:identifier() { Expr::DotOp(Box::new(a), name) }
        i:@ _ "(" args:((_ e:expression() _ {e}) ** ",") ")" { Expr::Call(Box::new(i), args) }
        "(" _ e:expression() _ ")" { e }
        "(" _ content:((_ e:expression() _ {e}) ++ ",") ")" { Expr::Tuple(content) }
        "(" _ content:((_ e:expression() _ {e}) ++ ",") "," _ ")" { Expr::Tuple(content) }
        "[" cont:((_ e:expression() _ {e}) ** ",") "]" { Expr::ArrayExplicit(cont) }
        "[" _ val:expression() _ ";" _ len:expression() _ "]" { Expr::ArrayInit(Box::new(val), Box::new(len)) }
        s:struct_init() { s }
        i:if_expr() { Expr::If(Box::new(i)) }
        m:match_expr() { Expr::Match(Box::new(m)) }
        e:exp_block() { Expr::Block(Box::new(e)) }
        b:bool_lit() { b }
        p:path() { Expr::Path(p) }
        l:str_lit() { l }
        l:char_lit() { l }
    }

    rule match_expr() -> Match<Expr>
        = "match" _ arg:expression() _ "{" _ branches:((_ case:match_case() _ "=>" _ expr:expression() _ {MatchBranch { case, block:expr }}) ** ",") _ "," _ "}" { Match{expr:arg, cases:branches} }
        / "match" _ arg:expression() _ "{" _ branches:((_ case:match_case() _ "=>" _ expr:expression() _ {MatchBranch { case, block:expr }}) ** ",") _ "}" { Match{expr:arg, cases:branches} }
        / "match" _ arg:expression() _ "{" _ branches:((_ case:match_case() _ "=>" _ expr:expression() _ {MatchBranch { case, block:expr }}) ** _) _ "}" { Match{expr:arg, cases:branches} }

    rule match_stat() -> Match<Stat>
        = "match" _ arg:expression() _ "{" _ branches:((_ case:match_case() _ "=>" _ expr:stat() _ {MatchBranch { case, block:expr }}) ** ",") _ "," _ "}" { Match{expr:arg, cases:branches} }
        / "match" _ arg:expression() _ "{" _ branches:((_ case:match_case() _ "=>" _ expr:stat() _ {MatchBranch { case, block:expr }}) ** ",") _ "}" { Match{expr:arg, cases:branches} }
        / "match" _ arg:expression() _ "{" _ branches:((_ case:match_case() _ "=>" _ expr:stat() _ {MatchBranch { case, block:expr }}) ** _) _ "}" { Match{expr:arg, cases:branches} }

    rule match_case() -> MatchCase = precedence!{
        t:ty() _ "(" _ params:((_ c:match_case() _ {c}) ** ",") _ ")" { MatchCase::Data( t, params ) }
        "(" _ params:((_ c:match_case() _ {c}) ** ",") _ ")" { MatchCase::Tuple( params ) }
        start:expression() _ ".." _ end:expression() { MatchCase::Range(start, end) }
        first:@ _ "|" _ cases:((_ c:match_case() _ {c}) ++ "|") { MatchCase::Multi(Box::new(first), cases) }
        name:identifier() { MatchCase::Param(name) }
        a:expression() { MatchCase::Literal(a) }
    }

    rule struct_init() -> Expr
        = t:ty() _ "{" _ args:((_ par:struct_parameter() _ {par}) ** ",") _ "," _ "}" { Expr::StructInit( t, args ) }
        / t:ty() _ "{" _ args:((_ par:struct_parameter() _ {par}) ** ",") _ "}" { Expr::StructInit( t, args ) }

    rule struct_parameter() -> (String, Expr)
        = name:identifier() _ ":" _ val:expression() { (name, val) }
        / name:identifier() { (name.clone(), Expr::Path(Trail { head: name, trail: Vec::new() })) }

    rule block() -> Block
        = "{" _ content:(( _ s:stat() _ {s}) ** _ ) _ "}" { Block{ content, return_value: None } }
        / exp_block()

    rule exp_block() -> Block
        = "{" _ content:(( _ s:stat() _ {s}) ** _ ) ret:expression() _ "}" { Block{content, return_value: Some(ret)} }

    pub rule fn_block() -> Block
        = _ "{" _ content:(( _ s:stat_noret() _ {s}) ** _) ret:expression() _ "}" _ { Block{content, return_value: Some(ret)} }
        / _ "{" _ content:(( _ s:stat_noret() _ {s}) ** _) ret:return_val() _ "}" _ { Block{content, return_value: Some(ret.val)} }
        / _ "{" _ content:(( _ s:stat_noret() _ {s}) ** _) _ "}" _ { Block{content, return_value: None} }

    rule while_stat() -> Stat
        = "while" _ arg:expression() _ block:block() { Stat::While(Box::new(arg), block) }

    rule for_stat() -> Stat
        = "for" _ arg:((_ i:identifier() _ {i})++ ",") _ "in" _ iter:expression() _ body:block() {Stat::For(arg, Box::new(iter), body)}

    rule if_expr() -> If
        = "if" _ arg:expression() _ block:exp_block() _ "else" _ el:exp_block() { If::If(arg, block, Some(Box::new(If::Else(el)))) }
        / "if" _ arg:expression() _ block:exp_block() _ "else" _ el:if_expr() { If::If(arg, block, Some(Box::new(el))) }

    rule if_stat() -> If
        = "if" _ arg:expression() _ block:block() _ "else" _ el:if_stat() { If::If(arg, block, Some(Box::new(el))) }
        / "if" _ arg:expression() _ block:block() _ "else" _ el:block() { If::If(arg, block, Some(Box::new(If::Else(el)))) }
        / "if" _ arg:expression() _ block:block() { If::If(arg, block, None) }

    rule stat() -> Stat
        = stat_noret()
        / return_stat()

    rule stat_noret() -> Stat
        = define()
        / break_stat()
        / while_stat()
        / for_stat()
        / e:block() { Stat::Block(Box::new(e)) }
        / i:if_stat() { Stat::If(Box::new(i)) }
        / m:match_stat() { Stat::Match(Box::new(m)) }
        / expr_stat()

    rule break_stat() -> Stat
        = "break" _ val:expression() _ ";" { Stat::Break(Some(Box::new(val))) }
        / "break" _ ";" { Stat::Break(None) }

    rule continue_stat() -> Stat
        = "continue" _ ";" { Stat::Continue }

    rule define() -> Stat
        = "let" _ "mut" _ name:identifier() _ "=" _ val:expression() _ ";" { Stat::Define(name, Box::new(val), true) }
        / "let" _ name:identifier() _ "=" _ val:expression() _ ";" { Stat::Define(name, Box::new(val), false) }

    rule expr_stat() -> Stat
        = ex:expression() _ ";" { Stat::ExprStat(ex) }

    rule return_val() -> Return
        = "return" _ val:expression() _ ";" { Return {val} }

    rule return_stat() -> Stat
        = val:return_val() { Stat::Return(val) }

    rule path() -> Trail
        = first:identifier() _ "::" _ path:((_ i:identifier() _ {i} ) ** "::") { Trail{ head: first, trail: path } }
        / first:identifier() { Trail{ head: first, trail: Vec::new() } }
        / expected!("path")

    rule ty() -> Type
        = "()" { Type::None }
        / "(" _ types:((_ t:ty() _ {t}) ++ ",") _ "," _ ")" { Type::Tuple(types) }
        / "(" _ types:((_ t:ty() _ {t}) ++ ",") _ ")" { Type::Tuple(types) }
        / "&" _ "mut" _ t:ty() { Type::MutRef(Box::new(t)) }
        / "&" _ t:ty() { Type::Ref(Box::new(t)) }
        / "*" _ "mut" _ t:ty() { Type::MutPtr(Box::new(t)) }
        / "*" _ t:ty() { Type::Ptr(Box::new(t)) }
        / p:path() { Type::Plain(p) }

    rule identifier() -> String
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() } }
        / expected!("identifier")

    rule str_lit() -> Expr
        = "\"" s:$(['a'..='z' | 'A'..='Z' | '0'..='9' | ' ' | '_' | '\n' | '\t' | '\r' | '@' |
        '\'' | '-' | '+' | '*' | '/' | '[' | ']' | '(' | ')' | '{' | '}' | '?' | '!' |
        '.' | ',' | ';' | ':' | '#' | '=' | '/' | '\\' | '%' | '$' | '§' | '~' | '>' | '<' |
        '|' | '`' | '^' | '°' | '€' ]*) "\"" { Expr::Literal( s.to_owned() ) }

    rule bool_lit() -> Expr
        = "true" { Expr::BoolLit(true) }
        / "false" { Expr::BoolLit(false) }

    rule char_lit() -> Expr
        = "\'" c:(['!'..='~' ]) "\'" { Expr::CharLit( c.to_owned() ) }

    rule num_type() -> String
        = ty:$(['a'..='z']['a'..='z' | '0'..='9']*) { ty.to_owned() }
        / "" { "".to_owned() }

    rule num_lit() -> Expr
        = n:$(['0'..='9' | '_']*) "." dot:$(['0'..='9' | '_']+) "e-" exp:$(['0'..='9' | '_']+) ty:num_type() { Expr::FloatLit( n.to_owned() + "." + dot + "e-" + exp, NumType::try_from(ty.as_ref()).unwrap() ) }
        / n:$(['0'..='9' | '_']*) "." dot:$(['0'..='9' | '_']+) "e+" exp:$(['0'..='9' | '_']+) ty:num_type() { Expr::FloatLit( n.to_owned() + "." + dot + "e+" + exp, NumType::try_from(ty.as_ref()).unwrap() ) }
        / n:$(['0'..='9' | '_']*) "." dot:$(['0'..='9' | '_']+) "e" exp:$(['0'..='9' | '_']+) ty:num_type() { Expr::FloatLit( n.to_owned() + "." + dot + "e" + exp, NumType::try_from(ty.as_ref()).unwrap() ) }
        / n:$(['0'..='9' | '_']*) "." dot:$(['0'..='9' | '_']+) ty:num_type() { Expr::FloatLit( n.to_owned() + "." + dot, NumType::try_from(ty.as_ref()).unwrap() ) }
        / "." dot:$(['0'..='9' | '_']+) ty:num_type() { Expr::FloatLit( "0.".to_owned() + dot, NumType::try_from(ty.as_ref()).unwrap() ) }
        / n:$(['0'..='9' | '_']*) "e" exp:$(['0'..='9' | '_']+) ty:num_type() { Expr::FloatLit( n.to_owned() + "e" + exp, NumType::try_from(ty.as_ref()).unwrap() ) }
        / n:$(['0'..='9' | '_']*) "e-" exp:$(['0'..='9' | '_']+) ty:num_type() { Expr::FloatLit( n.to_owned() + "e-" + exp, NumType::try_from(ty.as_ref()).unwrap() ) }
        / n:$(['0'..='9' | '_']*) "e+" exp:$(['0'..='9' | '_']+) ty:num_type() { Expr::FloatLit( n.to_owned() + "e+" + exp, NumType::try_from(ty.as_ref()).unwrap() ) }
        / n:$(['0'..='9' | '_']+) ty:num_type() { Expr::NumLit( n.to_owned(), NumType::try_from(ty.as_ref()).unwrap() ) }

    rule _() =  quiet!{[' ' | '\t' | '\n' | '\r']*}
});
