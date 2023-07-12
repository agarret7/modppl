pub fn codegen(ir: ModelIR) -> ToeknStream {
    let model_ident = ir.model_ident;

    quote! {
        let #model_ident: u32 = 1;
    }
}