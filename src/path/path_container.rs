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
            CiboulettePath::TypeIdRelated(_, _, y) => y,
            CiboulettePath::TypeIdRelationship(_, _, y) => y,
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
        Arc<CibouletteResourceType>,
    ),
    TypeIdRelationship(
        Arc<CibouletteResourceType>,
        CibouletteId<'request>,
        Arc<CibouletteResourceType>,
    ),
}

impl<'request> CiboulettePathBuilder<'request> {
    pub fn parse(url: &'request Url) -> Result<Self, CibouletteError> {
        let mut segs: [Option<&str>; 4] = [None; 4];
        let mut ii = 0;
        let segs_iter = url
            .path_segments()
            .unwrap_or_else(|| "".split('/'))
            .into_iter();

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
    ) -> Result<(Arc<CibouletteResourceType>, Arc<CibouletteResourceType>), CibouletteError> {
        let (nftype_i, nftype) = store
            .get_type_with_index(ftype.as_ref())
            .ok_or_else(|| CibouletteError::UnknownType(ftype.to_string()))?;
        let nstype_edge = nftype.relationships().get(stype.as_ref()).ok_or_else(|| {
            CibouletteError::UnknownRelationship(ftype.to_string(), stype.to_string())
        })?;
        let (nstype_1, nstype_2) = store
            .graph()
            .edge_endpoints(*nstype_edge)
            .ok_or_else(|| CibouletteError::RelNotInGraph(ftype.to_string(), stype.to_string()))?;
        let nstype = match nftype_i == nstype_1 {
            true => store
                .graph()
                .node_weight(nstype_2)
                .ok_or_else(|| CibouletteError::TypeNotInGraph(stype.to_string()))?,
            false => store
                .graph()
                .node_weight(nstype_1)
                .ok_or_else(|| CibouletteError::TypeNotInGraph(stype.to_string()))?,
        };
        Ok((nftype.clone(), nstype.clone()))
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
