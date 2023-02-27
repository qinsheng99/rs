// use serde::ser::{Serialize, SerializeStruct};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct J {
    pub id: i32,
    pub name: String,
}

// impl Serialize for J {
//     fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
//     where
//         T: Serializer,
//     {
//         let mut s = serializer.serialize_struct("J", 2)?;
//         s.serialize_field("id", &self.id)?;
//         s.serialize_field("name", &self.name)?;
//         s.end()
//     }
// }

// impl Deserialize for J<'_> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, Error> where D: Deserializer<'de> {
//         todo!()
//     }
// }

#[allow(dead_code)]
pub fn json_marshal() {
    let a = J {
        id: 1,
        name: String::from("123"),
    };

    let s = serde_json::to_string(&a).unwrap();
    println!("{}", s);
}

#[allow(dead_code)]
pub fn json_unmarshal() {
    let a: &'static str = r#"
        {
            "id":1,
            "name":"zxc"
        }
    "#;

    let j: J = serde_json::from_str(a).unwrap();

    println!("{}-{}", j.id, j.name);
}
