use syn::{Type, TypePath, PathArguments, GenericArgument};


pub fn ty_is_weak_trace_ref(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if path.segments.len() == 1 {
            let segment = &path.segments[0];
            if segment.ident == "Weak" {
                if let PathArguments::AngleBracketed(angle_bracketed) = &segment.arguments {
                    if angle_bracketed.args.len() == 1 {
                        if let Some(GenericArgument::Type(Type::Path(type_path))) = angle_bracketed.args.first() {
                            if type_path.path.segments.len() == 1 && type_path.path.segments[0].ident == "Trace" {
                                if let PathArguments::AngleBracketed(gen_args) = &type_path.path.segments[0].arguments {
                                    return gen_args.args.len() == 3;
                                }
                            } else if type_path.path.segments.len() == 1 && type_path.path.segments[0].ident == "DynTrace" {
                                if let PathArguments::AngleBracketed(gen_args) = &type_path.path.segments[0].arguments {
                                    return gen_args.args.len() == 2;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}
