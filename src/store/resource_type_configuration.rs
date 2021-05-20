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
    fn find_field_name(
        name: &str,
        resource_name: &str,
        ids: &CibouletteIdTypeSelector,
        schema: &MessyJsonObject,
    ) -> Result<ArcStr, CibouletteError> {
        schema
            .properties()
            .get_key_value(name)
            .map(|x| x.0.clone())
            .or_else(|| {
                ids.iter()
                    .find(|x| x.name().as_str() == name)
                    .map(|x| x.name().clone())
            })
            .ok_or_else(|| {
                CibouletteError::UnknownField(resource_name.to_string(), name.to_string())
            })
    }

    pub fn build(
        self,
        type_name: &str,
        ids: &CibouletteIdTypeSelector,
        schema: &MessyJsonObject,
    ) -> Result<CibouletteResourceTypePaginationConfiguration, CibouletteError> {
        let mut sort_by: Vec<ArcStr> = Vec::with_capacity(self.sort_by.len());
        let mut discriminants: Vec<ArcStr> = Vec::with_capacity(self.discriminants.len());
        let mut sort_set = BTreeSet::new();
        let mut discriminant_set = BTreeSet::new();

        for sort_el in self.sort_by() {
            if !sort_set.insert(sort_el.as_str()) {
                continue;
            }
            sort_by.push(Self::find_field_name(
                sort_el.as_str(),
                type_name,
                ids,
                schema,
            )?);
        }
        for discriminant in self.discriminants() {
            if !sort_set.contains(discriminant.as_str()) {
                return Err(CibouletteError::UnknownPaginationField(
                    type_name.to_string(),
                    discriminant.clone(),
                ));
            }
            let field = Self::find_field_name(discriminant.as_str(), type_name, ids, schema)?;
            if !discriminant_set.insert(field.clone()) {
                continue;
            }
            discriminants.push(field.clone());
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
