// Ty2: -------------------------------------------------------------------------------------------------------------------
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
// #[serde(rename_all = "PascalCase")]
pub struct Dog {
    #[serde(alias = "dogName")] // B:  Extra field of ALIAS
    pub name: String,
    pub breed: String,
    pub gender: Gender,

    #[serde(rename = "dogAge")] // Y: now this will became dogAge
    pub age: i32,
    pub owener: DogOwner,
    pub status: DogStatus,

    #[serde(skip)] // R: ignored in SER, will Devault in DESER
    // DESER DEFAULTS :
    // Stirng = ""
    // Vec =  []
    // bool = False
    pub secret_code: String,

    #[serde(default)] // B: Automatic value "0" when deseralised into struct.
    pub previous_owners: u8,
}
// Ty2: -------------------------------------------------------------------------------------------------------------------

fn default_gender() -> Gender { Gender::Male }
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DogOwner {
    pub name: String,
    pub age: i32,

    //  IMP: look at the fn default_gender() above, while DE-SER we get default gender even if not passed.
    #[serde(default = "default_gender")]
    pub gender: Gender,

    #[serde(skip_serializing)] // Y: will be skip only while SER. but in De-SER will work
    pub occupation: String,

    #[serde(skip_deserializing)] // B: will Skip on De-SER. but in Ser it will work.
    pub generated_at: u16,
}
// Ty2: -------------------------------------------------------------------------------------------------------------------

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DogStatus {
    pub is_vaxcinated: bool,

    #[serde(flatten)]
    // Ty: flatten the inner blocak and merge it into same block, making look consistnece
    pub which_vaxin: Option<VaxinList>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct VaxinList {
    #[serde(skip_serializing_if = "Option::is_none")] // B: option vs none
    pub v1: Option<Vaxins>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub v2: Option<Vaxins>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub v3: Option<Vaxins>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Vaxins {
    Booster,
    Rabies,
    TT,
    Fungal,
    ThreeInOne,
}

// Ty2: -------------------------------------------------------------------------------------------------------------------

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
}
// Ty2: -------------------------------------------------------------------------------------------------------------------
