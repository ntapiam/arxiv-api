#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use std::fmt::{self, Display, Formatter};

pub enum SortBy {
    Relevance,
    LastUpdatedDate,
    SubmittedDate,
}

pub enum SortOrder {
    Ascending,
    Descending,
}

pub enum SearchField {
    Title(String),
    Author(String),
    Abstract(String),
    Comment(String),
    JournalReference(String),
    SubjectCategory(String),
    ReportNumber(String),
    All(String),
}

pub enum SearchCriterion {
    Pure(SearchField),
    And(SearchField),
    Or(SearchField),
    AndNot(SearchField),
}

pub struct SearchCriteria(Vec<SearchCriterion>);

pub struct Query {
    pub search_query: Option<SearchCriteria>,
    pub id_list: Option<Vec<String>>,
    pub start: Option<usize>,
    pub max_results: Option<usize>,
    pub sort_by: Option<SortBy>,
    pub sort_order: Option<SortOrder>,
}

impl Default for Query {
    fn default() -> Self {
        Query {
            search_query: None,
            id_list: None,
            start: Some(0),
            max_results: Some(10),
            sort_by: None,
            sort_order: None,
        }
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "https://export.arxiv.org/api/query?")?;
        let mut parts: Vec<String> = vec![];
        if let Some(sc) = &self.search_query {
            parts.push(format!("search_query={}", sc));
        }
        if let Some(ids) = &self.id_list {
            parts.push(format!("id_list={}", ids.join(",")));
        }
        if let Some(idx) = self.start {
            parts.push(format!("start={}", idx));
        }
        if let Some(idx) = self.max_results {
            parts.push(format!("max_results={}", idx));
        }
        if let Some(sb) = &self.sort_by {
            parts.push(format!("sortBy={}", sb));
        }
        if let Some(so) = &self.sort_order {
            parts.push(format!("sortOrder={}", so));
        }
        write!(f, "{}", parts.join("&"))
    }
}

impl<S> From<S> for SearchCriterion
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        SearchCriterion::Pure(SearchField::All(s.into()))
    }
}

impl SearchCriteria {
    pub fn new() -> Self {
        SearchCriteria(vec![])
    }
    pub fn pure(mut self, sf: SearchField) -> Self {
        self.0.push(SearchCriterion::Pure(sf));
        self
    }

    pub fn and(mut self, sf: SearchField) -> Self {
        self.0.push(SearchCriterion::And(sf));
        self
    }
    pub fn or(mut self, sf: SearchField) -> Self {
        self.0.push(SearchCriterion::Or(sf));
        self
    }
    pub fn and_not(mut self, sf: SearchField) -> Self {
        self.0.push(SearchCriterion::AndNot(sf));
        self
    }
}

impl Display for SearchCriterion {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Pure(sf) => write!(f, "{}", sf),
            Self::And(sf) => write!(f, "+AND+{}", sf),
            Self::Or(sf) => write!(f, "+OR+{}", sf),
            Self::AndNot(sf) => write!(f, "+ANDNOT+{}", sf),
        }
    }
}

impl Display for SearchField {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Title(s) => write!(f, "ti:{}", s),
            Self::Author(s) => write!(f, "au:{}", s),
            Self::Abstract(s) => write!(f, "abs:{}", s),
            Self::Comment(s) => write!(f, "co:{}", s),
            Self::JournalReference(s) => write!(f, "jr:{}", s),
            Self::SubjectCategory(s) => write!(f, "cat:{}", s),
            Self::ReportNumber(s) => write!(f, "rn:{}", s),
            Self::All(s) => write!(f, "all:{}", s),
        }
    }
}

impl Display for SortBy {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Relevance => write!(f, "relevance"),
            Self::LastUpdatedDate => write!(f, "lastUpdatedDate"),
            Self::SubmittedDate => write!(f, "submittedDate"),
        }
    }
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Ascending => write!(f, "ascending"),
            Self::Descending => write!(f, "descending"),
        }
    }
}

impl Display for SearchCriteria {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let s: String = self
            .0
            .iter()
            .map(|sc| format!("{}", sc))
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{}", s)
    }
}
