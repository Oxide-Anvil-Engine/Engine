use std::sync::Mutex;
use std::{collections::VecDeque, sync::Arc, time::Duration};

pub struct FramesManager {
    pub time_to_death_fps: Duration,
    pub vec_fps: Arc<Mutex<VecDeque<f32>>>,
    pub sum_fps: Arc<Mutex<f64>>,
    pub target_fps: f32,
}

impl FramesManager {
    pub fn new(target_fps: f32, time_to_death_fps: Duration) -> Self {
        Self {
            target_fps,
            time_to_death_fps,
            vec_fps: Arc::new(Mutex::new(VecDeque::new())),
            sum_fps: Arc::new(Mutex::new(0.0)),
        }
    }

    pub fn add_frame(&mut self, delta_time: &Duration) {
        let fps = 1. / delta_time.as_secs_f32();
        let mut sum_guard = self.sum_fps.lock().unwrap();
        let mut vec_guard = self.vec_fps.lock().unwrap();

        *sum_guard += fps as f64;
        vec_guard.push_back(fps);

        while vec_guard.len() as u32
            > (self.time_to_death_fps.as_secs_f32() * self.target_fps) as u32
        {
            if let Some(old_fps) = vec_guard.pop_front() {
                *sum_guard -= old_fps as f64;
            }
        }
    }

    pub fn init_debug(&self) {
        println!(
            "Inicializando Frames com tempo para morte de FPS: {:?}",
            self.time_to_death_fps
        );

        // 1. 🟢 CLONAR OS ARC<MUTEX>
        // A tarefa precisa de sua própria cópia das referências.
        let sum_fps_arc = Arc::clone(&self.sum_fps);
        let vec_fps_arc = Arc::clone(&self.vec_fps);

        // 2. 🟢 SPAWN E INTERVALO
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            interval.tick().await; // Pula o tick inicial (por base imediato)

            loop {
                interval.tick().await;

                // 3. 🟢 ACESSO SEGURO E CÁLCULO
                let sum_fps_result = sum_fps_arc.lock();
                let vec_fps_result = vec_fps_arc.lock();

                match (sum_fps_result, vec_fps_result) {
                    (Ok(sum_fps), Ok(vec_fps)) => {
                        let len = vec_fps.len();
                        if len > 0 {
                            let avg_fps = *sum_fps / len as f64;

                            let mut ordenade_fps = vec_fps.iter().copied().collect::<Vec<f32>>();
                            ordenade_fps.sort_by(|a, b| a.partial_cmp(b).unwrap());

                            let perc_low1 = ((len as f32 * 0.01) as usize).min(len - 1);
                            let perc_low5 = ((len as f32 * 0.05) as usize).min(len - 1);
                            let perc_low25 = ((len as f32 * 0.25) as usize).min(len - 1);

                            let low1 = &ordenade_fps[0..=perc_low1];
                            let low5 = &ordenade_fps[0..=perc_low5];
                            let low25 = &ordenade_fps[0..=perc_low25];

                            let perc_max1 = ((len as f32 * 0.99) as usize).min(len - 1);
                            let perc_max5 = ((len as f32 * 0.95) as usize).min(len - 1);
                            let perc_max25 = ((len as f32 * 0.75) as usize).min(len - 1);

                            let max1 = &ordenade_fps[perc_max1..];
                            let max5 = &ordenade_fps[perc_max5..];
                            let max25 = &ordenade_fps[perc_max25..];

                            println!(
                                "[DEBUG] FPS Médio: {:.2} | 1% Low: {:.2} | 5% Low: {:.2} | 25% Low: {:.2} | 25% Max: {:.2} | 5% Max: {:.2} | 1% Max: {:.2}",
                                avg_fps,
                                low1.iter().sum::<f32>() / low1.len() as f32,
                                low5.iter().sum::<f32>() / low5.len() as f32,
                                low25.iter().sum::<f32>() / low25.len() as f32,
                                max25.iter().sum::<f32>() / max25.len() as f32,
                                max5.iter().sum::<f32>() / max5.len() as f32,
                                max1.iter().sum::<f32>() / max1.len() as f32,
                            );
                        }
                    }
                    _ => {
                        // Se a thread principal estiver em panic, o lock pode falhar.
                        eprintln!("[DEBUG] Erro ao obter lock do Mutex.");
                    }
                }
            }
        });
    }
}

impl Default for FramesManager {
    fn default() -> Self {
        Self::new(60.0, Duration::from_secs(60))
    }
}
