//! Parses type definitions in `vk.xml` and outputs Voodoo equivalent types
//! and wrappers.

#![allow(unused_mut, unused_variables)]


extern crate xml;

use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufReader, BufWriter};
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;


const INDENT: &'static str = "    ";
const PRINT: bool = false;
const ORIG_USE: &str = "vks";
const ORIG_PRE: &str = "vks::";


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

    for c in orig_pruned.chars() {
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

fn replace_suffix(orig: &str) -> String {
    if orig.contains("1D") { return orig.replace("1D", "1d"); }
    if orig.contains("2D") { return orig.replace("2D", "2d"); }
    if orig.contains("3D") { return orig.replace("3D", "3d"); }
    if orig.contains("KHR") { return orig.replace("KHR", "Khr"); }
    if orig.contains("EXT") { return orig.replace("EXT", "Ext"); }
    if orig.contains("AMD") { return orig.replace("AMD", "Amd"); }
    if orig.contains("NVX") { return orig.replace("NVX", "Nvx"); }
    if orig.contains("KHX") { return orig.replace("KHX", "Khx"); }
    if orig.contains("GOOGLE") { return orig.replace("GOOGLE", "Google");}
    if orig.contains("MVK") { return orig.replace("MVK", "Mvk"); }
    if orig.contains("NV") { return orig.replace("NV", "Nv"); }
    if orig.contains("NN") { return orig.replace("NN", "Nn"); }

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
        "VkFlags" => "u32".to_string(),
        "VkBool32" => "bool".to_string(),
        "VkDeviceSize" => "u64".to_string(),
        "VkSampleMask" => "u32".to_string(),
        "VkResult" => "ResultEnum".to_string(),
        other @ _ => {
            if other.len() > 2 && other.split_at(2).0 == "Vk" {
                let mut out_str = replace_suffix(other.split_at(2).1);
                if out_str.contains("IOSSurface") { out_str = out_str.replace("IOSSurface", "IosSurface"); }
                if out_str.contains("OSSurface") { out_str = out_str.replace("OSSurface", "OsSurface"); }
                if out_str.contains("FlagBits") { out_str = out_str.replace("FlagBits", "Flags"); }
                out_str
            } else if other.len() > 4 && other.split_at(4).0 == "PFN_" {
                String::from(other)
            // } else if other.contains("FlagBits") {
            //     other.replace("FlagBits", "Flags")
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
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn parse_structs() -> Vec<Struct> {
    let file = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/gen_src/vk.xml")).unwrap();
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
                if let Some(ref mut _st) = current_struct {
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
            Ok(XmlEvent::CData(_s)) => (),
            Ok(XmlEvent::Comment(_s)) => (),
            Ok(XmlEvent::Whitespace(_s)) => (),
            Err(e) => {
                println!("Error: {}", e);
                break;
            },
            _ => {}
        }
    }

    // println!("Structs: \n\n{:#?}", structs);
    println!("{} structs parsed", structs.len());
    structs
}

fn struct_is_excluded(orig_name: &str) -> bool {
    match orig_name {
        "VkRect3D" => true,
        "VkBindBufferMemoryInfoKHR" => true,
        "VkBindImageMemoryInfoKHR" => true,
        "VkInputAttachmentAspectReferenceKHR" => true,
        "VkRenderPassInputAttachmentAspectCreateInfoKHR" => true,
        "VkPhysicalDevicePointClippingPropertiesKHR" => true,
        "VkImageViewUsageCreateInfoKHR" => true,
        "VkPipelineTessellationDomainOriginStateCreateInfoKHR" => true,
        "VkSamplerYcbcrConversionInfoKHR" => true,
        "VkSamplerYcbcrConversionCreateInfoKHR" => true,
        "VkBindImagePlaneMemoryInfoKHR" => true,
        "VkImagePlaneMemoryRequirementsInfoKHR" => true,
        "VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR" => true,
        "VkSamplerYcbcrConversionImageFormatPropertiesKHR" => true,
        "VkSampleLocationEXT" => true,
        "VkSampleLocationsInfoEXT" => true,
        "VkAttachmentSampleLocationsEXT" => true,
        "VkSubpassSampleLocationsEXT" => true,
        "VkRenderPassSampleLocationsBeginInfoEXT" => true,
        "VkPipelineSampleLocationsStateCreateInfoEXT" => true,
        "VkPhysicalDeviceSampleLocationsPropertiesEXT" => true,
        "VkMultisamplePropertiesEXT" => true,
        "VkImageFormatListCreateInfoKHR" => true,
        "VkValidationCacheCreateInfoEXT" => true,
        "VkShaderModuleValidationCacheCreateInfoEXT" => true,
        _ => false,
    }
}

fn member_is_excluded(orig_name: &str) -> bool {
    match orig_name {
        "sType" => true,
        _ => false,
    }
}

fn function_is_unsafe(orig_name: &str) -> bool {
    match orig_name {
        "pNext" => true,
        _ => false,
    }
}

fn fixup_function_name(orig_name: &str) -> Option<String> {
    match orig_name {
        "type" => Some("type_of".to_string()),
        _ => None,
    }
}

fn is_experimental(orig_name: &str) -> bool {
    orig_name.contains("KHX") || orig_name.contains("NVX")
}

fn write_builder_set_fn(o: &mut BufWriter<File>, s: &Struct, m: &Member, bldr_type_params: &str) -> io::Result<()> {
    let t = INDENT;
    if member_is_excluded(&m.orig_name) { return Ok(()); }
    let unsafe_str = if function_is_unsafe(&m.orig_name) { " unsafe" } else { "" };
    let fixed_fn_name = fixup_function_name(&m.orig_name);
    let fn_name = match fixed_fn_name {
        Some(ref n) => n.as_str(),
        None => m.voodoo_name.as_str(),
    };

    let mut arg_is_generic = false;
    let mut arg_type = String::new();
    let mut fn_type_params = "'m".to_string();
    let arg_lifetime = "'a";

    match m.ty {
        _ => {
            if arg_is_generic {
                // Push lifetime param just in case.
                fn_type_params.push_str(", ");
                fn_type_params.push_str(arg_lifetime);
                fn_type_params.push_str(", T");
                arg_type.push_str("T");
            } else {
                if m.is_ptr {
                    fn_type_params.push_str(", ");
                    fn_type_params.push_str(arg_lifetime);
                    arg_type.push_str("&");
                    arg_type.push_str(arg_lifetime);
                    arg_type.push_str(" ");
                }
                arg_type.push_str(&m.ty);
            }
        },
    }

    writeln!(o, "{t}pub{} fn {}<{}>(&'m mut self, {}: {}) -> &'m mut {}Builder{} {{",
        unsafe_str, fn_name, fn_type_params, fn_name, arg_type, s.voodoo_name, bldr_type_params, t=t)?;

    writeln!(o, "{t}{t}self", t=t)?;

    write!(o, "{t}}}\n\n", t=t)?;

    Ok(())
}

fn write_structs(structs: &[Struct]) -> io::Result<()> {
    let output_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/output/structs.rs");
    let output_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output_file_path)
        .unwrap();

    let mut output_write = BufWriter::new(output_file);
    let o = &mut output_write;

    let t = INDENT;
    writeln!(o, "//! Structs.")?;
    write!(o, "\n")?;
    writeln!(o, "use std::ptr;")?;
    writeln!(o, "use std::ffi::{{CString, CStr}};")?;
    writeln!(o, "use std::marker::PhantomData;")?;
    writeln!(o, "use ::*;")?;
    writeln!(o, "use {};", ORIG_USE)?;
    writeln!(o, "use {}::{{PFN_vkAllocationFunction, PFN_vkReallocationFunction, \
        PFN_vkFreeFunction, \n{t}PFN_vkInternalAllocationNotification, \
        PFN_vkInternalFreeNotification, \n{t}PFN_vkDebugReportCallbackEXT}};", ORIG_USE, t=t)?;
    write!(o, "\n\n")?;

    // writeln!(o, "type DeviceSize = u64;")?;

    writeln!(o, "")?;

    for s in structs {
        if struct_is_excluded(&s.orig_name) { continue; }

        // BEGIN STRUCT
        // if s.comment.is_empty() {
        //     writeln!(o, "/// A `{}`.", s.orig_name)?;
        // } else {
        //     writeln!(o, "/// {}.", s.comment)?;
        // }
        writeln!(o, "/// A `{}`.", s.orig_name)?;
        writeln!(o, "///")?;
        writeln!(o, "/// {}", s.comment)?;
        if is_experimental(&s.orig_name) {
            writeln!(o, "#[cfg(feature = \"experimental\")]")?;
        }
        writeln!(o, "#[derive(Debug, Clone, Default)]")?;
        writeln!(o, "#[repr(C)]")?;
        write!(o, "pub struct {}", s.voodoo_name)?;
        if s.contains_ptr { write!(o, "<'s>")?; }
        writeln!(o, " {{")?;

        writeln!(o, "{t}raw: {}{},", ORIG_PRE, s.orig_name, t=t)?;
        if s.contains_ptr {
            writeln!(o, "{t}_p: PhantomData<&'s ()>,", t=t)?;
        }
        write!(o, "}}\n\n")?;
        // END STRUCT

        // BEGIN IMPL
        if is_experimental(&s.orig_name) {
            writeln!(o, "#[cfg(feature = \"experimental\")]")?;
        }
        // let type_params = if s.contains_ptr { "<'s>" } else { "" };
        // write!(o, "impl")?;
        // if s.contains_ptr { write!(o, "<'s>")?; }
        // write!(o, " {}", s.voodoo_name)?;
        // if s.contains_ptr { write!(o, "<'s>")?; }
        // writeln!(o, " {{")?;

        let type_params = if s.contains_ptr { "<'s>" } else { "" };
        write!(o, "impl{} {}{}", type_params, s.voodoo_name, type_params)?;
        writeln!(o, " {{")?;

        if !s.returnedonly {
            let type_params = if s.contains_ptr { "<'b>" } else { "" };
            write!(o, "{t}pub fn builder{tp}() -> {}Builder{tp}", s.voodoo_name, tp=type_params, t=t)?;
            writeln!(o, " {{")?;
            writeln!(o, "{t}{t}{}Builder::new()", s.voodoo_name, t=t)?;
            write!(o, "{t}}}\n\n", t=t)?;
        }

        for m in &s.members {
            if member_is_excluded(&m.orig_name) { continue; }
            let unsafe_str = if function_is_unsafe(&m.orig_name) { " unsafe" } else { "" };
            let fixed_fn_name = fixup_function_name(&m.orig_name);
            let fn_name = match fixed_fn_name {
                Some(ref n) => n.as_str(),
                None => m.voodoo_name.as_str(),
            };
            writeln!(o, "{t}pub{} fn {}(&self) {{", unsafe_str, fn_name, t=t)?;

            write!(o, "{t}}}\n\n", t=t)?;
        }

        writeln!(o, "{t}pub fn raw(&self) -> &{}{} {{", ORIG_PRE, s.orig_name, t=t)?;
        writeln!(o, "{t}{t}&self.raw", t=t)?;
        writeln!(o, "{t}}}", t=t)?;

        write!(o, "}}\n\n\n")?;
        // END IMPL

        // BEGIN BUILDER STRUCT
        if !s.returnedonly {
            writeln!(o, "/// A builder for `{}`.", s.orig_name)?;
            writeln!(o, "///")?;
            writeln!(o, "/// {}", s.comment)?;
            if is_experimental(&s.orig_name) {
                writeln!(o, "#[cfg(feature = \"experimental\")]")?;
            }
            writeln!(o, "#[derive(Debug, Clone, Default)]")?;
            write!(o, "pub struct {}Builder", s.voodoo_name)?;
            if s.contains_ptr { write!(o, "<'b>")?; }
            writeln!(o, " {{")?;
            writeln!(o, "{t}raw: {}{},", ORIG_PRE, s.orig_name, t=t)?;
            if s.contains_ptr {
                writeln!(o, "{t}_p: PhantomData<&'b ()>,", t=t)?;
            }
            write!(o, "}}\n\n")?;
        }
        // END BUILDER STRUCT

        // BEGIN BUILDER IMPL
        if !s.returnedonly {
            if is_experimental(&s.orig_name) {
                writeln!(o, "#[cfg(feature = \"experimental\")]")?;
            }

            let bldr_type_param = if s.contains_ptr { "<'b>" } else { "" };
            write!(o, "impl{} {}Builder{}", bldr_type_param, s.voodoo_name, bldr_type_param)?;
            writeln!(o, " {{")?;
            writeln!(o, "{t}pub fn new() -> {}Builder{} {{", s.voodoo_name, bldr_type_param, t=t)?;
            writeln!(o, "{t}{t}{}Builder {{", s.voodoo_name, t=t)?;
            writeln!(o, "{t}{t}{t}raw: {}{}::default(),", ORIG_PRE, s.orig_name, t=t)?;
            if s.contains_ptr {
                writeln!(o, "{t}{t}{t}_p: PhantomData,", t=t)?;
            }
            writeln!(o, "{t}{t}}}", t=t)?;
            write!(o, "{t}}}\n\n", t=t)?;

            for m in &s.members {
                write_builder_set_fn(o, s, m, bldr_type_param)?
            }



            write!(o, "}}\n\n\n")?;
        }
        // END BUILDER IMPL
    }

    Ok(())
}

fn main() {
    let structs: Vec<Struct> = parse_structs();
    write_structs(&structs).unwrap();

}

