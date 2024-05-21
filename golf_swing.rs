use plotters::prelude::*;
use std::f64::consts::PI;

struct SwingParameters {
    theta_h0: f64,
    omega_h: f64,
    alpha_h: f64,
    theta_s0: f64,
    omega_s: f64,
    alpha_s: f64,
    phi_a0: f64,
    omega_a: f64,
    alpha_a: f64,
    rc: f64,
    dt: f64,
    simulation_time: f64,
}

fn main() {
    let params = SwingParameters {
        theta_h0: 0.0,
        omega_h: 0.0,
        alpha_h: 5.0,
        theta_s0: 0.0,
        omega_s: 0.0,
        alpha_s: 10.0,
        phi_a0: 0.0,
        omega_a: 0.0,
        alpha_a: 15.0,
        rc: 1.0,
        dt: 0.01,
        simulation_time: 2.0,
    };

    let trajectory = simulate_swing(params);

    // Plotting the trajectory
    plot_trajectory(&trajectory).expect("Failed to plot trajectory");
    println!("Trajectory plotted successfully.");
}

fn simulate_swing(params: SwingParameters) -> Vec<(f64, f64)> {
    let mut theta_h = params.theta_h0;
    let mut theta_s = params.theta_s0;
    let mut phi_a = params.phi_a0;
    
    let mut omega_h = params.omega_h;
    let mut omega_s = params.omega_s;
    let mut omega_a = params.omega_a;

    let mut trajectory = Vec::new();

    for t in (0..).map(|x| x as f64 * params.dt).take_while(|&x| x <= params.simulation_time) {
        theta_h += omega_h * params.dt + 0.5 * params.alpha_h * params.dt * params.dt;
        theta_s += omega_s * params.dt + 0.5 * params.alpha_s * params.dt * params.dt;
        phi_a += omega_a * params.dt + 0.5 * params.alpha_a * params.dt * params.dt;

        omega_h += params.alpha_h * params.dt;
        omega_s += params.alpha_s * params.dt;
        omega_a += params.alpha_a * params.dt;

        if (t + params.dt) > params.simulation_time {
            let launch_angle = theta_h + theta_s + phi_a;
            let vc = (omega_h + omega_s + omega_a) * params.rc;
            let vx = vc * launch_angle.cos();
            let vy = vc * launch_angle.sin();
            trajectory.push(projectile_motion(vx, vy, params.dt));
        }
    }

    trajectory
}

fn projectile_motion(vx: f64, vy: f64, dt: f64) -> (f64, f64) {
    let g = 9.81;
    let t_flight = 2.0 * vy / g;
    let max_height = vy * vy / (2.0 * g);
    let range = vx * t_flight;
    (range, max_height)
}

fn plot_trajectory(trajectory: &[(f64, f64)]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("trajectory.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Golf Ball Trajectory", ("sans-serif", 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..100.0, 0.0..50.0)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        trajectory.iter().map(|&(x, y)| (x, y)),
        &RED,
    ))?;

    root.present()?;
    Ok(())
}
