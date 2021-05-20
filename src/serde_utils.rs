use serde::de::Deserialize;

use crate::CibouletteOptionalData;

#[inline]
fn handle_ident_check_doesnt_exists<'de, T, A: serde::de::MapAccess<'de>>(
    res: &mut Option<T>,
    field_name: &'static str,
) -> Result<(), A::Error> {
    if Option::is_some(res) {
        return Err(<A::Error as serde::de::Error>::duplicate_field(field_name));
    }
    Ok(())
}

#[inline]
pub fn handle_ident_in_map_stateless<'de, T: Deserialize<'de>, A: serde::de::MapAccess<'de>>(
    res: &mut Option<T>,
    mut map: &mut A,
    field_name: &'static str,
) -> Result<(), A::Error> {
    handle_ident_check_doesnt_exists::<T, A>(res, field_name)?;
    *res = Some(serde::de::MapAccess::next_value::<T>(&mut map)?);
    Ok(())
}

#[inline]
fn handle_ident_check_doesnt_exists_ciboulette_optional<'de, T, A: serde::de::MapAccess<'de>>(
    res: &mut CibouletteOptionalData<T>,
    field_name: &'static str,
) -> Result<(), A::Error> {
    if matches!(res, CibouletteOptionalData::Object(_)) {
        return Err(<A::Error as serde::de::Error>::duplicate_field(field_name));
    }
    Ok(())
}

#[inline]
pub fn handle_ident_in_map_stateless_ciboulette_optional<
    'de,
    T: Deserialize<'de>,
    A: serde::de::MapAccess<'de>,
>(
    res: &mut CibouletteOptionalData<T>,
    mut map: &mut A,
    field_name: &'static str,
) -> Result<(), A::Error> {
    handle_ident_check_doesnt_exists_ciboulette_optional::<T, A>(res, field_name)?;
    *res = CibouletteOptionalData::Object(serde::de::MapAccess::next_value::<T>(&mut map)?);
    Ok(())
}

#[inline]
pub fn handle_ident_in_map_stateful<'de, A: serde::de::MapAccess<'de>, V: Deserialize<'de>>(
    res: &mut Option<V>,
    mut map: &mut A,
    field_name: &'static str,
) -> Result<(), A::Error> {
    handle_ident_check_doesnt_exists::<V, A>(res, field_name)?;
    *res = Some(serde::de::MapAccess::next_value(&mut map)?);
    Ok(())
}
