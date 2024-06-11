const MAX_ITERATIONS: u32 = 400;
const SMOOTH_SHADING: bool = true;
const CUSTOM_ADDITIONAL_COLORS: bool = true;

struct Camera {
    position: vec2<f32>,
    size: vec2<f32>,
}

@group(0)
@binding(0)
var<uniform> camera: Camera;

@group(1)
@binding(0)
var<uniform> time: u32;

struct VertexInput {
    @location(0) coord: vec2<f32>,
};

@fragment
fn main(vertex: VertexInput) -> @location(0) vec4<f32> {
    var position = vertex.coord;

    // Camera scale
    position *= camera.size;

    // Camera offset
    position += camera.position;

    var iterations = mandelbrot(vec2<f32>(position), MAX_ITERATIONS);

    var color: vec3<f32>;
    if iterations % 2 == 0 {
        color += vec3(0.1, 0.0, 0.0);
    }

    if u32(floor(iterations)) == MAX_ITERATIONS {
        color = vec3(0.0);
    } else {
        // iterations = (iterations + time) % MAX_ITERATIONS;

        let color1 = color_palette(u32(floor(iterations)));
        let color2 = color_palette(u32(floor(iterations)) + 1);
        
        let fract: f32 = iterations % 1;
        color = color1 * (1.0 - fract) + color2 * fract;

        if (CUSTOM_ADDITIONAL_COLORS) {
            let relative_iterations = iterations / f32(MAX_ITERATIONS);
            color += vec3(0.4 * relative_iterations, 0.1 * relative_iterations, 0.0);
            color += vec3(0.0, 0.0, 0.6 - relative_iterations);
            color += vec3(0.3 * sin(position.y), 0.0, 0.3 * sin(position.y));
        }
    }


    return vec4<f32>(color, 1.0);
}

fn mandelbrot(position: vec2<f32>, max_iterations: u32) -> f32 {
    var x0: f32 = position.x;
    var y0: f32 = position.y;

    // Bulb check
    let q = (x0 - 0.25) * (x0 - 0.25) + y0 * y0;
    let left = q * (q + (x0 - 0.25));
    let right = 0.25 * y0 * y0;
    if (left <= right) {
        return f32(max_iterations);
    }

    var x: f32 = 0.0;
    var y: f32 = 0.0;
    var x2: f32 = 0.0;
    var y2: f32 = 0.0;
    var i: u32 = 0;

    var escape_time: f32;
    if (SMOOTH_SHADING) {
        escape_time = 1000.0;
    } else {
        escape_time = 4.0;
    }

    while (x2 + y2 <= escape_time && i < max_iterations) {
        y = (x + x) * y + y0;
        x = x2 - y2 + x0;
        x2 = x * x;
        y2 = y * y;
        i += u32(1);
    }

    if (SMOOTH_SHADING) {
        // Compute fractional iteration count for smooth shading
        let log_zn = log(x2 + y2) / 2.0;
        let nu = log(log_zn / log(2.0)) / log(2.0);

        return f32(i) + 1.0 - nu;
    }

    return f32(i);
}

fn color_palette(i: u32) -> vec3<f32> {
    let n = i % 16;
    if (n == 0) {
        return vec3(66.0 / 255.0, 30.0 / 255.0, 15.0 / 255.0);
    } else if (n == 1) {
        return vec3(25.0 / 255.0, 7.0 / 255.0, 26.0 / 255.0);
    } else if (n == 2) {
        return vec3(9.0 / 255.0, 1.0 / 255.0, 47.0 / 255.0);
    } else if (n == 3) {
        return vec3(4.0 / 255.0, 4.0 / 255.0, 73.0 / 255.0);
    } else if (n == 4) {
        return vec3(0.0 / 255.0, 7.0 / 255.0, 100.0 / 255.0);
    } else if (n == 5) {
        return vec3(12.0 / 255.0, 44.0 / 255.0, 138.0 / 255.0);
    } else if (n == 6) {
        return vec3(24.0 / 255.0, 82.0 / 255.0, 177.0 / 255.0);
    } else if (n == 7) {
        return vec3(57.0 / 255.0, 125.0 / 255.0, 209.0 / 255.0);
    } else if (n == 8) {
        return vec3(134.0 / 255.0, 181.0 / 255.0, 229.0 / 255.0);
    } else if (n == 9) {
        return vec3(211.0 / 255.0, 236.0 / 255.0, 248.0 / 255.0);
    } else if (n == 10) {
        return vec3(241.0 / 255.0, 233.0 / 255.0, 191.0 / 255.0);
    } else if (n == 11) {
        return vec3(248.0 / 255.0, 201.0 / 255.0, 95.0 / 255.0);
    } else if (n == 12) {
        return vec3(255.0 / 255.0, 170.0 / 255.0, 0.0 / 255.0);
    } else if (n == 13) {
        return vec3(204.0 / 255.0, 128.0 / 255.0, 0.0 / 255.0);
    } else if (n == 14) {
        return vec3(153.0 / 255.0, 87.0 / 255.0, 0.0 / 255.0);
    } else {
        return vec3(106.0 / 255.0, 52.0 / 255.0, 3.0 / 255.0);
    }
}