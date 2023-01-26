use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle}, cell::RefCell, borrow::Borrow, ops::Deref,
};

use egui::Color32;

use crate::scene::Scene;
use rand::Rng;

pub struct Renderer {
    thread_handles: Vec<JoinHandle<()>>,
    renderer_data: Arc<RendererData>,
}

struct RendererData {
    pub size: [usize; 2],
    scene: Scene,
    data: Mutex<RefCell<Vec<Color32>>>,

    bail_threads: AtomicBool,
}

impl Renderer {
    pub fn new(size: [usize; 2], scene: Scene) -> Renderer {
        let mut renderer = Renderer {
            thread_handles: vec![],
            renderer_data: Arc::new(RendererData {
                size,
                scene,
                data: Mutex::new(RefCell::new(vec![Color32::GRAY; size[0] * size[1]])),
                bail_threads: AtomicBool::new(false),
            }),
        };

        let num_cpus = num_cpus::get();
        let num_pixels = size[0] * size[1];

        for i in 0..num_cpus {
            let renderer_data_arc = renderer.renderer_data.clone();
            let starting_pixel_inclusive =
                (num_pixels as f32 * (i as f32 / num_cpus as f32)).floor() as usize;
            let ending_pixel_exclusive =
                (num_pixels as f32 * ((1.0 + i as f32) / num_cpus as f32)).floor() as usize;

            let starting_coordinate_inclusive = [
                starting_pixel_inclusive % size[0],
                starting_pixel_inclusive / size[0],
            ];
            let ending_coordinate_exclusive = [
                ending_pixel_exclusive % size[0],
                ending_pixel_exclusive / size[0] + 1,
            ];

            renderer.thread_handles.push(thread::spawn(move || {
                (*renderer_data_arc)
                    .render_thread_run(starting_coordinate_inclusive, ending_coordinate_exclusive);
            }));
        }

        renderer
    }

    /// Returns an image of `self.get_image_size()` size
    pub fn get_image(&self) -> Vec<Color32> {
        let refcell_mutexguard = self.renderer_data.data.lock().unwrap();
    	let refcell_ref = (*refcell_mutexguard).clone(); // clones entire image :(
    	refcell_ref.take()
    }

    /// Returns f32 between `0` (not started) and `1` (finished)
    pub fn get_progress(&self) -> f32 {
        0.42
    }

    pub fn get_image_size(&self) -> [usize; 2] {
        self.renderer_data.size
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.renderer_data
            .bail_threads
            .store(true, Ordering::Relaxed);

		let handles = std::mem::take(&mut self.thread_handles);
        handles.into_iter().for_each(|thread| {
            thread.join().unwrap();
        });
    }
}

impl RendererData {
    pub fn render_thread_run(
        &self,
        starting_coordinate_inclusive: [usize; 2],
        ending_coordinate_exclusive: [usize; 2],
    ) {
        let mut rng = rand::thread_rng();
        let color = Color32::from_rgb(rng.gen(), rng.gen(), rng.gen());

        for y in starting_coordinate_inclusive[1]..ending_coordinate_exclusive[1] {
            let start_x = if y == starting_coordinate_inclusive[1] {
                starting_coordinate_inclusive[0]
            } else {
                0
            };
            let end_x = if y + 1 == ending_coordinate_exclusive[1] {
                ending_coordinate_exclusive[0]
            } else {
                self.size[0]
            };

            for x in start_x..end_x {
                self.data.lock().unwrap().get_mut()[x + y * self.size[0]] = color;

                if self.bail_threads.load(Ordering::Relaxed) {
                    return;
                }
            }
        }
    }
}
