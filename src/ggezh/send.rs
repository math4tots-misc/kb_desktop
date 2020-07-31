use super::*;

impl GgezHandler {
    pub(super) fn send0(&mut self, code: u32, args: Vec<Val>) -> Result<Val, Val> {
        match code {
            // draw text
            1001 => {
                checkargc(&args, 6)?;
                let text = args[0].expect_string()?.clone();
                let x = args[1].expect_number()? as f32;
                let y = args[2].expect_number()? as f32;
                let xscale = args[3].expect_number()? as f32;
                let yscale = args[4].expect_number()? as f32;
                let fontscale = args[5].expect_number()? as f32;
                converr(graphics::draw(
                    &mut self.ctx,
                    graphics::Text::new(text.as_ref()).set_font(
                        graphics::Font::default(),
                        graphics::Scale::uniform(fontscale),
                    ),
                    graphics::DrawParam::default()
                        .dest([x, y])
                        .scale([xscale, yscale]),
                ))?;
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
