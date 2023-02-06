const { parse } = require("./grammar");
const fs = require("fs");
const path = require("path");

let path_and_file = process.argv[2];
let bindings = bindgen(path_and_file);
let file_name = path_and_file.split("/").pop().split(".")[0];

fs.writeFileSync(path.join(__dirname, `${file_name}.d.ts`), bindings);

function bindgen(input_file) {
    const example_input = fs.readFileSync(path.join(__dirname, input_file), "utf8");
    let ast = parse(example_input);
    let ts_wit_bindgen = ast_to_js(ast);
    return ts_wit_bindgen;
}

function ast_to_js(ast) {
    let res = "";
    for (let i = 0; i < ast.length; i++) {
        let obj = ast[i];
        let token = Object.keys(obj)[0];
        switch (token) {
            case "whitespace":
                break;
            case "comment":
                break;
            case "keyword":
                let keyword = Object.keys(obj[token])[0];
                switch (keyword) {
                    case "use_item":
                        res += "/* `use` is not currently supported */\n";
                        break;
                    case "type":
                        let type_alias = obj[token][keyword].alias.replace(/-/g, "_");
                        let type_ty = obj[token][keyword].ty;
                        res += "export type " + type_alias + " = " + witTypeToTsType(type_ty) + ";\n\n";
                        break;
                    case "record_item":
                        let record_name = obj[token][keyword].name.replace(/-/g, "_");
                        let record_fields = obj[token][keyword].fields;
                        res += "export interface " + record_name + " {\n";
                        for (let j = 0; j < record_fields.length; j++) {
                            let property_name = record_fields[j].property_name.replace(/-/g, "_");
                            let property_type = record_fields[j].property_type;
                            res += "    " + property_name + ": " + witTypeToTsType(property_type) + ";\n";
                        }
                        res += "}\n\n";
                        break;
                    case "flags_item":
                        let flags_name = obj[token][keyword].name.replace(/-/g, "_");
                        let flags_fields = obj[token][keyword].fields;
                        res += "export enum " + flags_name + " {\n";
                        for (let j = 0; j < flags_fields.length; j++) {
                            let field_name = flags_fields[j].replace(/-/g, "_");
                            res += "    " + field_name + ",\n";
                        }
                        res += "}\n\n";
                        break;
                    case "variant_item":
                        let variant_name = obj[token][keyword].name.replace(/-/g, "_");
                        let variant_fields = obj[token][keyword].fields;
                        res += "export type " + variant_name + " = ";
                        for (let j = 0; j < variant_fields.length; j++) {
                            let case_name = variant_fields[j].value.replace(/-/g, "_");
                            let case_ty = variant_fields[j].ty;
                            res += "{ type: \"" + case_name + "\", value: " + witTypeToTsType(case_ty) + " }";
                            if (j < variant_fields.length - 1) {
                                res += "\n| ";
                            }
                        }
                        res += ";\n\n";
                        break;
                    case "enum_item":
                        let enum_name = obj[token][keyword].name.replace(/-/g, "_");
                        let enum_cases = obj[token][keyword].cases;
                        res += "export enum " + enum_name + " {\n";
                        for (let j = 0; j < enum_cases.length; j++) {
                            let case_name = enum_cases[j].replace(/-/g, "_");
                            res += "    " + case_name + ",\n";
                        }
                        res += "}\n\n";
                        break;
                    case "union_item":
                        let union_name = obj[token][keyword].name.replace(/-/g, "_");
                        let union_cases = obj[token][keyword].cases;
                        res += "export type " + union_name + " = ";
                        for (let j = 0; j < union_cases.length; j++) {
                            let case_ty = witTypeToTsType(union_cases[j]);
                            res += case_ty;
                            if (j < union_cases.length - 1) {
                                res += " | ";
                            }
                        }
                        res += ";\n\n";
                        break;
                    case "func_item":
                        let func_signature = generateFuncSignature(false, obj[token][keyword]);
                        res += func_signature;
                        break;
                    case "resource_item":
                        let resource_name = obj[token][keyword].name.replace(/-/g, "_");
                        let resource_contents = obj[token][keyword].contents;
                        res += "export interface " + resource_name + " {\n";

                        if (resource_contents) {
                            for (let j = 0; j < resource_contents.length; j++) {
                                let content = resource_contents[j];
                                if (content.stat) {
                                    res += "    static ";
                                }
                                if (content.fn) {
                                    let func_signature = generateFuncSignature(true, content.fn.func_item);
                                    res += func_signature;
                                }
                                if (content.name) {
                                    let func_signature = "    " + generateFuncSignature(true, content);
                                    res += func_signature;
                                }
                            }
                        }
                        res += "}\n\n";
                        break;
                    default:
                        throw new Error("Unknown keyword: " + keyword);
                        break;
                }
                break;
            default:
                throw new Error("Unknown token: " + token);
        }
    }

    return res;
}


function witTypeToTsType(witType) {
    if (typeof witType === "object") {
        let containerType = Object.keys(witType)[0];
        switch (containerType) {
            case "tuple":
                let tupleTypes = witType[containerType];
                let tupleTypesStr = "";
                for (let i = 0; i < tupleTypes.length; i++) {
                    tupleTypesStr += witTypeToTsType(tupleTypes[i]);
                    if (i < tupleTypes.length - 1) {
                        tupleTypesStr += ", ";
                    }
                }
                return "[" + tupleTypesStr + "]";
            case "list":
                return witTypeToTsType(witType[containerType]) + "[]";
            case "option":
                return witTypeToTsType(witType[containerType]) + " | null";
            case "expected":
                let resultTypes = witType[containerType];
                return witTypeToTsType(resultTypes.ok) + " | " + witTypeToTsType(resultTypes.err);
            default:
                throw new Error(`Unsupported containerType: ${containerType}`);
        }
    } else if (typeof witType === "string") {
        switch (witType) {
            case "u8":
                return "number";
            case "u16":
                return "number";
            case "u32":
                return "number";
            case "u64":
                return "number";
            case "s8":
                return "number";
            case "s16":
                return "number";
            case "s32":
                return "number";
            case "s64":
                return "number";
            case "float32":
                return "number";
            case "float64":
                return "number";
            case "char":
                return "string";
            case "bool":
                return "boolean";
            case "string":
                return "string";
            case "unit":
                return "void";
            default:
                return witType.replace(/-/g, "_");
        }
    }
}

function generateFuncSignature(isResource, func) {
    let func_name = func.name.replace(/-/g, "_");
    let func_args = func.args;
    let func_ret = func.ret;
    let func_async = func.async;

    // iterate over func args if they exist
    let func_args_str = "";
    if (func_args) {
        if (!Array.isArray(func_args)) {
            func_args = [func_args];
        }

        for (let j = 0; j < func_args.length; j++) {
            let arg_name = func_args[j].param_name.replace(/-/g, "_");
            let arg_type = func_args[j].param_type;
            func_args_str += arg_name + ": " + witTypeToTsType(arg_type);
            if (j < func_args.length - 1) {
                func_args_str += ", ";
            }
        }
    }

    // if !isResource, add export function, else don't 
    let signature = isResource ? "" : "export function ";

    signature += func_name;
    signature += isResource ? ": " : "";
    signature += "(" + func_args_str + ")";

    if (func_ret) {
        // if async, return type is Promise<ret_type>
        if (func_async) {
            signature += ": Promise<" + witTypeToTsType(func_ret) + ">;\n";
        }
        // if not async, return type is ret_type
        else {
            signature += isResource ? " => " : ": ";
            signature += witTypeToTsType(func_ret) + ";\n";
        }
    } else {
        signature += ";\n\n";
    }
    return signature;
}