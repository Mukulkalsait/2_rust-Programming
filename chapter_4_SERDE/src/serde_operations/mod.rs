use crate::data_structs::*;

/// we added data into owenr1, dog1 and did serilised_dog = serde_json::to_string(&struct_var)
/// is_ok? if yes ok().unwrap();
pub fn serilise_data() {
    let owner1: DogOwner = DogOwner {
        name: "mukul".to_string(),
        age: 29,
        gender: Gender::Male,
        occupation: "Senior DevOps & System Engineer".to_string(),
        generated_at: 2026,
    };
    let vaxin_list1: VaxinList = VaxinList { v1: Some(Vaxins::Booster), v2: Some(Vaxins::TT), v3: Some(Vaxins::Fungal) };
    let status1: DogStatus = DogStatus { is_vaxcinated: true, which_vaxin: Some(vaxin_list1) };

    let dog1: Dog = Dog {
        name: "Julie".to_string(),
        breed: "dobarman".to_string(),
        gender: Gender::Female,
        age: 2017,
        owener: owner1,
        secret_code: "AXIE#@2423VC3424".to_string(),
        status: status1,
        previous_owners: 0,
    };

    let dog_ser = serde_json::to_string_pretty(&dog1);

    if dog_ser.is_ok() {
        println!("{}", dog_ser.ok().unwrap());
    } else {
        println!("{:#?}", dog_ser.err());
    }
}

/// R: to test if this really fails change the raw string data of json somelthing like changing type of the name field
/// here we give the exact matching data to the struct from raw json.
///
/// we used
/// var = serde_json.from_str::<struct>(&json_data_var)
/// var.is_ok if ok().unwrap()
pub fn deserilise_data() {
    let json_string = r#"
    {
    "dogName": "Julie",
    "breed": "dobarman",
    "gender": "female",
    "dogAge": 2017,
    "owener": {
        "name": "mukul",
        "age": 29,
        "occupation": "Senior DevOps & SYstem Engineer",
        "generated_at": 2026
    },
    "status": {
        "is_vaxcinated": true,
        "which_vaxin": "Rabeis"
    }
    }
    "#;
    let deserilised_dog = serde_json::from_str::<Dog>(&json_string);
    // here we used *TURBO FISH SYNTEX* to denote the data type.
    // serde_json::from_str::<TURBO FISH SYNTEX>(data_of_type)
    // Y:
    // In deseriliisng rust basically dont ahve any idea what kind of data is its going to revieve so we defineht data type here.
    if deserilised_dog.is_ok() {
        println!("De-Ser-ali-ze\n|------\n{:#?}\n|------", deserilised_dog.ok().unwrap());
    } else {
        println!("{:#?}", deserilised_dog.err())
    }
}
