var sourcesIndex = JSON.parse('{\
"app":["",[["cli",[],["format.rs","mod.rs","prompt.rs","repl.rs","run.rs","terminal.rs"]]],["main.rs","result.rs","rt.rs"]],\
"core":["",[],["ctx.rs","lib.rs","result.rs","typecheck.rs","unify.rs"]],\
"data":["",[],["dec.rs","hash_map.rs","hash_set.rs","lib.rs"]],\
"lowering":["",[],["ctx.rs","ext.rs","imp.rs","lib.rs","result.rs","types.rs"]],\
"lsp_browser":["",[],["lib.rs"]],\
"lsp_server":["",[],["lib.rs"]],\
"parser":["",[["grammar",[],["mod.rs","util.rs"]]],["common.rs","cst.rs","lib.rs"]],\
"printer":["",[],["ast.rs","de_bruijn.rs","lib.rs","print_to_string.rs","theme.rs","tokens.rs","types.rs"]],\
"source":["",[],["info.rs","lib.rs","result.rs"]],\
"syntax":["",[["ast",[],["def.rs","imp.rs","mod.rs","occurs.rs","subst.rs"]]],["common.rs","cst.rs","de_bruijn.rs","elab.rs","equiv.rs","lib.rs","named.rs"]],\
"tracer":["",[],["lib.rs"]],\
"tracer_macros":["",[],["codegen.rs","lib.rs","parser.rs","syntax.rs"]]\
}');
createSourceSidebar();
