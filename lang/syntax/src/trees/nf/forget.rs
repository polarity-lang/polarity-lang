use crate::common::*;
use crate::ust;

use super::def::*;

impl Forget for Nf {
    type Target = ust::Exp;

    fn forget(&self) -> Self::Target {
        match self {
            Nf::TypCtor { info, name, args } => ust::Exp::TypCtor {
                info: info.clone(),
                name: name.clone(),
                args: ust::Args { args: args.forget() },
            },
            Nf::Ctor { info, name, args } => ust::Exp::Ctor {
                info: info.clone(),
                name: name.clone(),
                args: ust::Args { args: args.forget() },
            },
            Nf::Type { info } => ust::Exp::Type { info: info.clone() },
            Nf::Comatch { info, name, is_lambda_sugar, body } => ust::Exp::Comatch {
                info: info.clone(),
                ctx: (),
                name: name.clone(),
                is_lambda_sugar: *is_lambda_sugar,
                body: body.forget(),
            },
            Nf::Neu { exp } => exp.forget(),
        }
    }
}

impl Forget for Neu {
    type Target = ust::Exp;

    fn forget(&self) -> Self::Target {
        match self {
            Neu::Var { info, name, idx } => {
                ust::Exp::Var { info: info.clone(), name: name.clone(), ctx: (), idx: *idx }
            }
            Neu::Dtor { info, exp, name, args } => ust::Exp::Dtor {
                info: info.clone(),
                exp: exp.forget(),
                name: name.clone(),
                args: ust::Args { args: args.forget() },
            },
            Neu::Match { info, name, on_exp, body } => ust::Exp::Match {
                info: info.clone(),
                ctx: (),
                name: name.clone(),
                on_exp: on_exp.forget(),
                motive: None,
                ret_typ: (),
                body: body.forget(),
            },
            Neu::Hole { info, kind } => ust::Exp::Hole { info: info.clone(), kind: *kind },
        }
    }
}

impl Forget for Match {
    type Target = ust::Match;

    fn forget(&self) -> Self::Target {
        let Match { info, cases } = self;

        ust::Match { info: info.clone(), cases: cases.forget() }
    }
}

impl Forget for Comatch {
    type Target = ust::Comatch;

    fn forget(&self) -> Self::Target {
        let Comatch { info, cases } = self;

        ust::Comatch { info: info.clone(), cases: cases.forget() }
    }
}

impl Forget for Case {
    type Target = ust::Case;

    fn forget(&self) -> Self::Target {
        let Case { info, name, args, body } = self;

        ust::Case {
            info: info.clone(),
            name: name.clone(),
            args: args.clone(),
            body: body.forget(),
        }
    }
}

impl Forget for Cocase {
    type Target = ust::Cocase;

    fn forget(&self) -> Self::Target {
        let Cocase { info, name, args, body } = self;

        ust::Cocase {
            info: info.clone(),
            name: name.clone(),
            params: args.clone(),
            body: body.forget(),
        }
    }
}

impl Forget for TypApp {
    type Target = ust::TypApp;

    fn forget(&self) -> Self::Target {
        let TypApp { info, name, args } = self;

        ust::TypApp {
            info: info.clone(),
            name: name.clone(),
            args: ust::Args { args: args.forget() },
        }
    }
}
