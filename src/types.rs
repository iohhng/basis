pub enum AtomType {Definition, Theorem, Lemma, Proposition, Corollary, Example, Exercise}

pub struct AtomMeta {
    pub id: String, 
    pub slug: String,
    pub title: Option<String>,
    pub tags: Vec<String>,
    pub status: Status, // draft or complete 
    pub created: DateTime<Utc>, // iso 8601, via chrono::DateTime<Utc>
    pub source: Vec<String>, // "key" or "key::where" 
    pub parent: Option<String>, // just for corollaries
}

pub struct Atom {
    pub meta: AtomMeta, 
    pub atom_type: AtomType,
    pub body: String, // raw typst with frontmatter removed
}

pub struct CompositeMeta {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub tags: Vec<String>,
    pub status: Status, 
    pub created: DateTime<Utc>,
    pub canonical_for: Vec<String>, // list of atoms which "live" in this composite
}

pub struct Composite {
    pub meta: CompositeMeta,
    pub body: String,
}

pub struct VaultConfig{
    pub next_id: String, // shared for atoms and composites
    pub website: String, // site url
}

pub struct Reference {
    pub entry_type: String, 
    pub author: Option<String>,
    pub title: Option<String>,
    pub year: Option<String>,
    // maybe more bibtex fields later
}

pub struct VaultIndex {
    pub cites: HashMap<String, HashSet<String>>,
    pub includes: HashMap<String, Vec<String>>,
    pub parent: HashMap<String, String>,
    pub canonical: HashMap<String, String,
    pub sources: HashMap<String, HashSet>>,
    pub dependents: HashMap<String, HashSet<String>>, // for incremental builds
}