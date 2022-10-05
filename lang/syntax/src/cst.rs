use std::rc::Rc;

use super::common::*;
use super::de_bruijn::*;

#[derive(Debug, Clone)]
pub struct Prg {
    pub decls: Vec<Decl>,
    pub exp: Option<Rc<Exp>>,
}

#[derive(Debug, Clone)]
pub enum Decl {
    Data(Data),
    Codata(Codata),
    Def(Def),
    Codef(Codef),
}

#[derive(Debug, Clone)]
pub struct Data {
    pub name: Ident,
    pub params: Telescope,
    pub ctors: Vec<Ctor>,
}

#[derive(Debug, Clone)]
pub struct Codata {
    pub name: Ident,
    pub params: Telescope,
    pub dtors: Vec<Dtor>,
}

#[derive(Debug, Clone)]
pub struct Ctor {
    pub name: Ident,
    pub params: Telescope,
    pub typ: TypApp,
}

#[derive(Debug, Clone)]
pub struct Dtor {
    pub name: Ident,
    pub params: Telescope,
    pub on_typ: TypApp,
    pub in_typ: Rc<Exp>,
}

#[derive(Debug, Clone)]
pub struct Def {
    pub name: Ident,
    pub params: Telescope,
    pub on_typ: TypApp,
    pub in_typ: Rc<Exp>,
    pub body: Match,
}

#[derive(Debug, Clone)]
pub struct Codef {
    pub name: Ident,
    pub params: Telescope,
    pub typ: TypApp,
    pub body: Comatch,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub cases: Vec<Case>,
}

#[derive(Debug, Clone)]
pub struct Comatch {
    pub cases: Vec<Cocase>,
}

#[derive(Debug, Clone)]
pub struct Case {
    pub name: Ident,
    pub args: Telescope,
    pub eqns: EqnParams,
    /// Body being `None` represents an absurd pattern
    pub body: Option<Rc<Exp>>,
}

#[derive(Debug, Clone)]
pub struct Cocase {
    pub name: Ident,
    pub args: Telescope,
    pub eqns: EqnParams,
    /// Body being `None` represents an absurd pattern
    pub body: Option<Rc<Exp>>,
}

#[derive(Debug, Clone)]
pub struct Eqn {
    pub lhs: Rc<Exp>,
    pub rhs: Rc<Exp>,
}

#[derive(Debug, Clone)]
pub struct TypApp {
    pub name: Ident,
    pub args: Args,
}

#[derive(Debug, Clone)]
pub enum Exp {
    Call { name: Ident, args: Args },
    DotCall { exp: Rc<Exp>, name: Ident, args: Args },
    Anno { exp: Rc<Exp>, typ: Rc<Exp> },
    Type,
}

/// Wrapper type signifying the wrapped parameters have telescope
/// semantics. I.e. each parameter binding in the parameter list is in scope
/// for the following parameters. This influences the lowering semantic.
#[derive(Debug, Clone)]
pub struct Telescope(pub Params);

pub type Params = Vec<Param>;
pub type EqnParams = Vec<EqnParam>;
pub type Args = Vec<Rc<Exp>>;

#[derive(Debug, Clone)]
pub struct Param {
    pub name: Ident,
    pub typ: Rc<Exp>,
}

#[derive(Debug, Clone)]
pub struct EqnParam {
    pub name: Ident,
    pub eqn: Eqn,
}

#[derive(Debug, Clone)]
pub enum Var {
    Bound(Idx),
    Free(Ident),
}
