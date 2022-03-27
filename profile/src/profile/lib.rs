#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Profile {
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
}
