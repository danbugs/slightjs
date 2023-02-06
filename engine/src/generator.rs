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

fn make_return_type(return_type: &str, resource_name: &str, result: &mut String, is_wrapped_in_err: bool) {
    // if return type contains "expected" then we need to make a match on the result
    if return_type.contains("expected") {
        // add match i32::from(*((ret + 0) as *const u8)) {
        result.push_str(&format!(
            "    match i32::from(*((ret + 0) as *const u8)) {{\n"
        ));
        result.push_str(&format!("      0 => Ok("));

        // expected looks like: expected<a, b>, get type a:
        let re = Regex::new(r#"expected<(.+),(.+)>"#).unwrap();
        let captures = re.captures(return_type).unwrap();
        let a = captures.get(1).unwrap().as_str().trim();

        // recursive call on a
        let _ = &make_return_type(a, resource_name, result, false);

        result.push_str(&format!("),\n"));

        // get type b:
        let b = captures.get(2).unwrap().as_str().trim();

        // recursive call on b
        result.push_str(&format!("      1 => anyhow::bail!("));
        let _ = &make_return_type(b, resource_name, result, true);
        result.push_str(&format!("),\n"));

        // panic otherwise
        result.push_str(&format!("      _ => panic!(\"Invalid expected value\"),\n"));

        // close match
        result.push_str(&format!("    }}\n"));
    } else if return_type.contains("option") {
        result.push_str(&format!(
            "    match i32::from(*((ret + 0) as *const u8)) {{\n"
        ));
        result.push_str(&format!("      0 => None,\n"));
        result.push_str(&format!("      1 => Some("));
        // panic otherwise
        result.push_str(&format!("panic!(\"Invalid option value\")"));

        // option looks like: option<a>, get type a:
        let re = Regex::new(r#"option<(.+)>"#).unwrap();
        let captures = re.captures(return_type).unwrap();
        let a = captures.get(1).unwrap().as_str().trim();

        // recursive call on a
        let _ = &make_return_type(a, resource_name, result, false);

        result.push_str(&format!("),\n"));

        // close match
        result.push_str(&format!("    }}\n"));
    } else if return_type.eq(resource_name) {
        result.push_str(&format!(
            "context.wrap_rust_value({}(*((ret + 4) as *const i32))).unwrap()",
            capitalize_first_letter(resource_name)
        ));
    } else if return_type.contains("list") {
        // add {
        result.push_str(&format!("{{\n"));

        result.push_str(&format!("      let base = *((ret + 4) as *const i32);\n"));
        result.push_str(&format!("      let len = *((ret + 8) as *const i32);\n"));
        result.push_str(&format!("      let mut result = Vec::with_capacity(len as usize);\n"));
        result.push_str(&format!("      for i in 0..len {{\n"));
        result.push_str(&format!("          let base_i = base + i *8;\n"));
        result.push_str(&format!("          result.push({{\n"));
        result.push_str(&format!("          let len_i = *((base_i + 4) as *const i32) as usize;\n"));
        result.push_str(&format!("          String::from_utf8(Vec::from_raw_parts(*((base_i + 0) as *const i32) as *mut _, len_i, len_i)).unwrap()\n"));
        result.push_str(&format!("          }});\n"));
        result.push_str(&format!("      }}\n"));
        result.push_str(&format!("      if len != 0 {{\n"));
        result.push_str(&format!("          std::alloc::dealloc(base as *mut _, std::alloc::Layout::from_size_align_unchecked((len as usize) * 8, 4));\n"));
        result.push_str(&format!("      }}\n"));
        result.push_str(&format!("      context.wrap_rust_value(result).unwrap()\n"));

        // close }
        result.push_str(&format!("}}\n"));
    } else {
        match return_type {
            "unit" => {
                if !is_wrapped_in_err {
                    result.push_str(&format!("context.undefined_value().unwrap(),\n"));
                } else {
                    result.push_str(&format!("(),\n"));
                }
            }
            "u8" | "u16" | "u32" | "s8" | "s16" | "s32" | "u64" | "s64" | "char" | "bool" => {
                if !is_wrapped_in_err {
                    result.push_str(&format!("context.value_from_i64(i64::from(*((ret + 4) as *const i64))).unwrap()"));
                } else {
                    result.push_str(&format!("i64::from(*((ret + 4) as *const i64))"));
                }
            }
            "float32" | "float64" => {
                // if wrapped_in_err, don't wrap in context.value_from_*
                if !is_wrapped_in_err {
                    result.push_str(&format!("context.value_from_f64(f64::from(*((ret + 4) as *const f64))).unwrap()"));
                } else {
                    result.push_str(&format!("f64::from(*((ret + 4) as *const f64))"));
                }
            }
            "string" => {
                if !is_wrapped_in_err {
                    result.push_str(&format!("context.value_from_str(String::from_utf8(Vec::from_raw_parts(*((ret + 0) as *const i32) as *mut _, *((ret + 4) as *const i32) as usize, *((ret + 4) as *const i32) as usize).unwrap().as_str()).unwrap()"));
                } else {
                    result.push_str(&format!("String::from_utf8(Vec::from_raw_parts(*((ret + 0) as *const i32) as *mut _, *((ret + 4) as *const i32) as usize, *((ret + 4) as *const i32) as usize)).unwrap()"));
                }
            }
            _ => {
                panic!("Unknown return type: {}", return_type)
            }
        }
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

pub fn generate_rust_code(text: &str) -> String {
    let (resource_name, functions) = parse_resource_name_and_functions(text).unwrap();
    dbg!(&resource_name, &functions);

    let mut externs = format!("use std::cell::RefCell;\nuse anyhow::Result;\nuse quickjs_wasm_rs::{{Context, Value}};\n\n");

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

    for (name, is_static, args, _) in functions.clone() {
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

        // add another argument i32 for the return value pointer
        externs = format!("{}_: i32);\n", externs);
    }

    // close extern def
    externs = format!("{}}}\n\n", externs);

    let mut fxns = String::new();

    for (function_name, is_static, args, return_type) in functions.clone() {
        fxns.push_str(&format!("fn {}_{}(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {{\n", resource_name, function_name));

        fxns.push_str("    unsafe {\n");

        // get return area pointer
        fxns.push_str(&format!(
            "    let ret = __{}_RET_AREA.0.as_mut_ptr() as i32;\n",
            resource_name.to_uppercase()
        ));

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
                fxns.push_str("    let self0: i32 = args[0].as_i32_unchecked();\n");
            }

            for (arg_name, arg_type) in args.clone() {

                // if it is not static, we need to add 1 to the position of the arg
                // to account for the self pointer
                let pos = if is_static {
                    args.iter().position(|(name, _)| name == &arg_name).unwrap()
                } else {
                    args.iter().position(|(name, _)| name == &arg_name).unwrap() + 1
                };

                // get variable def. for args
                fxns.push_str(&format!(
                    "    let {}: {} = args[{}].{};\n{}",
                    arg_name,
                    match arg_type.as_str() {
                        "u8" | "u16" | "u32" | "s8" | "s16" | "s32" | "char" | "bool" | "u64"
                        | "s64" | "float32" | "float64" | "string" =>
                            map_type(&arg_type, &resource_name),
                        _ => format!("&RefCell<{}>", map_type(&arg_type, &resource_name)),
                    },
                    pos,
                    match arg_type.as_str() {
                        "u8" | "u16" | "u32" | "s8" | "s16" | "s32" | "char" | "bool" =>
                            "as_i32_unchecked()",
                        "u64" | "s64" => "as_big_int_unchecked()",
                        "float32" | "float64" => "as_f64()",
                        "string" => "as_str().unwrap().to_string()",
                        _ => "get_rust_value().unwrap()",
                    },
                    match arg_type.as_str() {
                        "u8" | "u16" | "u32" | "s8" | "s16" | "s32" | "char" | "bool" | "u64"
                        | "s64" | "float32" | "float64" | "string" => "".to_string(),
                        _ => format!(
                            "    let {}: {} = {}.take();\n",
                            arg_name,
                            map_type(&arg_type, &resource_name),
                            arg_name
                        ),
                    },
                ));

                // continue with arg processing
                if arg_type == "char" || arg_type == "bool" {
                    fxns.push_str(&format!(
                        "    let {}_i32: i32 = wit_bindgen_rust::rt::as_i32({});\n",
                        arg_name, arg_name
                    ));
                } else if arg_type == "string" || arg_type.contains("list") {
                    fxns.push_str(&format!(
                        "    let {}_ptr: i32 = {}.as_ptr() as i32;\n",
                        arg_name, arg_name
                    ));
                    fxns.push_str(&format!(
                        "    let {}_len: i32 = {}.len() as i32;\n",
                        arg_name, arg_name
                    ));
                }
            }

            // if !is_static add "self0, " to the beginning of the args of fxn call
            if !is_static {
                fxns.push_str(&format!("\n    {}(self0, ", function_name));
            } else {
                fxns.push_str(&format!("\n    {}(", function_name));
            }

            // get other args for fxn call
            fxns.push_str(&format!(
                "{}",
                args.iter()
                    .map(|(arg_name, arg_type)| {
                        if arg_type == "char" || arg_type == "bool" {
                            format!("{}0_i32", arg_name)
                        } else if arg_type == "string" || arg_type.contains("list") {
                            format!("{}_ptr, {}_len", arg_name, arg_name)
                        } else {
                            format!("{}.take()", arg_name)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        } else { // if there are no args
            if !is_static {
                fxns.push_str("    let self0: i32 = args[0].as_i32_unchecked();\n");
            }
            // if is_static add "self0, " to the beginning of the args
            if !is_static {
                fxns.push_str(&format!("\n    {}(self0 ", function_name));
            } else {
                fxns.push_str(&format!("\n    {}(", function_name));
            }
        }
        // add final item
        fxns.push_str(", ret");

        let clean_return_type = return_type.replace("->", "").trim().to_string();
        // close the function call
        fxns.push_str(");\n");

        make_return_type(&clean_return_type, &resource_name, &mut fxns, false);

        //close unsafe
        fxns.push_str("   }\n");

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
