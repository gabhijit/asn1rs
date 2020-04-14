extern crate proc_macro;

use asn1rs::gen::rust::RustCodeGenerator as RustGenerator;
use asn1rs::gen::Generator;
use asn1rs::model::Model;
use asn1rs::parser::Tokenizer;
use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::LitStr;

#[proc_macro]
pub fn asn_to_rust(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as LitStr).value();
    let tokens = Tokenizer::default().parse(&input);
    let model = Model::try_from(tokens).unwrap();

    let mut generator = RustGenerator::default();
    generator.add_model(model.to_rust());

    generator
        .to_string()
        .unwrap()
        .into_iter()
        .map(|(_file, content)| content)
        .collect::<Vec<_>>()
        .join("\n")
        .parse()
        .unwrap()
}
