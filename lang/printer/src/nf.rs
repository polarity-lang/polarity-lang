use pretty::DocAllocator;

use syntax::nf::*;

use super::theme::ThemeExt;
use super::tokens::*;
use super::types::*;
use super::util::*;

impl<'a> Print<'a> for Nf {
    fn print(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        match self {
            Nf::TypCtor { info: _, name, args: subst } => {
                alloc.typ(name).append(subst.print(cfg, alloc).opt_parens())
            }
            Nf::Ctor { info: _, name, args: subst } => {
                alloc.ctor(name).append(subst.print(cfg, alloc).opt_parens())
            }
            Nf::Type { info: _ } => alloc.typ(TYPE),
            Nf::Comatch { info: _, name, body } => alloc
                .keyword(COMATCH)
                .append(alloc.space())
                .append(alloc.text(name))
                .append(alloc.space())
                .append(body.print(cfg, alloc)),
            Nf::Neu { exp } => exp.print(cfg, alloc),
        }
    }
}

impl<'a> Print<'a> for Neu {
    fn print(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        match self {
            Neu::Var { info: _, name, idx } => alloc.text(format!("{name}@{idx}")),
            Neu::Dtor { info: _, exp, name, args: subst } => exp
                .print(cfg, alloc)
                .append(DOT)
                .append(alloc.dtor(name))
                .append(subst.print(cfg, alloc).opt_parens()),
            Neu::Match { info: _, name, on_exp, body } => on_exp
                .print(cfg, alloc)
                .append(DOT)
                .append(alloc.keyword(MATCH))
                .append(alloc.space())
                .append(alloc.text(name))
                .append(alloc.space())
                .append(body.print(cfg, alloc)),
        }
    }
}
impl<'a> Print<'a> for Comatch {
    fn print(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let Comatch { info: _, cases } = self;
        let sep = alloc.text(COMMA).append(alloc.hardline());

        alloc
            .hardline()
            .append(alloc.intersperse(cases.iter().map(|x| x.print(cfg, alloc)), sep))
            .nest(INDENT)
            .append(alloc.hardline())
            .braces_from(cfg)
    }
}

impl<'a> Print<'a> for Match {
    fn print(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let Match { info: _, cases } = self;
        let sep = alloc.text(COMMA).append(alloc.hardline());
        alloc
            .hardline()
            .append(alloc.intersperse(cases.iter().map(|x| x.print(cfg, alloc)), sep))
            .nest(INDENT)
            .append(alloc.hardline())
            .braces_from(cfg)
    }
}

impl<'a> Print<'a> for Case {
    fn print(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let Case { info: _, name, args, body } = self;

        let body = match body {
            None => alloc.keyword(ABSURD),
            Some(body) => alloc
                .text(FAT_ARROW)
                .append(alloc.line())
                .append(body.print(cfg, alloc))
                .nest(INDENT),
        };

        alloc.ctor(name).append(args.print(cfg, alloc)).append(alloc.space()).append(body).group()
    }
}

impl<'a> Print<'a> for Cocase {
    fn print(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let Cocase { info: _, name, args, body } = self;

        let body = match body {
            None => alloc.keyword(ABSURD),
            Some(body) => alloc
                .text(FAT_ARROW)
                .append(alloc.line())
                .append(body.print(cfg, alloc))
                .nest(INDENT),
        };

        alloc.ctor(name).append(args.print(cfg, alloc)).append(alloc.space()).append(body).group()
    }
}