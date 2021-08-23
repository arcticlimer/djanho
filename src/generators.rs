use crate::{colors, decoder, vim};

pub fn generate_vimscript_config(theme: decoder::VSCodeTheme) -> String {
    let mut result = String::new();

    for token in theme.token_colors {
        match token {
            decoder::VSCodeHighlight {
                scope,
                settings:
                    decoder::VSCodeScopeSettings {
                        background: bg,
                        foreground: fg,
                        font_style: fs,
                    },
            } => {
                let background = bg.unwrap_or_default();
                let foreground = fg.unwrap_or_default();
                let text_style = fs.unwrap_or_default();

                match scope {
                    Some(decoder::VSCodeScope::Multiple(scopes)) => {
                        for group in scopes {
                            if let Some(group) = vim::map_groups(&group) {
                                let options = vim::Highlight {
                                    group: group.to_owned(),
                                    background: background.clone(),
                                    foreground: foreground.clone(),
                                    text_style: text_style.clone(),
                                };
                                result.push_str(&vim::highlight(&options))
                            }
                        }
                    }
                    Some(decoder::VSCodeScope::Single(scope)) => {
                        if let Some(group) = vim::map_groups(&scope) {
                            let options = vim::Highlight {
                                group: group.to_owned(),
                                background: background.clone(),
                                foreground: foreground.clone(),
                                text_style: text_style.clone(),
                            };
                            result.push_str(&vim::highlight(&options))
                        }
                    }
                    _ => (),
                }
            } // if let Some(group) = match scope {
              //     Some(decoder::VSCodeScope::Multiple(scopes)) => vim::map_groups(&scopes[0]),
              //     Some(decoder::VSCodeScope::Single(scope)) => vim::map_groups(&scope),
              //     _ => None,
              // } {
              //     let options = vim::Highlight {
              //         group: group.to_owned(),
              //         background,
              //         foreground,
              //         text_style,
              //     };

              //     result.push_str(&vim::highlight(options))
              // }
              // }
        }
    }

    let combined_opts = vim::combined_options();

    let mut bg = colors::from_hex_string("#000000ff").unwrap();

    if let Some(theme_colors) = theme.colors {
        for combined in combined_opts {
            let mut foreground: String = theme_colors
                .get(&combined.combinator_foreground)
                .cloned()
                .unwrap_or_default();

            let mut background: String = theme_colors
                .get(&combined.combinator_background)
                .cloned()
                .unwrap_or_default();

            if combined.combinator_background == "editor.background" {
                if let Ok(colors::RGBA { r, g, b, a }) =
                    colors::from_hex_string(&foreground.to_string())
                {
                    bg = colors::RGBA { r, g, b, a }
                }
            }

            // If the color is RGBA, we blend it with the background
            if colors::is_rgba(&foreground.to_string()) {
                if let Ok(colors::RGBA { r, g, b, a }) =
                    colors::from_hex_string(&foreground.to_string())
                {
                    let mut color = colors::blend(bg, colors::RGBA { r, g, b, a });
                    color = colors::scale(color, combined.color_scaler);
                    foreground = colors::to_rgb_hex_string(color);
                }
            }

            if colors::is_rgba(&background.to_string()) {
                if let Ok(colors::RGBA { r, g, b, a }) =
                    colors::from_hex_string(&background.to_string())
                {
                    let mbg = colors::RGBA { r, g, b, a };
                    let mut color = colors::blend(bg, mbg);
                    color = colors::scale(color, combined.color_scaler);

                    color.r = (color.r as f32 * 0.5) as u8;
                    color.g = (color.g as f32 * 0.5) as u8;
                    color.b = (color.b as f32 * 0.5) as u8;

                    background = colors::to_rgb_hex_string(color)
                }
            }

            let options = vim::Highlight {
                group: combined.vim_group,
                foreground,
                background,
                text_style: String::new(), // TODO: Maybe use it here
            };

            let line = vim::highlight(&options);
            result.push_str(&line)
        }
    }

    result
}

pub fn generate_lua_config(theme: decoder::VSCodeTheme) {}
