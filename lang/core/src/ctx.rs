//! Typechecking context

use std::fmt;
use std::rc::Rc;

use data::{HashMap, HashSet};
use syntax::ast;
use syntax::ast::Substitutable;
use syntax::common::*;
use syntax::de_bruijn::*;
use syntax::ust;
use tracer::trace;

#[derive(Debug, Clone)]
pub struct Ctx {
    types: HashMap<Ident, Rc<ust::TypAbs>>,
    ctors: HashMap<Ident, Rc<ust::Ctor>>,
    dtors: HashMap<Ident, Rc<ust::Dtor>>,
    type_for_xtor: HashMap<Ident, Ident>,
    xtors_in_type: HashMap<Ident, HashSet<Ident>>,
    bound: Vec<Vec<Rc<ust::Exp>>>,
}

impl Ctx {
    pub fn empty() -> Self {
        Self {
            types: HashMap::default(),
            ctors: HashMap::default(),
            dtors: HashMap::default(),
            type_for_xtor: HashMap::default(),
            xtors_in_type: HashMap::default(),
            bound: vec![],
        }
    }

    pub fn build(prg: &ust::Prg) -> Self {
        let mut types = HashMap::default();
        let mut ctors = HashMap::default();
        let mut dtors = HashMap::default();
        let mut type_for_xtor = HashMap::default();
        let mut xtors_in_type = HashMap::default();
        let mut xdefs_in_type: HashMap<Ident, HashSet<Ident>> = HashMap::default();
        let bound = vec![];

        for (decl_name, decl) in prg.decls.map.iter() {
            match decl {
                ust::Decl::Data(data) => {
                    types.insert(decl_name.clone(), data.typ.clone());
                    let mut xtors_set = HashSet::default();
                    for ctor in &data.ctors {
                        xtors_set.insert(ctor.clone());
                        type_for_xtor.insert(ctor.clone(), decl_name.clone());
                    }
                    xtors_in_type.insert(decl_name.clone(), xtors_set);
                }
                ust::Decl::Codata(codata) => {
                    types.insert(decl_name.clone(), codata.typ.clone());
                    let mut xtors_set = HashSet::default();
                    for dtor in &codata.dtors {
                        xtors_set.insert(dtor.clone());
                        type_for_xtor.insert(dtor.clone(), decl_name.clone());
                    }
                    xtors_in_type.insert(decl_name.clone(), xtors_set);
                }
                ust::Decl::Ctor(ctor) => {
                    ctors.insert(ctor.name.clone(), Rc::new(ctor.clone()));
                }
                ust::Decl::Dtor(dtor) => {
                    dtors.insert(dtor.name.clone(), Rc::new(dtor.clone()));
                }
                ust::Decl::Def(def) => {
                    dtors.insert(def.name.clone(), Rc::new(def.to_dtor()));
                    type_for_xtor.insert(def.name.clone(), def.on_typ.name.clone());
                    xdefs_in_type
                        .entry(def.on_typ.name.clone())
                        .or_default()
                        .insert(def.name.clone());
                }
                ust::Decl::Codef(codef) => {
                    ctors.insert(codef.name.clone(), Rc::new(codef.to_ctor()));
                    type_for_xtor.insert(codef.name.clone(), codef.typ.name.clone());
                    xdefs_in_type
                        .entry(codef.typ.name.clone())
                        .or_default()
                        .insert(codef.name.clone());
                }
            }
        }

        Self { types, ctors, dtors, type_for_xtor, xtors_in_type, bound }
    }

    #[trace("{} [ {} ] ~> {return:P}", self, idx, data::id)]
    pub fn bound(&self, idx: Idx) -> Rc<ust::Exp> {
        self.bound
            .get(self.bound.len() - 1 - idx.fst)
            .and_then(|ctx| ctx.get(ctx.len() - 1 - idx.snd))
            .expect("Unbound variable")
            .clone()
    }

    pub fn typ_for_xtor(&self, name: &Ident) -> &Ident {
        &self.type_for_xtor[name]
    }

    pub fn xtors_for_typ(&self, name: &Ident) -> &HashSet<Ident> {
        &self.xtors_in_type[name]
    }

    pub fn typ(&self, name: &Ident) -> Rc<ust::TypAbs> {
        self.types[name].clone()
    }

    pub fn ctor(&self, name: &Ident) -> Rc<ust::Ctor> {
        self.ctors[name].clone()
    }

    pub fn dtor(&self, name: &Ident) -> Rc<ust::Dtor> {
        self.dtors[name].clone()
    }

    /// Bind a single type
    pub fn bind<T, F>(&mut self, typ: Rc<ust::Exp>, f: F) -> T
    where
        F: Fn(&mut Ctx) -> T,
    {
        self.bind_fold([typ].into_iter(), (), |_, _, _| (), |ctx, _| f(ctx))
    }

    /// Bind an iterator `iter` of types
    ///
    /// Fold the iterator and consume the result
    /// under the inner context with all binders in scope.
    ///
    /// This is used for checking telescopes.
    ///
    /// * `iter` - An iterator of binders implementing `Typed`.
    /// * `acc` - Accumulator for folding the iterator
    /// * `f_acc` - Accumulator function run for each binder
    /// * `f_inner` - Inner function computing the final result under the context of all binders
    pub fn bind_fold<T, I: Iterator<Item = T>, O1, O2, F1, F2>(
        &mut self,
        iter: I,
        acc: O1,
        f_acc: F1,
        f_inner: F2,
    ) -> O2
    where
        T: Typed,
        F1: Fn(&mut Ctx, O1, T) -> O1,
        F2: FnOnce(&mut Ctx, O1) -> O2,
    {
        fn bind_inner<T, I: Iterator<Item = T>, O1, O2, F1, F2>(
            ctx: &mut Ctx,
            mut iter: I,
            acc: O1,
            f_acc: F1,
            f_inner: F2,
        ) -> O2
        where
            T: Typed,
            F1: Fn(&mut Ctx, O1, T) -> O1,
            F2: FnOnce(&mut Ctx, O1) -> O2,
        {
            match iter.next() {
                Some(x) => {
                    let x_t = x.typ();
                    let acc = f_acc(ctx, acc, x);
                    ctx.push(x_t);
                    let res = bind_inner(ctx, iter, acc, f_acc, f_inner);
                    ctx.pop();
                    res
                }
                None => f_inner(ctx, acc),
            }
        }

        self.level_inc_fst();
        let res = bind_inner(self, iter, acc, f_acc, f_inner);
        self.level_dec_fst();
        res
    }

    fn map<F>(&self, f: F) -> Self
    where
        F: Fn(&Rc<ust::Exp>) -> Rc<ust::Exp>,
    {
        let bound = self.bound.iter().map(|inner| inner.iter().map(&f).collect()).collect();
        // FIXME: inefficient clones
        Self {
            types: self.types.clone(),
            ctors: self.ctors.clone(),
            dtors: self.dtors.clone(),
            type_for_xtor: self.type_for_xtor.clone(),
            xtors_in_type: self.xtors_in_type.clone(),
            bound,
        }
    }

    fn shift(&mut self, by: (isize, isize)) {
        for lvl in 0..self.bound.len() {
            self.shift_at_lvl(lvl, by)
        }
    }

    fn shift_at_lvl(&mut self, lvl: usize, by: (isize, isize)) {
        for i in 0..self.bound[lvl].len() {
            self.bound[lvl][i] = self.bound[lvl][i].shift(by);
        }
    }

    /// Increment the first component of the current De-Bruijn level
    fn level_inc_fst(&mut self) {
        self.shift((1, 0));
        self.bound.push(vec![]);
    }

    /// Decrement the first component of the current De-Bruijn level
    fn level_dec_fst(&mut self) {
        self.bound.pop().unwrap();
        self.shift((-1, 0));
    }

    /// Push a binder contained in a binder list, incrementing the second dimension of the current De Bruijn level
    fn push(&mut self, typ: Rc<ust::Exp>) {
        self.bound.last_mut().expect("Cannot push without calling level_inc_fst first").push(typ);
        self.shift_at_lvl(self.bound.len() - 1, (0, 1));
    }

    /// Push a binder contained in a binder list, decrementing the second dimension of the current De Bruijn level
    fn pop(&mut self) {
        let err = "Cannot pop from empty context";
        self.bound.last_mut().expect(err).pop().expect(err);
        self.shift_at_lvl(self.bound.len() - 1, (0, -1));
    }
}

impl Leveled for Ctx {
    fn idx_to_lvl(&self, idx: Idx) -> Lvl {
        let fst = self.bound.len() - 1 - idx.fst;
        let snd = self.bound[fst].len() - 1 - idx.snd;
        Lvl { fst, snd }
    }

    fn lvl_to_idx(&self, lvl: Lvl) -> Idx {
        let fst = self.bound.len() - 1 - lvl.fst;
        let snd = self.bound[lvl.fst].len() - 1 - lvl.snd;
        Idx { fst, snd }
    }
}

impl Substitutable for Ctx {
    fn subst<L: Leveled, S: ast::Substitution>(&self, _lvl: &L, by: &S) -> Self {
        self.map(|exp| exp.subst(self, by))
    }
}

pub trait Typed {
    fn typ(&self) -> Rc<ust::Exp>;
}

impl Typed for Rc<ust::Exp> {
    fn typ(&self) -> Rc<ust::Exp> {
        self.clone()
    }
}

impl fmt::Display for Ctx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        use printer::PrintToString;
        let fmt_inner = |inner: &Vec<Rc<ust::Exp>>| {
            format!(
                "[{}]",
                inner.iter().map(|x| x.print_to_string()).collect::<Vec<_>>().join(", ")
            )
        };
        let s = self.bound.iter().map(fmt_inner).collect::<Vec<_>>().join(", ");
        write!(f, "[{}]", s)
    }
}