use std::fmt;

use js_sys::Date;

pub struct Task {
    pub name: Option<String>,
    pub project: Option<String>,
    pub initial_time: Option<Date>,
    pub end_time: Option<Date>,
}

impl Task {
    pub fn to_string(&self) -> String {
        let date = Date::new_0();
        // let x = self.initial_time.as_ref().unwrap_or_else(|| &date);
        format!(
            "name:{}, project:{}, initial_time: {:?}, end_time:{:?}",
            self.name.as_ref().unwrap_or(&"".to_string()),
            self.project.as_ref().unwrap_or(&"None".to_string()),
            self.initial_time
                .as_ref()
                .unwrap_or_else(|| &date)
                .to_date_string(),
            self.end_time
                .as_ref()
                .unwrap_or_else(|| &date)
                .to_iso_string(),
        )
    }
}

// impl fmt::Display for Task {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.write_str(self.name.clone().unwrap_or("".to_string()).as_str())
//     }
// }

// impl fmt::Debug for Task {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_struct("Task")
//             .field("name", &self.name)
//             .field("project", &self.project)
//             .field("initial_time", &self.initial_time)
//             .field("end_time", &self.end_time)
//             .finish()
//     }
// }

// assert_eq!(
//     "Foo { bar: 10, baz: \"Hello World\", addr: 127.0.0.1 }",
//     format!(
//         "{:?}",
//         Task {
//             bar: 10,
//             baz: "Hello World".to_string(),
//             addr: Ipv4Addr::new(127, 0, 0, 1),
//         }
//     )
// );
