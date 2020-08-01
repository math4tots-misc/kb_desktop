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

pub(super) fn as_text(val: Val) -> Result<HCow<Text>, Val> {
    if val.is_handle::<Text>() {
        val.into_hcow()
    } else {
        Ok(HCow::Owned(to_text(val.clone())?))
    }
}

fn to_text(val: Val) -> Result<Text, Val> {
    let mut frags = Vec::new();
    to_textfragments(val, &mut frags)?;
    let mut text = Text::default();
    for frag in frags {
        text.add(frag);
    }
    Ok(text)
}

fn to_textfragments(val: Val, out: &mut Vec<TextFragment>) -> Result<(), Val> {
    match val {
        Val::List(list) => match Rc::try_unwrap(list) {
            Ok(list) => {
                for item in list.into_inner() {
                    to_textfragments(item, out)?;
                }
            }
            Err(list) => {
                for item in list.borrow().iter() {
                    to_textfragments(item.clone(), out)?;
                }
            }
        },
        _ => out.push(to_textfragment(val)?),
    }
    Ok(())
}

fn to_textfragment(val: Val) -> Result<TextFragment, Val> {
    match val {
        Val::String(string) => Ok(TextFragment::new(string.unwrap_or_clone())),
        Val::Map(map) => {
            let mut map = map.to_string_keys()?;
            let mut frag = TextFragment::default();
            if let Some(textval) = map.remove("text") {
                frag.text = textval.expect_string()?.as_ref().to_owned();
            }
            if let Some(colorval) = map.remove("color") {
                frag.color = Some(to_color(&colorval)?);
            }
            if let Some(scaleval) = map.remove("scale") {
                let fontscale = scaleval.expect_number()? as f32;
                frag.scale = Some(graphics::Scale::uniform(fontscale));
            }
            if !map.is_empty() {
                let keys: Vec<_> = map.keys().collect();
                return Err(rterr!("Unused fragment attributes: {:?}", keys));
            }
            Ok(frag)
        }
        _ => Err(rterr!("Expected text fragment")),
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
