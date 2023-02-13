use regex::Regex;

fn map_type(input: &str, resource_name: &str) -> String {
    if input.eq(resource_name) {
        return capitalize_first_letter(resource_name);
    }

    match input {
        "u8" => "u8".to_owned(),
        "u16" => "u16".to_owned(),
        "u32" => "u32".to_owned(),
        "u64" => "u64".to_owned(),
        "s8" => "i8".to_owned(),
        "s16" => "i16".to_owned(),
        "s32" => "i32".to_owned(),
        "s64" => "i64".to_owned(),
        "float32" => "f32".to_owned(),
        "float64" => "f64".to_owned(),
        "char" => "char".to_owned(),
        "bool" => "bool".to_owned(),
        "string" => "String".to_owned(),
        "unit" => "()".to_owned(),
        "" => "".to_owned(),
        _ => {
            let re = Regex::new(r"^(list|option|expected)<(.*)>$").unwrap();
            if let Some(captures) = re.captures(input) {
                let prefix = match &captures[1] {
                    "list" => "Vec",
                    "option" => "Option",
                    "expected" => "Result",
                    _ => unreachable!(),
                };
                let inner = &captures[2];
                let re_expected = Regex::new(r"^(.*),(.*)$").unwrap();
                if let Some(captures_expected) = re_expected.captures(inner) {
                    format!(
                        "{}<{}, {}>",
                        prefix,
                        map_type(&captures_expected[1].trim(), resource_name),
                        map_type(&captures_expected[2].trim(), resource_name)
                    )
                } else {
                    format!("{}<{}>", prefix, map_type(inner, resource_name))
                }
            } else {
                "String".to_string()
            }
        }
    }
}

fn capitalize_first_letter(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn parse_resource_name_and_functions(
    text: &str,
) -> Result<(String, Vec<(String, bool, Vec<String>, String)>), &'static str> {
    let re = Regex::new(r#"resource\s*(\w+)\s*\{\s*([^}]+\s*)\}"#).unwrap();

    let captures = match re.captures(text) {
        Some(c) => c,
        None => return Err("Unable to parse resource name and functions"),
    };

    let resource_name = captures.get(1).unwrap().as_str().to_string();

    let function_list = captures.get(2).unwrap().as_str();
    let re_fn =
        Regex::new(r#"(static\s+)?([\w-]+):\s+func\s*\((.*?)\)\s*(->\s*([\w<>, ]+))?"#).unwrap();
    let functions: Vec<(String, bool, Vec<String>, String)> = re_fn
        .captures_iter(function_list)
        .map(|cap| {
            let is_static = cap.get(1).is_some();
            let name = cap[2].to_string().replace("-", "_");
            let args = cap[3]
                .split(",")
                .map(|arg| arg.trim().to_string())
                .collect();
            let return_type = cap
                .get(5)
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();
            (name, is_static, args, return_type)
        })
        .collect();

    Ok((resource_name, functions))
}

fn make_return_type(return_type: &str, resource_name: &str, src: &mut String) {
    // if return type is empty
    if return_type.is_empty() {
        src.push_str("\n        context.undefined_value()");
    }
    // else if return_type is unit
    else if return_type.eq("unit") {
        src.push_str("\n        context.null_value()");
    }
    // else if return_type is u8, u16, u32
    else if return_type.eq("u8") || return_type.eq("u16") || return_type.eq("u32") {
        src.push_str("\n        context.value_from_u32(res as u32)");
    }
    // else if return_type is u64
    else if return_type.eq("u64") {
        src.push_str("\n        context.value_from_u64(res as u64)");
    }
    // else if return_type is s8, s16, s32
    else if return_type.eq("s8") || return_type.eq("s16") || return_type.eq("s32") {
        src.push_str("\n        context.value_from_i32(res as i32)");
    }
    // else if return_type is s64
    else if return_type.eq("s64") {
        src.push_str("\n        context.value_from_i64(res as i64)");
    }
    // else if return_type is float32 or float 64
    else if return_type.eq("float32") || return_type.eq("float64") {
        src.push_str("\n        context.value_from_f64(res as f64)");
    }
    // else if return_type is bool
    else if return_type.eq("bool") {
        src.push_str("\n        let res_b = match res {");
        src.push_str("\n            0 => false,");
        src.push_str("\n            1 => true,");
        src.push_str("\n            _ => panic!(\"invalid bool discriminant\"),");
        src.push_str("\n        };");
        src.push_str("\n        context.value_from_bool(res_b)");
    }
    // else if return_type is char
    else if return_type.eq("char") {
        src.push_str("\n        context.value_from_str(&core::char::from_u32(res as u32).unwrap().to_string())");
    } else if return_type.eq("string") {
        src.push_str("\n        let ret_len = *((ret + 4) as *const i32) as usize;");
        src.push_str("\n        let res_s = String::from_utf8(Vec::from_raw_parts(*((ret + 0) as *const i32) as *mut _, ret_len, ret_len)).unwrap();");
        src.push_str("\n        context.value_from_str(res_s.as_str())");
    }
    // else if return_type contains option
    else if return_type.contains("option<") {
        // use regex to get the type inside option
        let re = Regex::new(r#"option<(\w+)>"#).unwrap();
        let captures = re.captures(return_type).unwrap();
        let inner_type = captures.get(1).unwrap().as_str();
        // add match i32::from(*((ret + 0) as *const u8)) {
        src.push_str("\n        match i32::from(*((ret + 0) as *const u8)) {");
        // add 0 => None,
        src.push_str("\n            0 => context.undefined_value(),");
        // add 1 => Some({make_inner_return_type(inner_type)}),
        src.push_str(
            format!(
                "\n            1 => {{{}",
                make_inner_return_type(inner_type, resource_name)
            )
            .as_str(),
        );
        src.push_str("\n            },");
        // add _ => panic!("invalid option discriminant"),
        src.push_str("\n            _ => panic!(\"invalid option discriminant\"),");
        // add }
        src.push_str("\n        }");
    // else if return_type contains expected
    } else if return_type.contains("expected<") {
        // use regex to get the type inside expected
        // write a regex that matches all of these:
        // "expected<u32, string>"
        // "expected<u64,list<u8>>"
        // "expected<s32, string>"
        // "expected<s64, string>"
        // "expected<float64, string>"
        // "expected<char, list<u8>>"
        // "expected<bool, list<u8>>"
        // "expected<string, string>"
        // "expected<list<string>, string>"
        // "expected<list<u8>, list<u8>>"

        let re =
            Regex::new(r#"expected<(\w+|list<\w+(?:,\s*\w+)*>),\s*(\w+|list<\w+(?:,\s*\w+)*>)>"#)
                .unwrap();
        let captures = re.captures(return_type).unwrap();
        let inner_type = captures.get(1).unwrap().as_str();
        let _error_type = captures.get(2).unwrap().as_str();
        // add match i32::from(*((ret + 0) as *const u8)) {
        src.push_str("\n        match i32::from(*((ret + 0) as *const u8)) {");
        // add 0 => Ok({make_inner_return_type(inner_type)}),
        src.push_str(
            format!(
                "\n            0 => {{{}",
                make_inner_return_type(inner_type, resource_name)
            )
            .as_str(),
        );
        src.push_str("\n            },");
        // add 1 => Err({make_inner_return_type(error_type)}),
        src.push_str("\n            1 => context.null_value(),");
        // add _ => panic!("invalid expected discriminant"),
        src.push_str("\n            _ => panic!(\"invalid expected discriminant\"),");
        // add }
        src.push_str("\n        }");
    }
    // else if return_type contains list<
    else if return_type.contains("list<") {
        // use regex to get the type inside list
        let re = Regex::new(r#"list<(\w+)>"#).unwrap();
        let captures = re.captures(return_type).unwrap();
        let inner_type = captures.get(1).unwrap().as_str();
        // add let ret_len = *((ret + 4) as *const i32) as usize;
        src.push_str("\n        let ret_len = *((ret + 4) as *const i32) as usize;");
        // add let res_v: Vec<{map_type(inner_type)}> = Vec::from_raw_parts(*((ret + 0) as *const i32) as *mut _, ret_len, ret_len);
        src.push_str(
            format!(
                "\n        let res_v: Vec<{}> = Vec::from_raw_parts(*((ret + 0) as *const i32) as *mut _, ret_len, ret_len);",
                map_type(inner_type, resource_name)
            )
            .as_str(),
        );
        // add let arr = context.array_value();
        src.push_str("\n        let arr = context.array_value().unwrap();");
        // add for i in 0..ret_len {
        src.push_str("\n        for i in 0..ret_len {");
        src.push_str(
            "\n            arr.append_property(context.value_from_str(&format!(\"{}\", res_v[i])).unwrap()).unwrap();",
        );
        // add }
        src.push_str("\n        }");

        // return arr
        src.push_str("\n        Ok(arr)");
    } else {
        src.push_str("\n        context.undefined_value()");
    }
}

fn make_inner_return_type(inner_type: &str, resource_name: &str) -> String {
    if inner_type.eq("unit") {
        "\n        context.null_value()".to_string()
    } else if inner_type.eq("u8") || inner_type.eq("u16") || inner_type.eq("u32") {
        "\n            context.value_from_u32(*((ret + 4) as *const i32) as u32)".to_string()
    } else if inner_type.eq("u64") {
        "\n            context.value_from_u64(*((ret + 8) as *const i64) as u64)".to_string()
    } else if inner_type.eq("s8")
        || inner_type.eq("s16")
        || inner_type.eq("s32")
        || inner_type.eq(resource_name)
    {
        "\n            context.value_from_i32(*((ret + 4) as *const i32))".to_string()
    } else if inner_type.eq("s64") {
        "\n            context.value_from_i64(*((ret + 8) as *const i64))".to_string()
    } else if inner_type.eq("float32") || inner_type.eq("float64") {
        "\n            context.value_from_f64(*((ret + 8) as *const f64))".to_string()
    } else if inner_type.eq("bool") {
        "\n            context.value_from_bool(match i32::from(*((ret + 1) as *const u8)) {\n                0 => false,\n                1 => true,\n                _ => panic!(\"invalid bool discriminant\"),\n})".to_string()
    } else if inner_type.eq("char") {
        "\n            context.value_from_str(&core::char::from_u32(*((ret + 4) as *const i32) as u32).unwrap().to_string())".to_string()
    } else if inner_type.eq("string") {
        "\n            let ret_len = *((ret + 8) as *const i32) as usize;\n            let res_s = String::from_utf8(Vec::from_raw_parts(*((ret + 4) as *const i32) as *mut _, ret_len, ret_len)).unwrap();\n            context.value_from_str(res_s.as_str())".to_string()
    } else if inner_type.contains("list<") {
        format!("{{\
\n              let ret_base = *((ret + 4) as *const i32);\
\n              let ret_len = *((ret + 8) as *const i32);\
\n              let mut res_v = Vec::with_capacity(ret_len as usize);\
\n              for i in 0..ret_len {{\
\n                  let base = ret_base + i * 8;\
\n                  res_v.push({{\
\n                    let len = *((base + 4) as *const i32) as usize;\
\n                    String::from_utf8(Vec::from_raw_parts(*((base + 0) as *const i32) as *mut _, len, len)).unwrap()\
\n                }});\
\n              }}\
\n              if ret_len != 0 {{\
\n                  std::alloc::dealloc(ret_base as *mut _, std::alloc::Layout::from_size_align((ret_len as usize) * 8, 4).unwrap());\
\n              }}\
\n              let arr = context.array_value().unwrap();\
\n              for i in 0..ret_len {{\
\n                  arr.append_property(context.value_from_str(&format!(\"{{}}\", res_v[i as usize])).unwrap()).unwrap();\
\n              }}\
\n              Ok(arr)\
\n}}", )
    } else {
        panic!("not implemented yet for {}", inner_type)
    }
}

pub fn generate_rust_code(text: &str) -> String {
    let (resource_name, functions) = parse_resource_name_and_functions(text).unwrap();

    let mut externs = format!("#![allow(warnings)]\nuse std::cell::RefCell;\nuse anyhow::Result;\nuse quickjs_wasm_rs::{{Context, Value}};\n\n");

    let resource_name_first_letter_capitalized = capitalize_first_letter(&resource_name);

    // add type for resource
    externs.push_str(&format!(
        "#[repr(align(4))]
struct __{}RetArea([u8; 16]);
static mut __{}_RET_AREA: __{}RetArea = __{}RetArea([0; 16]);

#[derive(Debug)]
#[repr(transparent)]
pub struct {}(i32);

",
        resource_name_first_letter_capitalized,
        resource_name.to_uppercase(),
        resource_name_first_letter_capitalized,
        resource_name_first_letter_capitalized,
        resource_name_first_letter_capitalized
    ));

    externs.push_str(&format!(
        "#[link(wasm_import_module = \"{}\")]\nextern \"C\" {{\n",
        resource_name
    ));

    for (name, is_static, args, return_type) in functions.clone() {
        externs = format!(
            "{}    #[cfg_attr(target_arch = \"wasm32\", link_name = \"{}::{}\")]\n    fn {}(",
            externs, resource_name, name, name
        );

        // if it's not static, we add a pointer arg for self
        if !is_static {
            externs = format!("{}_: i32, ", externs);
        }

        // if we have no args
        if !args.eq(&vec![""]) {
            // map args to rust types
            let args = args
                .iter()
                .map(|arg| {
                    let parts: Vec<&str> = arg.split(":").map(str::trim).collect();
                    let mapped_type = map_type(parts[1], &resource_name);
                    if mapped_type == "String" || mapped_type.starts_with("Vec<") {
                        format!("_: i32, _: i32")
                    } else if mapped_type == "bool" || mapped_type == "char" {
                        format!("_: i32")
                    } else if mapped_type == "u8"
                        || mapped_type == "u16"
                        || mapped_type == "u32"
                        || mapped_type == "u64"
                        || mapped_type == "i8"
                        || mapped_type == "i16"
                        || mapped_type == "i32"
                        || mapped_type == "i64"
                        || mapped_type == "f32"
                        || mapped_type == "f64"
                    {
                        format!("_: {}", mapped_type)
                    } else {
                        panic!("not supported")
                    }
                })
                .collect::<Vec<String>>();

            externs = format!("{}{}, ", externs, args.join(", "));
        }

        // if ret is empty, just close the function
        if return_type.is_empty() {
            externs = format!("{});\n", externs);
        }
        // else if ret is u8, u16, u32, s8, s16, or s32, add -> i32
        else if return_type == "u8"
            || return_type == "u16"
            || return_type == "u32"
            || return_type == "s8"
            || return_type == "s16"
            || return_type == "s32"
            || return_type == "bool"
            || return_type == "char"
        {
            externs = format!("{}) -> i32;\n", externs);
        }
        // else if ret is u64, or s64, add -> i64
        else if return_type == "u64" || return_type == "s64" {
            externs = format!("{}) -> i64;\n", externs);
        }
        // else if ret is f32, add -> f32
        else if return_type == "float32" {
            externs = format!("{}) -> f32;\n", externs);
        }
        // else if ret is f64, add -> f64
        else if return_type == "float64" {
            externs = format!("{}) -> f64;\n", externs);
        }
        // else add another argument i32 for the return value pointer
        else {
            externs = format!("{}_: i32);\n", externs);
        }
    }

    // close extern def
    externs = format!("{}}}\n\n", externs);

    let mut fxns = String::new();

    for (function_name, is_static, args, return_type) in functions.clone() {
        fxns.push_str(&format!("fn {}_{}(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {{\n", resource_name, function_name));

        fxns.push_str("    unsafe {\n");

        // if return type is not u8, u16, u32, s8, s16, s32, u64, s64, f32, f64, char, or bool, we need to allocate a return area
        if !return_type.is_empty()
            && return_type != "u8"
            && return_type != "u16"
            && return_type != "u32"
            && return_type != "s8"
            && return_type != "s16"
            && return_type != "s32"
            && return_type != "u64"
            && return_type != "s64"
            && return_type != "float32"
            && return_type != "float64"
            && return_type != "char"
            && return_type != "bool"
        {
            fxns.push_str(&format!(
                "        let ret = __{}_RET_AREA.0.as_mut_ptr() as i32;\n",
                resource_name.to_uppercase()
            ));
        }

        // if there are args...
        if !args.eq(&vec![""]) {
            // split arg name and type
            let args: Vec<(String, String)> = args
                .into_iter()
                .map(|arg_str| {
                    let parts: Vec<&str> = arg_str.split(":").map(|s| s.trim()).collect();
                    (parts[0].trim().to_string(), parts[1].trim().to_string())
                })
                .collect();

            // if it is not static, grab pointer to self
            if !is_static {
                fxns.push_str("        let self0: i32 = args[0].as_i32_unchecked();\n");
            }

            for (arg_name, arg_type) in args.clone() {
                // replace "-" w/ "_" in arg name
                let arg_name = arg_name.replace("-", "_");

                // if it is not static, we need to add 1 to the position of the arg
                // to account for the self pointer
                let pos = if is_static {
                    args.iter()
                        .position(|(name, _)| &name.replace("-", "_") == &arg_name)
                        .unwrap()
                } else {
                    args.iter()
                        .position(|(name, _)| &name.replace("-", "_") == &arg_name)
                        .unwrap()
                        + 1
                };

                // get variable def. for args
                fxns.push_str(&format!(
                    "        let {}: {} = args[{}].{};\n{}",
                    arg_name,
                    match arg_type.as_str() {
                        "u8" | "u16" | "u32" | "s8" | "s16" | "s32" | "char" | "bool" | "u64"
                        | "s64" | "float32" | "float64" | "string" =>
                            map_type(&arg_type, &resource_name),
                        _ => {
                            // all lists are now parsed as bytes
                            if arg_type.contains("list<") {
                                "Vec<u8>".to_string()
                            } else {
                                format!("&RefCell<{}>", map_type(&arg_type, &resource_name))
                            }
                        }
                    },
                    pos,
                    match arg_type.as_str() {
                        "u8" | "u16" | "u32" | "s8" | "s16" | "s32" => format!(
                            "as_i32_unchecked() as {}",
                            map_type(&arg_type, &resource_name)
                        ),
                        "bool" => "as_u32_unchecked() != 0".to_string(),
                        "char" => "as_u32_unchecked() as u8 as char".to_string(),
                        "u64" => "as_big_int_unchecked().unwrap().as_u64()".to_string(),
                        "s64" => "as_big_int_unchecked().unwrap().as_i64()".to_string(),
                        "float32" => "as_f64_unchecked() as f32".to_string(),
                        "float64" => "as_f64_unchecked()".to_string(),
                        "string" => "as_str().unwrap().to_string()".to_string(),

                        _ => {
                            if arg_type.contains("list<") {
                                "as_str_lossy().as_bytes().to_vec()".to_string()
                            } else {
                                "get_rust_value().unwrap()".to_string()
                            }
                        }
                    },
                    match arg_type.as_str() {
                        "u8" | "u16" | "u32" | "s8" | "s16" | "s32" | "char" | "bool" | "u64"
                        | "s64" | "float32" | "float64" | "string" => "".to_string(),
                        _ =>
                            if !arg_type.contains("list<") {
                                format!(
                                    "        let {}: {} = {}.take();\n",
                                    arg_name,
                                    map_type(&arg_type, &resource_name),
                                    arg_name
                                )
                            } else {
                                "".to_string()
                            },
                    },
                ));

                // continue with arg processing
                if arg_type == "string" || arg_type.contains("list") {
                    fxns.push_str(&format!(
                        "        let {}_ptr: i32 = {}.as_ptr() as i32;\n",
                        arg_name, arg_name
                    ));
                    fxns.push_str(&format!(
                        "        let {}_len: i32 = {}.len() as i32;\n",
                        arg_name, arg_name
                    ));
                }
            }

            // if !is_static add "self0, " to the beginning of the args of fxn call
            if !is_static {
                fxns.push_str(&format!("\n        let res = {}(self0, ", function_name));
            } else {
                fxns.push_str(&format!("\n        let res = {}(", function_name));
            }

            // get other args for fxn call
            fxns.push_str(&format!(
                "{}",
                args.iter()
                    .map(|(arg_name, arg_type)| {
                        let arg_name = arg_name.replace("-", "_");
                        if arg_type == "string" || arg_type.contains("list") {
                            format!("{}_ptr, {}_len", arg_name, arg_name)
                        } else if arg_type == "char" || arg_type == "bool" {
                            format!("{} as i32", arg_name)
                        } else {
                            format!("{}", arg_name)
                        }
                        // currently, we are not supporting arguments of type "expected", or "option"
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            ));

            // add final comma to arg list
            fxns.push_str(", ");
        } else {
            // if there are no args
            if !is_static {
                fxns.push_str("        let self0: i32 = args[0].as_i32_unchecked();\n");
            }
            // if is_static add "self0, " to the beginning of the args
            if !is_static {
                fxns.push_str(&format!("\n        let res = {}(self0, ", function_name));
            } else {
                fxns.push_str(&format!("\n        let res = {}(", function_name));
            }
        }

        // if return type is not u8, u16, u32, s8, s16, s32, u64, s64, f32, f64, char, or bool, we need to allocate a return area
        if !return_type.is_empty()
            && return_type != "u8"
            && return_type != "u16"
            && return_type != "u32"
            && return_type != "s8"
            && return_type != "s16"
            && return_type != "s32"
            && return_type != "u64"
            && return_type != "s64"
            && return_type != "float32"
            && return_type != "float64"
            && return_type != "char"
            && return_type != "bool"
        {
            // add final item
            fxns.push_str("ret");
        }

        // close the function call
        fxns.push_str(");\n");

        make_return_type(&return_type, &resource_name, &mut fxns);

        //close unsafe
        fxns.push_str("\n   }\n");

        // close the function
        fxns.push_str("}\n\n");
    }

    let mut src = format!("{}{}", externs, fxns);

    // create dynamic_context_injection function that gets called at init
    src.push_str(
        "pub fn dynamic_context_injection(context: &Context, global: &Value) -> Result<()> {\n",
    );

    // context object to inject onto
    src.push_str(&format!(
        "    let {} = context.object_value()?;\n",
        resource_name
    ));

    // create js callbacks for every fxn we generated
    for (function_name, _, _, _) in functions.clone() {
        src.push_str(&format!(
            "    {}.set_property(\"{}\", context.wrap_callback({}_{})?)?;\n",
            resource_name, function_name, resource_name, function_name
        ));
    }

    // add the context object to the global object
    src.push_str(&format!(
        "    global.set_property(\"{}\", {})?;\n",
        resource_name, resource_name
    ));

    src.push_str("    Ok(())\n}\n");

    return src;
}
