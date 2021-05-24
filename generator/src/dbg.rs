use crate::generator::*;
use crate::{GameState, Overlay};
use egui::{Button, CtxRef, ScrollArea, Slider};
use host_api::{HostApi, Input};
use std::cmp;

#[no_mangle]
pub extern "C" fn dbg_update(
    state: &mut GameState,
    _host_api: &mut dyn HostApi,
    egui_ctx: &CtxRef,
) -> bool {
    egui::Window::new("debug").show(egui_ctx, |ui| {
        ui.horizontal(|ui| {
            ui.radio_value(&mut state.generator, Generator::Sidewind, "Sidewind");
            ui.radio_value(&mut state.generator, Generator::BinaryTree, "Binary tree");
            ui.radio_value(&mut state.generator, Generator::Wilson, "Wilson");
            ui.radio_value(&mut state.generator, Generator::HuntAndKill, "Hunt & kill");
        });
        ui.horizontal(|ui| {
            ui.label("Width:");
            ui.add(
                Slider::new(&mut state.debug.debug_maze_width, 1..=90)
                    .clamp_to_range(true)
                    .integer(),
            );
            ui.label("Height: ");
            ui.add(
                Slider::new(&mut state.debug.debug_maze_height, 1..=90)
                    .clamp_to_range(true)
                    .integer(),
            );
        });
        ui.horizontal(|ui| {
            ui.label("Steps:");
            let slider = Slider::new(
                &mut state.debug.debug_step,
                0..=state.wilson.steps_count() - 1,
            )
            .clamp_to_range(true);
            ui.add(slider);
            let first = Button::new("⏮").enabled(state.debug.debug_step != 0);
            if ui.add(first).clicked() {
                state.debug.debug_step = 0;
            }
            let prev = Button::new("⏪").enabled(state.debug.debug_step != 0);
            if ui.add(prev).clicked() {
                state.debug.debug_step -= 1;
            }

            let no_more =
                state.wilson.finished() && state.debug.debug_step == state.wilson.steps_count() - 1;
            if no_more {
                state.debug.debug_autoplay = false;
                ui.add(Button::new("⏵").enabled(state.debug.debug_autoplay));
            } else if state.debug.debug_autoplay {
                let pause = Button::new("⏸").enabled(state.debug.debug_autoplay);
                if ui.add(pause).clicked() {
                    state.debug.debug_autoplay = false;
                }
            } else {
                let play = Button::new("⏵").enabled(!state.debug.debug_autoplay);
                if ui.add(play).clicked() {
                    state.debug.debug_autoplay = true;
                }
            }
            let next = Button::new("⏩").enabled(!no_more);
            if ui.add(next).clicked() {
                state.debug.debug_step += 1;
            }
            let last =
                Button::new("⏭").enabled(state.debug.debug_step != state.wilson.steps_count() - 1);
            if ui.add(last).clicked() {
                state.debug.finish_requested = true;
                // state.debug.debug_step = state.wilson.steps_count() - 1;
            }
            if !state.wilson.completed() {
                if ui.add(Button::new("finish")).clicked() {
                    state.debug.finish_requested = true;
                    // state.debug.debug_step = state.wilson.steps_count() - 1;
                }
            }
        });
        ui.horizontal(|ui| {
            ui.color_edit_button_srgb(&mut state.debug.debug_borders_color);
            if state.wilson.completed() {
                ui.radio_value(&mut state.overlay, None, "none");
                ui.radio_value(&mut state.overlay, Some(Overlay::Distances), "distances");
                ui.radio_value(
                    &mut state.overlay,
                    Some(Overlay::LongestPath),
                    "longest path",
                );
            }
        });
        ui.label(format!(
            "step(wilson {}, debug {})",
            state.wilson.next_step(),
            state.debug.debug_step
        ));
        // ui.separator();
        // ui.label(format!("visited: {:?}", state.wilson.visited));
        // ui.label(format!("current_walk: {:?}", state.wilson.current_walk));
        // ui.label(format!("links: {}", state.wilson.links.len()));
        // ui.label(format!("unvisited: {}", state.wilson.unvisited.len()));
        // ui.separator();
        // let mut area = ScrollArea::from_max_height(150.0);
        // area.show(ui, |ui| {
        //     let half = 3;
        //     let current = state.wilson.next_step();
        //     let first = current.saturating_sub(half);
        //     for x in first..current {
        //         ui.label(format!("{}: {:?}", x, state.wilson.steps[x]));
        //     }
        //     ui.colored_label(
        //         color::Color32::YELLOW,
        //         format!("{}: {:?}", current, state.wilson.steps[current]),
        //     );
        //     let last = (current + half) % state.wilson.steps_count();
        //     for x in (current + 1)..last {
        //         ui.label(format!("{}: {:?}", x, state.wilson.steps[x]));
        //     }
        // });
        // ui.separator();
        ui.horizontal(|ui| {
            if ui.button("restart").clicked() {
                state.debug.reload_requested = true;
            }
            if ui.button("restart & finish").clicked() {
                state.debug.reload_requested = true;
                state.debug.finish_requested = true;
            }
        });
    });
    true
}

pub fn debug_reload_maze(state: &mut GameState, _input: &Input) {
    if state.debug.reload_requested {
        state.distances.clear();
        state.longest_path.clear();
        state.overlay = None;
        state.maze_width = state.debug.debug_maze_width;
        state.maze_height = state.debug.debug_maze_height;
        state.debug.reload_requested = false;
        state.wilson = match state.generator {
            Generator::BinaryTree => {
                Box::new(BinaryTreeGen::new(state.maze_width, state.maze_height))
            }
            Generator::Sidewind => {
                Box::new(SidewinderGen::new(state.maze_width, state.maze_height))
            }
            Generator::Wilson => Box::new(WilsonGen::new(
                &mut state.rng,
                state.maze_width,
                state.maze_height,
            )),
            Generator::HuntAndKill => {
                Box::new(HuntAndKillGen::new(state.maze_width, state.maze_height))
            }
        };
        state.debug.debug_step = cmp::min(state.debug.debug_step, state.wilson.steps_count() - 1);
    }
    if state.debug.finish_requested {
        state.wilson.finish(&mut state.rng);
        state.debug.debug_step = state.wilson.steps_count() - 1;
        state.debug.finish_requested = false;
    }
    if state.debug.debug_autoplay {
        state.debug.debug_step = (state.debug.debug_step % state.wilson.steps_count()) + 1;
    }
    if state.debug.debug_step != state.wilson.next_step() {
        state
            .wilson
            .goto_step(&mut state.rng, state.debug.debug_step);
        state.debug.debug_step = state.wilson.next_step();
    }
}
