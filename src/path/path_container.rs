use super::*;

#[derive(Debug, Clone)]
pub enum CiboulettePathBuilder<'a> {
    Type(Cow<'a, str>),
    TypeId(Cow<'a, str>, Cow<'a, str>),
    TypeIdRelated(Cow<'a, str>, Cow<'a, str>, Cow<'a, str>),
    TypeIdRelationship(Cow<'a, str>, Cow<'a, str>, Cow<'a, str>),
}

impl<'a> CiboulettePath<'a> {
    pub fn main_type(&self) -> &'a CibouletteResourceType<'a> {
        match self {
            CiboulettePath::Type(x) => x,
            CiboulettePath::TypeId(x, _) => x,
            CiboulettePath::TypeIdRelated(x, _, _) => x,
            CiboulettePath::TypeIdRelationship(x, _, _) => x,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CiboulettePath<'a> {
    Type(&'a CibouletteResourceType<'a>),
    TypeId(&'a CibouletteResourceType<'a>, CibouletteId<'a>),
    TypeIdRelated(
        &'a CibouletteResourceType<'a>,
        CibouletteId<'a>,
        &'a CibouletteResourceType<'a>,
    ),
    TypeIdRelationship(
        &'a CibouletteResourceType<'a>,
        CibouletteId<'a>,
        &'a CibouletteResourceType<'a>,
    ),
}

impl<'a> CiboulettePathBuilder<'a> {
    pub fn parse(url: &'a Url) -> Result<Self, CibouletteError> {
        let mut segs: [Option<&str>; 4] = [None; 4];
        let segs_iter = url
            .path_segments()
            .unwrap_or_else(|| "".split('/'))
            .into_iter()
            .enumerate();

        for (i, seg) in segs_iter {
            if i >= 4 {
                return Err(CibouletteError::BadPath);
            }
            if seg.is_empty() {
                continue;
            }
            segs[i] = Some(seg);
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
        store: &'a CibouletteStore<'a>,
        ftype: Cow<'a, str>,
        stype: Cow<'a, str>,
    ) -> Result<
        (
            &'a CibouletteResourceType<'a>,
            &'a CibouletteResourceType<'a>,
        ),
        CibouletteError,
    > {
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
        Ok((nftype, nstype))
    }

    pub fn build(
        self,
        bag: &'a CibouletteStore<'a>,
    ) -> Result<CiboulettePath<'a>, CibouletteError> {
        match self {
            CiboulettePathBuilder::Type(type_) => {
                let ftype = bag
                    .get_type(type_.as_ref())
                    .ok_or_else(|| CibouletteError::UnknownType(type_.to_string()))?;
                Ok(CiboulettePath::Type(ftype))
            }
            CiboulettePathBuilder::TypeId(type_, id) => {
                let ftype = bag
                    .get_type(type_.as_ref())
                    .ok_or_else(|| CibouletteError::UnknownType(type_.to_string()))?;
                Ok(CiboulettePath::TypeId(
                    ftype,
                    CibouletteId::parse(*ftype.id_type(), id)?,
                ))
            }
            CiboulettePathBuilder::TypeIdRelated(ftype, id, stype) => {
                let (nftype, nstype) = Self::build_double_typed(&bag, ftype, stype)?;
                Ok(CiboulettePath::TypeIdRelated(
                    nftype,
                    CibouletteId::parse(*nftype.id_type(), id)?,
                    nstype,
                ))
            }
            CiboulettePathBuilder::TypeIdRelationship(ftype, id, stype) => {
                let (nftype, nstype) = Self::build_double_typed(&bag, ftype, stype)?;
                Ok(CiboulettePath::TypeIdRelationship(
                    nftype,
                    CibouletteId::parse(*nftype.id_type(), id)?,
                    nstype,
                ))
            }
        }
    }
}
