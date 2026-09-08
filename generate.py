import json
import re
import os
import sys
from collections import defaultdict

def to_snake_case(name):
    s = re.sub(r'\[(.*?)\]', r'_\1', name)  # handle price[lte] -> price_lte
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', s)
    s = re.sub(r'([a-z\d])([A-Z])', r'\1_\2', s)
    s = s.replace('-', '_').replace('.', '_').replace('/', '_').replace(' ', '_').replace(':', '_').replace('$', '_')
    s = s.lower()
    s = re.sub(r'[^a-z0-9_]', '_', s)
    s = re.sub(r'_+', '_', s).strip('_')
    keywords = {
        'type', 'match', 'move', 'ref', 'yield', 'use', 'mod', 'fn', 'struct', 
        'enum', 'box', 'loop', 'while', 'for', 'if', 'else', 'return', 'const',
        'static', 'mut', 'pub', 'trait', 'impl', 'as', 'where', 'async', 'await'
    }
    if s in keywords:
        s += '_'
    if s and s[0].isdigit():
        s = 'n_' + s
    if not s:
        s = 'param'
    return s

def to_pascal_case(name):
    name = re.sub(r'\[(.*?)\]', r'_\1', name)
    name = re.sub(r'[^a-zA-Z0-9_]', '_', name)
    parts = [p for p in name.split('_') if p]
    if not parts:
        return "Unknown"
    res = ''.join(p[0].upper() + p[1:] if len(p) > 1 else p.upper() for p in parts)
    if res[0].isdigit():
        res = "Type" + res
    return res

def clean_doc(doc, indent=""):
    if not doc:
        return ""
    lines = str(doc).strip().split('\n')
    out = []
    for line in lines:
        cleaned = line.replace('/*', '/ *').replace('*/', '* /')
        out.append(f"{indent}/// {cleaned}")
    return "\n".join(out) + "\n"

with open('swagger_definition_v3.json') as f:
    spec = json.load(f)

generated_structs = {}
in_progress = set()

def resolve_primitive_type(schema):
    if not isinstance(schema, dict):
        return 'serde_json::Value'
    t = schema.get('type')
    fmt = schema.get('format')
    if t == 'integer':
        return 'i32' if fmt == 'int32' else 'i64'
    if t == 'number':
        return 'f32' if fmt == 'float' else 'f64'
    if t == 'boolean':
        return 'bool'
    if t == 'string':
        if fmt == 'date-time':
            return 'time::OffsetDateTime'
        if fmt == 'date':
            return 'time::Date'
        if fmt == 'binary':
            return 'bytes::Bytes'
        return 'String'
    return 'serde_json::Value'

def extract_schema_definition(schema_name, schema):
    if not isinstance(schema, dict):
        return "serde_json::Value"
    if schema_name in generated_structs:
        return schema_name
    if schema_name in in_progress:
        return schema_name
    
    in_progress.add(schema_name)
    
    # Check if enum
    if 'enum' in schema and schema.get('type') == 'string':
        enum_variants = []
        for idx, val in enumerate(schema['enum']):
            v_name = to_pascal_case(str(val))
            if not v_name or v_name == "Unknown": v_name = f"Variant{idx}"
            enum_variants.append((v_name, str(val)))
        
        doc = clean_doc(schema.get('description', ''))
        code = f"{doc}#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]\n"
        code += f"pub enum {schema_name} {{\n"
        for v_name, orig in enum_variants:
            code += f'    #[serde(rename = "{orig}")]\n'
            code += f"    {v_name},\n"
        code += "}\n"
        code += f"impl Default for {schema_name} {{\n"
        code += f"    fn default() -> Self {{\n"
        code += f"        Self::{enum_variants[0][0]}\n"
        code += f"    }}\n"
        code += "}\n"
        code += f"impl core::fmt::Display for {schema_name} {{\n"
        code += f"    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{\n"
        code += f"        match self {{\n"
        for v_name, orig in enum_variants:
            code += f'            Self::{v_name} => write!(f, "{orig}"),\n'
        code += f"        }}\n"
        code += f"    }}\n"
        code += "}\n"
        generated_structs[schema_name] = code
        in_progress.remove(schema_name)
        return schema_name

    # Merge allOf / properties
    properties = {}
    required = set(schema.get('required', [])) if isinstance(schema.get('required'), list) else set()
    description = schema.get('description', '')

    def collect_props(s):
        nonlocal description
        if not isinstance(s, dict): return
        if 'description' in s and not description:
            description = s['description']
        if 'properties' in s and isinstance(s['properties'], dict):
            properties.update(s['properties'])
        if 'required' in s and isinstance(s['required'], list):
            required.update(s['required'])
        if 'allOf' in s and isinstance(s['allOf'], list):
            for sub in s['allOf']:
                collect_props(sub)

    collect_props(schema)

    if not properties:
        in_progress.remove(schema_name)
        if schema.get('type') == 'object' and 'additionalProperties' in schema:
            return "serde_json::Value"
        elif schema.get('type') == 'array':
            item_type = resolve_type(schema.get('items', {}), f"{schema_name}Item")
            return f"Vec<{item_type}>"
        else:
            return resolve_primitive_type(schema)

    # Build struct
    fields_code = ""
    for prop_name, prop_schema in properties.items():
        rust_field_name = to_snake_case(prop_name)
        prop_doc = clean_doc(prop_schema.get('description', '') if isinstance(prop_schema, dict) else '', "    ")
        prop_type = resolve_type(prop_schema, f"{schema_name}{to_pascal_case(prop_name)}")
        
        is_req = prop_name in required
        if not is_req:
            field_type = f"Option<{prop_type}>"
            serde_attr = f'    #[serde(rename = "{prop_name}", skip_serializing_if = "Option::is_none", default)]\n'
        else:
            field_type = prop_type
            serde_attr = f'    #[serde(rename = "{prop_name}")]\n'
        
        fields_code += prop_doc
        fields_code += serde_attr
        fields_code += f"    pub {rust_field_name}: {field_type},\n"

    doc = clean_doc(description)
    code = f"{doc}#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]\n"
    code += "#[serde(rename_all = \"camelCase\")]\n"
    code += f"pub struct {schema_name} {{\n"
    code += fields_code
    code += "}\n"
    
    generated_structs[schema_name] = code
    in_progress.remove(schema_name)
    return schema_name

def resolve_type(schema, hint_name):
    if not isinstance(schema, dict):
        return 'serde_json::Value'
    
    t = schema.get('type')
    if t == 'array':
        item_type = resolve_type(schema.get('items', {}), f"{hint_name}Item")
        return f"Vec<{item_type}>"
    if t == 'object' or 'properties' in schema or 'allOf' in schema:
        add_props = schema.get('additionalProperties')
        if isinstance(add_props, dict) and add_props:
            val_type = resolve_type(add_props, f"{hint_name}Value")
            return f"std::collections::HashMap<String, {val_type}>"
        elif add_props is True or (isinstance(add_props, bool) and add_props):
            return "std::collections::HashMap<String, serde_json::Value>"
        if 'properties' in schema or 'allOf' in schema:
            struct_name = to_pascal_case(hint_name)
            return extract_schema_definition(struct_name, schema)
        return 'serde_json::Value'
    if 'enum' in schema and schema.get('type') == 'string':
        enum_name = to_pascal_case(hint_name)
        return extract_schema_definition(enum_name, schema)
    
    return resolve_primitive_type(schema)

# 1. Component schemas
for name, schema in spec.get('components', {}).get('schemas', {}).items():
    extract_schema_definition(to_pascal_case(name), schema)

# 2. Extract schemas from operations (request bodies, response bodies, and parameters)
tag_operations = defaultdict(list)
for path, path_item in spec.get('paths', {}).items():
    for method, op in path_item.items():
        if not isinstance(op, dict) or 'operationId' not in op:
            continue
        raw_tag = op.get('tags', ['General'])[0]
        tag = to_pascal_case(raw_tag)
        op_id = op['operationId']
        pascal_op_id = to_pascal_case(op_id)
        
        # Process parameters
        for p in op.get('parameters', []):
            p_schema = p.get('schema', {})
            if 'enum' in p_schema and p_schema.get('type') == 'string':
                p_hint = f"{pascal_op_id}{to_pascal_case(p['name'])}"
                extract_schema_definition(p_hint, p_schema)

        # Request body
        rb = op.get('requestBody')
        req_type = None
        if rb and isinstance(rb, dict):
            content = rb.get('content', {}).get('application/json', {})
            schema = content.get('schema')
            if schema:
                req_type = resolve_type(schema, f"{pascal_op_id}Request")
        
        # Response body (prefer 200, 201, 202)
        resp_type = "serde_json::Value"
        responses = op.get('responses', {})
        for code in ['200', '201', '202']:
            if code in responses:
                resp_obj = responses[code]
                if isinstance(resp_obj, dict):
                    content = resp_obj.get('content', {}).get('application/json', {})
                    schema = content.get('schema')
                    if schema:
                        resp_type = resolve_type(schema, f"{pascal_op_id}Response")
                break
        
        tag_operations[tag].append({
            'operationId': op_id,
            'method': method.upper(),
            'path': path,
            'summary': op.get('summary', ''),
            'description': op.get('description', ''),
            'parameters': op.get('parameters', []),
            'req_type': req_type,
            'resp_type': resp_type
        })

# Write models to src/models.rs
with open('src/models.rs', 'w') as f:
    f.write("//! Strongly typed Brevo v3 API request and response data models.\n\n")
    f.write("#![allow(unused_imports, clippy::all)]\n\n")
    f.write("use serde::{Deserialize, Serialize};\n\n")
    for struct_code in generated_structs.values():
        f.write(struct_code)
        f.write("\n")

# Generate API modules in src/api/
os.makedirs('src/api', exist_ok=True)
api_modules = []

for tag, ops in tag_operations.items():
    mod_name = to_snake_case(tag)
    struct_name = f"{tag}Api"
    api_modules.append((mod_name, struct_name, tag))
    
    file_path = f"src/api/{mod_name}.rs"
    with open(file_path, 'w') as f:
        f.write(f"//! {tag} API endpoints.\n\n")
        f.write("#![allow(unused_imports, unused_mut, clippy::all)]\n\n")
        f.write("use crate::client::BrevoClient;\n")
        f.write("use crate::error::{Error, Result};\n")
        f.write("use crate::models::*;\n")
        f.write("use reqwest::Method;\n\n")
        f.write(f"/// Client for {tag} endpoints.\n")
        f.write("#[derive(Clone, Debug)]\n")
        f.write(f"pub struct {struct_name} {{\n")
        f.write("    client: BrevoClient,\n")
        f.write("}\n\n")
        f.write(f"impl {struct_name} {{\n")
        f.write("    pub(crate) fn new(client: BrevoClient) -> Self {\n")
        f.write("        Self { client }\n")
        f.write("    }\n\n")

        for op in ops:
            fn_name = to_snake_case(op['operationId'])
            doc_str = op['summary'] or op['description'] or f"Execute {op['operationId']}"
            f.write(clean_doc(doc_str, "    "))
            
            # Separate parameters: path, query, header
            path_params = []
            query_params = []
            for p in op['parameters']:
                p_in = p.get('in')
                p_name = p['name']
                p_rust_name = to_snake_case(p_name)
                p_schema = p.get('schema', {})
                p_type = resolve_type(p_schema, f"{to_pascal_case(op['operationId'])}{to_pascal_case(p_name)}")
                is_req = p.get('required', False) or p_in == 'path'
                
                if p_in == 'path':
                    path_params.append((p_name, p_rust_name, p_type))
                elif p_in == 'query':
                    query_params.append((p_name, p_rust_name, p_type, is_req))
            
            # Args signature
            args = ["&self"]
            for p_name, p_rust_name, p_type in path_params:
                if p_type == "String":
                    args.append(f"{p_rust_name}: &str")
                else:
                    args.append(f"{p_rust_name}: {p_type}")
            
            if op['req_type']:
                args.append(f"body: &{op['req_type']}")
            
            for p_name, p_rust_name, p_type, is_req in query_params:
                if is_req:
                    if p_type == "String":
                        args.append(f"{p_rust_name}: &str")
                    elif p_type.startswith("Vec<"):
                        args.append(f"{p_rust_name}: &[{p_type[4:-1]}]")
                    else:
                        args.append(f"{p_rust_name}: {p_type}")
                else:
                    if p_type == "String":
                        args.append(f"{p_rust_name}: Option<&str>")
                    elif p_type.startswith("Vec<"):
                        args.append(f"{p_rust_name}: Option<&[{p_type[4:-1]}]>")
                    else:
                        args.append(f"{p_rust_name}: Option<{p_type}>")
            
            args_str = ", ".join(args)
            resp_t = op['resp_type']
            
            f.write(f"    pub async fn {fn_name}({args_str}) -> Result<{resp_t}> {{\n")
            
            # Path formatting
            rust_path = op['path']
            for p_name, p_rust_name, _ in path_params:
                rust_path = rust_path.replace(f"{{{p_name}}}", f"{{{p_rust_name}}}")
            
            if path_params:
                f.write(f'        let path = format!("{rust_path}");\n')
            else:
                f.write(f'        let path = "{rust_path}";\n')
            
            f.write(f"        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;\n")
            
            if query_params:
                f.write("        {\n")
                f.write("            let mut query_pairs = url.query_pairs_mut();\n")
                for p_name, p_rust_name, p_type, is_req in query_params:
                    if is_req:
                        if p_type.startswith("Vec<"):
                            f.write(f'            for item in {p_rust_name} {{\n')
                            f.write(f'                query_pairs.append_pair("{p_name}", &item.to_string());\n')
                            f.write('            }\n')
                        else:
                            f.write(f'            query_pairs.append_pair("{p_name}", &{p_rust_name}.to_string());\n')
                    else:
                        if p_type.startswith("Vec<"):
                            f.write(f'            if let Some(items) = {p_rust_name} {{\n')
                            f.write(f'                for item in items {{\n')
                            f.write(f'                    query_pairs.append_pair("{p_name}", &item.to_string());\n')
                            f.write('                }\n')
                            f.write('            }\n')
                        else:
                            f.write(f'            if let Some(val) = {p_rust_name} {{\n')
                            f.write(f'                query_pairs.append_pair("{p_name}", &val.to_string());\n')
                            f.write('            }\n')
                f.write("        }\n")
            
            method_str = op['method']
            f.write(f"        let mut req_builder = self.client.http().request(Method::{method_str}, url);\n")
            if op['req_type']:
                f.write("        req_builder = req_builder.json(body);\n")
            
            f.write("        let req = req_builder.build()?;\n")
            f.write("        self.client.execute(req).await\n")
            f.write("    }\n\n")
        
        f.write("}\n")

# Write src/api/mod.rs
with open('src/api/mod.rs', 'w') as f:
    f.write("//! API modules categorized by Brevo domain resources.\n\n")
    for mod_name, struct_name, _ in api_modules:
        f.write(f"pub mod {mod_name};\n")
    f.write("\n")
    for mod_name, struct_name, _ in api_modules:
        f.write(f"pub use {mod_name}::{struct_name};\n")

# Also add accessor methods on BrevoClient
client_methods = ""
for mod_name, struct_name, tag in api_modules:
    client_methods += f"    /// Access the [`{struct_name}`] endpoints.\n"
    client_methods += f"    #[must_use]\n"
    client_methods += f"    pub fn {mod_name}(&self) -> api::{struct_name} {{\n"
    client_methods += f"        api::{struct_name}::new(self.clone())\n"
    client_methods += "    }\n\n"

with open('src/client_accessors.rs', 'w') as f:
    f.write("use crate::api;\n")
    f.write("use crate::client::BrevoClient;\n\n")
    f.write("impl BrevoClient {{\n")
    f.write(client_methods)
    f.write("}\n")

print(f"Generated {len(api_modules)} API resource modules in src/api/")
