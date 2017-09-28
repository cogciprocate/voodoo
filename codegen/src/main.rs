#![allow(unused_imports, dead_code, unused_variables, unused_mut)]

extern crate xml;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

/* The Plan:

- Open the file
- if a line begins with typedef struct, parse until a `}` is reached.
- Store type and member names, converting type names and member names (storing both
  camelCase and snake_case versions for later)

*/

#[derive(Clone, Debug)]
struct Member {
    name: String,
    ty: String,
}

#[derive(Clone, Debug)]
struct Struct {
    name: String,
    returnedonly: bool,
    members: Vec<Member>,
}

impl Struct {
    fn new(attribs: &Vec<OwnedAttribute>) -> Struct {
        let mut name = None;
        let mut returnedonly = false;

        for attrib in attribs {
            match attrib.name.local_name.as_str() {
                "category" => (),
                "name" => name = Some(attrib.value.clone()),
                "returnedonly" => if attrib.value == "true" { returnedonly = true },
                unknown @ _ => panic!("unknown struct attribute: {:?}={:?}",
                    unknown, attrib.value),
            }
        }

        Struct {
            name: name.expect("no struct name found"),
            returnedonly,
            members: vec![],
        }
    }
}


fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {
    let file = File::open("./gen_src/vk.xml").unwrap();
    let reader = BufReader::new(file);
    let parser = EventReader::new(reader);

    let mut structs: Vec<Struct> = Vec::with_capacity(400);

    let mut current_struct: Option<Struct> = None;
    let mut current_member: Option<Member> = None;

    let mut parsing_struct = false;
    let mut parsing_struct_depth = 0;

    let mut depth = 0;
    let mut struct_count = 0;

    for e in parser {
        if struct_count > 5 { break; }
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name == "type" &&
                        attributes.len() > 0 &&
                        attributes[0].name.local_name == "category" {
                    match attributes[0].value.as_str() {
                        "struct" => {
                            current_struct = Some(Struct::new(&attributes));
                            parsing_struct = true;
                            parsing_struct_depth = depth;
                        },
                        _ => (),
                    }
                }

                if parsing_struct {
                    print!("{}<{}", indent(depth), name);
                    for attrib in attributes {
                        print!(" {}=\"{}\"", attrib.name, attrib.value);
                    }
                    print!(">\n");
                }

                depth += 1;
            },
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;

                if parsing_struct {
                    println!("{}</{}>", indent(depth), name);
                }

                if name.local_name == "type" {
                    if parsing_struct {
                        if depth == parsing_struct_depth {
                            structs.push(current_struct.take().unwrap());
                            struct_count += 1;
                            parsing_struct = false;
                        }
                    }
                }
            },
            // Ok(XmlEvent::CData(s)) => println!("{}{}", indent(depth), s),
            // Ok(XmlEvent::Comment(s)) => println!("{}{}", indent(depth), s),
            Ok(XmlEvent::Characters(s)) => {
                if parsing_struct {
                    println!("{}{}", indent(depth), s);
                }
            },
            // Ok(XmlEvent::Whitespace(s)) => println!("{}{}", indent(depth), s),
            Err(e) => {
                println!("Error: {}", e);
                break;
            },
            _ => {}
        }
    }

    println!("Structs: \n\n{:?}", structs);
}



// /// Reads a file into a byte Vec.
// pub fn read_file<P: AsRef<Path>>(file: P) -> VooResult<Vec<u8>> {
//     let file_name = file.as_ref().display().to_string();
//     let f = File::open(file).expect("shader file not found");
//     let file_bytes = f.metadata().unwrap().len() as usize;
//     let mut contents = Vec::<u8>::with_capacity(file_bytes);
//     let mut reader = BufReader::new(f);
//     match reader.read_to_end(&mut contents) {
//         Ok(bytes) => {
//             assert_eq!(bytes, file_bytes);
//             if PRINT { println!("Read {} bytes from {}", bytes, &file_name); }
//         },
//         Err(e) => panic!("{}", e),
//     }
//     Ok(contents)
// }




