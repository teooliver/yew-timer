Commands:

trunk serve (build dev version and serve)

trunk build --release (build release version)

### onclick example

```rs

let onclick = Callback::from(|mouse_event: MouseEvent| {
      wasm_bindgen_futures::spawn_local(async move {
          let response = reqwest::get("http://localhost:5000/tasks/group?page=1")
              .await
              .unwrap();

          let text = response.text().await.unwrap();

          let v: GetAllTasks = serde_json::from_str(&text).unwrap();
          let name = v.results[0].tasks[0].name.as_str();

          // untyped response option
          // let v: Value = serde_json::from_str(&text).unwrap();
          // let name = v["results"][0]["tasks"][0]["name"].as_str().unwrap();

          // console::log!(name);
      });
    });
```
