var sourcesIndex = JSON.parse('{\
"console":["",[],["lib.rs"]],\
"core":["",[],["ctx.rs","lib.rs","result.rs","typecheck.rs","unify.rs"]],\
"data":["",[],["dec.rs","hash_map.rs","hash_set.rs","lib.rs"]],\
"lowering":["",[],["ctx.rs","ext.rs","imp.rs","lib.rs","result.rs","types.rs"]],\
"lsp_browser":["",[],["lib.rs"]],\
"lsp_server":["",[],["lib.rs"]],\
"parser":["",[["grammar",[],["mod.rs","util.rs"]]],["common.rs","cst.rs","lib.rs"]],\
"printer":["",[],["ast.rs","de_bruijn.rs","latex.rs","lib.rs","print_to_string.rs","theme.rs","tokens.rs","types.rs"]],\
"renaming":["",[],["ast.rs","ctx.rs","lib.rs"]],\
"source":["",[],["info.rs","lib.rs","result.rs"]],\
"syntax":["",[["ast",[["generic",[],["def.rs","fold.rs","imp.rs","map.rs","mod.rs","occurs.rs","visit.rs"]]],["mod.rs","subst.rs","typed.rs","untyped.rs"]]],["common.rs","cst.rs","de_bruijn.rs","equiv.rs","leveled_ctx.rs","lib.rs","matrix.rs","named.rs","tst.rs","ust.rs"]],\
"test_runner":["",[["cli",[],["mod.rs","run.rs"]]],["cases.rs","index.rs","infallible.rs","main.rs","phases.rs","runner.rs","suites.rs"]],\
"tracer":["",[],["lib.rs"]],\
"tracer_macros":["",[],["codegen.rs","lib.rs","parser.rs","syntax.rs"]],\
"xfunc":["",[["cli",[],["format.rs","ignore_colors.rs","mod.rs","prompt.rs","repl.rs","run.rs"]]],["main.rs","result.rs","rt.rs"]]\
}');
createSourceSidebar();
