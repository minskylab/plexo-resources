use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kube_derive::CustomResource;
use kube::core::{Resource, CustomResourceExt};
// use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
// use kube::{
//     // api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, ResourceExt},
//     // core::crd::CustomResourceExt,
//     CustomResource,
// };

#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(group = "kube.rs", version = "v1", kind = "Document", namespaced)]
pub struct DocumentSpec {
    title: String,
    content: String,
}
#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(group = "clux.dev", version = "v1", kind = "Foo", namespaced)]
struct FooSpec {
    info: String,
}



fn main() {
    println!("Hello, world!");
    // let docs: Api<Document> = Api::default_namespaced(client);
    // let d = Document::new("guide", DocumentSpec::default());
    // println!("doc: {:?}", d);
    // println!("crd: {:?}", serde_yaml::to_string(&Document::crd()));

    println!("kind = {}", Foo::kind(&())); // impl kube::Resource
let f = Foo::new("foo-1", FooSpec {
    info: "informative info".into(),
});
println!("foo: {:?}", f); // debug print on generated type
println!("crd: {}", serde_yaml::to_string(&Foo::crd()).unwrap()); // crd yaml
}
