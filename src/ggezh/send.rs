use super::*;

impl GgezHandler {
    pub(super) fn send0(&mut self, code: u32, args: Vec<Val>) -> Result<Val, Val> {
        match code {
            // draw text
            1001 => {
                checkargc(&args, 6)?;
                let mut args = args.into_iter();
                let mut text = as_text(args.next().unwrap())?;
                let x = args.next().unwrap().expect_number()? as f32;
                let y = args.next().unwrap().expect_number()? as f32;
                let xscale = args.next().unwrap().expect_number()? as f32;
                let yscale = args.next().unwrap().expect_number()? as f32;
                match args.next().unwrap() {
                    Val::Nil => {}
                    val => {
                        let fontscale = val.expect_number()? as f32;
                        text.map_mut(|text| {
                            text.set_font(
                                graphics::Font::default(),
                                graphics::Scale::uniform(fontscale),
                            );
                        });
                    }
                }
                converr(text.map_ref(|text| graphics::draw(
                    &mut self.ctx,
                    text,
                    graphics::DrawParam::default()
                        .dest([x, y])
                        .scale([xscale, yscale]),
                )))?;
                Ok(Val::Nil)
            }
            // clear screen
            1002 => {
                checkargc(&args, 1)?;
                let color = to_color(&args[0])?;
                graphics::clear(&mut self.ctx, color);
                Ok(Val::Nil)
            }
            _ => Err(rterr!("Unrecognized code {}", code)),
        }
    }
}
