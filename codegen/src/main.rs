#![allow(unused_imports, dead_code, unused_variables, unused_mut)]

extern crate xml;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

const PRINT: bool = false;


fn pascal_to_snake_case(orig: &str, prune_p: bool) -> (String, bool, bool) {
    let mut output = String::with_capacity(48);
    let mut prev_was_new_word = true;

    let mut first_is_p = false;
    let mut second_is_p = false;
    let mut p_was_pruned = false;
    let mut pp_was_pruned = false;
    let mut orig_pruned = orig;

    if prune_p {
        // Strip preceeding "p" or "pp" from name:
        for (char_idx, c) in orig.chars().enumerate() {
            if char_idx == 0 {
                if c == 'p' {
                    first_is_p = true;
                }
            } else if char_idx == 1 {
                if first_is_p {
                    if c.is_uppercase() {
                        orig_pruned = orig.split_at(1).1;
                        p_was_pruned = true;
                    } else if c == 'p' {
                        second_is_p = true;
                        continue;
                    }
                }
            } else if char_idx == 2 {
                if second_is_p {
                    if c.is_uppercase() {
                        orig_pruned = orig.split_at(2).1;
                        pp_was_pruned = true;
                    } else {
                        panic!("Unexpected \"pp\" at start of member name");
                    }
                }
            } else {
                break;
            }
        }
    }

    for (char_idx, c) in orig_pruned.chars().enumerate() {
        if c.is_lowercase() {
            output.push(c);
            prev_was_new_word = false;
        } else if c.is_uppercase() || c.is_numeric() {

            if !prev_was_new_word {
                output.push('_');
                prev_was_new_word = true;
            } else {
                prev_was_new_word = false;
            }

            if c.is_uppercase() {
                output.push_str(&c.to_lowercase().to_string());
            } else {
                output.push(c);
            }
        }
    }

    if PRINT { println!("{}   ->   {}", orig, &output); }
    assert!(!(p_was_pruned && pp_was_pruned));
    (output, p_was_pruned, pp_was_pruned)
}

fn pascalize_suffix(orig: &str) -> String {
    if orig.contains("1D") { return orig.replace("1D", "1d"); }
    if orig.contains("2D") { return orig.replace("2D", "2d"); }
    if orig.contains("3D") { return orig.replace("3D", "3d"); }
    if orig.contains("KHR") { return orig.replace("KHR", "Khr"); }
    if orig.contains("EXT") { return orig.replace("EXT", "Ext"); }
    if orig.contains("NVX") { return orig.replace("NVX", "Nvx"); }
    if orig.contains("KHX") { return orig.replace("KHX", "Khx"); }
    if orig.contains("GOOGLE") { return orig.replace("GOOGLE", "Google");}
    if orig.contains("MVK") { return orig.replace("MVK", "Mvk"); }
    if orig.contains("NV") { return orig.replace("NV", "Nv"); }
    orig.to_string()
}

fn convert_type_name(orig_type: &str) -> String {
    match orig_type {
        "float" => "f32".to_string(),
        "int32_t" => "i32".to_string(),
        "uint32_t" => "u32".to_string(),
        "char" => "i8".to_string(),
        "uint8_t" => "u8".to_string(),
        "void" => "()".to_string(),
        "size_t" => "usize".to_string(),
        "uint64_t" => "u64".to_string(),
        "int" => "i32".to_string(),
        other @ _ => {
            if other.len() > 2 && other.split_at(2).0 == "Vk" {
                pascalize_suffix(other.split_at(2).1)
            } else if other.len() > 4 && other.split_at(4).0 == "PFN_" {
                String::from(other.split_at(4).1)
            } else if !other.contains("ANativeWindow") &&
                    !other.contains("MirConnection") &&
                    !other.contains("MirSurface") &&
                    !other.contains("wl_display") &&
                    !other.contains("wl_surface") &&
                    !other.contains("HINSTANCE") &&
                    !other.contains("HWND") &&
                    !other.contains("Display") &&
                    !other.contains("Window") &&
                    !other.contains("xcb_connection_t") &&
                    !other.contains("xcb_window_t") &&
                    !other.contains("HANDLE") &&
                    !other.contains("SECURITY_ATTRIBUTES") &&
                    !other.contains("DWORD") &&
                    !other.contains("LPCWSTR") {
                panic!("unknown type: \"{}\"", other);
            } else {
                String::from(other)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
enum TypeCategory {
    None,
    Struct,
    Union,
    Enum,
    Other,
}


#[derive(Clone, Debug)]
struct Member {
    ty: String,
    orig_name: String,
    voodoo_name: String,
    is_ptr: bool,
    is_ptr_ptr: bool,
    is_const: bool,
    is_const_const: bool,
    is_struct: bool,
    optional: bool,
    noautovalidity: bool,
    externsync: bool,
    array_size: Option<String>,
    comment: Option<String>,
    values: Option<String>,
    len: Option<String>,
    altlen: Option<String>,
}

impl Member {
    fn new(attribs: &[OwnedAttribute]) -> Member {
        let mut member = Member {
            ty: String::new(),
            orig_name: String::new(),
            voodoo_name: String::new(),
            is_ptr: false,
            is_ptr_ptr: false,
            is_const: false,
            is_const_const: false,
            is_struct: false,
            optional: false,
            noautovalidity: false,
            externsync: false,
            array_size: None,
            comment: None,
            values: None,
            len: None,
            altlen: None,
        };

        for attrib in attribs {
            match attrib.name.local_name.as_str() {
                "values" => member.values = Some(attrib.value.clone()),
                "optional" => member.optional |= attrib.value == "true",
                "len" => member.len = Some(attrib.value.clone()),
                "noautovalidity" => member.noautovalidity |= attrib.value == "true",
                "altlen" => member.altlen = Some(attrib.value.clone()),
                "externsync" => member.externsync |= attrib.value == "true",
                unknown @ _ => panic!("unknown struct attribute: {:?}={:?}",
                    unknown, attrib.value),
            }
        }

        member
    }

    fn set_name(&mut self, orig_name: String) {
        assert!(self.orig_name.is_empty());
        assert!(self.voodoo_name.is_empty());
        let (voodoo_name, p_was_pruned, pp_was_pruned) = pascal_to_snake_case(&orig_name, true);
        assert!((p_was_pruned && self.is_ptr) || !p_was_pruned);
        assert!((pp_was_pruned && self.is_ptr_ptr) || !pp_was_pruned);
        self.voodoo_name = voodoo_name;
        self.orig_name = orig_name;
    }

    fn set_type(&mut self, orig_type: &str) {
        assert!(self.ty.is_empty());
        self.ty = convert_type_name(orig_type);
    }
}


#[derive(Clone, Debug)]
struct Struct {
    orig_name: String,
    voodoo_name: String,
    returnedonly: bool,
    structextends: Option<String>,
    comment: String,
    members: Vec<Member>,
    contains_ptr: bool,
}

impl Struct {
    fn new(attribs: &[OwnedAttribute]) -> Struct {
        let mut orig_name = None;
        let mut voodoo_name = None;
        let mut returnedonly = false;
        let mut structextends = None;
        let mut comment = String::new();

        for attrib in attribs {
            match attrib.name.local_name.as_str() {
                "category" => (),
                "name" => {
                    let name = attrib.value.clone();
                    voodoo_name = Some(convert_type_name(&name));
                    orig_name = Some(name);
                },
                "returnedonly" => returnedonly |= attrib.value == "true",
                "structextends" => structextends = Some(String::from(attrib.value.clone())),
                "comment" => comment = attrib.value.clone(),
                unknown @ _ => panic!("unknown struct attribute: {:?}={:?}",
                    unknown, attrib.value),
            }
        }

        Struct {
            orig_name: orig_name.expect("no struct name found"),
            voodoo_name: voodoo_name.expect("no struct name found"),
            returnedonly,
            structextends,
            comment,
            members: Vec::with_capacity(16),
            contains_ptr: false,
        }
    }
}


fn category(s: &str) -> TypeCategory {
    match s {
        "struct" => TypeCategory::Struct,
        "union" => TypeCategory::Union,
        "enum" => TypeCategory::Enum,
        _ => TypeCategory::Other,
    }
}

fn parse_stray_text(s: &str, current_member: &mut Member) {
    match s {
        // Brackets alone will have sizes set by a value wrapped in an <enum> tag.
        "[" => (),
        "]" => (),
        _ => {
            if s.starts_with("* const*") {
                assert!(!current_member.is_ptr);
                current_member.is_ptr_ptr = true;
                if current_member.is_const {
                    current_member.is_const = false;
                    current_member.is_const_const = true;
                } else {
                    current_member.is_const = true;
                }
            } else if s.starts_with("const") {
                assert!(!current_member.is_const);
                assert!(!current_member.is_const_const);
                if current_member.is_const {
                    current_member.is_const = false;
                    current_member.is_const_const = true;
                } else {
                    current_member.is_const = true;
                }
            } else if s.starts_with("struct") {
                assert!(!current_member.is_struct);
                current_member.is_struct = true;
            } else if s.starts_with("*") {
                assert!(!current_member.is_ptr);
                current_member.is_ptr = true;
            } else if s.starts_with("[") {
                let mut array_size = String::with_capacity(4);
                for (char_idx, c) in s.chars().enumerate() {
                    match c {
                        '[' => (),
                        ']' => assert!(char_idx == s.len() - 1),
                        digit @ _ => {
                            assert!(digit.is_numeric(),
                                "unexpected character found \
                                while parsing array size: {}", c);
                            array_size.push(digit);
                        },
                    }
                }

            } else {
                panic!("unknown characters present: {}", s)
            }
        }
    }
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn parse_structs() -> Vec<Struct> {
    let file = File::open("./gen_src/vk.xml").unwrap();
    let reader = BufReader::new(file);
    let parser = EventReader::new(reader);

    let mut structs: Vec<Struct> = Vec::with_capacity(400);

    let mut current_struct: Option<Struct> = None;
    let mut struct_start_depth = 0;
    let mut parsing_struct_comment = false;

    let mut current_member: Option<Member> = None;
    let mut member_start_depth = 0;
    let mut parsing_member_type = false;
    let mut parsing_member_name = false;
    let mut parsing_member_array_size = false;
    let mut parsing_member_comment = false;

    let mut depth = 0;

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let mut type_category = TypeCategory::None;
                if name.local_name == "type" {
                    for attrib in &attributes {
                        if attrib.name.local_name == "category" {
                            type_category = category(&attrib.value);
                        }
                    }
                }
                if type_category == TypeCategory::Struct {
                    current_struct = Some(Struct::new(&attributes));
                    struct_start_depth = depth;
                }
                if let Some(ref mut st) = current_struct {
                    match name.local_name.as_str() {
                        "member" => {
                            assert!(current_member.is_none());
                            current_member = Some(Member::new(&attributes));
                            member_start_depth = depth;
                        },
                        "type" => {
                            parsing_member_type = true;
                        },
                        "name" => {
                            parsing_member_name = true;
                        },
                        "enum" => {
                            parsing_member_array_size = true;
                        },
                        "comment" => {
                            if current_member.is_some() {
                                parsing_member_comment = true;
                            } else {
                                parsing_struct_comment = true;
                            }
                        },
                        unknown @ _ => panic!("unknown tag: \"{}\"", unknown),
                    }
                    if PRINT {
                        print!("{}<{}", indent(depth), name);
                        for attrib in attributes {
                            print!(" {}=\"{}\"", attrib.name, attrib.value);
                        }
                        print!(">\n");
                    }
                }
                depth += 1;
            },
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                if PRINT && current_struct.is_some() {
                    println!("{}</{}>", indent(depth), name);
                }
                if name.local_name == "member" && current_struct.is_some() {
                    if depth == member_start_depth {
                        let st = current_struct.as_mut().expect("no current struct");
                        let new_member = current_member.take().expect("no current member");
                        // new_member.validate();
                        st.members.push(new_member);
                    }
                } else if name.local_name == "type" && current_struct.is_some() {
                    if depth == struct_start_depth {
                        assert!(current_struct.is_some());
                        if let Some(st) = current_struct.take() {
                            structs.push(st);;
                        }
                    }
                }
            },
            Ok(XmlEvent::Characters(s)) => {
                if PRINT && current_struct.is_some() {
                    println!("{}{}", indent(depth), s.as_str());
                }
                if let Some(ref mut cur_mem) = current_member {
                    if s.len() > 0 {
                        if parsing_member_type {
                            cur_mem.set_type(&s);
                            parsing_member_type = false;
                        } else if parsing_member_name {
                            cur_mem.set_name(s);
                            parsing_member_name = false;
                        } else if parsing_member_array_size {
                            cur_mem.array_size = Some(s);
                            parsing_member_array_size = false;
                        } else if parsing_member_comment {
                            cur_mem.comment = Some(s);
                            parsing_member_comment = false;
                        } else {
                            parse_stray_text(&s, cur_mem);
                            if cur_mem.is_ptr || cur_mem.is_ptr_ptr {
                                current_struct.as_mut().unwrap().contains_ptr = true;
                            }
                        }
                    }
                } else if let Some(ref mut cur_struct) = current_struct {
                    if parsing_struct_comment && s.len() > 0 {
                        cur_struct.comment = String::from(s);
                        parsing_struct_comment = false;
                    }
                }
            },
            Ok(XmlEvent::CData(s)) => (),
            Ok(XmlEvent::Comment(s)) => (),
            Ok(XmlEvent::Whitespace(s)) => (),
            Err(e) => {
                println!("Error: {}", e);
                break;
            },
            _ => {}
        }
    }

    println!("Structs: \n\n{:#?}", structs);
    println!("{} structs parsed", structs.len());
    structs
}

fn main() {
    let mut structs: Vec<Struct> = parse_structs();


}

