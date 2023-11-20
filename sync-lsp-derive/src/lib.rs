use proc_macro::{TokenStream, Span};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Paren, Brace, Colon, Bracket, Semi, Eq};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, DeriveInput, Data, Fields, Ident, Meta, Expr, Lit, ExprLit,
    ExprAssign, ExprPath, Generics, Variant, Result, Error, PathSegment,
    FieldsUnnamed, Visibility, Pat, PatStruct, Path, PatTupleStruct, PatIdent,
    FieldPat, Member, Field, Type, TypeArray, LitInt, FieldMutability, PatSlice, ItemImpl, ImplItemType, TypeNever, TypePath, ImplItem,
};

#[proc_macro_attribute]
pub fn type_provider(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return Error::new(
            Span::call_site().into(),
            "The type_provider attribute does not take any arguments"
        ).to_compile_error().into()
    }

    let mut input = parse_macro_input!(input as ItemImpl);
    let mut types = Vec::new();

    let mut unit = Path {
        leading_colon: Some(Default::default()),
        segments: Punctuated::new()
    };

    unit.segments.push(PathSegment::from(Ident::new("sync_lsp", Span::call_site().into())));
    unit.segments.push(PathSegment::from(Ident::new("UnitType", Span::call_site().into())));

    let default = ImplItemType {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        type_token: Default::default(),
        ident: Ident::new("Default", Span::call_site().into()),
        eq_token: Eq::default(),
        ty: Type::Never(TypeNever {
            bang_token: Default::default()
        }),
        semi_token: Semi::default(),
        generics: Generics::default()
    };

    types.push(ImplItem::Type(ImplItemType {
        ident: Ident::new("Command", Span::call_site().into()),
        ty: Type::Path(TypePath {
            qself: None,
            path: {
                let mut unit = Path {
                    leading_colon: Some(Default::default()),
                    segments: Punctuated::new()
                };

                unit.segments.push(PathSegment::from(Ident::new("sync_lsp", Span::call_site().into())));
                unit.segments.push(PathSegment::from(Ident::new("workspace", Span::call_site().into())));
                unit.segments.push(PathSegment::from(Ident::new("execute_command", Span::call_site().into())));
                unit.segments.push(PathSegment::from(Ident::new("UnitCommand", Span::call_site().into())));
                unit
            }
        }),
        ..default.clone()
    }));

    types.push(ImplItem::Type(ImplItemType {
        ident: Ident::new("CodeLensData", Span::call_site().into()),
        ty: Type::Path(TypePath {
            qself: None,
            path: unit.clone()
        }),
        ..default.clone()
    }));

    types.push(ImplItem::Type(ImplItemType {
        ident: Ident::new("CompletionData", Span::call_site().into()),
        ty: Type::Path(TypePath {
            qself: None,
            path: unit.clone()
        }),
        ..default.clone()
    }));

    types.push(ImplItem::Type(ImplItemType {
        ident: Ident::new("Configuration", Span::call_site().into()),
        ty: Type::Path(TypePath {
            qself: None,
            path: unit.clone()
        }),
        ..default.clone()
    }));

    types.push(ImplItem::Type(ImplItemType {
        ident: Ident::new("InitializeOptions", Span::call_site().into()),
        ty: Type::Path(TypePath {
            qself: None,
            path: unit.clone()
        }),
        ..default.clone()
    }));

    types.push(ImplItem::Type(ImplItemType {
        ident: Ident::new("ShowMessageRequestData", Span::call_site().into()),
        ty: Type::Path(TypePath {
            qself: None,
            path: unit.clone()
        }),
        ..default.clone()
    }));

    types.push(ImplItem::Type(ImplItemType {
        ident: Ident::new("ApplyEditData", Span::call_site().into()),
        ty: Type::Path(TypePath {
            qself: None,
            path: unit.clone()
        }),
        ..default.clone()
    }));

    for item in input.items.iter() {
        let ImplItem::Type(item) = item else { continue };
        types.retain(|r#type| {
            let ImplItem::Type(r#type) = r#type else { return false };
            r#type.ident != item.ident
        });
    }

    input.items.extend_from_slice(&types);

    input.into_token_stream().into()
}

#[proc_macro_derive(Command, attributes(command))]
pub fn command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match input.data {
        Data::Struct(data) => {
            let variant = Variant {
                attrs: input.attrs,
                ident: Ident::new("Target", Span::call_site().into()),
                fields: data.fields,
                discriminant: None
            };
            impl_enum(input.ident, input.generics, vec![variant], None)
        },
        Data::Enum(data) => {
            for attr in input.attrs.iter() {
                let Meta::List(list) = &attr.meta else { continue };
                if list.path.is_ident("command") {
                    return Error::new(
                        attr.span(), 
                        "The command attribute is not supported in this position"
                    ).to_compile_error().into()
                }
            }

            let segment = PathSegment::from(Ident::new("Target", Span::call_site().into()));
            impl_enum(input.ident, input.generics, data.variants.into_iter().collect(), Some(segment))
        },
        Data::Union(..) => panic!("Command macro cannot be implemented on unions"),
    }
        .unwrap_or_else(|err| err.to_compile_error().into())
        .into()
}

fn impl_enum(ident: Ident, generics: Generics, variants: Vec<Variant>, segment: Option<PathSegment>) -> Result<TokenStream> {
    let arguments = Ident::new("Arguments", Span::call_site().into());
    let module = Ident::new(&format!("command_{}", ident.to_string().to_lowercase()), ident.span());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let where_clause_extension = where_clause.map(|where_clause| &where_clause.predicates);

    let commands = get_commands(variants.iter());
    let argument_variants = get_argument_variants(variants.iter());
    let variant_patterns = get_variant_patterns(segment, variants.iter(), false);
    let argument_patterns = get_variant_patterns(Some(PathSegment::from(arguments.clone())), argument_variants.iter(), true);
    let titles = variant_titles(variants.iter())?;

    Ok(quote!(
        #[allow(non_camel_case_types)]
        mod #module {
            use super::#ident as Target;
            use sync_lsp::workspace::execute_command::Command;
            use serde::{Serialize, Deserialize};
            use serde::de::{Deserializer, DeserializeOwned};
            use serde::ser::Serializer;

            impl #impl_generics Command for Target #ty_generics where Self: Clone, #arguments #ty_generics: Serialize + DeserializeOwned + Clone, #where_clause_extension {
                fn commands() -> Vec<String> {
                    let mut commands = Vec::new();
                    #(commands.push(#commands.to_string());)*
                    commands
                }

                fn serialize<__S__: Serializer>(&self, serializer: __S__) -> Result<__S__::Ok, __S__::Error> {
                    Title {
                        title: match &self {
                            #(#variant_patterns => #titles.to_string(),)*
                        },
                        arguments: <#arguments #ty_generics as From<_>>::from(self.clone())
                    }.serialize(serializer)
                }

                fn deserialize<'__de__, __D__: Deserializer<'__de__>>(deserializer: __D__) -> Result<Self, __D__::Error> {
                    <#arguments #ty_generics as Deserialize>::deserialize(deserializer).map(Into::into)
                }
            }

            impl #impl_generics From<Target #ty_generics> for #arguments #ty_generics #where_clause {
                fn from(command: Target #ty_generics) -> Self {
                    match command {
                        #(#variant_patterns => #argument_patterns,)*
                    }
                }
            }

            impl #impl_generics Into<Target #ty_generics> for #arguments #ty_generics #where_clause {
                fn into(self) -> Target #ty_generics {
                    match self {
                        #(#argument_patterns => #variant_patterns,)*
                    }
                }
            }

            #[derive(Serialize)]
            struct Title<T> {
                title: String,
                #[serde(flatten)]
                arguments: T
            }

            #[derive(Serialize, Deserialize, Clone)]
            #[serde(tag = "command", content = "arguments")]
            enum #arguments #generics {
                #(#argument_variants,)*
            }
        }
    ).into())
}

fn get_variant_patterns<'a>(segment: Option<PathSegment>, variants: impl Iterator<Item = &'a Variant>, cast_unit: bool) -> Vec<Pat> {
    let mut patterns = Vec::new();
    
    for variant in variants {

        let path = if let Some(segment) = segment.clone() {
            let mut path = Path::from(segment);
            path.segments.push(PathSegment::from(variant.ident.clone()));
            path
        } else {
            Path::from(PathSegment::from(variant.ident.clone()))
        };


        match variant.fields {
            Fields::Named(ref named) => {
                let mut fields = Punctuated::new();

                for (index, field) in named.named.iter().enumerate() {
                    fields.push(FieldPat {
                        attrs: Vec::new(),
                        member: Member::Named(field.ident.clone().unwrap()),
                        colon_token: Some(Colon::default()),
                        pat: Box::new(Pat::Ident(PatIdent {
                            attrs: Vec::new(),
                            by_ref: None,
                            mutability: None,
                            ident: Ident::new(&format!("m{index}"), Span::call_site().into()),
                            subpat: None
                        }))
                    })
                }

                patterns.push(Pat::Struct(PatStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path,
                    brace_token: Brace::default(),
                    fields,
                    rest: None
                }))
            },
            Fields::Unnamed(ref unnamed) => {
                let mut elems = Punctuated::new();

                for index in 0..unnamed.unnamed.len() {
                    elems.push(Pat::Ident(PatIdent {
                        attrs: Vec::new(),
                        by_ref: None,
                        mutability: None,
                        ident: Ident::new(&format!("m{index}"), Span::call_site().into()),
                        subpat: None
                    }));
                }
                
                if elems.len() == 1 && cast_unit {
                    let mut puncutated = Punctuated::new();
                    puncutated.push(elems.pop().unwrap().into_value());
                    elems.push(Pat::Slice(PatSlice {
                        attrs: Vec::new(),
                        bracket_token: Bracket::default(),
                        elems: puncutated
                    }))
                }

                patterns.push(Pat::TupleStruct(PatTupleStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path,
                    paren_token: Paren::default(),
                    elems
                }))
            },
            Fields::Unit => {
                patterns.push(Pat::Struct(PatStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path,
                    brace_token: Brace::default(),
                    fields: Punctuated::new(),
                    rest: None
                }))
            }
        }
    }

    patterns
}

fn get_argument_variants<'a>(variants: impl Iterator<Item = &'a Variant>) -> Vec<Variant> {
    variants.map(|variant| {
        let mut fields = variant.fields.clone();

        if let Fields::Named(named) = fields {
            fields = Fields::Unnamed(FieldsUnnamed {
                paren_token: Paren::default(),
                unnamed: named.named
            });
        }

        //Deserialisation will fail if the client omits arguments for a command with no arguments
        if let Fields::Unit = fields {
            fields = Fields::Unnamed(FieldsUnnamed {
                paren_token: Paren::default(),
                unnamed: Punctuated::new()
            });
        }

        //Cast commands with a single argument to an array
        if fields.len() == 1 {
            if let Fields::Unnamed(unnamed) = &mut fields {
                let first = unnamed.unnamed.pop().unwrap().into_value();
                unnamed.unnamed.push(Field {
                    attrs: Vec::new(),
                    vis: Visibility::Inherited,
                    mutability: FieldMutability::None,
                    ident: None,
                    colon_token: None,
                    ty: Type::Array(TypeArray {
                        bracket_token: Bracket::default(),
                        elem: Box::new(first.ty),
                        semi_token: Semi::default(),
                        len: Expr::Lit(ExprLit {
                            attrs: Vec::new(),
                            lit: Lit::Int(LitInt::new("1", Span::call_site().into()))
                        })
                    })
                })
            }
        }

        for field in fields.iter_mut() {
            field.attrs = Vec::new();
            field.ident = None;
            field.vis = Visibility::Inherited;
        }

        Variant {
            attrs: Vec::new(),
            ident: variant.ident.clone(),
            fields,
            discriminant: None
        }
    }).collect()
}

fn get_commands<'a>(variants: impl Iterator<Item = &'a Variant>) -> Vec<String> {
    variants.map(|variant| variant.ident.to_string()).collect()
}

fn variant_titles<'a>(variants: impl Iterator<Item = &'a Variant>) -> Result<Vec<String>> {
    let mut titles = Vec::new();
    
    for variant in variants {
        let mut title = variant.ident.to_string();

        for field in variant.fields.iter() {
            for attr in field.attrs.iter() {
                let Meta::List(list) = &attr.meta else { continue };
                if list.path.is_ident("command") {
                    return Err(Error::new(
                        attr.span(), 
                        "The command attribute is not supported on fields"
                    ))
                }
            }
        }

        for attribute in variant.attrs.iter() {
            let span = attribute.span();
            let Meta::List(list) = &attribute.meta else { continue };
            if !list.path.is_ident("command") { continue };
            let pair: ExprAssign = list.parse_args()?;

            let Expr::Path(ExprPath { path, .. }) = *pair.left else {
                return Err(Error::new(
                    span, 
                    "Expected a path for the left side of the command attribute")
                );
            };

            if !path.is_ident("title") {
                return Err(Error::new(
                    span, 
                    "The command attribute only supports the title field")
                );
            };

            let Expr::Lit(ExprLit { lit: Lit::Str(literal), .. }) = *pair.right else {
                return Err(Error::new(
                    span, 
                    "Expected a string literal for the right side of the command attribute")
                );
            };

            title = literal.value();
        }

        titles.push(title);
    }

    Ok(titles)
}