use std::{thread, time};

fn main() {
    let mut a: f64 = 1.0;
    let mut b: f64 = 1.0;

    loop {
        a += 0.07;
        b += 0.03;

        let (sa, ca) = a.sin_cos();
        let (sb, cb) = b.sin_cos();
        let mut q = [' '; 80*22];
        let mut z = [0.0_f64; 80*22];

        let mut theta: f64 = 0.0;
        while theta <= 6.28 {
            let (st, ct) = theta.sin_cos();

            let mut phi: f64 = 0.0;
            while phi <= 6.28 {
                let (sp, cp) = phi.sin_cos();
                let h = ct+2.0;
                let d = 1.0/(sp*h*sa+st*ca+5.0);
                let t = sp*h*ca-st*sa;

                // or x, y with 0?
                let x = (40.0+30.0*d*(cp*h*cb-t*sb)) as usize;
                let y = (12.0+15.0*d*(cp*h*sb+t*cb)) as usize;
                let o = x+80*y;
                let n = 8.0*((st*sa - sp*ct*ca)*cb - sp*ct*sa - st*ca - cp*ct*sb);

                if y<22 && x<79 && d > z[o] {
                    z[o] = d;
                    q[o] = "▁▂▂▃▄▄▅▆▆▇██".chars().nth(n as usize).or(Some('▁')).unwrap();
                }

                phi += 0.02;
            }

            theta += 0.07;
        }

        print!("\x1B[H{}",q.chunks(80).map(|l|l.iter().collect::<String>()).collect::<Vec<String>>().join("\n"));
        thread::sleep(time::Duration::from_millis(16));
    }
}
