use serde_json;
mod proto {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/test.rs"));
}

mod google {
    pub mod protobuf {
        #![allow(clippy::all)]
        include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
    }
}

fn main() {
    let test_msg = proto::TestMsg {
        num: 10,
        str: "hoge".to_string(),
        hito: Some(proto::Hito {
            age: 18,
            name: "osamu".to_string(),
            fuku: Some(proto::Fuku {
                size: 38,
                name: "Denimu".to_string(),
            }),
        }),
        mask: None,
    };

    let update_request = proto::TestMsg {
        num: 314,
        str: String::default(),
        hito: Some(proto::Hito {
            age: u32::default(),
            name: String::default(),
            fuku: Some(proto::Fuku {
                size: u32::default(),
                name: "damage jeans".to_string(),
            }),
        }),
        mask: Some(google::protobuf::FieldMask {
            paths: vec!["/num".to_string(), "/hito/fuku/name".to_string()],
        }),
    };

    let mut json_val = serde_json::to_value(&test_msg).unwrap();
    let json_req = serde_json::to_value(&update_request).unwrap();

    println!("{}", json_val);

    batch(&mut json_val, &json_req);

    println!("{}", json_val);
}

fn batch(val: &mut serde_json::Value, req: &serde_json::Value) {
    if let serde_json::Value::Array(paths) = req.pointer("/mask/paths").unwrap() {
        for path in paths {
            if let serde_json::Value::String(path_str) = path {
                let field = val.pointer_mut(&path_str).unwrap();
                *field = req.pointer(&path_str).unwrap().clone();
            }
        }
    }
}
