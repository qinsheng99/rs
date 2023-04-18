use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Deserialize)]
pub struct IssueOptions {
    pub token: String,
    pub repo: String,
    pub title: String,
    pub issue_type: String,
    pub body: String,
    pub assignee: String,
    pub labels: String,
    pub security_hole: bool,
    pub collaborators: String,
    pub milestone: i64,
}

pub const fn default_issue() -> IssueOptions {
    IssueOptions {
        token: String::new(),
        repo: String::new(),
        title: String::new(),
        issue_type: String::new(),
        body: String::new(),
        assignee: String::new(),
        labels: String::new(),
        security_hole: false,
        collaborators: String::new(),
        milestone: 0,
    }
}

impl Serialize for IssueOptions {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let mut s = serializer.serialize_struct("IssueOptions", 10)?;
        s.serialize_field("access_token", &self.token)?;
        s.serialize_field("repo", &self.repo)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("issue_type", &self.issue_type)?;
        s.serialize_field("body", &self.body)?;
        s.serialize_field("assignee", &self.assignee)?;
        s.serialize_field("labels", &self.labels)?;
        s.serialize_field("security_hole", &self.security_hole)?;
        s.serialize_field("collaborators", &self.collaborators)?;
        s.serialize_field("milestone", &self.milestone)?;
        s.end()
    }
}

impl IssueOptions {
    fn json_marshal(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
    fn set_token(&mut self, t: &str) -> &mut IssueOptions {
        self.token = t.to_string();
        self
    }
    fn set_title(&mut self, t: &str) -> &mut IssueOptions {
        self.title = t.to_string();
        self
    }
}

#[allow(dead_code)]
pub fn json_marshal() {
    let mut a = default_issue();
    a.set_token("qwe").set_title("issue");

    let s = a.json_marshal().unwrap();
    println!("{}", s);
}

#[allow(dead_code)]
pub fn json_unmarshal() {
    let a: &'static str = r#"
        {
            "access_token":"1",
            "assignee":"zxc"
        }
    "#;

    let j: IssueOptions = serde_json::from_str(a).unwrap();

    println!("{}-{}", j.token, j.assignee);
}
