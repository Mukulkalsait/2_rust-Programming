use reqwest::blocking::Response;
use serde::Deserialize;

///## 1:
/// any type T can be deserilize from JSON
/// *** T: for<'de> Deserialize<'de> ***  means
/// this type T can be created from seralized data ... (with deseralization).
/// <'de> can be anyting <'abc> and it work we use <'de> because its for Deseralization.
/// This insures that Deseralized data atleast leaves this long...
/// NOTE: Need of lifetimes here...
/// "Deseralise can create borrowed data. eg { name: &name_var}"
///## 1.2: HRTB "Hihger Rank Trait Bound":
///  ***"T: for<'de> Deserialize<'de>"***
///  here that "FOR" looks like looping which is not actually loop but work like loop
///  -> it says T must impliment Deserialize for EVERY POSSIBLE LIFETIME.
///   FOR => FOR ALL.
///   no loop because-> no iteration/runtimeLoop, its just a rule (compile time).
///   eg
///   ```rust
///   fn foo<T>()
///   where
///     T: for<'a> Trait<'a>{}
///     // hence T Must Satisfy:
///     Trait<'short>
///     Trait<'long>
///     Trait<'anyting>
///
///   ```
///
///## 2:
/// res.json::<T>() =. another turbo fish syntex ...
/// res.jaon() normal function but we dont know type before deseralization so
/// res.json::<T>() thats it.
///
///## 3: dyn => Dynamic dispatch: at runtime it can hold any type implimenting trait
///  eg  Box<dyn Error>
///  can hold
///     - reqwest::Error
///     - std::io::Error
///     - ApiError::Error (costume error)
///  Box because its unsized, hence heap alocated
///  ⭐ Dynamic dispatch => use when apps/CLI tools/ learning /prototypes
///  ⭐ dont use => libraries/public APIs use custome enums.... like API_Error for error here.
///## 4.The Ok(...?); block does the conversion for us.
///    without Ok(?); block the the returning type will be Result<T,Error>  which is not the case.
pub fn get_json<T: for<'de> Deserialize<'de>>(res: Response) -> Result<T, Box<dyn std::error::Error>> {
    Ok(res.json::<T>()?)
}
