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
extern crate proto;

use proto::factory::*;

//now you can do this in rust
let desc = get_descriptor(&"mypkg.MyType".to_string()).unwrap();
let message = desc.new_instance();
```

API Docs: [https://docs.rs/protobuf_message_factory](https://docs.rs/protobuf_message_factory)

### Usage

## Step 1
create a project to generate `proto`
   
   ```sh
   $ cargo new proto
   ```

Add this to Cargo.toml:


```
[dependencies]
protobuf = "2.8.0"

[build-dependencies]
protoc-rust = "2.8.0"
protobuf_message_factory = "0.1.3"
```

## Step 2
add `.proto` file into `src/`
## Step 3
add codes to build.rs

```rust
extern crate protobuf_message_factory;

use protobuf_message_factory::*;

...

fn main() {

    let proto_path = "src/";

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
## Step 4
add `proto` deps into your project's toml

```
[dependencies]
proto = {version="^0", path="proto_path"}
```
step 1 create a proj named `proto`,  replace `proto_path` into yours

### License

MIT
