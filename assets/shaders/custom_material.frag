#version 450

#define TWO_PI 6.28318530718

layout(location = 0) in vec2 v_Uv;
layout(location = 1) in vec2 v_Pr;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform CellMaterial {
    vec4 Color;
};

void main() {
    vec2 st = v_Uv;

    float  d = length( max(abs(st)-.8,0.) );
    o_Target = vec4(Color.xyz-vec3( smoothstep(.1,.15,d)* smoothstep(.9,.1,d)) ,Color.w);
}