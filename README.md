# protobuf_message_factory

this repo provide you a way to generate a message factory to create a message instance by message name.

```cpp
//use can do this in cpp
google::protobuf::Descriptor* desc =
    google::protobuf::DescriptorPool::generated_pool()
        ->FindMessageTypeByName("mypkg.MyType");
google::protobuf::Message* message =
    google::protobuf::MessageFactory::generated_factory()
        ->GetPrototype(desc)->New();
```

```rust
mod proto;

use proto::factory::*;

//now you can do this in rust
let desc = get_descriptor("mypkg.MyType".to_string()).unwrap();
let message = desc.new_instance();
```

API Docs: [https://docs.rs/protobuf_message_factory](https://docs.rs/protobuf_message_factory)

### Usage

Add this to your Cargo.toml:


```
[build-dependencies]
protobuf_message_factory = "0.1.0"
```

### add code to build.rs

```rust
extern crate protobuf_message_factory;

use protobuf_message_factory::*;

...

fn main() {

    let proto_path = "src/proto";

    let proto_files = get_protos_info(proto_path);
    let proto_messages = get_proto_list(&proto_files);


    //!!!   this is importent.   !!!
    protoc_rust::run(protoc_rust::Args {
        out_dir: proto_path,
        input: &protos,
        includes: &[proto_path],
        customize: Customize {
          ..Default::default()
        },
    }).expect("protoc");

    //now generate factory codes
    generate_factory_file(proto_path, &proto_files);
}
```

then add a empty file named `mod.rs` to `proto_path`, then put all proto files to `proto_path`.

now `cargo build` will generate `factory.rs`, `mod.rs` and proto's rs files.

### License

MIT
