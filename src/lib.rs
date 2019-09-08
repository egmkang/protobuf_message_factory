//! A crate to generate a message factory 

use std::fs::File;
use std::io::{Write, Read, BufReader, BufRead};
use std::path::PathBuf;
use glob::glob;
use regex::Regex;


/// protobuf message and file info
#[derive(Debug)]
pub struct ProtoMessageInfo {
    file: PathBuf,
    file_name: String,
    messages : Vec<String>,
}


/// get protobuf message and file info
pub fn get_protos_info(p:&str) -> Vec<ProtoMessageInfo> {
    let mut v = Vec::new();
    let mut path = p.to_string();

    let re = Regex::new(r"message\s+([^\s]+)\s*\{*$").unwrap();

    path.push_str("/*.proto");

    for entry in glob(path.as_str()).expect("Failed to read glob pattern") {
        if let Ok(path) = entry {

            let f = File::open(path.clone()).expect("Failed to open file");
            let reader = BufReader::new(f);
            let mut item = ProtoMessageInfo {
                file: path.clone(),
                file_name: path.file_stem().unwrap().to_str().unwrap().to_string(),
                messages: vec![],
            };

            for line in reader.lines() {
                for caps in re.captures_iter(line.unwrap().as_str()) {
                    item.messages.push(caps.get(1).unwrap().as_str().to_string());
                }
            }
            v.push(item)
        }
    }

    v
}

fn generate_crate_path(path:&str) -> String {
    let mut p = path.to_string();
    if p.ends_with("/") || p.ends_with("\\") {
        p = p.get(0..p.len()-1).unwrap().to_string();
    }
    p = p.replace("src/", "crate::").replace("src\\", "crate::");
    p = p.replace("\\", "::").replace("/", "::");
    p
}

/// generate factory into `path`
pub fn generate_factory_file(path: &str, v: &Vec<ProtoMessageInfo>) {
    let mut contents = "use std::cell::RefCell;
use std::collections::HashMap;
use protobuf::reflect::MessageDescriptor;
use protobuf::Message;


thread_local! {
    pub static GLOBAL_MAP : RefCell<HashMap<String, &'static MessageDescriptor>> = RefCell::new(HashMap::new());
}

pub fn register_message<M: Message>() {
    GLOBAL_MAP.with(|x| {
        let mut m = x.borrow_mut();
        let name = M::descriptor_static().full_name().to_string();
        if !m.contains_key(&name) {
            m.insert(name, M::descriptor_static());
        }
    })
}

pub fn get_descriptor(full_name: String) -> Option<&'static MessageDescriptor> {
    GLOBAL_MAP.with(move |x| {
        {
            let m = x.borrow_mut();
            if m.len() == 0 {
                drop(m);
                init_descriptors()
            }
        }
        {
            let m = x.borrow_mut();
            match m.get(&full_name) {
                Some(r) => Some(*r),
                None => None,
            }
        }
    })
}".to_string().into_bytes();

    let mut mod_file = File::create((path.to_string() + "/mod.rs").as_str()).unwrap();
    let mut factory_file = File::create((path.to_string() + "/factory.rs").as_str()).unwrap();

    mod_file.write(b"pub mod factory;\n");

    factory_file.write_all(&contents[..]);
    factory_file.write(b"\n\n");

    //crate path
    let crate_path = generate_crate_path(path);
    for item in v.iter() {
        factory_file.write_fmt(format_args!("use {}::{};\n", crate_path, item.file_name));
        mod_file.write_fmt(format_args!("pub mod {};\n", item.file_name));
    }

    factory_file.write(b"\nfn init_descriptors() {");

    for file in v.iter() {
        for msg in file.messages.iter() {
            factory_file.write_fmt(format_args!("\n    register_message::<{}::{}>();", file.file_name, msg));
        }
    }

    factory_file.write(b"\n}\n");
}

/// get proto's filename list
fn get_proto_list(v: &Vec<ProtoMessageInfo>) -> Vec<&str> {
    let mut r = Vec::new();

    for f in v.iter() {
        r.push(f.file.to_str().unwrap());
    }

    r
}

//fn main() {
//    let proto_path = "src/proto/";
//
//    let v = get_protos_info(proto_path);
//    let protos = get_proto_list(&v);
//
//    protoc_rust::run(protoc_rust::Args {
//        out_dir: proto_path,
//        input: &protos,
//        includes: &[proto_path],
//        customize: Customize {
//          ..Default::default()
//        },
//    }).expect("protoc");
//
//    generate_factory_file(proto_path, &v);
//}
