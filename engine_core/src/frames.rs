use std::sync::Mutex;
use std::{collections::VecDeque, sync::Arc, time::Duration};
use std::time::Instant;
use crate::runtime::Tokio;

pub struct FrameManager {
    pub(crate) vec_fps: Arc<Mutex<VecDeque<f32>>>,
    pub(crate) sum_fps: Arc<Mutex<f64>>,
    pub(crate) last_frame_instant: Option<Instant>,
    pub(crate) desc: FrameManagerDesc,
}

#[derive(Clone)]
pub struct FrameManagerDesc {
    pub(crate) target_fps: f32,                // 0.0 => sem limite (Poll)
    pub(crate) time_to_death_fps: Duration,    // janela deslizante em segundos
    pub(crate) tokio: Tokio,
}

impl FrameManagerDesc {
    pub fn new(target_fps: f32, time_to_death_fps: Duration, tokio: Tokio) -> Self {
        Self { target_fps, time_to_death_fps, tokio }
    }
}

impl Default for FrameManagerDesc {
    fn default() -> Self {
        Self {
            target_fps: 60.0,
            time_to_death_fps: Duration::from_secs(60),
            tokio: Tokio::new(),
        }
    }
}

impl FrameManager {
    pub fn new(desc: FrameManagerDesc) -> Self {
        Self {
            vec_fps: Arc::new(Mutex::new(VecDeque::new())),
            sum_fps: Arc::new(Mutex::new(0.0)),
            last_frame_instant: None,
            desc,
        }
    }

    pub fn run(&self) {
        if self.desc.tokio.runtime.is_some() {
            self.init_debug(self.desc.tokio.runtime.as_ref().unwrap());
        }
    }

    fn max_samples(&self) -> usize {
        if self.desc.target_fps <= 0.0 {
            // modo ilimitado: janela ~5s em 120fps (estimativa)
            (5.0 * 120.0) as usize
        } else {
            (self.desc.time_to_death_fps.as_secs_f32() * self.desc.target_fps) as usize
        }.max(1)
    }

    pub fn add_frame(&mut self, delta_time: &Duration) {
        let dt = delta_time.as_secs_f32();
        if !dt.is_finite() || dt <= 0.0 {
            return;
        }
        let fps = 1.0 / dt;

        let mut sum_guard = self.sum_fps.lock().unwrap();
        let mut vec_guard = self.vec_fps.lock().unwrap();

        // debug opcional
        // println!("FPS Add, dt = {:.4} s -> {:.1} fps", dt, fps);

        *sum_guard += fps as f64;
        vec_guard.push_back(fps);

        let max_samples = self.max_samples();
        while vec_guard.len() > max_samples {
            if let Some(old_fps) = vec_guard.pop_front() {
                *sum_guard -= old_fps as f64;
            }
        }
    }

    pub fn init_debug(&self, runtime: &tokio::runtime::Runtime) {
        println!("Inicializando Frames com tempo para morte de FPS: {:?}", self.desc.time_to_death_fps);

        let sum_fps_arc = Arc::clone(&self.sum_fps);
        let vec_fps_arc = Arc::clone(&self.vec_fps);

        runtime.spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            interval.tick().await; // pula primeiro tick
            loop {
                interval.tick().await;

                let sum_fps_result = sum_fps_arc.lock();
                let vec_fps_result = vec_fps_arc.lock();

                match (sum_fps_result, vec_fps_result) {
                    (Ok(sum_fps), Ok(vec_fps)) => {
                        let len = vec_fps.len();
                        if len > 0 {
                            let avg_fps = *sum_fps / len as f64;

                            let mut sorted = vec_fps.iter().copied().collect::<Vec<f32>>();
                            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

                            let idx = |q: f32| ((sorted.len() as f32 * q).floor() as usize).min(sorted.len()-1);

                            let low1  = sorted[idx(0.01)];
                            let low5  = sorted[idx(0.05)];
                            let low25 = sorted[idx(0.25)];
                            let max25 = sorted[idx(0.75)];
                            let max5  = sorted[idx(0.95)];
                            let max1  = sorted[idx(0.99)];

                            println!(
                                "[DEBUG] FPS Médio: {:.1} | 1% Low: {:.1} | 5% Low: {:.1} | 25% Low: {:.1} | 25% Max: {:.1} | 5% Max: {:.1} | 1% Max: {:.1}",
                                avg_fps as f32, low1, low5, low25, max25, max5, max1
                            );
                        }
                    }
                    _ => eprintln!("[DEBUG] Erro ao obter lock do Mutex."),
                }
            }
        });
    }

    // chame isto APÓS apresentar o frame (em RedrawRequested)
    pub fn register_frame(&mut self) {
        let now = Instant::now();
        let dt = if let Some(prev) = self.last_frame_instant {
            now.saturating_duration_since(prev)
        } else {
            // primeiro frame: escolha um dt válido
            if self.desc.target_fps <= 0.0 {
                Duration::from_millis(1)
            } else {
                Duration::from_secs_f32(1.0 / self.desc.target_fps)
            }
        };
        self.last_frame_instant = Some(now);
        self.add_frame(&dt);
    }

    // None => ilimitado (Poll), Some(dt) => limitado (WaitUntil)
    pub fn next_frame_control_flow(&self) -> Option<Duration> {
        if self.desc.target_fps <= 0.0 {
            None
        } else {
            Some(Duration::from_secs_f32(1.0 / self.desc.target_fps))
        }
    }
}

impl Default for FrameManager {
    fn default() -> Self { Self::new(FrameManagerDesc::default()) }
}
