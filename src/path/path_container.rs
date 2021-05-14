use super::*;

/// ## Path builder for `JSON:API` requests
#[derive(Debug, Clone)]
pub enum CiboulettePathBuilder<'request> {
    /// When selecting a whole type `/public__peoples`
    Type(Cow<'request, str>),
    /// When selecting a single object `/public__peoples/863b8f21-ccc2-49bb-aa18-cc65faace9b7`
    TypeId(Cow<'request, str>, Cow<'request, str>),
    /// When selecting a related object `/public__peoples/acec9cfa-ada1-4653-adde-a64691b46dfb/public__articles`
    TypeIdRelated(Cow<'request, str>, Cow<'request, str>, Cow<'request, str>),
    /// When selecting a relationship `/public__peoples/acec9cfa-ada1-4653-adde-a64691b46dfb/relationships/public__articles`
    TypeIdRelationship(Cow<'request, str>, Cow<'request, str>, Cow<'request, str>),
}

impl<'request> CiboulettePath<'request> {
    /// Return the main type of the path
    ///
    /// The base type for `Type` and `TypeId` and the related type for `TypeIdRelated` and `TypeIdRelationship`
    pub fn main_type(&self) -> &Arc<CibouletteResourceType> {
        match self {
            CiboulettePath::Type(x) => x,
            CiboulettePath::TypeId(x, _) => x,
            CiboulettePath::TypeIdRelated(_, _, y) => y.related_type(),
            CiboulettePath::TypeIdRelationship(_, _, y) => y.related_type(),
        }
    }

    /// Return the first type of the path
    pub fn base_type(&self) -> &Arc<CibouletteResourceType> {
        match self {
            CiboulettePath::Type(x) => x,
            CiboulettePath::TypeId(x, _) => x,
            CiboulettePath::TypeIdRelated(x, _, _) => x,
            CiboulettePath::TypeIdRelationship(x, _, _) => x,
        }
    }
}

/// ## Path of a `JSON:API` request
#[derive(Debug, Clone)]
pub enum CiboulettePath<'request> {
    /// The base type
    Type(Arc<CibouletteResourceType>),
    /// The base type and its id
    TypeId(Arc<CibouletteResourceType>, CibouletteIdSelector<'request>),
    /// The base type, its id and the relationship details with the related type
    TypeIdRelated(
        Arc<CibouletteResourceType>,
        CibouletteIdSelector<'request>,
        CibouletteResourceRelationshipDetails,
    ),
    /// The base type, its id and the relationship details with the related type
    TypeIdRelationship(
        Arc<CibouletteResourceType>,
        CibouletteIdSelector<'request>,
        CibouletteResourceRelationshipDetails,
    ),
}

impl<'request> CiboulettePathBuilder<'request> {
    /// Parse an URL, returning a [CiboulettePathBuilder](CiboulettePathBuilder)
    pub fn parse(url: &'request Url) -> Result<Self, CibouletteError> {
        let mut segs: [Option<&str>; 4] = [None; 4];
        let mut ii = 0;
        let segs_iter = url.path_segments().unwrap_or_else(|| "".split('/'));

        for seg in segs_iter {
            if ii >= 4 {
                return Err(CibouletteError::BadPath);
            }
            if seg.is_empty() {
                continue;
            }
            segs[ii] = Some(seg);
            ii += 1;
        }
        match segs {
            [None, None, None, None] => Err(CibouletteError::MissingTypeInPath),
            [Some(ftype), None, None, None] => {
                Ok(CiboulettePathBuilder::Type(Cow::Borrowed(ftype)))
            }
            [Some(ftype), Some(id), None, None] => Ok(CiboulettePathBuilder::TypeId(
                Cow::Borrowed(ftype),
                Cow::Borrowed(id),
            )),
            [Some(ftype), Some(id), Some(stype), None] => Ok(CiboulettePathBuilder::TypeIdRelated(
                Cow::Borrowed(ftype),
                Cow::Borrowed(id),
                Cow::Borrowed(stype),
            )),
            [Some(ftype), Some(id), Some(rel_keyword), Some(stype)] => {
                if !rel_keyword.eq("relationships") {
                    return Err(CibouletteError::BadPath);
                }
                Ok(CiboulettePathBuilder::TypeIdRelationship(
                    Cow::Borrowed(ftype),
                    Cow::Borrowed(id),
                    Cow::Borrowed(stype),
                ))
            }
            _ => Err(CibouletteError::BadPath),
        }
    }

    /// Build a path with a main type and related type, returning the main type and the relationship's metadata
    fn build_double_typed(
        store: &CibouletteStore,
        ftype: Cow<'request, str>,
        stype: Cow<'request, str>,
    ) -> Result<
        (
            Arc<CibouletteResourceType>,
            CibouletteResourceRelationshipDetails,
        ),
        CibouletteError,
    > {
        let nftype = store.get_type(ftype.as_ref())?;
        let nstype_rel = nftype.get_relationship_details(store, stype.as_ref())?;
        Ok((nftype.clone(), nstype_rel))
    }

    /// Build the [CiboulettePath](CiboulettePath)
    pub fn build(
        self,
        store: &CibouletteStore,
    ) -> Result<CiboulettePath<'request>, CibouletteError> {
        match self {
            CiboulettePathBuilder::Type(type_) => {
                let ftype = store.get_type(type_.as_ref())?;
                Ok(CiboulettePath::Type(ftype.clone()))
            }
            CiboulettePathBuilder::TypeId(type_, id) => {
                let ftype = store.get_type(type_.as_ref())?;
                Ok(CiboulettePath::TypeId(
                    ftype.clone(),
                    CibouletteIdSelector::build_id(ftype.ids(), id, true)?,
                ))
            }
            CiboulettePathBuilder::TypeIdRelated(ftype, id, stype) => {
                let (nftype, nstype) = Self::build_double_typed(&store, ftype, stype)?;
                Ok(CiboulettePath::TypeIdRelated(
                    nftype.clone(),
                    CibouletteIdSelector::build_id(nftype.ids(), id, true)?,
                    nstype,
                ))
            }
            CiboulettePathBuilder::TypeIdRelationship(ftype, id, stype) => {
                let (nftype, nstype) = Self::build_double_typed(&store, ftype, stype)?;
                Ok(CiboulettePath::TypeIdRelationship(
                    nftype.clone(),
                    CibouletteIdSelector::build_id(nftype.ids(), id, true)?,
                    nstype,
                ))
            }
        }
    }
}
