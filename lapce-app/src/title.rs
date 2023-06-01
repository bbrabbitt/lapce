use std::sync::Arc;

use floem::{
    menu::{Menu, MenuItem},
    peniko::kurbo::Point,
    reactive::{
        create_memo, ReadSignal, RwSignal, SignalGet, SignalGetUntracked, SignalSet,
        SignalWith, WriteSignal,
    },
    style::{AlignItems, Dimension, Display, JustifyContent, Style},
    view::View,
    views::{container, stack, Decorators},
    views::{label, svg},
    AppContext,
};
use lapce_core::meta;

use crate::{
    app::clickable_icon,
    command::LapceWorkbenchCommand,
    config::{color::LapceColor, icon::LapceIcons, LapceConfig},
    source_control::SourceControlData,
    update::ReleaseInfo,
    workspace::LapceWorkspace,
};

fn left(
    source_control: RwSignal<SourceControlData>,
    config: ReadSignal<Arc<LapceConfig>>,
) -> impl View {
    let branch = move || {
        source_control.with(|source_control| {
            format!(
                "{}{}",
                source_control.branch,
                if source_control.file_diffs.is_empty() {
                    ""
                } else {
                    "*"
                }
            )
        })
    };
    stack(move || {
        (
            container(move || {
                svg(move || config.get().ui_svg(LapceIcons::REMOTE)).style(
                    move || {
                        Style::BASE.size_px(26.0, 26.0).color(
                            *config.get().get_color(LapceColor::LAPCE_REMOTE_ICON),
                        )
                    },
                )
            })
            .style(move || {
                Style::BASE
                    .height_pct(100.0)
                    .padding_horiz_px(10.0)
                    .items_center()
                    .background(
                        *config.get().get_color(LapceColor::LAPCE_REMOTE_LOCAL),
                    )
            }),
            stack(move || {
                (
                    svg(move || config.get().ui_svg(LapceIcons::SCM)).style(
                        move || {
                            let config = config.get();
                            let icon_size = config.ui.icon_size() as f32;
                            Style::BASE.size_px(icon_size, icon_size).color(
                                *config.get_color(LapceColor::LAPCE_ICON_ACTIVE),
                            )
                        },
                    ),
                    label(branch).style(|| Style::BASE.margin_left_px(10.0)),
                )
            })
            .style(move || {
                Style::BASE
                    .display(if branch().is_empty() {
                        Display::None
                    } else {
                        Display::Flex
                    })
                    .height_pct(100.0)
                    .padding_horiz_px(10.0)
                    .border_right(1.0)
                    .border_color(*config.get().get_color(LapceColor::LAPCE_BORDER))
                    .align_items(Some(AlignItems::Center))
            }),
        )
    })
    .style(move || {
        Style::BASE
            .height_pct(100.0)
            .flex_basis(Dimension::Points(0.0))
            .flex_grow(1.0)
            .items_center()
    })
}

fn middle(
    workspace: Arc<LapceWorkspace>,
    set_workbench_command: WriteSignal<Option<LapceWorkbenchCommand>>,
    config: ReadSignal<Arc<LapceConfig>>,
) -> impl View {
    let local_workspace = workspace.clone();
    stack(move || {
        (
            stack(move || {
                (
                    clickable_icon(
                        || LapceIcons::LOCATION_BACKWARD,
                        || {},
                        || false,
                        || false,
                        config,
                    )
                    .style(move || Style::BASE.margin_horiz_px(6.0)),
                    clickable_icon(
                        || LapceIcons::LOCATION_FORWARD,
                        || {},
                        || false,
                        || false,
                        config,
                    )
                    .style(move || Style::BASE.margin_right_px(6.0)),
                )
            })
            .style(|| {
                Style::BASE
                    .flex_basis(Dimension::Points(0.0))
                    .flex_grow(1.0)
                    .justify_content(Some(JustifyContent::FlexEnd))
            }),
            container(|| {
                stack(|| {
                    (
                        svg(move || config.get().ui_svg(LapceIcons::SEARCH)).style(
                            move || {
                                let config = config.get();
                                let icon_size = config.ui.icon_size() as f32;
                                Style::BASE.size_px(icon_size, icon_size).color(
                                    *config.get_color(LapceColor::LAPCE_ICON_ACTIVE),
                                )
                            },
                        ),
                        label(move || {
                            if let Some(s) = local_workspace.display() {
                                s
                            } else {
                                "Open Folder".to_string()
                            }
                        })
                        .style(|| {
                            Style::BASE.padding_left_px(10.0).padding_right_px(5.0)
                        }),
                        container(move || {
                            svg(move || {
                                config.get().ui_svg(LapceIcons::PALETTE_MENU)
                            })
                            .style(move || {
                                let config = config.get();
                                let icon_size = config.ui.icon_size() as f32;
                                Style::BASE.size_px(icon_size, icon_size).color(
                                    *config.get_color(LapceColor::LAPCE_ICON_ACTIVE),
                                )
                            })
                        })
                        .style(move || Style::BASE.padding_px(5.0)),
                    )
                })
                .style(|| Style::BASE.align_items(Some(AlignItems::Center)))
            })
            .on_click(move |_| {
                if workspace.clone().path.is_some() {
                    set_workbench_command.set(Some(LapceWorkbenchCommand::Palette));
                } else {
                    set_workbench_command
                        .set(Some(LapceWorkbenchCommand::PaletteWorkspace));
                }
                true
            })
            .style(move || {
                let config = config.get();
                Style::BASE
                    .flex_basis(Dimension::Points(0.0))
                    .flex_grow(10.0)
                    .min_width_px(200.0)
                    .max_width_px(500.0)
                    .height_px(26.0)
                    .justify_content(Some(JustifyContent::Center))
                    .align_items(Some(AlignItems::Center))
                    .border(1.0)
                    .border_color(*config.get_color(LapceColor::LAPCE_BORDER))
                    .border_radius(6.0)
                    .background(*config.get_color(LapceColor::EDITOR_BACKGROUND))
            }),
            container(move || {
                clickable_icon(
                    || LapceIcons::START,
                    move || {
                        set_workbench_command
                            .set(Some(LapceWorkbenchCommand::PaletteRunAndDebug))
                    },
                    || false,
                    || false,
                    config,
                )
                .style(move || Style::BASE.margin_horiz_px(6.0))
            })
            .style(move || {
                Style::BASE
                    .flex_basis(Dimension::Points(0.0))
                    .flex_grow(1.0)
                    .justify_content(Some(JustifyContent::FlexStart))
            }),
        )
    })
    .style(|| {
        Style::BASE
            .flex_basis(Dimension::Points(0.0))
            .flex_grow(2.0)
            .align_items(Some(AlignItems::Center))
            .justify_content(Some(JustifyContent::Center))
    })
}

fn right(
    set_workbench_command: WriteSignal<Option<LapceWorkbenchCommand>>,
    latest_release: ReadSignal<Arc<Option<ReleaseInfo>>>,
    update_in_progress: RwSignal<bool>,
    config: ReadSignal<Arc<LapceConfig>>,
) -> impl View {
    let cx = AppContext::get_current();
    let latest_version = create_memo(cx.scope, move |_| {
        let latest_release = latest_release.get();
        let latest_version =
            latest_release.as_ref().as_ref().map(|r| r.version.clone());
        if latest_version.is_some()
            && latest_version.as_deref() != Some(meta::VERSION)
        {
            latest_version
        } else {
            None
        }
    });

    let has_update = move || latest_version.with(|v| v.is_some());

    container(move || {
        stack(|| {
            (
                clickable_icon(
                    || LapceIcons::SETTINGS,
                    move || {
                        cx.id.show_context_menu(
                            Menu::new("")
                                .entry(MenuItem::new("Command Palette").action(
                                    move || {
                                        set_workbench_command.set(Some(
                                            LapceWorkbenchCommand::PaletteCommand,
                                        ))
                                    },
                                ))
                                .separator()
                                .entry(MenuItem::new("Open Settings").action(
                                    move || {
                                        set_workbench_command.set(Some(
                                            LapceWorkbenchCommand::OpenSettings,
                                        ))
                                    },
                                ))
                                .separator()
                                .entry(
                                    if let Some(v) = latest_version.get_untracked() {
                                        if update_in_progress.get_untracked() {
                                            MenuItem::new(format!(
                                                "Update in progress ({v})"
                                            ))
                                            .enabled(false)
                                        } else {
                                            MenuItem::new(format!(
                                                "Restart to update ({v})"
                                            ))
                                            .action(move || {
                                                set_workbench_command.set(Some(
                                            LapceWorkbenchCommand::RestartToUpdate,
                                        ))
                                            })
                                        }
                                    } else {
                                        MenuItem::new("No update available")
                                            .enabled(false)
                                    },
                                ),
                            Point::ZERO,
                        );
                    },
                    || false,
                    || false,
                    config,
                ),
                container(|| {
                    label(|| "1".to_string()).style(move || {
                        let config = config.get();
                        Style::BASE
                            .font_size(10.0)
                            .color(*config.get_color(LapceColor::EDITOR_BACKGROUND))
                            .border_radius(100.0)
                            .margin_left_px(5.0)
                            .margin_top_px(10.0)
                            .background(*config.get_color(LapceColor::EDITOR_CARET))
                    })
                })
                .style(move || {
                    let has_update = has_update();
                    Style::BASE
                        .absolute()
                        .size_pct(100.0, 100.0)
                        .justify_end()
                        .items_end()
                        .apply_if(!has_update, |s| s.hide())
                }),
            )
        })
        .style(move || Style::BASE.margin_horiz_px(6.0))
    })
    .style(|| {
        Style::BASE
            .flex_basis(Dimension::Points(0.0))
            .flex_grow(1.0)
            .justify_content(Some(JustifyContent::FlexEnd))
    })
}

pub fn title(
    workspace: Arc<LapceWorkspace>,
    source_control: RwSignal<SourceControlData>,
    set_workbench_command: WriteSignal<Option<LapceWorkbenchCommand>>,
    latest_release: ReadSignal<Arc<Option<ReleaseInfo>>>,
    update_in_progress: RwSignal<bool>,
    config: ReadSignal<Arc<LapceConfig>>,
) -> impl View {
    stack(move || {
        (
            left(source_control, config),
            middle(workspace, set_workbench_command, config),
            right(
                set_workbench_command,
                latest_release,
                update_in_progress,
                config,
            ),
        )
    })
    .style(move || {
        let config = config.get();
        Style::BASE
            .width_pct(100.0)
            .height_px(37.0)
            .items_center()
            .background(*config.get_color(LapceColor::PANEL_BACKGROUND))
            .border_bottom(1.0)
            .border_color(*config.get_color(LapceColor::LAPCE_BORDER))
    })
}
