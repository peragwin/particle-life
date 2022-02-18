mod params;
mod particles;
mod universe;

use params::Params;
use particles::{Particle, ParticleTypes, DIAMETER};
use universe::Universe;

use clap::{App, Arg};
use eframe::{
    egui,
    egui::{emath, Color32, Pos2, Rect},
    epi,
};

struct ParticleLife {
    universe: Universe,
    particles: Vec<Particle>,
    particle_types: ParticleTypes,
    params: Params,
}

impl epi::App for ParticleLife {
    fn name(&self) -> &str {
        "Particle Life"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            self.draw_ui(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.particles =
                self.universe
                    .step(&self.particle_types, &self.params, &self.particles);
            self.render(ui)
        });
    }
}

impl ParticleLife {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        let mut params = self.params;

        ui.add(egui::Label::new("Mean Attraction"));
        ui.add(
            egui::DragValue::new(&mut params.mean_attraction)
                .speed(0.005)
                .min_decimals(3),
        );
        ui.end_row();

        ui.add(egui::Label::new("Sigma Attraction"));
        ui.add(
            egui::DragValue::new(&mut params.std_attraction)
                .speed(0.005)
                .min_decimals(3),
        );
        ui.end_row();

        ui.add(egui::Label::new("Min Radius Lower"));
        ui.add(
            egui::DragValue::new(&mut params.min_radius_lower)
                .speed(0.1)
                .min_decimals(1),
        );
        ui.end_row();

        ui.add(egui::Label::new("Min Radius Upper"));
        ui.add(
            egui::DragValue::new(&mut params.min_radius_upper)
                .speed(0.1)
                .min_decimals(2),
        );
        ui.end_row();

        ui.add(egui::Label::new("Max Radius Lower"));
        ui.add(
            egui::DragValue::new(&mut params.max_radius_lower)
                .speed(0.1)
                .min_decimals(1),
        );
        ui.end_row();

        ui.add(egui::Label::new("Max Radius Upper"));
        ui.add(
            egui::DragValue::new(&mut params.max_radius_upper)
                .speed(0.1)
                .min_decimals(2),
        );
        ui.end_row();

        ui.add(egui::Label::new("Friction"));
        ui.add(
            egui::DragValue::new(&mut params.friction)
                .speed(0.005)
                .min_decimals(3),
        );
        ui.end_row();

        if ui.button("Randomize").clicked() {
            self.particle_types.randomize(&params);
        }

        self.params = params;
    }

    fn render(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.ctx().request_repaint();

        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::drag());

        let from = {
            let (w, h) = self.universe.size();
            Rect::from_two_pos(Pos2::from((0., 0.)), Pos2::from((w, h)))
        };
        let to_screen = emath::RectTransform::from_to(from, response.rect); //from: response.rect.square_proportions())

        let mut shapes = vec![];
        shapes.push(egui::Shape::rect_filled(response.rect, 0., Color32::BLACK));
        for p in self.particles.iter() {
            let pos = Pos2::new(p.pos.x, p.pos.y);
            let [r, g, b] = self.particle_types.get_color(p.typ).0;
            shapes.push(egui::Shape::circle_filled(
                to_screen.transform_pos(pos),
                DIAMETER / 2.,
                Color32::from_rgb(r, g, b),
            ));
        }

        painter.extend(shapes);

        response
    }
}

fn main() {
    let matches = App::new("particle_sys")
        .arg(
            Arg::with_name("num_particles")
                .long("num_particles")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("num_types")
                .long("num_types")
                .takes_value(true),
        )
        .arg(Arg::with_name("width").long("width").takes_value(true))
        .get_matches();

    let num_particles = matches
        .value_of("num_particles")
        .unwrap_or("1024")
        .parse::<usize>()
        .unwrap();

    let num_types = matches
        .value_of("num_types")
        .unwrap_or("8")
        .parse::<u8>()
        .unwrap();

    let width = matches
        .value_of("width")
        .unwrap_or("128")
        .parse::<f32>()
        .unwrap();

    let params = Params {
        mean_attraction: 0.0,
        std_attraction: 0.04,
        min_radius_lower: 0.,
        min_radius_upper: 10.,
        max_radius_lower: 10.,
        max_radius_upper: 40.,
        friction: 0.05,
        wrap: true,
    };

    let universe = Universe::new(width, width);
    let particle_types = ParticleTypes::new(num_types, &params);
    let particles = universe.create_particles(&particle_types, num_particles);
    let app = ParticleLife {
        universe,
        particle_types,
        particles,
        params,
    };

    // let (sys, particles) = ParticleSys::new(8, params, num_particles as usize);
    // let app = ParticleApp { sys, particles };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
