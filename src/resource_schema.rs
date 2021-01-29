use super::*;

pub enum CibouletteResourceSchema {
    Array(Arc<CibouletteResourceSchema>),
    Bool,
    Int,
    Obj(HashMap<String, Arc<CibouletteResourceSchema>>),
    String,
    Null,
}

pub enum CibouletteResourceSchemaValue<'a> {
    Array(Vec<CibouletteResourceSchemaValue<'a>>),
    Bool(bool),
    Int(i64),
    Obj(HashMap<Cow<'a, str>, Arc<CibouletteResourceSchemaValue<'a>>>),
    String(Cow<'a, str>),
    Null,
}
