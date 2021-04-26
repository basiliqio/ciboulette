use super::*;

#[derive(Debug, Clone)]
pub enum CiboulettePathBuilder<'request> {
    Type(Cow<'request, str>),
    TypeId(Cow<'request, str>, Cow<'request, str>),
    TypeIdRelated(Cow<'request, str>, Cow<'request, str>, Cow<'request, str>),
    TypeIdRelationship(Cow<'request, str>, Cow<'request, str>, Cow<'request, str>),
}

impl<'request> CiboulettePath<'request> {
    pub fn main_type(&self) -> &Arc<CibouletteResourceType> {
        match self {
            CiboulettePath::Type(x) => x,
            CiboulettePath::TypeId(x, _) => x,
            CiboulettePath::TypeIdRelated(_, _, y) => y.related_type(),
            CiboulettePath::TypeIdRelationship(x, _, _) => x,
        }
    }

    pub fn base_type(&self) -> &Arc<CibouletteResourceType> {
        match self {
            CiboulettePath::Type(x) => x,
            CiboulettePath::TypeId(x, _) => x,
            CiboulettePath::TypeIdRelated(x, _, _) => x,
            CiboulettePath::TypeIdRelationship(x, _, _) => x,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CiboulettePath<'request> {
    Type(Arc<CibouletteResourceType>),
    TypeId(Arc<CibouletteResourceType>, CibouletteId<'request>),
    TypeIdRelated(
        Arc<CibouletteResourceType>,
        CibouletteId<'request>,
        CibouletteResourceRelationshipDetails,
    ),
    TypeIdRelationship(
        Arc<CibouletteResourceType>,
        CibouletteId<'request>,
        CibouletteResourceRelationshipDetails,
    ),
}

impl<'request> CiboulettePathBuilder<'request> {
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
                    CibouletteId::parse(*ftype.id_type(), id)?,
                ))
            }
            CiboulettePathBuilder::TypeIdRelated(ftype, id, stype) => {
                let (nftype, nstype) = Self::build_double_typed(&store, ftype, stype)?;
                Ok(CiboulettePath::TypeIdRelated(
                    nftype.clone(),
                    CibouletteId::parse(*nftype.id_type(), id)?,
                    nstype,
                ))
            }
            CiboulettePathBuilder::TypeIdRelationship(ftype, id, stype) => {
                let (nftype, nstype) = Self::build_double_typed(&store, ftype, stype)?;
                Ok(CiboulettePath::TypeIdRelationship(
                    nftype.clone(),
                    CibouletteId::parse(*nftype.id_type(), id)?,
                    nstype,
                ))
            }
        }
    }
}
