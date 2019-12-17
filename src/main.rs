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
    let json_val = serde_json::to_value(&test_msg).unwrap();
    println!("{}", json_val.pointer("/hito/fuku/size").unwrap());
}
