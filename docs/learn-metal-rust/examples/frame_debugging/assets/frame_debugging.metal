#include <metal_stdlib>
using namespace metal;
struct VertexIn { float2 position; float4 color; };
struct VertexOut { float4 position [[position]]; float4 color; };
struct Args { float rotation; };
vertex VertexOut vs(device const VertexIn* v [[buffer(0)]],
                    constant Args& args [[buffer(1)]],
                    uint vid [[vertex_id]]) {
    float2 p = v[vid].position;
    float c = cos(args.rotation), s = sin(args.rotation);
    VertexOut o;
    o.position = float4(p.x*c - p.y*s, p.x*s + p.y*c, 0.0, 1.0);
    o.color = v[vid].color;
    return o;
}
fragment float4 fs(VertexOut i [[stage_in]]) { return i.color; }

