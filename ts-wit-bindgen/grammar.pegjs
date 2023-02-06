{
function flatten1(obj) {
  let result = [];
  for (let key in obj) {
    if (obj.hasOwnProperty(key)) {
      if (obj[key].hasOwnProperty("value")) {
        result.push(obj[key]);
      } else {
        result = result.concat(flatten1(obj[key]));
      }
    }
  }
  return result;
}

function flatten2(obj) {
    var result = [];
    for (var key in obj) {
        if (typeof obj[key] === "object") {
            result = result.concat(flatten2(obj[key]));
        } else {
            result.push(obj[key]);
        }
    }
    return result;
}

function flatten3(json) {
  let result = [];
  for (let key in json) {
    let element = json[key];
    if (typeof element === 'object') {
      result = result.concat(flatten3(element));
    } else {
      result.push(json);
      break;
    }
  }
  return result;
}

function flatten4(json) {
    let result = []
    if (typeof json === "object") {
        for (const [key, valuen] of Object.entries(json)) {
            if (key === "valuen") {
                result.push(...flatten4(valuen))
            } else {
                result.push(json[key])
            }
        }
    } else {
        result.push(json)
    }
    return result
}
}
start
	= token:(
    	_
    	/ comment
        / operator
        / keyword
       )*
    
// tokens
_
  = value:[ \t\n\r] { return { whitespace: value } }
  
comment 
    = "///" value:(character_that_isnt_a_newline *) _? { return { comment: value.join('') } }
    / "/**" value:(any_unicode_character *) "*/" { return { comment: value.join('') } }
    / "//" value:(character_that_isnt_a_newline *) _? { return { comment: value.join('') } }
    / "/*" value:(any_unicode_character *) "*/" { return { comment: value.join('') } }
    
operator
	= value:'=' { return { operator: value } }
    / value:',' { return { operator: value } }
    / value:':' { return { operator: value } }
    / value:';' { return { operator: value } }
    / value:'(' { return { operator: value } }
    / value:')' { return { operator: value } }
    / value:'{' { return { operator: value } }
    / value:'}' { return { operator: value } }
    / value:'<' { return { operator: value } }
    / value:'>' { return { operator: value } }
    / value:'*' { return { operator: value } }
    / value:'->' { return { operator: value } }
    
keyword
    = value:use_item { return { keyword: value } }
    / value:type_item { return { keyword: value } }
    / value:resource_item { return { keyword: value } }
    / value:func_item { return { keyword: value } }
    / value:'u8'  { return { keyword: value } }
    / value:'u16'  { return { keyword: value } }
    / value:'u32'  { return { keyword: value } }
    / value:'u64' { return { keyword: value } }
    / value:'s8'  { return { keyword: value } }
    / value:'s16'  { return { keyword: value } }
    / value:'s32'  { return { keyword: value } }
    / value:'s64' { return { keyword: value } }
    / value:'float32'  { return { keyword: value } }
    / value:'float64' { return { keyword: value } }
    / value:'char' { return { keyword: value } }
    / value:'handle' { return { keyword: value } }
    / value:record_item { return { keyword: value } }
    / value:enum_item { return { keyword: value } }
    / value:flags_item { return { keyword: value } }
    / value:variant_item { return { keyword: value } }
    / value:union_item { return { keyword: value } }
    / value:'bool' { return { keyword: value } }
    / value:'string' { return { keyword: value } }
    / value:'option' { return { keyword: value } }
    / value:'list' { return { keyword: value } }
    / value:'expected' { return { keyword: value } }
    / value:'unit' { return { keyword: value } }
    / value:'as' { return { keyword: value } }
    / value:'from' { return { keyword: value } }
    / value:'static' { return { keyword: value } }
    / value:'interface' { return { keyword: value } }
    / value:'tuple' { return { keyword: value } }
    / value:'async' { return { keyword: value } }
    / value:'future' { return { keyword: value } }
    / value:'stream' { return { keyword: value } }

// items
use_item
	= "use" _+ uses:use_names _+ "from" _+ from:id { return { use_item: {uses, from: from.join('')} } }

use_names 
	= "*"
    / "{" _* value:use_names_list _* "}" { return value }

use_names_list
	= value0:use_names_item _* ',' _* valuen:use_names_list? { return flatten1({value0, valuen}) }
    / use_names_item

use_names_item
	= value:id  _* 'as' _* alias:id { return { value:value.join(''), alias:alias.join('') } }
    / value:id { return { value: value.join('') } }
   	

type_item 
	= 'type' _* alias:id _* '=' _* ty:ty { return { type: { alias:alias.join(''), ty } }}


record_item
	= 'record' _* name:id _* '{' _* fields:record_fields _* '}' { return { record_item: {name: name.join(''), fields} } }

record_fields
	= value0:record_field _* ',' _* valuen:record_fields? { return flatten3({ value0, valuen }) }
    / record_field

record_field
	= property_name:id _* ':' _* property_type:ty { return { record_field: { property_name: property_name.join(''), property_type } } }
    

flags_item
	= 'flags' _* name:id _* '{' _* fields:flags_fields _* '}' { return { flags_item: {name: name.join(''), fields} } }

flags_fields
	= value0:id _* ',' _* valuen:flags_fields? { return flatten4({ value0: value0.join(''), valuen }) }
    / value:id { return value.join('') }
    
    
variant_item
	= 'variant' _* name:id _* '{' _* fields:variant_cases _* '}' { return { variant_item: {name: name.join(''), fields} } }

variant_cases
	= value0:variant_case _* ',' _* valuen:variant_cases? { return flatten3({value0, valuen}) }
    / value:variant_case

variant_case
	= value:id _* '(' _* ty:ty _* ')' { return { value: value.join(''), ty } }
    / value:id { return { value: value.join('')} } 
    

enum_item
	= 'enum' _* name:id _* '{' _* cases:enum_cases _* '}' { return { enum_item: {name: name.join(''), cases} } }

enum_cases
	= value0:id _* ',' _* valuen:enum_cases? { return flatten4({value0: value0.join(''), valuen}) }
    / value:id { return { value: value.join('')} } 
    
    
union_item
	= 'union' _* name:id _* '{' _* cases:union_cases _* '}' { return { union_item: {name: name.join(''), cases} } }

union_cases 
	= value0:ty _* ',' _* valuen:union_cases? { return flatten4({value0, valuen}) }
	/ value:ty


func_item 
	= name:id _* ':' _* async:'async'? _* 'func' _* '(' _* args:func_args? _* ')' ret:func_ret { return { func_item: { name: name.join(''), async, args, ret} } }

func_args 
	= value0:func_arg _* ',' _* valuen:func_args? { return flatten3({value0, valuen}) }
    / value:func_arg

func_arg 
	= param_name:id _* ':' _* param_type:ty { return { param_name: param_name.join(''), param_type } }

func_ret 
	= _* '->' _* ty:ty { return ty}
    / _* { return null }
    

resource_item
	= 'resource' _* name:id contents:resource_contents { return { resource_item: { name: name.join(''), contents } } }

resource_contents
	= _* '{' defs:resource_defs _* '}'{ return defs }
    / _* { return null }

resource_defs 
	= value0:resource_def valuen:resource_defs? { return flatten3({ value0, valuen }) }

resource_def 
	= _* stat:'static'? (_*)? fn:func_item { return { stat, fn } }

// token helpers
character_that_isnt_a_newline
	= [^\n]

any_unicode_character
	= [^\*\/]

// identifiers, and types
id
	= [a-zA-Z_0-9-]+

ty 
    = 'u8' 
    / 'u16' 
    / 'u32' 
    / 'u64'
    / 's8' 
    / 's16' 
    / 's32' 
    / 's64'
    / 'float32' 
    / 'float64'
    / 'char'
    / 'bool'
    / 'string'
    / 'unit'
    / tuple
    / list
    / option
    / expected
    / future
    / stream
    / value:id { return value.join('') }

tuple
	= 'tuple' _* '<' _* value:tuple_list _* '>' { return { tuple: value } }
    
tuple_list
	= value0:ty _* ',' _* valuen:tuple_list? { return flatten4({value0, valuen}) }
    / ty

list
	= 'list' _* '<' _* value:ty _* '>' { return { list: value } }

option
	= 'option' _* '<' _* value:ty _* '>' { return { option: value } }

expected
	= 'expected' _* '<' _* ok:ty _* ',' _* err:ty _* '>' { return { expected: { ok, err } } }

future
	= 'future' _* '<' _* value:ty _* '>' { return { future: { value } } }

stream
	= 'stream' _* '<' _* value0:ty _* ',' _* value1:ty _* '>' { return { stream: { value0, value1 } } }