use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kube_derive::CustomResource;
use kube::core::{Resource, CustomResourceExt};


#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
enum TeamScope {
    Private,
    Public,
    Internal,
}

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(group = "plexo.dev", version = "v1", kind = "Team", namespaced)]
struct TeamSpec {
    name: String,
    scope: TeamScope,

    members: Option<Vec<MemberSpec>>,
}

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(group = "plexo.dev", version = "v1", kind = "Member", namespaced)]
struct MemberSpec {
    name: String,
    email: Option<String>,

    teams: Option<Vec<TeamSpec>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
enum IssueStatus {
    Backlog,
    ToDo,
    InProgress,
    InReview,
    Done,
    Canceled,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
enum IssuePriority {
    Low,
    Medium,
    High,
    Urgent,
}


#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(group = "plexo.dev", version = "v1", kind = "Issue", namespaced)]
struct IssueSpec {
    code: String,

    project: ProjectSpec,
    
    title: String,
    description: Option<String>,
    labels: Option<Vec<String>>,
    

    status: Option<IssueStatus>,
    priority: Option<IssuePriority>,

    assigned: Option<MemberSpec>,
    
    parent_issue: Option<Box<IssueSpec>>,
}


#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(group = "plexo.dev", version = "v1", kind = "Project", namespaced)]
struct ProjectSpec {
    name: String,
    issues: Option<Vec<IssueSpec>>,
}



fn main() {
    // println!("Hello, world!");
    // let docs: Api<Document> = Api::default_namespaced(client);
    // let d = Document::new("guide", DocumentSpec::default());
    // println!("doc: {:?}", d);
    // println!("crd: {:?}", serde_yaml::to_string(&Document::crd()));

    // println!("kind = {}", Project::kind(&())); // impl kube::Resource
    // let f = Project::new("project-1", ProjectSpec {
    //     name: "informative info".into(),
    //     issues: vec![
    //         IssueSpec {
    //             title: "issue 1".into(),
    //         },
    //         IssueSpec {
    //             title: "issue 2".into(),
    //         },
    //     ],
    // });

    // println!("foo: {:?}", f); // debug print on generated type
    println!("crd: {}", serde_yaml::to_string(&Project::crd()).unwrap()); // crd yaml
}
