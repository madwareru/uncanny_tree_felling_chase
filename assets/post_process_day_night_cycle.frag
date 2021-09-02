#version 330 core
precision lowp float;

in vec4 color;
in vec2 uv;
layout(location = 0) out vec4 frag_color;

uniform float time;
uniform sampler2D Texture;

void main() {
    float day_time = time / 15.0;

    vec3 tex_color = texture(Texture, uv).rgb * color.rgb;
    float l = dot(vec3(0.3, 0.59, 0.11), tex_color);

    vec3 morning_color = tex_color * vec3(0.75, 0.75, 0.8);
    vec3 evening_color = tex_color * vec3(0.8, 0.6, 0.6);
    vec3 night_color = tex_color * vec3(0.3, 0.3, 0.9);

    float lerp_t = fract(day_time * 4.0);
    float day_fract = fract(day_time);

    frag_color = vec4(
       step(day_fract, 0.25) * mix(night_color, morning_color, lerp_t) +
       step(day_fract, 0.5) * step(0.25, day_fract) * mix(morning_color, tex_color, lerp_t) +
       step(day_fract, 0.75) * step(0.5, day_fract) * mix(tex_color, evening_color, lerp_t) +
       step(0.75, day_fract) * mix(evening_color, night_color, lerp_t),
       1.0
    );
}