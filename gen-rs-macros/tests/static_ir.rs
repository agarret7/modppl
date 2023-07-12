
#[test]
fn test_repr() {
    let builder = StaticIRBuilder::new();
    let x = builder.add_constant_node();
    builder.set_return_node(x);
}