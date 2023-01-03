use proc_macro::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

// `proc_macro_derive` indicates that this is a procedural macro and that
// `Builder` is its name
#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    // `.ident` is the name of the type/struct where we are applying
    // our derive macro. Example: in the tests, the `name` would be `Commander`
    let name_ident = ast.ident;

    // Here we interpolate the ident name + "Builder" to generate a struct
    // with the name `CommandBuilder`
    let builder_name = format_ident!("{}Builder", name_ident);

    let fields = get_struct_fields(&ast.data);
    dbg!(fields);

    // we then use `quote!` macro to output rust code
    let tokens = quote! {
        use std::error::Error;

        pub struct #builder_name {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #name_ident {
           pub fn builder() -> #builder_name {
                #builder_name {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
           }
        }



        impl #builder_name {
            // pub fn build(&mut self) -> Result<#name_ident, Box<dyn Error>> {
            //     // match #ast.data.fields {
            //     // }


            // }

            fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }

            fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }

            fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }

            fn current_dir(&mut self, current_dir: String) -> &mut Self {
                self.current_dir = Some(current_dir);
                self
            }
        }
    };

    // equivalent to TokenStream::from(tokens)
    tokens.into()
}

fn get_struct_fields(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                // let recurse = fields.named.iter().map(|f| {
                //     let name = &f.ident;

                // });
                let r = &fields.named.iter().map(|f| {
                    let name = &f.ident.unwrap();
                    name
                });
                todo!()
            }
            Fields::Unnamed(ref fields) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },

        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
