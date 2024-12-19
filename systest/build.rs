use std::env;

fn main() {
    let mut cfg = ctest::TestGenerator::new();
    if let Ok(out) = env::var("DEP_LZMA_INCLUDE") {
        cfg.include(&out);
    }

    cfg.header("lzma.h");
    cfg.type_name(|n, _s, _| n.to_string());
    cfg.define("LZMA_API_STATIC", None);
    cfg.skip_type(|n| n == "__enum_ty");
    cfg.field_name(|_s, field| if field == "type_" { "type" } else { field }.to_string());
    cfg.generate("../lzma-sys/src/lib.rs", "all.rs");
}
