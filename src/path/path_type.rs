use super::*;

#[derive(Debug, Clone)]
pub enum CiboulettePathBuilder<'a> {
    Type(Cow<'a, str>),
    TypeId(Cow<'a, str>, Cow<'a, str>),
    TypeIdRelationship(Cow<'a, str>, Cow<'a, str>, Cow<'a, str>),
}

#[derive(Debug, Clone)]
pub enum CiboulettePath<'a> {
    Type(&'a CibouletteResourceType),
    TypeId(&'a CibouletteResourceType, Cow<'a, str>),
    TypeIdRelationship(
        &'a CibouletteResourceType,
        Cow<'a, str>,
        &'a CibouletteResourceType,
    ),
}

impl<'a> CiboulettePathBuilder<'a> {
    pub fn build(self, bag: &'a CibouletteStore) -> Result<CiboulettePath<'a>, CibouletteError> {
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
                Ok(CiboulettePath::TypeId(ftype, id))
            }
            CiboulettePathBuilder::TypeIdRelationship(ftype_, id, stype) => {
                let (nftype_i, nftype) = bag
                    .get_type_with_index(ftype_.as_ref())
                    .ok_or_else(|| CibouletteError::UnknownType(ftype_.to_string()))?;
                let nstype_edge = nftype.relationships().get(stype.as_ref()).ok_or_else(|| {
                    CibouletteError::UnknownRelationship(ftype_.to_string(), stype.to_string())
                })?;
                let (nstype_1, nstype_2) = bag
                    .graph()
                    .edge_endpoints(*nstype_edge)
                    .ok_or_else(|| CibouletteError::RelNotInGraph(stype.to_string()))?;
                let nstype = match nftype_i == nstype_1 {
                    true => bag
                        .graph()
                        .node_weight(nstype_2)
                        .ok_or_else(|| CibouletteError::TypeNotInGraph(stype.to_string()))?,
                    false => bag
                        .graph()
                        .node_weight(nstype_1)
                        .ok_or_else(|| CibouletteError::TypeNotInGraph(stype.to_string()))?,
                };
                Ok(CiboulettePath::TypeIdRelationship(nftype, id, nstype))
            }
        }
    }
}
