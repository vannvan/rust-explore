## 方法记录

### 树形

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct Node {
    child_uuid: String,
    doc_id: u32,
    id: u32,
    level: u32,
    open_window: u32,
    parent_uuid: String,
    prev_uuid: String,
    sibling_uuid: String,
    title: String,
    #[serde(rename = "type")]
    node_type: String,
    url: String,
    uuid: String,
    visible: u32,
    children: Vec<Node>,
}

fn build_tree(nodes: &[Node]) -> Node {
    let mut node_map: HashMap<String, usize> = HashMap::new();
    let mut root = Node {
        child_uuid: "".to_string(),
        doc_id: 0,
        id: 0,
        level: 0,
        open_window: 0,
        parent_uuid: "".to_string(),
        prev_uuid: "".to_string(),
        sibling_uuid: "".to_string(),
        title: "".to_string(),
        node_type: "".to_string(),
        url: "".to_string(),
        uuid: "".to_string(),
        visible: 0,
        children: Vec::new(),
    };

    for (index, node) in nodes.iter().enumerate() {
        node_map.insert(node.uuid.clone(), index);
    }

    for node in nodes {
        let parent_uuid = &node.parent_uuid;
        if parent_uuid.is_empty() {
            root.children.push(node.clone());
        } else if let Some(parent_index) = node_map.get(parent_uuid) {
            let parent = &mut root.children[*parent_index];
            parent.children.push(node.clone());
        }
    }

    root
}

fn main() {
    let data = r#"
        [
            {
                "child_uuid": "",
                "doc_id": 125301470,
                "id": 125301470,
                "level": 0,
                "open_window": 0,
                "parent_uuid": "",
                "prev_uuid": "",
                "sibling_uuid": "nzQzJVzrt-S9ebwm",
                "title": "一个非md格式文件，会失败！",
                "type": "DOC",
                "url": "kiotse0q7uvepd0l",
                "uuid": "kqRSJukCi9qnsJAy",
                "visible": 1
            },
            // ...
        ]
    "#;

    let nodes: Vec<Node> = serde_json::from_str(data).unwrap();
    let tree = build_tree(&nodes);

    println!("{:#?}", tree);
}
```

## 查看支持构建的目标列表

> rustc --print target-list
