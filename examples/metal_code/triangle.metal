#include <metal_stdlib>
using namespace metal;

struct VertexOut {
    float4 position [[position]];
    float4 color;
};

vertex VertexOut triangle_vertex(uint vid [[vertex_id]]) {
    float2 positions[3] = {
        float2(-0.5, -0.5), float2(0.5, -0.5), float2(0.0, 0.5)
    };
    float3 colors[3] = {
        float3(1.0, 0.2, 0.2), float3(0.2, 1.0, 0.2), float3(0.2, 0.2, 1.0)
    };
    VertexOut out;
    out.position = float4(positions[vid], 0.0, 1.0);
    out.color = float4(colors[vid], 1.0);
    return out;
}

fragment float4 triangle_fragment(VertexOut in [[stage_in]]) {
    return in.color;
}
