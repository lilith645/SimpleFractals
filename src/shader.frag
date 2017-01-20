#version 150 core

out vec4 color;

uniform float zoom;
uniform float offset;
uniform int fractal_type;

vec3 hsv2rbg(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    // MandelbrotSet
    if (fractal_type == 0) {
        vec2 c = gl_FragCoord.xy / (767.0+zoom) * 4 - offset;
        vec2 z = c;
        int max_iterations = 1000;
        float i;
        for(i = 0; i < max_iterations; ++i) {
            z = vec2(pow(z.x, 2) - pow(z.y, 2), 2*z.x*z.y) +c;
            if(length(z) > 2.0) {
              break;
            }
        }
    
        if(i == max_iterations) {
            color = vec4(0.0, 0.0, 0.0, 1.0);
        } else {
            float val = i / float(max_iterations);
            color = vec4(hsv2rbg(vec3(val, 1.0, 1.0)), 1.0);
        }
    // SierpinskiCarpet
    } else if (fractal_type == 1) {
        int x = int(gl_FragCoord.x);
        int y = int(gl_FragCoord.y);
        while(x > 0 || y > 0) {
          if(x%3 == 1 && y%3 == 1) {
            break;
          }
          x /= 3;
          y /= 3;
        }
    
        if(x <= 0 || y <= 0) {
          color = vec4(0.0, 0.0, 0.0, 1.0);
        } else {
          color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    }
}
