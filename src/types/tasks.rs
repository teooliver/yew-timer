use js_sys::Date;
use wasm_bindgen::JsValue;

pub struct Task {
    pub name: Option<String>,
    pub project: Option<String>,
    pub initial_time: Option<Date>,
    pub end_time: Option<Date>,
}

impl Task {
    pub fn to_string(&self) -> String {
        let date = Date::new_0();
        format!(
            "Task {{ name: {}, project: {}, initial_time: {}, end_time: {} }}",
            self.name.as_ref().unwrap_or(&"".to_string()),
            self.project.as_ref().unwrap_or(&"None".to_string()),
            self.initial_time
                .as_ref()
                .unwrap_or_else(|| &date)
                .to_iso_string(),
            self.end_time
                .as_ref()
                .unwrap_or_else(|| &date)
                .to_iso_string(),
        )
    }
}
// #[test]
// fn it_converts_to_string() {
//     assert_eq!(
//     "Task { name: Name, project: None, initial_time: 2022-02-14T23:01:46.258Z, end_time: 2022-02-14T23:01:46.258Z }",
//     format!(
//       "{:?}",
//       Task {
//         name: Some("Name".to_string()),
//         project: None,
//         initial_time: Some(Date::new(&JsValue::from_str("2022-02-14T23:01:46.258Z"))),
//         end_time: Some(Date::new(&JsValue::from_str("2022-02-14T23:01:46.258Z")))
//       }.to_string()
//     )
//   );
// }
