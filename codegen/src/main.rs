//! Parses type definitions in `vk.xml` and outputs Voodoo equivalent types
//! and wrappers.

#![allow(unused_mut, unused_variables, dead_code)]


extern crate xml;

use std::mem;
use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufReader, BufWriter};
use std::collections::HashMap;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;


const INDENT: &'static str = "    ";
const PRINT: bool = false;
const ORIG_USE: &str = "vks";
const ORIG_PRE: &str = "vks::";


/// Swaps an arbitrary member name with a replacement. The replacement name
/// will have been determined within the ffi definition lib (`vks`).
fn filter_member_name(orig: &mut String) {
    if orig == "type" {
        mem::replace(orig, "type_".to_string());
    }
}


/// Converts a pascal string to snake case and returns whether or not a "p" or
/// "pp" was pruned (indicating a pointer member).
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

/// Changes capitalization of extension and dimension type name suffixes.
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

/// Converts C/Vulkan names into Rust/Voodoo conventions.
fn convert_type_name(orig_type: &str) -> String {
    match orig_type {
        "float" => "f32".to_string(),
        "int32_t" => "i32".to_string(),
        "uint32_t" => "u32".to_string(),
        "char" => "i8".to_string(),
        "uint8_t" => "u8".to_string(),
        "void" => "c_void".to_string(),
        "size_t" => "usize".to_string(),
        "uint64_t" => "u64".to_string(),
        "int" => "i32".to_string(),
        "VkFlags" => "u32".to_string(),
        "VkBool32" => "bool".to_string(),
        "VkDeviceSize" => "u64".to_string(),
        "VkSampleMask" => "u32".to_string(),
        "VkResult" => "ResultEnum".to_string(),
        "VkSurfaceKHR" => "Surface".to_string(),
        "VkSwapchainKHR" => "Swapchain".to_string(),
        "Window" => "u32".to_string(),
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

/// Returns true if `orig_name` specifies a dispatchable or non-dispatchable
/// handle.
///
/// What's a hashmap?
fn struct_is_handle_type(orig_name: &str) -> bool {
    match orig_name {
        "VkInstance" => true,
        "VkPhysicalDevice" => true,
        "VkDevice" => true,
        "VkQueue" => true,
        "VkSemaphore" => true,
        "VkCommandBuffer" => true,
        "VkFence" => true,
        "VkDeviceMemory" => true,
        "VkBuffer" => true,
        "VkImage" => true,
        "VkEvent" => true,
        "VkQueryPool" => true,
        "VkBufferView" => true,
        "VkImageView" => true,
        "VkShaderModule" => true,
        "VkPipelineCache" => true,
        "VkPipelineLayout" => true,
        "VkRenderPass" => true,
        "VkPipeline" => true,
        "VkDescriptorSetLayout" => true,
        "VkSampler" => true,
        "VkDescriptorPool" => true,
        "VkDescriptorSet" => true,
        "VkFramebuffer" => true,
        "VkCommandPool" => true,
        "VkSurfaceKHR" => true,
        "VkSwapchainKHR" => true,
        "VkDisplayKHR" => true,
        "VkDisplayModeKHR" => true,
        "VkDescriptorUpdateTemplateKHR" => true,
        "VkSamplerYcbcrConversionKHR" => true,
        "VkDebugReportCallbackEXT" => true,
        "VkObjectTableNVX" => true,
        "VkIndirectCommandsLayoutNVX" => true,
        "VkValidationCacheEXT" => true,
        _ => false,
    }
}


/// A category of parsable type within the source spec. document.
#[derive(Clone, Debug, PartialEq, Eq)]
enum TypeCategory {
    None,
    Struct,
    Union,
    Enum,
    Other,
}

/// TODO: Make this a `::new`.
fn category(s: &str) -> TypeCategory {
    match s {
        "struct" => TypeCategory::Struct,
        "union" => TypeCategory::Union,
        "enum" => TypeCategory::Enum,
        _ => TypeCategory::Other,
    }
}

/// A member of a struct.
#[derive(Clone, Debug)]
struct Member {
    orig_type: String,
    voodoo_type: String,
    orig_name: String,
    voodoo_name: String,
    is_ptr: bool,
    is_ptr_ptr: bool,
    is_const: bool,
    is_const_const: bool,
    is_struct: bool,
    is_handle_type: bool,
    ptr_count_member_orig_name: Option<String>,
    is_ptr_count: bool,
    optional: bool,
    noautovalidity: bool,
    externsync: bool,
    array_len: Option<String>,
    comment: Option<String>,
    values: Option<String>,
    len: Option<String>,
    altlen: Option<String>,
}

impl Member {
    fn new(attribs: &[OwnedAttribute]) -> Member {
        let mut member = Member {
            orig_type: String::new(),
            voodoo_type: String::new(),
            orig_name: String::new(),
            voodoo_name: String::new(),
            is_ptr: false,
            is_ptr_ptr: false,
            is_const: false,
            is_const_const: false,
            is_struct: false,
            is_handle_type: false,
            ptr_count_member_orig_name: None,
            is_ptr_count: false,
            optional: false,
            noautovalidity: false,
            externsync: false,
            array_len: None,
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

    /// Specifies the name of a member, filtering it for an invalid name
    /// (currently only "type" is invalid) and generating a snake_case
    /// `voodoo_name` to use in the output code.
    fn set_name(&mut self, mut orig_name: String) {
        assert!(self.orig_name.is_empty());
        assert!(self.voodoo_name.is_empty());
        filter_member_name(&mut orig_name);
        let (voodoo_name, p_was_pruned, pp_was_pruned) = pascal_to_snake_case(&orig_name, true);
        assert!((p_was_pruned && self.is_ptr) || !p_was_pruned);
        assert!((pp_was_pruned && self.is_ptr_ptr) || !pp_was_pruned);
        // if orig_name.contains("Count") {
        //     self.is_count = true;
        // }
        self.voodoo_name = voodoo_name;
        self.orig_name = orig_name;
    }

    /// Specifies the type of a member, both original and converted, and
    /// whether or not the member is a dispatchable or non-dispatchable handle
    /// type.
    fn set_type(&mut self, orig_type: String) {
        assert!(self.orig_type.is_empty() && self.voodoo_type.is_empty());
        self.is_handle_type = struct_is_handle_type(&orig_type);
        self.voodoo_type = convert_type_name(&orig_type);
        self.orig_type = orig_type;
    }
}


/// A field that must be created within a generated struct to store referenced
/// information for convenience. Structs containing special fields are no
/// longer `repr(C)`.
#[derive(Clone, Debug)]
struct SpecialField {
    name: String,
    ty_struct: String,
    ty_builder: String,
    default_val: String,
}

fn special_field(m: &Member) -> Option<SpecialField> {
    if member_is_excluded(&m.orig_name) { return None; }

    match m.voodoo_type.as_str() {
         "i8" => {
            if m.is_ptr {
                return Some(SpecialField {
                    name: m.voodoo_name.clone(),
                    ty_struct: "Option<CharStr<'s>>".to_string(),
                    ty_builder: "Option<CharStr<'b>>".to_string(),
                    default_val: "None".to_string(),
                });
            } else if m.is_ptr_ptr {
                return Some(SpecialField {
                    name: m.voodoo_name.clone(),
                    ty_struct: "Option<CharStrs<'s>>".to_string(),
                    ty_builder: "Option<CharStrs<'b>>".to_string(),
                    default_val: "None".to_string(),
                });
            }
        },
        "PipelineShaderStageCreateInfo" |
        "DescriptorSetLayoutBinding" |
        "PhysicalDevice" |
        "DeviceMemory" |
        "DescriptorSetLayout" |
        "CommandBuffer" |
        "ImageView" |
        "Semaphore" |
        "Swapchain" |
        "Sampler" => {
            assert!(!m.is_ptr_ptr);
            if m.is_ptr {
                return Some(SpecialField {
                    name: m.voodoo_name.clone(),
                    ty_struct: format!("Option<SmallVec<[{}{}; 8]>>", ORIG_PRE, m.orig_type),
                    ty_builder: format!("Option<SmallVec<[{}{}; 8]>>", ORIG_PRE, m.orig_type),
                    default_val: "None".to_string(),
                });
            }
        },
        _ => (),
    }
    None
}


/// A struct type parsed from the API spec. which will be generated anew.
#[derive(Clone, Debug)]
struct Struct {
    orig_name: String,
    voodoo_name: String,
    returnedonly: bool,
    structextends: Option<String>,
    comment: String,
    members: Vec<Member>,
    contains_ptr: bool,
    is_handle_type: bool,
    special_fields: HashMap<String, SpecialField>,
}

impl Struct {
    fn new(attribs: &[OwnedAttribute]) -> Struct {
        let mut orig_name = None;
        let mut voodoo_name = None;
        let mut returnedonly = false;
        let mut structextends = None;
        let mut comment = String::new();
        let mut is_handle_type = false;

        for attrib in attribs {
            match attrib.name.local_name.as_str() {
                "category" => (),
                "name" => {
                    let name = attrib.value.clone();
                    voodoo_name = Some(convert_type_name(&name));
                    is_handle_type = struct_is_handle_type(&name);
                    orig_name = Some(name);
                },
                "returnedonly" => returnedonly |= attrib.value == "true",
                "structextends" => structextends = Some(String::from(attrib.value.clone())),
                "comment" => comment = attrib.value.clone(),
                unknown @ _ => panic!("unknown struct attribute: {:?}=\"{:?}\"",
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
            is_handle_type,
            special_fields: HashMap::new(),
        }
    }

    /// Searches for a member within this struct which matches the naming
    /// conventions used to relate pointers with corresponding count fields
    /// then marks that relationship.
    fn match_ptr_count_member(&mut self, m: &mut Member) {
        if !m.is_ptr && m.orig_name.len() > 5 && m.orig_name.contains("Count") {
            // Check for pointers members matching a count member. NOTE: This
            // will never find a match, the spec consistently defines count
            // members before their corresponding pointers.
            //
            // TODO: Implement Irregularities
            let (head, tail) = m.orig_name.split_at(m.orig_name.len() - 5);
            if tail == "Count" {
                for ptr_m in self.members.iter_mut().filter(|ptr_m|
                        (ptr_m.is_ptr || ptr_m.is_ptr_ptr) && ptr_m.orig_name.starts_with("p")) {
                    // let (ptr_first, ptr_tail) = ptr_m.orig_name.split_at(1);
                    let (ptr_pre, ptr_tail) = if m.orig_name.starts_with("pp") {
                        m.orig_name.split_at(2)
                    } else {
                        m.orig_name.split_at(1)
                    };
                    let (ptr_tail_first, ptr_tail_tail) = ptr_tail.split_at(1);
                    let ptr_tail_lower = format!("{}{}", ptr_tail_first.to_lowercase(), ptr_tail_tail);
                    if ptr_tail_lower.contains(head) {
                        if PRINT { println!("Pointer member found matching count member: \
                            ptr: \"{}\", cnt: \"{}\"", ptr_tail_lower, head); }
                        ptr_m.ptr_count_member_orig_name = Some(m.orig_name.clone());
                        m.is_ptr_count = true;
                        // NOTE: Currently unreachable: count always comes first.
                    }
                }
            }
        } else if (m.is_ptr || m.is_ptr_ptr) && m.orig_name.starts_with("p") {
            // Check for a count member matching a pointer member.
            let (ptr_pre, ptr_tail_) = if m.orig_name.starts_with("pp") {
                m.orig_name.split_at(2)
            } else {
                m.orig_name.split_at(1)
            };
            assert!(ptr_pre == "pp" || ptr_pre == "p");
            // Compensate for pointer member name irregularities:
            let mut ptr_tail = ptr_tail_
                .replace("ImageIndices", "Swapchains")
                .replace("ResolveAttachments", "ColorAttachments")
                .replace("Indices", "Indexes")
                .replace("Entries", "Entrys")
                .replace("Dependencies", "Dependencys");
            if self.orig_name == "VkPresentInfoKHR" {
                ptr_tail = ptr_tail.replace("Results", "Swapchains");
            } else if self.orig_name == "VkPresentRegionsKHR" {
                ptr_tail = ptr_tail.replace("Regions", "Swapchains");
            } else if self.orig_name == "VkPresentTimesInfoGOOGLE"  {
                ptr_tail = ptr_tail.replace("Times", "Swapchains");
            }

            let (ptr_tail_first, ptr_tail_tail) = ptr_tail.split_at(1);
            let ptr_tail_lower = format!("{}{}", ptr_tail_first.to_lowercase(), ptr_tail_tail);
            for cnt_m in self.members.iter_mut().filter(|cnt_m| cnt_m.orig_name.len() > 5) {
                // Compensate for count member name irregularities:
                let cnt_orig_name = cnt_m.orig_name
                    .replace("descriptorSet", "setLayout")
                    .replace("SFRRect", "sFRRect")
                    .replace("codeSize", "codeCount");
                let (cnt_head, cnt_tail) = cnt_orig_name.split_at(cnt_orig_name.len() - 5);
                // // Compensate for count member name irregularities:
                // let cnt_head = cnt_head_
                //     .replace("descriptorSet", "setLayout")
                //     .replace("SFRRect", "sFRRect");
                if ptr_tail_lower.contains(&cnt_head) && cnt_tail == "Count" {
                    assert!(ptr_tail_lower.starts_with(&cnt_head));
                    if PRINT { println!("Count member found matching pointer member: \
                            ptr: \"{}\", cnt: \"{}\"", ptr_tail_lower, cnt_head); }

                    // println!("Count member found matching pointer member: \
                    //         ptr: \"{}\", cnt: \"{}\"", ptr_tail_lower, cnt_head);

                    // Damn irregularities ("pCoverageModulationTable")....
                    assert!(m.orig_name.ends_with("s") ||
                        m.orig_name == "pCoverageModulationTable" ||
                        m.orig_name == "pCode");
                    m.ptr_count_member_orig_name = Some(cnt_m.orig_name.clone());
                    cnt_m.is_ptr_count = true;
                }
            }
        }
    }

    fn add_member(&mut self, mut m: Member) {
        if let Some(sf) = special_field(&m) {
            self.special_fields.reserve(4);
            self.special_fields.insert(m.voodoo_name.clone(), sf);
        }
        self.match_ptr_count_member(&mut m);
        self.members.push(m);
    }

    fn special_field(&self, member_voodoo_name: &str) -> Option<&SpecialField> {
        self.special_fields.get(member_voodoo_name)
    }

    fn is_repr_c(&self) -> bool {
        self.special_fields.len() == 0
    }
}


/// Parses and categorizes the usable data from the characters in the source
/// document.
fn parse_stray_text(s: &str, m: &mut Member) {
    match s {
        // Brackets alone will have sizes set by a value wrapped in an <enum> tag.
        "[" => (),
        "]" => (),
        _ => {
            if s.starts_with("* const*") {
                assert!(!m.is_ptr);
                m.is_ptr_ptr = true;
                if m.is_const {
                    m.is_const = false;
                    m.is_const_const = true;
                } else {
                    m.is_const = true;
                }
            } else if s.starts_with("const") {
                assert!(!m.is_const);
                assert!(!m.is_const_const);
                if m.is_const {
                    m.is_const = false;
                    m.is_const_const = true;
                } else {
                    m.is_const = true;
                }
            } else if s.starts_with("struct") {
                assert!(!m.is_struct);
                m.is_struct = true;
            } else if s.starts_with("*") {
                assert!(!m.is_ptr);
                m.is_ptr = true;
            } else if s.starts_with("[") {
                let mut array_len = String::with_capacity(4);
                for (char_idx, c) in s.chars().enumerate() {
                    match c {
                        '[' => (),
                        ']' => assert!(char_idx == s.len() - 1),
                        digit @ _ => {
                            assert!(digit.is_numeric(),
                                "unexpected character found \
                                while parsing array size: {}", c);
                            array_len.push(digit);
                        },
                    }
                }
                m.array_len = Some(array_len);
            } else {
                panic!("unknown characters present: {}", s)
            }
        }
    }
}

/// Used for printing purposes (and taken from the original xml-rs example).
fn indent(size: usize) -> String {
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

/// Parses a source XML API spec. and pulls out (currently only) structs.
fn parse_structs() -> (HashMap<String, Struct>, Vec<String>) {
    let file = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/gen_src/vk.xml")).unwrap();
    let reader = BufReader::new(file);
    let parser = EventReader::new(reader);

    let mut structs = HashMap::with_capacity(300);
    let mut struct_order = Vec::with_capacity(300);

    let mut current_struct: Option<Struct> = None;
    let mut struct_start_depth = 0;
    let mut parsing_struct_comment = false;

    let mut current_member: Option<Member> = None;
    let mut member_start_depth = 0;
    let mut parsing_member_type = false;
    let mut parsing_member_name = false;
    let mut parsing_member_array_len = false;
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
                            parsing_member_array_len = true;
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
                        let s = current_struct.as_mut().expect("no current struct");
                        let new_member = current_member.take().expect("no current member");
                        s.add_member(new_member)
                    }
                } else if name.local_name == "type" && current_struct.is_some() {
                    if depth == struct_start_depth {
                        assert!(current_struct.is_some());
                        if let Some(s) = current_struct.take() {
                            let key = s.voodoo_name.clone();
                            struct_order.push(key.clone());
                            structs.insert(key, s);

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
                            cur_mem.set_type(s);
                            parsing_member_type = false;
                        } else if parsing_member_name {
                            cur_mem.set_name(s);
                            parsing_member_name = false;
                        } else if parsing_member_array_len {
                            cur_mem.array_len = Some(s);
                            parsing_member_array_len = false;
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

    println!("{} structs parsed", structs.len());
    (structs, struct_order)
}

/// Returns true if a struct with a certain name is to be ignored.
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

/// Returns true if a member is to be ignored during generation.
fn member_is_excluded(orig_name: &str) -> bool {
    match orig_name {
        "sType" => true,
        _ => false,
    }
}

/// Returns true if the member is a pointer that must be treated unsafely.
fn function_is_unsafe(m: &Member) -> bool {
    match m.orig_name.as_str() {
        "pNext" => true,
        "pTag" => true,
        "pUserData" => true,
        "window" => true,
        "pData" => m.is_ptr,
        "connection" => m.is_ptr,
        "mir_surface" => m.is_ptr,
        "pInitialData" => m.is_ptr,
        "mirSurface" => m.is_ptr,
        "display" => m.is_ptr,
        "surface" => m.is_ptr,
        "dpy" => m.is_ptr,
        "pView" => m.is_ptr,
        _ => false,
    }
}

/// Filters a function name. Similar to `::filter_member_name` except modifies
/// the name of a (visible) function rather than a field.
fn filter_function_name(orig_name: &str) -> Option<String> {
    match orig_name {
        "type_" => Some("type_of".to_string()),
        _ => None,
    }
}

/// Returns true if a type name is to be marked with the `experimental`
/// feature gate.
fn is_experimental(orig_name: &str) -> bool {
    orig_name.contains("KHX") || orig_name.contains("NVX")
}

/// Returns true if the argument is to be interpreted as a slice rather than a
/// reference. This is fiddly, fragile, and contains irregularities.
fn arg_is_slice(m: &Member, fn_name: &str) -> bool {
    m.is_ptr && (fn_name.ends_with("s")
        && fn_name != "enabled_features"
        && fn_name != "attributes"
        || fn_name == "code"
        || fn_name == "coverage_modulation_table")

}

/// The getter and setter signatures of a struct member. And all kinds of
/// other stuff.
struct MemberSig {
    fn_name: String,
    arg_lifetime: &'static str,
    unsafe_to_set: bool,
    arg_is_slice: bool,
    arg_is_struct: bool,
    arg_is_repr_c: bool,
    convert_arg: bool,
    convert_arg_twice: bool,
    convert_to_bits: bool,
    convert_return: bool,
    convert_return_to_c_str: bool,
    convert_return_to_slice: bool,
    convert_return_to_flags: bool,
    // cast_return_as_voodoo_type: bool,
    arg_type: String,
    return_type: String,
    where_clause: String,
    set_fn_type_params: String,
    get_fn_type_params: String,
}

impl MemberSig {
    fn new(m: &Member, s: &Struct, structs: &HashMap<String, Struct>,
            bldr_type_params: &str,) -> MemberSig {
        // The function name, blemishes removed:
        let fn_name = match filter_function_name(&m.orig_name) {
            Some(filtered_name) => filtered_name,
            None => m.voodoo_name.clone(),
        };

        let arg_is_slice = arg_is_slice(m, &fn_name);
        let arg_is_repr_c = match structs.get(&m.voodoo_type) {
            Some(ts) => ts.is_repr_c(),
            None => false,
        };

        let mut sig = MemberSig {
            fn_name,
            arg_lifetime: "'a",
            unsafe_to_set: function_is_unsafe(&m),
            arg_is_slice,
            arg_is_struct: structs.contains_key(&m.voodoo_type),
            arg_is_repr_c,
            convert_arg: false,
            convert_arg_twice: false,
            convert_to_bits: false,
            convert_return: false,
            convert_return_to_c_str: false,
            convert_return_to_slice: false,
            convert_return_to_flags: false,
            arg_type: String::with_capacity(128),
            return_type: String::with_capacity(128),
            where_clause: String::with_capacity(128),
            set_fn_type_params: "'m".to_string(),
            get_fn_type_params: "'a".to_string(),
        };

        // Type signature:
        if let Some(ref len) = m.array_len {
            if m.voodoo_type == "i8" {
                sig.convert_return_to_c_str = true;
                sig.return_type.push_str("&'a CStr");
            } else {
                sig.convert_return_to_slice = true;
                sig.arg_type.push_str("[");
                sig.arg_type.push_str(&m.voodoo_type);
                sig.arg_type.push_str("; ");
                match len.parse::<usize>() {
                    Ok(_) => sig.arg_type.push_str(len),
                    Err(_) => {
                        sig.arg_type.push_str(ORIG_PRE);
                        sig.arg_type.push_str(len);
                    },
                }
                sig.arg_type.push_str("]");
                sig.return_type.push_str(&format!("&[{}]", m.voodoo_type));
            }
        } else if m.voodoo_type == "i8" {
            assert!(s.special_field(&m.voodoo_name).is_some());
            sig.convert_arg = true;
            sig.set_fn_type_params.push_str(", ");
            sig.set_fn_type_params.push_str(sig.arg_lifetime);
            sig.set_fn_type_params.push_str(", T");
            sig.arg_type.push_str("T");
            if m.is_ptr {
                assert!(bldr_type_params.contains("'b"));
                sig.where_clause = format!(" where {a}: 'b, T: Into<CharStr<{a}>>", a=sig.arg_lifetime);
                sig.return_type.push_str("&'a CStr");
                sig.convert_return_to_c_str = true;
            } else if m.is_ptr_ptr {
                assert!(bldr_type_params.contains("'b"));
                sig.where_clause = format!(" where {a}: 'b, T: Into<CharStrs<{a}>>", a=sig.arg_lifetime);
                sig.return_type.push_str("&'a [*const c_char]");
                sig.convert_return_to_slice = true;
            }
        } else if m.voodoo_type == "u32" && m.orig_name.contains("Version") {
            sig.convert_arg = true;
            sig.convert_arg_twice = true;
            sig.convert_return = true;
            sig.set_fn_type_params.push_str(", T");
            sig.arg_type.push_str("T");
            sig.where_clause = format!(" where T: Into<Version>");
            sig.return_type.push_str("Version");
        } else if m.voodoo_type.contains("Flags") {
            if m.voodoo_type == "PipelineStageFlags" {
                // Irregularity.
                if m.is_ptr {
                    sig.set_fn_type_params.push_str(", ");
                    sig.set_fn_type_params.push_str(sig.arg_lifetime);
                    sig.arg_type.push_str("&");
                    sig.arg_type.push_str(sig.arg_lifetime);
                    sig.arg_type.push_str(" ");
                } else {
                    sig.convert_to_bits = true;
                    sig.convert_return_to_flags = true;
                }
            } else {
                sig.convert_to_bits = true;
                sig.convert_return_to_flags = true;
            }
            sig.arg_type.push_str(&m.voodoo_type);
            sig.return_type.push_str(&sig.arg_type);
        } else if sig.arg_is_slice {
            sig.set_fn_type_params.push_str(", ");
            sig.set_fn_type_params.push_str(sig.arg_lifetime);
            sig.arg_type.push_str("&");
            sig.arg_type.push_str(sig.arg_lifetime);
            sig.arg_type.push_str(" ");
            assert!(!m.is_const_const);
            if !m.is_const {
                sig.arg_type.push_str("mut ");
            }
            sig.arg_type.push_str("[");
            if m.is_handle_type {
                sig.arg_type.push_str("&");
                sig.arg_type.push_str(sig.arg_lifetime);
                sig.arg_type.push_str(" ");
                assert!(bldr_type_params.contains("'b"));
                sig.where_clause = format!(" where {a}: 'b", a=sig.arg_lifetime);
            }
            sig.arg_type.push_str(&m.voodoo_type);
            sig.arg_type.push_str("]");
            if sig.arg_is_repr_c {
                sig.return_type.push_str(&sig.arg_type);
            } else {
                sig.return_type.push_str("&");
                sig.return_type.push_str(sig.arg_lifetime);
                sig.return_type.push_str(" ");
                sig.return_type.push_str("[");
                // if m.is_const {
                //     sig.return_type.push_str("*const ");
                // } else {
                //     sig.return_type.push_str("*mut ");
                // }
                if sig.arg_is_struct || m.is_handle_type {
                    sig.return_type.push_str(ORIG_PRE);
                    sig.return_type.push_str(&m.orig_type);
                } else {
                    sig.return_type.push_str(&m.voodoo_type);
                }
                sig.return_type.push_str("]");
            }
        } else if m.is_ptr && sig.unsafe_to_set {
            if m.is_const {
                sig.arg_type.push_str("*const ");
            } else {
                sig.arg_type.push_str("*mut ");
            }
            sig.arg_type.push_str(&m.voodoo_type);
            sig.return_type.push_str(&sig.arg_type);
        } else {
            if m.is_ptr || m.is_handle_type {
                sig.set_fn_type_params.push_str(", ");
                sig.set_fn_type_params.push_str(sig.arg_lifetime);
                sig.arg_type.push_str("&");
                sig.arg_type.push_str(sig.arg_lifetime);
                sig.arg_type.push_str(" ");
            }
            sig.arg_type.push_str(&m.voodoo_type);
            if !m.is_handle_type {
                // Irregularities:
                if m.orig_name != "pSampleMask" &&
                    m.orig_name != "pCoverageModulationTable" {
                    sig.convert_arg = true;
                    sig.convert_return = true;
                }
            }
            sig.return_type.push_str(&sig.arg_type);
        }

        sig
    }
}


/// Writes a setter function to the output buffer.
fn write_set_fn(o: &mut BufWriter<File>, s: &Struct, m: &Member, bldr_type_params: &str,
        structs: &HashMap<String, Struct>, is_for_builder: bool) -> io::Result<()> {
    let t = INDENT;
    // Skip excluded members and "...Count" members that have associated
    // pointer members (stuff that gets merged into a slice).
    if member_is_excluded(&m.orig_name) { return Ok(()); }
    if m.is_ptr_count { return Ok(()); }

    // let fn_is_unsafe = function_is_unsafe(&m);

    // // The function name, blemishes removed:
    // let fn_name = match filter_function_name(&m.orig_name) {
    //     Some(filtered_name) => filtered_name,
    //     None => m.voodoo_name.clone(),
    // };

    // Signature and all kinds of other stuff:
    let sig = MemberSig::new(m, s, structs, bldr_type_params);

    // Are you wearing a seatbelt?
    let unsafe_str = if sig.unsafe_to_set { " unsafe" } else { "" };

    // Function signature:
    writeln!(o, "{t}pub{} fn {}<{}>(mut self, {}: {}) -> {}Builder{}{} {{",
        unsafe_str, sig.fn_name, sig.set_fn_type_params, sig.fn_name, sig.arg_type,
        s.voodoo_name, bldr_type_params, sig.where_clause, t=t)?;

    let set_counts = |o: &mut BufWriter<File>, extra_indent: &str| -> io::Result<()> {
        if let Some(ref count_orig_name) = m.ptr_count_member_orig_name {
            writeln!(o, "{t}{t}{x}assert!(self.raw.{c} == 0 || self.raw.{c} == {f}.len() as _, \n\
                {t}{t}{t}{x}\"count inconsistency found when specifying `{s}::{f}`.\");",
                c=count_orig_name, f=sig.fn_name, s=s.voodoo_name, t=t, x=extra_indent)?;
            writeln!(o, "{t}{t}{x}self.raw.{c} = {f}.len() as _;", c=count_orig_name, f=sig.fn_name,
                t=t, x=extra_indent)?;
        }
        Ok(())
    };

    if s.special_field(&m.voodoo_name).is_some() {
        write!(o, "{t}{t}self.{} = Some({}", m.voodoo_name, sig.fn_name, t=t)?;
        if sig.arg_is_slice {
            if m.is_handle_type {
                write!(o, ".iter().map(|h| h.handle()).collect()")?;
            } else if sig.arg_is_struct {
                write!(o, ".iter().map(|h| h.raw).collect()")?;
            }
        } else if sig.convert_arg {
            write!(o, ".into()")?;
        }
        writeln!(o, ");")?;
        writeln!(o, "{t}{t}{{", t=t)?;
        writeln!(o, "{t}{t}{t}let {} = self.{}.as_ref().unwrap();", m.voodoo_name, sig.fn_name, t=t)?;
        writeln!(o, "{t}{t}{t}self.raw.{} = {}.as_ptr();",
            m.orig_name, m.voodoo_name, t=t)?;
        set_counts(o, t)?;
        writeln!(o, "{t}{t}}}", t=t)?;
    } else {
        set_counts(o, "")?;
        write!(o, "{t}{t}self.raw.{} = ", m.orig_name, t=t)?;
        if sig.arg_is_struct {
            if m.is_ptr {
                if sig.arg_is_slice && sig.arg_is_repr_c {
                    if m.is_const {
                        write!(o, "{}.as_ptr() as *const ", sig.fn_name)?;
                        write!(o, "{}{} as *const _", ORIG_PRE, m.orig_type)?;
                    } else {
                        write!(o, "{}.as_mut_ptr() as *mut ", sig.fn_name)?;
                        write!(o, "{}{} as *mut _", ORIG_PRE, m.orig_type)?;
                    }
                } else {
                    write!(o, "{}.raw()", sig.fn_name)?;
                }
            } else {
                if let Some(ref len) = m.array_len {
                    write!(o, "[")?;
                    for idx in 0..len.parse::<u32>().unwrap() {
                        write!(o, "{}[{}].raw, ", sig.fn_name, idx)?;
                    }
                    write!(o, "]")?;
                } else {
                    write!(o, "{}.raw", sig.fn_name)?;
                }
            }
        } else {
            if m.is_handle_type {
                if sig.arg_is_slice {
                    assert!(s.special_fields.contains_key(&m.voodoo_name),
                        "\"{}\" is lacking a special field for {{ {}: {} }}",
                        s.voodoo_name, sig.fn_name, sig.arg_type);
                    write!(o, "{}", sig.fn_name)?;
                } else {
                    if m.is_ptr {
                        write!(o, "&")?;
                    }
                    write!(o, "{}.handle()", sig.fn_name)?;
                }
            } else if m.voodoo_type.as_str() == "bool" {
                write!(o, "{} as u32", sig.fn_name)?;
            } else if sig.convert_to_bits {
                write!(o, "{}.bits()", sig.fn_name)?;
            } else if sig.arg_is_slice {
                if m.is_const {
                    write!(o, "{}.as_ptr() as *const {} as *const _", sig.fn_name, m.voodoo_type)?;
                } else {
                    write!(o, "{}.as_mut_ptr() as *mut {} as *mut _", sig.fn_name, m.voodoo_type)?;
                }
            } else if sig.convert_arg && !m.is_ptr {
                write!(o, "{}.into()", sig.fn_name)?;
                if sig.convert_arg_twice {
                    write!(o, ".into()")?;
                }
            } else if m.voodoo_type == "PipelineStageFlags" && m.is_ptr {
                write!(o, "{} as *const PipelineStageFlags as *const _", sig.fn_name)?;
            } else {
                write!(o, "{}", sig.fn_name)?;
            }
        }
        writeln!(o, ";")?;
    }
    writeln!(o, "{t}{t}self", t=t)?;

    write!(o, "{t}}}\n\n", t=t)?;

    Ok(())
}


/// Writes a getter function to the output buffer.
fn write_get_fn(o: &mut BufWriter<File>, s: &Struct, m: &Member, bldr_type_params: &str,
        structs: &HashMap<String, Struct>, is_for_builder: bool) -> io::Result<()> {
    let t = INDENT;
    if member_is_excluded(&m.orig_name) || m.is_ptr_count { return Ok(()); }
    let sig = MemberSig::new(m, s, structs, bldr_type_params);

    let fn_name = if m.is_handle_type {
        format!("{}_handle", sig.fn_name)
    } else {
        sig.fn_name
    };

    let return_type = if m.is_handle_type {
        if let Some(_) = m.ptr_count_member_orig_name {
            format!("&'a [{}{}]", ORIG_PRE, m.orig_type)
        } else if m.is_ptr {
            format!("&'a {}{}", ORIG_PRE, m.orig_type)
        } else {
            format!("{}{}", ORIG_PRE, m.orig_type)
        }
    } else {
        sig.return_type
    };

    writeln!(o, "{t}pub fn {}<{}>(&'a self) -> {} {{", fn_name, sig.get_fn_type_params,
        return_type, t=t)?;

    if sig.arg_is_struct {
        if m.is_ptr {
            if sig.arg_is_slice {
                if let Some(ref count_orig_name) = m.ptr_count_member_orig_name {
                    if sig.arg_is_repr_c {
                        write!(o, "{t}{t}", t=t)?;
                        write!(o, "unsafe {{ slice::from_raw_parts(self.raw.{} as *const _, \
                            self.raw.{} as usize) }}", m.orig_name, count_orig_name)?;
                    } else {
                        write!(o, "{t}{t}", t=t)?;
                        write!(o, "unsafe {{ slice::from_raw_parts(self.raw.{} as *const _, \
                            self.raw.{} as usize) }}", m.orig_name, count_orig_name)?;
                    }
                } else {
                    // write!(o, "{t}{t}&*(self.raw.{} as *const {}{} as *const _)", m.orig_name,
                    //     ORIG_PRE, m.orig_type, t=t)?;
                    // write!(o, "{t}{t}", t=t)?;
                    // write!(o, "unsafe {{ slice::from_raw_parts(self.raw.{} as *const _, \
                    //     self.raw.{} as usize) }}", m.orig_name, count_orig_name)?;
                }
            } else {
                write!(o, "{t}{t}unsafe {{ &*(self.raw.{} as *const {}{} as *const _) }}", m.orig_name,
                    ORIG_PRE, m.orig_type, t=t)?;
            }
        } else {
            if let Some(ref len) = m.array_len {
                write!(o, "{t}{t}", t=t)?;
                write!(o, "unsafe {{ slice::from_raw_parts(&self.raw.{} \
                    as *const {}{} as *const _, ", m.orig_name, ORIG_PRE, m.orig_type)?;
                match len.parse::<usize>() {
                    Ok(_) => write!(o, "{} as usize) ", len)?,
                    Err(_) => write!(o, "{}{} as usize) ", ORIG_PRE, len)?,
                }
                write!(o, "}}",  )?;
            } else {
                // write!(o, "{}.raw", sig.fn_name)?;
                write!(o, "{t}{t}self.raw.{}", m.orig_name, t=t)?;
                write!(o, ".into()")?;
            }
        }
    } else {
        // write!(o, "{t}{t}self.raw.{}", m.orig_name, t=t)?;
        if let Some(ref count_orig_name) = m.ptr_count_member_orig_name {
            write!(o, "{t}{t}", t=t)?;
            write!(o, "unsafe {{ slice::from_raw_parts(self.raw.{} as *const _, self.raw.{} as usize) }}",
                m.orig_name, count_orig_name)?;
        } else if sig.convert_return_to_c_str {
            write!(o, "{t}{t}", t=t)?;
            write!(o, "unsafe {{ CStr::from_ptr(")?;
            match m.array_len {
                Some(_) => write!(o, "&self.raw.{} as *const _", m.orig_name)?,
                None => write!(o, "self.raw.{}", m.orig_name)?,
            }
            write!(o, ") }}")?;
        } else if sig.convert_return_to_slice {
            match m.array_len {
                Some(ref len) => {
                    // write!(o, "{t}{t}", t=t)?;
                    // write!(o, "unsafe {{ slice::from_raw_parts(self.raw.{} as *const _, \
                    //     {} as usize) }}", m.orig_name, len)?;

                    write!(o, "{t}{t}", t=t)?;
                    write!(o, "unsafe {{ slice::from_raw_parts(&self.raw.{} as *const _, ", m.orig_name)?;
                    match len.parse::<usize>() {
                        Ok(_) => write!(o, "{} as usize) ", len)?,
                        Err(_) => write!(o, "{}{} as usize) ", ORIG_PRE, len)?,
                    }
                    write!(o, "}}",  )?;
                },
                None => panic!("supposed to convert to slice but no len avail"),
            }
        } else if sig.convert_return_to_flags {
            write!(o, "{t}{t}{}::from_bits(self.raw.{})\n{t}{t}{t}\
                .expect(\"{}::{}: error converting flags\")",
                m.voodoo_type, m.orig_name, s.voodoo_name, fn_name, t=t)?;
        } else if m.voodoo_type.as_str() == "bool" {
            write!(o, "{t}{t}self.raw.{} != 0", m.orig_name, t=t)?;
        } else if sig.convert_return && !m.is_ptr {
            write!(o, "{t}{t}self.raw.{}", m.orig_name, t=t)?;
            write!(o, ".into()")?;
        } else if m.is_handle_type && m.is_ptr {
            write!(o, "{t}{t}unsafe {{ &*(self.raw.{} as *const _) }}", m.orig_name, t=t)?;
        } else if m.is_ptr {
            if sig.unsafe_to_set {
                write!(o, "{t}{t}self.raw.{}", m.orig_name, t=t)?;
            // } else if sig.arg_is_slice {

            } else {
                write!(o, "{t}{t}unsafe {{ &*(self.raw.{} as *const _) }}", m.orig_name, t=t)?;
            }
        } else {
            write!(o, "{t}{t}self.raw.{}", m.orig_name, t=t)?;
        }
    }


    // IF IS_REPR_C:
    // unsafe { *(&self.raw.memoryTypes
    //         as *const [vks::VkMemoryType; vks::VK_MAX_MEMORY_TYPES]
    //         as *const [MemoryType; vks::VK_MAX_MEMORY_TYPES]) }

    write!(o, "\n")?;

    write!(o, "{t}}}\n\n", t=t)?;
    Ok(())
}


/// Writes struct and corresponding builder definitions to an output file
/// which is overwritten if it exists.
fn write_structs(structs: &HashMap<String,Struct>, struct_order: &[String]) -> io::Result<()> {
    // let output_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/output/structs.rs");
    let output_file_path = "/src/voodoo/src/structs.rs";
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
    writeln!(o, "#![allow(unused_mut)]")?;
    write!(o, "\n")?;
    writeln!(o, "use std::ptr;")?;
    writeln!(o, "use std::ffi::{{CString, CStr}};")?;
    writeln!(o, "use std::marker::PhantomData;")?;
    writeln!(o, "use std::slice;")?;
    writeln!(o, "use libc::{{c_void, c_char}};")?;
    writeln!(o, "use num_traits::ToPrimitive;")?;
    writeln!(o, "use smallvec::SmallVec;")?;
    writeln!(o, "use ::*;")?;
    writeln!(o, "use {};", ORIG_USE)?;
    writeln!(o, "use {}::{{PFN_vkAllocationFunction, PFN_vkReallocationFunction, \
        PFN_vkFreeFunction, \n{t}PFN_vkInternalAllocationNotification, \
        PFN_vkInternalFreeNotification, \n{t}PFN_vkDebugReportCallbackEXT}};", ORIG_USE, t=t)?;
    write!(o, "\n\n")?;

    writeln!(o, "")?;

    for s_key in struct_order {
        let s = structs.get(s_key).unwrap();
        if struct_is_excluded(&s.orig_name) { continue; }

        // ################## STRUCT ####################
        writeln!(o, "/// A `{}`.", s.orig_name)?;
        writeln!(o, "///")?;
        writeln!(o, "/// {}", s.comment)?;
        if is_experimental(&s.orig_name) {
            writeln!(o, "#[cfg(feature = \"experimental\")]")?;
        }
        writeln!(o, "#[derive(Debug, Clone, Default)]")?;
        if s.is_repr_c() {
            writeln!(o, "#[repr(C)]")?;
        }
        write!(o, "pub struct {}", s.voodoo_name)?;
        if s.contains_ptr { write!(o, "<'s>")?; }
        writeln!(o, " {{")?;

        // Raw:
        writeln!(o, "{t}raw: {}{},", ORIG_PRE, s.orig_name, t=t)?;
        // Special fields:
        for (_, field) in &s.special_fields {
            writeln!(o, "{t}{}: {},", field.name, field.ty_struct, t=t)?;
        }
        // Phantom data:
        if s.contains_ptr {
            writeln!(o, "{t}_p: PhantomData<&'s ()>,", t=t)?;
        }

        write!(o, "}}\n\n")?;

        // ################# STRUCT IMPL #################
        if is_experimental(&s.orig_name) {
            writeln!(o, "#[cfg(feature = \"experimental\")]")?;
        }

        let struct_type_param = if s.contains_ptr { "<'s>" } else { "" };
        let bldr_type_param = if s.contains_ptr { "<'b>" } else { "" };
        write!(o, "impl{} {}{}", struct_type_param, s.voodoo_name, struct_type_param)?;
        writeln!(o, " {{")?;

        if !s.returnedonly {
            write!(o, "{t}pub fn builder{tp}() -> {}Builder{tp}", s.voodoo_name, tp=bldr_type_param, t=t)?;
            writeln!(o, " {{")?;
            writeln!(o, "{t}{t}{}Builder::new()", s.voodoo_name, t=t)?;
            write!(o, "{t}}}\n\n", t=t)?;
        }

        for m in &s.members {
            // Write getter function:
            write_get_fn(o, s, m, bldr_type_param, structs, false)?;
        }

        writeln!(o, "{t}pub fn raw(&self) -> &{}{} {{", ORIG_PRE, s.orig_name, t=t)?;
        writeln!(o, "{t}{t}&self.raw", t=t)?;
        writeln!(o, "{t}}}", t=t)?;

        write!(o, "}}\n\n\n")?;

        // ################ IMPL FROM/INTO STRUCT ################

        if is_experimental(&s.orig_name) {
            writeln!(o, "#[cfg(feature = \"experimental\")]")?;
        }
        write!(o, "impl{} From<{}{}> for {}{}", struct_type_param, s.voodoo_name, struct_type_param,
            ORIG_PRE, s.orig_name,)?;
        writeln!(o, " {{")?;
        writeln!(o, "{t}fn from(f: {}{}) -> {}{} {{", s.voodoo_name, struct_type_param,
            ORIG_PRE, s.orig_name, t=t)?;
        writeln!(o, "{t}{t}f.raw", t=t)?;
        writeln!(o, "{t}}}", t=t)?;
        write!(o, "}}\n\n\n")?;

        if is_experimental(&s.orig_name) {
            writeln!(o, "#[cfg(feature = \"experimental\")]")?;
        }
        write!(o, "impl{} From<{}{}> for {}{}", struct_type_param, ORIG_PRE, s.orig_name,
            s.voodoo_name, struct_type_param)?;
        writeln!(o, " {{")?;
        writeln!(o, "{t}fn from(f: {}{}) -> {}{} {{", ORIG_PRE, s.orig_name,
            s.voodoo_name, struct_type_param, t=t)?;
        write!(o, "{t}{t}{} {{ raw: f, ", s.voodoo_name, t=t)?;
        for (_, field) in &s.special_fields {
            write!(o, "{}: {}, ", field.name, field.default_val)?;
        }
        if s.contains_ptr {
            write!(o, "_p: PhantomData ")?;
        }
        writeln!(o, "}}")?;
        writeln!(o, "{t}}}", t=t)?;

        write!(o, "}}\n\n\n")?;

        // ################ BUILDER ################
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

            // Raw:
            writeln!(o, "{t}raw: {}{},", ORIG_PRE, s.orig_name, t=t)?;
            // Special fields:
            for (_, field) in &s.special_fields {
                writeln!(o, "{t}{}: {},", field.name, field.ty_builder, t=t)?;
            }
            // Phantom data:
            if s.contains_ptr {
                writeln!(o, "{t}_p: PhantomData<&'b ()>, ", t=t)?;
            }
            write!(o, "}}\n\n")?;
        }

        // ############## BUILDER IMPL ##############
        if !s.returnedonly {
            if is_experimental(&s.orig_name) {
                writeln!(o, "#[cfg(feature = \"experimental\")]")?;
            }

            // let bldr_type_param = if s.contains_ptr { "<'b>" } else { "" };
            write!(o, "impl{} {}Builder{}", bldr_type_param, s.voodoo_name, bldr_type_param)?;
            writeln!(o, " {{")?;
            // NEW:
            writeln!(o, "{t}pub fn new() -> {}Builder{} {{", s.voodoo_name, bldr_type_param, t=t)?;
            writeln!(o, "{t}{t}{}Builder {{", s.voodoo_name, t=t)?;
            // Raw:
            writeln!(o, "{t}{t}{t}raw: {}{}::default(),", ORIG_PRE, s.orig_name, t=t)?;
            // Special fields:
            for (_, field) in &s.special_fields {
                writeln!(o, "{t}{t}{t}{}: {},", field.name, field.default_val, t=t)?;
            }
            // Phantom data:
            if s.contains_ptr {
                writeln!(o, "{t}{t}{t}_p: PhantomData,", t=t)?;
            }
            writeln!(o, "{t}{t}}}", t=t)?;
            write!(o, "{t}}}\n\n", t=t)?;

            for m in &s.members {
                // Write setter function:
                write_set_fn(o, s, m, bldr_type_param, &structs, true)?
            }

            // BUILD:
            write!(o, "{t}pub fn build(self) -> {}{p}", s.voodoo_name, p=bldr_type_param, t=t)?;
            writeln!(o," {{")?;
            writeln!(o, "{t}{t}{} {{", s.voodoo_name, t=t)?;
            // Raw:
            writeln!(o, "{t}{t}{t}raw: self.raw,", t=t)?;
            // Special fields:
            for (_, field) in &s.special_fields {
                writeln!(o, "{t}{t}{t}{fn}: self.{fn},", fn=field.name, t=t)?;
            }
            // Phantom data:
            if s.contains_ptr {
                writeln!(o, "{t}{t}{t}_p: PhantomData,", t=t)?;
            }
            writeln!(o, "{t}{t}}}", t=t)?;
            write!(o, "{t}}}\n\n", t=t)?;

            write!(o, "}}\n\n\n")?;
        }
    }

    Ok(())
}

fn main() {
    let (structs, struct_order) = parse_structs();
    write_structs(&structs, &struct_order).unwrap();

}

