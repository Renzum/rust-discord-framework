use syn::spanned::Spanned as _;

enum CommandKind {
    Slash,
    Message,
    User,
}
enum CommandArg {
    Name,
    Description,
    Type(CommandKind),
    Unknown,
}

pub struct CommandArgs {
    name: String,
    description: Option<String>,
    kind: CommandKind,
}

pub fn get_description(meta_vec: &Vec<syn::Meta>) -> std::option::Option<String> {
    for meta in meta_vec {
        if let syn::Meta::NameValue(name) = meta {
            if name.path.segments.len() == 1 && name.path.segments[0].ident == "description" {
                if let syn::Expr::Lit(syn::ExprLit{
                    lit: syn::Lit::Str(str), ..
                }) = &name.value {
                    return Some(str.value());
                }
            }
        };
    }

    None
}