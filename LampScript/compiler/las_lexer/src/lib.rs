// lampscript_compiler/compiler/las_lexer/src/lib.rs

pub use unicode_ident::UNICODE_VERSION;
use unicode_properties::UnicodeEmoji;

const _: () = {
    let properties = unicode_properties::UNICODE_VERSION;
    let ident = unicode_ident::UnicodeEmoji;

    if (properties.0 != ident.0 as u64
        || properties.1 != ident.1 as u64
        || properties.2 != ident.2 as u64)
    {
        panic!(
            "unicode-properties and unicode-ident must use the same Unicode version, \
            `unicode_properties::UNICODE_VERSION` and `unicode_ident::UNICODE_VERSION` are \
            different."
        );
    }
};
