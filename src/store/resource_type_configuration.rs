use super::*;

/// Configuration needed to add a new type to the ciboulette store
#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub(crate)")]
pub struct CibouletteResourceTypeConfiguration {
    pub ids: CibouletteIdTypeSelector,
    pub schema: MessyJsonObject,
    pub pagination: Option<CibouletteResourceTypePaginationConfigurationBuilder>,
}

impl CibouletteResourceTypeConfiguration {
    pub fn new(
        ids: CibouletteIdTypeSelector,
        schema: MessyJsonObject,
        pagination: Option<CibouletteResourceTypePaginationConfigurationBuilder>,
    ) -> Self {
        CibouletteResourceTypeConfiguration {
            ids,
            schema,
            pagination,
        }
    }
}

/// Pagination option builder for new ciboulette type
#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub(crate)")]
pub struct CibouletteResourceTypePaginationConfigurationBuilder {
    /// A list of fields ot sort by. i.e `["created_at", "id"]`
    pub sort_by: Vec<String>,
    /// The discriminant keys. i.e. `["id"]`
    pub discriminants: Vec<String>,
    /// encoding when generating the links.
    pub encoding: CiboulettePaginationEncoding,
}

impl CibouletteResourceTypePaginationConfigurationBuilder {
    pub fn build(
        self,
        type_name: &str,
        schema: &MessyJsonObject,
    ) -> Result<CibouletteResourceTypePaginationConfiguration, CibouletteError> {
        let mut sort_by: Vec<ArcStr> = Vec::with_capacity(self.sort_by.len());
        let mut discriminants: Vec<ArcStr> = Vec::with_capacity(self.discriminants.len());
        let mut seen_set = BTreeSet::new();

        for sort_el in self.sort_by() {
            if !seen_set.insert(sort_el.as_str()) {
                return Err(CibouletteError::UnknownField(
                    type_name.to_string(),
                    sort_el.clone(),
                ));
            }
            sort_by.push(
                schema
                    .properties()
                    .get_key_value(sort_el.as_str())
                    .ok_or_else(|| {
                        CibouletteError::UnknownField(type_name.to_string(), sort_el.clone())
                    })?
                    .0
                    .clone(),
            );
        }
        for discriminant in self.discriminants() {
            if !seen_set.insert(discriminant.as_str()) {
                return Err(CibouletteError::UnknownField(
                    type_name.to_string(),
                    discriminant.clone(),
                ));
            }
            discriminants.push(
                schema
                    .properties()
                    .get_key_value(discriminant.as_str())
                    .ok_or_else(|| {
                        CibouletteError::UnknownField(type_name.to_string(), discriminant.clone())
                    })?
                    .0
                    .clone(),
            );
        }
        Ok(CibouletteResourceTypePaginationConfiguration {
            sort_by,
            discriminants,
            encoding: self.encoding,
        })
    }
}

/// Pagination option for new ciboulette type
#[derive(Clone, Debug, Getters, MutGetters, Hash)]
#[getset(get = "pub", get_mut = "pub(crate)")]
pub struct CibouletteResourceTypePaginationConfiguration {
    /// A list of fields ot sort by. i.e `["created_at", "id"]`
    pub sort_by: Vec<ArcStr>,
    /// The discriminant keys. i.e. `["id"]`
    pub discriminants: Vec<ArcStr>,
    /// encoding when generating the links.
    pub encoding: CiboulettePaginationEncoding,
}

/// Encoding option for pagination links
#[derive(Clone, Debug, Copy, Hash)]
pub enum CiboulettePaginationEncoding {
    /// Comma separated
    Raw,
    /// Bincode -> base64 encoded
    Base64,
}
