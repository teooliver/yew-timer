use js_sys::Date;
// use wasm_bindgen::JsValue;

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllTasks {
    pub results: Vec<Result>,
    pub pagination: Option<Pagination>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    #[serde(rename = "_id")]
    pub id: String,
    pub tasks: Vec<TaskResponse>,
    #[serde(rename = "total_time")]
    pub total_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResponse {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "initial_time")]
    pub initial_time: String,
    #[serde(rename = "end_time")]
    pub end_time: String,
    pub project: String,
    #[serde(rename = "project_color")]
    pub project_color: String,
    pub client: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub previous: Value,
    pub next: String,
    #[serde(rename = "next_page")]
    pub next_page: i64,
    #[serde(rename = "previous_page")]
    pub previous_page: Value,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_items")]
    pub total_items: i64,
    pub size: i64,
    pub start: i64,
}
