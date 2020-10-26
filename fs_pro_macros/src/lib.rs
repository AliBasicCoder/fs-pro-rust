extern crate proc_macro;
extern crate syn;
use proc_macro::TokenStream;
use proc_macro_error::{abort, emit_error, proc_macro_error};
use syn::parse_macro_input;

fn vec_compare(arr1: &Vec<String>, arr2: &[&str]) -> bool {
  if arr1.len() != arr2.len() {
    return false;
  }
  let mut i = 0;
  while i < arr1.len() {
    let a = &arr1[i];
    let b = &arr2[i];
    if a != b {
      return false;
    }
    i += 1;
  }
  true
}

fn vec_compare2(arr1: &Vec<String>, arr2: &[&str], arr3: &[&str]) -> bool {
  if vec_compare(arr1, arr2) || vec_compare(arr1, arr3) {
    return true;
  }
  false
}

fn join(arr: &Vec<String>, with: &str) -> String {
  let mut result = String::new();
  let mut i = 0;
  for item in arr {
    result.push_str(item.as_str());
    if i != arr.len() - 1 {
      result.push_str(with);
    }
    i += 1;
  }
  result
}

fn valid_pattern(pattern: &String) -> bool {
  let pattern = pattern.replace(".", "\\.").replace("*", ".*");
  let regex = regex::Regex::new(pattern.as_str());
  if let Err(_) = regex {
    false
  } else {
    true
  }
}

type ParseResult = Vec<(
  String,
  usize,
  Option<String>,
  Option<String>,
  Option<Vec<String>>,
)>;

fn to_string(result: ParseResult, struct_name: String) -> String {
  let mut shape_describe = String::from("[");
  let mut shape_new_top = String::new();
  let mut shape_new_bottom = String::new();
  let mut shape_new_file = String::new();
  let mut shape_new_shaped_dir = String::new();
  let mut shape_new_dir = String::new();
  let mut i: usize = 0;
  for (filed_name, filed_type, file_name, pattern, target_struct_name) in &result {
    if *filed_type == 0 {
      shape_describe.push_str(
        format!(
          "File(\"{}\", \"{}\")",
          filed_name,
          file_name.as_ref().unwrap_or(&filed_name)
        )
        .as_str(),
      );
      shape_new_top.push_str(format!("let mut {}: Option<File> = None;\n", filed_name).as_str());

      shape_new_file.push_str(format!("{} => {} = Some(file),\n", i, filed_name).as_str());
    }
    if *filed_type == 1 {
      shape_describe.push_str(
        format!(
          "DirectoryPattern(\"{}\", \"{}\", \"{}\")",
          filed_name,
          file_name.as_ref().unwrap_or(&filed_name),
          pattern.as_ref().unwrap()
        )
        .as_str(),
      );
      shape_new_top.push_str(format!("let mut {}: Option<Dir> = None;\n", filed_name).as_str());

      shape_new_dir.push_str(format!("{} => {} = Some(dir),\n", i, filed_name).as_str());
    }
    if *filed_type == 2 {
      let struct_path = join(target_struct_name.as_ref().unwrap(), "::");
      shape_describe.push_str(
        format!(
          "DirectorySchema(\"{}\", \"{}\", {}::shape_describe())",
          filed_name,
          file_name.as_ref().unwrap_or(&filed_name),
          struct_path
        )
        .as_str(),
      );
      shape_new_top
        .push_str(format!("let mut {}: Option<{}> = None;\n", filed_name, struct_path).as_str());
      shape_new_shaped_dir.push_str(
        format!(
          "{} => {} = Some({}::shape_new(dir)),\n",
          i, filed_name, struct_path
        )
        .as_str(),
      );
    }
    i += 1;
    shape_describe.push_str(",");
    shape_new_bottom.push_str(format!("{0}: {0}.unwrap(),\n", filed_name).as_str());
  }
  shape_describe.push_str("]");
  format!(
    r#"
  #[doc(hidden)]
  impl ::fs_pro::shape::ShapeDescribe for {} {{
    fn shape_describe() -> &'static ::fs_pro::shape::ShapeSchemaStatic<'static> {{
       use ::fs_pro::shape::ShapeItemStatic::*;
       use ::fs_pro::shape::ShapeDescribe;
       use ::lazy_static::lazy_static;
       lazy_static! {{
         static ref shape: [::fs_pro::shape::ShapeItemStatic<'static>; {}] = {};
       }}
       &*shape
    }}
    fn shape_new(inst: ::fs_pro::shape::ShapeInst) -> Self {{
       use ::fs_pro::shape::ShapeInstItem;
       use ::core::option::Option;
       {}
       let mut i: usize = 0;
       for item in inst {{
          match item {{
            ShapeInstItem::File(file) => match i {{
              {}
              _ => {{}}
            }}
            ShapeInstItem::Directory(dir) => match i {{
              {}
              _ => {{}}
            }}
            ShapeInstItem::ShapedDirectory(dir) => match i {{
              {}
              _ => {{}}
            }}
          }}
          i += 1;
       }}
       Self {{
          {}
       }}
    }}
  }}
  "#,
    struct_name,
    result.len(),
    shape_describe,
    shape_new_top,
    shape_new_file,
    shape_new_dir,
    shape_new_shaped_dir,
    shape_new_bottom
  )
}

#[proc_macro_derive(Shape, attributes(name, pattern))]
#[proc_macro_error]
pub fn derive_helper_attr(item: TokenStream) -> TokenStream {
  let input = parse_macro_input!(item as syn::DeriveInput);
  let regex = regex::Regex::new(" *= *\"([^/\"\\\\]*)\"").unwrap();
  let mut result: ParseResult = vec![];
  #[allow(unused_assignments)]
  let mut struct_name = String::new();
  if let syn::Data::Struct(strc) = &input.data {
    struct_name = input.ident.to_string();
    if let syn::Fields::Named(fileds) = &strc.fields {
      for item in &fileds.named {
        match &item.vis {
          syn::Visibility::Public(_) => {}
          _ => emit_error!(item, "all fileds must be public"),
        }
        let filed_name = (&item.ident).as_ref().unwrap().to_string();
        let mut file_name = None;
        let mut pattern = None;
        let mut target_struct_name = None;
        let mut filed_type: usize = 0;
        if let syn::Type::Path(t) = &item.ty {
          let mut actual: Vec<String> = vec![];
          for seg in &t.path.segments {
            match seg.arguments {
              syn::PathArguments::None => {}
              _ => emit_error!(t, "types cannot have arguments"),
            }
            actual.push(seg.ident.to_string());
          }
          if vec_compare2(&actual, &["File"], &["fs_pro", "File"]) {
            filed_type = 0;
          } else if vec_compare2(&actual, &["Dir"], &["fs_pro", "Dir"]) {
            filed_type = 1;
          } else {
            target_struct_name = Some(actual);
            filed_type = 2;
          }
        } else {
          emit_error!(
            item.ty,
            "a field can be only File, Dir or a struct implementing Shape"
          );
        }
        for attr in &item.attrs {
          if let syn::AttrStyle::Inner(_) = &attr.style {
            unimplemented!()
          }
          if attr.path.is_ident("name") {
            let st = attr.tokens.to_string();
            if !regex.is_match(st.as_str()) {
              emit_error!(attr, "syntax error in name");
            }
            file_name = Some(
              regex
                .captures(st.as_str())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string(),
            );
          } else if attr.path.is_ident("pattern") {
            if filed_type != 1 {
              emit_error!(attr, "pattern can only be used with Dir fields");
            }
            let st = attr.tokens.to_string();
            if !regex.is_match(st.as_str()) {
              emit_error!(attr, "syntax error in pattern");
            }
            pattern = Some(
              regex
                .captures(st.as_str())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string(),
            );
            if !valid_pattern((&pattern).as_ref().unwrap()) {
              emit_error!(attr, "invlaid pattern")
            }
          } else {
            if !attr.path.is_ident("doc") {
              emit_error!(attr, "unknown attribute");
            }
          }
        }
        if filed_type == 1 {
          if let None = pattern {
            emit_error!(item, "Dir fields must have pattern attribute");
          }
        }
        result.push((
          filed_name,
          filed_type,
          file_name,
          pattern,
          target_struct_name,
        ));
      }
    } else {
      abort!(input, "fields must be named")
    }
  } else {
    abort!(input, "Shape can be only used on struct(s)")
  }
  to_string(result, struct_name)
    .parse::<TokenStream>()
    .unwrap()
}
