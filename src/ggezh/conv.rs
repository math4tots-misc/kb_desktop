use super::*;

pub(super) fn to_color(val: &Val) -> Result<Color, Val> {
    let list = val.expect_list()?;
    if list.borrow().len() == 3 {
        let [r, g, b] = to_3f32(val)?;
        Ok([r, g, b, 1.0].into())
    } else {
        Ok(to_4f32(val)?.into())
    }
}

pub(super) fn to_3f32(val: &Val) -> Result<[f32; 3], Val> {
    let list = val.expect_list()?;
    let list = list.borrow();
    if list.len() != 3 {
        return Err(rterr!("Expected 3 numbers, but got {} values", list.len()));
    }
    let x1 = list[0].expect_number()? as f32;
    let x2 = list[1].expect_number()? as f32;
    let x3 = list[2].expect_number()? as f32;
    Ok([x1, x2, x3])
}

pub(super) fn to_4f32(val: &Val) -> Result<[f32; 4], Val> {
    let list = val.expect_list()?;
    let list = list.borrow();
    if list.len() != 3 {
        return Err(rterr!("Expected 3 numbers, but got {} values", list.len()));
    }
    let x1 = list[0].expect_number()? as f32;
    let x2 = list[1].expect_number()? as f32;
    let x3 = list[2].expect_number()? as f32;
    let x4 = list[3].expect_number()? as f32;
    Ok([x1, x2, x3, x4])
}
