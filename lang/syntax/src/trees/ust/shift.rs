use crate::common::*;

use super::def::*;

impl ShiftInRange for Exp {
    fn shift_in_range<R: ShiftRange>(&self, range: R, by: (isize, isize)) -> Self {
        match self {
            Exp::Var { info, name, ctx: (), idx } => Exp::Var {
                info: info.clone(),
                name: name.clone(),
                ctx: (),
                idx: idx.shift_in_range(range, by),
            },
            Exp::TypCtor { info, name, args } => Exp::TypCtor {
                info: info.clone(),
                name: name.clone(),
                args: args.shift_in_range(range, by),
            },
            Exp::Ctor { info, name, args } => Exp::Ctor {
                info: info.clone(),
                name: name.clone(),
                args: args.shift_in_range(range, by),
            },
            Exp::Dtor { info, exp, name, args } => Exp::Dtor {
                info: info.clone(),
                exp: exp.shift_in_range(range.clone(), by),
                name: name.clone(),
                args: args.shift_in_range(range, by),
            },
            Exp::Anno { info, exp, typ } => Exp::Anno {
                info: info.clone(),
                exp: exp.shift_in_range(range.clone(), by),
                typ: typ.shift_in_range(range, by),
            },
            Exp::Type { info } => Exp::Type { info: info.clone() },
            Exp::Match { info, name, on_exp, motive, ret_typ, body } => Exp::Match {
                info: info.clone(),
                name: name.clone(),
                on_exp: on_exp.shift_in_range(range.clone(), by),
                motive: motive.shift_in_range(range.clone(), by),
                ret_typ: ret_typ.shift_in_range(range.clone(), by),
                body: body.shift_in_range(range, by),
            },
            Exp::Comatch { info, name, is_lambda_sugar, body } => Exp::Comatch {
                info: info.clone(),
                name: name.clone(),
                is_lambda_sugar: *is_lambda_sugar,
                body: body.shift_in_range(range, by),
            },
            Exp::Hole { info, kind } => Exp::Hole { info: info.clone(), kind: *kind },
        }
    }
}

impl ShiftInRange for Motive {
    fn shift_in_range<R: ShiftRange>(&self, range: R, by: (isize, isize)) -> Self {
        let Motive { info, param, ret_typ } = self;

        Motive {
            info: info.clone(),
            param: param.clone(),
            ret_typ: ret_typ.shift_in_range(range.shift(1), by),
        }
    }
}

impl ShiftInRange for Match {
    fn shift_in_range<R: ShiftRange>(&self, range: R, by: (isize, isize)) -> Self {
        let Match { info, cases } = self;
        Match { info: info.clone(), cases: cases.shift_in_range(range, by) }
    }
}

impl ShiftInRange for Comatch {
    fn shift_in_range<R: ShiftRange>(&self, range: R, by: (isize, isize)) -> Self {
        let Comatch { info, cases } = self;
        Comatch { info: info.clone(), cases: cases.shift_in_range(range, by) }
    }
}

impl ShiftInRange for Case {
    fn shift_in_range<R: ShiftRange>(&self, range: R, by: (isize, isize)) -> Self {
        let Case { info, name, args, body } = self;
        Case {
            info: info.clone(),
            name: name.clone(),
            args: args.clone(),
            body: body.shift_in_range(range.shift(1), by),
        }
    }
}

impl ShiftInRange for Cocase {
    fn shift_in_range<R: ShiftRange>(&self, range: R, by: (isize, isize)) -> Self {
        let Cocase { info, name, params: args, body } = self;
        Cocase {
            info: info.clone(),
            name: name.clone(),
            params: args.clone(),
            body: body.shift_in_range(range.shift(1), by),
        }
    }
}

impl ShiftInRange for TypApp {
    fn shift_in_range<R: ShiftRange>(&self, range: R, by: (isize, isize)) -> Self {
        let TypApp { info, name, args } = self;
        TypApp { info: info.clone(), name: name.clone(), args: args.shift_in_range(range, by) }
    }
}

impl ShiftInRange for Args {
    fn shift_in_range<R: ShiftRange>(&self, range: R, by: (isize, isize)) -> Self {
        Self { args: self.args.shift_in_range(range, by) }
    }
}